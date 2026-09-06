//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/conntrack.h
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
//
// Copyright (c) 2015 Nicira, Inc.
//
pub const OVS_CONNTRACK_H: c_int = 1;

extern "C" {
    pub fn ovs_ct_init(: *mut net) -> c_int;
}
extern "C" {
    pub fn ovs_ct_exit_start(net: *mut net);
}
extern "C" {
    pub fn ovs_ct_exit_finish(net: *mut net);
}
extern "C" {
    pub fn ovs_ct_verify(: *mut net, attr: ovs_key_attr) -> bool;
}
extern "C" {
    pub fn ovs_ct_action_to_attr(: *const ovs_conntrack_info, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_ct_clear(skb: *mut sk_buff, key: *mut sw_flow_key) -> c_int;
}
extern "C" {
    pub fn ovs_ct_free_action(a: *const nlattr);
}

// Clear 'ct_orig_proto' to mark the non-existence of original
// direction key fields.
//
pub const CT_SUPPORTED_MASK: c_int = 0;

