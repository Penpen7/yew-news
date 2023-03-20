use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GoodButtonProps {
    pub good_point: usize,
}

#[function_component]
pub fn GoodButton(props: &GoodButtonProps) -> Html {
    let good = use_state(|| props.good_point);
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
