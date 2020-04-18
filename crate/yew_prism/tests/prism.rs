use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_prism::{Prism, Props};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn shoud_create_prism_component() {
    let props_prism = Props {
        code: "let greeting: &str = \"Hello World\";".to_string(),
        language: "rust".to_string(),
    };

    let link = ComponentLink::new();

    let prism = Prism::create(props_prism, link.clone());

    let prism_vnode = prism.render();
    let code_ref = NodeRef::default();

    let vnode_expected = html! {
        <pre>
            <code class="language-rust", ref=code_ref/>
        </pre>
    };

    assert_eq!(prism_vnode, vnode_expected);
}
