use crate::components::uiparts::ArticleText::ArticleText;
use crate::components::uiparts::GoodButton::GoodButton;
use crate::components::uiparts::Header::Header;
use crate::models::article::article;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TemplateArticleProps {
    pub id: usize,
}

#[function_component]
pub fn TemplateArticle(props: &TemplateArticleProps) -> Html {
    let articles = vec![
        article {
            title: "Rust".to_string(),
            text: "毎日Rustをやっています".to_string(),
        },
        article {
            title: "hello".to_string(),
            text: "レコードをたくさんゲットしました".to_string(),
        },
    ];
    match articles.get(props.id) {
        Some(v) => html! {
            <>
                <Header />
                <ArticleText text={v.text.clone()}/>
                <GoodButton />
            </>
        },
        _ => html! {
            <Redirect<Route> to={Route::NotFound}/>
        },
    }
}
