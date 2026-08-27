//! Product path constants extending upstream shell paths.

pub use orbital_shell::paths::*;

/// Auth routes (owned by lepton-auth; duplicated here for auth-free route guards).
pub const AUTH_SIGNIN: &str = "/auth/signin";
pub const AUTH_SIGNUP: &str = "/auth/signup";
pub const USER_ACCOUNT_SETTINGS: &str = "/user/account-settings";

/// Lepton user settings — appearance preferences page.
pub const USER_APPEARANCE: &str = "/user/appearance";

/// Permission management UI (Wave 4 gauge); stub path for auth-free guards.
pub const PERMISSION_PERMISSIONS: &str = "/permissions";
