import UIKit
import WebKit

final class HelpViewController: UIViewController {

    let webView = WKWebView()

    override func loadView() {
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        webView.navigationDelegate = self

        navigationItem.title = "Информация"

        let closeButton: UIBarButtonItem
        closeButton = UIBarButtonItem(barButtonSystemItem: .close,
                                      target: self,
                                      action: #selector(done))

        navigationItem.rightBarButtonItem = closeButton

        loadHelp()
    }

    private func loadHelp() {
        DispatchQueue.global(qos: .userInitiated).async {
            guard let helpURL = Bundle.main.url(forResource: "help",
                                                withExtension: "html"),
                  let helpHTMLString = try? String(contentsOf: helpURL) else {
                preconditionFailure("missing help file")
            }

            DispatchQueue.main.async {
                self.webView.loadHTMLString(helpHTMLString, baseURL: nil)
            }
        }
    }

    @objc
    private func done() {
        dismiss(animated: true, completion: nil)
    }

    override var keyCommands: [UIKeyCommand]? {

        let exitCommand = UIKeyCommand(input: UIKeyCommand.inputEscape,
                                       modifierFlags: [],
                                       action: #selector(done))
        exitCommand.discoverabilityTitle = "Убрать информацию"
        return [exitCommand]
    }
}

extension HelpViewController: WKNavigationDelegate {
    func webView(_ webView: WKWebView,
                 decidePolicyFor navigationAction: WKNavigationAction,
                 decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
        if navigationAction.navigationType == .linkActivated,
           let url = navigationAction.request.url,
           UIApplication.shared.canOpenURL(url)  {
            UIApplication.shared.open(url)
            decisionHandler(.cancel)
        } else {
            decisionHandler(.allow)
        }
    }
}
