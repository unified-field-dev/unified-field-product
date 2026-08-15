//! Product app bar: branding, breadcrumbs, search, and host-controlled utilities.
//!
//! On viewports below [`Breakpoint::Md`], wraps the sticky bar in Orbital
//! [`HideOnScroll`] so chrome tucks on scroll-down and returns on scroll-up.
//!
//! # Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | [`UnifiedFieldAppBar`] chrome composition | Session / appearance preference storage (`uf-product`) |
//! | [`BreadcrumbLink`], search / utilities slots | Auth menu widget (`lepton_shell::AppBarUserMenu` via [`crate::ShellAuthMenu`]) |
//! | Default utilities from offering inventory | Help / Apps / Appearance button widgets (`uf-help`, `uf-apps`, `uf-appearance`) |
//!
//! # Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Top app bar for a product shell | [`UnifiedFieldAppBar`] |
//! | Breadcrumb trail | [`BreadcrumbLink`] |
//! | Host-provided auth menu region | [`ShellAuthMenu`] |
//! | Host-controlled trailing utilities | [`AppBarUtilities`] slot (omit for [`DefaultAppBarUtilities`]) |
//! | Optional search region | [`AppBarSearchSlot`] |
//!
//! # Example
//!
//! Crate-root Getting started shows full shell composition. For the search
//! combobox alone, see [`crate::SearchSourcePicker`].

use crate::shell_layout::ShellSidebarToggle;
use crate::{HostNotificationBell, WorkspaceSearch, WorkspaceSearchMobileTrigger};
use leptos::prelude::*;
use orbital_base_components::use_breakpoint_down;
use orbital_theme::Breakpoint;
use uf_product::collect_app_bar_utilities;
use uf_product::components::{
    AppBar, AppBarDensity, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing,
    HideOnScroll, LayoutSidebarToggle, MaterialCorners, MaterialElevation, MaterialVariant, Title3,
};
use uf_product::primitives::{
    Avatar, AvatarColor, AvatarConfig, AvatarShape, Breadcrumb, BreadcrumbButton, BreadcrumbItem,
};
use uf_product::provide_app_bar_menu_extras;
use uf_product::theme::product_avatar_letter;

/// Optional auth menu slot — host passes `lepton_shell::AppBarUserMenu` here.
#[slot]
pub struct ShellAuthMenu {
    /// The auth menu content, typically a user avatar/dropdown component.
    pub children: Children,
}

/// Optional trailing utilities slot — host children replace the default offering pack.
#[slot]
pub struct AppBarUtilities {
    /// Host-controlled trailing utilities (buttons, menus, etc.).
    pub children: ChildrenFn,
}

/// Breadcrumb navigation item for [`UnifiedFieldAppBar`].
#[derive(Clone, PartialEq, Eq)]
pub struct BreadcrumbLink {
    /// Display text for the breadcrumb.
    pub title: String,
    /// Link target the breadcrumb navigates to when clicked.
    pub url: String,
}

