//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-mmu.h
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
// Copyright (C) 2013--2024 Intel Corporation
pub const ISYS_MMID: c_int = 1;
pub const PSYS_MMID: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_mmu_info {
    pub dev: *mut device,
    pub l1_pt: *mut u32,
    pub l1_pt_dma: u32,
    pub l2_pts: *mut u32,
    pub dummy_l2_pt: *mut u32,
    pub dummy_l2_pteval: u32,
    pub dummy_page: *mut c_void,
    pub dummy_page_pteval: u32,
    pub aperture_start: dma_addr_t,
    pub aperture_end: dma_addr_t,
    pub pgsize_bitmap: c_ulong,
    pub /: *mut *mut spinlock_t lock; / Serialize access to users,
    pub dmap: *mut ipu6_dma_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_mmu {
    pub node: list_head,
    pub mmu_hw: *mut ipu6_mmu_hw,
    pub nr_mmus: c_uint,
    pub mmid: c_uint,
    pub pgtbl: phys_addr_t,
    pub dev: *mut device,
    pub dmap: *mut ipu6_dma_mapping,
    pub vma_list: list_head,
    pub trash_page: *mut page,
    pub /: *mut *mut dma_addr_t pci_trash_page; / IOVA from PCI DMA services (parent),
    pub /: *mut *mut dma_addr_t iova_trash_page; / IOVA for IPU6 child nodes to use,
    pub ready: bool,
    pub /: *mut *mut spinlock_t ready_lock; / Serialize access to bool ready,
    pub mmu): *mut *mut void (tlb_invalidate)(struct ipu6_mmu,
}

extern "C" {
    pub fn ipu6_mmu_cleanup(mmu: *mut ipu6_mmu);
}
extern "C" {
    pub fn ipu6_mmu_hw_init(mmu: *mut ipu6_mmu) -> c_int;
}
extern "C" {
    pub fn ipu6_mmu_hw_cleanup(mmu: *mut ipu6_mmu);
}
