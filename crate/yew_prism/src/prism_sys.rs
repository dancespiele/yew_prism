use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "prismjs")]
extern "C" {
    pub type Language;
    pub static languages: Language;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Language, prop: String) -> Language;

    pub fn highlight(code: String, lang: Language) -> String;
}
