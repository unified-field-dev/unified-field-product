//! Runtime permission evaluation plugged in by the host at composition time.

use std::sync::Arc;

use leptos::prelude::*;

/// Evaluates named permissions for the current request actor.
#[cfg(feature = "ssr")]
#[async_trait::async_trait]
pub trait PermissionBackend: Send + Sync {
    /// Return whether the current actor holds `permission_name`.
    async fn has_permission(&self, permission_name: &str) -> Result<bool, ServerFnError>;
}

/// Install a permission evaluator for this Leptos runtime (host bootstrap).
#[cfg(feature = "ssr")]
pub fn provide_permission_backend(backend: Arc<dyn PermissionBackend>) {
    provide_context(backend);
}

/// Read the installed backend, if any.
#[cfg(feature = "ssr")]
pub fn use_permission_backend() -> Option<Arc<dyn PermissionBackend>> {
    use_context::<Arc<dyn PermissionBackend>>()
}

/// Fail-closed gate for server functions and admin paths.
#[cfg(feature = "ssr")]
pub async fn require_permission(permission_name: &str) -> Result<(), ServerFnError> {
    let Some(backend) = use_permission_backend() else {
        return Err(ServerFnError::new(
            higgs::server_runtime::permission_denied_payload(permission_name),
        ));
    };
    let allowed = backend.has_permission(permission_name).await.map_err(|e| {
        ServerFnError::new(higgs::server_runtime::permission_check_failed_payload(
            permission_name,
            &e.to_string(),
        ))
    })?;
    if !allowed {
        return Err(ServerFnError::new(
            higgs::server_runtime::permission_denied_payload(permission_name),
        ));
    }
    Ok(())
}

/// Soft check: returns false when no backend is installed or the check fails.
#[cfg(feature = "ssr")]
pub async fn has_permission(permission_name: &str) -> bool {
    let Some(backend) = use_permission_backend() else {
        return false;
    };
    backend
        .has_permission(permission_name)
        .await
        .unwrap_or(false)
}

/// Route-guard helper: fail closed when no backend is wired.
///
/// Callable from SSR and hydrate (POST to the host). Hosts must
/// [`provide_permission_backend`] in the Leptos request context.
///
/// # Errors
///
/// Returns [`ServerFnError`] when the backend check itself fails (mapped from
/// backend errors). Missing backend or empty name yields [`Ok`]`(false)`.
#[server(CheckPermissionByName)]
pub async fn check_permission_by_name(permission_name: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let name = permission_name.trim();
        if name.is_empty() {
            return Ok(false);
        }
        let Some(backend) = use_permission_backend() else {
            return Ok(false);
        };
        return backend.has_permission(name).await;
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = permission_name;
        Ok(false)
    }
}
