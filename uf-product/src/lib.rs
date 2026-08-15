#![recursion_limit = "256"]
//! # uf-product (Unified Field product package)
//!
//! Re-exports Zone A (`orbital-zone-a`) and adds product wiring without lepton UI crates.
//! Session loading ([`get_session`] / [`init_auth_resource`]) lives here. Hosts compose the
//! app-bar auth menu via `uf-integrations` `ShellAuthMenu` / `provide_shell_auth_menu`
//! (typically `lepton_shell::AppBarUserMenu` from `lepton-uf-app`).
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | Reactive session / signed-in user | [`AuthContext`], [`use_auth_state`], [`use_authenticated_user`] |
//! | Load session from host middleware | [`get_session`], [`init_auth_resource`] |
//! | Auth dialog (sign-in modal) | [`use_auth_dialog_controller`], [`AuthDialogController`] |
//! | Gate a page behind login | [`routes::RequireAuthenticated`] |
//! | App registration + route discovery (SSR) | [`routes::AppRegistration`], [`routes::AppRegistry`] |
//! | Permission manifest shapes | [`permissions`], [`AppPermissionManifest`] |
//! | Picker / in-page search sources | [`search_sources`] (re-exports `uf-search-core`) |
//! | Content index / AppBar workspace search | [`workspace_search`] |
//! | Light/dark/brand appearance | [`theme`], [`services`] |
//! | Page-view / appearance analytics | [`telemetry`] |
//! | Zone A design system re-exports | [`components`], [`primitives`], [`models`], [`nav`] |
//! | Shell chrome (sibling crate) | `uf-integrations` |
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | Session bridge, auth dialog control, route guards, permission manifest shapes | Host axum-login middleware / credential stores (lepton-auth) |
//! | App registration metadata (`AppRegistration` / `AppRegistry`) | Build-time `uf_app!` scanning (`uf-codegen`) and proc-macro expansion (`uf-product-macros`) |
//! | Search contract re-exports from `uf-search-core` (pickers) | Search combobox UI (`uf-integrations::SearchSourcePicker`) |
//! | Per-user content index writer/query (`workspace_search`) | AppBar search UI (`uf-integrations::WorkspaceSearch`) |
//! | Appearance preferences + Zone A design-system re-exports | Shell app bar / layout chrome (`uf-integrations`) |
//!
//! ## Features
//!
//! - **Zone A design system** — [`components`], [`primitives`], [`context`], [`models`],
//!   [`nav`], and (behind `preview`/`ssr`/`hydrate`) [`preview`] are re-exported from
//!   `orbital-zone-a` so downstream apps depend only on `uf-product`.
//! - **Session bridge** — [`get_session`] / [`init_auth_resource`] map host axum-login
//!   sessions into [`AuthSession`] (behind `ssr` / `hydrate`).
//! - **Permissions** — [`permissions`] defines the manifest shapes ([`AppPermissionManifest`]
//!   and friends) apps use to declare their permission surface.
//! - **App registration + routing** — [`routes`] provides `AppRegistration`/`AppRegistry`
//!   (SSR-only, inventory-backed) plus [`routes::RequireAuthenticated`] for gating pages.
//! - **Search sources (pickers)** — [`search_sources`] re-exports `uf-search-core` for
//!   in-page pickers (not the AppBar content index).
//! - **Workspace content index** — [`workspace_search`] maintains per-user
//!   `UnifiedFieldSearchDocument` rows and owner-scoped query for AppBar search.
//! - **Appearance + theming** — [`theme`] and [`services`] provide light/dark/brand
//!   preferences, persisted client-side and (behind `ssr`/`hydrate`) server-backed.
//! - **Telemetry** — [`telemetry`] tracks page views and appearance changes for analytics.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Reactive session state | [`AuthContext`], [`use_auth_state`], [`use_authenticated_user`] |
//! | Load session from host middleware | [`get_session`], [`init_auth_resource`] |
//! | Auth dialog control (sign-in modal, etc.) | [`use_auth_dialog_controller`], [`AuthDialogController`], [`AuthDialogIntent`] |
//! | Gating a page behind login | [`routes::RequireAuthenticated`] |
//! | Help skip while a gate is showing | [`provide_access_gate_state`], [`AccessGateActive`] |
//! | Auth route + return path (no shell dialog) | [`routes::auth_signin_href`], [`routes::auth_signup_href`] |
//! | Declaring an app's permission surface | [`AppPermissionManifest`], [`PermissionSpec`], [`PermissionDomainSpec`], [`PermissionEnum`] |
//! | App registration + route discovery (SSR) | [`routes::AppRegistration`], [`routes::AppRegistry`] |
//! | Design system primitives/components | [`components`], [`primitives`], [`models`], [`nav`] (re-exported from `orbital-zone-a`) |
//! | Picker combobox contracts | [`search_sources`] (re-exports `uf-search-core`) |
//! | AppBar content index upsert/query | [`workspace_search`] |
//! | Light/dark/brand appearance preferences | [`theme`], [`services::{get_my_appearance, save_my_appearance, use_appearance_preferences}`](services) |
//! | Page-view / appearance-change analytics | [`telemetry`] |
//!
//! ## Getting started
//!
//! Most apps only need the auth/session helpers and a couple of route guards:
//!
//! ```rust,ignore
//! use uf_product::{use_auth_state, routes::RequireAuthenticated};
//! use leptos::prelude::*;
//!
//! #[component]
//! fn ProtectedPage() -> impl IntoView {
//!     view! {
//!         <RequireAuthenticated>
//!             <p>{move || format!("Signed in as {}", use_auth_state().get().is_authenticated())}</p>
//!         </RequireAuthenticated>
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | `RequireAuthenticated` + `use_auth_state` |
//! | Mid / detailed | workspace `uf-product/examples/` | `uf_app_registration` (`uf_app!`), `app_route_paths`, `auth_shell_host` (Axum gate) |
//! | Nested UI | workspace `examples/` | `shell-chrome-host`, `component-preview-host` |
//!
//! ```bash
//! cargo run -p uf-product --example uf_app_registration --features ssr
//! ```
//!
//! ## Where to look next
//!
//! - [`AuthContext`] / [`use_auth_state`] — reactive session state.
//! - [`permissions`] — permission manifest contracts.
//! - [`routes`] — app registration + route guards.
//! - `uf-integrations` — shell app bar, `WorkspaceSearch`, and `SearchSourcePicker`.
//! - [`workspace_search`] — per-user content index (SideEffect/Iter writers + query).
//! - `uf-search-core` — picker DTOs/registry (re-exported from [`search_sources`]).
//! - `uf-product-macros` / `uf-codegen` — `uf_app!` registration and build-time route discovery.

// Narrow allow: Zone A / Spectra / inventory re-exports and generated SSR modules
// expand many public items without local rustdoc. Prefer documenting new product-owned
// APIs at the item; do not widen this allow for ordinary new modules.
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
