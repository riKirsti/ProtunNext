//! Platform ownership boundary; packet processing is shared.
#[cfg(unix)]
pub(crate) type TunHandle = std::fs::File;
#[cfg(unix)]
pub(crate) use crate::unix::{boottime, run_owned};
