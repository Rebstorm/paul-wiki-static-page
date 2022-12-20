use yew::{function_component, html, Html};

use crate::components::{
    main_view::MainView
};

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <MainView title="Stack">
            <h2 id="projects-contributed-to">{"Projects contributed to"}</h2>
            <p><a href="https://github.com/Rebstorm/yah">{"Yah (Yet another home screen)"}</a>{": A home screen with several integrations like SolarEdge, Roomba, Philips Hue and more. Written in TS."}</p>
            <p><a href="https://github.com/denolib/awesome-deno">{"Deno"}</a>{": Collection of Deno list"}</p>
            <p><a href="https://github.com/tw-in-js/twind">{"twind"}</a>{" Tailwind-In-JS"}</p>

            <h2 id="current-stack">{"Current interest"}</h2>
            <img src="static/ts.svg"/>{" "}
            <img src="static/rust.png"/>
            <br />
            <h3 style="margin-bottom: auto" id="written in">{"This page was written in rust using WASM & "} <a href="https://yew.rs/">{"Yew"}</a></h3>
        </MainView>
    }
}
