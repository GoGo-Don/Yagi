//! Main dashboard content area component.

use crate::components::{AddGoatForm, GoatDelete, GoatList};
use yew::prelude::*;

/// Dashboard area showing goat list, forms, and visualizations placeholder.
///
/// Currently all rendered for skeleton display.
#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="dashboard" style="flex: 1; padding: 24px;">
            <h1>{"Dashboard"}</h1>
            <GoatList />
            <AddGoatForm />
            <GoatDelete />
            <div style="border: 1px dashed #bbb; margin-top: 30px; padding: 16px;">
                <h3>{"Visualizations"}</h3>
                <p>{"Graphs and analytics coming soon!"}</p>
            </div>
        </div>
    }
}
