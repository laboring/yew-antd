use antd::prelude::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 2;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+2" }</button>
            <p>{ *counter }</p>
            <Button style="color:blue;" is_loading=true>{"test"}</Button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
