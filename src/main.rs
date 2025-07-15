use render::*;
use sim::System;
use sycamore::prelude::*;

mod render;
mod sim;
mod vec;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    sycamore::render(App);
}

#[component]
fn App() -> View {
    let sim = Sim(create_signal(System::default()));
    provide_context(sim);

    provide_context(Settings {
        run: create_signal(false),
        tickrate: create_signal(1.),
    });

    view! {
        RenderSim()
        Measurements()
        Menu()
    }
}

#[derive(Clone, Copy)]
struct Sim(Signal<System>);

#[derive(Clone, Copy, Debug)]
struct Settings {
    run: Signal<bool>,
    tickrate: Signal<f64>,
}

#[component]
fn Measurements() -> View {
    let sim = use_context::<Sim>();
    let settings = use_context::<Settings>();

    let cm = sim.0.map(|s| s.center_mass());
    let energy = sim.0.map(|s| s.energy());
    let momentum = sim.0.map(|s| s.momentum());

    create_effect(move || {
        log::debug!("{settings:?}");
    });

    view! {
        div {
            p { "R = " (cm.get().to_string()) }
            p { "K + U = " (energy) }
            p { "p = " (momentum.get().to_string()) }
        }

        label(r#for="settings-run") { "start/stop" }
        input(id="settings-run", r#type="checkbox", bind:checked=settings.run)

        label(r#for="settings-tickrate") { "tickrate" }
        input(id="settings-tickrate", r#type="range", bind:valueAsNumber=settings.tickrate)
    }
}
