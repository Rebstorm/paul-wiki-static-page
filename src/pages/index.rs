use yew::{function_component, html, Html};
use yew_router::prelude::*;



use crate::components::{
    main_view::MainView
};

use crate::navigation::route::AvailableRoutes;


#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div class="index">
          <MainView title="Paul" >
                <p>{"Welcome."} </p>
                <p>{"Glad that you want to know more about me."}</p>
                <p>{"I do dev."}</p>
                <p>{"Sometimes I do real dev work as well. Would you like to know more? Check out my "}
                    <Link<AvailableRoutes> classes="target" to={AvailableRoutes::Info}>
                        {"current stack."}
                    </Link<AvailableRoutes>>
                </p>
            </MainView>
        </div>
    }
}
