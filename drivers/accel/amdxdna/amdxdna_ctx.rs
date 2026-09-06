//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/amdxdna_ctx.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ert_cmd_opcode {
    ERT_START_CU = 0,
    ERT_START_DPU = 18,
    ERT_CMD_CHAIN = 19,
    ERT_START_NPU = 20,
    ERT_START_NPU_PREEMPT = 21,
    ERT_START_NPU_PREEMPT_ELF = 22,
    ERT_INVALID_CMD	= ~0U,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ert_cmd_state {
    ERT_CMD_STATE_INVALID,
    ERT_CMD_STATE_NEW,
    ERT_CMD_STATE_QUEUED,
    ERT_CMD_STATE_RUNNING,
    ERT_CMD_STATE_COMPLETED,
    ERT_CMD_STATE_ERROR,
    ERT_CMD_STATE_ABORT,
    ERT_CMD_STATE_SUBMITTED,
    ERT_CMD_STATE_TIMEOUT,
    ERT_CMD_STATE_NORESPONSE,
}

//
// Interpretation of the beginning of data payload for ERT_START_NPU in
// amdxdna_cmd. The rest of the payload in amdxdna_cmd is regular kernel args.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cmd_start_npu {
    pub /: *mut *mut u64 buffer; / instruction buffer address,
    pub /: *mut *mut u32 buffer_size; / size of buffer in bytes,
    pub /: *mut *mut u32 prop_count; / properties count,
    pub /: *mut *mut u32 prop_args[]; / properties and regular kernel arguments,
}

//
// Interpretation of the beginning of data payload for ERT_CMD_CHAIN in
// amdxdna_cmd. The rest of the payload in amdxdna_cmd is cmd BO handles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cmd_chain {
    pub command_count: u32,
    pub submit_index: u32,
    pub error_index: u32,
    pub reserved: [u32; 3],
    pub data: [u64; ],
}

//
// Interpretation of the beginning of data payload for ERT_START_NPU_PREEMPT in
// amdxdna_cmd. The rest of the payload in amdxdna_cmd is regular kernel args.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cmd_preempt_data {
    pub /: *mut *mut u64 inst_buf; / instruction buffer address,
    pub /: *mut *mut u64 save_buf; / save buffer address,
    pub /: *mut *mut u64 restore_buf; / restore buffer address,
    pub /: *mut *mut u32 inst_size; / size of instruction buffer in bytes,
    pub /: *mut *mut u32 save_size; / size of save buffer in bytes,
    pub /: *mut *mut u32 restore_size; / size of restore buffer in bytes,
    pub /: *mut *mut u32 inst_prop_cnt; / properties count,
    pub /: *mut *mut u32 prop_args[]; / properties and regular kernel arguments,
}

pub const AMDXDNA_CMD_CTX_HEALTH_V1: c_int = 1;
pub const AMDXDNA_CMD_CTX_HEALTH_AIE2: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_ctx_health {
    pub version: u32,
    pub npu_gen: u32,
}

// Exec buffer command header format

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_cmd {
    pub header: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_hwctx {
    pub client: *mut amdxdna_client,
    pub priv: *mut amdxdna_hwctx_priv,
    pub name: *mut c_char,
    pub id: u32,
    pub max_opc: u32,
    pub num_tiles: u32,
    pub mem_size: u32,
    pub fw_ctx_id: u32,
    pub col_list_len: u32,
    pub col_list: *mut u32,
    pub start_col: u32,
    pub num_col: u32,
    pub umq_bo_hdl: u32,
    pub doorbell_offset: u32,
    pub num_unused_col: u32,
    pub last_attached_heap: u32,
    pub qos: amdxdna_qos_info,
    pub cus: *mut amdxdna_hwctx_param_config_cu,
    pub syncobj_hdl: u32,
    pub job_submit_cnt: core::sync::atomic::AtomicI64,
    pub ____cacheline_aligned_in_smp: atomic64_t job_free_cnt,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_job_opcode {
    DEFAULT_IO,
    SYNC_DEBUG_BO,
    ATTACH_DEBUG_BO,
    DETACH_DEBUG_BO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_drv_cmd {
    pub opcode: amdxdna_job_opcode,
    pub result: u32,
    pub refcnt: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amdxdna_job_priv {
    pub aie2_health: *mut app_health_report,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_sched_job {
    pub base: drm_sched_job,
    pub refcnt: kref,
    pub hwctx: *mut amdxdna_hwctx,
    pub mm: *mut mm_struct,
// The fence to notice DRM scheduler that job is done by hardware
    pub fence: *mut dma_fence,
// user can wait on this fence
    pub out_fence: *mut dma_fence,
    pub job_done: bool,
    pub job_timeout: bool,
    pub seq: u64,
    pub drv_cmd: *mut amdxdna_drv_cmd,
    pub cmd_bo: *mut amdxdna_gem_obj,
    pub priv: amdxdna_job_priv,
    pub bo_cnt: usize,
    pub __counted_by(bo_cnt): *mut *mut drm_gem_object bos[],
}

extern "C" {
    pub fn FIELD_GET(_arg: AMDXDNA_CMD_OPCODE, _arg: cmd->header) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: AMDXDNA_CMD_STATE, _arg: cmd->header) -> return;
}
extern "C" {
    pub fn amdxdna_cmd_get_cu_idx(abo: *mut amdxdna_gem_obj) -> u32;
}
extern "C" {
    pub fn amdxdna_sched_job_cleanup(job: *mut amdxdna_sched_job);
}
extern "C" {
    pub fn amdxdna_hwctx_remove_all(client: *mut amdxdna_client);
}
extern "C" {
    pub fn amdxdna_hwctx_sync_debug_bo(client: *mut amdxdna_client, debug_bo_hdl: u32) -> c_int;
}
extern "C" {
    pub fn amdxdna_update_heap(client: *mut amdxdna_client, hwctx: *mut amdxdna_hwctx) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_create_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_config_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_destroy_hwctx_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_submit_cmd_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_wait_cmd_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
