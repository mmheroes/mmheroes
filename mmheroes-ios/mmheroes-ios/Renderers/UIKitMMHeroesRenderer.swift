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
final class UIKitMMHeroesRenderer {

    enum Error: Swift.Error {
        case threadCancelled
    }

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
        case receivedInput(MMHEROES_Input)
        case ignoringInput
    }

    private let inputStateCondition = NSCondition()
    private var inputState = InputState.ignoringInput // guarded by inputStateCondition

    private let didFinishRedrawingCondition = NSCondition()
    private var didFinishRedrawing = false // guarded by didFinishRedrawingCondition

    private let font: UIFont

    private let requestDrawingRenderedContent: (NSAttributedString, Caret) -> Void

    init(font: UIFont,
         _ requestDrawingRenderedContent: @escaping (NSAttributedString, Caret) -> Void) {
        self.font = font
        self.requestDrawingRenderedContent = requestDrawingRenderedContent
    }

    /// This method is called from the main thread when the user produces
    /// some input.
    /// Returns `true` if the input has been accepted.
    @discardableResult
    func sendInput(_ input: MMHEROES_Input) -> Bool {
        inputStateCondition.lock()
        defer { inputStateCondition.unlock() }
        if case .waitingForInput = inputState {
            inputState = .receivedInput(input)
            inputStateCondition.signal()
            return true
        }
        return false
    }

    func viewDidFinishRedrawing() {
        didFinishRedrawingCondition.lock()
        defer { didFinishRedrawingCondition.unlock() }
        didFinishRedrawing = true
        didFinishRedrawingCondition.signal()
    }
}

extension UIKitMMHeroesRenderer: Renderer {

    func clearScreen() throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        for i in 0 ..< lines.count {
            lines[i].fragments.removeAll(keepingCapacity: true)
        }
        currentLine = 0
        currentColumn = 0
        currentPriority = 0
    }

    func flush() throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
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

        didFinishRedrawingCondition.lock()
        defer { didFinishRedrawingCondition.unlock() }
        while !didFinishRedrawing {
            didFinishRedrawingCondition.wait()
        }
        didFinishRedrawing = false
    }

    func writeString(_ string: String) throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
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

    func moveCursor(toLine line: Int, column: Int) throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        self.currentLine = line
        self.currentColumn = column
    }

    func getCursorPosition() throws -> (Int, Int) {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        return (currentLine, currentColumn)
    }

    func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        foregroundColor = foreground.makeUIColor()
        backgorundColor = background.makeUIColor()
    }

    func getch() throws -> MMHEROES_Input {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        inputStateCondition.lock()
        defer { inputStateCondition.unlock() }
        inputState = .waitingForInput
        while true {
            if Thread.current.isCancelled { throw Error.threadCancelled }
            if case .receivedInput(let input) = inputState {
                inputState = .ignoringInput
                return input
            }
            inputStateCondition.wait()
        }
    }

    func sleep(ms: Int) throws {
        if Thread.current.isCancelled { throw Error.threadCancelled }
        Thread.sleep(forTimeInterval: TimeInterval(ms) / 1000)
    }
}

extension MMHEROES_Color {

    private static let uiColors = [#colorLiteral(red: 0, green: 0, blue: 0, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.3333333333, blue: 0, alpha: 1), #colorLiteral(red: 0, green: 0.3333333333, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0, green: 0.6666666667, blue: 0.6666666667, alpha: 1), #colorLiteral(red: 0.6666666667, green: 0.6666666667, blue: 0.6666666667, alpha: 1),
                                   #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 0.3333333333, alpha: 1), #colorLiteral(red: 0.3333333333, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 0.3333333333, blue: 1, alpha: 1), #colorLiteral(red: 0.3333333333, green: 1, blue: 1, alpha: 1), #colorLiteral(red: 1, green: 1, blue: 1, alpha: 1)]

    func makeUIColor() -> UIColor {
        Self.uiColors[Int(self.rawValue)]
    }
}
