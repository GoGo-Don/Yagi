//! Form to add a new goat to the global store and backend.
//! Features:
//! - Controlled local form state
//! - Field-level validation and global error handling
//! - Logging for all stages
//! - Calls async store action to submit to backend

use crate::components::add_goat_components::{BreedInput, GenderInput};
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
    let name = use_state(|| "".to_string());
    let breed = use_state(|| "Beetal".to_string());
    let other_breed = use_state(|| "".to_string());
    let gender = use_state(|| "Male".to_string());
    let offspring = use_state(|| "".to_string());
    let cost = use_state(|| "".to_string());
    let weight = use_state(|| "".to_string());
    let current_price = use_state(|| "".to_string());
    let diet = use_state(|| "".to_string());
    let last_bred = use_state(|| "".to_string());
    let health_status = use_state(|| "".to_string());

    // Callbacks for breed/gender (used by subcomponents)
    let update_breed = {
        let breed = breed.clone();
        Callback::from(move |new_breed: String| breed.set(new_breed))
    };
    let update_other_breed = {
        let other_breed = other_breed.clone();
        Callback::from(move |new_other: String| other_breed.set(new_other))
    };
    let update_gender = {
        let gender = gender.clone();
        Callback::from(move |new_gen: String| gender.set(new_gen))
    };

    let error = use_state(|| None::<String>);

    let (_state, dispatch) = use_store::<GoatStore>();

    let onsubmit = {
        let name = name.clone();
        let breed = breed.clone();
        let other_breed = other_breed.clone();
        let gender = gender.clone();
        let offspring = offspring.clone();
        let cost = cost.clone();
        let weight = weight.clone();
        let current_price = current_price.clone();
        let diet = diet.clone();
        let last_bred = last_bred.clone();
        let health_status = health_status.clone();
        let error = error.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            // Validate required fields (example for name, can add more)
            if name.trim().is_empty() {
                error!("Validation failed: Name missing.");
                error.set(Some("Name is required.".to_string()));
                return;
            }
            if breed.trim().is_empty() {
                error!("Validation failed: Breed missing.");
                error.set(Some("Breed is required.".to_string()));
                return;
            }
            if gender.trim().is_empty() {
                error!("Validation failed: Gender missing.");
                error.set(Some("Gender is required.".to_string()));
                return;
            }

            // Parse f64 fields, with error handling for invalid inputs
            let offspring_val = match offspring.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    error!("Validation failed: Offspring not number.");
                    error.set(Some("Offspring must be a number.".to_string()));
                    return;
                }
            };
            let cost_val = match cost.parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    error!("Validation failed: cost not number.");
                    error.set(Some("Cost must be a number.".to_string()));
                    return;
                }
            };
            let weight_val = match weight.parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    error!("Validation failed: Weight not number.");
                    error.set(Some("Weight must be a number.".to_string()));
                    return;
                }
            };
            let current_price_val = match current_price.parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    error!("Validation failed: Current price not number.");
                    error.set(Some("Current price must be a number.".to_string()));
                    return;
                }
            };

            // Build breed and gender
            let selected_breed = if breed.as_str() == "Other" {
                Breed::Other((*other_breed).clone())
            } else {
                Breed::from_str(&(*breed).clone())
            };

            let selected_gender = Gender::from_str(&(*gender).clone()).unwrap();

            // Create GoatParams (handle other fields or optional fields as needed)
            let goat = GoatParams {
                name: (*name).clone(),
                breed: selected_breed,
                gender: selected_gender,
                offspring: offspring_val as i32, // or keep as f64 if that's what your struct expects
                cost: cost_val,
                weight: weight_val,
                current_price: current_price_val,
                diet: (*diet).clone(),
                last_bred: Some((*last_bred).clone()),
                health_status: (*health_status).clone(),
                vaccinations: vec![],
                diseases: vec![],
            };

            // Call the store: GoatStore::add_goat_async(dispatch, new_goat);

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
            if let Some(msg) = &*error {
                <p style="color: red;">{msg}</p>
            }
            <form onsubmit={onsubmit}>
                <label>{ "Name:" }
                    <input
                        type="text"
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
                </label>
                <br/>

                <label>{ "Breed:" }
                    <BreedInput
                        selected={(*breed).clone()}
                        other_value={(*other_breed).clone()}
                        on_breed_change={update_breed}
                        on_other_change={update_other_breed}
                    />
                </label>
                <br/>

                <label>{ "Gender:" }
                    <GenderInput
                        selected={(*gender).clone()}
                        on_gender_change={update_gender}
                    />
                </label>
                <br/>

                <label>{ "Offspring:" }
                    <input
                        type="number"
                        value={(*offspring).clone()}
                        oninput={Callback::from({
                            let offspring = offspring.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    offspring.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Cost:" }
                    <input
                        type="number"
                        step="0.01"
                        value={(*cost).clone()}
                        oninput={Callback::from({
                            let cost = cost.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    cost.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Weight:" }
                    <input
                        type="number"
                        step="0.01"
                        value={(*weight).clone()}
                        oninput={Callback::from({
                            let weight = weight.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    weight.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Current Price:" }
                    <input
                        type="number"
                        step="0.01"
                        value={(*current_price).clone()}
                        oninput={Callback::from({
                            let current_price = current_price.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    current_price.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Diet:" }
                    <input
                        type="text"
                        value={(*diet).clone()}
                        oninput={Callback::from({
                            let diet = diet.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    diet.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Last Bred:" }
                    <input
                        type="text"
                        value={(*last_bred).clone()}
                        oninput={Callback::from({
                            let last_bred = last_bred.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    last_bred.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>

                <label>{ "Health Status:" }
                    <input
                        type="text"
                        value={(*health_status).clone()}
                        oninput={Callback::from({
                            let health_status = health_status.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    health_status.set(input.value());
                                }
                            }
                        })}
                    />
                </label>
                <br/>
                <button type="submit">{"Add Goat"}</button>
            </form>
        </div>
    }
}
