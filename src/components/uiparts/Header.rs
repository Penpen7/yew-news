use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! { <h1><Link<Route> to={Route::Home}>{"Penpen7ニュース"}</Link<Route>></h1> }
}
