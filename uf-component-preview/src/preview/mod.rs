//! Registry-driven preview navigation and routing helpers.
//!
//! # Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | Merging Orbital + local preview registrations | Leaf component implementations (Orbital crates) |
//! | Catalog shell, nav, search, and slug page | Product shell layout (`uf-integrations`) |
//!
//! # Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Collect all preview registrations | [`collect_preview_registrations`] |
//! | Route one slug to its preview | [`PreviewSlugPage`], [`preview_page`] |
//! | Catalog chrome | [`PreviewCatalogShell`], [`PreviewCatalogNav`], [`PreviewCatalogSearch`] |
//!
//! Teaching path: live `/orbital` via `examples/component-preview-host`.

mod catalog_nav;
mod catalog_search;
mod catalog_shell;
pub mod fixtures;
mod manual_registrations;
mod nav;
mod navigation;
mod paths;
mod registry;
mod slug_page;
mod theme_toggle;

#[cfg(test)]
mod registry_tests;

pub use catalog_nav::PreviewCatalogNav;
pub use catalog_search::PreviewCatalogSearch;
pub use catalog_shell::PreviewCatalogShell;
pub use nav::PreviewNav;
pub use paths::preview_page;
pub use registry::collect_preview_registrations;
pub use slug_page::PreviewSlugPage;
pub use uf_product::preview::PreviewRegistration;
