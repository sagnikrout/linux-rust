//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip6_checksum.h
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
// Checksumming functions for IPv6
//
// Authors:	Jorge Cwik, <jorge@laser.satlink.net>
// Arnt Gulbrandsen, <agulbra@nvg.unit.no>
// Borrows very liberally from tcp.c and ip.c, see those
// files for more names.
//
// Fixes:
//
// Ralf Baechle			:	generic ipv6 checksum
// <ralf@waldorf-gmbh.de>
//

extern "C" {
    pub fn csum_ipv6_magic(_arg: saddr, _arg: daddr, _arg: len, _arg: IPPROTO_TCP, _arg: base) -> return;
}
extern "C" {
    pub fn csum_ipv6_magic(_arg: saddr, _arg: daddr, _arg: len, _arg: IPPROTO_UDP, _arg: base) -> return;
}
