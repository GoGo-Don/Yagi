use crate::store::GoatStore;
use log::{info, warn};
use std::collections::{HashMap, HashSet};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(DeleteGoatsForm)]
pub fn delete_goats_form() -> Html {
    let names_input = use_state(|| "".to_string());
    let results = use_state(|| HashMap::<String, String>::new());
    let (_state, dispatch) = use_store::<GoatStore>();

    // OnSubmit handler
    let onsubmit = {
        let names_input = names_input.clone();
        let results = results.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();

            let value = names_input.trim();
            // Split names, trim, and deduplicate
            let names: Vec<String> = value
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<HashSet<_>>() // deduplicate
                .into_iter()
                .collect();

            if names.is_empty() {
                let mut map = HashMap::new();
                map.insert(
                    "".to_string(),
                    "Please enter at least one name.".to_string(),
                );
                results.set(map);
                return;
            }

            // Clear old results and begin new deletes
            results.set(HashMap::new());

            for name in names {
                let dispatch = dispatch.clone();
                let name_clone = name.clone();
                let results = results.clone();

                GoatStore::delete_goat_async(
                    dispatch,
                    name.clone(),
                    Callback::from(move |res| {
                        results.set({
                            let mut new_map = (*results).clone();
                            match res {
                                Ok(_) => {
                                    new_map.insert(
                                        name_clone.clone(),
                                        "Deleted successfully.".to_string(),
                                    );
                                }
                                Err(e) => {
                                    new_map.insert(name_clone.clone(), format!("Failed: {}", e));
                                }
                            }
                            new_map
                        });
                    }),
                );
            }

            names_input.set("".to_string());
        })
    };

    // Handler for input field value
    let oninput = {
        let names_input = names_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                names_input.set(input.value());
            }
        })
    };

    html! {
        <div>
            <h3>{ "Delete Goats" }</h3>
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    placeholder="Goat names, comma separated"
                    value={(*names_input).clone()}
                    oninput={oninput}
                />
                <button type="submit">{ "Delete" }</button>
            </form>
            <ul>
                { for results.iter().map(|(name, msg)| html! {
                    <li>
                        <b>{ name }</b>{ ": " }{ msg }
                    </li>
                })}
            </ul>
        </div>
    }
}
