use aes::cipher::{BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use cbc::Encryptor;
use des::Des;
use flate2::write::GzEncoder;
use flate2::Compression;
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use num_bigint_dig::BigUint;
use rand::rngs::OsRng;
use rand::RngCore;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::io::Write;
#[cfg(debug_assertions)]
use std::thread;
#[cfg(debug_assertions)]
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const SKLAND_ORGANIZATION: &str = "UWXspnCCJN4sfYlNfqps";
const SKLAND_APP_ID: &str = "default";
// 固定 UA：Wiki API 与数美 DID 请求必须使用同一值，禁止改为动态获取或其它 UA。
const SKLAND_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36";
const SKLAND_RSA_MODULUS_HEX: &str = concat!(
    "a6c4c36bee7f19793d2d13547d8ff98f8b1a229cf91e33e938546227fecd24ec",
    "de6b2407e767478c111f9087016fb21ce8d0b36679492947cb88954fc6115afc",
    "98ddbcb0a9b0776829ca0aa3d1e3e7066396e2fd195706d260afa2ce4855164d",
    "95fdda0ba0c6d6cba6fe52703c9a48ef8348985910727d1b0ada0765ee596a0b",
);
const SKLAND_RSA_PUBLIC_EXPONENT: u32 = 65_537;
const SKLAND_RSA_MODULUS_LEN: usize = 128;
const SKLAND_API_HOST: &str = "https://zonai.skland.com";

#[cfg(target_os = "android")]
mod android_entry {
    use super::run;
    use tauri::{
        handle_android_plugin_response, send_channel_data,
        tao::platform::android::prelude::{
            android_fn, create as tao_create, destroy as tao_destroy, focus as tao_focus,
            memory as tao_memory, pause as tao_pause, resume as tao_resume, save as tao_save,
            start as tao_start, stop as tao_stop, JClass, JNIEnv, JObject, JString, PACKAGE,
        },
        wry::{
            android_setup,
            prelude::{
                assetLoaderDomain as wry_asset_loader_domain,
                handleReceivedTitle as wry_handle_received_title,
                handleRequest as wry_handle_request, ipc as wry_ipc, jboolean, jint, jobject,
                jstring, onActivityDestroy as wry_on_activity_destroy, onEval as wry_on_eval,
                onPageLoaded as wry_on_page_loaded, onPageLoading as wry_on_page_loading,
                shouldOverride as wry_should_override, withAssetLoader as wry_with_asset_loader,
            },
        },
    };

    const PACKAGE_NAME: &str = "com/eihrteam/wikiplus/pub";

    fn ensure_package_name() {
        PACKAGE.get_or_init(|| PACKAGE_NAME);
    }

    fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("attempt to unwind out of `rust` with err: {:?}", error);
                std::process::abort()
            }
        }
    }

    fn start_app_inner() {
        ensure_package_name();
        stop_unwind(run);
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_create"]
    pub unsafe extern "C" fn java_wry_activity_create(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        ensure_package_name();
        unsafe {
            tao_create(env, class, activity, android_setup, start_app_inner);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_start"]
    pub unsafe extern "C" fn java_wry_activity_start(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            tao_start(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_stop"]
    pub unsafe extern "C" fn java_wry_activity_stop(env: JNIEnv, class: JClass, activity: JObject) {
        unsafe {
            tao_stop(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_resume"]
    pub unsafe extern "C" fn java_wry_activity_resume(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            tao_resume(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_pause"]
    pub unsafe extern "C" fn java_wry_activity_pause(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            tao_pause(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_save"]
    pub unsafe extern "C" fn java_wry_activity_save(env: JNIEnv, class: JClass, activity: JObject) {
        unsafe {
            tao_save(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_destroy"]
    pub unsafe extern "C" fn java_wry_activity_destroy(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            tao_destroy(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_memory"]
    pub unsafe extern "C" fn java_wry_activity_memory(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            tao_memory(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_focus"]
    pub unsafe extern "C" fn java_wry_activity_focus(env: JNIEnv, class: JClass, has_focus: i32) {
        unsafe {
            tao_focus(env, class, has_focus);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_WryActivity_onActivityDestroy"]
    pub unsafe extern "C" fn java_wry_activity_on_activity_destroy(
        env: JNIEnv,
        class: JClass,
        activity: JObject,
    ) {
        unsafe {
            wry_on_activity_destroy(env, class, activity);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_handleRequest"]
    pub unsafe extern "C" fn java_rust_web_view_client_handle_request(
        env: JNIEnv,
        class: JClass,
        webview_id: JString,
        request: JObject,
        is_document_start_script_enabled: jboolean,
    ) -> jobject {
        unsafe {
            wry_handle_request(
                env,
                class,
                webview_id,
                request,
                is_document_start_script_enabled,
            )
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_withAssetLoader"]
    pub unsafe extern "C" fn java_rust_web_view_client_with_asset_loader(
        env: JNIEnv,
        class: JClass,
    ) -> jboolean {
        unsafe { wry_with_asset_loader(env, class) }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_assetLoaderDomain"]
    pub unsafe extern "C" fn java_rust_web_view_client_asset_loader_domain(
        env: JNIEnv,
        class: JClass,
    ) -> jstring {
        unsafe { wry_asset_loader_domain(env, class) }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_shouldOverride"]
    pub unsafe extern "C" fn java_rust_web_view_client_should_override(
        env: JNIEnv,
        class: JClass,
        url: JString,
    ) -> jboolean {
        unsafe { wry_should_override(env, class, url) }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebView_shouldOverride"]
    pub unsafe extern "C" fn java_rust_web_view_should_override(
        env: JNIEnv,
        class: JClass,
        url: JString,
    ) -> jboolean {
        unsafe { wry_should_override(env, class, url) }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebView_onEval"]
    pub unsafe extern "C" fn java_rust_web_view_on_eval(
        env: JNIEnv,
        class: JClass,
        id: jint,
        result: JString,
    ) {
        unsafe {
            wry_on_eval(env, class, id, result);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_onPageLoading"]
    pub unsafe extern "C" fn java_rust_web_view_client_on_page_loading(
        env: JNIEnv,
        class: JClass,
        url: JString,
    ) {
        unsafe {
            wry_on_page_loading(env, class, url);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebViewClient_onPageLoaded"]
    pub unsafe extern "C" fn java_rust_web_view_client_on_page_loaded(
        env: JNIEnv,
        class: JClass,
        url: JString,
    ) {
        unsafe {
            wry_on_page_loaded(env, class, url);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_Ipc_ipc"]
    pub unsafe extern "C" fn java_ipc_ipc(env: JNIEnv, class: JClass, url: JString, body: JString) {
        unsafe {
            wry_ipc(env, class, url, body);
        }
    }

    #[export_name = "Java_com_eihrteam_wikiplus_pub_RustWebChromeClient_handleReceivedTitle"]
    pub unsafe extern "C" fn java_rust_web_chrome_client_handle_received_title(
        env: JNIEnv,
        class: JClass,
        webview: JObject,
        title: JString,
    ) {
        unsafe {
            wry_handle_received_title(env, class, webview, title);
        }
    }

    android_fn!(
        app_tauri,
        plugin,
        PluginManager,
        handlePluginResponse,
        [i32, JString, JString],
    );
    android_fn!(
        app_tauri,
        plugin,
        PluginManager,
        sendChannelData,
        [i64, JString],
    );

    #[allow(non_snake_case)]
    pub fn handlePluginResponse(
        mut env: JNIEnv,
        _: JClass,
        id: i32,
        success: JString,
        error: JString,
    ) {
        handle_android_plugin_response(&mut env, id, success, error);
    }

    #[allow(non_snake_case)]
    pub fn sendChannelData(mut env: JNIEnv, _: JClass, id: i64, data: JString) {
        send_channel_data(&mut env, id, data);
    }
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct SignHeaders {
    platform: String,
    timestamp: String,
    dId: String,
    vName: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SklandCommandResponse {
    result: bool,
    status: i32,
    statusCode: i32,
    msg: String,
    data: Option<Value>,
}

fn build_command_response(
    result: bool,
    status: i32,
    status_code: i32,
    msg: String,
    data: Option<Value>,
) -> SklandCommandResponse {
    SklandCommandResponse {
        result,
        status,
        statusCode: status_code,
        msg,
        data,
    }
}

fn parse_skland_api_json(status_code: i32, raw: &str) -> SklandCommandResponse {
    let parsed: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => {
            return build_command_response(
                false,
                status_code,
                status_code,
                if raw.is_empty() {
                    "Invalid response body".to_string()
                } else {
                    raw.to_string()
                },
                None,
            )
        }
    };

    let obj = match parsed.as_object() {
        Some(v) => v,
        None => {
            return build_command_response(
                (200..300).contains(&status_code),
                if (200..300).contains(&status_code) {
                    0
                } else {
                    status_code
                },
                status_code,
                if (200..300).contains(&status_code) {
                    String::new()
                } else {
                    "Unexpected response format".to_string()
                },
                Some(parsed),
            )
        }
    };

    let code = obj.get("code").and_then(Value::as_i64).map(|v| v as i32);
    let status = obj.get("status").and_then(Value::as_i64).map(|v| v as i32);
    let msg = obj
        .get("msg")
        .and_then(Value::as_str)
        .or_else(|| obj.get("message").and_then(Value::as_str))
        .unwrap_or_default()
        .to_string();
    let data = obj.get("data").cloned();

    if let Some(code_value) = code {
        return build_command_response(
            code_value == 0,
            status.unwrap_or(code_value),
            status_code,
            msg,
            data,
        );
    }

    if let Some(status_value) = status {
        return build_command_response(status_value == 0, status_value, status_code, msg, data);
    }

    let is_http_ok = (200..300).contains(&status_code);
    build_command_response(
        is_http_ok,
        if is_http_ok { 0 } else { status_code },
        status_code,
        msg,
        data.or(Some(parsed)),
    )
}

fn default_skland_headers(request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    request
        .header("User-Agent", SKLAND_USER_AGENT)
        .header("Referer", "https://wiki.skland.com/")
        .header("Origin", "https://wiki.skland.com")
        .header("Accept", "*/*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,ja;q=0.7")
        .header("Cache-Control", "no-cache")
}

async fn send_skland_request(request: reqwest::RequestBuilder) -> SklandCommandResponse {
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(error) => {
            return build_command_response(false, -1, -1, error.to_string(), None);
        }
    };

    let status_code = response.status().as_u16() as i32;
    let text = match response.text().await {
        Ok(text) => text,
        Err(error) => {
            return build_command_response(
                false,
                status_code,
                status_code,
                error.to_string(),
                None,
            );
        }
    };

    parse_skland_api_json(status_code, &text)
}

fn generate_skland_sign(path: &str, data: &str, headers: &SignHeaders, token: &str) -> String {
    let mut s = String::from(path);
    s.push_str(data);
    s.push_str(&headers.timestamp);

    // 序列化 headers，必须保证字段顺序
    // serde_json 默认不保证顺序，但我们可以手动构建字符串或者使用 preserve_order feature (如果开启)
    // 为了稳健，手动构建 JSON 字符串以匹配 JS 逻辑
    // JS: JSON.stringify({ platform, timestamp, dId, vName })
    let headers_json = format!(
        r#"{{"platform":"{}","timestamp":"{}","dId":"{}","vName":"{}"}}"#,
        headers.platform, headers.timestamp, headers.dId, headers.vName
    );
    s.push_str(&headers_json);

    // HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(token.as_bytes()).expect("HMAC can take key of any size");
    mac.update(s.as_bytes());
    let result = mac.finalize();
    let hmac_hex = hex::encode(result.into_bytes());

    // MD5
    let mut hasher = Md5::new();
    hasher.update(hmac_hex.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn current_unix_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

fn build_skland_error_message(response: &SklandCommandResponse) -> String {
    if !response.msg.trim().is_empty() {
        return response.msg.clone();
    }

    if response.status > 0 {
        return format!("Skland API returned code {}", response.status);
    }

    if response.statusCode > 0 {
        return format!("Request failed with status {}", response.statusCode);
    }

    "Skland API request failed".to_string()
}

fn is_skland_auth_error(status: i32) -> bool {
    matches!(status, 10001..=10004)
}

fn random_public_cred() -> String {
    Uuid::new_v4().simple().to_string()
}

fn extract_token_from_value(value: &Value) -> Option<String> {
    match value {
        Value::String(v) => {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Value::Object(map) => {
            for key in ["token", "accessToken", "content"] {
                if let Some(token) = map.get(key).and_then(Value::as_str) {
                    let trimmed = token.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }

            map.get("data").and_then(extract_token_from_value)
        }
        _ => None,
    }
}

fn extract_token_from_response(response: &SklandCommandResponse) -> Option<String> {
    response.data.as_ref().and_then(extract_token_from_value)
}

async fn execute_wiki_request(
    request: reqwest::RequestBuilder,
) -> Result<String, SklandCommandResponse> {
    let response = request
        .send()
        .await
        .map_err(|error| build_command_response(false, -1, -1, error.to_string(), None))?;
    let status_code = response.status().as_u16() as i32;
    let text = response.text().await.map_err(|error| {
        build_command_response(false, status_code, status_code, error.to_string(), None)
    })?;
    let parsed = parse_skland_api_json(status_code, &text);

    if parsed.result {
        Ok(text)
    } else {
        Err(parsed)
    }
}

async fn refresh_skland_public_token(client: &Client, d_id: &str) -> Result<String, String> {
    let trimmed_did = d_id.trim();
    if trimmed_did.is_empty() {
        return Err("Device ID (dId) is required".to_string());
    }

    let path = "/web/v1/auth/refresh";
    let response = send_skland_request(
        default_skland_headers(client.get(format!("{SKLAND_API_HOST}{path}")))
            .header("Did", trimmed_did),
    )
    .await;

    if !response.result {
        return Err(build_skland_error_message(&response));
    }

    extract_token_from_response(&response)
        .ok_or_else(|| "Refresh token response missing token".to_string())
}

async fn perform_signed_wiki_get(
    client: &Client,
    path: &str,
    query: &str,
    public_cred: &str,
    sign_token: &str,
    d_id: &str,
) -> Result<String, SklandCommandResponse> {
    let url = if query.is_empty() {
        format!("{SKLAND_API_HOST}{path}")
    } else {
        format!("{SKLAND_API_HOST}{path}?{query}")
    };
    let timestamp = current_unix_timestamp();
    let headers = SignHeaders {
        platform: "3".to_string(),
        timestamp: timestamp.clone(),
        dId: d_id.to_string(),
        vName: "1.0.0".to_string(),
    };
    let sign = generate_skland_sign(path, query, &headers, sign_token);

    execute_wiki_request(
        default_skland_headers(client.get(url))
            .header("Cred", public_cred)
            .header("Did", d_id)
            .header("Sign", sign)
            .header("Timestamp", timestamp)
            .header("Platform", "3")
            .header("Vname", "1.0.0")
            .header("Content-Type", "application/json"),
    )
    .await
}

fn build_catalog_url_and_query(
    main_id: Option<&str>,
    sub_id: Option<&str>,
) -> Result<(String, String), String> {
    let base = "https://zonai.skland.com/web/v1/wiki/item/catalog";
    let mut params: Vec<(&str, &str)> = Vec::new();

    if let Some(v) = main_id.map(str::trim).filter(|v| !v.is_empty()) {
        params.push(("typeMainId", v));
    }
    if let Some(v) = sub_id.map(str::trim).filter(|v| !v.is_empty()) {
        params.push(("typeSubId", v));
    }

    let url = if params.is_empty() {
        Url::parse(base).map_err(|e| e.to_string())?
    } else {
        Url::parse_with_params(base, &params).map_err(|e| e.to_string())?
    };

    let query = url.query().unwrap_or_default().to_string();
    Ok((url.into(), query))
}

fn build_item_url_and_query(item_id: &str) -> Result<(String, String), String> {
    let trimmed_item_id = item_id.trim();
    if trimmed_item_id.is_empty() {
        return Err("itemId is required".to_string());
    }

    let url = Url::parse_with_params(
        "https://zonai.skland.com/web/v1/wiki/item/info",
        &[("id", trimmed_item_id)],
    )
    .map_err(|e| e.to_string())?;

    let query = url.query().unwrap_or_default().to_string();
    Ok((url.into(), query))
}

// ==================== Media Download & Save ====================

#[derive(Serialize)]
struct MediaSaveResult {
    success: bool,
    path: String,
    message: String,
}

/// Download a file from a URL and save it to the appropriate location.
/// - On Android: downloads to app cache, then invokes Kotlin plugin to move to MediaStore.
/// - On other platforms: saves directly to the Downloads directory.
#[tauri::command]
async fn download_and_save_media(
    app: AppHandle,
    url: String,
    file_name: String,
    mime_type: String,
    target: String,
) -> Result<MediaSaveResult, String> {
    // Sanitize filename
    let safe_name = file_name
        .trim()
        .replace(|c: char| r#"\/:*?"<>|"#.contains(c), "_");
    let safe_name = if safe_name.is_empty() {
        format!(
            "file_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        )
    } else {
        safe_name
    };

    // Download the file
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 16) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    if bytes.is_empty() {
        return Err("Downloaded file is empty".to_string());
    }

    // Platform-specific save logic
    save_media_bytes(&app, &bytes, &safe_name, &mime_type, &target).await
}

/// Download media from a URL and save (for cases where we already have the bytes as base64).
/// This command accepts raw bytes encoded as base64 from the frontend as a fallback
/// when the CDN URL is not directly accessible (e.g., blob URLs, local files).
#[tauri::command]
async fn save_media_from_bytes(
    app: AppHandle,
    base64_data: String,
    file_name: String,
    mime_type: String,
    target: String,
) -> Result<MediaSaveResult, String> {
    let safe_name = file_name
        .trim()
        .replace(|c: char| r#"\/:*?"<>|"#.contains(c), "_");
    let safe_name = if safe_name.is_empty() {
        format!(
            "file_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        )
    } else {
        safe_name
    };

    let bytes = BASE64_STANDARD
        .decode(&base64_data)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    if bytes.is_empty() {
        return Err("Data is empty".to_string());
    }

    save_media_bytes(&app, &bytes, &safe_name, &mime_type, &target).await
}

async fn save_media_bytes(
    app: &AppHandle,
    bytes: &[u8],
    file_name: &str,
    mime_type: &str,
    target: &str,
) -> Result<MediaSaveResult, String> {
    #[cfg(target_os = "android")]
    {
        save_media_android(app, bytes, file_name, mime_type, target).await
    }

    #[cfg(target_os = "ios")]
    {
        save_media_ios(app, bytes, file_name, mime_type, target).await
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        save_media_desktop(app, bytes, file_name, mime_type, target).await
    }
}

#[cfg(not(target_os = "android"))]
async fn save_media_desktop(
    app: &AppHandle,
    bytes: &[u8],
    file_name: &str,
    _mime_type: &str,
    _target: &str,
) -> Result<MediaSaveResult, String> {
    let download_dir = app
        .path()
        .download_dir()
        .map_err(|e| format!("Cannot resolve Downloads directory: {e}"))?;

    // Ensure directory exists
    std::fs::create_dir_all(&download_dir)
        .map_err(|e| format!("Cannot create Downloads directory: {e}"))?;

    let save_path = download_dir.join(file_name);

    // Avoid overwriting: add suffix if file exists
    let final_path = if save_path.exists() {
        let stem = save_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let ext = save_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let mut counter = 1u32;
        loop {
            let new_name = if ext.is_empty() {
                format!("{stem}_{counter}")
            } else {
                format!("{stem}_{counter}.{ext}")
            };
            let candidate = download_dir.join(&new_name);
            if !candidate.exists() {
                break candidate;
            }
            counter += 1;
        }
    } else {
        save_path
    };

    std::fs::write(&final_path, bytes).map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(MediaSaveResult {
        success: true,
        path: final_path.to_string_lossy().to_string(),
        message: "Saved to Downloads".to_string(),
    })
}

// NOTE: With the mobile-first architecture, JS no longer calls Rust commands on
// Android/iOS — the native plugin handles download + save directly. This function
// is kept for compilation but is effectively dead code on mobile.
#[cfg(target_os = "android")]
async fn save_media_android(
    app: &AppHandle,
    bytes: &[u8],
    _file_name: &str,
    _mime_type: &str,
    _target: &str,
) -> Result<MediaSaveResult, String> {
    // Write bytes to app's cache directory (Rust and Kotlin share the same path)
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Cannot resolve app cache dir: {e}"))?;

    std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Cannot create cache dir: {e}"))?;

    let temp_name = format!(
        "_rust_media_{}.tmp",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );
    let temp_path = cache_dir.join(&temp_name);

    std::fs::write(&temp_path, bytes).map_err(|e| format!("Failed to write temp file: {e}"))?;

    let temp_path_str = temp_path.to_string_lossy().to_string();

    Ok(MediaSaveResult {
        success: false,
        path: temp_path_str,
        message: "Temp file written (dead code path on mobile-first architecture)".to_string(),
    })
}

// NOTE: Same as save_media_android — dead code on mobile-first architecture.
#[cfg(target_os = "ios")]
async fn save_media_ios(
    app: &AppHandle,
    bytes: &[u8],
    file_name: &str,
    _mime_type: &str,
    _target: &str,
) -> Result<MediaSaveResult, String> {
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Cannot resolve app cache dir: {e}"))?;

    std::fs::create_dir_all(&cache_dir).map_err(|e| format!("Cannot create cache dir: {e}"))?;

    let temp_name = format!(
        "_rust_media_{}_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
        file_name
    );
    let temp_path = cache_dir.join(&temp_name);

    std::fs::write(&temp_path, bytes).map_err(|e| format!("Failed to write temp file: {e}"))?;

    Ok(MediaSaveResult {
        success: false,
        path: temp_path.to_string_lossy().to_string(),
        message: "Temp file written (dead code path on mobile-first architecture)".to_string(),
    })
}

#[tauri::command]
async fn fetch_wiki_catalog(
    main_id: Option<String>,
    sub_id: Option<String>,
    d_id: String,
    _user_agent: String,
) -> Result<String, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let d_id = d_id.trim().to_string();
    if d_id.is_empty() {
        return Err("Device ID (dId) is required for public Wiki access".to_string());
    }
    let public_cred = random_public_cred();
    let mut sign_token = refresh_skland_public_token(&client, &d_id).await?;
    let path = "/web/v1/wiki/item/catalog";
    let (_, query) = build_catalog_url_and_query(main_id.as_deref(), sub_id.as_deref())?;

    match perform_signed_wiki_get(&client, path, &query, &public_cred, &sign_token, &d_id).await {
        Ok(content) => Ok(content),
        Err(response) if is_skland_auth_error(response.status) => {
            sign_token = refresh_skland_public_token(&client, &d_id).await?;
            perform_signed_wiki_get(&client, path, &query, &public_cred, &sign_token, &d_id)
                .await
                .map_err(|retry_response| build_skland_error_message(&retry_response))
        }
        Err(response) => Err(build_skland_error_message(&response)),
    }
}

#[tauri::command]
async fn fetch_wiki_item(
    item_id: String,
    d_id: String,
    _user_agent: String,
) -> Result<String, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let d_id = d_id.trim().to_string();
    if d_id.is_empty() {
        return Err("Device ID (dId) is required for public Wiki access".to_string());
    }

    let public_cred = random_public_cred();
    let mut sign_token = refresh_skland_public_token(&client, &d_id).await?;
    let path = "/web/v1/wiki/item/info";
    let (_, query) = build_item_url_and_query(&item_id)?;

    match perform_signed_wiki_get(&client, path, &query, &public_cred, &sign_token, &d_id).await {
        Ok(content) => Ok(content),
        Err(response) if is_skland_auth_error(response.status) => {
            sign_token = refresh_skland_public_token(&client, &d_id).await?;
            perform_signed_wiki_get(&client, path, &query, &public_cred, &sign_token, &d_id)
                .await
                .map_err(|retry_response| build_skland_error_message(&retry_response))
        }
        Err(response) => Err(build_skland_error_message(&response)),
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct FingerprintData {
    smid: String,
    canvas: String,
    clientSize: String,
    svm: i64,
    pmf: i64,
    plugins: String,
    timezone: i64,
    platform: String,
    url: String,
    referer: String,
    res: String,
    status: i64,
    vpw: String,
    trees: String,
    time: i64,
    #[serde(rename = "box")]
    r#box: String,
}

// 数美 DID 的 Protocol 102 要求对部分字段做 DES-ECB 混淆。
// 这里的 DES 仅用于兼容上游遗留协议，不能复用于本地敏感数据的通用加密。
fn protocol_102_legacy_des_ecb_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    use des::cipher::{BlockEncrypt, KeyInit};
    let cipher = Des::new_from_slice(key).map_err(|e| e.to_string())?;

    let block_size = 8;
    let mut padded_data = data.to_vec();
    let remainder = data.len() % block_size;
    if remainder != 0 {
        let padding_len = block_size - remainder;
        padded_data.extend(std::iter::repeat(0u8).take(padding_len));
    }

    for chunk in padded_data.chunks_mut(block_size) {
        let block = des::cipher::generic_array::GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }
    Ok(padded_data)
}

fn get_protocol_102_map() -> BTreeMap<String, (String, bool, String)> {
    let mut map = BTreeMap::new();
    // Key: original_name, Value: (obfuscated_name, is_encrypt, legacy_des_key)
    map.insert("appId".into(), ("xx".into(), true, "uy7mzc4h".into()));
    map.insert("box".into(), ("jf".into(), false, "".into()));
    map.insert("canvas".into(), ("yk".into(), true, "snrn887t".into()));
    map.insert("clientSize".into(), ("zx".into(), true, "cpmjjgsu".into()));
    map.insert(
        "organization".into(),
        ("dp".into(), true, "78moqjfc".into()),
    );
    map.insert("os".into(), ("pj".into(), true, "je6vk6t4".into()));
    map.insert("platform".into(), ("gm".into(), true, "pakxhcd2".into()));
    map.insert("plugins".into(), ("kq".into(), true, "v51m3pzl".into()));
    map.insert("pmf".into(), ("vw".into(), true, "2mdeslu3".into()));
    map.insert("protocol".into(), ("protocol".into(), false, "".into()));
    map.insert("referer".into(), ("ab".into(), true, "y7bmrjlc".into()));
    map.insert("res".into(), ("hf".into(), true, "whxqm2a7".into()));
    map.insert("rtype".into(), ("lo".into(), true, "x8o2h2bl".into()));
    map.insert("sdkver".into(), ("sc".into(), true, "9q3dcxp2".into()));
    map.insert("status".into(), ("an".into(), true, "2jbrxxw4".into()));
    map.insert("subVersion".into(), ("ns".into(), true, "eo3i2puh".into()));
    map.insert("svm".into(), ("qr".into(), true, "fzj3kaeh".into()));
    map.insert("time".into(), ("nb".into(), true, "q2t3odsk".into()));
    map.insert("timezone".into(), ("as".into(), true, "1uv05lj5".into()));
    map.insert("tn".into(), ("py".into(), true, "x9nzj1bp".into()));
    map.insert("trees".into(), ("pi".into(), true, "acfs0xo4".into()));
    map.insert("ua".into(), ("bj".into(), true, "k92crp1t".into()));
    map.insert("url".into(), ("cf".into(), true, "y95hjkoo".into()));
    map.insert("version".into(), ("version".into(), false, "".into()));
    map.insert("vpw".into(), ("ca".into(), true, "r9924ab5".into()));
    map
}

fn flatten_json_values(value: &Value, output: &mut String) {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for key in keys {
                flatten_json_values(&map[key], output);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                flatten_json_values(v, output);
            }
        }
        Value::String(s) => {
            output.push_str(s);
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                output.push_str(&(i * 10000).to_string());
            } else if let Some(f) = n.as_f64() {
                output.push_str(&(f * 10000.0).to_string());
            }
        }
        Value::Bool(b) => {
            output.push_str(if *b { "true" } else { "false" });
        }
        _ => {}
    }
}

fn rsa_pkcs1v15_encrypt(
    message: &[u8],
    modulus: &BigUint,
    exponent: &BigUint,
    modulus_len: usize,
    rng: &mut impl RngCore,
) -> Result<Vec<u8>, String> {
    const PKCS1V15_MIN_PADDING_LEN: usize = 8;

    if message.len() > modulus_len.saturating_sub(PKCS1V15_MIN_PADDING_LEN + 3) {
        return Err("message too long for RSA PKCS#1 v1.5 encryption".into());
    }

    let padding_len = modulus_len - message.len() - 3;
    let mut encoded_message = Vec::with_capacity(modulus_len);

    encoded_message.extend_from_slice(&[0x00, 0x02]);
    for _ in 0..padding_len {
        let mut byte = 0u8;
        while byte == 0 {
            let mut buffer = [0u8; 1];
            rng.fill_bytes(&mut buffer);
            byte = buffer[0];
        }
        encoded_message.push(byte);
    }
    encoded_message.push(0x00);
    encoded_message.extend_from_slice(message);

    let encrypted = BigUint::from_bytes_be(&encoded_message).modpow(exponent, modulus);
    let mut ciphertext = encrypted.to_bytes_be();
    if ciphertext.len() < modulus_len {
        let mut prefixed = vec![0u8; modulus_len - ciphertext.len()];
        prefixed.extend_from_slice(&ciphertext);
        ciphertext = prefixed;
    }

    Ok(ciphertext)
}

fn encrypt_with_skland_public_key(message: &[u8]) -> Result<String, String> {
    let modulus = BigUint::parse_bytes(SKLAND_RSA_MODULUS_HEX.as_bytes(), 16)
        .ok_or_else(|| "failed to parse Skland RSA public key".to_string())?;
    let exponent = BigUint::from(SKLAND_RSA_PUBLIC_EXPONENT);
    let mut rng = OsRng;
    let ciphertext = rsa_pkcs1v15_encrypt(
        message,
        &modulus,
        &exponent,
        SKLAND_RSA_MODULUS_LEN,
        &mut rng,
    )?;

    Ok(BASE64_STANDARD.encode(ciphertext))
}

#[tauri::command]
async fn generate_did(fingerprint: FingerprintData) -> Result<String, String> {
    log::info!("Starting DID generation (Protocol 102)...");

    let uuid = Uuid::new_v4().to_string();
    let pri_id_full = {
        let mut hasher = Md5::new();
        hasher.update(uuid.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    };
    let pri_id = &pri_id_full[..16];

    let ep = encrypt_with_skland_public_key(uuid.as_bytes())?;

    // 1. 构建原始内层字段 (未混淆)
    let mut inner_data_fields: BTreeMap<String, Value> = BTreeMap::new();
    inner_data_fields.insert("appId".into(), Value::String(SKLAND_APP_ID.into()));
    inner_data_fields.insert("box".into(), Value::String(fingerprint.r#box));
    inner_data_fields.insert("canvas".into(), Value::String(fingerprint.canvas));
    inner_data_fields.insert(
        "clientSize".into(),
        Value::String(fingerprint.clientSize.clone()),
    );
    inner_data_fields.insert(
        "organization".into(),
        Value::String(SKLAND_ORGANIZATION.into()),
    );
    inner_data_fields.insert("os".into(), Value::String("web".into()));
    inner_data_fields.insert("platform".into(), Value::String(fingerprint.platform));
    inner_data_fields.insert("plugins".into(), Value::String(fingerprint.plugins));
    inner_data_fields.insert(
        "pmf".into(),
        Value::Number(serde_json::Number::from(fingerprint.pmf)),
    );
    inner_data_fields.insert("protocol".into(), Value::Number(102.into()));
    inner_data_fields.insert("referer".into(), Value::String(fingerprint.referer));
    inner_data_fields.insert("res".into(), Value::String(fingerprint.res));
    inner_data_fields.insert("rtype".into(), Value::String("all".into()));
    inner_data_fields.insert("sdkver".into(), Value::String("3.0.0".into()));
    inner_data_fields.insert("smid".into(), Value::String(fingerprint.smid));
    inner_data_fields.insert(
        "status".into(),
        Value::Number(serde_json::Number::from(fingerprint.status)),
    );
    inner_data_fields.insert("subVersion".into(), Value::String("1.0.0".into()));
    inner_data_fields.insert(
        "svm".into(),
        Value::Number(serde_json::Number::from(fingerprint.svm)),
    );
    inner_data_fields.insert(
        "time".into(),
        Value::Number(serde_json::Number::from(fingerprint.time)),
    );
    inner_data_fields.insert(
        "timezone".into(),
        Value::Number(serde_json::Number::from(fingerprint.timezone)),
    );
    inner_data_fields.insert("trees".into(), Value::String(fingerprint.trees));
    // 数美 DID 内层指纹字段 ua 固定为 SKLAND_USER_AGENT，禁止使用动态 UA。
    inner_data_fields.insert("ua".into(), Value::String(SKLAND_USER_AGENT.to_string()));
    inner_data_fields.insert("url".into(), Value::String(fingerprint.url));
    inner_data_fields.insert("version".into(), Value::String("3.0.0".into()));
    inner_data_fields.insert("vpw".into(), Value::String(fingerprint.vpw));

    // 2. 计算 tn 并注入到 inner_data_fields
    let mut tn_input = String::new();
    flatten_json_values(
        &Value::Object(inner_data_fields.clone().into_iter().collect()),
        &mut tn_input,
    );
    let mut hasher = Md5::new();
    hasher.update(tn_input.as_bytes());
    let tn = hex::encode(hasher.finalize());
    inner_data_fields.insert("tn".into(), Value::String(tn));

    // 3. 执行字段混淆
    let protocol_map = get_protocol_102_map();
    let mut obfuscated_inner_data: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in inner_data_fields {
        if let Some((obfuscated_name, is_encrypt, legacy_des_key)) = protocol_map.get(&key) {
            if *is_encrypt {
                let value_str = match &value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => serde_json::to_string(&value).unwrap(),
                };
                let encrypted_value = protocol_102_legacy_des_ecb_encrypt(
                    legacy_des_key.as_bytes(),
                    value_str.as_bytes(),
                )?;
                obfuscated_inner_data.insert(
                    obfuscated_name.clone(),
                    Value::String(BASE64_STANDARD.encode(encrypted_value)),
                );
            } else {
                obfuscated_inner_data.insert(obfuscated_name.clone(), value);
            }
        } else {
            // smid 等不混淆字段
            obfuscated_inner_data.insert(key, value);
        }
    }

    // 4. 核心加密链路: Data -> JSON -> GZIP -> Base64 -> AES -> Hex
    let json_str = serde_json::to_string(&obfuscated_inner_data).unwrap();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json_str.as_bytes()).unwrap();
    let gzipped_data = encoder.finish().unwrap();
    let gzipped_base64 = BASE64_STANDARD.encode(&gzipped_data);

    let iv = "0102030405060708".as_bytes();
    type Aes128Cbc = Encryptor<Aes128>;
    let cipher = Aes128Cbc::new_from_slices(pri_id.as_bytes(), iv).map_err(|e| e.to_string())?;
    let final_data =
        cipher.encrypt_padded_vec_mut::<block_padding::Pkcs7>(gzipped_base64.as_bytes());
    let final_data_hex = hex::encode(final_data);

    // 7. 发送最终请求
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .post("https://fp-it.portal101.cn/deviceprofile/v4")
        // 数美 DID 请求头 UA 固定，不允许跟随设备或前端动态变化。
        .header("User-Agent", SKLAND_USER_AGENT)
        .json(&serde_json::json!({
            "appId": SKLAND_APP_ID,
            "organization": SKLAND_ORGANIZATION,
            "ep": ep,
            "data": final_data_hex,
            "os": "web",
            "encode": 5,
            "compress": 2
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let resp_json: Value = resp.json().await.map_err(|e| e.to_string())?;
        if resp_json["code"] == 1100 {
            if let Some(device_id) = resp_json["detail"]["deviceId"].as_str() {
                log::info!("DID generated successfully: {}", device_id);
                Ok(format!("B{}", device_id))
            } else {
                Err("deviceId not found in response".to_string())
            }
        } else {
            Err(format!("Server returned error: {}", resp_json))
        }
    } else {
        Err(format!("Request failed with status: {}", resp.status()))
    }
}

#[cfg_attr(target_os = "ios", tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_android_intent::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            download_and_save_media,
            save_media_from_bytes,
            fetch_wiki_catalog,
            fetch_wiki_item,
            generate_did
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            schedule_dev_webview_recovery(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn schedule_dev_webview_recovery<R: tauri::Runtime>(app: &tauri::App<R>) {
    #[cfg(debug_assertions)]
    {
        let Some(dev_url) = app.config().build.dev_url.clone() else {
            return;
        };

        let app_handle = app.handle().clone();
        let target_url = dev_url.as_str().trim_end_matches('/').to_string();
        let redirect_script = format!(
            "window.location.replace({});",
            serde_json::to_string(dev_url.as_str()).expect("dev URL should serialize")
        );

        thread::spawn(move || {
            for delay_ms in [750_u64, 1_500, 3_000, 5_000, 8_000] {
                thread::sleep(Duration::from_millis(delay_ms));

                let Some(window) = app_handle.get_webview_window("main") else {
                    log::warn!("failed to find main webview window for dev navigation recovery");
                    return;
                };

                let current_url = match window.url() {
                    Ok(url) => url,
                    Err(error) => {
                        log::warn!("failed to inspect current webview URL: {error}");
                        continue;
                    }
                };

                let current_url_normalized = current_url.as_str().trim_end_matches('/');
                let matches_hash_route = current_url_normalized
                    .starts_with(&(target_url.clone() + "/#"))
                    || current_url_normalized.starts_with(&(target_url.clone() + "#"));

                if current_url_normalized == target_url || matches_hash_route {
                    return;
                }

                log::warn!(
                    "dev webview is at {}; retrying navigation to {}",
                    current_url,
                    dev_url
                );

                if let Err(error) = window.eval(redirect_script.clone()) {
                    log::warn!("failed to force dev webview redirect via eval: {error}");
                }

                if let Err(error) = window.navigate(dev_url.clone()) {
                    log::warn!("failed to retry dev webview navigation: {error}");
                }
            }
        });
    }

    #[cfg(not(debug_assertions))]
    let _ = app;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Error;

    const TEST_RSA_MODULUS_HEX: &str = concat!(
        "c92119081f5d81e97893f0024d24bbbc4cbea2d6570992dd7b5250fa47adce54",
        "6edbe72d2088d3ffd92dff70c8d6ac16d8b69f2ab76cbe326be173ce1d4a0bbf",
    );
    const TEST_RSA_PRIVATE_EXPONENT_HEX: &str = concat!(
        "13992809449525fd8c044e54cb13933f8bf2df8727400591935cb80b4b44c260",
        "078e504195116f39feedcfecb9c4c3732f0f17e311d4d901f8220b159fd5a901",
    );
    const TEST_RSA_MODULUS_LEN: usize = 64;

    struct SequenceRng {
        bytes: Vec<u8>,
        index: usize,
    }

    impl SequenceRng {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, index: 0 }
        }

        fn next_byte(&mut self) -> u8 {
            let byte = self.bytes[self.index % self.bytes.len()];
            self.index += 1;
            byte
        }
    }

    impl RngCore for SequenceRng {
        fn next_u32(&mut self) -> u32 {
            let mut bytes = [0u8; 4];
            self.fill_bytes(&mut bytes);
            u32::from_le_bytes(bytes)
        }

        fn next_u64(&mut self) -> u64 {
            let mut bytes = [0u8; 8];
            self.fill_bytes(&mut bytes);
            u64::from_le_bytes(bytes)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_byte();
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    #[test]
    fn rsa_pkcs1v15_encrypt_builds_a_valid_padded_block() {
        let modulus = BigUint::parse_bytes(TEST_RSA_MODULUS_HEX.as_bytes(), 16).unwrap();
        let public_exponent = BigUint::from(65_537_u32);
        let private_exponent =
            BigUint::parse_bytes(TEST_RSA_PRIVATE_EXPONENT_HEX.as_bytes(), 16).unwrap();
        let message = b"did-check";
        let mut rng = SequenceRng::new(vec![0, 0x11, 0, 0x22, 0x33, 0x44, 0x55]);

        let ciphertext = rsa_pkcs1v15_encrypt(
            message,
            &modulus,
            &public_exponent,
            TEST_RSA_MODULUS_LEN,
            &mut rng,
        )
        .unwrap();

        assert_eq!(ciphertext.len(), TEST_RSA_MODULUS_LEN);

        let decrypted = BigUint::from_bytes_be(&ciphertext).modpow(&private_exponent, &modulus);
        let mut encoded_message = decrypted.to_bytes_be();
        if encoded_message.len() < TEST_RSA_MODULUS_LEN {
            let mut prefixed = vec![0u8; TEST_RSA_MODULUS_LEN - encoded_message.len()];
            prefixed.extend_from_slice(&encoded_message);
            encoded_message = prefixed;
        }

        assert_eq!(encoded_message[0], 0x00);
        assert_eq!(encoded_message[1], 0x02);

        let separator_index = encoded_message[2..]
            .iter()
            .position(|byte| *byte == 0)
            .map(|index| index + 2)
            .expect("PKCS#1 v1.5 block should contain a separator byte");

        assert!(separator_index >= 10);
        assert!(encoded_message[2..separator_index]
            .iter()
            .all(|byte| *byte != 0));
        assert_eq!(&encoded_message[separator_index + 1..], message);
    }

    #[test]
    fn encrypt_with_skland_public_key_returns_fixed_size_ciphertext() {
        let ciphertext = BASE64_STANDARD
            .decode(encrypt_with_skland_public_key(b"123e4567-e89b-12d3-a456-426614174000").unwrap())
            .unwrap();

        assert_eq!(ciphertext.len(), SKLAND_RSA_MODULUS_LEN);
    }
}
