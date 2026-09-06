//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/device.h
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
// Arch specific extensions to struct device
//

//
// Arch extensions to struct device.
//
// When adding fields, consider macio_add_one_device in
// drivers/macintosh/macio_asic.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_archdata {
//
// These two used to be a union. However, with the hybrid ops we need
// both so here we store both a DMA offset for direct mappings and
// an iommu_table for remapped DMA.
//
    pub dma_offset: dma_addr_t,

    pub iommu_table_base: *mut iommu_table,

    pub pci_data: *mut pci_dn,

    pub edev: *mut eeh_dev,

    pub fail_iommu: c_int,

    pub iov_data: *mut c_void,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdev_archdata {
    pub dma_mask: u64,
//
// Pointer to nvdimm_pmu structure, to handle the unregistering
// of pmu device
//
    pub priv: *mut c_void,
}
