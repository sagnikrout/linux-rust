//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cache.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 ARM Ltd.
//

pub const CLIDR_LOUU_SHIFT: c_int = 27;
pub const CLIDR_LOC_SHIFT: c_int = 24;
pub const CLIDR_LOUIS_SHIFT: c_int = 21;

// Ctypen, bits[3(n - 1) + 2 : 3(n - 1)], for n = 1 to 7

// Ttypen, bits [2(n - 1) + 34 : 2(n - 1) + 33], for n = 1 to 7

//
// Memory returned by kmalloc() may be used for DMA, so we must make
// sure that all such allocations are cache aligned. Otherwise,
// unrelated code may cause parts of the buffer to be read into the
// cache before the transfer is done, causing old data to be seen by
// the CPU.
//

pub const ICACHEF_ALIASING: c_int = 0;
//
// Whilst the D-side always behaves as PIPT on AArch64, aliasing is
// permitted in the I-cache.
//
extern "C" {
    pub fn test_bit(_arg: ICACHEF_ALIASING, _arg: &__icache_flags) -> return;
}
extern "C" {
    pub fn SYS_FIELD_GET(_arg: CTR_EL0, _arg: CWG, _arg: read_cpuid_cachetype()) -> return;
}

extern "C" {
    pub fn cache_line_size() -> c_int;
}

// Compress a u64 MPIDR value into 32 bits.
//
// These bits are expected to be RES0. If not, return a value with
// the upper 32 bits set to force the caller to give up on 32 bit
// cache ids.
//

//
// Read the effective value of CTR_EL0.
//
// According to ARM ARM for ARMv8-A (ARM DDI 0487C.a),
// section D10.2.33 "CTR_EL0, Cache Type Register" :
//
// CTR_EL0.IDC reports the data cache clean requirements for
// instruction to data coherence.
//
// 0 - dcache clean to PoU is required unless :
// (CLIDR_EL1.LoC == 0) || (CLIDR_EL1.LoUIS == 0 && CLIDR_EL1.LoUU == 0)
// 1 - dcache clean to PoU is not required for i-to-d coherence.
//
// This routine provides the CTR_EL0 with the IDC field updated to the
// effective state.
//

