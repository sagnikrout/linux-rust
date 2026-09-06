//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_h323_asn1.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// BER and PER decoding library for H.323 conntrack/NAT module.
//
// Copyright (c) 2006 by Jing Min Zhao <zhaojingmin@users.sourceforge.net>
//
// This library is based on H.225 version 4, H.235 version 2 and H.245
// version 7. It is extremely optimized to decode only the absolutely
// necessary objects in a signal for Linux kernel NAT module use, so don't
// expect it to be a full ASN.1 library.
//
// Features:
//
// 1. Small. The total size of code plus data is less than 20 KB (IA32).
// 2. Fast. Decoding Netmeeting's Setup signal 1 million times on a PIII 866
// takes only 3.9 seconds.
// 3. No memory allocation. It uses a static object. No need to initialize or
// cleanup.
// 4. Thread safe.
// 5. Support embedded architectures that has no misaligned memory access
// support.
//
// Limitations:
//
// 1. At most 30 faststart entries. Actually this is limited by ethernet's MTU.
// If a Setup signal contains more than 30 faststart, the packet size will
// very likely exceed the MTU size, then the TPKT will be fragmented. I
// don't know how to handle this in a Netfilter module. Anybody can help?
// Although I think 30 is enough for most of the cases.
// 2. IPv4 addresses only.
//
// H.323 Types
//

//
// Decode Functions Return Codes
//

//
// Decode Functions
//
extern "C" {
    pub fn DecodeRasMessage(buf: *mut c_uchar, sz: usize, ras: *mut *mut RasMessage) -> c_int;
}
extern "C" {
    pub fn DecodeQ931(buf: *mut c_uchar, sz: usize, q931: *mut *mut Q931) -> c_int;
}
