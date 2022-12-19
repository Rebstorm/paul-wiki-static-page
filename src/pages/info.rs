use yew::{function_component, html, Html};


#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <div>
          {"I am a fucking info"}
        </div>
    }
}
