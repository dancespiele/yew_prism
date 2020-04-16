use crate::prism_sys::{highlight, languages as lang};
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Prism {
    code_ref: NodeRef,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub code: String,
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

    fn mounted(&mut self) -> ShouldRender {
        let template = highlight(
            self.props.code.clone(),
            lang.get(self.props.language.clone()),
        );

        if let Some(code) = self.code_ref.cast::<HtmlElement>() {
            code.set_inner_html(&template);
        }

        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
