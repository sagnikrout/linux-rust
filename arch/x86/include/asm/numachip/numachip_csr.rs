//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/numachip/numachip_csr.h
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Numascale NumaConnect-Specific Header file
//
// Copyright (C) 2011 Numascale AS. All rights reserved.
//
// Send feedback to <support@numascale.com>
//

pub const CSR_NODE_SHIFT: c_int = 16;

pub const CSR_NODE_MASK: c_uint = 0x0fff		/* 4K nodes */;
// 32K CSR space, b15 indicates geo/non-geo
pub const CSR_OFFSET_MASK: c_uint = 0x7fffUL;

//
// Local CSR space starts in global CSR space with "nodeid" = 0xfff0, however
// when using the direct mapping on x86_64, both start and size needs to be
// aligned with PMD_SIZE which is 2M
//
pub const NUMACHIP_LCSR_BASE: c_uint = 0x3ffffe000000ULL;
pub const NUMACHIP_LCSR_LIM: c_uint = 0x3fffffffffffULL;

pub const NUMACHIP_LAPIC_BITS: c_int = 8;
extern "C" {
    pub fn swab32(_arg: readl(lcsr_address(offset))) -> return;
}
//
// On NumaChip2, local CSR space is 16MB and starts at fixed offset below 4G
//
pub const NUMACHIP2_LCSR_BASE: c_uint = 0xf0000000UL;
pub const NUMACHIP2_LCSR_SIZE: c_uint = 0x1000000UL;
pub const NUMACHIP2_APIC_ICR: c_uint = 0x100000;
pub const NUMACHIP2_TIMER_DEADLINE: c_uint = 0x200000;
pub const NUMACHIP2_TIMER_INT: c_uint = 0x200008;
pub const NUMACHIP2_TIMER_NOW: c_uint = 0x200018;
pub const NUMACHIP2_TIMER_RESET: c_uint = 0x200020;
extern "C" {
    pub fn readl(_arg: numachip2_lcsr_address(offset)) -> return;
}
extern "C" {
    pub fn readq(_arg: numachip2_lcsr_address(offset)) -> return;
}
