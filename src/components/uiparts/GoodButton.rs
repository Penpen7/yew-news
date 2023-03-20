use yew::prelude::*;
#[function_component]
pub fn GoodButton() -> Html {
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
