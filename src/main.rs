mod app;
mod components;
mod pages;
mod navigation;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
