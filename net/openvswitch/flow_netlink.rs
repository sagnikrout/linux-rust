//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/flow_netlink.h
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
// Copyright (c) 2007-2013 Nicira, Inc.
//
pub const FLOW_NETLINK_H: c_int = 1;

extern "C" {
    pub fn ovs_tun_key_attr_size() -> usize;
}
extern "C" {
    pub fn ovs_key_attr_size() -> usize;
}
extern "C" {
    pub fn ovs_nla_put_identifier(flow: *const sw_flow, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_nla_put_masked_key(flow: *const sw_flow, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_nla_put_mask(flow: *const sw_flow, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_nla_get_ufid(: *mut sw_flow_id, : *const nlattr, log: bool) -> bool;
}
extern "C" {
    pub fn ovs_nla_get_ufid_flags(attr: *const nlattr) -> u32;
}
extern "C" {
    pub fn ovs_nla_free_flow_actions(: *mut sw_flow_actions);
}
extern "C" {
    pub fn ovs_nla_free_flow_actions_rcu(: *mut sw_flow_actions);
}
