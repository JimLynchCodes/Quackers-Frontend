use std::sync::Arc;

use futures::lock::Mutex;

#[cfg(target_arch = "wasm32")]
pub mod wasm_silent_mode {

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
        pub async fn is_silent_mode_js() -> JsValue;
    }

}

pub async fn check_silent_mode(shared_status: Arc<Mutex<Option<bool>>>) {
    
    let mut status = shared_status.lock().await;

    #[cfg(not(target_arch = "wasm32"))] {   
        println!("Not wasm, not checking silent mode...");  
        *status = Some(false);
    }
    
    #[cfg(target_arch = "wasm32")] {

        use wasm_silent_mode::is_silent_mode_js;

        let result = is_silent_mode_js().await;
        let is_silent = result.as_bool().unwrap_or(false);

        if is_silent {
            web_sys::console::log_1(&"Silencer is ON!".into());
        } else {
            web_sys::console::log_1(&"Silencer is OFF!".into());
        }
        *status = Some(is_silent);
    }

}
