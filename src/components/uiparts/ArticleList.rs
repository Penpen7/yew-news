use crate::models::article::article;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleListProps {
    pub articles: Vec<article>,
}

#[function_component]
pub fn ArticleList(props: &ArticleListProps) -> Html {
    let articles = props
        .articles
        .iter()
        .map(|x| html! {<li><Link<Route> to={Route::Article { id: x.id }}>{x.title.clone()}</Link<Route>></li>})
        .collect::<Html>();

    html! {
        <ul>{ articles }</ul>
    }
}
