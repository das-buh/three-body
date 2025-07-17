use crate::{Sim, vec::Vec2};
use sycamore::prelude::*;

#[component]
pub fn RenderSim() -> View {
    let sim = use_context::<Sim>();

    let bodies = sim
        .system
        .map(|s| s.bodies().map(|a| (a.id(), a.r)).collect::<Vec<_>>());

    view! {
        Indexed(
            list=bodies,
            view=|(id, Vec2(x, y))| view! {
                p { (format!("(i={id} r={x},{y})")) }
            },
        )
    }
}
