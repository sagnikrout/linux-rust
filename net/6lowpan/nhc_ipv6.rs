//! Automatically rewritten from C to Rust
//! Source: net/6lowpan/nhc_ipv6.c
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
// 6LoWPAN IPv6 Header compression according to RFC6282
//

pub const LOWPAN_NHC_IPV6_ID_0: c_uint = 0xee;
pub const LOWPAN_NHC_IPV6_MASK_0: c_uint = 0xfe;
    LOWPAN_NHC(nhc_ipv6, "RFC6282 IPv6", NEXTHDR_IPV6, 0, LOWPAN_NHC_IPV6_ID_0,
    LOWPAN_NHC_IPV6_MASK_0, core::ptr::null_mut(), core::ptr::null_mut());
    module_lowpan_nhc(nhc_ipv6);
    MODULE_DESCRIPTION("6LoWPAN next header RFC6282 IPv6 compression");
    MODULE_LICENSE("GPL");
