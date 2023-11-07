use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub is_loading: bool,
    pub children: Html,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Button)]
pub fn ant_button(props: &Props) -> Html {
    html! { <button style={props.style.clone()}>{props.children.clone()}</button>}
}
