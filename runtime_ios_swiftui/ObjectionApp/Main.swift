import SwiftUI

@main
struct Main: App {
    var body: some Scene {
        WindowGroup {
            GlobalProvider {
                Root()
            }
        }
    }
}
