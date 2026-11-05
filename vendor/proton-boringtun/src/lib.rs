// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

//! Simple implementation of the client-side of the WireGuard protocol.
//!
//! The crate is pure: it performs no I/O, reads no clock, draws no OS entropy and ships no lock.
//! The caller provides them, see [`noise::Tunn`]:
//! - the monotonic time `now` (`proton_os_interface::time::Instant`) at creation and on every
//!   [`noise::Tunn::update_timers`], the only call that advances time,
//! - the wall-clock time `now_system` (`proton_os_interface::time::SystemTime`) at creation,
//! - a cryptographically secure RNG (`rand::CryptoRng`),
//! - a lock factory (`proton_os_interface::lock::LockFactory`).

pub mod noise;

// Re-exported so callers use the exact versions that appear in the API (Instant, LockFactory,
// CryptoRng): a version skew gives confusing trait-mismatch errors.
pub use proton_os_interface;
pub use rand;

/// Re-export of the x25519 types
pub mod x25519 {
    pub use x25519_dalek::{
        EphemeralSecret, PublicKey, ReusableSecret, SharedSecret, StaticSecret,
    };
}
