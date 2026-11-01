//! Unix socket setup; packet I/O and backpressure are shared across platforms.
use crate::{Error, Result};
use std::{
    io,
    net::{SocketAddr, TcpStream},
    os::fd::AsRawFd,
};
pub(crate) fn bind_device(fd: i32, device: &str, ipv6: bool) -> Result<()> {
    if device.is_empty() || device.len() >= libc::IFNAMSIZ || device.as_bytes().contains(&0) {
        return Err(Error::Invalid("uplink device"));
    }
    let device = std::ffi::CString::new(device).map_err(|_| Error::Invalid("uplink device"))?;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let _ = ipv6;
        // SAFETY: live fd and bounded, NUL-terminated interface name.
        if unsafe {
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_BINDTODEVICE,
                device.as_ptr().cast(),
                device.as_bytes_with_nul().len() as libc::socklen_t,
            )
        } < 0
        {
            return Err(io::Error::last_os_error().into());
        }
    }
    #[cfg(target_vendor = "apple")]
    {
        // SAFETY: bounded, NUL-terminated interface name remains valid.
        let index = unsafe { libc::if_nametoindex(device.as_ptr()) };
        if index == 0 {
            return Err(io::Error::last_os_error().into());
        }
        let (level, option) = if ipv6 {
            (libc::IPPROTO_IPV6, libc::IPV6_BOUND_IF)
        } else {
            (libc::IPPROTO_IP, libc::IP_BOUND_IF)
        };
        // SAFETY: live socket, correctly sized read-only interface-index scalar.
        if unsafe {
            libc::setsockopt(
                fd,
                level,
                option,
                (&index as *const u32).cast(),
                size_of::<u32>() as libc::socklen_t,
            )
        } < 0
        {
            return Err(io::Error::last_os_error().into());
        }
    }
    Ok(())
}

/// Protect against recursive routing before connect sends the first SYN.
pub(crate) fn connect_tcp(
    endpoint: SocketAddr,
    device: &str,
    protector: Option<&dyn crate::api::types::SocketProtector>,
) -> Result<crate::network::ConnectingTcp> {
    let socket = socket2::Socket::new(
        socket2::Domain::for_address(endpoint),
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;
    socket.set_nonblocking(true)?;
    let fd = socket.as_raw_fd();
    if !device.is_empty() {
        bind_device(fd, device, endpoint.is_ipv6())?;
    }
    if let Some(protector) = protector
        && !protector.protect_socket(fd)
    {
        return Err(io::Error::from(io::ErrorKind::PermissionDenied).into());
    }
    match socket.connect(&endpoint.into()) {
        Ok(()) => {}
        Err(error) if error.raw_os_error() == Some(libc::EINPROGRESS) => {}
        Err(error) => return Err(error.into()),
    }
    let socket: TcpStream = socket.into();
    socket.set_nodelay(true)?;
    Ok(crate::network::ConnectingTcp::new(socket))
}
