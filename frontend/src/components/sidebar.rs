//! Sidebar navigation for the goat dashboard app.

use yew::prelude::*;

/// Sidebar UI with navigation buttons.
///
/// Currently static buttons; to be enhanced for routing/navigation.
#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <nav class="sidebar" style="
            width: 220px;
            background-color: #ececec;
            display: flex;
            flex-direction: column;
            gap: 12px;
            padding: 20px 5px;
        ">
            <button>{"Goat List"}</button>
            <button>{"Add Goat"}</button>
            <button>{"Delete Goat"}</button>
            <button>{"Visualizations"}</button>
        </nav>
    }
}
