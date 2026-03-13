import Foundation
import Photos
import Tauri
import UIKit

class OpenLinkArgs: Decodable {
  let url: String
}

class SaveMediaFromFileArgs: Decodable {
  let filePath: String
  let fileName: String?
  let mimeType: String?
  let target: String?
}

class SaveMediaArgs: Decodable {
  let base64Data: String
  let fileName: String?
  let mimeType: String?
  let target: String?
}

class SaveMediaFromUrlArgs: Decodable {
  let sourceUrl: String
  let fileName: String?
  let mimeType: String?
  let target: String?
}

class AndroidIntentPlugin: Plugin {
  @objc public func openLink(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(OpenLinkArgs.self)
    guard let url = URL(string: args.url.trimmingCharacters(in: .whitespacesAndNewlines)) else {
      invoke.reject("Invalid URL")
      return
    }

    DispatchQueue.main.async {
      UIApplication.shared.open(url, options: [:]) { success in
        if success {
          invoke.resolve()
        } else {
          invoke.reject("Failed to open URL")
        }
      }
    }
  }

  @objc public func saveMediaFromFile(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(SaveMediaFromFileArgs.self)

    let filePath = args.filePath.trimmingCharacters(in: .whitespacesAndNewlines)
    if filePath.isEmpty {
      invoke.reject("filePath is required")
      return
    }

    let fileName = sanitizeFileName(args.fileName)
    if fileName.isEmpty {
      invoke.reject("fileName is required")
      return
    }

    let sourceURL = URL(fileURLWithPath: filePath)
    if !FileManager.default.fileExists(atPath: sourceURL.path) {
      invoke.reject("File not found: \(filePath)")
      return
    }

    let mimeType = (args.mimeType ?? "application/octet-stream").lowercased()
    let target = (args.target ?? "downloads").lowercased()
    let saveToPhotos = target == "images" || mimeType.hasPrefix("image/") || mimeType.hasPrefix("video/")

    if saveToPhotos {
      saveToPhotosLibrary(invoke: invoke, sourceURL: sourceURL, fileName: fileName, mimeType: mimeType)
    } else {
      saveToDocuments(invoke: invoke, sourceURL: sourceURL, fileName: fileName)
    }
  }

  /// Save media from base64-encoded data. Called directly from JS on mobile.
  @objc public func saveMedia(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(SaveMediaArgs.self)

    let base64Data = args.base64Data.trimmingCharacters(in: .whitespacesAndNewlines)
    guard !base64Data.isEmpty, let data = Data(base64Encoded: base64Data) else {
      invoke.reject("base64Data is required or invalid")
      return
    }

    let fileName = sanitizeFileName(args.fileName)
    if fileName.isEmpty {
      invoke.reject("fileName is required")
      return
    }

    let mimeType = (args.mimeType ?? "application/octet-stream").lowercased()
    let target = (args.target ?? "downloads").lowercased()

    // Write to temp file, then use shared save logic
    let tempDir = FileManager.default.temporaryDirectory
    let tempURL = tempDir.appendingPathComponent("_swift_media_\(Int(Date().timeIntervalSince1970 * 1000))_\(fileName)")

    do {
      try data.write(to: tempURL)
    } catch {
      invoke.reject("Failed to write temp file: \(error.localizedDescription)")
      return
    }

    let saveToPhotos = target == "images" || mimeType.hasPrefix("image/") || mimeType.hasPrefix("video/")
    if saveToPhotos {
      saveToPhotosLibrary(invoke: invoke, sourceURL: tempURL, fileName: fileName, mimeType: mimeType)
    } else {
      saveToDocuments(invoke: invoke, sourceURL: tempURL, fileName: fileName)
    }
  }

