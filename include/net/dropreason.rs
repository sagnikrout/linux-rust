//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dropreason.h
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
// enum skb_drop_reason_subsys - subsystem tag for (extended) drop reasons
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skb_drop_reason_subsys {
// @SKB_DROP_REASON_SUBSYS_CORE: core drop reasons defined above
    SKB_DROP_REASON_SUBSYS_CORE,

//
// @SKB_DROP_REASON_SUBSYS_MAC80211_UNUSABLE: mac80211 drop reasons
// for unusable frames, see net/mac80211/drop.h
//
    SKB_DROP_REASON_SUBSYS_MAC80211_UNUSABLE,

//
// @SKB_DROP_REASON_SUBSYS_OPENVSWITCH: openvswitch drop reasons,
// see net/openvswitch/drop.h
//
    SKB_DROP_REASON_SUBSYS_OPENVSWITCH,

//
// @SKB_DROP_REASON_SUBSYS_QDISC: TC qdisc drop reasons,
// see include/net/dropreason-qdisc.h
//
    SKB_DROP_REASON_SUBSYS_QDISC,

// @SKB_DROP_REASON_SUBSYS_NUM: number of subsystems defined
    SKB_DROP_REASON_SUBSYS_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drop_reason_list {
    pub reasons: *const *const c_char,
    pub n_reasons: usize,
}

// Note: due to dynamic registrations, access must be under RCU
extern "C" {
    pub fn drop_reasons_unregister_subsys(subsys: skb_drop_reason_subsys);
}
