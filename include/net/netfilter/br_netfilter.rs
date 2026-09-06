//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/br_netfilter.h
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
    pub fn nf_bridge_update_protocol(skb: *mut sk_buff);
}
extern "C" {
    pub fn nf_bridge_encap_header_len(skb: *const sk_buff) -> c_uint;
}
extern "C" {
    pub fn br_nf_pre_routing_finish_bridge(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn br_validate_ipv6(net: *mut net, skb: *mut sk_buff) -> c_int;
}

