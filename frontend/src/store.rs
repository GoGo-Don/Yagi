//! Global state management for the goat dashboard app.
//!
//! Uses `yewdux` for reactive state updates,
//! provides asynchronous fetching of goats from backend API,
//! and implements robust error handling and logging.

use crate::errors::AppError;
use gloo_net::http::Request;
use log::{error, info, trace, warn};
use shared::GoatParams;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
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
    /// * `dinamespatch` - A `Dispatch` handle to the current `GoatStore` state,
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

    /// Asynchronously attempts to delete a goat by name from the backend and updates the local store accordingly.
    /// Reports success or returns an error if unable to delete the goat.
    ///
    /// --------ARGUMENTS---------
    ///
    /// - `dispatch`:   Dispatch<Self>
    ///                 A `Dispatch` handle to the current `GoatStore` state,
    ///                 allowing mutation through yewdux reducers.
    /// - `goat_name`:  String
    ///                 The name of the goat to be deleted (case-sensitive).
    /// - `on_result`:  Callback<Result<(), AppError>>
    ///                 Callback called after the delete attempt finishes,
    ///                 with `Ok(())` on success or an `AppError` variant on failure.
    ///
    /// --------RETURNS----------
    ///
    /// - This function does not return a value directly;
    ///   all results and errors are communicated via the provided `on_result` callback.
    ///
    /// ------UPGRADE PENDING------
    ///     Retry or user confirmation for network/server errors.
    pub fn delete_goat_async(
        dispatch: Dispatch<Self>,
        goat_name: String,
        on_result: Callback<Result<(), AppError>>,
    ) {
        spawn_local(async move {
            trace!("Deleting goat {}", goat_name);
            // Attempt HTTP DELETE, expecting backend to handle /goats/{name}
            let url = "http://127.0.0.1:8000/goats";
            let body = serde_json::json!({ "name": goat_name });
            let outcome = match Request::delete(url).json(&body).unwrap().send().await {
                Ok(response) if response.ok() => {
                    dispatch.reduce_mut(|store| {
                        let initial_len = store.goats.len();
                        store.goats.retain(|g| g.name != goat_name);
                        if store.goats.len() < initial_len {
                            info!("Deleted goat '{}' from local store and backend.", goat_name);
                        } else {
                            warn!("Goat '{}' not found in local store, but backend deletion succeeded.", goat_name);
                        }
                    });
                    Ok(())
                }
                Ok(response) => {
                    let msg = format!(
                        "Server error {} for deleting '{}'",
                        response.status(),
                        goat_name
                    );
                    error!("{}", msg);
                    Err(AppError::Unexpected(msg))
                }
                Err(e) => {
                    let msg = format!("Network error: {} while deleting '{}'", e, goat_name);
                    error!("{}", msg);
                    Err(AppError::NetworkError(msg))
                }
            };
            on_result.emit(outcome);
        });
    }

    /// Asynchronously sends an updated goat record to the backend server and updates the local store on success.
    ///
    /// --------ARGUMENTS---------
    ///
    /// - `dispatch`:    Dispatch<Self>
    ///                  A `Dispatch` handle to the current `GoatStore` state,
    ///                  allowing mutation through yewdux reducers.
    /// - `updated_goat`: GoatParams
    ///                  Struct containing the updated details of the goat.
    /// - `on_result`:   Callback<Result<(), AppError>>
    ///                  Callback which receives `Ok(())` on successful update,
    ///                  or an `AppError` detailing any failure.
    ///
    /// --------RETURNS----------
    ///
    /// - This function returns nothing directly; it reports via the `on_result` callback.
    ///
    /// ------UPGRADE PENDING------
    /// - Improve retry logic on network errors.
    /// - Add validation or conflict resolution based on backend response.
    pub fn update_goat_async(
        dispatch: Dispatch<Self>,
        updated_goat: GoatParams,
        on_result: Callback<Result<(), AppError>>,
    ) {
        spawn_local(async move {
            trace!("Updating goat");
            // Assume your backend expects PUT with JSON payload at /goats/{name}
            let url = "http://127.0.0.1:8000/goats";
            let response = Request::put(&url).json(&updated_goat).unwrap().send().await;

            let outcome = match response {
                Ok(resp) if resp.ok() => {
                    // Update local store on success
                    dispatch.reduce_mut(|store| {
                        if let Some(pos) =
                            store.goats.iter().position(|g| g.name == updated_goat.name)
                        {
                            store.goats[pos] = updated_goat.clone();
                        } else {
                            // Optionally add if not found
                            store.goats.push(updated_goat.clone());
                        }
                    });
                    info!("Successfully updated goat '{}'", updated_goat.name);
                    Ok(())
                }
                Ok(resp) => {
                    let msg = format!(
                        "Server returned error {} while updating '{}'",
                        resp.status(),
                        updated_goat.name
                    );
                    error!("{}", msg);
                    Err(AppError::Unexpected(msg))
                }
                Err(err) => {
                    let msg = format!(
                        "Network error while updating '{}': {}",
                        updated_goat.name, err
                    );
                    error!("{}", msg);
                    Err(AppError::NetworkError(msg))
                }
            };
            on_result.emit(outcome);
        });
    }
}
