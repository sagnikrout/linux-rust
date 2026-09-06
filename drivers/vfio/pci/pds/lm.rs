//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/pds/lm.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vfio_lm_file {
    pub filep: *mut file,
    pub /: *mut *mut mutex lock; / protect live migration data file,
    pub /: *mut *mut u64 size; / Size with valid data,
    pub /: *mut *mut u64 alloc_size; / Total allocated size. Always >= len,
    pub /: *mut *mut *mut void page_mem; / memory allocated for pages,
    pub /: *mut *mut *mut *mut page pages; / Backing pages for file,
    pub npages: c_ulonglong,
    pub /: *mut *mut sg_table sg_table; / SG table for backing pages,
    pub /: *mut *mut *mut pds_lm_sg_elem sgl; / DMA mapping,
    pub sgl_addr: dma_addr_t,
    pub num_sge: u16,
    pub /: *mut *mut *mut scatterlist last_offset_sg; / Iterator,
    pub sg_last_entry: c_uint,
    pub last_offset: c_ulong,
    pub disabled: bool,
}

extern "C" {
    pub fn pds_vfio_put_save_file(pds_vfio: *mut pds_vfio_pci_device);
}
extern "C" {
    pub fn pds_vfio_put_restore_file(pds_vfio: *mut pds_vfio_pci_device);
}
