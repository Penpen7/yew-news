use crate::components::uiparts::ArticleText::ArticleText;
use crate::components::uiparts::ArticleTitle::ArticleTitle;
use crate::components::uiparts::GoodButton::GoodButton;
use crate::components::uiparts::Header::Header;
use crate::models::article::GetArticles;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TemplateArticleProps {
    pub id: usize,
}

#[function_component]
pub fn TemplateArticle(props: &TemplateArticleProps) -> Html {
    let articles = GetArticles();
    match articles.get(props.id) {
        Some(v) => html! {
            <>
                <Header />
                <ArticleTitle title={v.title.clone()} />
                <ArticleText text={v.text.clone()} />
                <GoodButton good_point={v.good_point} />
            </>
        },
        _ => html! {
            <Redirect<Route> to={Route::NotFound}/>
        },
    }
}
