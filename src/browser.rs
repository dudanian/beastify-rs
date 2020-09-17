use js_sys::Object;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

pub mod runtime {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["browser", "runtime"], js_name = getURL)]
        pub fn get_url(path: &str) -> JsValue;
    }
}

pub mod tabs {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["browser", "tabs"], js_name = insertCSS)]
        pub fn insert_css(details: &Object) -> Promise;

        #[wasm_bindgen(js_namespace = ["browser", "tabs"], js_name = removeCSS)]
        pub fn remove_css(details: &Object) -> Promise;

        #[wasm_bindgen(js_namespace = ["browser", "tabs"], js_name = sendMessage)]
        pub fn send_message(tab_id: u32, message: &JsValue) -> Promise;

        #[wasm_bindgen(js_namespace = ["browser", "tabs"], js_name = executeScript)]
        pub fn execute_script(details: &Object) -> Promise;

        #[wasm_bindgen(js_namespace = ["browser", "tabs"])]
        pub fn query(queryObj: &Object) -> Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        // duck-typed Tab class
        #[wasm_bindgen(js_namespace = ["browser", "tabs"])]
        pub type Tab;

        #[wasm_bindgen(method, getter)]
        pub fn id(this: &Tab) -> Option<u32>;
    }
}
