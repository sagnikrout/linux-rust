//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pnv-ocxl.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

pub const PNV_OCXL_TL_MAX_TEMPLATE: c_int = 63;
pub const PNV_OCXL_TL_BITS_PER_RATE: c_int = 4;

pub const PNV_OCXL_ATSD_TIMEOUT: c_int = 1;
// TLB Management Instructions
pub const PNV_OCXL_ATSD_LNCH: c_uint = 0x00;
// Radix Invalidate

// Radix Invalidation Control
// 0b00 Just invalidate TLB.
// 0b01 Invalidate just Page Walk Cache.
// 0b10 Invalidate TLB, Page Walk Cache, and any
// caching of Partition and Process Table Entries.
//

// Number and Page Size of translations to be invalidated

// Invalidation Criteria
// 0b00 Invalidate just the target VA.
// 0b01 Invalidate matching PID.
//

// 0b1: Process Scope, 0b0: Partition Scope

// Invalidation Flag

// Actual Page Size to be invalidated
// 000 4KB
// 101 64KB
// 001 2MB
// 010 1GB
//

// Defines the large page select
// L=0b0 for 4KB pages
// L=0b1 for large pages)
//

// Process ID

// NoFlush – Assumed to be 0b0

pub const PNV_OCXL_ATSD_AVA: c_uint = 0x08;

pub const PNV_OCXL_ATSD_STAT: c_uint = 0x10;
extern "C" {
    pub fn pnv_ocxl_get_actag(dev: *mut pci_dev, base: *mut u16, enabled: *mut u16, supported: *mut u16) -> c_int;
}
extern "C" {
    pub fn pnv_ocxl_get_pasid_count(dev: *mut pci_dev, count: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pnv_ocxl_get_xsl_irq(dev: *mut pci_dev, hwirq: *mut c_int) -> c_int;
}
extern "C" {
    pub fn pnv_ocxl_spa_setup(dev: *mut pci_dev, spa_mem: *mut c_void, PE_mask: c_int, platform_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pnv_ocxl_spa_release(platform_data: *mut c_void);
}
extern "C" {
    pub fn pnv_ocxl_spa_remove_pe_from_cache(platform_data: *mut c_void, pe_handle: c_int) -> c_int;
}
extern "C" {
    pub fn pnv_ocxl_unmap_lpar(arva: *mut void __iomem);
}
