use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_speech_recog);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<SpeechRecog<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_speech_recog)?;
  Ok(SpeechRecog(handle))
}

/// Access to the speech-recog APIs.
pub struct SpeechRecog<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SpeechRecog<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }

  pub fn is_supported(&self, payload: IsSupportedRequest) -> crate::Result<IsSupportedResponse> {
    self
      .0
      .run_mobile_plugin("isSupported", payload)
      .map_err(Into::into)
  }
}
