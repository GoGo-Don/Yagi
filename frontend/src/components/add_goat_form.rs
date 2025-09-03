//! Form to add a new goat to the global store and backend.
//! Features:
//! - Controlled local form state
//! - Field-level validation and global error handling
//! - Logging for all stages
//! - Calls async store action to submit to backend

use crate::store::GoatStore;
use log::{error, info};
use shared::{Breed, DiseaseRef, Gender, GoatParams, VaccineRef};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

/// Helper function to trim and check a string input field.
fn required_field(field: &str) -> Option<String> {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        Some("This field is required.".to_owned())
    } else {
        None
    }
}

/// The AddGoatForm component allows entry and submission of a new Goat.
#[function_component(AddGoatForm)]
pub fn add_goat_form() -> Html {
    // Form state for fields and validation errors
    let name = use_state(|| "".to_owned());
    let breed = use_state(|| "".to_owned());
    let error = use_state(|| None::<String>);

    let (_state, dispatch) = use_store::<GoatStore>();

    // Submit handler for the form
    let onsubmit = {
        let name = name.clone();
        let breed = breed.clone();
        let error = error.clone();
        let dispatch = dispatch.clone();
        let vaccinations: Vec<VaccineRef> = Vec::new();
        let diseases: Vec<DiseaseRef> = Vec::new();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            // Validate fields
            if let Some(err) = required_field(&name) {
                error!("Validation failed: Name missing.");
                error.set(Some(format!("Name: {err}")));
                return;
            }
            if let Some(err) = required_field(&breed) {
                error!("Validation failed: Breed missing.");
                error.set(Some(format!("Breed: {err}")));
                return;
            }

            // Build goat
            let goat = GoatParams {
                name: (*name).clone(),
                breed: Breed::Beetal,
                gender: Gender::Male,
                offspring: 1,
                cost: 100.0,
                weight: 50.0,
                current_price: 120.0,
                diet: String::from("Hay"),
                last_bred: None,
                health_status: String::from("Healthy"),
                vaccinations: vaccinations.clone(),
                diseases: diseases.clone(),
            };

            // Make request to backend and update store
            info!("Submitting new goat: {:?}", goat);
            GoatStore::add_goat_async(dispatch.clone(), goat);

            // Optionally, reset form state
            name.set("".to_owned());
            breed.set("".to_owned());
            error.set(None);
        })
    };

    html! {
        <div>
            <h3>{"Add Goat"}</h3>
            { if let Some(msg) = &*error {
                html! { <p style="color: red;">{msg}</p> }
            } else {
                html! {}
            }}
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    placeholder="Goat Name"
                    value={(*name).clone()}
                    oninput={Callback::from({
                        let name = name.clone();
                        move |e: InputEvent| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                name.set(input.value());
                            }
                        }
                    })}
                />
                <input
                    type="text"
                    placeholder="Breed"
                    value={(*breed).clone()}
                    oninput={Callback::from({
                        let breed = breed.clone();
                        move |e: InputEvent| {
                            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                breed.set(input.value());
                            }
                        }
                    })}
                />
                <button type="submit">{"Add Goat"}</button>
            </form>
        </div>
    }
}
