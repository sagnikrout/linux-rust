//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/checksum.h
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

//
// Computes the checksum of a memory block at src, length len,
// and adds in "sum" (32-bit), while copying the block to dst.
// If an access exception occurs on src or dst, it stores -EFAULT
// to *src_err or *dst_err respectively (if that pointer is not
// NULL), and, for an error on src, zeroes the rest of dst.
//
// Like csum_partial, this must be called with even lengths,
// except for the last fragment.
//
extern "C" {
    pub fn csum_partial_copy_generic(src: *const c_void, dst: *mut c_void, len: c_int) -> __wsum;
}
extern "C" {
    pub fn csum_partial_copy_generic()src: *mut (void , _arg: dst, _arg: len) -> return;
}
// Macro flag: #define HAVE_CSUM_COPY_USER
extern "C" {
    pub fn csum_partial_copy_generic(_arg: src, )dst: *mut (void , _arg: len) -> return;
}

//
// turns a 32-bit partial checksum (e.g. from csum_partial) into a
// 1's complement 16-bit checksum.
//
// swap the two 16-bit halves of sum
// if there is a carry from adding the two 16-bit halves,
// it will carry from the lower half into the upper half,
// giving us the correct sum in the upper half.
//

//
// computes the checksum of the TCP/UDP pseudo-header
// returns a 16-bit checksum, already complemented
//
extern "C" {
    pub fn csum_fold(_arg: csum_tcpudp_nofold(saddr, _arg: daddr, _arg: len, _arg: proto, _arg: sum)) -> return;
}
// Macro flag: #define HAVE_ARCH_CSUM_ADD

// Macro flag: #define HAVE_ARCH_CSUM_SHIFT
// rotate sum to align it with a 16b boundary
//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.  ihl is the number
// of 32-bit words and is always >= 5.
//

extern "C" {
    pub fn csum_fold(_arg: ip_fast_csum_nofold(iph, _arg: ihl)) -> return;
}
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
    pub fn __csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum;
}
// (const u16 *)(buff + 4));
// (const u32 *)(buff + 4));
// (const u16 *)(buff + 8));
// (const u32 *)(buff + 8));
// (const u16 *)(buff + 12));
// (const u32 *)(buff + 12));
//
// this routine is used for miscellaneous IP-like checksums, mainly
// in icmp.c
//
extern "C" {
    pub fn csum_fold(_arg: csum_partial(buff, _arg: len, _arg: 0)) -> return;
}

