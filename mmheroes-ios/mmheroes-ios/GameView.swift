import UIKit

private let terminalWindow = (0 ..< UIKitMMHeroesRenderer.numberOfLines).map { _ in
    String(repeating: " ", count: UIKitMMHeroesRenderer.numberOfColumns)
}.joined(separator: "\n") as NSString

final class GameView: UIView {

    var font: UIFont?
    var text: NSAttributedString?

    fileprivate let caretLayer = CaretLayer()

    var caret: Caret {
        get {
            caretLayer.caret
        }
        set {
            caretLayer.caret = newValue
        }
    }

    private var blinkingTimer: Timer?

    var caretHidden: Bool {
        get {
            caretLayer.isHidden
        }
        set {
            caretLayer.isHidden = newValue
        }
    }

    var didRedraw: (() -> ())?

    override func draw(_ rect: CGRect) {
        guard let font = self.font else { return }

        let terminalWindowBoundingBox = terminalWindow
            .size(withAttributes: [.font : font])

        drawText(in: rect, terminalWindowBoundingBox)
        drawCaret(in: rect, terminalWindowBoundingBox)
        didRedraw?()
    }

    var desiredAspectRatio: CGFloat {
        guard let font = self.font else {
            assertionFailure("Font should be set")
            return 1.724
        }
        let terminalWindowBoundingBox = terminalWindow
            .size(withAttributes: [.font : font])
        return terminalWindowBoundingBox.width / terminalWindowBoundingBox.height
    }

    private func scaleFactor(_ terminalWindowBoundingBox: CGSize) -> CGFloat {
        let factorX = bounds.width / terminalWindowBoundingBox.width
        let factorY = bounds.height / terminalWindowBoundingBox.height
        return min(factorX, factorY)
    }

    private func drawText(in rect: CGRect, _ terminalWindowBoundingBox: CGSize) {
        guard let context = UIGraphicsGetCurrentContext(),
              let text = self.text else { return }
        let factor = scaleFactor(terminalWindowBoundingBox)
        context.saveGState()
        context.textMatrix = .identity
        context.scaleBy(x: factor, y: factor)
        text.draw(at: rect.origin)
        context.restoreGState()
    }

    private func drawCaret(in rect: CGRect, _ terminalWindowBoundingBox: CGSize) {
        if blinkingTimer == nil {
            let timer = Timer.scheduledTimer(timeInterval: 0.125,
                                             target: caretLayer,
                                             selector: #selector(CaretLayer.blink(_:)),
                                             userInfo: nil,
                                             repeats: true)
            self.blinkingTimer = timer
            timer.fire()
        }

        let columnHeight = terminalWindowBoundingBox.width /
            CGFloat(UIKitMMHeroesRenderer.numberOfColumns)

        let lineHeight = terminalWindowBoundingBox.height /
            CGFloat(UIKitMMHeroesRenderer.numberOfLines)

        let factor = scaleFactor(terminalWindowBoundingBox)
        let transform = CGAffineTransform(scaleX: factor, y: factor)

        let caretSize = CGSize(width: columnHeight, height: lineHeight / 10)
            .applying(transform)

        let caretPosition = CGPoint(
            x: columnHeight * CGFloat(caret.column),
            y: lineHeight * CGFloat(caret.line) + lineHeight - caretSize.height
        ).applying(transform)

        let caretRect = CGRect(origin: caretPosition, size: caretSize)
        caretRect.applying(CGAffineTransform(scaleX: factor, y: factor))
        caretLayer.path = CGPath(rect: caretRect, transform: nil)

        if caretLayer.superlayer == nil {
            self.layer.addSublayer(caretLayer)
        }
    }
}

private class CaretLayer: CAShapeLayer {

    var caret = Caret()

    @objc
    func blink(_ timer: Timer) {
        CATransaction.begin()
        // By default changes to CALayer's properties are animated.
        // We want the layer to just blink, not fade,
        // so we disable the default behavior.
        CATransaction.setDisableActions(true)
        if fillColor == nil {
            fillColor = self.caret.color.makeUIColor().cgColor
        } else {
            fillColor = nil
        }
        CATransaction.commit()
    }
}
