//! Optional Help product offering for Unified Field shells.
//!
//! Ships the app-bar Help menu (bug / feature / security reports and replay),
//! a spotlight tour player backed by compile-time step inventory, visit progress
//! in Valence plus a signed-out `localStorage` mirror, and a GitHub feedback bot.
//! Help center CMS routes, Orbital spotlight primitives, platform anon identity
//! claims, and Neutrino secret wiring live in other crates.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Author a step (macro + DOM anchor) | [`help_spotlight_step`], `spotlight = "…"` matches element `id` |
//! | Link inventory into the host binary | App crate `ensure_help_steps_linked()`; [`ensure_linked`] for uf-help's own submissions |
//! | Mount the tour player | [`HelpTourPlayer`] via `uf-integrations` feature `offering-help` / `full` on `UnifiedFieldShellLayout` |
//! | Collect steps for a pathname | [`collect_help_steps_for_route`], [`HelpStepDescriptor`] |
//! | Match inventory `route` to pathname | [`route_matches`] (exact; plus `"/apps/:app_name"`) |
//! | Pending / seen semantics | [`compute_pending`], stable [`HelpStepDescriptor::feature_highlight`] keys |
//! | Signed-in visit rows | [`help_list_visits_for_route`], [`help_mark_steps_seen`], [`help_request_replay_for_route`] |
//! | Signed-out mirror + merge | [`LOCAL_STORAGE_KEY`], [`read_local_visits`], [`merge_local_into_server`] |
//! | Replay on current route only | [`request_replay_current_route`], [`notify_help_replay`], Help menu replay |
//! | Skip tour during auth gates | [`HelpTourPlayer`] + [`uf_product::AccessGateActive`] |
//! | App-bar Help control | [`AppBarHelpButton`] |
//! | Report dialogs + GitHub submit | [`HelpReportDialog`], [`submit_help_bug_report`], … |
//! | Typed failures | [`HelpError`], [`HelpError::into_server_fn_error`] |
//!
//! ## Authoring ladder
//!
//! ### 1. Highlight — register a step and anchor the cutout
//!
//! Add a Leptos body with [`help_spotlight_step`] (re-exported from this crate).
//! When `spotlight` is set, give the target UI the same string as its HTML `id`
//! (Orbital passes it to `SpotlightTourStep` as `anchor_id`). Omit `spotlight` to center the panel with no cutout.
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use uf_help::help_spotlight_step;
//!
//! #[help_spotlight_step(
//!     route = "/apps",
//!     feature_highlight = "apps-search",
//!     title = "Search apps",
//!     spotlight = "apps-search-input",
//!     order = 10,
//! )]
//! #[component]
//! pub fn AppsSearchHelp() -> impl IntoView {
//!     view! {
//!         <input id="apps-search-input" />
//!         <p data-testid="help-step-apps-search">"Use search to find apps."</p>
//!     }
//! }
//! ```
//!
//! Keep `feature_highlight` stable across releases. Renaming a key is a new
//! highlight: returning users see that step once without replaying older keys on
//! the same route.
//!
//! ### 2. Mid — force-link inventory in the host binary
//!
//! Inventory only runs if the step module is linked. Call an empty
//! `ensure_help_steps_linked()` from the app crate (see `uf_apps::ensure_help_linked`)
//! at startup or from a route entry point so `inventory` submissions are retained.
//!
//! ```rust,ignore
//! uf_apps::ensure_help_linked();
//! uf_welcome::ensure_help_linked();
//! ```
//!
//! Hosts that define their own steps use the same pattern in `help_steps.rs`.
//!
//! ### 3. Mount — enable `offering-help` and use the stock shell
//!
//! Depend on `uf-integrations` with feature `offering-help` (or `full`). The default
//! `UnifiedFieldShellLayout` mounts [`HelpTourPlayer`] beside page chrome; call [`ensure_linked`] once if you rely
//! on uf-help's app-bar utility registration.
//!
//! ### 4. Runtime — matching, progress, replay
//!
//! - **Route matching** — step `route` is exact pathname equality except
//!   `"/apps/:app_name"`, which matches `/apps/{slug}` (one segment). Valence rows
//!   store the inventory pattern, not the live slug ([`inventory_route_keys_for_pathname`]).
//! - **Pending** — no visit row, or visit with `replay == true` ([`compute_pending`]).
//! - **Signed-out** — progress lives under [`LOCAL_STORAGE_KEY`]; on first signed-in
//!   write, [`help_mark_steps_seen`] merges missing local rows into Valence
//!   ([`merge_local_into_server`] on read keeps local-only rows visible until then).
//! - **Replay** — Help → Replay spotlight tour sets replay for the **current route
//!   only** ([`help_request_replay_for_route`] / [`local_request_replay_for_route`]).
//!
//! Server failures map through [`HelpError::into_server_fn_error`]: callers receive
//! [`ServerFnError::ServerError`](leptos::prelude::ServerFnError) with the
//! [`Display`](std::fmt::Display) message only (no structured variant on the wire).
//! See [`mod@error`] and [`mod@server`] `# Errors` sections.
//!
//! ## Examples
//!
//! | Example | What it shows |
//! |---------|---------------|
//! | `examples/shell-chrome-host/` | Shell layout with default offerings; `cargo check -p shell-chrome-host --features ssr` |
//! | `uf-product-ui-e2e/end2end/tests/help_spotlight.spec.ts` | Once (anon/authed), replay current route, access-gate skip, apps/welcome steps |
//! | `uf-apps` `help_steps.rs` | Seeded `/apps` steps + `uf_apps::ensure_help_linked` |
//! | `uf-notifications` `help_steps.rs` | Bell / inbox steps on `/notifications` (link inventory the same way) |
//!
//! ## Getting started
//!
//! ```bash
//! cargo doc -p uf-help --features ssr --open
//! cargo check -p uf-help --features ssr
//! ```

