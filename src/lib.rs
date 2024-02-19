mod app;
mod components;
mod models;

use app::App;
use wasm_bindgen::prelude::*;
use yew::start_app;

#[wasm_bindgen(start)]
pub fn run_app() {
    start_app::<App>();
}
