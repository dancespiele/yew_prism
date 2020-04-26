use crate::prism_sys::{highlight, languages as lang};
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Prism {
    pub code_ref: NodeRef,
    pub props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Code that you want to highlight
    pub code: String,
    /// Language of that code (rust, javascript, etc...)
    pub language: String,
}

impl Component for Prism {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            code_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let template = highlight(
            self.props.code.clone(),
            lang.get(self.props.language.clone()),
        );

        if let Some(code) = self.code_ref.cast::<HtmlElement>() {
            code.set_inner_html(&template);
        }
    }

    fn view(&self) -> Html {
        html! {
            <pre>
                <code class=format!("language-{}", self.props.language) ref=self.code_ref.clone()>
                </code>
            </pre>
        }
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn shoud_create_prism_component() {
    let props_prism = Props {
        code: "let greeting: &str = \"Hello World\";".to_string(),
        language: "rust".to_string(),
    };

    let link = ComponentLink::new();

    let prism = Prism::create(props_prism, link);

    let prism_vnode = prism.render();
    let code_ref = NodeRef::default();

    let vnode_expected = html! {
        <pre>
            <code class="language-rust", ref=code_ref/>
        </pre>
    };

    assert_eq!(prism_vnode, vnode_expected);
}
