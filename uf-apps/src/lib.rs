#![recursion_limit = "256"]
//! Apps directory for Unified Field product hosts.
//!
//! Lists every product app registered via `uf_app!`, and shows an
//! ownership/goals/tasks detail page for each.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | `/apps` index and detail UI | App registration macros / codegen |
//! | Server fns that list registered apps | Shell chrome (`uf-integrations`) |
//! | App-bar Apps search launcher | Workspace `SearchSourcePicker` |
//!
//! ## Features
//!
//! - **Index page** — [`AppsIndexPage`] renders a searchable grid of every registered app.
//! - **Detail page** — [`AppDetailPage`] shows an app's overview card.
//! - **App-bar launcher** — [`AppBarAppsButton`] opens [`apps_launcher::AppsLauncher`]
//!   (centered Dialog). Type to filter; select goes to `route_path`.
//! - **Registered app** — this crate registers itself as the `"apps"` app via
//!   `uf_app!`, mounted at `/apps`.
//!
//! ## Getting started
//!
//! Mount [`UfAppsRoutes`] inside your host's `<Routes>`, and call
//! [`ensure_app_bar_linked`] so the Apps utility appears in the default app bar:
//!
//! ```rust,ignore
//! use leptos_router::components::Routes;
//! use uf_apps::{ensure_app_bar_linked, UfAppsRoutes};
//!
//! ensure_app_bar_linked();
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <UfAppsRoutes />
//!     </Routes>
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`UfAppsRoutes`] — nested routes for the apps directory.
//! - [`mod@apps_launcher`] — app-bar search launcher (Dialog + typeahead).
//! - [`mod@server`] — server functions backing the index/detail pages.
//! - `uf-product` — `AppRegistration` / `AppRegistry` contracts.
//! - `uf-product-macros` — `uf_app!` used by this crate.
//! - `examples/shell-chrome-host` — mounts this crate beside shell chrome.

// Page/layout components and `orbital_routes_extract` expansions are UI surface noise;
// crate-root Features / Owns tables are the integrator path.
#![allow(missing_docs)]
#![allow(clippy::unused_unit, unused_imports)]

use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route};
use leptos_router::path;
#[cfg(feature = "lazy-routes")]
use leptos_router::Lazy;
use uf_product_macros::uf_app;

mod app_bar_button;
/// App-bar Apps search launcher (Dialog / OverlayDrawer + typeahead).
pub mod apps_launcher;
mod components;
mod help_steps;
mod layout;
#[cfg(feature = "lazy-routes")]
mod lazy_routes;
mod pages;
mod prefetch;
/// Server functions backing the apps directory index/detail pages.
pub mod server;

pub use app_bar_button::{
    ensure_linked as ensure_app_bar_linked, AppBarAppsButton, APP_BAR_UTILITY_ORDER,
};
pub use apps_launcher::{safe_app_route_path, AppsLauncher, AppsLauncherBody, AppsLauncherResult};
pub use layout::AppsLayout;
#[cfg(feature = "lazy-routes")]
pub use lazy_routes::{prefetch_family, AppDetailRoute, AppsIndexRoute};
pub use pages::{AppDetailPage, AppsIndexPage};
pub use prefetch::PrefetchAppFamily;

/// Force-link Help spotlight inventory from this crate.
pub fn ensure_help_linked() {
    help_steps::ensure_help_steps_linked();
}

// Define the Apps application metadata.
uf_app! {
    name: "Apps",
    id: "apps",
    description: "Apps directory and detail pages",
    icon: "📱",
    version: "0.1.0",
    routes: UfAppsRoutes,
    route_path: "/apps",
    repository: "https://github.com/unified-field-dev/unified-field-product",
    crate_name: "uf-apps",
}

/// Apps application routes: index and detail views.
#[allow(missing_docs)]
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn UfAppsRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    #[cfg(feature = "lazy-routes")]
    {
        view! {
            <ParentRoute path=path!("apps") view=AppsLayout>
                <Route path=path!("") view={Lazy::<AppsIndexRoute>::new()} />
                <Route path=path!(":app_name") view={Lazy::<AppDetailRoute>::new()} />
            </ParentRoute>
        }
        .into_inner()
    }
    #[cfg(not(feature = "lazy-routes"))]
    {
        view! {
            <ParentRoute path=path!("apps") view=AppsLayout>
                <Route path=path!("") view=AppsIndexPage />
                <Route path=path!(":app_name") view=AppDetailPage />
            </ParentRoute>
        }
        .into_inner()
    }
}
