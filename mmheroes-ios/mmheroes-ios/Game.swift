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

    private(set) var requests: [RendererRequest] = [.clearScreen, .flush]

    private static let rendererRequestCallback: MMHEROES_RendererRequestCallback =
        { context, request in
            Unmanaged<Game>.fromOpaque(context!).takeUnretainedValue()
                .requests
                .append(RendererRequest(request))
        }

    let mode: MMHEROES_GameMode
    let seed: UInt64
    fileprivate var handle: UnsafeMutableRawPointer! = nil

    private let inputLogPtr = UnsafeMutablePointer<String>.allocate(capacity: 1)

    var inputLog: String {
        return inputLogPtr.pointee
    }

    init(mode: MMHEROES_GameMode, seed: UInt64, highScores: [HighScore]? = nil) {
        self.mode = mode
        self.seed = seed
        inputLogPtr.initialize(to: "")
        let inputRecorderSink = MMHEROES_InputRecorderSink(
            context: inputLogPtr,
            sink: { context, data, len in
                let log = context!.assumingMemoryBound(to: String.self)
                let bytes = UnsafeBufferPointer(start: data, count: Int(len))
                log.pointee += String(decoding: bytes, as: UTF8.self)
                return true
            },
            display: { context, formatter in
                let log = context!.assumingMemoryBound(to: String.self)
                return log.pointee.withUTF8 { buffer in
                    mmheroes_rust_display(
                        buffer.baseAddress, UInt(buffer.count),
                        formatter
                    )
                }

            }
        )
        if let highScores = highScores {
            handle = convertHighScoresToFFIFriendlyPointer(highScores) { buffer in
                mmheroes_game_create(
                    mode,
                    seed,
                    buffer.baseAddress,
                    nil,
                    allocator,
                    Unmanaged.passUnretained(self).toOpaque(),
                    Self.rendererRequestCallback,
                    inputRecorderSink
                )
            }
        } else {
            handle = mmheroes_game_create(
                mode,
                seed,
                nil,
                nil,
                allocator,
                Unmanaged.passUnretained(self).toOpaque(),
                Self.rendererRequestCallback,
                inputRecorderSink
            )
        }
    }

    var dayAndTime: (Int, Time)? {
        var day: UInt8 = 255
        var time = Time(rawValue: 255)
        if mmheroes_game_get_current_time(handle, &day, &time.rawValue) {
            return (Int(day), time)
        }
        return nil
    }

    var highScores: [HighScore] {
        get {
            withExtendedLifetime(self) {
                var result = [MMHEROES_HighScore](repeating: MMHEROES_HighScore(),
                                                  count: Int(MMHEROES_SCORE_COUNT))
                mmheroes_game_get_high_scores(handle, &result)

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
                    mmheroes_game_set_high_scores(handle, buffer.baseAddress)
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
        requests.removeAll()
        return withExtendedLifetime(self) {
            mmheroes_continue(handle, input)
        }
    }

    func flushInputRecorder() {
        withExtendedLifetime(self) {
            _ = mmheroes_flush_input_recorder(handle)
        }
    }

    deinit {
        mmheroes_game_destroy(handle, nil, deallocator)
        inputLogPtr.deinitialize(count: 1)
        inputLogPtr.deallocate()
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
