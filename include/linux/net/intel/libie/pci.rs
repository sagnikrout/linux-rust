//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/libie/pci.h
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
// Copyright (C) 2025 Intel Corporation

//
// struct libie_pci_mmio_region - structure for MMIO region info
// @list: used to add a MMIO region to the list of MMIO regions in
// libie_mmio_info
// @addr: virtual address of MMIO region start
// @offset: start offset of the MMIO region
// @size: size of the MMIO region
// @bar_idx: BAR index to which the MMIO region belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_pci_mmio_region {
    pub list: list_head,
    pub addr: *mut void __iomem,
    pub offset: resource_size_t,
    pub size: resource_size_t,
    pub bar_idx: u16,
}

//
// struct libie_mmio_info - contains list of MMIO regions
// @pdev: PCI device pointer
// @mmio_list: list of MMIO regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libie_mmio_info {
    pub pdev: *mut pci_dev,
    pub mmio_list: list_head,
}

extern "C" {
    pub fn libie_pci_unmap_all_mmio_regions(mmio_info: *mut libie_mmio_info);
}
extern "C" {
    pub fn libie_pci_init_dev(pdev: *mut pci_dev) -> c_int;
}
