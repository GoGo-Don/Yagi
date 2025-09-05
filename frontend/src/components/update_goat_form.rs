use crate::components::add_goat_components::{BreedInput, GenderInput};
use crate::store::GoatStore;
use log::{error, info, trace, warn};
use shared::{Breed, Gender, GoatParams};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(UpdateGoatForm)]
pub fn update_goat_form() -> Html {
    let (state, dispatch) = use_store::<GoatStore>();

    // States for inputs and control flow
    let search_name = use_state(|| "".to_string());
    let found_goat = use_state(|| None::<GoatParams>);
    let error = use_state(|| None::<String>);
    let success = use_state(|| None::<String>);

    // Editable fields state
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

    // Handler to load goat details from store by name
    let on_search = {
        let search_name = search_name.clone();
        let state = state.clone();
        let found_goat = found_goat.clone();
        let error = error.clone();
        let success = success.clone();

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

        Callback::from(move |_| {
            success.set(None);
            error.set(None);
            let query = (*search_name).to_lowercase();
            if query.is_empty() {
                error.set(Some("Please enter the name of goat to update".to_string()));
                found_goat.set(None);
                return;
            }
            if let Some(goat) = state.goats.iter().find(|g| g.name.to_lowercase() == query) {
                found_goat.set(Some(goat.clone()));
                name.set(goat.name.clone());

                // Use to_str methods to populate strings
                breed.set(match &goat.breed {
                    Breed::Other(_) => "Other".to_string(),
                    other => Breed::to_str(other).to_string(),
                });
                other_breed.set(if let Breed::Other(s) = &goat.breed {
                    s.clone()
                } else {
                    "".to_string()
                });
                gender.set(Gender::to_str(&goat.gender).to_string());

                offspring.set(goat.offspring.to_string());
                cost.set(goat.cost.to_string());
                weight.set(goat.weight.to_string());
                current_price.set(goat.current_price.to_string());
                diet.set(goat.diet.clone());
                last_bred.set(goat.last_bred.clone().unwrap_or_default());
                health_status.set(goat.health_status.clone());
                error.set(None);
            } else {
                error.set(Some(format!("Goat '{}' not found", *search_name)));
                found_goat.set(None);
            }
        })
    };

    // Submit handler
    let onsubmit = {
        let dispatch = dispatch.clone();
        let found_goat = found_goat.clone();
        let error = error.clone();
        let success = success.clone();

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

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            let mut errs = Vec::new();

            if name.trim().is_empty() {
                errs.push("Name must not be empty.");
            }
            if breed.trim().is_empty() {
                errs.push("Breed must be selected.");
            }
            if gender.trim().is_empty() {
                errs.push("Gender must be selected.");
            }

            let offspring_val = offspring.parse::<i32>().unwrap_or_else(|_| {
                errs.push("Offspring must be integer.");
                0
            });
            let cost_val = cost.parse::<f64>().unwrap_or_else(|_| {
                errs.push("Cost must be number.");
                0.0
            });

            let weight_val = weight.parse::<f64>().unwrap_or_else(|_| {
                errs.push("Weight must be number.");
                0.0
            });
            let current_price_val = current_price.parse::<f64>().unwrap_or_else(|_| {
                errs.push("Current price must be number.");
                0.0
            });

            if !errs.is_empty() {
                error.set(Some(errs.join(" ")));
                success.set(None);
                return;
            }

            let breed_enum = if breed.as_str() == "Other" {
                Breed::Other(other_breed.to_string())
            } else {
                Breed::from_str(&breed.to_string())
            };

            let gender_enum = Gender::from_str(gender.as_str()).unwrap_or(Gender::Male);

            if found_goat.is_none() {
                error.set(Some("No goat loaded to update".to_string()));
                success.set(None);
                return;
            }

            let updated = GoatParams {
                name: name.to_string(),
                breed: breed_enum,
                gender: gender_enum,
                offspring: offspring_val,
                cost: cost_val,
                weight: weight_val,
                current_price: current_price_val,
                diet: diet.to_string(),
                last_bred: Some(last_bred.to_string()),
                health_status: health_status.to_string(),
                vaccinations: found_goat.as_ref().unwrap().vaccinations.clone(),
                diseases: found_goat.as_ref().unwrap().diseases.clone(),
            };

            dispatch.reduce_mut(|store| {
                if let Some(pos) = store.goats.iter().position(|g| g.name == updated.name) {
                    store.goats[pos] = updated.clone();
                }
            });

            let dispatch = dispatch.clone();
            let error = error.clone();
            let success = success.clone();
            GoatStore::update_goat_async(
                dispatch,
                updated.clone(),
                Callback::from(move |res| match res {
                    Ok(_) => {
                        trace!("Deleted successfully.");
                        error.set(None);
                        success.set(Some("Goat updated successfully.".to_string()));
                    }
                    Err(e) => error.set(Some(format!("Failed: {}", e))),
                }),
            );
        })
    };

    html! {
        <div>
            <h3>{ "Update Goat Details" }</h3>

            // Search Box
            <input
                type="text"
                placeholder="Goat name to edit"
                value={(*search_name).clone()}
                oninput={Callback::from({
                    let search_name = search_name.clone();
                    move |e: InputEvent| {
                        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                            search_name.set(input.value());
                        }
                    }
                })}
            />
            <button onclick={on_search}>{ "Load Goat" }</button>

            // Messages
            if let Some(err) = &*error {
                <p style="color: red;">{err.clone()}</p>
            } else if let Some(msg) = &*success {
                <p style="color: green;">{msg.clone()}</p>
            }

            if found_goat.is_some() {
                <form onsubmit={onsubmit}>
                    <label>{ "Name:" }
                        <input type="text"
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
                            on_breed_change={Callback::from(move |v| breed.set(v))}
                            on_other_change={Callback::from(move |v| other_breed.set(v))}
                        />
                    </label>
                    <br/>
                    <label>{ "Gender:" }
                        <GenderInput
                            selected={(*gender).clone()}
                            on_gender_change={Callback::from(move |v| gender.set(v))}
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
                    <button type="submit">{ "Save Changes" }</button>
                </form>
            }
        </div>
    }
}
