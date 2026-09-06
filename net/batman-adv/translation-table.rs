//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/translation-table.h
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
// Copyright (C) B.A.T.M.A.N. contributors:
//
// Marek Lindner, Simon Wunderlich, Antonio Quartulli
//

extern "C" {
    pub fn batadv_tt_init(bat_priv: *mut batadv_priv) -> c_int;
}
extern "C" {
    pub fn batadv_tt_local_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn batadv_tt_global_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn batadv_tt_global_entry_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_tt_free(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_tt_local_commit_changes(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_tt_local_resize_to_mtu(mesh_iface: *mut net_device);
}
extern "C" {
    pub fn batadv_tt_cache_init() -> c_int;
}
extern "C" {
    pub fn batadv_tt_cache_destroy();
}
//
// batadv_tt_global_entry_put() - decrement the tt_global_entry refcounter and
// possibly release it
// @tt_global_entry: tt_global_entry to be free'd
//
