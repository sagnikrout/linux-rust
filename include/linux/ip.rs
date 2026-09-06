//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ip.h
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
// Definitions for the IP protocol.
//
// Version:	@(#)ip.h	1.0.2	04/28/93
//
// Authors:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

extern "C" {
    pub fn ntohs(skb_network_header_len(skb: ip_hdr(skb)->tot_len) -) -> return;
}
extern "C" {
    pub fn iph_totlen(_arg: skb, _arg: ip_hdr(skb)) -> return;
}
// IPv4 datagram length is stored into 16bit field (tot_len)
pub const IP_MAX_MTU: c_uint = 0xFFFFU;
