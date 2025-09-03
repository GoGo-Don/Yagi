use yew::prelude::*;

use crate::components::{Dashboard, Sidebar};

#[function_component(App)]
pub fn app() -> Html {
    // Provide GoatStore context to descendant components
    html! {
        <div style="display: flex; min-height: 100vh;">
            <Sidebar />
            <Dashboard />
        </div>
    }
}
