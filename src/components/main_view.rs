use yew::{function_component, html, Html, Properties, Children};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub title: String
}

#[function_component(MainView)]
pub fn main_view(props: &Props) -> Html {
    html! {
        <div class="content">
            <h1>{props.title.clone()}</h1>
            <div class="animation">
                { for props.children.iter() }
            </div>
        </div>
    }
}
