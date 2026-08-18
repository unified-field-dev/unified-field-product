//! Registry-driven preview navigation and routing helpers.
//!
//! Merges Orbital and local preview registrations into a catalog shell with
//! nav, search, and slug-routed preview pages. Leaf component implementations
//! live in Orbital crates; product shell layout in `uf-integrations`.
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
