use yew::prelude::*;
use yew_prism::Prism;

pub struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Yew Prism"}</h1>

                <span><a href="https://prismjs.com/" target="_blank">{"Prism"}</a>{" component for yew"}</span>

                <h2>{"Example of highlighting rust code"}</h2>
                <Prism
                    code="let greeting: &str = \"Hello World\";"
                    language="rust"
                />

                <h3>{"Code:"}</h3>
                <Prism
                    code="\t<Prism
    \tcode=\"let greeting: &str = \"Hello World\"\"
    \tlanguage=\"rust\"
    />"
                    language="rust"
                />

                <h2>{"Example of highlighting html code"}</h2>

                <Prism
                    code="<body>\n\t<h1>Hello World</h1>\n\t<div>Yew Prism is awesome</div>\n</body>"
                    language="markup"
                />

                <h3>{"Code:"}</h3>

                <Prism
                    code="\t<Prism
    \tcode=\"<body>\\n\\t<h1>Hello World</h1>\\n\\t<div>Yew Prism is awesome</div>\\n</body>\"
    \tlanguage=\"markup\"
    />"
                    language="rust"
                />

            </div>
        }
    }
}
