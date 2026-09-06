//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_notify.c
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


//
// linux/drivers/video/fb_notify.c
//
// Copyright (C) 2006 Antonino Daplas <adaplas@pol.net>
//
// 2001 - Documented with DocBook
// - Brad Douglas <brad@neruo.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

    static BLOCKING_NOTIFIER_HEAD(fb_notifier_list);
//
// fb_register_client - register a client notifier
// @nb: notifier block to callback on events
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fb_register_client(nb: *mut notifier_block) -> c_int {
    int fb_register_client(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&fb_notifier_list, nb);
    }
    EXPORT_SYMBOL(fb_register_client);
//
// fb_unregister_client - unregister a client notifier
// @nb: notifier block to callback on events
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn fb_unregister_client(nb: *mut notifier_block) -> c_int {
    int fb_unregister_client(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&fb_notifier_list, nb);
    }
    EXPORT_SYMBOL(fb_unregister_client);
//
// fb_notifier_call_chain - notify clients of fb_events
// @val: value passed to callback
// @v: pointer passed to callback
//
// Return: The return value of the last notifier function
//
#[no_mangle]
pub unsafe extern "C" fn fb_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int {
    int fb_notifier_call_chain(unsigned long val, void *v)
    {
    return blocking_notifier_call_chain(&fb_notifier_list, val, v);
    }
    EXPORT_SYMBOL_GPL(fb_notifier_call_chain);
