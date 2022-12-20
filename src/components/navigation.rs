use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigation::route::AvailableRoutes;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav class="container">
            <Link<AvailableRoutes> classes="target" to={AvailableRoutes::Home}>
                <div class="icon home" aria-describedby="home" />
            </Link<AvailableRoutes>>
            <Link<AvailableRoutes> classes="target" to={AvailableRoutes::Info}>
                <div class="icon syntax" aria-describedby="Pauls stack" />
            </Link<AvailableRoutes>>
            <Link<AvailableRoutes> classes="target" to={AvailableRoutes::NotFound}>
                <div class="icon copyright" aria-describedby="copyright" />
            </Link<AvailableRoutes>>
        </nav>
    }
}

