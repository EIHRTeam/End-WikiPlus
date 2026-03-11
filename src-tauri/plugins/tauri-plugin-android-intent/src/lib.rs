use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_android_intent);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-intent")
        .setup(|_app, _api| {
            #[cfg(target_os = "android")]
            {
                _api.register_android_plugin(
                    "com.eihrteam.wikiplus.android_intent",
                    "AndroidIntentPlugin",
                )?;
            }
            #[cfg(target_os = "ios")]
            {
                _api.register_ios_plugin(init_plugin_android_intent)?;
            }
            Ok(())
        })
        .build()
}
