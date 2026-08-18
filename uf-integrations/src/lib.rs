#![recursion_limit = "256"]
//! Shell integrations (app bar, search picker, layout). Auth menu is host-provided
//! via a slot or Leptos context ([`HostAuthMenu`]). Trailing utilities are a generic
//! children slot (default = optional product offerings via inventory).
//!
//! Session loading and route guards live in `uf-product`. Auth UI widgets come
//! from `lepton_shell::AppBarUserMenu`. Help, Apps, and Appearance widgets live
//! in sibling offering crates. Search provider registry and Valence queries live
//! in `uf-search-core`.
//!
//! ## Features
//!
//! - **App bar** — [`UnifiedFieldAppBar`] composes branding, breadcrumbs
//!   ([`BreadcrumbLink`]), workspace [`WorkspaceSearch`] / compact Dialog, and
//!   [`AppBarUtilities`] (omit for [`DefaultAppBarUtilities`]).
//! - **Offerings** — `offering-help`, `offering-apps`, `offering-appearance`, and
//!   `full` (default) control which product offerings participate in the default
//!   utilities pack. `offering-apps` is a marker: link `uf-apps` from the host.
//! - **Shell layout** — [`UnifiedFieldShellLayout`] wraps Orbital's `Layout` with the app
//!   bar/left-nav slots ([`ShellAppBar`], [`ShellLeftNav`]) and a permission-denied toast bus.
//! - **Picker search** — [`SearchSourcePicker`] is a reusable, multi-source combobox backed
//!   by `uf-search-core` (separate from AppBar content index).
//! - **Workspace content search** — [`WorkspaceSearch`] / [`WorkspaceSearchMobileTrigger`]
//!   query `uf_product::workspace_search` (per-user index).
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Branding + breadcrumbs + search + utilities | [`UnifiedFieldAppBar`], [`BreadcrumbLink`], [`AppBarSearchSlot`], [`AppBarUtilities`], [`DefaultAppBarUtilities`] |
//! | Host auth user menu | [`ShellAuthMenu`] slot, or [`provide_shell_auth_menu`] + [`HostAuthMenu`] |
//! | Host notification bell | [`provide_shell_notification_bell`] + [`HostNotificationBell`] |
//! | Top-level shell composition (app bar + left nav + toasts) | [`UnifiedFieldShellLayout`], [`ShellAppBar`], [`ShellLeftNav`], [`ShellSidebarToggle`] |
//! | Multi-source search combobox (pickers) | [`SearchSourcePicker`] |
//! | AppBar content-index search | [`WorkspaceSearch`], [`WorkspaceSearchMobileTrigger`], [`AppBarSearchSlot`] |
//! | Generic "not built yet" / 404 pages | [`UnifiedFieldComingSoonPage`], [`coming_soon_fill_for_path`], [`UnifiedFieldNotFoundPage`] |
//!
//! ## Getting started
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use lepton_shell::AppBarUserMenu;
//! use uf_integrations::{
//!     provide_shell_auth_menu, provide_shell_notification_bell, HostAuthMenu, ShellAppBar,
//!     ShellAuthMenu, UnifiedFieldAppBar, UnifiedFieldShellLayout,
//! };
//! use uf_notifications::NotificationBell;
//!
//! // At the host root (once):
//! provide_shell_auth_menu(|| view! { <AppBarUserMenu /> });
//! provide_shell_notification_bell(|| view! { <NotificationBell /> });
//!
//! #[component]
//! fn AppShell(children: Children) -> impl IntoView {
//!     view! {
//!         <UnifiedFieldShellLayout>
//!             <ShellAppBar slot>
//!                 <UnifiedFieldAppBar app_name="My App".to_string()>
//!                     <ShellAuthMenu slot:auth_menu>
//!                         <HostAuthMenu />
//!                     </ShellAuthMenu>
//!                 </UnifiedFieldAppBar>
//!             </ShellAppBar>
//!             {children()}
//!         </UnifiedFieldShellLayout>
//!     }
//! }
//! ```
//!
//! Mid-level: wire [`SearchSourcePicker`] with keys from `uf-search-core` and fill `options`
//! from a server fn that calls `SearchSourceRegistry::query_many` (see the picker's `# Examples`).
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | `UnifiedFieldShellLayout` + `UnifiedFieldAppBar` + host auth menu + notification bell |
//! | Mid | [`SearchSourcePicker`] item docs | Combobox keys + `query_many` options |
//! | Detailed | workspace `examples/shell-chrome-host` | Layout, app bar, search, coming-soon, and 404 on one tree |
//!
//! ```bash
//! cargo check -p shell-chrome-host --features ssr
//! ```
//!
//! Lib doctests are disabled (`[lib] doctest = false`); use workspace hosts such as
//! `examples/shell-chrome-host` for compile-checked composition.
//!
//! ## Where to look next
//!
//! - [`UnifiedFieldShellLayout`] — top-level shell composition.
//! - [`UnifiedFieldAppBar`] — the app bar itself.
//! - [`SearchSourcePicker`] — multi-source search combobox.
//! - `uf-product` — session, `RequireAuthenticated`, appearance preferences.
//! - `uf-help` / `uf-apps` / `uf-appearance` — optional app-bar offerings.
//! - `uf-search-core` — `SearchSourceKey` / provider registry.
//! - `examples/shell-chrome-host` — runnable shell chrome composition.

// Leptos `#[component]` / slot expansions still leave many props undocumented.
// Document facades at the crate root and on entry components; keep this allow until
// prop-level docs are ratcheted per component.
#![allow(missing_docs)]
mod app_bar;
mod coming_soon_page;
mod host_auth_menu;
mod host_notification_bell;
mod not_found_page;
mod search_source_picker;
mod shell_layout;
mod workspace_search;

pub use app_bar::{
    AppBarSearchSlot, AppBarTrailingSlot, AppBarUtilities, BreadcrumbLink, DefaultAppBarUtilities,
    ShellAuthMenu, UnifiedFieldAppBar,
};
pub use coming_soon_page::{coming_soon_fill_for_path, UnifiedFieldComingSoonPage};
pub use host_auth_menu::{provide_shell_auth_menu, HostAuthMenu, ShellAuthMenuFactory};
pub use host_notification_bell::{
    provide_shell_notification_bell, HostNotificationBell, ShellNotificationBellFactory,
};
pub use not_found_page::UnifiedFieldNotFoundPage;
pub use search_source_picker::SearchSourcePicker;
pub use shell_layout::{ShellAppBar, ShellLeftNav, ShellSidebarToggle, UnifiedFieldShellLayout};
pub use workspace_search::{WorkspaceSearch, WorkspaceSearchDialog, WorkspaceSearchMobileTrigger};
