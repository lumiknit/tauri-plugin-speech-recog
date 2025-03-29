use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SpeechRecogExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.speech_recog().ping(payload)
}

#[command]
pub(crate) async fn is_supported<R: Runtime>(
    app: AppHandle<R>,
    payload: IsSupportedRequest,
) -> Result<IsSupportedResponse> {
    app.speech_recog().is_supported(payload)
}