#![allow(missing_docs)]

pub use inventory;
pub use uf_help_macros::help_spotlight_step;

mod app_bar_help;
mod error;
mod github;
mod report;
mod repository;
mod server;
mod service;
#[cfg(feature = "ssr")]
mod service_reports;
mod step_inventory;
mod tour;

pub use app_bar_help::{AppBarHelpButton, APP_BAR_UTILITY_ORDER};
pub use error::HelpError;
pub use github::{
    clear_github_token_resolver, set_github_token_resolver, BugReportPayload, CreateIssue,
    FeatureRequestPayload, GitHubFeedbackClient, HttpGitHubClient, MockGitHubClient,
    PrivateVulnReport, SecurityReportPayload, HELP_GITHUB_FEEDBACK_SECRET,
    UF_HELP_GITHUB_TOKEN_ENV,
};
pub use report::{HelpReportDialog, HelpReportKind};
pub use repository::{parse_github_owner_repo, resolve_help_repository};
pub use server::{
    help_list_visits_for_route, help_mark_steps_seen, help_pending_steps_for_route,
    help_repository_for_route, help_request_replay_for_route, submit_help_bug_report,
    submit_help_feature_request, submit_help_security_report, HelpPendingStepDto,
};
pub use service::{
    apply_replay_for_route, compute_pending, local_mark_steps_seen, local_request_replay_for_route,
    merge_local_into_server, read_local_visits, read_local_visits_for_route, replay_from_stored,
    replay_to_stored, write_local_visits, HelpStepKey, HelpVisitRecord, LOCAL_STORAGE_KEY,
};
pub use step_inventory::{
    collect_help_steps, collect_help_steps_for_route, inventory_route_keys_for_pathname,
    route_matches, HelpStepDescriptor,
};
pub use tour::{notify_help_replay, request_replay_current_route, HelpTourPlayer};

use uf_product::register_app_bar_utility;

/// Ensure this crate's inventory submissions are linked (call from hosts if needed).
pub fn ensure_linked() {
    register_app_bar_utility();
}
