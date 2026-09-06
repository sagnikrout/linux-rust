//! Automatically rewritten from C to Rust
//! Source: security/lsm_notifier.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// LSM notifier functions
//

    static BLOCKING_NOTIFIER_HEAD(blocking_lsm_notifier_chain);
#[no_mangle]
pub unsafe extern "C" fn call_blocking_lsm_notifier(event: enum lsm_event, data: *mut c_void) -> c_int {
    int call_blocking_lsm_notifier(enum lsm_event event, void *data)
    {
    return blocking_notifier_call_chain(&blocking_lsm_notifier_chain,
    event, data);
    }
    EXPORT_SYMBOL(call_blocking_lsm_notifier);
#[no_mangle]
pub unsafe extern "C" fn register_blocking_lsm_notifier(nb: *mut notifier_block) -> c_int {
    int register_blocking_lsm_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&blocking_lsm_notifier_chain,
    nb);
    }
    EXPORT_SYMBOL(register_blocking_lsm_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_blocking_lsm_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_blocking_lsm_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&blocking_lsm_notifier_chain,
    nb);
    }
    EXPORT_SYMBOL(unregister_blocking_lsm_notifier);
