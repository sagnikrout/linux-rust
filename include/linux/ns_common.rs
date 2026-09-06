//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ns_common.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn is_current_namespace(ns: *mut ns_common) -> bool;
}
extern "C" {
    pub fn __ns_common_init(ns: *mut ns_common, ns_type: u32, ops: *const proc_ns_operations, inum: c_int) -> c_int;
}
extern "C" {
    pub fn __ns_common_free(ns: *mut ns_common);
}
extern "C" {
    pub fn ns_owner(ns: *mut ns_common) -> *mut ns_common __must_check;
}

extern "C" {
    pub fn may_see_all_namespaces() -> bool;
}
extern "C" {
    pub fn atomic_read(_arg: &ns->__ns_ref_active) -> return;
}
extern "C" {
    pub fn refcount_read(_arg: &ns->__ns_ref) -> return;
}
extern "C" {
    pub fn refcount_dec_and_lock(_arg: &ns->__ns_ref, _arg: ns_lock) -> return;
}

extern "C" {
    pub fn __ns_ref_active_put(ns: *mut ns_common);
}

extern "C" {
    pub fn __ns_ref_active_get(ns: *mut ns_common);
}

