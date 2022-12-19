use yew::prelude::*;
use yew_router::prelude::*;


use crate::components::{
     navigation::Navigation
};

use crate::navigation::{
    route::AvailableRoutes, route_switch::switch
};


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container">
                <Navigation />
                <Switch<AvailableRoutes> render={switch} />
            </div>
        </BrowserRouter>
    }
}
