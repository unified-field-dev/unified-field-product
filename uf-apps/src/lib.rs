#![recursion_limit = "256"]
//! Apps directory for Unified Field product hosts.
//!
//! Lists every product app registered with `uf_app!` on a searchable `/apps`
//! index, and opens a detail page with name, description, primary route, and
//! optional repository / docs.rs links.
//!
//! Registration metadata comes from [`uf_product::AppRegistration`] /
//! [`uf_product::AppRegistry`]. Shell chrome that hosts the app-bar Apps control
//! lives in `uf-integrations`.
//!
//! ## Features
//!
//! | Feature | Start here |
//! |---------|------------|
//! | Searchable app index | [`AppsIndexPage`], [`server::get_apps_page`] |
//! | App detail overview | [`AppDetailPage`], [`server::get_app_overview`] |
//! | App-bar Apps launcher | [`ensure_app_bar_linked`], [`AppBarAppsButton`], [`apps_launcher::AppsLauncher`] |
//! | Register this directory as an app | `uf_app!` below (`id: "apps"`, `/apps`) |
//!
//! ### Routes vs app-bar launcher
//!
//! Mount [`UfAppsRoutes`] (or lazy equivalents) in the host router and `/apps`
//! works immediately. [`ensure_app_bar_linked`] is separate: it registers the
//! app-bar Apps button and [`AppsLauncher`] dialog. Skip it when you only need
//! the directory pages or you drive the launcher yourself.
//!
//! ## Searchable app index
//!
//! [`AppsIndexPage`] binds a search box to [`server::get_apps_page`]. Each call
//! Returns an [`orbital_paging::Page`] of [`server::AppDirectoryItem`] rows
//! (name, slug, description, `route_path`, optional `repository` / `crate_name`).
//! The server sorts by name, applies a case-insensitive name/description filter
//! when `query` is non-blank, then slices with the orbital over-fetch pattern.
//!
//! ```rust,ignore
//! use uf_apps::server::{filter_apps_by_query, get_apps_page, AppDirectoryItem};
//!
//! // Same filter the index page uses before pagination:
//! let mut apps: Vec<AppDirectoryItem> = /* from get_apps() or tests */;
//! filter_apps_by_query(&mut apps, Some("beacon"));
//!
//! // Infinite scroll fetch (what AppsIndexPage passes to OrbitalInfiniteScroll):
//! let page = get_apps_page(0, 12, Some("beacon".into())).await?;
//! ```
//!
//! **Failure modes:** [`server::get_apps_page`] returns [`leptos::prelude::ServerFnError`]
//! only when the server-fn transport or SSR wiring fails. An empty registry,
//! blank search, or no matches still return `Ok` with zero items. The UI shows
//! [`uf_product::components::EmptyState`] when the filtered list is empty.
//!
//! ## App detail overview
//!
//! [`AppDetailPage`] reads `:app_name` from the router and calls
//! [`server::get_app_overview`]. Success yields [`server::AppOverview`]; a missing
//! slug returns `Ok(None)` and the page shows a warning banner. Transport or SSR
//! errors surface as `Err(ServerFnError)` with an error
//! [`uf_product::primitives::MessageBar`].
//!
//! ## Getting started
//!
//! ### 1. Mount the directory
//!
//! ```rust,ignore
//! use leptos_router::components::Routes;
//! use uf_apps::{ensure_app_bar_linked, UfAppsRoutes};
//!
//! // Optional: app-bar Apps button + launcher dialog (routes work without this).
//! ensure_app_bar_linked();
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <UfAppsRoutes />
//!     </Routes>
//! }
//! ```
//!
//! Runnable host: `cargo check -p shell-chrome-host --features ssr`.
//!
//! ### 2. Define an app so it appears in the directory
//!
//! Product apps declare metadata with `uf_app!`. The inventory entry is what
//! [`AppsIndexPage`] and the launcher list:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::{ParentRoute, Route};
//! use leptos_router::path;
//! use uf_product_macros::uf_app;
//!
//! #[component(transparent)]
//! fn SampleBeaconRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
//!     view! {
//!         <ParentRoute path=path!("sample-beacon") view=|| view! { <p>"Sample Beacon"</p> }>
//!             <Route path=path!("") view=|| view! { <p>"ok"</p> } />
//!         </ParentRoute>
//!     }
//!     .into_inner()
//! }
//!
//! uf_app! {
//!     name: "Sample Beacon",
//!     id: "sample-beacon",
//!     description: "Teaching app registered with uf_app!",
//!     icon: "Cube",
//!     version: "0.1.0",
//!     routes: SampleBeaconRoutes,
//!     route_path: "/sample-beacon",
//! }
//! ```
//!
//! Inventory-only smoke (no Leptos mount):
//! `cargo run -p uf-product --example uf_app_registration --features ssr`.
//!
//! ### 3. Open the launcher from custom chrome
//!
//! Prefer [`ensure_app_bar_linked`] for the stock app bar. To drive the dialog
//! yourself:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use uf_apps::apps_launcher::AppsLauncher;
//!
//! let open = RwSignal::new(false);
//! view! {
//!     <button on:click=move |_| open.set(true)>"Apps"</button>
//!     <AppsLauncher open=open />
//! }
//! ```
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started §1–2 | Mount [`UfAppsRoutes`], optional [`ensure_app_bar_linked`], full `uf_app!` |
//! | Mid | Searchable app index above | [`server::filter_apps_by_query`] + [`server::get_apps_page`] |
//! | Detailed | `shell-chrome-host`, `uf_app_registration` | Full host + inventory smoke |
//!
//! ## Where to look next
//!
//! - [`UfAppsRoutes`] — nested `/apps` + `/apps/:app_name` routes.
//! - [`AppsIndexPage`] / [`AppDetailPage`] — index grid and overview card.
//! - [`mod@apps_launcher`] — Dialog typeahead ([`AppsLauncher`], [`safe_app_route_path`]).
//! - [`mod@server`] — [`server::get_apps_page`], [`server::get_app_overview`], [`server::AppDirectoryItem`].
//! - [`mod@help_steps`] — seeded Help spotlight steps; call [`ensure_help_linked`] so
//!   inventory links into the host binary (see [`uf_help`] authoring ladder).
//! - [`uf_product::AppRegistration`] / [`uf_product::AppRegistry`] — registration contracts.
//! - `uf_product_macros::uf_app` — registration macro (see that crate; smoke:
//!   `cargo run -p uf-product --example uf_app_registration --features ssr`).
//! - `examples/shell-chrome-host` — mounts this crate beside shell chrome
//!   (`cargo check -p shell-chrome-host --features ssr`).

// Page/layout components and `orbital_routes_extract` expansions are UI surface noise;
// crate-root Features / Concern → API tables are the integrator path.
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
