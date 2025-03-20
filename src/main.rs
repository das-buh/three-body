use render::*;
use sim::System;
use sycamore::prelude::*;

mod render;
mod sim;
mod vec;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(App);
}

#[component]
fn App() -> View {
    let sim = Sim(create_signal(System::default()));
    provide_context(sim);

    view! {
        Bodies()
    }
}

#[derive(Clone, Copy)]
struct Sim(Signal<System>);
