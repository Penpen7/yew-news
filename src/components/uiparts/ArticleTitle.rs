use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct ArticleTitleProps {
    pub title: String,
}
#[function_component]
pub fn ArticleTitle(props: &ArticleTitleProps) -> Html {
    html! { <h2> { props.title.clone() } </h2> }
}
