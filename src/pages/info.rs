use yew::{function_component, html, Html};

use crate::components::{
    main_view::MainView
};

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <MainView title="Stack">
            <div> {"hi"} </div>
            <div> {"bye"} </div>
        </MainView>
    }
}
