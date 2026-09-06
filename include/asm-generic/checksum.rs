//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/checksum.h
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
// computes the checksum of a memory block at buff, length len,
// and adds in "sum" (32-bit)
//
// returns a 32-bit number suitable for feeding into itself
// or csum_tcpudp_magic
//
// this function must be called with even lengths, except
// for the last fragment, which may be odd
//
// it's best to have buff aligned on a 32-bit boundary
//
extern "C" {
    pub fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum;
}

//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.
//
extern "C" {
    pub fn ip_fast_csum(iph: *const c_void, ihl: c_uint) -> __sum16;
}

//
// Fold a partial checksum
//

//
// computes the checksum of the TCP/UDP pseudo-header
// returns a 16-bit checksum, already complemented
//

extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}

//
// this routine is used for miscellaneous IP-like checksums, mainly
// in icmp.c
//
extern "C" {
    pub fn ip_compute_csum(buff: *const c_void, len: c_int) -> __sum16;
}
