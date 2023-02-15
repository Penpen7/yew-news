use yew::prelude::*;
use yew_router::prelude::*;

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
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn TemplateHome() -> Html {
    html! {
        <>
            <Header />
        </>
    }
}

#[function_component]
fn TemplateNotFound() -> Html {
    html! {
        <>
            <Header />
            {"ページが見つかりません。"}
        </>
    }
}

struct article {
    title: String,
    text: String,
}

#[function_component]
fn TemplateArticle(props: &TemplateArticleProps) -> Html {
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

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component]
fn Header() -> Html {
    html! { <h1>{ "Penpen7ニュース" }</h1> }
}

#[derive(Properties, PartialEq, Clone)]
struct ArticleTextProps {
    text: String,
}

#[derive(Properties, PartialEq, Clone)]
struct TemplateArticleProps {
    id: usize,
}

#[function_component]
fn GoodButton() -> Html {
    let good = use_state(|| 0);
    let on_good_button_click = {
        let good = good.clone();
        move |_| {
            good.set(*good + 1);
        }
    };
    html! {
      <>
        <div>{*good}</div>
        <button onclick={on_good_button_click}>{"いいね"}</button>
      </>
    }
}

#[function_component]
fn ArticleText(props: &ArticleTextProps) -> Html {
    html! { <p> { props.text.clone() } </p> }
}
