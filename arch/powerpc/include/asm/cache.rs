//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cache.h
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

// bytes per L1 cache line

pub const L1_CACHE_SHIFT: c_int = 4;
pub const MAX_COPY_PREFETCH: c_int = 1;
pub const IFETCH_ALIGN_SHIFT: c_int = 2;

pub const L1_CACHE_SHIFT: c_int = 6;
pub const MAX_COPY_PREFETCH: c_int = 4;
pub const IFETCH_ALIGN_SHIFT: c_int = 3;

pub const MAX_COPY_PREFETCH: c_int = 4;

pub const L1_CACHE_SHIFT: c_int = 7;

pub const L1_CACHE_SHIFT: c_int = 5;

pub const L1_CACHE_SHIFT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_cache_info {
    pub size: u32,
    pub line_size: u32,
    pub /: *mut *mut u32 block_size; / L1 only,
    pub log_block_size: u32,
    pub blocks_per_page: u32,
    pub sets: u32,
    pub assoc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc64_caches {
    pub l1d: ppc_cache_info,
    pub l1i: ppc_cache_info,
    pub l2: ppc_cache_info,
    pub l3: ppc_cache_info,
}

extern "C" {
    pub fn _get_L2CR() -> c_long;
}
extern "C" {
    pub fn _get_L3CR() -> c_long;
}
extern "C" {
    pub fn _set_L2CR(long: unsigned);
}
extern "C" {
    pub fn _set_L3CR(long: unsigned);
}

extern "C" {
    pub fn __volatile__(0: "dcbz, "memory": %0" : : "r"(addr) :) -> __asm__;
}
extern "C" {
    pub fn __volatile__(0: "dcbi, "memory": %0" : : "r"(addr) :) -> __asm__;
}
extern "C" {
    pub fn __volatile__(0: "dcbf, "memory": %0" : : "r"(addr) :) -> __asm__;
}
extern "C" {
    pub fn __volatile__(0: "dcbst, "memory": %0" : : "r"(addr) :) -> __asm__;
}
extern "C" {
    pub fn volatile(0: "icbi, "memory": %0" : : "r"(addr) :) -> asm;
}
extern "C" {
    pub fn volatile(0: "iccci, "memory": %0" : : "r"(addr) :) -> asm;
}

