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
        if #available(iOS 13.0, *) {
            closeButton = UIBarButtonItem(barButtonSystemItem: .close,
                                          target: self,
                                          action: #selector(done))
        } else {
            closeButton = UIBarButtonItem(title: "Закрыть",
                                          style: .done,
                                          target: self,
                                          action: #selector(done))
        }

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
        if #available(iOS 9.0, *) {
            exitCommand.discoverabilityTitle = "Убрать информацию"
        }
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
            if #available(iOS 10.0, *) {
                UIApplication.shared.open(url)
            } else {
                UIApplication.shared.openURL(url)
            }
            decisionHandler(.cancel)
        } else {
            decisionHandler(.allow)
        }
    }
}
