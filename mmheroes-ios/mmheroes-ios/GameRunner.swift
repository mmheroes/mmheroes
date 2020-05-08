import UIKit

private struct TextFragment {
    let startColumn: Int
    let priority: Int
    let text: NSAttributedString

    var endColumn: Int { startColumn + text.string.count }
}

private struct ConsoleLine {
    var fragments: [TextFragment] = []
}

/// This renderer runs in a background thread.
final class GameRunner {

    static let numberOfLines = 24

    static let numberOfColumns = 80

    private var lines = [ConsoleLine](repeating: ConsoleLine(), count: numberOfLines)

    private var currentLine = 0
    private var currentColumn = 0

    /// Strings that are added later have greater priority.
    private var currentPriority = 0

    private var foregroundColor = MMHEROES_Color_White.makeUIColor()
    private var backgorundColor = MMHEROES_Color_Black.makeUIColor()

    private enum InputState {
        case waitingForInput
        case ignoringInput
    }

    private var inputState = InputState.waitingForInput

    private let worker: DispatchQueue
    private let gameUI: GameUI
    private let font: UIFont
    private let requestDrawingRenderedContent: (NSAttributedString, Caret) -> Void

    init(worker: DispatchQueue,
         gameUI: GameUI,
         font: UIFont,
         _ requestDrawingRenderedContent: @escaping (NSAttributedString, Caret) -> Void) {
        self.worker = worker
        self.gameUI = gameUI
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
    func continueGame(input: MMHEROES_Input, completion: @escaping (GameStatus) -> Void) {
        guard case .waitingForInput = inputState else {
            worker.async { completion(.unexpectedInput) }
            return
        }
        inputState = .ignoringInput
        guard gameUI.continueGame(input: input) else {
            worker.async { completion(.gameEnded) }
            return
        }

        var requests = gameUI.requests()

        // По очереди выполняем все запросы. Запрос 'sleep' — особый случай.
        // Он асинхронный. Если встречаем его, то прерываем цикл и продолжаем его
        // уже после того как sleep завершится.
        func go() {
            while let request = requests.next() {
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
                    sleep(ms: milliseconds, completion: go)
                    return
                }
            }
            inputState = .waitingForInput
            completion(.expectingMoreInput)
        }

        worker.async(execute: go)
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
            (0 ..< Self.numberOfLines).map { _ in
                let spaces = String(repeating: " ", count: Self.numberOfColumns)
                return NSMutableAttributedString(string: spaces,
                                                 attributes: [.font : font])
            }
        for i in 0 ..< lines.count {
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
                let range = NSRange(startIndex ..< endIndex, in: resultLineString)
                resultLines[i].replaceCharacters(in: range, with: fragment.text)
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
                let attributedString = NSAttributedString(
                    string: String(line),
                    attributes: [.foregroundColor : foregroundColor,
                                 .backgroundColor : backgorundColor,
                                 .font : font]
                )
                let fragment = TextFragment(startColumn: currentColumn,
                                            priority: currentPriority,
                                            text: attributedString)
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
        foregroundColor = foreground.makeUIColor()
        backgorundColor = background.makeUIColor()
    }

    private func sleep(ms: Int, completion: @escaping () -> Void) {
        worker.asyncAfter(deadline: .now() + .milliseconds(ms), execute: completion)
    }
}

extension MMHEROES_Color {

    private static let uiColors = [#colorLiteral(red: 0, green: 0, blue: 0, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.3333333333, blue: 0, alpha: 1), #colorLiteral(red: 0, green: 0.3333333333, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.6666666667, blue: 0.6666666667, alpha: 1),
                                   #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 1, alpha: 1)]

    func makeUIColor() -> UIColor {
        Self.uiColors[Int(self.rawValue)]
    }
}
