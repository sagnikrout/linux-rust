//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/checksum_32.h
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
    pub fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> asmlinkage __wsum;
}
//
// the same as csum_partial, but copies from src while it
// checksums, and handles user-space pointer exceptions correctly, when needed.
//
// here even more important to align src and dst on a 32-bit (or even
// better 64-bit) boundary
//
extern "C" {
    pub fn csum_partial_copy_generic(src: *const c_void, dst: *mut c_void, len: c_int) -> asmlinkage __wsum;
}
//
// Note: when you get a NULL pointer exception here this means someone
// passed in an incorrect kernel address to one of these functions.
//
// If you use these functions directly please don't forget the
// access_ok().
//
extern "C" {
    pub fn csum_partial_copy_generic(_arg: src, _arg: dst, _arg: len) -> return;
}
//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.
//
// By Jorge Cwik <jorge@laser.satlink.net>, adapted for linux by
// Arnt Gulbrandsen.
//
// Since the input registers which are loaded with iph and ihl
//
// Fold a partial checksum
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
    pub fn csum_fold(_arg: csum_partial(buff, _arg: len, _arg: 0)) -> return;
}
extern "C" {
    pub fn csum_fold(_arg: sum) -> return;
}
//
// Copy and checksum to user
//
