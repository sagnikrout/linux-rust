//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/checksum.h
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
// S390 fast network checksum routines
//
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Ulrich Hild        (first version)
// Martin Schwidefsky (heavily optimized CKSM version)
// D.J. Barrow        (third attempt)
//

extern "C" {
    pub fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum;
}
extern "C" {
    pub fn csum_partial_copy_nocheck(src: *const c_void, dst: *mut c_void, len: c_int) -> __wsum;
}
//
// Fold a partial checksum without adding pseudo headers.
//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksums on 4 octet boundaries.
//
extern "C" {
    pub fn csum_fold(32): ( __wsum)(csum >>) -> return;
}
//
// Computes the checksum of the TCP/UDP pseudo-header.
// Returns a 32-bit checksum.
//
// Computes the checksum of the TCP/UDP pseudo-header.
// Returns a 16-bit checksum, already complemented.
//
extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}
//
// Used for miscellaneous IP-like checksums, mainly icmp.
//
extern "C" {
    pub fn csum_fold(_arg: csum_partial(buff, _arg: len, _arg: 0)) -> return;
}
extern "C" {
    pub fn csum_fold(32): ( __wsum)(sum >>) -> return;
}
