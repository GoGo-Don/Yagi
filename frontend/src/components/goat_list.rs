//! Component to display the list of goats in a compact table with
//! asynchronous loading and error states handled gracefully.
//!
//! It triggers fetching on mount and provides a Refresh button,
//! leveraging application store state for consistency.

use crate::store::GoatStore;
use log::info;
use yew::prelude::*;
use yewdux::prelude::use_store;

/// GoatList component:
/// Shows all goats fetched from backend in a compact HTML table.
/// Shows loading text and error messages based on global store state.
///
/// Features:
/// - Fetch goats list asynchronously on mount.
/// - Refresh button to re-fetch the goats list.
/// - Informative logging.
/// - Shows error messages in UI if fetch fails.
#[function_component(GoatList)]
pub fn goat_list() -> Html {
    let (state, dispatch) = use_store::<GoatStore>();

    // Fetch goats once on mount, using empty dependencies ()
    use_effect_with(
        (), // empty tuple dependencies means run once
        {
            let dispatch = dispatch.clone();
            move |_| {
                GoatStore::fetch_goats(dispatch);
                || {}
            }
        },
    );

    // Callback for Refresh button to fetch goats again
    let refresh = {
        Callback::from(move |_| {
            GoatStore::fetch_goats(dispatch.clone());
        })
    };

    // Render UI based on current loading/error state from store
    html! {
        <div style="margin-bottom: 24px;">
            <h2>{"All Goats"}</h2>

            // Show loading indicator
            if state.loading {
                <p>{"Loading goats..."}</p>
            }
            // Show error message if any
            else if let Some(err_msg) = &state.error {
                <p style="color: red;">{format!("Error loading goats: {}", err_msg)}</p>
            }
            // Otherwise, render the goats table with refresh option
            else {
                <>
                    <button onclick={refresh} style="margin-bottom: 10px;">{"Refresh"}</button>
                    <div style="overflow-x: auto;">
                        <table style="border-collapse: collapse; width: 100%;">
                            <thead>
                                <tr>
                                    <th>{"Name"}</th>
                                    <th>{"Breed"}</th>
                                    <th>{"Gender"}</th>
                                    <th>{"Offspring"}</th>
                                    <th>{"Cost"}</th>
                                    <th>{"Weight"}</th>
                                    <th>{"Current Price"}</th>
                                    <th>{"Diet"}</th>
                                    <th>{"Last Bred"}</th>
                                    <th>{"Health Status"}</th>
                                    <th>{"Vaccinations"}</th>
                                    <th>{"Diseases"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    for state.goats.iter().map(|goat| html! {
                                        <tr>
                                            <td>{&goat.name}</td>
                                            <td>{format!("{:?}", goat.breed)}</td>
                                            <td>{format!("{:?}", goat.gender)}</td>
                                            <td>{goat.offspring}</td>
                                            <td>{format!("{:.2}", goat.cost)}</td>
                                            <td>{format!("{:.2}", goat.weight)}</td>
                                            <td>{format!("{:.2}", goat.current_price)}</td>
                                            <td>{&goat.diet}</td>
                                            <td>{goat.last_bred.as_deref().unwrap_or("-")}</td>
                                            <td>{&goat.health_status}</td>
                                            <td>{format!("{:?}", goat.vaccinations)}</td>
                                            <td>{format!("{:?}", goat.diseases)}</td>
                                        </tr>
                                    })
                                }
                            </tbody>
                        </table>
                    </div>
                </>
            }
        </div>
    }
}
