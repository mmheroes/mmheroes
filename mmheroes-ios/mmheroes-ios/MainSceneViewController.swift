import UIKit

private let gameStateRestorationKey =
    "com.broadwaylamb.mmheroes.ios.gameStateRestorationKey"

final class MainSceneViewController: UIViewController {

    private let gameView = GameView()

    private let helpButton = UIButton(type: .infoDark)

    // Cache this view controller.
    private let helpViewController = HelpViewController()

    private var gameRunner: GameRunner?

    private var gameHasStarted = false

    private let consoleFont: UIFont = .monospacedSystemFont(ofSize: 12, weight: .regular)

    private let selectionFeedbackGenerator = UISelectionFeedbackGenerator()
    private let confirmationFeedbackGenerator = UIImpactFeedbackGenerator(style: .medium)

    override init(nibName nibNameOrNil: String?, bundle nibBundleOrNil: Bundle?) {
        super.init(nibName: nibNameOrNil, bundle: nibBundleOrNil)
        restorationIdentifier = Self.restorationIdentifier
    }

    required init?(coder: NSCoder) {
        super.init(coder: coder)
        restorationIdentifier = Self.restorationIdentifier
    }

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
            startGame(restoredState: nil)
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

    private func setupGameView() {
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

        let swipeUpGestureRecognizer =
            SelectOptionGestureRecognizer { [weak self] gr, direction in
                switch direction {
                case .up:
                    self?.moveUp(gr)
                case .down:
                    self?.moveDown(gr)
                }
            }
        view.addGestureRecognizer(swipeUpGestureRecognizer)
    }

    private func updateGameViewLayout() {

        // Yep, no AutoLayout. We target iOS 8, and the features we need are not available
        // there.

        let minimumContentMargin: CGFloat = 11

        var minX = view.bounds.minX
        var minY = view.bounds.minY
        var maxX = minX + view.bounds.width
        var maxY = minY + view.bounds.height

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

    private func startGame(restoredState: NSCoder?) {
        gameHasStarted = true

        let runner = GameRunner(font: consoleFont) { [weak self] text, caret in
            self?.gameView.text = text
            self?.gameView.caret = caret
            self?.gameView.setNeedsDisplay()
        }

        self.gameRunner = runner

        do {
            if let restoredState = restoredState {
                try runner.restoreGameState(from: restoredState)
                Task.detached {
                    try await runner.render()
                }
                return
            }
        } catch DecodingError.valueNotFound {
            // do nothing
        } catch {
            assertionFailure(String(describing: error))
        }

        Task.detached {
            try await self.continueGame(MMHEROES_Input_Enter)
        }
    }

    override var keyCommands: [UIKeyCommand]? {
        guard presentedViewController == nil else {
            // Don't respond to key presses if another view controller is active.
            return nil
        }

        let upCommand = UIKeyCommand(input: UIKeyCommand.inputUpArrow,
                                     modifierFlags: [],
                                     action: #selector(moveUp(_:)))
        let downCommand = UIKeyCommand(input: UIKeyCommand.inputDownArrow,
                                       modifierFlags: [],
                                       action: #selector(moveDown(_:)))
        let confirmCommand = UIKeyCommand(input: "\r",
                                          modifierFlags: [],
                                          action: #selector(confirm(_:)))
        upCommand.discoverabilityTitle = "Выбрать предыдущий вариант"
        downCommand.discoverabilityTitle = "Выбрать следующий вариант"

        confirmCommand.discoverabilityTitle = "Подтвердить выбор"
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
        if sender is UIGestureRecognizer {
            selectionFeedbackGenerator.selectionChanged()
        }
        Task.detached {
            try await self.continueGame(MMHEROES_Input_KeyUp)
        }
    }

    @IBAction func moveDown(_ sender: Any) {
        if sender is UIGestureRecognizer {
            selectionFeedbackGenerator.selectionChanged()
        }
        Task.detached {
            try await self.continueGame(MMHEROES_Input_KeyDown)
        }
    }

    @IBAction func confirm(_ sender: Any) {
        if sender is UIGestureRecognizer {
            confirmationFeedbackGenerator.impactOccurred()
        }
        Task.detached {
            try await self.continueGame(MMHEROES_Input_Enter)
        }
    }

    @objc func anyKey(_ sender: Any) {
        Task.detached {
            try await self.continueGame(MMHEROES_Input_Other)
        }
    }

    private func continueGame(_ input: MMHEROES_Input) async throws {
        guard let gameRunner = self.gameRunner else { return }
        switch try await gameRunner.continueGame(input: input) {
        case .unexpectedInput:
            break // Do nothing
        case .expectingMoreInput:
            selectionFeedbackGenerator.prepare()
            confirmationFeedbackGenerator.prepare()
        case .gameEnded:
            startGame(restoredState: nil)
        }

        // TODO: Add various haptic feedback for events.
        // For example, vibration on the "ding" screen,
        // failure feedback on death and success feedback on success.
        // Need proper FFI for that.
    }

    override var prefersHomeIndicatorAutoHidden: Bool {
        true
    }

    // MARK: - State restoration

    static let restorationIdentifier = "MainSceneViewController"

    override func encodeRestorableState(with coder: NSCoder) {
        super.encodeRestorableState(with: coder)
        if let gameRunner = self.gameRunner {
            do {
                try gameRunner.encodeGameState(to: coder)
            } catch {
                assertionFailure(String(describing: error))
            }
        }
    }

    override func decodeRestorableState(with coder: NSCoder) {
        startGame(restoredState: coder)
        super.decodeRestorableState(with: coder)
    }
}
