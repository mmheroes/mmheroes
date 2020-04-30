/// This renderer does nothing except replaying the input values passed to it to whoever
/// calls its `getch` method.
///
/// We use it to restore the game state.
final class RecordedInputRenderer<RecordedInput: Sequence>
    where RecordedInput.Element == MMHEROES_Input
{
    enum Error: Swift.Error {
        case noMoreInput
    }

    private var iterator: RecordedInput.Iterator
    private var next: RecordedInput.Element?

    init(input: RecordedInput) {
        iterator = input.makeIterator()
        next = iterator.next()
    }

    var finished: Bool { next == nil }
}

extension RecordedInputRenderer: Renderer {
    func clearScreen() throws {
        if finished { throw Error.noMoreInput }
    }

    func flush() throws {
        if finished { throw Error.noMoreInput }
    }

    func writeString(_ string: String) throws {
        if finished { throw Error.noMoreInput }
    }

    func moveCursor(toLine line: Int, column: Int) throws {
        if finished { throw Error.noMoreInput }
    }

    func getCursorPosition() throws -> (Int, Int) {
        if finished { throw Error.noMoreInput }
        return (0, 0)
    }

    func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) throws {
        if finished { throw Error.noMoreInput }
    }

    func getch() throws -> MMHEROES_Input {
        if let input = next {
            next = iterator.next()
            return input
        } else {
            throw Error.noMoreInput
        }
    }

    func sleep(ms: Int) throws {
        if finished { throw Error.noMoreInput }
    }
}
