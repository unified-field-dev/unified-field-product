//! Harness PermissionBackend for product UI e2e (not live Gauge).

use std::sync::Arc;

use leptos::prelude::*;
use uf_product::permissions::{provide_permission_backend, PermissionBackend};

/// Tower-session key set by seed when allowlisted permission checks should pass.
pub const E2E_PERMISSION_ALLOW_SESSION_KEY: &str = "uf_e2e_permission_allow";

/// Permission name that always denies (deny-gate demo page).
pub const E2E_PERMISSION_DENY: &str = "e2e.permission.deny";

/// Permission name allowed when [`E2E_PERMISSION_ALLOW_SESSION_KEY`] is set.
pub const E2E_PERMISSION_ALLOW: &str = "e2e.permission.allow";

struct E2ePermissionBackend;

#[async_trait::async_trait]
impl PermissionBackend for E2ePermissionBackend {
    async fn has_permission(&self, permission_name: &str) -> Result<bool, ServerFnError> {
        match permission_name {
            E2E_PERMISSION_DENY => Ok(false),
            E2E_PERMISSION_ALLOW => Ok(e2e_permission_allow_from_session().await),
            _ => Ok(false),
        }
    }
}

async fn e2e_permission_allow_from_session() -> bool {
    use leptos_axum::extract;
    use tower_sessions::Session;

    let Ok(session) = extract::<Session>().await else {
        return false;
    };
    session
        .get::<bool>(E2E_PERMISSION_ALLOW_SESSION_KEY)
        .await
        .ok()
        .flatten()
        .unwrap_or(false)
}

/// Install the e2e permission stub into Leptos context (SSR request scope).
pub fn wire_e2e_permissions() {
    provide_permission_backend(Arc::new(E2ePermissionBackend));
}
