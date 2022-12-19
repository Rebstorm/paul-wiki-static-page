use yew::{Html, html};
use crate::navigation::route::Route;


use crate::pages::{
    index::Index, info::Info
};



pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Index /> },
        Route::Secure => html! {
            <div></div>
        },
        Route::Info => html! { <Info /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
