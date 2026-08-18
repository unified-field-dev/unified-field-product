#![recursion_limit = "256"]
//! Signed-in welcome landing page for Unified Field product hosts.
//!
//! Authenticated users land on `/welcome` and see featured apps plus recent /
//! most-used / popular shortcuts from Spectra page views. Operators with
//! `WelcomeAdmin` manage featured apps at `/welcome/admin`.
//!
//! Host session middleware and shell chrome live outside this crate. Page-view
//! emission uses `uf_product::telemetry::PageViewTracker`.
//!
//! ## Features
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `lazy-routes` (default) | WASM-split welcome + admin route views; use [`prefetch_family`] |
//! | `admin-permissions` | Gauge-backed `WelcomeAdmin` gate on `/welcome/admin` |
//! | `ssr` / `hydrate` | Server and client Leptos graphs |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Mount welcome + admin | [`UfWelcomeRoutes`] |
//! | Welcome page | [`WelcomePage`] |
//! | Featured admin | [`WelcomeAdminPage`], `welcome::featured` (SSR) |
//! | Permission manifest | [`permissions::WelcomePermission`] |
//! | Help spotlight inventory | [`ensure_help_linked`] |
//!
//! ## Getting started
//!
//! Mount [`UfWelcomeRoutes`] inside the host `<Routes>`, provide `Arc<Spectra>`, and
//! mount `uf_product::PageViewTracker` so usage cards receive page-view events.
//!
//! ```rust,ignore
//! use leptos_router::components::Routes;
//! use uf_welcome::UfWelcomeRoutes;
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <UfWelcomeRoutes />
//!     </Routes>
//! }
//! ```
//!
//! Enable `admin-permissions` (and Gauge) when `/welcome/admin` should enforce
//! `WelcomeAdmin`. Without that feature, the admin page still mounts; authorization
//! follows the host's Gauge wiring and e2e session override (`E2E_WELCOME_ADMIN_SESSION_KEY`
//! under SSR). Featured service errors use `FeaturedError` (see `welcome/featured`).
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | Mount [`UfWelcomeRoutes`] |
//! | Mid | `welcome/featured` + `admin-permissions` | Featured admin gate + `FeaturedError` |
//! | Detailed | `examples/shell-chrome-host`, `tests/featured_service_integ.rs` | Shell mount + admin integ |
//!
//! ```bash
//! cargo check -p shell-chrome-host --features ssr
//! cargo test -p uf-welcome --features ssr --test featured_service_integ
//! ```
//!
//! ## Where to look next
//!
//! - [`UfWelcomeRoutes`] — nested routes for welcome + admin.
//! - [`welcome`] — pages, cards, featured service, server fns.
//! - `uf_product::telemetry::usage` — Spectra aggregation helpers.
//! - `examples/shell-chrome-host` — mounts this crate beside shell chrome.

#![allow(missing_docs)]
#![allow(clippy::unused_unit, unused_imports)]
#![deny(clippy::missing_errors_doc)]

use leptos::prelude::*;
#[cfg(feature = "lazy-routes")]
use leptos_router::Lazy;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path,
};
use uf_product_macros::uf_app;

mod help_steps;
#[cfg(feature = "lazy-routes")]
mod lazy_routes;
/// Welcome permission manifest (`WelcomeAdmin`).
pub mod permissions;
#[allow(clippy::too_long_first_doc_paragraph)]
pub mod welcome;

#[cfg(feature = "ssr")]
pub mod embedded_surreal;
#[cfg(feature = "ssr")]
pub mod generated;
#[cfg(feature = "ssr")]
mod schemas;

#[cfg(feature = "lazy-routes")]
pub use lazy_routes::{prefetch_family, WelcomeAdminPageRoute, WelcomePageRoute};
#[cfg(feature = "ssr")]
pub use welcome::featured;
#[cfg(feature = "ssr")]
pub use welcome::server::E2E_WELCOME_ADMIN_SESSION_KEY;
pub use welcome::{WelcomeAdminPage, WelcomeLayout, WelcomePage};

/// Force-link Help spotlight inventory from this crate.
pub fn ensure_help_linked() {
    help_steps::ensure_help_steps_linked();
}

uf_app! {
    name: "Welcome",
    id: "welcome",
    description: "Signed-in welcome landing for Unified Field product hosts",
    icon: "👋",
    version: "0.1.0",
    routes: UfWelcomeRoutes,
    route_path: "/welcome",
    repository: "https://github.com/unified-field-dev/unified-field-product",
    permission_manifest: permissions::WelcomePermission,
}

/// Welcome application routes (`/welcome` and `/welcome/admin`).
#[component(transparent)]
pub fn UfWelcomeRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    #[cfg(feature = "lazy-routes")]
    {
        view! {
            <ParentRoute path=path!("") view=Outlet>
                <ParentRoute path=path!("welcome") view=WelcomeLayout>
                    <Route path=path!("") view={Lazy::<WelcomePageRoute>::new()} />
                    <Route path=path!("admin") view={Lazy::<WelcomeAdminPageRoute>::new()} />
                </ParentRoute>
            </ParentRoute>
        }
        .into_inner()
    }
    #[cfg(not(feature = "lazy-routes"))]
    {
        view! {
            <ParentRoute path=path!("") view=Outlet>
                <ParentRoute path=path!("welcome") view=WelcomeLayout>
                    <Route path=path!("") view=WelcomePage />
                    <Route path=path!("admin") view=WelcomeAdminPage />
                </ParentRoute>
            </ParentRoute>
        }
        .into_inner()
    }
}
