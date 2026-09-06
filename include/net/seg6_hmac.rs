//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/seg6_hmac.h
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
// SR-IPv6 implementation
//
// Author:
// David Lebrun <david.lebrun@uclouvain.be>
//

pub const SEG6_HMAC_RING_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg6_hmac_info {
    pub node: rhash_head,
    pub rcu: rcu_head,
    pub hmackeyid: u32,
// The raw key, kept only so it can be returned back to userspace
    pub secret: [c_char; SEG6_HMAC_SECRET_LEN],
    pub slen: u8,
    pub alg_id: u8,
// The prepared key, which the calculations actually use
    pub sha1: hmac_sha1_key,
    pub sha256: hmac_sha256_key,
    pub key: },
}

extern "C" {
    pub fn seg6_hmac_info_del(net: *mut net, key: u32) -> c_int;
}
extern "C" {
    pub fn seg6_hmac_validate_skb(skb: *mut sk_buff) -> bool;
}

extern "C" {
    pub fn seg6_hmac_net_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn seg6_hmac_net_exit(net: *mut net);
}

