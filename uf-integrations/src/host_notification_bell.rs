//! Host-provided notification bell for product shells.

use std::sync::Arc;

use leptos::prelude::*;

/// Factory that renders the host's notification bell (`uf_notifications::NotificationBell`).
#[derive(Clone)]
pub struct ShellNotificationBellFactory {
    render: Arc<dyn Fn() -> AnyView + Send + Sync>,
}

impl ShellNotificationBellFactory {
    /// Create a factory from a view builder.
    pub fn new<F, V>(f: F) -> Self
    where
        F: Fn() -> V + Send + Sync + 'static,
        V: IntoView,
    {
        Self {
            render: Arc::new(move || f().into_any()),
        }
    }

    /// Render the host notification bell.
    pub fn render(&self) -> AnyView {
        (self.render)()
    }
}

/// Provide the notification bell for product layouts that call [`HostNotificationBell`].
pub fn provide_shell_notification_bell<F, V>(f: F)
where
    F: Fn() -> V + Send + Sync + 'static,
    V: IntoView,
{
    provide_context(ShellNotificationBellFactory::new(f));
}

/// Renders the host-provided notification bell, or nothing when unset.
#[component]
pub fn HostNotificationBell() -> impl IntoView {
    match use_context::<ShellNotificationBellFactory>() {
        Some(factory) => factory.render(),
        None => ().into_any(),
    }
}
