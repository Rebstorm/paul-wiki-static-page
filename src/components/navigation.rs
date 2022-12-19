use yew::prelude::*;
use yew_router::prelude::*;
use crate::navigation::route::Route;


#[function_component(Navigation)]
pub fn navigation() -> Html {

    let navigator = use_navigator().unwrap();

    html! {
        <nav>
            {"hello"}
        </nav>
    }
}

