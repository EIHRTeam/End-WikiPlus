package com.eihrteam.wikiplus.android_intent

import android.Manifest
import android.app.Activity
import android.content.ContentValues
import android.content.Intent
import android.content.pm.PackageManager
import android.media.MediaScannerConnection
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.provider.MediaStore
import android.provider.Settings
import android.util.Base64
import android.util.Log
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import java.io.File
import java.io.FileOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.util.Locale

// ═══════════════════════════════════════════════════════════════════
// Typed argument classes (Tauri 2.x @InvokeArg pattern)
// ═══════════════════════════════════════════════════════════════════

@InvokeArg
internal class OpenLinkArgs {
    lateinit var url: String
}

@InvokeArg
internal class SaveMediaArgs {
    lateinit var base64Data: String
    lateinit var fileName: String
    var mimeType: String = "application/octet-stream"
    var target: String = "downloads"
}

@InvokeArg
internal class SaveMediaFromFileArgs {
    lateinit var filePath: String
    lateinit var fileName: String
    var mimeType: String = "application/octet-stream"
    var target: String = "downloads"
}

@InvokeArg
internal class SaveMediaFromUrlArgs {
    lateinit var sourceUrl: String
    lateinit var fileName: String
    var mimeType: String = "application/octet-stream"
    var target: String = "downloads"
}

