use yew::prelude::*;
use yew_router::prelude::*;


use crate::components::{
     navigation::Navigation
};

use crate::pages::{
    index::Index, info::Info
};

use crate::navigation::{
    route::Route, route_switch::switch
};



#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Navigation />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}
