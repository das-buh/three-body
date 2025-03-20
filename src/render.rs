use crate::{vec::Vec2, Sim};
use sycamore::prelude::*;

#[component]
pub fn RenderSim() -> View {
    let sim = use_context::<Sim>();

    let bodies = sim.0.map(|system| {
        system
            .bodies()
            .map(|(i, _, p, _)| (i, p))
            .collect::<Vec<_>>()
    });

    view! {
        Indexed(
            list=bodies,
            view=|(i, Vec2(x, y))| view! {
                p { (format!("(i={i} p={x},{y})")) }
            },
        )
    }
}