@TauriPlugin
class AndroidIntentPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        private const val TAG = "AndroidIntentPlugin"
        private const val TARGET_IMAGES = "images"
        private const val TARGET_DOWNLOADS = "downloads"
        private const val REQUEST_WRITE_EXTERNAL_STORAGE = 12091
        private const val REQUEST_MEDIA_PERMISSIONS = 12092
    }

    private val ioScope = CoroutineScope(Dispatchers.IO + SupervisorJob())

    @Command
    fun openLink(invoke: Invoke) {
        val args = try {
            invoke.parseArgs(OpenLinkArgs::class.java)
        } catch (e: Exception) {
            invoke.reject("Invalid arguments: ${e.message}")
            return
        }

        val url = args.url.trim()
        if (url.isEmpty()) {
            invoke.reject("URL is empty")
            return
        }

        try {
            val intent = Intent(Intent.ACTION_VIEW, Uri.parse(url)).apply {
                addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
            }
            activity.startActivity(intent)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Failed to open URL: ${e.message}")
        }
    }

    @Command
    fun saveMedia(invoke: Invoke) {
        val args = try {
            invoke.parseArgs(SaveMediaArgs::class.java)
        } catch (e: Exception) {
            Log.e(TAG, "saveMedia: failed to parse args", e)
            invoke.reject("Invalid arguments: ${e.message}")
            return
        }

        val base64Data = args.base64Data.trim()
        val fileName = sanitizeFileName(args.fileName)
        val mimeType = args.mimeType.trim().ifBlank { "application/octet-stream" }
        val target = args.target.trim().lowercase(Locale.ROOT).ifBlank { TARGET_DOWNLOADS }

        if (base64Data.isEmpty()) {
            invoke.reject("base64Data is empty")
            return
        }
        if (fileName.isNullOrEmpty()) {
            invoke.reject("fileName is required")
            return
        }

        val bytes = try {
            Base64.decode(base64Data, Base64.DEFAULT)
        } catch (e: IllegalArgumentException) {
            invoke.reject("Invalid base64Data: ${e.message}")
            return
        }

        Log.d(TAG, "saveMedia: decoded ${bytes.size} bytes, target=$target, fileName=$fileName")

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            saveMediaLegacy(invoke, bytes, fileName, mimeType, target)
            return
        }

        saveMediaScoped(invoke, bytes, fileName, mimeType, target)
    }

    @Command
    fun saveMediaFromFile(invoke: Invoke) {
        val args = try {
            invoke.parseArgs(SaveMediaFromFileArgs::class.java)
        } catch (e: Exception) {
            Log.e(TAG, "saveMediaFromFile: failed to parse args", e)
            invoke.reject("Invalid arguments: ${e.message}")
            return
        }

        val filePath = args.filePath.trim()
        val fileName = sanitizeFileName(args.fileName)
        val mimeType = args.mimeType.trim().ifBlank { "application/octet-stream" }
        val target = args.target.trim().lowercase(Locale.ROOT).ifBlank { TARGET_DOWNLOADS }

        if (filePath.isEmpty()) {
            invoke.reject("filePath is required")
            return
        }
        if (fileName.isNullOrEmpty()) {
            invoke.reject("fileName is required")
            return
        }

        val file = resolveFilePath(filePath)
        if (file == null || !file.exists() || !file.isFile) {
            invoke.reject("File not found: $filePath (resolved=${file?.absolutePath})")
            return
        }

        val bytes = try {
            file.readBytes()
        } catch (e: Exception) {
            invoke.reject("Failed to read file: ${e.message}")
            return
        }

        // Clean up temp file after reading
        try { file.delete() } catch (_: Exception) {}

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            saveMediaLegacy(invoke, bytes, fileName, mimeType, target)
            return
        }

        saveMediaScoped(invoke, bytes, fileName, mimeType, target)
    }

    /**
     * Resolve a file path: if absolute and exists, use directly;
     * otherwise search common app-internal directories.
     */
    private fun resolveFilePath(path: String): File? {
        // 1) Absolute path — use directly
        val direct = File(path)
        if (direct.isAbsolute && direct.exists()) return direct

        // 2) Relative path — search app-internal directories
        val candidates = listOfNotNull(
            activity.filesDir,
            activity.cacheDir,
            activity.getExternalFilesDir(null),
            activity.externalCacheDir,
        )
        for (base in candidates) {
            val resolved = File(base, path)
            if (resolved.exists() && resolved.isFile) return resolved
        }

        // 3) Also check direct subdirectories
        for (base in candidates) {
            base.listFiles()?.filter { it.isDirectory }?.forEach { subDir ->
                val resolved = File(subDir, path)
                if (resolved.exists() && resolved.isFile) return resolved
            }
        }

        return null
    }

    @Command
    fun saveMediaFromUrl(invoke: Invoke) {
        val args = try {
            invoke.parseArgs(SaveMediaFromUrlArgs::class.java)
        } catch (e: Exception) {
            Log.e(TAG, "saveMediaFromUrl: failed to parse args", e)
            invoke.reject("Invalid arguments: ${e.message}")
            return
        }

        val sourceUrl = args.sourceUrl.trim()
        val fileName = sanitizeFileName(args.fileName)
        val mimeType = args.mimeType.trim().ifBlank { "application/octet-stream" }
        val target = args.target.trim().lowercase(Locale.ROOT).ifBlank { TARGET_DOWNLOADS }

        if (sourceUrl.isEmpty()) {
            invoke.reject("sourceUrl is required")
            return
        }
        if (fileName.isNullOrEmpty()) {
            invoke.reject("fileName is required")
            return
        }

        Log.d(TAG, "saveMediaFromUrl: downloading $sourceUrl -> $fileName (target=$target)")

        // Network I/O on coroutine IO dispatcher (recommended by Tauri 2.x docs)
        ioScope.launch {
            try {
                val bytes = readBytesFromSource(sourceUrl)
                Log.d(TAG, "saveMediaFromUrl: downloaded ${bytes.size} bytes")

                if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
                    saveMediaLegacy(invoke, bytes, fileName, mimeType, target)
                } else {
                    saveMediaScoped(invoke, bytes, fileName, mimeType, target)
                }
            } catch (e: Exception) {
                Log.e(TAG, "saveMediaFromUrl: failed", e)
                invoke.reject("Failed to download and save: ${e.message}")
            }
        }
    }

    private fun saveMediaScoped(
        invoke: Invoke,
        bytes: ByteArray,
        fileName: String,
        mimeType: String,
        target: String,
    ) {
        val (collection, relativePath) = when (target) {
            TARGET_IMAGES -> Pair(
                MediaStore.Images.Media.EXTERNAL_CONTENT_URI,
                "${Environment.DIRECTORY_PICTURES}/Wiki+"
            )
            TARGET_DOWNLOADS -> Pair(
                MediaStore.Downloads.EXTERNAL_CONTENT_URI,
                "${Environment.DIRECTORY_DOWNLOADS}/Wikiplus"
            )
            else -> {
                invoke.reject("Unsupported target: $target")
                return
            }
        }

        val resolver = activity.contentResolver
        val values = ContentValues().apply {
            put(MediaStore.MediaColumns.DISPLAY_NAME, fileName)
            put(MediaStore.MediaColumns.MIME_TYPE, mimeType)
            put(MediaStore.MediaColumns.RELATIVE_PATH, relativePath)
            put(MediaStore.MediaColumns.IS_PENDING, 1)
        }

        var uri = resolver.insert(collection, values)
        if (uri == null && target == TARGET_DOWNLOADS) {
            // Some ROMs reject RELATIVE_PATH for Downloads, retry without it.
            val retryValues = ContentValues(values).apply {
                remove(MediaStore.MediaColumns.RELATIVE_PATH)
            }
            uri = resolver.insert(collection, retryValues)
        }

        if (uri == null) {
            invoke.reject("Failed to create MediaStore entry")
            return
        }

        try {
            resolver.openOutputStream(uri)?.use { outputStream ->
                outputStream.write(bytes)
                outputStream.flush()
            } ?: throw IllegalStateException("Failed to open output stream")

            ContentValues().apply {
                put(MediaStore.MediaColumns.IS_PENDING, 0)
                resolver.update(uri, this, null, null)
            }

            val result = JSObject()
            result.put("uri", uri.toString())
            result.put("fileName", fileName)
            invoke.resolve(result)
        } catch (e: Exception) {
            resolver.delete(uri, null, null)
            invoke.reject("Failed to save media: ${e.message}")
        }
    }

    private fun saveMediaLegacy(
        invoke: Invoke,
        bytes: ByteArray,
        fileName: String,
        mimeType: String,
        target: String,
    ) {
        val permission = Manifest.permission.WRITE_EXTERNAL_STORAGE
        val granted = ContextCompat.checkSelfPermission(activity, permission) == PackageManager.PERMISSION_GRANTED
        if (!granted) {
            ActivityCompat.requestPermissions(activity, arrayOf(permission), REQUEST_WRITE_EXTERNAL_STORAGE)
            invoke.reject("Storage permission requested. Please grant it and retry.")
            return
        }

        val baseDir = when (target) {
            TARGET_IMAGES -> {
                val pictures = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_PICTURES)
                File(pictures, "Wiki+")
            }
            TARGET_DOWNLOADS -> {
                val downloads = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS)
                File(downloads, "Wikiplus")
            }
            else -> {
                invoke.reject("Unsupported target: $target")
                return
            }
        }

        if (!baseDir.exists() && !baseDir.mkdirs()) {
            invoke.reject("Failed to create directory: ${baseDir.absolutePath}")
            return
        }

        val file = File(baseDir, fileName)
        try {
            FileOutputStream(file).use { output ->
                output.write(bytes)
                output.flush()
            }

            MediaScannerConnection.scanFile(
                activity,
                arrayOf(file.absolutePath),
                arrayOf(mimeType),
                null,
            )

            val result = JSObject()
            result.put("uri", Uri.fromFile(file).toString())
            result.put("fileName", fileName)
            invoke.resolve(result)
        } catch (e: Exception) {
            if (file.exists()) {
                file.delete()
            }
            invoke.reject("Failed to save media: ${e.message}")
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Permission Management Commands
    // ═══════════════════════════════════════════════════════════════════

    /**
     * Check the status of media-related permissions.
     * Returns a JSON object with permission names as keys and boolean granted status as values.
     */
    @Command
    override fun checkPermissions(invoke: Invoke) {
        val result = JSObject()

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            // Android 13+ (API 33): granular media permissions
            result.put("readMediaImages", isPermissionGranted(Manifest.permission.READ_MEDIA_IMAGES))
            result.put("readMediaAudio", isPermissionGranted(Manifest.permission.READ_MEDIA_AUDIO))
            result.put("readMediaVideo", isPermissionGranted(Manifest.permission.READ_MEDIA_VIDEO))
            // Write via MediaStore doesn't need permission on Q+
            result.put("writeExternalStorage", true)
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            // Android 10-12 (API 29-32): scoped storage, read still needs READ_EXTERNAL_STORAGE
            val readGranted = isPermissionGranted(Manifest.permission.READ_EXTERNAL_STORAGE)
            result.put("readMediaImages", readGranted)
            result.put("readMediaAudio", readGranted)
            result.put("readMediaVideo", readGranted)
            result.put("writeExternalStorage", true) // MediaStore handles writes
        } else {
            // Android 9 and below: legacy storage
            val writeGranted = isPermissionGranted(Manifest.permission.WRITE_EXTERNAL_STORAGE)
            val readGranted = isPermissionGranted(Manifest.permission.READ_EXTERNAL_STORAGE)
            result.put("readMediaImages", readGranted)
            result.put("readMediaAudio", readGranted)
            result.put("readMediaVideo", readGranted)
            result.put("writeExternalStorage", writeGranted)
        }

        result.put("platform", "android")
        result.put("sdkVersion", Build.VERSION.SDK_INT)
        invoke.resolve(result)
    }

    /**
     * Request media-related permissions.
     * On Android 13+, requests READ_MEDIA_IMAGES, READ_MEDIA_AUDIO, READ_MEDIA_VIDEO.
     * On older versions, requests READ_EXTERNAL_STORAGE (and WRITE_EXTERNAL_STORAGE if < Q).
     */
    @Command
    override fun requestPermissions(invoke: Invoke) {
        val permissions = mutableListOf<String>()

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (!isPermissionGranted(Manifest.permission.READ_MEDIA_IMAGES)) {
                permissions.add(Manifest.permission.READ_MEDIA_IMAGES)
            }
            if (!isPermissionGranted(Manifest.permission.READ_MEDIA_AUDIO)) {
                permissions.add(Manifest.permission.READ_MEDIA_AUDIO)
            }
            if (!isPermissionGranted(Manifest.permission.READ_MEDIA_VIDEO)) {
                permissions.add(Manifest.permission.READ_MEDIA_VIDEO)
            }
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            if (!isPermissionGranted(Manifest.permission.READ_EXTERNAL_STORAGE)) {
                permissions.add(Manifest.permission.READ_EXTERNAL_STORAGE)
            }
        } else {
            if (!isPermissionGranted(Manifest.permission.READ_EXTERNAL_STORAGE)) {
                permissions.add(Manifest.permission.READ_EXTERNAL_STORAGE)
            }
            if (!isPermissionGranted(Manifest.permission.WRITE_EXTERNAL_STORAGE)) {
                permissions.add(Manifest.permission.WRITE_EXTERNAL_STORAGE)
            }
        }

        if (permissions.isEmpty()) {
            // All permissions already granted
            val result = JSObject()
            result.put("allGranted", true)
            result.put("requested", false)
            invoke.resolve(result)
            return
        }

        ActivityCompat.requestPermissions(
            activity,
            permissions.toTypedArray(),
            REQUEST_MEDIA_PERMISSIONS
        )

        val result = JSObject()
        result.put("allGranted", false)
        result.put("requested", true)
        invoke.resolve(result)
    }

    /**
     * Open the system app settings page for this app.
     */
    @Command
    fun openAppSettings(invoke: Invoke) {
        try {
            val intent = Intent(Settings.ACTION_APPLICATION_DETAILS_SETTINGS).apply {
                data = Uri.fromParts("package", activity.packageName, null)
                addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
            }
            activity.startActivity(intent)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject("Failed to open app settings: ${e.message}")
        }
    }

    private fun isPermissionGranted(permission: String): Boolean {
        return ContextCompat.checkSelfPermission(activity, permission) == PackageManager.PERMISSION_GRANTED
    }

    // ═══════════════════════════════════════════════════════════════════

    private fun sanitizeFileName(rawFileName: String?): String? {
        val trimmed = rawFileName?.trim().orEmpty()
        if (trimmed.isEmpty()) {
            return null
        }
        return trimmed.replace(Regex("""[\\/:*?"<>|]"""), "_")
    }

    private fun readBytesFromSource(sourceUrl: String): ByteArray {
        if (sourceUrl.startsWith("asset://", ignoreCase = true) ||
            sourceUrl.startsWith("tauri://", ignoreCase = true) ||
            sourceUrl.startsWith("app://", ignoreCase = true)
        ) {
            val uri = Uri.parse(sourceUrl)
            val candidates = mutableListOf<String>()

            uri.encodedPath?.let { encodedPath ->
                if (encodedPath.isNotBlank()) {
                    val decoded = Uri.decode(encodedPath)
                    if (decoded.isNotBlank()) {
                        candidates.add(decoded)
                        if (decoded.startsWith("/")) {
                            candidates.add(decoded.removePrefix("/"))
                        } else {
                            candidates.add("/$decoded")
                        }
                    }
                }
            }

            uri.getQueryParameter("path")?.let { pathParam ->
                if (pathParam.isNotBlank()) {
                    candidates.add(pathParam)
                    if (!pathParam.startsWith("/")) {
                        candidates.add("/$pathParam")
                    }
                }
            }

            for (candidate in candidates.distinct()) {
                val file = File(candidate)
                if (file.exists() && file.isFile) {
                    return file.inputStream().use { it.readBytes() }
                }
            }

            throw IllegalArgumentException("Unsupported sourceUrl scheme")
        }

        if (sourceUrl.startsWith("content://", ignoreCase = true) ||
            sourceUrl.startsWith("file://", ignoreCase = true)
        ) {
            val uri = Uri.parse(sourceUrl)
            val input = activity.contentResolver.openInputStream(uri)
                ?: throw IllegalStateException("Unable to open source URI")
            return input.use { it.readBytes() }
        }

        if (sourceUrl.startsWith("http://", ignoreCase = true) ||
            sourceUrl.startsWith("https://", ignoreCase = true)
        ) {
            val connection = URL(sourceUrl).openConnection() as HttpURLConnection
            return try {
                connection.instanceFollowRedirects = true
                connection.connectTimeout = 15000
                connection.readTimeout = 30000
                connection.requestMethod = "GET"
                connection.connect()
                val status = connection.responseCode
                if (status !in 200..299) {
                    throw IllegalStateException("HTTP $status")
                }
                connection.inputStream.use { it.readBytes() }
            } finally {
                connection.disconnect()
            }
        }

        if (sourceUrl.startsWith("/")) {
            return File(sourceUrl).inputStream().use { it.readBytes() }
        }

        throw IllegalArgumentException("Unsupported sourceUrl scheme")
    }
}
