//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/seg6.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg6_pernet_data {
    pub lock: mutex,
    pub tun_src: *mut in6_addr __rcu,

    pub hmac_infos: rhashtable,

}

extern "C" {
    pub fn seg6_init() -> c_int;
}
extern "C" {
    pub fn seg6_exit();
}

extern "C" {
    pub fn seg6_iptunnel_init() -> c_int;
}
extern "C" {
    pub fn seg6_iptunnel_exit();
}
extern "C" {
    pub fn seg6_local_init() -> c_int;
}
extern "C" {
    pub fn seg6_local_exit();
}

extern "C" {
    pub fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: c_int, reduced: bool) -> bool;
}
extern "C" {
    pub fn seg6_icmp_srh(skb: *mut sk_buff, opt: *mut inet6_skb_parm);
}
extern "C" {
    pub fn seg6_do_srh_inline(skb: *mut sk_buff, osrh: *mut ipv6_sr_hdr) -> c_int;
}
// If the packet which invoked an ICMP error contains an SRH return
// the true destination address from within the SRH, otherwise use the
// destination address in the IP header.
//
