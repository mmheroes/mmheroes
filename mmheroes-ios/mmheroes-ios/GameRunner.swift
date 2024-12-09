import UIKit

private struct TextFragment: Codable {
    let startColumn: Int
    let priority: Int
    let foregroundColor: MMHEROES_Color
    let backgroundColor: MMHEROES_Color
    let text: String

    var endColumn: Int { startColumn + text.count }
}

private struct ConsoleLine: Codable {
    var fragments: [TextFragment] = []
}

@MainActor
final class GameRunner {

    static let numberOfLines = Int(MMHEROES_TERMINAL_HEIGHT)

    static let numberOfColumns = Int(MMHEROES_TERMINAL_WIDTH)

    private var lines = [ConsoleLine](repeating: ConsoleLine(), count: numberOfLines)

    private var currentLine = 0
    private var currentColumn = 0

    /// Strings that are added later have greater priority.
    private var currentPriority = 0

    private var foregroundColor = MMHEROES_Color_White
    private var backgroundColor = MMHEROES_Color_Black

    private enum InputState {
        case waitingForInput
        case ignoringInput
    }

    private var inputState = InputState.waitingForInput

    private var game: Game
    private let font: UIFont
    private let requestDrawingRenderedContent: @MainActor (NSAttributedString, Caret) -> Void

    init(font: UIFont,
         _ requestDrawingRenderedContent: @escaping @MainActor (NSAttributedString, Caret) -> Void) {

        let diamondBirthday = DateComponents(month: 12, day: 3)
        let today = Calendar.current.dateComponents([.month, .day], from: Date())

        let mode: MMHEROES_GameMode
        if diamondBirthday == today {
            mode = MMHEROES_GameMode_God
        } else {
            mode = MMHEROES_GameMode_SelectInitialParameters
        }

        self.game = Game(mode: mode, seed: .random(in: 0 ... .max), highScores: nil)
        self.font = font
        self.requestDrawingRenderedContent = requestDrawingRenderedContent
    }

    enum GameStatus {
        case unexpectedInput
        case expectingMoreInput
        case gameEnded
    }

    /// Асинхронно продолжает игру до следующего запроса на ввод.
    /// `completion` выполняется в `worker`.
    func continueGame(input: MMHEROES_Input) async throws -> GameStatus {
        guard case .waitingForInput = inputState else {
            return .unexpectedInput
        }
        inputState = .ignoringInput
        guard game.continueGame(input: input) else {
            return .gameEnded
        }
        try await render()
        return .expectingMoreInput
    }

    func render() async throws {
        for request in game.requests {
            switch request {
            case .clearScreen:
                clearScreen()
            case .flush:
                flush()
            case .writeString(let s):
                writeString(s)
            case .moveCursor(line: let line, column: let column):
                moveCursor(toLine: line, column: column)
            case .setColor(foreground: let foreground, background: let background):
                setColor(foreground: foreground, background: background)
            case .sleep(milliseconds: let milliseconds):
                try await sleep(ms: milliseconds)
            }
        }
        inputState = .waitingForInput
    }

    private func clearScreen() {
        for i in 0 ..< lines.count {
            lines[i].fragments.removeAll(keepingCapacity: true)
        }
        currentLine = 0
        currentColumn = 0
        currentPriority = 0
    }

