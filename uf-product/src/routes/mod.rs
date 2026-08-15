//! Route registration, redirect hygiene, and auth-aware route guards.
//!
//! Orbital splits routing into two layers:
//!
//! - App metadata registration through `AppRegistration` and the inventory-backed
//!   `AppRegistry` on SSR builds.
//! - Runtime UI guards such as [`RequireAuthenticated`] for pages that need
//!   sign-in, verified email, or a specific permission.
//!
//! # Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | [`RequireAuthenticated`] gate UI and redirect hygiene | Host axum-login middleware (lepton-auth) |
//! | SSR `AppRegistration` / `AppRegistry` discovery | `uf_app!` expansion (`uf-product-macros`) and build-time scan (`uf-codegen`) |
//! | Safe `referer` parsing for post-auth redirects | Shell chrome / left nav (`uf-integrations`) |
//!
//! Named permission checks currently fail closed (Gauge not wired yet).
//!
//! # Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Gate page content behind login / permission | [`RequireAuthenticated`] |
//! | Tell Help a sign-in (or other) gate is showing | [`provide_access_gate_state`], [`AccessGateActive`] |
//! | Discover apps registered by `uf_app!` (SSR) | [`AppRegistration`], [`AppRegistry`] |
//! | Parse safe redirect targets from referer | [`parse_referer_from_search`], [`sanitize_referer_path`] |
//! | Auth route + return path when no shell dialog | [`auth_signin_href`], [`auth_signup_href`] |
//!
//! # Example
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use uf_product::routes::RequireAuthenticated;
//!
//! #[component]
//! fn CounterAdminPage() -> impl IntoView {
//!     view! {
//!         <RequireAuthenticated permission_name=Some("counter.admin.set")>
//!             <h1>"Counter Admin"</h1>
//!         </RequireAuthenticated>
//!     }
//! }
//! ```
//!
//! SSR registration (becomes an [`AppRegistration`] in [`AppRegistry`]):
//!
//! ```rust,ignore
//! uf_product_macros::uf_app! {
//!     id: "counter",
//!     name: "Counter",
//!     description: "Realtime shared counter example",
//!     icon: "NumberSymbolSquare24Regular",
//!     route_path: "/counter"
//! }
//! ```
//!
//! Mid / detailed: `uf-product/examples/uf_app_registration`, `app_route_paths`,
//! `auth_shell_host`.

mod access_gate;
mod referer;
#[cfg(feature = "ssr")]
mod registration;
mod require_authenticated;

pub use access_gate::{provide_access_gate_state, use_access_gate_active, AccessGateActive};
pub use referer::{
    auth_path_with_referer, auth_signin_href, auth_signup_href, parse_referer_from_search,
    sanitize_referer_path,
};
#[cfg(feature = "ssr")]
pub use registration::{get_all_app_route_paths, AppRegistration, AppRegistry};
pub use require_authenticated::{authenticated_route_condition, RequireAuthenticated};
