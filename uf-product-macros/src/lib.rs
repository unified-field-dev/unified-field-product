//! Proc macros for Unified Field product apps (registration, server context, search sources).
//!
//! Design-system macros (`#[component_doc]`, route extraction) live in `orbital-macros`.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | `uf_app!`, `#[server]`, permission-manifest derive, `define_search_sources!` | Design-system macros (`orbital-macros`) |
//! | Compile-time registration shape scanned by codegen | Runtime registry types (`uf-product::routes`, `uf-search-core`) |
//! | Optional `permission = …` gate attribute on `#[server]` | Gauge-backed permission evaluation at runtime |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Register a product app for shell + codegen | [`uf_app!`] |
//! | Server fn with operation context / permission gate | [`macro@server`] |
//! | Derive a permission manifest | [`derive_uf_permission_manifest`] |
//! | Declare search sources for `/search` | [`define_search_sources!`] |
//!
//! ## Features
//!
//! - [`uf_app!`] — register a product app (id, name, icon, routes, optional
//!   `repository` / `crate_name`) so the shell can discover it, and so
//!   `uf-codegen`'s build-script scan can find its route component.
//! - `#[server]` ([`macro@server`]) — wraps Leptos's `#[server]` with operation-context plumbing
//!   and an optional permission gate.
//! - `#[derive(UfPermissionManifest)]` ([`derive_uf_permission_manifest`]) — derive
//!   a permission manifest for a crate-local enum/struct.
//! - [`define_search_sources!`] — register one or more backend search sources for the
//!   `/search` command palette.
//!
//! ## Getting started
//!
//! In a product-app crate's `lib.rs`:
//!
//! ```rust,ignore
//! use uf_product_macros::uf_app;
//!
//! uf_app! {
//!     name: "Counter",
//!     id: "counter",
//!     description: "A simple counter application",
//!     icon: "📊",
//!     version: "0.1.0",
//!     routes: CounterRoutes,
//!     route_path: "/counter",
//!     repository: "https://github.com/example/counter",
//!     crate_name: "counter-app",
//! }
//! ```
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | `uf_app!` registration fields |
//! | Mid / detailed | `uf-product` example `uf_app_registration` | Inventory path list includes `/sample-beacon` |
//!
//! ```bash
//! cargo run -p uf-product --example uf_app_registration --features ssr
//! ```
//!
//! ## Where to look next
//!
//! - [`uf_app`] — expands to app metadata + `inventory::submit!` registration.
//! - [`macro@server`] — SSR-side operation context wrapper around `#[leptos::server]`.
//! - [`define_search_sources!`] — search source ids + SSR descriptors (`uf-search-core`).
//! - `uf-codegen` — build-time scan of `uf_app!` into route import tables.
//! - `uf-product` — runtime `AppRegistration` / guards / permission contracts.

use proc_macro::TokenStream;

mod app_definition;
mod permission_manifest_derive;
mod search_sources;
mod server;

/// Register a product app for Unified Field shell discovery.
///
/// Expands to metadata helpers and (on SSR) an `inventory::submit!` of
/// `uf_product::routes::AppRegistration`. `uf-codegen` also scans this invocation
/// at build time to emit route imports.
///
/// | Field | Required | Notes |
/// |-------|----------|-------|
/// | `id` | yes | String literal; stable app id |
/// | `name` | yes | Display name |
/// | `description` | yes | Short blurb for discovery UI |
/// | `icon` | yes | Icon key or emoji understood by the shell |
/// | `version` | yes | Semver string for display |
/// | `routes` | yes* | Route component type; required for codegen imports |
/// | `route_path` | yes | Primary prefix, e.g. `"/counter"` |
/// | `permission_manifest` | no | Path to an `AppPermissionManifestProvider` |
/// | `brand_seed` | no | Optional `#RRGGBB` override |
/// | `permissions`, `navigation` | no | Accepted for forward compatibility; not expanded yet |
///
/// \* Codegen skips packages that omit `routes`.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_product_macros::uf_app;
///
/// uf_app! {
///     name: "Counter",
///     id: "counter",
///     description: "A simple counter application",
///     icon: "📊",
///     version: "0.1.0",
///     routes: CounterRoutes,
///     route_path: "/counter",
/// }
/// ```
#[proc_macro]
pub fn uf_app(input: TokenStream) -> TokenStream {
    app_definition::expand_uf_app(input.into()).into()
}

/// Wrapper around Leptos `#[server]` with operation context and optional permission gate.
///
/// Without arguments, wraps the async body in `uf_product::ssr::with_operation(fn_name, …)`.
/// With `permission = "…"`, uses `higgs::server_runtime::with_operation` and a Gauge
/// `actor_can` check before the body runs.
///
/// # Examples
///
/// ```rust,ignore
/// #[uf_product_macros::server]
/// pub async fn counter_get() -> Result<i32, ServerFnError> {
///     let v = uf_product::ssr::valence().await?;
///     // …
///     Ok(0)
/// }
///
/// #[uf_product_macros::server(permission = "counter.admin.view")]
/// pub async fn counter_admin_get() -> Result<i32, ServerFnError> {
///     Ok(0)
/// }
/// ```
#[proc_macro_attribute]
pub fn server(attr: TokenStream, input: TokenStream) -> TokenStream {
    server::expand_server(attr.into(), input.into()).into()
}

/// Derive helper for crate-local Unified Field permission manifests.
///
/// Place `#[permission_manifest(domain_key = "…", domain_name = "…", domain_description = "…")]`
/// on the type and `#[permission(name = "…", description = "…")]` on each variant.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_product_macros::UfPermissionManifest;
///
/// #[derive(UfPermissionManifest)]
/// #[permission_manifest(
///     domain_key = "counter_admin",
///     domain_name = "Counter Admin",
///     domain_description = "Administrative actions for the counter app"
/// )]
/// enum CounterPermission {
///     #[permission(name = "counter.admin.view", description = "View counter admin")]
///     ViewAdmin,
///     #[permission(name = "counter.admin.set", description = "Set the counter")]
///     SetCounter,
/// }
/// ```
#[proc_macro_derive(UfPermissionManifest, attributes(permission_manifest, permission))]
pub fn derive_uf_permission_manifest(input: TokenStream) -> TokenStream {
    permission_manifest_derive::expand_derive_permission_manifest(input.into()).into()
}

/// Define search sources and SSR descriptor registrations.
///
/// Each variant needs `id`, `label`, `description`, and a `provider` path implementing
/// `uf_search_core::SearchSourceProvider`. On SSR, expands to `inventory::submit!` of
/// `SearchSourceDescriptor` entries.
///
/// # Examples
///
/// ```rust,ignore
/// use uf_product_macros::define_search_sources;
///
/// define_search_sources! {
///     enum AppSearchSourceId {
///         Apps => {
///             id: "apps",
///             label: "Apps",
///             description: "Search registered apps",
///             provider: AppsSearchSource
///         }
///     }
/// }
/// ```
///
/// Wire the resulting keys into `uf_integrations::SearchSourcePicker` and implement
/// the provider against `uf_search_core::SearchSourceProvider`.
#[proc_macro]
pub fn define_search_sources(input: TokenStream) -> TokenStream {
    search_sources::expand_define_search_sources(input.into()).into()
}
