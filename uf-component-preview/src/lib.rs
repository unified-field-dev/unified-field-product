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
//! Shipped product app chrome, host auth, session, and search production wiring
//! live outside this crate. Design-system component implementations come from
//! Orbital leaf crates.
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
//! ## Features
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `preview` | Marks this crate as a preview consumer of Orbital / uf-product preview APIs. |
//! | `record-history` | Pulls `record-history-leptos` preview registrations into the catalog (SSR graph only). |
//! | `tag-catalog` | Pulls `tag-app` preview registrations into the catalog (SSR graph only). |
//! | `hydrate` / `ssr` | Leptos client/server split. Hydrate keeps `record-history` / `tag-catalog` off so Valence/SQLite never enters WASM. |
//!
//! Enable `record-history` and `tag-catalog` on the SSR host that builds the catalog
//! binary (see `examples/component-preview-host`). Leave them off hydrate.
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
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | Mount [`OrbitalComponentRoutes`] |
//! | Mid | [`components::examples`] | Teaching `#[component_doc]` widget (`demo-status-pill`) |
//! | Detailed | `examples/component-preview-host` | Full `/orbital` host; enable `record-history` / `tag-catalog` on SSR |
//!
//! ```bash
//! cargo check -p component-preview-host --features ssr
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
