use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Language;
    #[wasm_bindgen(js_namespace = Prism)]
    pub static languages: Language;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Language, prop: String) -> Language;

    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlight(code: String, lang: Language) -> String;
}
