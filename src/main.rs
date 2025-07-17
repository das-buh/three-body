use render::*;
use sim::{Body, System};
use sycamore::{
    prelude::*,
    web::{events::Event, wasm_bindgen::JsCast},
};
use vec::Vec2;

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
    provide_context(Sim {
        system: create_signal(System::default()),
    });

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
struct Sim {
    system: Signal<System>,
}

#[derive(Clone, Copy, Debug)]
struct Settings {
    run: Signal<bool>,
    tickrate: Signal<f64>,
}

#[component]
fn Measurements() -> View {
    let sim = use_context::<Sim>();
    let system = sim.system;
    let settings = use_context::<Settings>();

    let cm = system.map(|s| s.center_mass());
    let energy = system.map(|s| s.energy());
    let momentum = system.map(|s| s.momentum());

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

#[component]
fn Menu() -> View {
    let sim = use_context::<Sim>();
    let system = sim.system;
    let settings = use_context::<Settings>();

    let bodies = system.map(|s| {
        s.bodies()
            .map(|a @ &Body { m, r, v, .. }| (a.id(), m, r, v))
            .collect::<Vec<_>>()
    });

    let add = move |_| {
        if !settings.run.get() {
            system.update(|s| s.add_body(2e30, Vec2(0., 0.), Vec2(0., 0.)))
        }
    };

    let debug = move |_| {
        system.with_untracked(|s| {
            for a @ &Body { m, r, v, .. } in s.bodies() {
                let id = a.id();
                log::debug!("{id} {m} {r} {v}");
            }
        })
    };

    view! {
        div {
            Keyed(
                list=bodies,
                view=|(id, m, r, v)| view! {
                    MenuItem(id=id, m=m, r=r, v=v)
                },
                key=|(id, ..)| *id,
            )
            button(on:click=add) { "add" }
            button(on:click=debug) { "debug" }
        }
    }
}

#[component(inline_props)]
fn MenuItem(id: u64, m: f64, r: Vec2, v: Vec2) -> View {
    let sim = use_context::<Sim>();
    let system = sim.system;
    let settings = use_context::<Settings>();

    fn create_signal_update(
        initial: f64,
        system: Signal<System>,
        get_value: impl Fn(&mut System) -> &mut f64,
    ) -> (Signal<f64>, impl Fn(f64)) {
        let signal = create_signal(initial);
        let update = move |new| system.update(|s| *get_value(s) = new);
        (signal, update)
    }

    let (m, update_m) = create_signal_update(m, system, move |s| &mut s.body_mut(id).m);
    let (rx, update_rx) = create_signal_update(r.0, system, move |s| &mut s.body_mut(id).r.0);
    let (ry, update_ry) = create_signal_update(r.1, system, move |s| &mut s.body_mut(id).r.1);
    let (vx, update_vx) = create_signal_update(v.0, system, move |s| &mut s.body_mut(id).v.0);
    let (vy, update_vy) = create_signal_update(v.1, system, move |s| &mut s.body_mut(id).v.1);

    let delete = move |_| {
        if !settings.run.get() {
            log::debug!("delete {id}");
            system.update(|s| s.remove_body(id))
        }
    };

    view! {
        div {
            Parameter(name="m", id=format!("body-m-{id}"), value=*m, update=update_m)
            Parameter(name="x", id=format!("body-x-{id}"), value=*rx, update=update_rx)
            Parameter(name="y", id=format!("body-y-{id}"), value=*ry, update=update_ry)
            Parameter(name="vx", id=format!("body-vx-{id}"), value=*vx, update=update_vx)
            Parameter(name="vy", id=format!("body-vy-{id}"), value=*vy, update=update_vy)

            button(on:click=delete) { "x" }
        }
    }
}

#[component(inline_props)]
fn Parameter(
    name: &'static str,
    id: String,
    value: ReadSignal<f64>,
    update: impl Fn(f64) + 'static,
) -> View {
    let settings = use_context::<Settings>();

    let input = move |event: Event| {
        let target = event.target().unwrap();
        let input = target
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();

        if let Ok(input) = input.parse::<f64>() {
            update(input);
        }
    };

    let label = id.clone();
    let display = create_memo(move || format!("{:.3e}", value.get()));
    view! {
        div {
            label(r#for=label) { (name) "=" }
            input(id=id, on:input=input, readonly=settings.run, value=display)
        }
    }
}
