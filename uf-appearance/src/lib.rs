//! Optional Appearance product offering — desktop app-bar appearance popover.
//!
//! Preference storage and Valence services stay in `uf-product` for now.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | App-bar appearance button / popover | `AppearancePreferences` persistence (`uf-product`) |
//! | Inventory registration for default utilities | Compact avatar-menu appearance rows (`uf-product`) |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | App-bar appearance popover | [`AppBarAppearanceButton`] |
//! | Link inventory into the host | [`ensure_linked`] |
//!
//! ## Getting started
//!
//! Depend on this crate (or enable `uf-integrations` feature `offering-appearance` / `full`).

#![allow(missing_docs)]

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use uf_product::app_bar_dark_mode_bind;
use uf_product::paths::USER_APPEARANCE;
use uf_product::primitives::{
    Button, ButtonAppearance, Divider, Flex, FlexGap, Popover, PopoverPosition, PopoverSize,
    PopoverTrigger, PopoverTriggerType, Switch,
};
use uf_product::{register_app_bar_utility, AppBarUtilityContribution};

/// Sort order for the default Appearance control in the app-bar utilities pack.
pub const APP_BAR_UTILITY_ORDER: u8 = 30;

/// Appearance popover for the Unified Field app bar — dark / light mode and settings link.
#[component]
pub fn AppBarAppearanceButton() -> impl IntoView {
    let navigate = use_navigate();
    let dark = app_bar_dark_mode_bind();

    let open_appearance_settings = Callback::new(move |_| {
        navigate(USER_APPEARANCE, NavigateOptions::default());
    });

    view! {
        <Popover
            trigger_type=PopoverTriggerType::Click
            position=PopoverPosition::BottomEnd
            size=PopoverSize::Small
        >
            <PopoverTrigger slot>
                <div data-testid="app-bar-appearance">
                    <Button
                        appearance=ButtonAppearance::Subtle
                        icon=icondata::AiBgColorsOutlined
                        attr:aria-label="Appearance"
                    />
                </div>
            </PopoverTrigger>
            <Flex vertical=true gap=FlexGap::Small>
                <div data-testid="app-bar-appearance-menu">
                    <Switch bind=dark label="Dark mode" />
                </div>
                <Divider />
                <div data-testid="app-bar-appearance-settings-link">
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on_click=open_appearance_settings
                    >
                        "Appearance settings"
                    </Button>
                </div>
            </Flex>
        </Popover>
    }
}

fn render_appearance_utility() -> AnyView {
    view! { <AppBarAppearanceButton /> }.into_any()
}

inventory::submit! {
    AppBarUtilityContribution::new(APP_BAR_UTILITY_ORDER, "appearance", render_appearance_utility)
}

/// Ensure this crate's inventory submissions are linked (call from hosts if needed).
pub fn ensure_linked() {
    register_app_bar_utility();
}
