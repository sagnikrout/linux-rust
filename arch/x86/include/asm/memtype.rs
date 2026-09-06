//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/memtype.h
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

extern "C" {
    pub fn pat_enabled() -> bool;
}
extern "C" {
    pub fn pat_bp_init();
}
extern "C" {
    pub fn pat_cpu_init();
}
extern "C" {
    pub fn memtype_free(start: u64, end: u64) -> c_int;
}
extern "C" {
    pub fn memtype_free_io(start: resource_size_t, end: resource_size_t);
}
extern "C" {
    pub fn pat_pfn_immune_to_uc_mtrr(pfn: c_ulong) -> bool;
}
extern "C" {
    pub fn x86_has_pat_wp() -> bool;
}
extern "C" {
    pub fn pgprot2cachemode(pgprot: pgprot_t) -> page_cache_mode;
}
