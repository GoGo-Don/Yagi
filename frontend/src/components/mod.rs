//! The components module groups all reusable UI components for the goat dashboard app.
//! This mod.rs makes components accessible when imported as `crate::components::*`.

pub mod add_goat_components;
pub mod add_goat_form;
pub mod dashboard;
pub mod delete_goat_form;
pub mod goat_list;
pub mod sidebar;
pub mod update_goat_form;

// Optionally re-export for easier import elsewhere
pub use add_goat_form::AddGoatForm;
pub use dashboard::Dashboard;
pub use delete_goat_form::DeleteGoatsForm;
pub use goat_list::GoatList;
pub use sidebar::Sidebar;
pub use update_goat_form::UpdateGoatForm;

