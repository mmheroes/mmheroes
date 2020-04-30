import UIKit

private let terminalWindow = (0 ..< UIKitMMHeroesRenderer.numberOfLines).map { _ in
    String(repeating: " ", count: UIKitMMHeroesRenderer.numberOfColumns)
}.joined(separator: "\n") as NSString

final class GameView: UIView {

    var font: UIFont?
    var content: NSAttributedString?

    var didRedraw: (() -> ())?

    override func draw(_ rect: CGRect) {
        guard let context = UIGraphicsGetCurrentContext(),
              let text = self.content,
              let font = self.font else { return }

        let terminalWindowBoundingBox = terminalWindow
            .size(withAttributes: [.font : font])

        let factorX = bounds.width / terminalWindowBoundingBox.width
        let factorY = bounds.height / terminalWindowBoundingBox.height
        let factor = min(factorX, factorY)

        context.saveGState()
        context.textMatrix = .identity
        context.scaleBy(x: factor, y: factor)
        text.draw(at: rect.origin)
        context.restoreGState()
        didRedraw?()
    }
}
