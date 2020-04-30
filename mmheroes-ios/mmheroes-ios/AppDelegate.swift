import UIKit
import Version

private let applicationInfoKey = "com.jaskiewiczs.mmheroes.applicationInfoKey"

@UIApplicationMain
final class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?

    let applicationInfo = ApplicationInfo(version: Bundle.main.version)

    func application(
        _ application: UIApplication,
        willFinishLaunchingWithOptions
            launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil
    ) -> Bool {
        return true
    }

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions
            launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        return true
    }

    func application(_ application: UIApplication,
                     shouldSaveApplicationState coder: NSCoder) -> Bool {
        do {
            try coder.encode(applicationInfo, forKey: applicationInfoKey)
            return true
        } catch {
            return false
        }
    }

    func application(_ application: UIApplication,
                     shouldRestoreApplicationState coder: NSCoder) -> Bool {
        do {
            let previousRunApplicationInfo =
                try coder.decode(ApplicationInfo.self, forKey: applicationInfoKey)
            return
                previousRunApplicationInfo.version.major == applicationInfo.version.major
        } catch {
            return false
        }
    }
}

