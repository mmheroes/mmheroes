import UIKit

private let gameStateRestorationKey =
    "com.jaskiewiczs.mmheroes.gameStateRestorationKey"

final class MainSceneViewController: UIViewController {

    @IBOutlet var gameView: GameView!

    private var mainRenderer: UIKitMMHeroesRenderer?
    private var gameThread: Thread?

    private var font = UIFont(name: "Menlo", size: 12)!

    let gameStateLock = NSLock()

    /// This variable is used for state restoration.
    var gameState = GameState() // guarded by gameStateLock
    var runner: GameRunner?     // guarded by gameStateLock

    private var gameHasStarted = false

    override func viewDidLoad() {
        super.viewDidLoad()
        setUpKeyCommands()
        gameView.didRedraw = { [weak self] in
            self?.mainRenderer?.viewDidFinishRedrawing()
        }
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)

        if !gameHasStarted {
            gameHasStarted = true
            startGame()
        }
    }

    deinit {
        gameThread?.cancel()
        mainRenderer?.sendInput(MMHEROES_Input_EOF)
    }

    private func startGame() {
        let mainRenderer = UIKitMMHeroesRenderer(font: font) { [weak self] text, caret in
            DispatchQueue.main.async {
                self?.gameView.text = text
                self?.gameView.caret = caret
                self?.gameView.font = self?.font
                self?.gameView.setNeedsDisplay()
            }
        }
        self.mainRenderer = mainRenderer

        let gameThread = GameThread(vc: self, mainRenderer: mainRenderer)

        gameThread.name = "com.jaskiewiczs.mmheroes.game-loop-thread"

        self.gameThread = gameThread

        gameThread.start()
    }

    private func setUpKeyCommands() {

        let basicKeyCommands = [
            UIKeyCommand(input: UIKeyCommand.inputUpArrow,
                         modifierFlags: [],
                         action: #selector(moveUp(_:))),
            UIKeyCommand(input: UIKeyCommand.inputDownArrow,
                         modifierFlags: [],
                         action: #selector(moveDown(_:))),
            UIKeyCommand(input: "\r",
                         modifierFlags: [],
                         action: #selector(confirm(_:))),
        ]

        basicKeyCommands.forEach(addKeyCommand)

        func registerAnyKeyCommand(_ input: String) {
            addKeyCommand(UIKeyCommand(input: input,
                                       modifierFlags: [],
                                       action: #selector(anyKey(_:))))
        }

        registerAnyKeyCommand(UIKeyCommand.inputEscape)
        registerAnyKeyCommand(UIKeyCommand.inputLeftArrow)
        registerAnyKeyCommand(UIKeyCommand.inputRightArrow)
        registerAnyKeyCommand(UIKeyCommand.inputPageUp)
        registerAnyKeyCommand(UIKeyCommand.inputPageDown)
        if #available(iOS 13.4, *) {
            registerAnyKeyCommand(UIKeyCommand.f1)
            registerAnyKeyCommand(UIKeyCommand.f2)
            registerAnyKeyCommand(UIKeyCommand.f3)
            registerAnyKeyCommand(UIKeyCommand.f4)
            registerAnyKeyCommand(UIKeyCommand.f5)
            registerAnyKeyCommand(UIKeyCommand.f6)
            registerAnyKeyCommand(UIKeyCommand.f7)
            registerAnyKeyCommand(UIKeyCommand.f8)
            registerAnyKeyCommand(UIKeyCommand.f9)
            registerAnyKeyCommand(UIKeyCommand.f10)
            registerAnyKeyCommand(UIKeyCommand.f11)
            registerAnyKeyCommand(UIKeyCommand.f12)
            registerAnyKeyCommand(UIKeyCommand.inputHome)
            registerAnyKeyCommand(UIKeyCommand.inputEnd)
        }

        for asciiCode in 1 ..< (128 as UInt8) {
            if asciiCode == 0x0D {
                // We've already handled "\r".
                continue
            }
            registerAnyKeyCommand(String(UnicodeScalar(asciiCode)))
        }
    }

    @IBAction func moveUp(_ sender: Any) {
        didReceiveInput(MMHEROES_Input_KeyUp)
    }

    @IBAction func moveDown(_ sender: Any) {
        didReceiveInput(MMHEROES_Input_KeyDown)
    }

    @IBAction func confirm(_ sender: Any) {
        didReceiveInput(MMHEROES_Input_Enter)
    }

    @objc func anyKey(_ sender: Any) {
        didReceiveInput(MMHEROES_Input_Other)
    }

    private func didReceiveInput(_ input: MMHEROES_Input) {
        guard let renderer = self.mainRenderer else { return }
        if renderer.sendInput(input) {
            gameState.input.append(input)
        }
    }

    override func encodeRestorableState(with coder: NSCoder) {
        super.encodeRestorableState(with: coder)
        gameStateLock.lock()
        defer { gameStateLock.unlock() }
        do {
            try coder.encodeEncodable(gameState,
                                      forKey: gameStateRestorationKey)
        } catch {
            assertionFailure(String(describing: error))
        }
    }

    override func decodeRestorableState(with coder: NSCoder) {
        gameStateLock.lock()
        defer { gameStateLock.unlock() }
        do {
            gameState = try coder
                .decodeDecodable(GameState.self,
                                 forKey: gameStateRestorationKey)
        } catch DecodingError.valueNotFound {
            // do nothing
        } catch {
            assertionFailure(String(describing: error))
        }
        super.decodeRestorableState(with: coder)
    }

    override var prefersHomeIndicatorAutoHidden: Bool {
        return true
    }
}

