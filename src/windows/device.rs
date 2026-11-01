//! Narrow Wintun C API boundary. No packet objects/Arc clones/allocations are
//! created per read or write; the one ring-to-pool copy preserves driver lifetime.
use crate::{Error, Result};
use std::{ffi::c_void, io, path::PathBuf, sync::Arc};
use windows_sys::{
    Win32::{
        Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_NO_MORE_ITEMS, GetLastError, HANDLE},
        NetworkManagement::{
            IpHelper::{ConvertInterfaceLuidToGuid, ConvertInterfaceLuidToIndex},
            Ndis::NET_LUID_LH,
        },
        System::LibraryLoader::{LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LOAD_LIBRARY_SEARCH_SYSTEM32},
    },
    core::GUID,
};
type Handle = *mut c_void;
struct Api {
    _library: libloading::Library,
    create: unsafe extern "system" fn(*const u16, *const u16, *const GUID) -> Handle,
    close: unsafe extern "system" fn(Handle),
    luid: unsafe extern "system" fn(Handle, *mut NET_LUID_LH),
    start: unsafe extern "system" fn(Handle, u32) -> Handle,
    end: unsafe extern "system" fn(Handle),
    event: unsafe extern "system" fn(Handle) -> HANDLE,
    receive: unsafe extern "system" fn(Handle, *mut u32) -> *mut u8,
    release: unsafe extern "system" fn(Handle, *const u8),
    allocate: unsafe extern "system" fn(Handle, u32) -> *mut u8,
    send: unsafe extern "system" fn(Handle, *const u8),
}
impl Api {
    fn load() -> Result<Arc<Self>> {
        // Never search the working directory or process PATH for a driver DLL.
        // The embedding application's trusted package supplies the signed DLL.
        let path: PathBuf = std::env::current_exe()?
            .parent()
            .ok_or(Error::Invalid("application directory"))?
            .join("wintun.dll");
        // SAFETY: absolute application-package path. Dependencies are restricted
        // to its directory and System32; loading is an explicit connect action.
        let library: libloading::Library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                &path,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        }
        .map_err(|_| Error::Invalid("load trusted application wintun.dll"))?
        .into();
        // SAFETY: official Wintun API symbol names and C signatures. The owning
        // library is retained in Api for every function pointer's full lifetime.
        unsafe {
            macro_rules! symbol {
                ($name:literal) => {
                    *library
                        .get(concat!($name, "\0").as_bytes())
                        .map_err(|_| Error::Invalid("Wintun API version"))?
                };
            }
            Ok(Arc::new(Self {
                create: symbol!("WintunCreateAdapter"),
                close: symbol!("WintunCloseAdapter"),
                luid: symbol!("WintunGetAdapterLUID"),
                start: symbol!("WintunStartSession"),
                end: symbol!("WintunEndSession"),
                event: symbol!("WintunGetReadWaitEvent"),
                receive: symbol!("WintunReceivePacket"),
                release: symbol!("WintunReleaseReceivePacket"),
                allocate: symbol!("WintunAllocateSendPacket"),
                send: symbol!("WintunSendPacket"),
                _library: library,
            }))
        }
    }
}
pub(crate) struct Session {
    api: Arc<Api>,
    adapter: Handle,
    session: Handle,
    pub(crate) luid: NET_LUID_LH,
    pub(crate) index: u32,
    pub(crate) guid: GUID,
}
pub(crate) enum CreateError {
    Library(Error),
    Adapter(Error),
    Index(Error),
    Session(Error),
}
// SAFETY: Wintun's handles are thread independent. Packet I/O is confined to
// the one packet worker; other owners only read IDs/read-event and configure
// the OS interface. Arc ownership keeps session, adapter and DLL alive.
unsafe impl Send for Session {}
// SAFETY: same ownership rule as Send; no API exposes its raw ring pointers.
unsafe impl Sync for Session {}
impl Session {
    pub(crate) fn create(capacity: u32) -> std::result::Result<Arc<Self>, CreateError> {
        if !(0x20000..=0x4000000).contains(&capacity) || !capacity.is_power_of_two() {
            return Err(CreateError::Session(Error::Invalid(
                "Wintun ring capacity must be power of two from 128KiB to 64MiB",
            )));
        }
        let api = Api::load().map_err(CreateError::Library)?;
        let mut random = [0; 8];
        getrandom::fill(&mut random)
            .map_err(|e| CreateError::Adapter(Error::Protocol(e.to_string())))?;
        let name: Vec<u16> = format!("ProtunNext-{:016x}", u64::from_ne_bytes(random))
            .encode_utf16()
            .chain(Some(0))
            .collect();
        let kind: Vec<u16> = "ProtunNext".encode_utf16().chain(Some(0)).collect();
        // SAFETY: live NUL-terminated names, null requests a fresh adapter GUID.
        let adapter = unsafe { (api.create)(name.as_ptr(), kind.as_ptr(), std::ptr::null()) };
        if adapter.is_null() {
            return Err(CreateError::Adapter(io::Error::last_os_error().into()));
        }
        let mut owner = Self {
            api,
            adapter,
            session: std::ptr::null_mut(),
            // SAFETY: plain Windows output structures, initialized before use.
            luid: unsafe { std::mem::zeroed() },
            index: 0,
            // SAFETY: GUID is an integer-only output structure.
            guid: unsafe { std::mem::zeroed() },
        };
        // SAFETY: valid owned adapter and writable LUID/GUID/index outputs.
        unsafe {
            (owner.api.luid)(owner.adapter, &mut owner.luid);
            super::status(ConvertInterfaceLuidToIndex(&owner.luid, &mut owner.index))
                .map_err(|error| CreateError::Index(error.into()))?;
            super::status(ConvertInterfaceLuidToGuid(&owner.luid, &mut owner.guid))
                .map_err(|error| CreateError::Index(error.into()))?;
            owner.session = (owner.api.start)(owner.adapter, capacity);
        }
        if owner.session.is_null() {
            return Err(CreateError::Session(io::Error::last_os_error().into()));
        }
        Ok(Arc::new(owner))
    }
    pub(crate) fn read_event(&self) -> io::Result<HANDLE> {
        // SAFETY: live session; returned handle is borrowed until session end.
        let handle = unsafe { (self.api.event)(self.session) };
        if handle == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(handle)
        }
    }
    pub(crate) fn read(&self, output: &mut [u8]) -> io::Result<usize> {
        let mut length = 0;
        // SAFETY: live session and valid scalar output. Release before returning.
        let packet = unsafe { (self.api.receive)(self.session, &mut length) };
        if packet.is_null() {
            // SAFETY: immediately retrieves the failed receive's thread error.
            let code = unsafe { GetLastError() };
            return Err(if code == ERROR_NO_MORE_ITEMS {
                io::ErrorKind::WouldBlock.into()
            } else {
                io::Error::from_raw_os_error(code as i32)
            });
        }
        let result = if length as usize > output.len() {
            Err(io::ErrorKind::InvalidData.into())
        } else {
            // SAFETY: Wintun provides length readable bytes; distinct live output
            // slice has sufficient capacity. Ring ownership ends only at release.
            unsafe {
                std::ptr::copy_nonoverlapping(packet, output.as_mut_ptr(), length as usize);
            }
            Ok(length as usize)
        };
        // SAFETY: exactly the pointer returned by receive, released exactly once.
        unsafe {
            (self.api.release)(self.session, packet);
        }
        result
    }
    pub(crate) fn write(&self, input: &[u8]) -> io::Result<usize> {
        if input.is_empty() || input.len() > 65535 {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        // SAFETY: live session, bounded Wintun packet length.
        let packet = unsafe { (self.api.allocate)(self.session, input.len() as u32) };
        if packet.is_null() {
            // SAFETY: immediately retrieves the failed allocation's thread error.
            let code = unsafe { GetLastError() };
            return Err(if code == ERROR_BUFFER_OVERFLOW {
                io::ErrorKind::WouldBlock.into()
            } else {
                io::Error::from_raw_os_error(code as i32)
            });
        }
        // SAFETY: allocation provides this many exclusive writable bytes; input
        // is separate pool storage. No fallible branch can abandon an allocation.
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), packet, input.len());
            (self.api.send)(self.session, packet);
        }
        Ok(input.len())
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        // SAFETY: final Arc owner, after packet-device/event bridge shutdown;
        // DLL stays loaded throughout Drop. Only the adapter we created closes.
        unsafe {
            if !self.session.is_null() {
                (self.api.end)(self.session);
            }
            (self.api.close)(self.adapter);
        }
    }
}
