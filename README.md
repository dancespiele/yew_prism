# `Yew Prism`

Yew Prism is a highlighter code component based in [Prism](https://prismjs.com) for [yew](https://yew.rs)

## How it works

1. Install the `prismjs` node module

    `npm install prismjs`

2. Import the prismjs module and styles, and all the languages component which you want to use for highlighting,
in your javascript/typescript main file yew project

```typescript
import 'prismjs/themes/prism.css';
import 'prismjs';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-rust';
import module from '../crate/Cargo.toml';

module.run();
```

**Note:** You can use [yew-parcel-template](https://github.com/spielrs/yew-parcel-template) or another template described
[here](https://yew.rs/docs/getting-started/starter-templates) to create a yew project

3. Add yew_prism in your cargo.toml

```toml
[dependencies]
yew = { version = "0.17", features = ["web_sys"]}
yew_prism="0.4"
```

4. Now you are ready to use the component &#128640;

## Example

```rust
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
            <Prism
                code="let greeting: &str = \"Hello World\";"
                language="rust"
            />
        }
    }
}

```

## Run documentation page

1. `git clone https://github.com/spielrs/yew_prism.git`
3. `cd yew_prism`
4. `npm install` 
5. `npm start`

## License

Yew Prism is MIT licensed. See [license](LICENSE)
