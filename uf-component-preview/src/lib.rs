#![recursion_limit = "256"]
#![allow(missing_docs)]
//! Component library and preview tool for Orbital UI components ("Orbital" dev app).
//!
//! Renders a registry-driven catalog at `/orbital/{slug}` covering previewable
//! components across Orbital leaf crates (via
//! [`orbital_primitives::preview::collect_all_preview_registrations`]), plus
//! Unified Field–specific previews and layout examples. Primary teaching path is
//! the live `/orbital` app (see `examples/component-preview-host`); rustdoc indexes
//! the mount APIs integrators call.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | `/orbital` route tree ([`OrbitalComponentRoutes`]) and catalog shell | Shipped product app chrome (`uf-integrations`) |
//! | Registry merge + slug-routed preview pages | Design-system component implementations (Orbital / Zone A) |
//! | Preview-only layout placeholders and fixtures | Host auth, session, or search production wiring |
//!
//! ## Features
//!
//! - **Registry-driven catalog** — [`preview::collect_preview_registrations`] merges
//!   static registrations from every Orbital preview-enabled crate plus this crate's
//!   own [`preview::fixtures`] and hand-written pages; [`preview::PreviewSlugPage`]
//!   renders whichever registration matches the current route.
//! - **Catalog shell** — [`preview::PreviewCatalogShell`], [`preview::PreviewCatalogNav`],
//!   and [`preview::PreviewCatalogSearch`] provide the AppBar/sidebar/search chrome.
//! - **Layout examples** — [`components::layouts`] holds preview-only placeholder
//!   layout compositions (calendar, chat, files, plugin settings, activity feed).
//! - **Registered app** — this crate registers itself as the `"orbital"` Orbital app
//!   via `uf_app!`, mounted at `/orbital`.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Mount the Orbital catalog routes | [`OrbitalComponentRoutes`], [`paths`] |
//! | Build / merge the preview registry | [`preview::collect_preview_registrations`] |
//! | Render a slug from the registry | [`preview::PreviewSlugPage`] |
//! | Catalog chrome (nav / search / shell) | [`preview::PreviewCatalogShell`], [`preview::PreviewCatalogNav`], [`preview::PreviewCatalogSearch`] |
//! | Prefetch a component family | [`prefetch_family`] |
//!
//! ## Getting started
//!
//! Mount [`OrbitalComponentRoutes`] inside your host's `<Routes>` (typically only in
//! non-production builds):
//!
//! ```rust,ignore
//! use leptos_router::components::Routes;
//! use uf_component_preview::OrbitalComponentRoutes;
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <OrbitalComponentRoutes />
//!     </Routes>
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`OrbitalComponentRoutes`] — the `/orbital` route tree.
//! - [`preview`] — the registry, catalog shell, and slug-routed preview page.
//! - [`pages`] — the introduction page and top-level dev layout.
//! - `examples/component-preview-host` — mounts this catalog in-workspace.
//! - [`components::examples`] — teaching `#[component_doc]` widget (`demo-status-pill`).

use uf_product_macros::uf_app;

mod lazy_routes;

/// Preview-only components: hand-written component previews and layout examples.
pub mod components;
/// Top-level pages: the introduction page and the dev shell layout.
pub mod pages;
/// The preview registry, catalog shell chrome, and slug-routed preview page.
pub mod preview;
/// The `/orbital` route tree.
pub mod routes;

// Re-export route components and generated paths for easy use
pub use lazy_routes::prefetch_family;
pub use routes::paths;
pub use routes::OrbitalComponentRoutes;
/// Back-compat alias for hosts that still import [`OrbitalDevRoutes`].
pub use routes::OrbitalComponentRoutes as OrbitalDevRoutes;

// Define the Orbital component preview application metadata.
uf_app! {
    name: "Orbital",
    id: "orbital",
    description: "Component library and preview tool for Orbital UI components",
    icon: "🧩",
    version: "0.1.0",
    routes: OrbitalComponentRoutes,
    route_path: "/orbital",
}