  /// Save media by downloading from a URL. Called directly from JS on mobile.
  @objc public func saveMediaFromUrl(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(SaveMediaFromUrlArgs.self)

    let sourceUrl = args.sourceUrl.trimmingCharacters(in: .whitespacesAndNewlines)
    guard !sourceUrl.isEmpty, let url = URL(string: sourceUrl) else {
      invoke.reject("sourceUrl is required")
      return
    }

    let fileName = sanitizeFileName(args.fileName)
    if fileName.isEmpty {
      invoke.reject("fileName is required")
      return
    }

    let mimeType = (args.mimeType ?? "application/octet-stream").lowercased()
    let target = (args.target ?? "downloads").lowercased()

    // Download on background thread
    DispatchQueue.global(qos: .userInitiated).async {
      do {
        let data = try Data(contentsOf: url)
        let tempDir = FileManager.default.temporaryDirectory
        let tempURL = tempDir.appendingPathComponent("_swift_media_\(Int(Date().timeIntervalSince1970 * 1000))_\(fileName)")
        try data.write(to: tempURL)

        let saveToPhotos = target == "images" || mimeType.hasPrefix("image/") || mimeType.hasPrefix("video/")
        if saveToPhotos {
          self.saveToPhotosLibrary(invoke: invoke, sourceURL: tempURL, fileName: fileName, mimeType: mimeType)
        } else {
          self.saveToDocuments(invoke: invoke, sourceURL: tempURL, fileName: fileName)
        }
      } catch {
        invoke.reject("Failed to download: \(error.localizedDescription)")
      }
    }
  }

  // ═══════════════════════════════════════════════════════════════════
  // Permission Management Commands
  // ═══════════════════════════════════════════════════════════════════

  @objc override public func checkPermissions(_ invoke: Invoke) {
    var payload: [String: Any] = [
      "platform": "ios"
    ]

    // Check Photo Library permission
    if #available(iOS 14, *) {
      let status = PHPhotoLibrary.authorizationStatus(for: .addOnly)
      payload["photoLibraryAddOnly"] = (status == .authorized || status == .limited)
      let readStatus = PHPhotoLibrary.authorizationStatus(for: .readWrite)
      payload["photoLibraryReadWrite"] = (readStatus == .authorized || readStatus == .limited)
    } else {
      let status = PHPhotoLibrary.authorizationStatus()
      let granted = (status == .authorized)
      payload["photoLibraryAddOnly"] = granted
      payload["photoLibraryReadWrite"] = granted
    }

    // On iOS, file saving to Documents doesn't need special permission
    payload["fileAccess"] = true

