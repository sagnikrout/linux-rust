//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/snmp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Definitions for MIBs
//
// Author: Hideaki YOSHIFUJI <yoshfuji@linux-ipv6.org>
//
// ipstats mib definitions
//
// RFC 1213:  MIB-II
// RFC 2011 (updates 1213):  SNMPv2-MIB-IP
// RFC 2863:  Interfaces Group MIB
// RFC 2465:  IPv6 MIB: General Group
// draft-ietf-ipv6-rfc2011-update-10.txt: MIB for IP: IP Statistics Tables
//
// frequently written fields in fast path, kept in same cache line
// other fields
// icmp mib definitions
//
// RFC 1213:  MIB-II ICMP Group
// RFC 2011 (updates 1213):  SNMPv2 MIB for IP: ICMP group
//

// icmp6 mib definitions
//
// RFC 2466:  ICMPv6-MIB
//

// tcp mib definitions
//
// RFC 1213:  MIB-II TCP group
// RFC 2012 (updates 1213):  SNMPv2-MIB-TCP
//
// udp mib definitions
//
// RFC 1213:  MIB-II UDP group
// RFC 2013 (updates 1213):  SNMPv2-MIB-UDP
//
// linux mib definitions
// linux Xfrm mib definitions
// linux TLS mib definitions
