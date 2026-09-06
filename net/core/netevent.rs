//! Automatically rewritten from C to Rust
//! Source: net/core/netevent.c
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
// Network event notifiers
//
// Authors:
// Tom Tucker             <tom@opengridcomputing.com>
// Steve Wise             <swise@opengridcomputing.com>
//
// Fixes:
//

    static ATOMIC_NOTIFIER_HEAD(netevent_notif_chain);
//
// register_netevent_notifier - register a netevent notifier block
// @nb: notifier
//
// Register a notifier to be called when a netevent occurs.
// The notifier passed is linked into the kernel structures and must
// not be reused until it has been unregistered. A negative errno code
// is returned on a failure.
//
#[no_mangle]
pub unsafe extern "C" fn register_netevent_notifier(nb: *mut notifier_block) -> c_int {
    int register_netevent_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_register(&netevent_notif_chain, nb);
    }
    EXPORT_SYMBOL_GPL(register_netevent_notifier);
//
// unregister_netevent_notifier - unregister a netevent notifier block
// @nb: notifier
//
// Unregister a notifier previously registered by
// register_neigh_notifier(). The notifier is unlinked into the
// kernel structures and may then be reused. A negative errno code
// is returned on a failure.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_netevent_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_netevent_notifier(struct notifier_block *nb)
    {
    return atomic_notifier_chain_unregister(&netevent_notif_chain, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_netevent_notifier);
//
// call_netevent_notifiers - call all netevent notifier blocks
// @val: value passed unmodified to notifier function
// @v:   pointer passed unmodified to notifier function
//
// Call all neighbour notifier blocks.  Parameters and return value
// are as for notifier_call_chain().
//
#[no_mangle]
pub unsafe extern "C" fn call_netevent_notifiers(val: c_ulong, v: *mut c_void) -> c_int {
    int call_netevent_notifiers(unsigned long val, void *v)
    {
    return atomic_notifier_call_chain(&netevent_notif_chain, val, v);
    }
    EXPORT_SYMBOL_GPL(call_netevent_notifiers);
