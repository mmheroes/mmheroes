import UIKit

private let threshold: CGFloat = 12

final class SelectOptionGestureRecognizer: UIPanGestureRecognizer {

    enum Direction {
        case up
        case down
    }

    private class Target: NSObject {
        private var previousY: CGFloat = 0

        @objc
        func callCallback(_ gestureRecognizer: SelectOptionGestureRecognizer) {

            let y = gestureRecognizer.location(in: gestureRecognizer.view).y

            if gestureRecognizer.state == .began {
                self.previousY = y
            }

            guard let viewHeight = gestureRecognizer.view?.bounds.height else { return }

            let delta = y - previousY

            if abs(delta) > viewHeight / threshold {
                self.previousY = y
                gestureRecognizer.callback(gestureRecognizer, delta > 0 ? .down : .up)
            }
        }
    }

    private let callback: (SelectOptionGestureRecognizer, Direction) -> Void

    private let target = Target()

    init(_ callback: @escaping (SelectOptionGestureRecognizer, Direction) -> Void) {
        self.callback = callback
        super.init(target: target, action: #selector(Target.callCallback(_:)))
    }
}
