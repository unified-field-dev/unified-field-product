//! Help step inventory collected from `#[help_spotlight_step]`.
//!
//! Each [`HelpStepDescriptor`] is submitted through the `inventory` crate at link
//! time. App crates keep steps in a `help_steps` module and expose an empty
//! `ensure_help_steps_linked()` that references every step component so the linker
//! retains submissions (see `uf_apps::ensure_help_linked`).
//!
//! [`collect_help_steps_for_route`] filters by [`route_matches`]: exact pathname
//! equality, plus the `"/apps/:app_name"` pattern for single-segment app overview
//! paths. [`inventory_route_keys_for_pathname`] returns the pattern keys used when
//! reading or writing Valence visit rows (not the live browser slug).

use leptos::prelude::AnyView;
use uf_product::primitives::PopoverPosition;

/// One spotlight tour step registered by product UI authors.
///
/// `feature_highlight` is the durable identity for “seen”. Adding a new key later
/// shows only that step to users who already finished older ones on the route.
pub struct HelpStepDescriptor {
    /// Route this step belongs to.
    ///
    /// Usually an exact pathname (for example `"/apps"` or `"/welcome"`).
    /// The pattern `"/apps/:app_name"` matches any single-segment app overview
    /// path such as `/apps/welcome` (not bare `/apps` or multi-segment paths).
    pub route: &'static str,
    /// Stable per-step identity within the route.
    pub feature_highlight: &'static str,
    /// Human-readable panel header (defaults to `feature_highlight` when the
    /// macro omits `title`).
    pub title: &'static str,
    /// Optional DOM element id for the spotlight cutout.
    ///
    /// When omitted or empty, Orbital centers the panel in the viewport (no
    /// cutout target).
    pub spotlight: Option<&'static str>,
    /// Sort order within the route (lower first).
    pub order: u16,
    /// Popover placement relative to the spotlight anchor (default: Top).
    pub position: Option<PopoverPosition>,
    /// Renders the step body as an [`AnyView`].
    pub render: fn() -> AnyView,
}

inventory::collect!(HelpStepDescriptor);

/// Collect every registered help step, sorted by [`HelpStepDescriptor::order`].
#[must_use]
pub fn collect_help_steps() -> Vec<&'static HelpStepDescriptor> {
    let mut items: Vec<&'static HelpStepDescriptor> =
        inventory::iter::<HelpStepDescriptor>.into_iter().collect();
    items.sort_by_key(|d| d.order);
    items
}

/// Whether inventory `pattern` applies to browser `pathname`.
///
/// Exact equality for normal routes. `"/apps/:app_name"` matches pathnames with
/// exactly two segments (`apps` + non-empty slug), not `/apps` or `/apps/a/b`.
#[must_use]
pub fn route_matches(pattern: &str, pathname: &str) -> bool {
    if pattern == pathname {
        return true;
    }
    if pattern == "/apps/:app_name" {
        return matches_apps_app_name(pathname);
    }
    false
}

fn matches_apps_app_name(pathname: &str) -> bool {
    let path = pathname.trim_end_matches('/');
    let Some(rest) = path.strip_prefix("/apps/") else {
        return false;
    };
    !rest.is_empty() && !rest.contains('/')
}

/// Collect help steps whose [`HelpStepDescriptor::route`] matches `pathname`.
#[must_use]
pub fn collect_help_steps_for_route(pathname: &str) -> Vec<&'static HelpStepDescriptor> {
    collect_help_steps()
        .into_iter()
        .filter(|d| route_matches(d.route, pathname))
        .collect()
}

/// Distinct inventory route keys that apply to `pathname` (exact or pattern).
///
/// Used when reading/writing Valence visits so rows keyed by
/// `"/apps/:app_name"` are found while browsing `/apps/{slug}`.
#[must_use]
pub fn inventory_route_keys_for_pathname(pathname: &str) -> Vec<&'static str> {
    let mut keys: Vec<&'static str> = collect_help_steps_for_route(pathname)
        .into_iter()
        .map(|d| d.route)
        .collect();
    keys.sort_unstable();
    keys.dedup();
    if keys.is_empty() {
        // Preserve legacy callers that store the raw pathname.
        return Vec::new();
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_help_steps_is_sorted_by_order() {
        let items = collect_help_steps();
        let mut prev = 0u16;
        for item in &items {
            assert!(item.order >= prev);
            prev = item.order;
            assert!(!item.route.is_empty());
            assert!(!item.feature_highlight.is_empty());
        }
    }

    #[test]
    fn route_matches_exact() {
        assert!(route_matches("/apps", "/apps"));
        assert!(route_matches("/welcome", "/welcome"));
        assert!(!route_matches("/apps", "/apps/welcome"));
        assert!(!route_matches("/welcome", "/apps"));
    }

    #[test]
    fn route_matches_apps_app_name_pattern() {
        assert!(route_matches("/apps/:app_name", "/apps/welcome"));
        assert!(route_matches("/apps/:app_name", "/apps/apps"));
        assert!(route_matches("/apps/:app_name", "/apps/foo"));
        assert!(!route_matches("/apps/:app_name", "/apps"));
        assert!(!route_matches("/apps/:app_name", "/apps/"));
        assert!(!route_matches("/apps/:app_name", "/apps/foo/bar"));
        assert!(!route_matches("/apps/:app_name", "/welcome"));
    }
}
