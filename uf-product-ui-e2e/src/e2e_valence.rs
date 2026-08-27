//! Process-wide mem Valence for Playwright featured-admin scenarios.

use std::sync::{Arc, OnceLock};

use valence::{
    register_noop_deletion_dispatcher_for_tests, router_key, Actor, InMemoryBackend, Valence,
    MEM_ENGINE_ID,
};

static E2E_VALENCE: OnceLock<Arc<Valence>> = OnceLock::new();

/// Build (once) the shared System Valence for this e2e host process.
pub async fn init_e2e_valence() -> Arc<Valence> {
    if let Some(existing) = E2E_VALENCE.get() {
        return existing.clone();
    }
    register_noop_deletion_dispatcher_for_tests();
    let key = router_key("default", MEM_ENGINE_ID);
    let valence = Arc::new(
        Valence::builder()
            .add_backend("default", Arc::new(InMemoryBackend::new()))
            .default_backend_key(key)
            .with_actor(Actor::System {
                operation: "e2e_welcome_host".into(),
            })
            .build()
            .expect("e2e Valence"),
    );
    let _ = E2E_VALENCE.set(valence);
    E2E_VALENCE.get().expect("e2e Valence just set").clone()
}

/// Shared System Valence (call [`init_e2e_valence`] at boot first).
pub fn e2e_valence() -> Arc<Valence> {
    E2E_VALENCE
        .get()
        .expect("init_e2e_valence must run before e2e_valence")
        .clone()
}
