//! Global state management for the goat dashboard app.
//!
//! Uses `yewdux` for reactive state updates,
//! provides asynchronous fetching of goats from backend API,
//! and implements robust error handling and logging.

use std::rc::Rc;

use crate::errors::AppError;
use gloo_net::http::Request;
use log::{error, info};
use shared::GoatParams;
use wasm_bindgen_futures::spawn_local;
use yew::functional::Hook;
use yewdux::prelude::*;

/// Shared global store for the application's goat data.
///
/// Holds the current list of goats,
/// the loading state for ongoing fetches,
/// and any error messages from network or parsing failures.
#[derive(Default, Clone, PartialEq, Store)]
pub struct GoatStore {
    /// The complete list of goats retrieved from backend
    pub goats: Vec<GoatParams>,

    /// True while data is currently being loaded
    pub loading: bool,

    /// Contains error message if the last fetch failed
    pub error: Option<String>,
}

impl GoatStore {
    /// Asynchronously fetches the list of goats from the backend API.
    ///
    /// Issues a GET request to `"http://sample/goats"`.
    /// Updates the store's `goats`, `loading`, and `error` fields as appropriate.
    ///
    /// # Arguments
    ///
    /// * `dispatch` - A `Dispatch` handle to the current `GoatStore` state,
    ///                allowing mutation through yewdux reducers.
    ///
    /// # Behavior
    ///
    /// - Sets `loading` to true and clears previous errors before fetching.
    /// - On success, sets `goats` with received data, `loading` to false, clears errors.
    /// - On failure (network or parse), records error messages and sets `loading` to false.
    ///
    /// # Logging
    ///
    /// Logs info on success and detailed errors on failure.
    pub fn fetch_goats(dispatch: Dispatch<Self>) {
        // Set the loading flag & clear errors before fetching
        dispatch.reduce_mut(|state| {
            state.loading = true;
            state.error = None;
        });

        // Spawn a local future compatible with WASM runtime
        spawn_local(async move {
            let url = String::from("http://127.0.0.1:8000/goats");
            let response = Request::get(&url).send().await;
            info!("Sending fetch_goats request to {}", url);

            match response {
                Ok(resp) => {
                    // Attempt to parse JSON response into Vec<GoatParams>
                    let parse_result = resp.json::<Vec<GoatParams>>().await;

                    match parse_result {
                        Ok(goats) => {
                            info!("Successfully fetched {} goats", goats.len());
                            dispatch.reduce_mut(|state| {
                                state.goats = goats;
                                state.loading = false;
                                state.error = None;
                            });
                        }
                        Err(parse_err) => {
                            let err_msg = format!("Failed to parse goats JSON: {}", parse_err);
                            error!("{}", err_msg);
                            dispatch.reduce_mut(|state| {
                                state.loading = false;
                                state.error = Some(err_msg);
                            });
                        }
                    }
                }
                Err(req_err) => {
                    let err_msg = format!("HTTP request failed: {}", req_err);
                    error!("{}", err_msg);
                    dispatch.reduce_mut(|state| {
                        state.loading = false;
                        state.error = Some(err_msg);
                    });
                }
            }
        });
    }

    /// Adds a new goat synchronously (local update)
    pub fn add_goat(dispatch: Dispatch<Self>, goat: GoatParams) {
        info!("Adding goat locally: {:?}", goat);
        dispatch.reduce_mut(|store| {
            store.goats.push(goat);
        });
    }

    /// Attempts to add a new goat by sending it to the backend.
    ///
    /// On success, updates store state and appends to goats list.
    /// On failure, records error and logs it.
    pub fn add_goat_async(dispatch: Dispatch<Self>, goat: GoatParams) {
        // Set loading state, clear previous errors
        dispatch.reduce_mut(|store| {
            store.loading = true;
            store.error = None;
        });

        // Clone dispatch for use in async context
        spawn_local({
            let dispatch = dispatch.clone();
            async move {
                match Request::post("http://127.0.0.1:8000/goats")
                    .json(&goat)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        info!("Successfully added goat to backend.");
                        dispatch.reduce_mut(|store| {
                            store.goats.push(goat);
                            store.loading = false;
                        });
                    }
                    Ok(resp) => {
                        let err_msg = format!("Server error: HTTP {}", resp.status());
                        error!("{}", err_msg);
                        dispatch.reduce_mut(|store| {
                            store.loading = false;
                            store.error = Some(err_msg.clone());
                        });
                    }
                    Err(net_err) => {
                        let err_msg = format!("Network error: {}", net_err);
                        error!("{}", err_msg);
                        dispatch.reduce_mut(|store| {
                            store.loading = false;
                            store.error = Some(err_msg.clone());
                        });
                    }
                }
            }
        });
    }
}
