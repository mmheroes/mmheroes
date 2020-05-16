private let allocator: MMHEROES_Allocator = { _, size, alignment in
    .allocate(byteCount: Int(size), alignment: Int(alignment))
}

private let deallocator: MMHEROES_Deallocator = { _, mem, _ in
    mem?.deallocate()
}

struct Time: RawRepresentable {
    var rawValue: MMHEROES_Time
    init(rawValue: MMHEROES_Time) {
        self.rawValue = rawValue
    }
}

struct HighScore {
    var name: String
    var money: MMHEROES_Money
}

final class Game {

    let mode: MMHEROES_GameMode
    let seed: UInt64
    fileprivate let handle: OpaquePointer

    init(mode: MMHEROES_GameMode, seed: UInt64) {
        self.mode = mode
        self.seed = seed
        handle = mmheroes_game_create(mode, seed, nil, allocator)
    }

    var dayAndTime: (Int, Time)? {
        var day: UInt8 = 255
        var time = Time(rawValue: 255)
        if mmheroes_game_get_current_time(handle, &day, &time.rawValue) {
            return (Int(day), time)
        }
        return nil
    }

    deinit {
        mmheroes_game_destroy(handle, nil, deallocator)
    }
}

private func convertHighScoresToFFIFriendlyPointer<R>(
    _ highScores: [HighScore],
    _ body: (UnsafeBufferPointer<MMHEROES_HighScore>) -> R
) -> R {
    precondition(highScores.count >= Int(MMHEROES_SCORE_COUNT))
    var ffiHighScores = [MMHEROES_HighScore]()
    ffiHighScores.reserveCapacity(Int(MMHEROES_SCORE_COUNT))
    func doTheWork(_ iterator: IndexingIterator<[HighScore]>) -> R {
        var iterator = iterator
        if var highScore = iterator.next() {
            return highScore.name.withUTF8 { buffer in
                ffiHighScores
                    .append(MMHEROES_HighScore(name: buffer.baseAddress,
                                               name_len: UInt(buffer.count),
                                               score: highScore.money))
                return doTheWork(iterator)
            }
        } else {
            precondition(ffiHighScores.count >= Int(MMHEROES_SCORE_COUNT))
            return ffiHighScores.withUnsafeBufferPointer(body)
        }
    }

    return doTheWork(highScores.makeIterator())
}

final class GameUI {

    private let handle: OpaquePointer

    let game: Game

    init(game: Game, highScores: [HighScore]? = nil) {
        self.game = game
        if let highScores = highScores {
            handle = convertHighScoresToFFIFriendlyPointer(highScores) { buffer in
                mmheroes_game_ui_create(game.handle,
                                        buffer.baseAddress,
                                        nil,
                                        allocator)
            }
        } else {
            handle = mmheroes_game_ui_create(game.handle, nil, nil, allocator)
        }
    }

    deinit {
        mmheroes_game_ui_destroy(handle, nil, deallocator)
    }

    var highScores: [HighScore] {
        get {
            withExtendedLifetime(self) {
                var result = [MMHEROES_HighScore](repeating: MMHEROES_HighScore(),
                                                  count: Int(MMHEROES_SCORE_COUNT))
                mmheroes_game_ui_get_high_scores(handle, &result)

                return result.map { ffiHighScore in
                    let nameBuf = UnsafeBufferPointer(start: ffiHighScore.name,
                                                      count: Int(ffiHighScore.name_len))
                    let name = String(decoding: nameBuf, as: UTF8.self)
                    return HighScore(name: name, money: ffiHighScore.score)
                }
            }
        }
        set {
            withExtendedLifetime(self) {
                convertHighScoresToFFIFriendlyPointer(newValue) { buffer in
                    mmheroes_game_ui_set_high_scores(handle, buffer.baseAddress)
                }
            }
        }
    }

    @discardableResult
    func replay(recordedInput: String) -> Bool {
        withExtendedLifetime(self) {
            recordedInput.withCString(encodedAs: UTF8.self) { buf in
                mmheroes_replay(handle, buf, UInt(recordedInput.utf8.count))
            }
        }
    }

    func continueGame(input: MMHEROES_Input) -> Bool {
        withExtendedLifetime(self) {
            mmheroes_continue(handle, input)
        }
    }

    func requests() -> RendererRequestIterator {
        withExtendedLifetime(self) {
            var iterator = MMHEROES_RendererRequestIterator()
            mmheroes_renderer_request_iterator_begin(&iterator, handle)
            return RendererRequestIterator(underlying: iterator)
        }
    }
}

enum RendererRequest {
    case clearScreen
    case flush
    case writeString(String)
    case moveCursor(line: Int, column: Int)
    case setColor(foreground: MMHEROES_Color, background: MMHEROES_Color)
    case sleep(milliseconds: Int)
}

extension RendererRequest {
    fileprivate init(_ request: MMHEROES_RendererRequest) {
        switch request.tag {
        case MMHEROES_RendererRequest_ClearScreen:
            self = .clearScreen
        case MMHEROES_RendererRequest_Flush:
            self = .flush
        case MMHEROES_RendererRequest_WriteStr:
            let buf = UnsafeBufferPointer(start: request.write_str.buf,
                                          count: Int(request.write_str.length))
            self = .writeString(String(decoding: buf, as: UTF8.self))
        case MMHEROES_RendererRequest_MoveCursor:
            self = .moveCursor(line: Int(request.move_cursor.line),
                               column: Int(request.move_cursor.column))
        case MMHEROES_RendererRequest_SetColor:
            self = .setColor(foreground: request.set_color.foreground,
                             background: request.set_color.background)
        case MMHEROES_RendererRequest_Sleep:
            self = .sleep(milliseconds: Int(request.sleep.milliseconds))
        default:
            fatalError("unreachable")
        }
    }
}

struct RendererRequestIterator {
    fileprivate var underlying: MMHEROES_RendererRequestIterator
}

extension RendererRequestIterator: Sequence, IteratorProtocol {
    typealias Element = RendererRequest

    mutating func next() -> RendererRequest? {
        var request = MMHEROES_RendererRequest()
        if mmheroes_renderer_request_iterator_next(&underlying, &request) {
            return RendererRequest(request)
        }
        return nil
    }
}

final class InputRecorder {

    private let handle: OpaquePointer

    private var sink: UnsafeMutablePointer<MMHEROES_InputRecorderSink>

    var recording: String

    init(recording: String = "") {
        self.recording = recording
        sink = .allocate(capacity: 1)
        sink.initialize(to: .init())
        handle = mmheroes_input_recorder_create(sink, nil, allocator)
        sink.pointee =
            .init(context: Unmanaged.passRetained(self).toOpaque()) { context, buf, len in
                let recorder = Unmanaged<InputRecorder>.fromOpaque(context!)
                    .takeUnretainedValue()
                let buffer = UnsafeBufferPointer(start: buf, count: Int(len))
                recorder.recording += String(decoding: buffer, as: UTF8.self)
                return true
            }
    }

    deinit {
        mmheroes_input_recorder_destroy(handle, nil, deallocator)
        sink.deallocate()
    }

    func record(_ input: MMHEROES_Input) {
        mmheroes_input_recorder_record(handle, input)
    }

    func flush() {
        mmheroes_input_recorder_flush(handle)
    }
}

