//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/csum-partial_64.c
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
// arch/x86_64/lib/csum-partial.c
//
// This file contains network checksum routines that are better done
// in an architecture-specific manner due to speed.
//

#[no_mangle]
pub unsafe extern "C" fn csum_finalize_sum(temp64: u64) -> __wsum {
    static inline __wsum csum_finalize_sum(u64 temp64)
    {
    return ( __wsum)((temp64 + ror64(temp64, 32)) >> 32);
    }
#[no_mangle]
pub unsafe extern "C" fn update_csum_40b(sum: c_ulong, m[5]: c_ulong) -> c_ulong {
    static inline unsigned long update_csum_40b(unsigned long sum, const unsigned long m[5])
    {
    asm("addq %1,%0\n\t"
    "adcq %2,%0\n\t"
    "adcq %3,%0\n\t"
    "adcq %4,%0\n\t"
    "adcq %5,%0\n\t"
    "adcq $0,%0"
    :"+r" (sum)
    :"m" (m[0]), "m" (m[1]), "m" (m[2]),
    "m" (m[3]), "m" (m[4]));
    return sum;
    }
//
// Do a checksum on an arbitrary memory area.
// Returns a 32bit checksum.
//
// This isn't as time critical as it used to be because many NICs
// do hardware checksumming these days.
//
// Still, with CHECKSUM_COMPLETE this is called to compute
// checksums on IPv6 headers (40 bytes) and other small parts.
// it's best to have buff aligned on a 64-bit boundary
//
#[no_mangle]
pub unsafe extern "C" fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum {
    __wsum csum_partial(const void *buff, int len, __wsum sum)
    {
    let mut temp64: u64 = ( u64)sum;
// Do two 40-byte chunks in parallel to get better ILP
    if (likely(len >= 80)) {
    let mut temp64_2: u64 = 0;
    do {
    temp64 = update_csum_40b(temp64, buff);
    temp64_2 = update_csum_40b(temp64_2, buff + 40);
    buff += 80;
    len -= 80;
    } while (len >= 80);
    asm("addq %1,%0\n\t"
    "adcq $0,%0"
    :"+r" (temp64): "r" (temp64_2));
    }
//
// len == 40 is the hot case due to IPv6 headers, so return
// early for that exact case without checking the tail bytes.
//
    if (len >= 40) {
    temp64 = update_csum_40b(temp64, buff);
    len -= 40;
    if (!len)
    return csum_finalize_sum(temp64);
    buff += 40;
    }
    if (len & 32) {
    asm("addq 0*8(%[src]),%[res]\n\t"
    "adcq 1*8(%[src]),%[res]\n\t"
    "adcq 2*8(%[src]),%[res]\n\t"
    "adcq 3*8(%[src]),%[res]\n\t"
    "adcq $0,%[res]"
    : [res] "+r"(temp64)
    : [src] "r"(buff), "m"(*(const char(*)[32])buff));
    buff += 32;
    }
    if (len & 16) {
    asm("addq 0*8(%[src]),%[res]\n\t"
    "adcq 1*8(%[src]),%[res]\n\t"
    "adcq $0,%[res]"
    : [res] "+r"(temp64)
    : [src] "r"(buff), "m"(*(const char(*)[16])buff));
    buff += 16;
    }
    if (len & 8) {
    asm("addq 0*8(%[src]),%[res]\n\t"
    "adcq $0,%[res]"
    : [res] "+r"(temp64)
    : [src] "r"(buff), "m"(*(const char(*)[8])buff));
    buff += 8;
    }
    if (len & 7) {
    let mut shift: c_uint = (-len << 3) & 63;
    unsigned long trail;
    trail = (load_unaligned_zeropad(buff) << shift) >> shift;
    asm("addq %[trail],%[res]\n\t"
    "adcq $0,%[res]"
    : [res] "+r"(temp64)
    : [trail] "r"(trail));
    }
    return csum_finalize_sum(temp64);
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
    return csum_fold(csum_partial(buff, len, 0));
    }
    EXPORT_SYMBOL(ip_compute_csum);
