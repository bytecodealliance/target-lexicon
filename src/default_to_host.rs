use crate::Triple;

/// A simple wrapper around `Triple` that provides an implementation of
/// `Default` which defaults to `Triple::host()`.
pub struct DefaultToHost(pub Triple);

impl Default for DefaultToHost {
    fn default() -> Self {
        Self(Triple::host())
    }
}