    private func flush() {
        let resultLines: [NSMutableAttributedString] =
            (0..<Self.numberOfLines).map { _ in
                let spaces = String(repeating: " ", count: Self.numberOfColumns)
                return NSMutableAttributedString(string: spaces,
                                                 attributes: [.font: font])
            }
        for i in 0..<lines.count {
            lines[i].fragments.sort { lhs, rhs in
                if lhs.startColumn == rhs.startColumn {
                    return lhs.priority < rhs.priority
                }
                return lhs.startColumn < rhs.startColumn
            }

            for fragment in lines[i].fragments {
                let resultLineString = resultLines[i].string
                let startIndex = resultLineString
                    .index(resultLineString.startIndex, offsetBy: fragment.startColumn)
                let endIndex = resultLineString
                    .index(resultLineString.startIndex, offsetBy: fragment.endColumn)
                let range = NSRange(startIndex..<endIndex, in: resultLineString)
                let fragmentAttributedText = NSAttributedString(
                    string: fragment.text,
                    attributes: [
                        .font: font,
                        .foregroundColor: fragment.foregroundColor.makeUIColor(),
                        .backgroundColor: fragment.backgroundColor.makeUIColor()
                    ]
                )
                resultLines[i].replaceCharacters(in: range, with: fragmentAttributedText)
            }
        }

        let result = NSMutableAttributedString(string: "", attributes: [.font : font])
        for line in resultLines {
            result.append(line)
            result.append(NSAttributedString(string: "\n", attributes: [.font : font]))
        }


        let caret = Caret(line: currentLine,
                          column: currentColumn,
                          color: MMHEROES_Color_White)
        requestDrawingRenderedContent(result, caret)
    }

    private func writeString(_ string: String) {
        for (line, endsWithNewline) in string.lines {
            if !line.isEmpty {
                let fragment = TextFragment(startColumn: currentColumn,
                                            priority: currentPriority,
                                            foregroundColor: foregroundColor,
                                            backgroundColor: backgroundColor,
                                            text: String(line))
                currentColumn += line.count
                lines[currentLine].fragments.append(fragment)
            }
            if endsWithNewline {
                currentLine += 1
                currentColumn = 0
            }
        }
        currentPriority += 1
    }

    private func moveCursor(toLine line: Int, column: Int) {
        self.currentLine = line
        self.currentColumn = column
    }

    private func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) {
        foregroundColor = foreground
        backgroundColor = background
    }

    private func sleep(ms: Int) async throws {
        try await Task.sleep(nanoseconds: UInt64(ms * 1_000_000))
    }
}

extension GameRunner {

    private struct RestorableState: Codable {
        var lines: [ConsoleLine]
        var currentLine: Int
        var currentColumn: Int
        var currentPriority: Int
        var foregroundColor: MMHEROES_Color
        var backgorundColor: MMHEROES_Color
        var seed: UInt64
        var mode: MMHEROES_GameMode
        var recordedInput: String
    }

    func restoreGameState(from coder: NSCoder) throws {
        let restorableState = try coder.decodeDecodable(
            RestorableState.self,
            forKey: Bundle.main.bundleIdentifier! + "GameRunner.restorableState"
        )
        lines = restorableState.lines
        currentLine = restorableState.currentLine
        currentColumn = restorableState.currentColumn
        currentPriority = restorableState.currentPriority
        foregroundColor = restorableState.foregroundColor
        backgroundColor = restorableState.backgorundColor
        game = Game(mode: restorableState.mode, seed: restorableState.seed)
        game.replay(recordedInput: restorableState.recordedInput)
    }

    func encodeGameState(to coder: NSCoder) throws {
        game.flushInputRecorder()
        let restorableState = RestorableState(lines: lines,
                                              currentLine: currentLine,
                                              currentColumn: currentColumn,
                                              currentPriority: currentPriority,
                                              foregroundColor: foregroundColor,
                                              backgorundColor: backgroundColor,
                                              seed: game.seed,
                                              mode: game.mode,
                                              recordedInput: game.inputLog)
        try coder.encodeEncodable(
            restorableState,
            forKey: Bundle.main.bundleIdentifier! + "GameRunner.restorableState"
        )
    }
}

extension MMHEROES_Color {

    private static let uiColors = [#colorLiteral(red: 0, green: 0, blue: 0, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.3333333333, blue: 0, alpha: 1), #colorLiteral(red: 0, green: 0.3333333333, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.6666666667, blue: 0.6666666667, alpha: 1),
                                   #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 1, alpha: 1)]

    func makeUIColor() -> UIColor {
        Self.uiColors[Int(self.rawValue)]
    }
}
