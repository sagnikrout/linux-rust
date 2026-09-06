//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/checksum_64.h
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
// Checksums for x86-64
// Copyright 2002 by Andi Kleen, SuSE Labs
// with some code from asm-x86/checksum.h
//

//
// csum_fold - Fold and invert a 32bit checksum.
// @sum: 32bit unfolded sum
//
// Fold a 32bit running checksum to 16bit and invert it. This is usually
// the last step before putting a checksum into a packet.
// Make sure not to mix with 64bit checksums.
//
// Returns: new checksum value
//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.
//
// By Jorge Cwik <jorge@laser.satlink.net>, adapted for linux by
// Arnt Gulbrandsen.
//
// ip_fast_csum - Compute the IPv4 header checksum efficiently.
// @iph: ipv4 header
// @ihl: length of header / 4
//
// Returns: header checksum
//
// Since the input registers which are loaded with iph and ihl
//
// csum_tcpudp_nofold - Compute an IPv4 pseudo header checksum.
// @saddr: source address
// @daddr: destination address
// @len: length of packet
// @proto: ip protocol of packet
// @sum: initial sum to be added in (32bit unfolded)
//
// Returns: the pseudo header checksum the input data. Result is
// 32bit unfolded.
//
// csum_tcpudp_magic - Compute an IPv4 pseudo header checksum.
// @saddr: source address
// @daddr: destination address
// @len: length of packet
// @proto: ip protocol of packet
// @sum: initial sum to be added in (32bit unfolded)
//
// Returns: the 16bit pseudo header checksum the input data already
// complemented and ready to be filled in.
//
extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}
//
// csum_partial - Compute an internet checksum.
// @buff: buffer to be checksummed
// @len: length of buffer.
// @sum: initial sum to be added in (32bit unfolded)
//
// Returns: the 32bit unfolded internet checksum of the buffer.
// Before filling it in it needs to be csum_fold()'ed.
// buff should be aligned to a 64bit boundary if possible.
//
extern "C" {
    pub fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum;
}
// Do not call this directly. Use the wrappers below
extern "C" {
    pub fn csum_partial_copy_generic(src: *const c_void, dst: *mut c_void, len: c_int) -> __visible __wsum;
}
extern "C" {
    pub fn csum_and_copy_from_user(src: *const void __user, dst: *mut c_void, len: c_int) -> __wsum;
}
extern "C" {
    pub fn csum_and_copy_to_user(src: *const c_void, dst: *mut void __user, len: c_int) -> __wsum;
}
extern "C" {
    pub fn csum_partial_copy_nocheck(src: *const c_void, dst: *mut c_void, len: c_int) -> __wsum;
}
//
// ip_compute_csum - Compute an 16bit IP checksum.
// @buff: buffer address.
// @len: length of buffer.
//
// Returns: the 16bit folded/inverted checksum of the passed buffer.
// Ready to fill in.
//
extern "C" {
    pub fn ip_compute_csum(buff: *const c_void, len: c_int) -> __sum16;
}
pub const _HAVE_ARCH_IPV6_CSUM: c_int = 1;
//
// csum_ipv6_magic - Compute checksum of an IPv6 pseudo header.
// @_saddr: source address
// @_daddr: destination address
// @len: length of packet
// @proto: protocol of packet
// @sum: initial sum (32bit unfolded) to be added in
//
// Computes an IPv6 pseudo header checksum. This sum is added the checksum
// into UDP/TCP packets and contains some link layer information.
//
// Returns: the unfolded 32bit checksum.
//
// Macro flag: #define HAVE_ARCH_CSUM_ADD
