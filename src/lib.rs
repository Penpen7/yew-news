use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod models;
use components::templates::TemplateArticle::TemplateArticle;
use components::templates::TemplateHome::TemplateHome;
use components::templates::TemplateNotFound::TemplateNotFound;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/yew-news")]
    Home,
    #[at("/yew-news/article/:id")]
    Article { id: usize },
    #[not_found]
    #[at("/yew-news/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <TemplateHome/>},
        Route::Article { id } => html! { <TemplateArticle { id }/>},
        Route::NotFound => html! { <TemplateNotFound />},
    }
}