impl BreadcrumbLink {
    /// Construct a breadcrumb entry from a title and link target.
    pub fn new(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

#[component]
fn AppBarBranding(
    /// App name.
    app_name: String,
    /// Avatar letter.
    avatar_letter: String,
    /// Homepage URL.
    homepage_url: String,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Logo {
            display: flex;
            align-items: center;
            gap: 12px;
            color: var(--orb-color-text-primary);
        }

        .LogoLink {
            display: flex;
            align-items: center;
            gap: 12px;
            text-decoration: none;
            color: inherit;
            transition: opacity 0.2s ease;
        }

        .LogoLink:hover {
            opacity: 0.8;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <a href=homepage_url class=class_names.logo_link>
            <div class=class_names.logo>
                <Avatar config=AvatarConfig {
                    initials: Some(avatar_letter),
                    name: Some(app_name.clone()),
                    shape: AvatarShape::Square,
                    size: Some(32),
                    color: AvatarColor::Brand,
                    ..Default::default()
                } />
                <Title3>{app_name}</Title3>
            </div>
        </a>
    }
}

#[component]
fn AppBarBreadcrumbs(
    /// List of breadcrumbs.
    breadcrumbs: Vec<BreadcrumbLink>,
) -> impl IntoView {
    if breadcrumbs.is_empty() {
        let _: () = view! { <></> };
        return ().into_any();
    }

    view! {
        <Breadcrumb>
            {breadcrumbs.into_iter().map(|breadcrumb| {
                view! {
                    <BreadcrumbItem>
                        <a href=breadcrumb.url style="text-decoration: none; color: inherit;">
                            <BreadcrumbButton>{breadcrumb.title}</BreadcrumbButton>
                        </a>
                    </BreadcrumbItem>
                }
            }).collect::<Vec<_>>()}
        </Breadcrumb>
    }
    .into_any()
}

/// Default workspace content-index search for product shells.
#[component]
pub fn AppBarSearchSlot() -> impl IntoView {
    view! { <WorkspaceSearch /> }
}

/// Default trailing utilities from linked product offerings (inventory).
///
/// With `uf-integrations` feature `full` (and a host that links `uf-apps`), this
/// typically renders Help → Apps → Appearance. Hosts may replace it via
/// [`AppBarUtilities`].
#[component]
pub fn DefaultAppBarUtilities() -> impl IntoView {
    #[cfg(feature = "offering-help")]
    uf_help::ensure_linked();
    #[cfg(feature = "offering-appearance")]
    uf_appearance::ensure_linked();
    #[cfg(feature = "offering-apps")]
    {
        // `uf-apps` is not a dep of this crate (cycle with AppsLayout). Hosts that
        // enable `offering-apps` / `full` should also depend on `uf-apps` so its
        // inventory row is linked (shell-chrome-host and product hosts already do).
    }

    let items = collect_app_bar_utilities();
    view! {
        {items
            .into_iter()
            .map(|c| (c.render)())
            .collect_view()}
    }
}

/// Alias for [`DefaultAppBarUtilities`] (legacy name).
#[component]
pub fn AppBarTrailingSlot() -> impl IntoView {
    view! { <DefaultAppBarUtilities /> }
}

/// Unified Field product app bar — branding, search, and shell utilities.
#[component]
pub fn UnifiedFieldAppBar(
    /// App name.
    app_name: String,
    /// Optional app ID.
    #[prop(optional)]
    app_id: Option<&'static str>,
    /// Optional app logo initial.
    #[prop(optional)]
    app_logo_initial: Option<String>,
    /// Optional homepage URL.
    #[prop(optional)]
    homepage_url: Option<String>,
    /// Optional breadcrumbs.
    #[prop(optional)]
    breadcrumbs: Option<Vec<BreadcrumbLink>>,
    /// Whether interactive is enabled.
    #[prop(default = true)]
    interactive: bool,
    /// Optional flag for whether to show sidebar toggle.
    #[prop(optional)]
    show_sidebar_toggle: Option<bool>,
    /// Optional auth menu.
    #[prop(optional)]
    auth_menu: Option<ShellAuthMenu>,
    /// Optional trailing utilities (omit to use [`DefaultAppBarUtilities`]).
    #[prop(optional)]
    app_bar_utilities: Option<AppBarUtilities>,
) -> impl IntoView {
    let avatar_letter = app_id
        .map(|id| product_avatar_letter(id).to_string())
        .or(app_logo_initial)
        .unwrap_or_else(|| {
            app_name
                .chars()
                .next()
                .map_or_else(|| "?".to_string(), |c| c.to_uppercase().to_string())
        });
    let homepage_url = homepage_url.unwrap_or_else(|| "/".to_string());
    let breadcrumbs = breadcrumbs.unwrap_or_default();
    let show_sidebar_toggle = show_sidebar_toggle
        .or_else(|| use_context::<ShellSidebarToggle>().map(|ctx| ctx.0))
        .unwrap_or(false);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .LeadingRow {
            display: flex;
            align-items: center;
            gap: 12px;
            min-width: 0;
        }

        .TrailingRow {
            display: flex;
            align-items: center;
            gap: 8px;
            width: 100%;
            min-width: 0;
            justify-content: flex-end;
        }

        .SearchContainer {
            width: min(250px, 100%);
            flex: 1 1 auto;
            min-width: 0;
            max-width: 250px;
            margin-right: auto;
        }

        .Utilities {
            display: flex;
            align-items: center;
            gap: 8px;
            flex-shrink: 0;
        }

        .ChromeLocked {
            pointer-events: none;
        }
    };

    // Same breakpoint as Layout sidebar overlay — one avatar/login menu on phones.
    let compact = use_breakpoint_down(Breakpoint::Md);
    provide_app_bar_menu_extras(compact.into());

    let utilities_children = app_bar_utilities.map(|AppBarUtilities { children }| children);

    view! {
        <style>{style_sheet}</style>
        <HideOnScroll enabled=Signal::derive(move || compact.get())>
            <AppBar
                class=Signal::derive(move || {
                    if interactive {
                        String::new()
                    } else {
                        class_names.chrome_locked.to_string()
                    }
                })
                position=AppBarPosition::Sticky
                density=AppBarDensity::Compact
            >
                <AppBarMaterial
                    variant=MaterialVariant::Frost
                    elevation=MaterialElevation::Flat
                    corners=MaterialCorners::Square
                    slot
                />
                <AppBarLeading slot>
                    <div class=class_names.leading_row>
                        {show_sidebar_toggle.then(|| view! {
                            <LayoutSidebarToggle />
                        })}
                        <AppBarBranding
                            app_name=app_name
                            avatar_letter=avatar_letter
                            homepage_url=homepage_url
                        />
                        <AppBarBreadcrumbs breadcrumbs=breadcrumbs />
                    </div>
                </AppBarLeading>
                <AppBarTrailing slot>
                    <div class=class_names.trailing_row data-testid="app-bar-trailing-row">
                        {move || {
                            let utilities_view = match &utilities_children {
                                Some(children) => children().into_any(),
                                None => view! { <DefaultAppBarUtilities /> }.into_any(),
                            };
                            if compact.get() {
                                // Compact: search icon → Dialog; utilities stay in trailing.
                                view! {
                                    <div class=class_names.utilities data-testid="app-bar-trailing-compact">
                                        <WorkspaceSearchMobileTrigger />
                                        {utilities_view}
                                    </div>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <div class=class_names.search_container data-testid="app-bar-search">
                                        <AppBarSearchSlot />
                                    </div>
                                    <div class=class_names.utilities data-testid="app-bar-trailing">
                                        {utilities_view}
                                    </div>
                                }
                                .into_any()
                            }
                        }}
                        <div data-testid="app-bar-notification-bell">
                            <HostNotificationBell />
                        </div>
                        <div data-testid="app-bar-user-menu">
                            {auth_menu.map(|ShellAuthMenu { children }| children())}
                        </div>
                    </div>
                </AppBarTrailing>
            </AppBar>
        </HideOnScroll>
    }
}
