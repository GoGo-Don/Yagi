//! Entrypoint of the Yew goat management webapp.
//! Initializes wasm_logger for descriptive logging in browser console.

use frontend::app;
use wasm_logger;
use yew::Renderer;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    Renderer::<app::App>::new().render();
}
