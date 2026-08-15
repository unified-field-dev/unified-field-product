//! Proc macros for Help spotlight tour step registration.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | [`help_spotlight_step`] attribute expansion | Runtime inventory types (`uf-help`) |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Register a Leptos help body as a tour step | [`help_spotlight_step`] |
//!
//! Keep `feature_highlight` keys stable. A later release that adds a new key
//! shows that step to returning users without replaying older steps.

#![allow(missing_docs)]

use proc_macro::TokenStream;

mod help_spotlight_step;

/// Register a Leptos component as a Help spotlight tour step.
///
/// Keeps the original `#[component]` fn and submits a
/// `uf_help::HelpStepDescriptor` via inventory.
///
/// | Attribute | Required | Notes |
/// |-----------|----------|-------|
/// | `route` | yes | Exact route path, e.g. `"/apps"` |
/// | `feature_highlight` | yes | Stable per-step identity (seen / replay key) |
/// | `title` | no | Panel header; defaults to `feature_highlight` |
/// | `spotlight` | no | DOM element id for the cutout; omit to center in the viewport |
/// | `position` | no | Popover placement: `top` (default), `bottom`, `left`, `right`, or `*-start` / `*-end` |
/// | `order` | no | Sort key within the route (default `0`) |
///
/// # Examples
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
///     view! { <p>"Use search to find installed apps by name."</p> }
/// }
/// ```
#[proc_macro_attribute]
pub fn help_spotlight_step(attr: TokenStream, input: TokenStream) -> TokenStream {
    help_spotlight_step::expand(attr.into(), input.into()).into()
}
