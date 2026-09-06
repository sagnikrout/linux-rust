//! Automatically rewritten from C to Rust
//! Source: lib/checksum.c
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
// IP/TCP/UDP checksumming routines
//
// Authors:	Jorge Cwik, <jorge@laser.satlink.net>
// Arnt Gulbrandsen, <agulbra@nvg.unit.no>
// Tom May, <ftom@netcom.com>
// Andreas Schwab, <schwab@issan.informatik.uni-dortmund.de>
// Lots of code moved from tcp.c and ip.c; see those files
// for more names.
//
// 03/02/96	Jes Sorensen, Andreas Schwab, Roman Hodek:
// Fixed some nasty bugs, causing some horrible crashes.
// A: At some points, the sum (%0) was used as
// length-counter instead of the length counter
// (%1). Thanks to Roman Hodek for pointing this out.
// B: GCC seems to mess up if one uses too many
// data-registers to hold input values and one tries to
// specify d0 and d1 as scratch registers. Letting gcc
// choose these registers itself solves the problem.
//
// Revised by Kenneth Albanowski for m68knommu. Basic problem: unaligned access
    kills, so most of the assembly has to go. */

#[no_mangle]
unsafe extern "C" fn do_csum(buff: *const c_uchar, len: c_int) -> c_uint {
    static unsigned int do_csum(const unsigned char *buff, int len)
    {
    int odd;
    let mut result: c_uint = 0;
    if (len <= 0)
    goto out;
    odd = 1 & (unsigned long) buff;
    if (odd) {

    result += (*buff << 8);

    result = *buff;

    len--;
    buff++;
    }
    if (len >= 2) {
    if (2 & (unsigned long) buff) {
    result += *(unsigned short *) buff;
    len -= 2;
    buff += 2;
    }
    if (len >= 4) {
    const unsigned char *end = buff + ((unsigned)len & ~3);
    let mut carry: c_uint = 0;
    do {
    let mut w: c_uint = *(unsigned int *) buff;
    buff += 4;
    result += carry;
    result += w;
    carry = (w > result);
    } while (buff < end);
    result += carry;
    result = (result & 0xffff) + (result >> 16);
    }
    if (len & 2) {
    result += *(unsigned short *) buff;
    buff += 2;
    }
    }
    if (len & 1)

    result += *buff;

    result += (*buff << 8);

    result = csum_from32to16(result);
    if (odd)
    result = ((result >> 8) & 0xff) | ((result & 0xff) << 8);
    out:
    return result;
    }

//
// This is a version of ip_compute_csum() optimized for IP headers,
// which always checksum on 4 octet boundaries.
//
#[no_mangle]
pub unsafe extern "C" fn ip_fast_csum(iph: *const c_void, ihl: c_uint) -> __sum16 {
    __sum16 ip_fast_csum(const void *iph, unsigned int ihl)
    {
    return ( __sum16)~do_csum(iph, ihl*4);
    }
    EXPORT_SYMBOL(ip_fast_csum);

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
#[no_mangle]
pub unsafe extern "C" fn csum_partial(buff: *const c_void, len: c_int, wsum: __wsum) -> __wsum {
    __wsum csum_partial(const void *buff, int len, __wsum wsum)
    {
    let mut sum: c_uint = ( unsigned int)wsum;
    let mut result: c_uint = do_csum(buff, len);
// add in old sum, and carry..
    result += sum;
    if (sum > result)
    result += 1;
    return ( __wsum)result;
    }
    EXPORT_SYMBOL(csum_partial);
//
// this routine is used for miscellaneous IP-like checksums, mainly
// in icmp.c
//
#[no_mangle]
pub unsafe extern "C" fn ip_compute_csum(buff: *const c_void, len: c_int) -> __sum16 {
    __sum16 ip_compute_csum(const void *buff, int len)
    {
    return ( __sum16)~do_csum(buff, len);
    }
    EXPORT_SYMBOL(ip_compute_csum);

#[no_mangle]
pub unsafe extern "C" fn from64to32(x: u64) -> u32 {
    static inline u32 from64to32(u64 x)
    {
// add up 32-bit and 32-bit for 32+c bit
    x = (x & 0xffffffff) + (x >> 32);
// add up carry..
    x = (x & 0xffffffff) + (x >> 32);
    return (u32)x;
    }
    __wsum csum_tcpudp_nofold(__be32 saddr, __be32 daddr,
    __u32 len, __u8 proto, __wsum sum)
    {
    let mut s: c_ulonglong = ( u32)sum;
    s += ( u32)saddr;
    s += ( u32)daddr;

    s += proto + len;

    s += (proto + len) << 8;

    return ( __wsum)from64to32(s);
    }
    EXPORT_SYMBOL(csum_tcpudp_nofold);
