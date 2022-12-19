use yew::{function_component, html, Html};



use crate::components::{
    main_view::MainView
};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div>
          <MainView />
        </div>
    }
}
