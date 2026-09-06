//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/icmp.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the ICMP module.
//
// Version:	@(#)icmp.h	1.0.4	05/13/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp_err {
    pub errno: c_int,
    pub fatal:1: c_uint,
}

extern "C" {
    pub fn icmp_ndo_send(skb_in: *mut sk_buff, type: c_int, code: c_int, info: __be32);
}

extern "C" {
    pub fn icmp_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn icmp_err(skb: *mut sk_buff, info: u32) -> c_int;
}
extern "C" {
    pub fn icmp_init() -> c_int;
}
extern "C" {
    pub fn icmp_out_count(net: *mut net, type: c_uchar);
}
extern "C" {
    pub fn icmp_build_probe(skb: *mut sk_buff, icmphdr: *mut icmphdr) -> bool;
}
