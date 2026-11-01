//! Platform ownership boundary; packet processing is shared.
pub(crate) fn send_pressure(error: &std::io::Error) -> bool {
    #[cfg(unix)]
    {
        error.raw_os_error() == Some(libc::ENOBUFS)
    }
    #[cfg(windows)]
    {
        error.raw_os_error() == Some(windows_sys::Win32::Networking::WinSock::WSAENOBUFS)
    }
}
#[cfg(unix)]
pub(crate) type TunHandle = std::fs::File;
#[cfg(unix)]
pub(crate) use crate::unix::{boottime, run_owned};
#[cfg(unix)]
pub(crate) fn clone_tun(tun: &TunHandle) -> crate::Result<TunHandle> {
    Ok(tun.try_clone()?)
}
#[cfg(windows)]
pub(crate) type TunHandle = std::sync::Arc<crate::windows::Interface>;
#[cfg(windows)]
pub(crate) use crate::windows::{boottime, run_owned};
#[cfg(windows)]
pub(crate) fn clone_tun(tun: &TunHandle) -> crate::Result<TunHandle> {
    Ok(tun.clone())
}