    invoke.resolve(payload)
  }

  @objc override public func requestPermissions(_ invoke: Invoke) {
    if #available(iOS 14, *) {
      let currentStatus = PHPhotoLibrary.authorizationStatus(for: .addOnly)
      if currentStatus == .authorized || currentStatus == .limited {
        invoke.resolve([
          "allGranted": true,
          "requested": false
        ])
        return
      }

      PHPhotoLibrary.requestAuthorization(for: .addOnly) { status in
        invoke.resolve([
          "allGranted": (status == .authorized || status == .limited),
          "requested": true
        ])
      }
    } else {
      let currentStatus = PHPhotoLibrary.authorizationStatus()
      if currentStatus == .authorized {
        invoke.resolve([
          "allGranted": true,
          "requested": false
        ])
        return
      }

      PHPhotoLibrary.requestAuthorization { status in
        invoke.resolve([
          "allGranted": (status == .authorized),
          "requested": true
        ])
      }
    }
  }

  @objc public func openAppSettings(_ invoke: Invoke) throws {
    DispatchQueue.main.async {
      guard let settingsURL = URL(string: UIApplication.openSettingsURLString) else {
        invoke.reject("Cannot create settings URL")
        return
      }
      UIApplication.shared.open(settingsURL, options: [:]) { success in
        if success {
          invoke.resolve()
        } else {
          invoke.reject("Failed to open app settings")
        }
      }
    }
  }

  // ═══════════════════════════════════════════════════════════════════

  private func saveToPhotosLibrary(invoke: Invoke, sourceURL: URL, fileName: String, mimeType: String) {
    let persistMedia: () -> Void = {
      var placeholder: PHObjectPlaceholder?
      PHPhotoLibrary.shared().performChanges {
        let request = PHAssetCreationRequest.forAsset()
        if mimeType.hasPrefix("video/") {
          request.addResource(with: .video, fileURL: sourceURL, options: nil)
        } else {
          request.addResource(with: .photo, fileURL: sourceURL, options: nil)
        }
        placeholder = request.placeholderForCreatedAsset
      } completionHandler: { success, error in
        if success {
          try? FileManager.default.removeItem(at: sourceURL)
          var payload: [String: Any] = [
            "fileName": fileName
          ]
          if let localIdentifier = placeholder?.localIdentifier {
            payload["uri"] = "ph://\(localIdentifier)"
          }
          invoke.resolve(payload)
        } else {
          invoke.reject(error?.localizedDescription ?? "Failed to save media to Photos")
        }
      }
    }

    if #available(iOS 14, *) {
      let authorizationHandler: (PHAuthorizationStatus) -> Void = { status in
        guard status == .authorized || status == .limited else {
          invoke.reject("Photo Library permission denied")
          return
        }

        persistMedia()
      }

      let status = PHPhotoLibrary.authorizationStatus(for: .addOnly)
      if status == .notDetermined {
        PHPhotoLibrary.requestAuthorization(for: .addOnly, handler: authorizationHandler)
      } else {
        authorizationHandler(status)
      }
    } else {
      let authorizationHandler: (PHAuthorizationStatus) -> Void = { status in
        guard status == .authorized else {
          invoke.reject("Photo Library permission denied")
          return
        }

        persistMedia()
      }

      let status = PHPhotoLibrary.authorizationStatus()
      if status == .notDetermined {
        PHPhotoLibrary.requestAuthorization(authorizationHandler)
      } else {
        authorizationHandler(status)
      }
    }
  }

  private func saveToDocuments(invoke: Invoke, sourceURL: URL, fileName: String) {
    guard let documentsDir = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first else {
      invoke.reject("Cannot resolve documents directory")
      return
    }

    let destinationURL = uniqueDestinationURL(baseDir: documentsDir, fileName: fileName)

    do {
      if FileManager.default.fileExists(atPath: destinationURL.path) {
        try FileManager.default.removeItem(at: destinationURL)
      }
      try FileManager.default.copyItem(at: sourceURL, to: destinationURL)
      try? FileManager.default.removeItem(at: sourceURL)

      invoke.resolve([
        "uri": destinationURL.absoluteString,
        "fileName": destinationURL.lastPathComponent,
      ])
    } catch {
      invoke.reject("Failed to save file: \(error.localizedDescription)")
    }
  }

  private func uniqueDestinationURL(baseDir: URL, fileName: String) -> URL {
    let original = baseDir.appendingPathComponent(fileName)
    if !FileManager.default.fileExists(atPath: original.path) {
      return original
    }

    let ext = original.pathExtension
    let stem = original.deletingPathExtension().lastPathComponent
    var counter = 1

    while true {
      let candidateName: String
      if ext.isEmpty {
        candidateName = "\(stem)_\(counter)"
      } else {
        candidateName = "\(stem)_\(counter).\(ext)"
      }
      let candidate = baseDir.appendingPathComponent(candidateName)
      if !FileManager.default.fileExists(atPath: candidate.path) {
        return candidate
      }
      counter += 1
    }
  }

  private func sanitizeFileName(_ raw: String?) -> String {
    let trimmed = (raw ?? "").trimmingCharacters(in: .whitespacesAndNewlines)
    guard !trimmed.isEmpty else { return "" }
    let invalid = CharacterSet(charactersIn: "\\/:*?\"<>|")
    let components = trimmed.components(separatedBy: invalid)
    let result = components.joined(separator: "_")
    return result.isEmpty ? "file_\(Int(Date().timeIntervalSince1970 * 1000))" : result
  }
}

@_cdecl("init_plugin_android_intent")
func initPlugin() -> Plugin {
  return AndroidIntentPlugin()
}
