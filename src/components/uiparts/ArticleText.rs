use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct ArticleTextProps {
    pub text: String,
}
#[function_component]
pub fn ArticleText(props: &ArticleTextProps) -> Html {
    html! { <p> { props.text.clone() } </p> }
}
