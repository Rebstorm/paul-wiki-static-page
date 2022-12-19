use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigation::route::AvailableRoutes;

#[function_component(Navigation)]
pub fn navigation() -> Html {

    html! {
        <nav class="container">
            <Link<AvailableRoutes> to={AvailableRoutes::Home}>
                <div>{"home we go"} </div>
            </Link<AvailableRoutes>>
            <Link<AvailableRoutes> to={AvailableRoutes::Info}>
                { "Info" }
            </Link<AvailableRoutes>>
            <Link<AvailableRoutes> to={AvailableRoutes::NotFound}>
                { "Copyright" }
            </Link<AvailableRoutes>>
        </nav>
    }
}

