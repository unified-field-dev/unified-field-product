#![recursion_limit = "256"]
//! # uf-product — Unified Field product shell APIs
//!
//! Leptos product hosts pull session state, route guards, app registration,
//! permission manifests, workspace search contracts, appearance preferences,
//! design-system primitives, and page-view telemetry from this crate.
//!
//! At the host root, call [`provide_auth_context`] and [`init_auth_resource`] so
//! [`get_session`] hydrates [`AuthContext`]. Compose the default app-bar auth menu
//! through `uf-integrations` (`ShellAuthMenu` / `provide_shell_auth_menu`; usually
//! `lepton_shell::AppBarUserMenu`). Wrap gated pages in
//! [`routes::RequireAuthenticated`]. Register each product app with `uf_app!` so
//! [`routes::AppRegistry`] and `uf-codegen` can discover routes and metadata.
//!
//! ## Organized by task
//!
//! | Task | Start here | Example / errors |
//! |------|------------|------------------|
//! | Reactive session / signed-in user | [`AuthContext`], [`use_auth_state`], [`use_authenticated_user`] | Getting started below; profile via [`use_authenticated_user`] |
//! | Load session from host middleware | [`session`], [`get_session`], [`init_auth_resource`] | [`get_session`] # Errors when SSR auth extract fails |
//! | Auth dialog (sign-in modal) | [`use_auth_dialog_controller`], [`AuthDialogController`] | Shell layout + `provide_shell_auth_menu` |
//! | Gate a page behind login | [`routes::RequireAuthenticated`] | [`routes`] module example; named permissions fail closed |
//! | Auth route + return path (no shell dialog) | [`routes::auth_signin_href`], [`routes::auth_signup_href`] | Used when no [`AuthDialogController`] |
//! | Help skip while a gate is showing | [`provide_access_gate_state`], [`AccessGateActive`] | Pair with [`routes::RequireAuthenticated`] |
//! | App registration + route discovery (SSR) | [`routes::AppRegistration`], [`routes::AppRegistry`] | `uf_app_registration` example; [`routes`] inventory vs mounted routes |
//! | Permission manifest shapes | [`permissions`], [`AppPermissionManifest`] | Runnable manifest sample in [`permissions`] |
//! | Design-system primitives / components | [`components`], [`primitives`], [`models`], [`nav`] | Re-exported from `orbital-zone-a` for one dependency path |
//! | Picker / in-page search sources | [`search_sources`] | `uf-search-core` registry contracts |
//! | Content index / AppBar workspace search | [`workspace_search`] | [`workspace_search`] writer + [`workspace_search::WorkspaceSearchError`] |
//! | Light/dark/brand appearance | [`theme`], [`services`] | [`services`] `save_my_appearance` # Errors |
//! | Page-view / appearance analytics | [`telemetry`] | [`PageViewTracker`] for `uf_app!` products |
//! | Shell chrome (sibling crate) | `uf-integrations` | App bar, `WorkspaceSearch`, picker UI |
//!
//! Host axum-login middleware and credential stores live in **lepton-auth**.
//! Build-time `uf_app!` scanning is **uf-codegen** / **uf-product-macros**.
//! Shell app bar, layout, and search UI live in **uf-integrations**.
//! Design-system modules ([`components`], [`primitives`], [`models`], [`nav`],
//! [`context`], and optionally [`preview`]) are re-exported from `orbital-zone-a`
//! so app crates can depend on `uf-product` alone for those imports.
//!
//! ## Getting started
//!
//! Provide auth once near the router root, then gate pages and read the signed-in
//! profile (not the `is_authenticated()` bool):
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use uf_product::{
//!     init_auth_resource, provide_auth_context, use_authenticated_user,
//!     routes::RequireAuthenticated,
//! };
//!
//! #[component]
//! fn AppRoot() -> impl IntoView {
//!     let auth = provide_auth_context(Default::default());
//!     let _session = init_auth_resource(&auth);
//!     view! { <ProtectedPage /> }
//! }
//!
//! #[component]
//! fn ProtectedPage() -> impl IntoView {
//!     let user = use_authenticated_user();
//!     view! {
//!         <RequireAuthenticated>
//!             <p>{move || {
//!                 user.get()
//!                     .and_then(|u| u.display_name.clone())
//!                     .unwrap_or_else(|| "you".to_string())
//!             }}</p>
//!         </RequireAuthenticated>
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | `init_auth_resource` + `RequireAuthenticated` + `use_authenticated_user` |
//! | Mid / detailed | workspace `uf-product/examples/` | `uf_app_registration` (`uf_app!`), `app_route_paths`, `auth_shell_host` (Axum inventory gate) |
//! | Nested UI | workspace `examples/` | `shell-chrome-host`, `component-preview-host` |
//!
//! ```bash
//! cargo run -p uf-product --example uf_app_registration --features ssr
//! cargo run -p uf-product --example auth_shell_host --features ssr
//! ```
//!
//! ## Where to look next
//!
//! - [`AuthContext`] / [`use_auth_state`] — reactive session state.
//! - [`permissions`] — permission manifest contracts.
//! - [`routes`] — app registration + route guards.
//! - `uf-integrations` — shell app bar, `WorkspaceSearch`, and `SearchSourcePicker`.
//! - [`workspace_search`] — per-user content index (SideEffect/Iter writers + query).
//! - `uf-search-core` — picker DTOs/registry (also via [`search_sources`]).
//! - `uf-product-macros` / `uf-codegen` — `uf_app!` registration and build-time route discovery.

// Narrow allow: design-system / Spectra / inventory re-exports and generated SSR
// modules expand many public items without local rustdoc. Prefer documenting new
// product-owned APIs at the item; do not widen this allow for ordinary new modules.
#![allow(missing_docs)]

pub use orbital_zone_a::context;
pub use orbital_zone_a::models;
pub use orbital_zone_a::primitives;

pub mod app_bar_menu_extras;
pub mod app_bar_utilities;
pub mod auth_dialog;
pub mod components;
pub mod nav;
pub mod paths;
pub mod permissions;
pub mod routes;
pub mod search_sources;
pub mod services;
/// Host session → [`AuthSession`] bridge (`get_session` / `init_auth_resource`).
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod session;
pub mod theme;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod workspace_search;

#[cfg(any(feature = "hydrate", feature = "ssr", feature = "preview"))]
pub use orbital_zone_a::preview;

pub use permissions::AppPermissionManifest;
pub use permissions::AppPermissionManifestProvider;
pub use permissions::PermissionDomainSpec;
pub use permissions::PermissionEnum;
pub use permissions::PermissionSpec;

#[cfg(feature = "ssr")]
pub use quark::inventory;
#[cfg(feature = "ssr")]
pub use routes::AppRegistration;
#[cfg(feature = "ssr")]
pub use routes::AppRegistry;

pub use app_bar_menu_extras::{
    app_bar_dark_mode_bind, provide_app_bar_menu_extras, use_app_bar_menu_extras,
    AppBarAppearanceMenuItems, AppBarCompactMenuExtras, AppBarMenuExtrasInjection,
};
pub use app_bar_utilities::{
    collect_app_bar_utilities, register_app_bar_utility, AppBarUtilityContribution,
};
pub use auth_dialog::{
    provide_auth_dialog_controller, use_auth_dialog_controller, AuthDialogController,
    AuthDialogIntent,
};
pub use context::{
    provide_auth_context, use_auth_context, use_auth_state, use_authenticated_user, AuthContext,
};
pub use models::auth::{AnonymousUser, AuthSession, AuthenticatedUser};
pub use routes::{provide_access_gate_state, use_access_gate_active, AccessGateActive};

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub use session::{get_session, init_auth_resource};

#[cfg(any(feature = "hydrate", feature = "ssr", feature = "preview"))]
pub use preview::{collect_preview_registrations, PreviewRegistration};

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub use crate::theme::{
    apply_appearance_preferences, read_local_appearance, write_local_appearance,
    UF_SHELL_BRAND_SEED,
};
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub use services::{
    get_my_appearance, init_appearance_resource, provide_appearance_context, save_my_appearance,
    use_appearance_preferences, AppearanceContext, AppearanceData, AppearancePreferences,
    APPEARANCE_STORAGE_KEY, PRODUCT_BRAND_PRESETS,
};
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub use telemetry::AppearanceThemeController;
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub use telemetry::PageViewTracker;

pub use orbital_theme::ThemeMode;
pub use orbital_zone_a::{
    hide_boot_loader, OrbitalBootLoaderHeadAssets, OrbitalBootOverlay, OrbitalFirstPaintHeadAssets,
};

pub mod tokens;
pub use orbital_zone_a::{orbital_shell, OrbitalTemplate};

#[cfg(feature = "ssr")]
pub mod embedded_surreal;
#[cfg(feature = "ssr")]
pub mod generated;
#[cfg(feature = "ssr")]
mod schemas;
#[cfg(feature = "ssr")]
pub mod spectra_schemas;
#[cfg(all(feature = "ssr", feature = "spectra-telemetry"))]
pub mod spectra_topics;
#[cfg(feature = "ssr")]
pub mod ssr;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod telemetry;
