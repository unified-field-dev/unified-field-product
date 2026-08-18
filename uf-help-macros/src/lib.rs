//! Proc macros for Help spotlight tour step registration.
//!
//! Expands [`help_spotlight_step`] into `inventory` entries consumed by the
//! `uf-help` crate. The full authoring ladder (DOM anchor, force-link, shell mount,
//! replay) lives on the `uf-help` crate root.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Register a Leptos help body as a tour step | [`help_spotlight_step`] |
//!
//! This crate has no Cargo features. Hosts opt in through `uf-help` / `uf-integrations`
//! (`offering-help`).

#![allow(missing_docs)]

use proc_macro::TokenStream;

mod help_spotlight_step;

/// Register a Leptos component as a Help spotlight tour step.
///
/// Keeps the original `#[component]` fn and submits a
/// `uf_help::HelpStepDescriptor` via `inventory`.
///
/// | Attribute | Required | Notes |
/// |-----------|----------|-------|
/// | `route` | yes | Exact route path, e.g. `"/apps"`, or `"/apps/:app_name"` for one-segment app overviews |
/// | `feature_highlight` | yes | Stable per-step identity (seen / replay key); do not rename in place |
/// | `title` | no | Panel header; defaults to `feature_highlight` |
/// | `spotlight` | no | DOM element `id` for the cutout; omit to center in the viewport |
/// | `position` | no | Popover placement: `top` (default), `bottom`, `left`, `right`, or `*-start` / `*-end` |
/// | `order` | no | Sort key within the route (default `0`) |
///
/// # Examples
///
/// Pair the macro with a matching HTML `id` on the spotlight target and a
/// `data-testid` on the step body for Playwright (see `uf-apps/src/help_steps.rs`).
///
/// ```rust,ignore
/// use leptos::prelude::*;
/// use uf_help_macros::help_spotlight_step;
///
/// #[help_spotlight_step(
///     route = "/apps",
///     feature_highlight = "apps-search",
///     title = "Search apps",
///     spotlight = "apps-search-input",
///     position = "bottom",
///     order = 10,
/// )]
/// #[component]
/// pub fn AppsSearchHelp() -> impl IntoView {
///     view! {
///         <input id="apps-search-input" />
///         <p data-testid="help-step-apps-search">
///             "Use search to find installed apps by name."
///         </p>
///     }
/// }
///
/// /// Call from the app crate so inventory is linked into the host binary.
/// pub fn ensure_help_steps_linked() {}
/// ```
#[proc_macro_attribute]
pub fn help_spotlight_step(attr: TokenStream, input: TokenStream) -> TokenStream {
    help_spotlight_step::expand(attr.into(), input.into()).into()
}
