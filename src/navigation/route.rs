use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AvailableRoutes {
    #[at("/")]
    Home,
    #[at("/info")]
    Info,
    #[not_found]
    #[at("/404")]
    NotFound,
}