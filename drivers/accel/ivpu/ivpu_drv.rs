//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_drv.h
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
// Copyright (C) 2020-2026 Intel Corporation
//

pub const PCI_DEVICE_ID_MTL: c_uint = 0x7d1d;
pub const PCI_DEVICE_ID_ARL: c_uint = 0xad1d;
pub const PCI_DEVICE_ID_LNL: c_uint = 0x643e;
pub const PCI_DEVICE_ID_PTL_P: c_uint = 0xb03e;
pub const PCI_DEVICE_ID_WCL: c_uint = 0xfd3e;
pub const PCI_DEVICE_ID_NVL: c_uint = 0xd71d;
pub const IVPU_HW_IP_37XX: c_int = 37;
pub const IVPU_HW_IP_40XX: c_int = 40;
pub const IVPU_HW_IP_50XX: c_int = 50;
pub const IVPU_HW_IP_60XX: c_int = 60;
pub const IVPU_HW_IP_REV_LNL_B0: c_int = 4;
pub const IVPU_HW_IP_REV_NVL_A0: c_int = 0;
pub const IVPU_HW_BTRS_MTL: c_int = 1;
pub const IVPU_HW_BTRS_LNL: c_int = 2;
pub const IVPU_GLOBAL_CONTEXT_MMU_SSID: c_int = 0;
// SSID 1 is used by the VPU to represent reserved context
pub const IVPU_RESERVED_CONTEXT_MMU_SSID: c_int = 1;
pub const IVPU_USER_CONTEXT_MIN_SSID: c_int = 2;

pub const IVPU_MIN_DB: c_int = 1;
pub const IVPU_MAX_DB: c_int = 255;

pub const IVPU_CMDQ_MIN_ID: c_int = 1;
pub const IVPU_CMDQ_MAX_ID: c_int = 255;
pub const IVPU_PLATFORM_SILICON: c_int = 0;
pub const IVPU_PLATFORM_SIMICS: c_int = 2;
pub const IVPU_PLATFORM_FPGA: c_int = 3;
pub const IVPU_PLATFORM_HSLE: c_int = 4;
pub const IVPU_PLATFORM_INVALID: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_wa_table {
    pub punit_disabled: bool,
    pub clear_runtime_mem: bool,
    pub interrupt_clear_with_0: bool,
    pub disable_clock_relinquish: bool,
    pub wp0_during_power_up: bool,
    pub disable_d0i2: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_user_limits {
    pub hash_node: hlist_node,
    pub vdev: *mut ivpu_device,
    pub ref: kref,
    pub max_ctx_count: u32,
    pub max_db_count: u32,
    pub uid: u32,
    pub db_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_device {
    pub drm: drm_device,
    pub regb: *mut void __iomem,
    pub regv: *mut void __iomem,
    pub platform: u32,
    pub irq: u32,
    pub wa: ivpu_wa_table,
    pub hw: *mut ivpu_hw_info,
    pub mmu: *mut ivpu_mmu_info,
    pub fw: *mut ivpu_fw_info,
    pub ipc: *mut ivpu_ipc_info,
    pub pm: *mut ivpu_pm_info,
    pub gctx: ivpu_mmu_context,
    pub rctx: ivpu_mmu_context,
    pub /: *mut *mut mutex context_list_lock; / Protects user context addition/removal,
    pub context_xa: xarray,
    pub context_xa_limit: xa_limit,
    pub 8): DECLARE_HASHTABLE(user_limits,,
    pub /: *mut *mut mutex user_limits_lock; / Protects user_limits,
    pub db_xa: xarray,
    pub db_limit: xa_limit,
    pub db_next: u32,
    pub irq_dct_work: work_struct,
    pub context_abort_work: work_struct,
    pub job_destroy_list: llist_head,
    pub job_destroy_work: work_struct,
    pub job_destroy_wq: *mut workqueue_struct,
    pub /: *mut *mut mutex bo_list_lock; / Protects bo_list,
    pub bo_list: list_head,
    pub /: *mut *mut mutex submitted_jobs_lock; / Protects submitted_jobs,
    pub submitted_jobs_xa: xarray,
    pub job_done_consumer: ivpu_ipc_consumer,
    pub job_timeout_counter: core::sync::atomic::AtomicI32,
    pub faults_detected: core::sync::atomic::AtomicI32,
    pub unique_id_counter: core::sync::atomic::AtomicI64,
    pub busy_start_ts: ktime_t,
    pub busy_time: ktime_t,
    pub boot: c_int,
    pub jsm: c_int,
    pub tdr: c_int,
    pub inference: c_int,
    pub autosuspend: c_int,
    pub d0i3_entry_msg: c_int,
    pub state_dump_msg: c_int,
    pub timeout: },
}

//
// file_priv has its own refcount (ref) that allows user space to close the fd
// without blocking even if VPU is still processing some jobs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_file_priv {
    pub ref: kref,
    pub vdev: *mut ivpu_device,
    pub /: *mut *mut mutex lock; / Protects cmdq,
    pub cmdq_xa: xarray,
    pub ctx: ivpu_mmu_context,
    pub /: *mut *mut mutex ms_lock; / Protects ms_instance_list, ms_info_bo,
    pub ms_instance_list: list_head,
    pub ms_info_bo: *mut ivpu_bo,
    pub job_limit: xa_limit,
    pub user_limits: *mut ivpu_user_limits,
    pub job_id_next: u32,
    pub cmdq_limit: xa_limit,
    pub cmdq_id_next: u32,
    pub has_mmu_faults: bool,
    pub bound: bool,
    pub aborted: bool,
}

extern "C" {
    pub fn ivpu_file_priv_put(link: *mut ivpu_file_priv);
}
extern "C" {
    pub fn ivpu_boot(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_shutdown(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_prepare_for_reset(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_is_capable(vdev: *mut ivpu_device, capability: u32) -> bool;
}
extern "C" {
    pub fn container_of(_arg: dev, ivpu_device: struct, _arg: drm) -> return;
}
