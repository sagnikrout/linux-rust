//! Automatically rewritten from C Header to Rust Module
//! Source: net/psp/psp.h
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


// SPDX-License-Identifier: GPL-2.0-only

extern "C" {
    pub fn psp_dev_free(psd: *mut psp_dev);
}
extern "C" {
    pub fn psp_dev_check_access(psd: *mut psp_dev, net: *mut net, admin: bool) -> c_int;
}
extern "C" {
    pub fn psp_has_assoc_dev_in_ns(psd: *mut psp_dev, net: *mut net) -> bool;
}
extern "C" {
    pub fn psp_attach_netdev_notifier() -> c_int;
}
extern "C" {
    pub fn psp_nl_notify_dev(psd: *mut psp_dev, cmd: u32);
}
extern "C" {
    pub fn psp_dev_tx_key_del(psd: *mut psp_dev, pas: *mut psp_assoc);
}
extern "C" {
    pub fn psp_assocs_key_rotated(psd: *mut psp_dev);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &psd->refcnt) -> return;
}
