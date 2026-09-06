//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inetpeer.h
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
// INETPEER - A storage for permanent information about peers
//
// Authors:	Andrey V. Savochkin <saw@msu.ru>
//

// IPv4 address key for cache lookups
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_addr_key {
    pub addr: __be32,
    pub vif: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inetpeer_addr {
    pub a4: ipv4_addr_key,
    pub a6: in6_addr,
    pub key: [u32; INETPEER_MAXKEYSZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_peer {
    pub rb_node: rb_node,
    pub hash: u64,
    pub daddr: inetpeer_addr,
    pub metrics: [u32; RTAX_MAX],
    pub /: *mut *mut u32 rate_tokens; / rate limiting for ICMP,
    pub n_redirects: u32,
    pub rate_last: c_ulong,
//
// Once inet_peer is queued for deletion (refcnt == 0), following field
// is not available: rid
// We can share memory with rcu_head to help keep inet_peer small.
//
    pub /: *mut *mut atomic_t rid; / Frag reception counter,
}

// following fields might be frequently dirtied
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_peer_base {
    pub rb_root: rb_root,
    pub lock: seqlock_t,
    pub total: c_int,
}

extern "C" {
    pub fn inet_peer_base_init(: *mut inet_peer_base);
}

// can be called with or without local BH being disabled
extern "C" {
    pub fn inet_getpeer(_arg: base, _arg: &daddr) -> return;
}
extern "C" {
    pub fn inet_getpeer(_arg: base, _arg: &daddr) -> return;
}
// can be called from BH context or outside
extern "C" {
    pub fn inet_putpeer(p: *mut inet_peer);
}
extern "C" {
    pub fn inet_peer_xrlim_allow(peer: *mut inet_peer, timeout: c_int) -> bool;
}
extern "C" {
    pub fn inetpeer_invalidate_tree(: *mut inet_peer_base);
}
