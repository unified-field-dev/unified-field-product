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
//! | Provide session context once (host root) | [`provide_auth_context`], then [`init_auth_resource`] | Getting started; [`provide_auth_context`] / [`init_auth_resource`] `# Examples` |
//! | Show the signed-in name / email | [`use_authenticated_user`] | Getting started; [`use_authenticated_user`] `# Examples` |
//! | Branch anonymous vs signed-in | [`use_auth_state`] | Getting started; [`use_auth_state`] `# Examples` |
//! | Load session from host middleware | [`get_session`] | [`get_session`] `# Errors` when SSR auth extract fails |
//! | Auth dialog (sign-in modal) | [`use_auth_dialog_controller`], [`AuthDialogController`] | Shell layout + `provide_shell_auth_menu` |
//! | Gate a page behind login | [`routes::RequireAuthenticated`] | Getting started; [`routes`] module example; named permissions fail closed |
//! | Auth route + return path (no shell dialog) | [`routes::auth_signin_href`], [`routes::auth_signup_href`] | Used when no [`AuthDialogController`] |
//! | Help skip while a gate is showing | [`provide_access_gate_state`], [`AccessGateActive`] | Pair with [`routes::RequireAuthenticated`] |
//! | App registration + route discovery (SSR) | [`routes::AppRegistration`], [`routes::AppRegistry`] | `uf_app_registration` example; [`routes`] inventory vs mounted routes |
//! | Permission manifest shapes | [`permissions`], [`AppPermissionManifest`] | Runnable manifest sample in [`permissions`] |
//! | Design-system primitives / components | [`components`], [`primitives`], [`models`], [`nav`] | Re-exported from `orbital-zone-a` for one dependency path |
//! | Picker / in-page search sources | [`search_sources`] | `uf-search-core` registry contracts |
//! | Content index / AppBar workspace search | [`workspace_search`] | [`workspace_search`] writer + [`workspace_search::WorkspaceSearchError`] |
//! | Light/dark/brand appearance | [`theme`], [`services`] | [`services`] `save_my_appearance` `# Errors` |
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
//! Call these in order:
//!
//! 1. [`provide_auth_context`] — put [`AuthContext`] into Leptos context (once, near `Router`).
//! 2. [`init_auth_resource`] — fetch [`get_session`] and write it into that context.
//! 3. In pages: [`use_authenticated_user`] for profile fields, or [`use_auth_state`]
//!    when you need the [`AuthSession::Anonymous`] branch.
//!
//! `rust,ignore` because this needs a Leptos runtime. Copy it into a `ssr`/`hydrate`
//! host. Compile-checked neighbors: `auth_shell_host` (Axum inventory gate),
//! `examples/shell-chrome-host` (full shell).
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use uf_product::{
//!     init_auth_resource, provide_auth_context, use_auth_state, use_authenticated_user,
//!     routes::RequireAuthenticated, AuthSession,
//! };
//!
//! #[component]
//! fn AppRoot() -> impl IntoView {
//!     let auth = provide_auth_context(Default::default());
//!     let _session = init_auth_resource(&auth);
//!     view! {
//!         <SessionChip />
//!         <ProtectedPage />
//!     }
//! }
//!
//! #[component]
//! fn SessionChip() -> impl IntoView {
//!     let session = use_auth_state();
//!     view! {
//!         <span>{move || match session.get() {
//!             AuthSession::Anonymous(_) => "Guest",
//!             AuthSession::Authenticated(_) => "Signed in",
//!         }}</span>
//!     }
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
//! After sign-in or sign-out, call [`AuthContext::trigger_refresh`] so
//! [`init_auth_resource`] re-runs [`get_session`].
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | provide → hydrate → `use_authenticated_user` / `use_auth_state` |
//! | Mid | [`provide_auth_context`], [`use_authenticated_user`], [`use_auth_state`] | Same flow on the items |
//! | Detailed | workspace `uf-product/examples/` | `uf_app_registration` (`uf_app!`), `app_route_paths`, `auth_shell_host` (Axum inventory gate) |
//! | Nested UI | workspace `examples/` | `shell-chrome-host`, `component-preview-host` |
//!
//! ```bash
//! cargo run -p uf-product --example uf_app_registration --features ssr
//! cargo run -p uf-product --example auth_shell_host --features ssr
//! ```
//!
//! ## Where to look next
//!
//! - [`provide_auth_context`] / [`use_authenticated_user`] / [`use_auth_state`] — session.
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
/// Shared session signals. Provide once with [`provide_auth_context`]; pages read
/// [`use_authenticated_user`] or [`use_auth_state`] instead of constructing this.
///
/// After sign-in or sign-out, call [`AuthContext::trigger_refresh`].
pub use context::AuthContext;

/// Insert [`AuthContext`] into Leptos context. Call once near the host `Router`.
///
/// Pair with [`init_auth_resource`] so [`get_session`] fills the session signal.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_product::{init_auth_resource, provide_auth_context};
///
/// let auth = provide_auth_context(Default::default());
/// let _session = init_auth_resource(&auth);
/// ```
pub use context::provide_auth_context;

/// Handle for [`AuthContext::trigger_refresh`] and `session_loaded`.
///
/// # Panics
///
/// Panics if [`provide_auth_context`] has not run in an ancestor.
pub use context::use_auth_context;

/// Full [`AuthSession`] memo: match [`AuthSession::Anonymous`] vs
/// [`AuthSession::Authenticated`].
///
/// Prefer [`use_authenticated_user`] when you only need profile fields.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_product::{use_auth_state, AuthSession};
///
/// let session = use_auth_state();
/// let label = move || match session.get() {
///     AuthSession::Anonymous(_) => "Guest".to_string(),
///     AuthSession::Authenticated(user) => user
///         .display_name
///         .clone()
///         .unwrap_or_else(|| user.user_id.clone()),
/// };
/// ```
pub use context::use_auth_state;

/// Signed-in profile memo (`None` while anonymous or still loading).
///
/// Use this for display name, email, and roles. Pair with
/// [`routes::RequireAuthenticated`] so the `None` branch is not the page body.
///
/// # Examples
///
/// ```rust,ignore
/// use leptos::prelude::*;
/// use uf_product::use_authenticated_user;
///
/// let user = use_authenticated_user();
/// view! {
///     <p>{move || {
///         user.get()
///             .and_then(|u| u.display_name.clone())
///             .unwrap_or_else(|| "signed out".to_string())
///     }}</p>
/// }
/// ```
pub use context::use_authenticated_user;
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
