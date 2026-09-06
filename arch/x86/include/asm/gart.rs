//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/gart.h
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
    pub fn set_up_gart_resume(_arg: u32, _arg: u32);
}
// PTE bits.
pub const GPTE_VALID: c_int = 1;
pub const GPTE_COHERENT: c_int = 2;
// Aperture control register bits.

// GART cache control register bits.

// K8 On-cpu GART registers
pub const AMD64_GARTAPERTURECTL: c_uint = 0x90;
pub const AMD64_GARTAPERTUREBASE: c_uint = 0x94;
pub const AMD64_GARTTABLEBASE: c_uint = 0x98;
pub const AMD64_GARTCACHECTL: c_uint = 0x9c;

extern "C" {
    pub fn early_gart_iommu_check();
}
extern "C" {
    pub fn gart_iommu_init() -> c_int;
}
extern "C" {
    pub fn gart_parse_options(: *mut c_char) -> void __init;
}
extern "C" {
    pub fn gart_iommu_hole_init();
}

pub const gart_iommu_aperture: c_int = 0;
pub const gart_iommu_aperture_allowed: c_int = 0;
pub const gart_iommu_aperture_disabled: c_int = 1;

extern "C" {
    pub fn agp_amd64_init() -> c_int;
}
//
// Don't enable translation but enable GART IO and CPU accesses.
// Also, set DISTLBWALKPRB since GART tables memory is UC.
//
// address of the mappings table
// Enable GART translation for this hammer.
