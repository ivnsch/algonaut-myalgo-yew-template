use wasm_bindgen::prelude::*;

/// Bindings to JS
#[wasm_bindgen(module = "/src/myalgo_glue.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "connectWallet")]
    pub async fn connect_wallet() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = "signTransaction")]
    pub async fn sign_transaction(transaction: JsValue) -> Result<JsValue, JsValue>;
}
