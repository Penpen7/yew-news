use crate::components::uiparts::Header::Header;
use yew::prelude::*;
#[function_component]
pub fn TemplateNotFound() -> Html {
    html! {
        <>
            <Header />
            {"ページが見つかりません。"}
        </>
    }
}
