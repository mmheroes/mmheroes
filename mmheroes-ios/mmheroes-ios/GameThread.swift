import UIKit

final class GameThread: Thread {

    private weak var vc: MainSceneViewController?

    private let mainRenderer: UIKitMMHeroesRenderer

    init(vc: MainSceneViewController, mainRenderer: UIKitMMHeroesRenderer) {
        self.vc = vc
        self.mainRenderer = mainRenderer
    }

    override func main() {
        while !Thread.current.isCancelled {
            vc?.gameStateLock.lock()
            guard let gameState = vc?.gameState else {
                // We don't need to unlock the lock here, since vc is already gone,
                // and the lock with it.
                return
            }

            let recordedInputRenderer = RecordedInputRenderer(input: gameState.input)
            let tryRenderer = TryRenderer(primaryRenderer: recordedInputRenderer,
                                          fallbackRenderer: mainRenderer)
            let runner = GameRunner(renderer: tryRenderer)
            vc?.runner = runner
            vc?.gameStateLock.unlock()

            do {
                try runner.run(seed: gameState.seed, mode: MMHEROES_GameMode_God)
            } catch UIKitMMHeroesRenderer.Error.threadCancelled {
                return
            } catch {
                assertionFailure(String(describing: error))
            }
            vc?.gameStateLock.lock()
            vc?.gameState = GameState()
            vc?.runner = nil
            vc?.gameStateLock.unlock()
        }
    }
}
