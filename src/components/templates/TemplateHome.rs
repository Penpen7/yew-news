use crate::components::uiparts::ArticleList::ArticleList;
use crate::components::uiparts::Header::Header;
use crate::models::article::GetArticles;
use yew::prelude::*;
#[function_component]
pub fn TemplateHome() -> Html {
    let articles = GetArticles();
    html! {
        <>
            <Header />
            <ArticleList { articles } />
        </>
    }
}
