//! Optional Help product offering for Unified Field shells.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | App-bar Help menu (report + replay) | Help center CMS / markdown routes |
//! | Spotlight tour inventory + player | Orbital spotlight primitives |
//! | Visit progress (Valence + localStorage) | Platform anon identity claim |
//! | GitHub feedback bot (issues / private vuln) | Neutrino secret store implementation |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | App-bar Help control | [`AppBarHelpButton`] |
//! | Default tour player | [`HelpTourPlayer`] |
//! | Author a step | [`help_spotlight_step`] via `uf-help-macros` |
//! | Collect inventory | [`collect_help_steps`], [`collect_help_steps_for_route`], [`HelpStepDescriptor`] |
//! | Match a pathname to a step `route` | [`route_matches`] (exact, plus `"/apps/:app_name"`) |
//! | Visits / pending | [`service`], [`server`] |
//! | Reports | [`report`], [`submit_help_bug_report`], … |
//! | Link inventory into the host | [`ensure_linked`] |
//!
//! ## Features
//!
//! - **Incremental highlights** — progress is per `feature_highlight`. A later
//!   release that adds a new key shows that step to returning users; already-seen
//!   steps stay quiet until Help → Replay spotlight tour (current route only).
//! - **AdaptiveMenu Help** — Bug, Feature, Security, Replay (popover ≥ Md, drawer below).
//! - **Repository from `uf_app!`** — deep links and the GitHub bot target
//!   `AppRegistration.repository` for the active route.
//! - **Route matching** — step `route` is exact pathname equality, except
//!   `"/apps/:app_name"` which matches a single segment under `/apps` (visits store
//!   that pattern, not the slug).
//! - **Access gates** — auto-play stays off while a RequireAuthenticated empty
//!   state is showing (sign-in, email verification, or permission required).
//!
//! ## Getting started
//!
//! Depend on this crate (or enable `uf-integrations` feature `offering-help` /
//! `full`). The stock shell mounts [`HelpTourPlayer`] when `offering-help` is on.

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
