
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: GPL-2.0

//! Helper crate for KASAN testing.
//!
//! Provides behavior to check the sanitization of Rust code.

use core::ptr::addr_of_mut;
use kernel::prelude::*;

/// Trivial UAF - allocate a big vector, grab a pointer partway through,
/// drop the vector, and touch it.
#[no_mangle]
pub extern "C" fn kasan_test_rust_uaf() -> u8 {
    let mut v: KVec<u8> = KVec::new();
    for _ in 0..4096 {
        v.push(0x42, GFP_KERNEL).unwrap();
    }
    let ptr: *mut u8 = addr_of_mut!(v[2048]);
    drop(v);
    // SAFETY: Incorrect, on purpose.
    unsafe { *ptr }
}
