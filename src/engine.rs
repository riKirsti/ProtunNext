//! Bounded workers share packet ownership and policy, without a second receiver.
mod offline;
mod packets;
mod policy;
mod reactor;

pub(crate) use offline::run_offline;
pub(crate) use reactor::run;

use crate::{Error, Result, tun_device::TunDevice};
use mio::{Poll, Token};
const TUN: Token = Token(1);
const NET: Token = Token(2);
const CONTROL: Token = Token(3);
const BUDGET: usize = 64;

fn detach_tun(
    tun: &mut Option<TunDevice>,
    poll: &Poll,
    observer: Option<&crate::connection::Observer>,
    error: Error,
) -> Result<()> {
    let Some(observer) = observer else {
        return Err(error);
    };
    if let Some(old) = tun.take() {
        let _ = old.deregister(poll.registry());
    }
    observer.interface(false, Some(error.to_string()));
    Ok(())
}
