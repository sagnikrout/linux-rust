//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/um/asm/checksum.h
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
// Do not call this directly. Declared for export type visibility.
extern "C" {
    pub fn csum_partial_copy_generic(src: *const c_void, dst: *mut c_void, len: c_int) -> __visible __wsum;
}
//
// csum_fold - Fold and invert a 32bit checksum.
// sum: 32bit unfolded sum
//
// Fold a 32bit running checksum to 16bit and invert it. This is usually
// the last step before putting a checksum into a packet.
// Make sure not to mix with 64bit checksums.
//
// csum_tcpup_nofold - Compute an IPv4 pseudo header checksum.
// @saddr: source address
// @daddr: destination address
// @len: length of packet
// @proto: ip protocol of packet
// @sum: initial sum to be added in (32bit unfolded)
//
// Returns the pseudo header checksum the input data. Result is
// 32bit unfolded.
//
// computes the checksum of the TCP/UDP pseudo-header
// returns a 16-bit checksum, already complemented
//
extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}
//
// ip_fast_csum - Compute the IPv4 header checksum efficiently.
// iph: ipv4 header
// ihl: length of header / 4
//
// Since the input registers which are loaded with iph and ipl

