use std::sync::Arc;

use futures::lock::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
export function isSilentMode() {
    return new Promise((resolve) => {
        const audio = new Audio();
        audio.src = 'data:audio/mp3;base64,...'; // A silent MP3 encoded in base64
        audio.play().then(() => resolve(false)).catch(() => resolve(true));
    });
}
"#)]
extern "C" {
    #[wasm_bindgen(js_name = "isSilentMode")]
    async fn is_silent_mode_js() -> JsValue;
}

pub async fn check_silent_mode(shared_status: Arc<Mutex<Option<bool>>>) {
    let result = is_silent_mode_js().await;
    let is_silent = result.as_bool().unwrap_or(false);
    let mut status = shared_status.lock().await;

    #[cfg(target_arch = "wasm32")] {

        if is_silent {
            web_sys::log_1(&"Silencer is ON! {:?}".into(), is_silent);
        } else {
            web_sys::log_1(&"Silencer is OFF! {:?}".into(), is_silent);
        }
    }

    *status = Some(is_silent);
}
