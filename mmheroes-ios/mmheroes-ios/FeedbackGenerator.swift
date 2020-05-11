import UIKit

protocol FeedbackGenerator: AnyObject {
    func prepare()
}

protocol SelectionFeedbackGenerator: FeedbackGenerator {
    func selectionChanged()
}

@available(iOS 10.0, *)
extension UIFeedbackGenerator: FeedbackGenerator {}

@available(iOS 10.0, *)
extension UISelectionFeedbackGenerator: SelectionFeedbackGenerator {}

private final class NoopFeedbackGenerator: SelectionFeedbackGenerator {

    private init() {}

    static let shared = NoopFeedbackGenerator()

    func selectionChanged() {}
    func prepare() {}
}

func makeSelectionFeedbackGenerator() -> SelectionFeedbackGenerator {
    if #available(iOS 10.0, *) {
        return UISelectionFeedbackGenerator()
    } else {
        return NoopFeedbackGenerator.shared
    }
}
