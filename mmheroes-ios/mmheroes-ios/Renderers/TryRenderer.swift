/// A renderer that in each method first tries to call the same method of the
/// primary renderer, and if that throws an error, calls that method on the fallback
/// renderer.
final class TryRenderer<PrimaryRenderer: Renderer, FallbackRenderer: Renderer> {

    let primaryRenderer: PrimaryRenderer

    let fallbackRenderer: FallbackRenderer

    init(primaryRenderer: PrimaryRenderer, fallbackRenderer: FallbackRenderer) {
        self.primaryRenderer = primaryRenderer
        self.fallbackRenderer = fallbackRenderer
    }
}

extension TryRenderer: Renderer {

    func clearScreen() throws {
        do {
            try primaryRenderer.clearScreen()
        } catch {
            try fallbackRenderer.clearScreen()
        }
    }

    func flush() throws {
        do {
            try primaryRenderer.flush()
        } catch {
            try fallbackRenderer.flush()
        }
    }

    func writeString(_ string: String) throws {
        do {
            try primaryRenderer.writeString(string)
        } catch {
            try fallbackRenderer.writeString(string)
        }
    }

    func moveCursor(toLine line: Int, column: Int) throws {
        do {
            try primaryRenderer.moveCursor(toLine: line, column: column)
        } catch {
            try fallbackRenderer.moveCursor(toLine: line, column: column)
        }
    }

    func getCursorPosition() throws -> (Int, Int) {
        do {
            return try primaryRenderer.getCursorPosition()
        } catch {
            return try fallbackRenderer.getCursorPosition()
        }
    }

    func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) throws {
        do {
            try primaryRenderer.setColor(foreground: foreground, background: background)
        } catch {
            try fallbackRenderer.setColor(foreground: foreground, background: background)
        }
    }

    func getch() throws -> MMHEROES_Input {
        do {
            return try primaryRenderer.getch()
        } catch {
            return try fallbackRenderer.getch()
        }
    }

    func sleep(ms: Int) throws {
        do {
            try primaryRenderer.sleep(ms: ms)
        } catch {
            try fallbackRenderer.sleep(ms: ms)
        }
    }
}
