use yew::{Html, html};
use crate::navigation::route::AvailableRoutes;


use crate::pages::{
    index::Index, info::Info
};



pub fn switch(routes: AvailableRoutes) -> Html {
    match routes {
        AvailableRoutes::Home => html! { <Index /> },
        AvailableRoutes::Info => html! { <Info /> },
        AvailableRoutes::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
