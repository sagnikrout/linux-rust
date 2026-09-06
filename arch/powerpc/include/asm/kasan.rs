//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kasan.h
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

// Macro flag: #define EXPORT_SYMBOL_KASAN(fn)

pub const KASAN_SHADOW_SCALE_SHIFT: c_int = 3;

//
// The shadow ends before the highest accessible address
// because we don't need a shadow for the shadow. Instead:
// c00e000000000000 << 3 + a80e000000000000 = c00fc00000000000
//
pub const KASAN_SHADOW_END: c_uint = 0xc00fc00000000000UL;

//
// The shadow ends before the highest accessible address
// because we don't need a shadow for the shadow.
// But it doesn't hurt to have a shadow for the shadow,
// keep shadow end aligned eases things.
//
pub const KASAN_SHADOW_END: c_uint = 0xc000200000000000UL;

extern "C" {
    pub fn kasan_early_init();
}
extern "C" {
    pub fn kasan_mmu_init();
}
extern "C" {
    pub fn kasan_init();
}
extern "C" {
    pub fn kasan_late_init();
}

extern "C" {
    pub fn kasan_update_early_region(k_start: c_ulong, k_end: c_ulong, pte: pte_t);
}
extern "C" {
    pub fn kasan_init_shadow_page_tables(k_start: c_ulong, k_end: c_ulong) -> c_int;
}
extern "C" {
    pub fn kasan_init_region(start: *mut c_void, size: usize) -> c_int;
}

