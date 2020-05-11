import UIKit

protocol FeedbackGenerator: AnyObject {
    func prepare()
}

protocol ImpactFeedbackGenerator: FeedbackGenerator {
    func impactOccurred()
}

enum ImpactFeedbackStyle: Int {
    case light = 0
    case medium = 1
    case heavy = 2
    case soft = 3
    case rigid = 4
}

protocol SelectionFeedbackGenerator: FeedbackGenerator {
    func selectionChanged()
}

@available(iOS 10.0, *)
extension UIFeedbackGenerator: FeedbackGenerator {}

@available(iOS 10.0, *)
extension UISelectionFeedbackGenerator: SelectionFeedbackGenerator {}

@available(iOS 10.0, *)
extension UIImpactFeedbackGenerator: ImpactFeedbackGenerator {}

private final class NoopFeedbackGenerator
    : SelectionFeedbackGenerator,
      ImpactFeedbackGenerator
{

    private init() {}

    static let shared = NoopFeedbackGenerator()

    func prepare() {}

    func impactOccurred() {}

    func selectionChanged() {}
}

func makeSelectionFeedbackGenerator() -> SelectionFeedbackGenerator {
    if #available(iOS 10.0, *) {
        return UISelectionFeedbackGenerator()
    } else {
        return NoopFeedbackGenerator.shared
    }
}

func makeImpactFeedbackGenerator(style: ImpactFeedbackStyle) -> ImpactFeedbackGenerator {
    if #available(iOS 10.0, *) {
        let style = UIImpactFeedbackGenerator.FeedbackStyle(rawValue: style.rawValue)!
        return UIImpactFeedbackGenerator(style: style)
    } else {
        return NoopFeedbackGenerator.shared
    }
}
