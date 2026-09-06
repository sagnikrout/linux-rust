//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/iidc_rdma_idpf.h
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
// Copyright (C) 2025 Intel Corporation.

// struct to be populated by core LAN PCI driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_vport_dev_info {
    pub adev: *mut auxiliary_device,
    pub core_adev: *mut auxiliary_device,
    pub netdev: *mut net_device,
    pub vport_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_vport_auxiliary_dev {
    pub adev: auxiliary_device,
    pub vdev_info: *mut iidc_rdma_vport_dev_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_vport_auxiliary_drv {
    pub adrv: auxiliary_driver,
    pub event): *mut iidc_rdma_event,
}

// struct to be populated by core LAN PCI driver
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iidc_function_type {
    IIDC_FUNCTION_TYPE_PF,
    IIDC_FUNCTION_TYPE_VF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_lan_mapped_mem_region {
    pub region_addr: *mut u8 __iomem,
    pub size: __le64,
    pub start_offset: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_priv_dev_info {
    pub msix_entries: *mut msix_entry,
    pub /: *mut *mut u16 msix_count; / How many vectors are reserved for this device,
    pub ftype: iidc_function_type,
    pub num_memory_regions: __le16,
    pub mapped_mem_regions: *mut iidc_rdma_lan_mapped_mem_region,
}

extern "C" {
    pub fn idpf_idc_vport_dev_ctrl(cdev_info: *mut iidc_rdma_core_dev_info, up: bool) -> c_int;
}
