import UIKit

final class GameView: UIView {

    var content: NSAttributedString?

    var didRedraw: (() -> ())?

    override func draw(_ rect: CGRect) {
        content?.draw(in: rect)
        didRedraw?()
    }
}
