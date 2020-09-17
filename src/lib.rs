use wasm_bindgen::prelude::*;

mod browser;
mod choose_beast;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    choose_beast::execute();
    Ok(())
}
