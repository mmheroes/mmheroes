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
    private var inputState = InputState.ignoringInput

    private let didFinishRedrawingCondition = NSCondition()
    private var didFinishRedrawing = false

    private let font: UIFont

    private let requestDrawingRenderedContent: (NSAttributedString) -> Void

    init(font: UIFont,
         _ requestDrawingRenderedContent: @escaping (NSAttributedString) -> Void) {
        self.font = font
        self.requestDrawingRenderedContent = requestDrawingRenderedContent
    }

    /// This method is called from the main thread when the user produces
    /// some input.
    func sendInput(_ input: MMHEROES_Input) {
        inputStateCondition.lock()
        defer { inputStateCondition.unlock() }
        if case .waitingForInput = inputState {
            inputState = .receivedInput(input)
            inputStateCondition.signal()
        }
    }

    func viewDidFinishRedrawing() {
        didFinishRedrawingCondition.lock()
        defer { didFinishRedrawingCondition.unlock() }
        didFinishRedrawing = true
        didFinishRedrawingCondition.signal()
    }
}

extension UIKitMMHeroesRenderer: Renderer {

    func clearScreen() {
        for i in 0 ..< lines.count {
            lines[i].fragments.removeAll(keepingCapacity: true)
        }
        currentLine = 0
        currentColumn = 0
        currentPriority = 0
    }

    func flush() {
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

        requestDrawingRenderedContent(result)

        didFinishRedrawingCondition.lock()
        defer { didFinishRedrawingCondition.unlock() }
        while !didFinishRedrawing {
            didFinishRedrawingCondition.wait()
        }
        didFinishRedrawing = false
    }

    func writeString(_ string: String) {
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

    func moveCursor(toLine line: Int, column: Int) {
        self.currentLine = line
        self.currentColumn = column
    }

    func getCursorPosition() -> (Int, Int) {
        return (currentLine, currentColumn)
    }

    func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) {
        foregroundColor = foreground.makeUIColor()
        backgorundColor = background.makeUIColor()
    }

    func getch() -> MMHEROES_Input {
        inputStateCondition.lock()
        defer { inputStateCondition.unlock() }
        inputState = .waitingForInput
        while true {
            if case .receivedInput(let input) = inputState {
                inputState = .ignoringInput
                return input
            }
            inputStateCondition.wait()
        }
    }

    func sleep(ms: Int) {
        Thread.sleep(forTimeInterval: TimeInterval(ms) / 1000)
    }
}

extension MMHEROES_Color {
    func makeUIColor() -> UIColor {
        switch self {
        case MMHEROES_Color_Black:
            return #colorLiteral(red: 0, green: 0, blue: 0, alpha: 1)
        case MMHEROES_Color_Yellow:
            return #colorLiteral(red: 0.6, green: 0.6, blue: 0, alpha: 1)
        case MMHEROES_Color_White:
            return #colorLiteral(red: 0.7490196078, green: 0.7490196078, blue: 0.7490196078, alpha: 1)
        case MMHEROES_Color_Gray:
            return #colorLiteral(red: 0.4, green: 0.4, blue: 0.4, alpha: 1)
        case MMHEROES_Color_Red:
            return #colorLiteral(red: 0.8980392157, green: 0, blue: 0, alpha: 1)
        case MMHEROES_Color_Green:
            return #colorLiteral(red: 0, green: 0.8509803922, blue: 0, alpha: 1)
        case MMHEROES_Color_YellowBright:
            return #colorLiteral(red: 0.8980392157, green: 0.8980392157, blue: 0, alpha: 1)
        case MMHEROES_Color_Cyan:
            return #colorLiteral(red: 0, green: 0.8980392157, blue: 0.8980392157, alpha: 1)
        case MMHEROES_Color_WhiteBright:
            return #colorLiteral(red: 0.9960784314, green: 0.9882352941, blue: 1, alpha: 1)
        default:
            fatalError("unreachable")
        }
    }
}
