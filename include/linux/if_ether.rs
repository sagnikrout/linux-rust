//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_ether.h
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
// Global definitions for the Ethernet IEEE 802.3 interface.
//
// Version:	@(#)if_ether.h	1.0.1a	02/08/94
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
//

// XX:XX:XX:XX:XX:XX

// Prefer this version in TX path, instead of
// skb_reset_mac_header() + eth_hdr()
//
extern "C" {
    pub fn sysfs_format_mac(buf: *mut c_char, addr: *const c_uchar, len: c_int) -> isize;
}
