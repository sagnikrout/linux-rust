//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netevent.h
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
//
// Generic netevent notifiers
//
// Authors:
// Tom Tucker              <tom@opengridcomputing.com>
// Steve Wise              <swise@opengridcomputing.com>
//
// Changes:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netevent_redirect {
    pub old: *mut dst_entry,
    pub new: *mut dst_entry,
    pub neigh: *mut neighbour,
    pub daddr: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netevent_notif_type {
    NETEVENT_NEIGH_UPDATE = 1, /* arg is struct neighbour ptr */
    NETEVENT_REDIRECT,	   /* arg is struct netevent_redirect ptr */
    NETEVENT_DELAY_PROBE_TIME_UPDATE, /* arg is struct neigh_parms ptr */
    NETEVENT_IPV4_MPATH_HASH_UPDATE, /* arg is struct net ptr */
    NETEVENT_IPV6_MPATH_HASH_UPDATE, /* arg is struct net ptr */
    NETEVENT_IPV4_FWD_UPDATE_PRIORITY_UPDATE, /* arg is struct net ptr */
}

extern "C" {
    pub fn register_netevent_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_netevent_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn call_netevent_notifiers(val: c_ulong, v: *mut c_void) -> c_int;
}
