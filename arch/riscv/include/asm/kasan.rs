//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kasan.h
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
// Copyright (C) 2019 Andes Technology Corporation
//
// The following comment was copied from arm64:
// KASAN_SHADOW_START: beginning of the kernel virtual addresses.
// KASAN_SHADOW_END: KASAN_SHADOW_START + 1/N of kernel virtual addresses,
// where N = (1 << KASAN_SHADOW_SCALE_SHIFT).
//
// KASAN_SHADOW_OFFSET:
// This value is used to map an address to the corresponding shadow
// address by the following formula:
// shadow_addr = (address >> KASAN_SHADOW_SCALE_SHIFT) + KASAN_SHADOW_OFFSET
//
// (1 << (64 - KASAN_SHADOW_SCALE_SHIFT)) shadow addresses that lie in range
// [KASAN_SHADOW_OFFSET, KASAN_SHADOW_END) cover all 64-bits of virtual
// addresses. So KASAN_SHADOW_OFFSET should satisfy the following equation:
// KASAN_SHADOW_OFFSET = KASAN_SHADOW_END -
// (1ULL << (64 - KASAN_SHADOW_SCALE_SHIFT))
//
pub const KASAN_SHADOW_SCALE_SHIFT: c_int = 3;

//
// Depending on the size of the virtual address space, the region may not be
// aligned on PGDIR_SIZE, so force its alignment to ease its population.
//

extern "C" {
    pub fn kasan_init();
}
extern "C" {
    pub fn kasan_early_init() -> asmlinkage void;
}
extern "C" {
    pub fn kasan_swapper_init();
}

