//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/pds/dirty.h
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
pub struct pds_vfio_region {
    pub host_seq: *mut c_ulong,
    pub host_ack: *mut c_ulong,
    pub bmp_bytes: u64,
    pub size: u64,
    pub start: u64,
    pub page_size: u64,
    pub sgl: *mut pds_lm_sg_elem,
    pub sgl_addr: dma_addr_t,
    pub dev_bmp_offset_start_byte: u32,
    pub num_sge: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vfio_dirty {
    pub regions: *mut pds_vfio_region,
    pub num_regions: u8,
    pub is_enabled: bool,
}

extern "C" {
    pub fn pds_vfio_dirty_is_enabled(pds_vfio: *mut pds_vfio_pci_device) -> bool;
}
extern "C" {
    pub fn pds_vfio_dirty_set_enabled(pds_vfio: *mut pds_vfio_pci_device);
}
extern "C" {
    pub fn pds_vfio_dirty_set_disabled(pds_vfio: *mut pds_vfio_pci_device);
}
extern "C" {
    pub fn pds_vfio_dma_logging_stop(vdev: *mut vfio_device) -> c_int;
}
