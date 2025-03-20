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
        Measurements()
    }
}

#[derive(Clone, Copy)]
struct Sim(Signal<System>);

#[component]
fn Measurements() -> View {
    let sim = use_context::<Sim>();

    let cm = sim.0.map(|system| system.center_mass());
    let energy = sim.0.map(|system| system.energy());
    let momentum = sim.0.map(|system| system.momentum());

    view! {
        p { "R = " (cm.get().to_string()) }
        p { "K + U = " (energy) }
        p { "p = " (momentum.get().to_string()) }
    }
}
