//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/amdxdna_pci_drv.h
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
//
// Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
//

//
// struct amdxdna_dev_ops - Device hardware operation callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_ops {
    pub xdna): *mut *mut int (init)(struct amdxdna_dev,
    pub xdna): *mut *mut void (fini)(struct amdxdna_dev,
    pub xdna): *mut *mut int (resume)(struct amdxdna_dev,
    pub xdna): *mut *mut int (suspend)(struct amdxdna_dev,
    pub num_vfs): *mut *mut *mut int (sriov_configure)(struct amdxdna_dev xdna, int,
    pub vma): *mut *mut *mut int (mmap)(struct amdxdna_client client, struct vm_area_struct,
    pub hwctx): *mut *mut int (hwctx_init)(struct amdxdna_hwctx,
    pub hwctx): *mut *mut void (hwctx_fini)(struct amdxdna_hwctx,
    pub size): *mut *mut *mut *mut int (hwctx_config)(struct amdxdna_hwctx hwctx, u32 type, u64 value, void buf, u32,
    pub debug_bo_hdl): *mut *mut *mut int (hwctx_sync_debug_bo)(struct amdxdna_hwctx hwctx, u32,
    pub heap): *mut *mut *mut int (hwctx_heap_expand)(struct amdxdna_hwctx hwctx, struct amdxdna_gem_obj,
    pub cur_seq): *mut *mut *mut void (hmm_invalidate)(struct amdxdna_gem_obj abo, unsigned long,
    pub seq): *mut *mut *mut *mut int (cmd_submit)(struct amdxdna_hwctx hwctx, struct amdxdna_sched_job job, u64,
    pub timeout): *mut *mut *mut int (cmd_wait)(struct amdxdna_hwctx hwctx, u64 seq, u32,
    pub args): *mut *mut *mut int (get_aie_info)(struct amdxdna_client client, struct amdxdna_drm_get_info,
    pub args): *mut *mut *mut int (set_aie_state)(struct amdxdna_client client, struct amdxdna_drm_set_state,
    pub args): *mut *mut *mut int (get_array)(struct amdxdna_client client, struct amdxdna_drm_get_array,
    pub rev): *mut *mut *mut int (get_dev_revision)(struct amdxdna_dev xdna, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_fw_feature_tbl {
    pub features: u64,
    pub major: u32,
    pub max_minor: u32,
    pub min_minor: u32,
}

//
// struct amdxdna_dev_info - Device hardware information
// Record device static information, like reg, mbox, PSP, SMU bar index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_info {
    pub reg_bar: c_int,
    pub mbox_bar: c_int,
    pub sram_bar: c_int,
    pub psp_bar: c_int,
    pub smu_bar: c_int,
    pub doorbell_bar: c_int,
    pub device_type: c_int,
    pub first_col: c_int,
    pub dev_mem_buf_shift: u32,
    pub dev_mem_base: u64,
    pub dev_mem_size: usize,
    pub default_vbnv: *const c_char,
    pub rev_vbnv_tbl: *const amdxdna_rev_vbnv,
    pub dev_heap_max_size: usize,
    pub dev_priv: *const amdxdna_dev_priv,
    pub fw_feature_tbl: *const amdxdna_fw_feature_tbl,
    pub ops: *const amdxdna_dev_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_fw_ver {
    pub major: u32,
    pub minor: u32,
    pub sub: u32,
    pub build: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev {
    pub ddev: drm_device,
    pub dev_handle: *mut amdxdna_dev_hdl,
    pub dev_info: *const amdxdna_dev_info,
    pub xrs_hdl: *mut c_void,
    pub /: *mut *mut mutex dev_lock; / per device lock,
    pub client_list: list_head,
    pub /: *mut *mut mutex client_lock; / client_list,
    pub fw_ver: amdxdna_fw_ver,
    pub notifier*/: *mut *mut rw_semaphore notifier_lock; / for mmu,
    pub notifier_wq: *mut workqueue_struct,
    pub group: *mut iommu_group,
    pub domain: *mut iommu_domain,
    pub iovad: iova_domain,
// Accurate board name queried from firmware, or default_vbnv as fallback
    pub vbnv: *const c_char,
    pub carveout: *mut amdxdna_carveout,
}

//
// struct amdxdna_device_id - PCI device info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_device_id {
    pub device: c_ushort,
    pub revision: u8,
    pub dev_info: *const amdxdna_dev_info,
}

//
// struct amdxdna_client - amdxdna client
// A per fd data structure for managing context and other user process stuffs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_client {
    pub node: list_head,
    pub pid: pid_t,
    pub hwctx_srcu: srcu_struct,
    pub hwctx_xa: xarray,
    pub next_hwctxid: u32,
    pub xdna: *mut amdxdna_dev,
    pub filp: *mut drm_file,
    pub /: *mut *mut mutex mm_lock; / protect memory related,
    pub dev_heap_xa: xarray,
    pub dev_heap_mm: drm_mm,
    pub dev_heap_nid: u32,
    pub total_heap_size: usize,
    pub sva: *mut iommu_sva,
    pub pasid: c_int,
    pub mm: *mut mm_struct,
    pub heap_usage: usize,
    pub total_bo_usage: usize,
    pub total_int_bo_usage: usize,
}

// Add device info below
extern "C" {
    pub fn amdxdna_sysfs_init(xdna: *mut amdxdna_dev) -> c_int;
}
extern "C" {
    pub fn amdxdna_sysfs_fini(xdna: *mut amdxdna_dev);
}
extern "C" {
    pub fn amdxdna_iommu_init(xdna: *mut amdxdna_dev) -> c_int;
}
extern "C" {
    pub fn amdxdna_iommu_fini(xdna: *mut amdxdna_dev);
}
extern "C" {
    pub fn amdxdna_dma_map_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) -> c_int;
}
extern "C" {
    pub fn amdxdna_dma_unmap_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj);
}
