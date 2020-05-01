import UIKit

private let gameStateRestorationKey =
    "com.jaskiewiczs.mmheroes.gameStateRestorationKey"

final class MainSceneViewController: UIViewController {

    private let gameView = GameView()

    private let helpButton = UIButton(type: .infoDark)

    // Cache this view controller.
    private let helpViewController = HelpViewController()

    private var mainRenderer: UIKitMMHeroesRenderer?
    private var gameThread: Thread?

    private var consoleFont = UIFont(name: "Menlo", size: 12)!

    let gameStateLock = NSLock()

    /// This variable is used for state restoration.
    var gameState = GameState() // guarded by gameStateLock
    var runner: GameRunner?     // guarded by gameStateLock

    private var gameHasStarted = false

    override func viewDidLoad() {
        super.viewDidLoad()

        view.backgroundColor = MMHEROES_Color_Black.makeUIColor()

        setupGameView()
        setupGestureRecognizers()
        setupHelpButton()

        DispatchQueue.main.async {
            // This will asynchronously call the viewDidLoad, but will not layout
            // everything yet and will not block the main thread
            self.helpViewController.view.setNeedsLayout()
        }
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)

        if !gameHasStarted {
            gameHasStarted = true
            startGame()
        }
    }

    override func viewWillLayoutSubviews() {
        super.viewWillLayoutSubviews()
        updateGameViewLayout()
        updateHelpButtonPosition()
    }

    override func viewWillTransition(
        to size: CGSize,
        with coordinator: UIViewControllerTransitionCoordinator
    ) {
        // Hide the caret during orientation change, otherwise it is misplaced
        // during the rotation animation.
        gameView.caretHidden = true
        super.viewWillTransition(to: size, with: coordinator)
        coordinator.animate(
            alongsideTransition: { context in

            },
            completion: { [weak self] context in
                self?.gameView.caretHidden = false
            }
        )
    }

    deinit {
        gameThread?.cancel()
        mainRenderer?.sendInput(MMHEROES_Input_EOF)
    }

    private func setupGameView() {
        gameView.didRedraw = { [weak self] in
            self?.mainRenderer?.viewDidFinishRedrawing()
        }

        gameView.font = consoleFont
        gameView.backgroundColor = MMHEROES_Color_Black.makeUIColor()

        view.addSubview(gameView)

        updateGameViewLayout()
    }

    private func setupGestureRecognizers() {
        // Double tap means "Confirm choice"
        let tapGestureRecognizer = UITapGestureRecognizer()
        tapGestureRecognizer.numberOfTapsRequired = 2
        tapGestureRecognizer.addTarget(self, action: #selector(confirm(_:)))
        view.addGestureRecognizer(tapGestureRecognizer)

        // Swipe up means "Go to the previous option"
        let swipeUpGestureRecognizer = UISwipeGestureRecognizer()
        swipeUpGestureRecognizer.direction = .up
        swipeUpGestureRecognizer.addTarget(self, action: #selector(moveUp(_:)))
        view.addGestureRecognizer(swipeUpGestureRecognizer)

        // Swipe down means "Go to the next option"
        let swipeDownGestureRecognizer = UISwipeGestureRecognizer()
        swipeDownGestureRecognizer.direction = .down
        swipeDownGestureRecognizer.addTarget(self, action: #selector(moveDown(_:)))
        view.addGestureRecognizer(swipeDownGestureRecognizer)
    }

    private func updateGameViewLayout() {

        // Yep, no AutoLayout. We target iOS 8, and the features we need are not available
        // there.

        let minimumContentMargin: CGFloat = 11

        var minX = view.bounds.minX
        var minY = view.bounds.minY
        var maxX = minX + view.bounds.width
        var maxY = minY + view.bounds.height

        if #available(iOS 11.0, *) {
            let maxHorizontalInset = max(view.safeAreaInsets.left,
                                         view.safeAreaInsets.right,
                                         minimumContentMargin)
            let maxVerticalInset = max(view.safeAreaInsets.top,
                                       view.safeAreaInsets.bottom,
                                       minimumContentMargin)
            minX += maxHorizontalInset
            maxX -= maxHorizontalInset
            minY += maxVerticalInset
            maxY -= maxVerticalInset
        } else {
            minX += minimumContentMargin
            maxX -= minimumContentMargin
            minY += minimumContentMargin
            maxY -= minimumContentMargin
        }

        var width = maxX - minX
        var height = maxY - minY
        let currentAspectRatio = width / height
        let desiredAspectRatio = gameView.desiredAspectRatio

        if currentAspectRatio > desiredAspectRatio {
            // The view is stretched horizontally too much, reduce width.
            width = height * desiredAspectRatio
        } else {
            // The view is too much of a square, or even stretched vertically,
            // reduce height.
            height = width / desiredAspectRatio
        }

        // Center horizontally
        let x = minX + (maxX - minX - width) / 2
        let y = minY + (maxY - minY - height) / 2

        gameView.frame = CGRect(x: x,
                                y: y,
                                width: width,
                                height: height)

        // This is to update the caret position
        gameView.setNeedsDisplay()
    }

    private func setupHelpButton() {
        helpButton.tintColor = UIColor.white
        helpButton.frame.size = CGSize(width: 70, height: 70)
        helpButton.addTarget(self, action: #selector(help(_:)), for: .touchUpInside)
        view.addSubview(helpButton)
    }

    @objc
    private func help(_ button: UIButton) {
        let navigationController =
            UINavigationController(rootViewController: helpViewController)
        navigationController.modalPresentationStyle = .pageSheet
        navigationController.navigationBar.tintColor = .white
        navigationController.navigationBar.barStyle = .black
        navigationController.navigationBar.isTranslucent = true
        present(navigationController, animated: true, completion: nil)
    }

    private func updateHelpButtonPosition() {
        // Place the button in the lower right corner
        helpButton.frame.origin.x = view.bounds.maxX - helpButton.frame.width
        helpButton.frame.origin.y = view.bounds.maxY - helpButton.frame.height
    }

    private func startGame() {
        let mainRenderer =
            UIKitMMHeroesRenderer(font: consoleFont) { [weak self] text, caret in
                DispatchQueue.main.async {
                    self?.gameView.text = text
                    self?.gameView.caret = caret
                    self?.gameView.font = self?.consoleFont
                    self?.gameView.setNeedsDisplay()
                }
            }

        self.mainRenderer = mainRenderer

        let gameThread = GameThread(vc: self, mainRenderer: mainRenderer)

        gameThread.name = "com.jaskiewiczs.mmheroes.game-loop-thread"

        self.gameThread = gameThread

        gameThread.start()
    }

    override var keyCommands: [UIKeyCommand]? {
        guard presentedViewController == nil else {
            // Don't respond to key presses if another view controller is active.
            return nil
        }

        let upCommand = UIKeyCommand(input: UIKeyCommand.inputUpArrow,
                                     modifierFlags: [],
                                     action: #selector(moveUp(_:)))
        if #available(iOS 9.0, *) {
            upCommand.discoverabilityTitle = "Выбрать предыдущий вариант"
        }
        let downCommand = UIKeyCommand(input: UIKeyCommand.inputDownArrow,
                                       modifierFlags: [],
                                       action: #selector(moveDown(_:)))
        if #available(iOS 9.0, *) {
            downCommand.discoverabilityTitle = "Выбрать следующий вариант"
        }
        let confirmCommand = UIKeyCommand(input: "\r",
                                          modifierFlags: [],
                                          action: #selector(confirm(_:)))
        if #available(iOS 9.0, *) {
            confirmCommand.discoverabilityTitle = "Подтвердить выбор"
        }
        var commands = [
            upCommand,
            downCommand,
            confirmCommand,
        ]


        func registerAnyKeyCommand(_ input: String) {
            commands.append(UIKeyCommand(input: input,
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

        return commands
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

