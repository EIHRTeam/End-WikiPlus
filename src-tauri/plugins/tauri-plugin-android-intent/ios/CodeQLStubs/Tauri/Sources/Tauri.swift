import Foundation

public enum TauriStubError: Error {
  case unavailable
}

@objc open class Invoke: NSObject {
  @objc public func reject(_ message: String) {}

  @objc public func resolve() {}

  @objc public func resolve(_ payload: [String: Any]) {}

  open func parseArgs<T: Decodable>(_ type: T.Type) throws -> T {
    throw TauriStubError.unavailable
  }
}

@objc open class Plugin: NSObject {
  public override init() {
    super.init()
  }

  @objc open func checkPermissions(_ invoke: Invoke) {}

  @objc open func requestPermissions(_ invoke: Invoke) {}
}
