//! Form to delete a goat by ID or name.

use crate::errors::AppError;
use log::{error, info};
use yew::prelude::*;

/// Component for deleting a goat.
/// Logs errors on invalid input.
#[function_component(GoatDelete)]
pub fn goat_delete() -> Html {
    let error = use_state(|| None::<String>);

    let onsubmit = {
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Example minimal input check
            let valid_input = false;
            if !valid_input {
                let err = AppError::invalid_input("Provide a valid goat ID or name to delete.");
                error!("Delete goat form error: {}", err);
                error.set(Some(err.to_string()));
                return;
            }

            info!("Delete goat request submitted.");
            error.set(None);
        })
    };

    html! {
        <div style="margin-bottom: 24px;">
            <h2>{"Delete Goat"}</h2>
            {if let Some(err_msg) = &*error {
                html! { <div class="error" style="color: red;">{format!("Error: {}", err_msg)}</div> }
            } else {
                html! {}
            }}
            <form {onsubmit}>
                <input type="text" placeholder="Goat Name or ID" />
                <button type="submit">{"Delete Goat"}</button>
            </form>
        </div>
    }
}
