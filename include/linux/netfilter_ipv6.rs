//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter_ipv6.h
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


// IPv6-specific defines for netfilter.
// (C)1998 Rusty Russell -- This code is GPL.
// (C)1999 David Jeffery
// this header was blatantly ripped from netfilter_ipv4.h
// it's amazing what adding a bunch of 6s can do =8^)
//

// Check for an extension
// Extra routing may needed on local out, as the QUEUE target never returns
// control to the table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_rt_info {
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub mark: u_int32_t,
}

extern "C" {
    pub fn ipv6_chk_addr(_arg: net, _arg: addr, _arg: dev, _arg: strict) -> return;
}

extern "C" {
    pub fn __nf_ip6_route(_arg: net, _arg: dst, _arg: fl, _arg: strict) -> return;
}

extern "C" {
    pub fn br_ip6_fragment(_arg: net, _arg: sk, _arg: skb, _arg: data, _arg: output) -> return;
}

extern "C" {
    pub fn ip6_route_me_harder(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn ip6_route_me_harder(_arg: net, _arg: sk, _arg: skb) -> return;
}

extern "C" {
    pub fn __cookie_v6_init_sequence(_arg: iph, _arg: th, _arg: mssp) -> return;
}

extern "C" {
    pub fn __cookie_v6_check(_arg: iph, _arg: th) -> return;
}

extern "C" {
    pub fn nf_ip6_check_hbh_len(skb: *mut sk_buff, plen: *mut u32) -> c_int;
}

