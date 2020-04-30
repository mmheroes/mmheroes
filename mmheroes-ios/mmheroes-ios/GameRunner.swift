protocol Renderer: AnyObject {

    func clearScreen() throws

    func flush() throws

    func writeString(_ string: String) throws

    func moveCursor(toLine line: Int, column: Int) throws

    func getCursorPosition() throws -> (Int, Int)

    func setColor(foreground: MMHEROES_Color, background: MMHEROES_Color) throws

    func getch() throws -> MMHEROES_Input

    func sleep(ms: Int) throws
}

private class Box<T> {
    let value: T
    init(_ value: T) {
        self.value = value
    }
}

final class GameRunner {

    private let renderer: Renderer

    private var polymorphicRenderer: MMHEROES_PolymorphicRenderer

    init(renderer: Renderer) {
        self.renderer = renderer
        polymorphicRenderer = MMHEROES_PolymorphicRenderer(
            renderer_ctx: Unmanaged.passRetained(Box(renderer)).toOpaque(),
            clear_screen: { ctx, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    try renderer.clearScreen()
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            flush: { ctx, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    try renderer.flush()
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            move_cursor_to: { ctx, line, column, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    try renderer.moveCursor(toLine: Int(line), column: Int(column))
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            get_cursor_position: { ctx, lineOut, columnOut, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    let (line, column) = try renderer.getCursorPosition()
                    lineOut!.initialize(to: Int32(line))
                    columnOut!.initialize(to: Int32(column))
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            set_color: { ctx, foreground, background, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    try renderer.setColor(foreground: foreground, background: background)
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            write_str: { ctx, cStr, len, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    return try cStr!
                        .withMemoryRebound(to: UInt8.self, capacity: Int(len)) { base in
                            let buffer = UnsafeBufferPointer(start: base, count: Int(len))
                            let string = String(decoding: buffer, as: UTF8.self)
                            try renderer.writeString(string)
                            return true
                        }
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            getch: { ctx, chOut, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    let input = try renderer.getch()
                    chOut!.initialize(to: input)
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            },
            sleep_ms: { ctx, ms, errOut in
                let renderer = Unmanaged<Box<Renderer>>
                    .fromOpaque(ctx!)
                    .takeUnretainedValue()
                    .value
                do {
                    try renderer.sleep(ms: Int(ms))
                    return true
                } catch {
                    errOut!.initialize(to: Unmanaged.passRetained(Box(error)).toOpaque())
                    return false
                }
            }
        )
    }

    func run(seed: UInt64, mode: MMHEROES_GameMode) throws {
        try withUnsafeMutablePointer(to: &polymorphicRenderer) { polymorphicRendererPtr in
            var error: MMHEROES_OpaqueError?
            if mmheroes_run_game(polymorphicRendererPtr, mode, seed, &error) {
                return
            }
            throw Unmanaged<Box<Error>>.fromOpaque(error!).takeUnretainedValue().value
        }
    }
}
