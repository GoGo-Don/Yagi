//! The components module groups all reusable UI components for the goat dashboard app.
//! This mod.rs makes components accessible when imported as `crate::components::*`.

pub mod add_goat_form;
pub mod dashboard;
pub mod goat_delete;
pub mod goat_list;
pub mod sidebar;

// Optionally re-export for easier import elsewhere
pub use add_goat_form::AddGoatForm;
pub use dashboard::Dashboard;
pub use goat_delete::GoatDelete;
pub use goat_list::GoatList;
pub use sidebar::Sidebar;
