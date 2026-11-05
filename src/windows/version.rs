//! Check the explicit release contract before creating an adapter or route.
use crate::{Error, Result};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, IsWow64Process2};

#[repr(C)]
struct Version {
    size: u32,
    major: u32,
    minor: u32,
    build: u32,
    platform: u32,
    service_pack: [u16; 128],
}
#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(version: *mut Version) -> i32;
}
fn allowed(major: u32, minor: u32, build: u32, process: u16, native: u16) -> bool {
    major == 10 && minor == 0 && matches!(build, 26200 | 26300) && process == 0 && native == 0x8664
}
pub(super) fn require_supported() -> Result<()> {
    let mut version = Version {
        size: size_of::<Version>() as _,
        major: 0,
        minor: 0,
        build: 0,
        platform: 0,
        service_pack: [0; 128],
    };
    let (mut process, mut native) = (0u16, 0u16);
    // SAFETY: both API calls receive live, correctly sized writable outputs;
    // the pseudo-handle identifies this process and is never closed.
    let version_status = unsafe { RtlGetVersion(&mut version) };
    // SAFETY: live scalar outputs and the current-process pseudo-handle.
    let machine_status = unsafe { IsWow64Process2(GetCurrentProcess(), &mut process, &mut native) };
    if version_status != 0 || machine_status == 0 {
        return Err(Error::Unsupported(
            "cannot verify the Windows release and native architecture",
        ));
    }
    if !allowed(version.major, version.minor, version.build, process, native) {
        return Err(Error::Unsupported(
            "requires Windows 11 25H2 or 26H2 on native x64",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn annual_release_families_do_not_accept_26h1_or_arm_emulation() {
        for build in [26200, 26300] {
            assert!(allowed(10, 0, build, 0, 0x8664));
        }
        for build in [26100, 28000, 30000] {
            assert!(!allowed(10, 0, build, 0, 0x8664));
        }
        assert!(!allowed(10, 0, 26300, 0x8664, 0xaa64));
        assert!(!allowed(10, 0, 26300, 0, 0xaa64));
    }
}
