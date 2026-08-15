//! SSR request-context utilities — [`higgs_host`] + [`higgs()`] (no concrete auth models).

#[allow(deprecated)]
pub use higgs_host::{
    current_operation, data_plane, host_ctx, with_operation, DataPlaneCtx, HostRequestCtx,
};

/// Extract Higgs request context for server functions.
pub async fn higgs() -> Result<higgs::Higgs, leptos::prelude::ServerFnError> {
    higgs::Higgs::from_request().await
}

/// Build session-scoped Valence for the current request actor.
pub fn valence(ctx: &higgs::Higgs) -> Result<valence::Valence, leptos::prelude::ServerFnError> {
    ctx.valence()
        .map_err(|e| leptos::prelude::ServerFnError::ServerError(e.to_string()))
}

/// Parse `session_user_id()` into a Valence [`valence::RecordId`].
pub fn session_user_record_id(
    session_user_id: &str,
) -> Result<valence::RecordId, leptos::prelude::ServerFnError> {
    let (table, id) = session_user_id.split_once(':').ok_or_else(|| {
        leptos::prelude::ServerFnError::new(format!("invalid session user id: {session_user_id}"))
    })?;
    Ok(valence::RecordId::new(table, id))
}

#[cfg(test)]
mod tests;
