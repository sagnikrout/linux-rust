//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie2_pci.h
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
// Copyright (C) 2023-2026, Advanced Micro Devices, Inc.
//

// Firmware determines device memory base address and size
pub const AIE2_DEVM_BASE: c_uint = 0x4000000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie2_sram_reg_idx {
    MBOX_CHANN_OFF = 0,
    FW_ALIVE_OFF,
    SRAM_MAX_INDEX /* Keep this at the end */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_config_category {
    AIE2_RT_CFG_INIT,
    AIE2_RT_CFG_CLK_GATING,
    AIE2_RT_CFG_FORCE_PREEMPT,
    AIE2_RT_CFG_FRAME_BOUNDARY_PREEMPT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_config {
    pub type: u32,
    pub value: u32,
    pub category: u32,
    pub feature_mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpm_clk_freq {
    pub npuclk: u32,
    pub hclk: u32,
}

//
// Define the maximum number of pending commands in a hardware context.
// Must be power of 2!
//
pub const HWCTX_MAX_CMDS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_hwctx_priv {
    pub heap: *mut amdxdna_gem_obj,
    pub mbox_chann: *mut c_void,
    pub sched: drm_gpu_scheduler,
    pub entity: drm_sched_entity,
    pub /: *mut *mut mutex io_lock; / protect seq and cmd order,
    pub job_free_wq: wait_queue_head,
    pub num_pending: u32,
    pub seq: u64,
    pub job_sem: semaphore,
    pub job_done: bool,
// Completed job counter
    pub completed: u64,
    pub cmd_buf: [*mut amdxdna_gem_obj; HWCTX_MAX_CMDS],
    pub syncobj: *mut drm_syncobj,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie2_dev_status {
    AIE2_DEV_UNINIT,
    AIE2_DEV_INIT,
    AIE2_DEV_START,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie2_exec_msg_ops {
    pub msg_op): *mut *mut size_t size, u32,
    pub msg_op): *mut *mut size_t size, u32,
    pub cmd_cnt): *mut *mut *mut void (init_chain_req)(void req, u64 slot_addr, size_t size, u32,
    pub size): *mut *mut *mut *mut int (fill_cf_slot)(struct amdxdna_gem_obj cmd_bo, void slot, size_t,
    pub size): *mut *mut *mut *mut int (fill_dpu_slot)(struct amdxdna_gem_obj cmd_bo, void slot, size_t,
    pub size): *mut *mut *mut *mut int (fill_preempt_slot)(struct amdxdna_gem_obj cmd_bo, void slot, size_t,
    pub size): *mut *mut *mut *mut int (fill_elf_slot)(struct amdxdna_gem_obj cmd_bo, void slot, size_t,
    pub cmd_op): *mut *mut u32 (get_chain_msg_op)(u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_hdl {
    pub aie: aie_device,
    pub priv: *const amdxdna_dev_priv,
    pub sram_base: *mut void __iomem,
    pub mbox_base: *mut void __iomem,
    pub total_col: u32,
    pub version: amdxdna_drm_query_aie_version,
    pub exec_msg_ops: *mut aie2_exec_msg_ops,
// power management and clock
    pub pw_mode: amdxdna_power_mode_type,
    pub dpm_level: u32,
    pub dft_dpm_level: u32,
    pub max_dpm_level: u32,
    pub clk_gating: u32,
    pub npuclk_freq: u32,
    pub hclk_freq: u32,
    pub max_tops: u32,
    pub curr_tops: u32,
    pub force_preempt_enabled: u32,
    pub frame_boundary_preempt: u32,
// Mailbox and the management channel
    pub mbox: *mut mailbox,
    pub async_events: *mut async_events,
    pub dev_status: aie2_dev_status,
    pub hwctx_num: u32,
    pub last_async_err: amdxdna_async_error,
    pub last_signal_ts: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie2_hw_ops {
    pub dpm_level): *mut *mut *mut int (set_dpm)(struct amdxdna_dev_hdl ndev, u32,
    pub ndev): *mut *mut int (update_counters)(struct amdxdna_dev_hdl,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie2_fw_feature {
    AIE2_NPU_COMMAND,
    AIE2_PREEMPT,
    AIE2_TEMPORAL_ONLY,
    AIE2_APP_HEALTH,
    AIE2_ADD_HOST_BUFFER,
    AIE2_UPDATE_PROPERTY,
    AIE2_GET_DEV_REVISION,
    AIE2_FEATURE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_priv {
    pub fw_path: *const c_char,
    pub rt_config: *const rt_config,
    pub dpm_clk_tbl: *const dpm_clk_freq,
pub const COL_ALIGN_NONE: c_int = 0;
pub const COL_ALIGN_NATURE: c_int = 1;
    pub col_align: u32,
    pub col_opc: u32,
    pub mbox_dev_addr: u32,
// If mbox_size is 0, use BAR size. See MBOX_SIZE macro
    pub mbox_size: u32,
    pub hwctx_limit: u32,
    pub sram_dev_addr: u32,
    pub sram_offs: [aie_bar_off_pair; SRAM_MAX_INDEX],
    pub psp_regs_off: [aie_bar_off_pair; PSP_MAX_REGS],
    pub smu_regs_off: [aie_bar_off_pair; SMU_MAX_REGS],
    pub hw_ops: *const aie2_hw_ops,
}

// aie2 npu hw config
// aie2_pm.c
extern "C" {
    pub fn aie2_pm_init(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
extern "C" {
    pub fn aie2_pm_set_mode(ndev: *mut amdxdna_dev_hdl, target: amdxdna_power_mode_type) -> c_int;
}
extern "C" {
    pub fn aie2_pm_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> c_int;
}
// aie2_error.c
extern "C" {
    pub fn aie2_error_async_events_alloc(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
extern "C" {
    pub fn aie2_error_async_events_free(ndev: *mut amdxdna_dev_hdl);
}
extern "C" {
    pub fn aie2_error_async_msg_thread(data: *mut c_void) -> c_int;
}
// aie2_message.c
extern "C" {
    pub fn aie2_msg_init(ndev: *mut amdxdna_dev_hdl);
}
extern "C" {
    pub fn aie2_destroy_mgmt_chann(ndev: *mut amdxdna_dev_hdl);
}
extern "C" {
    pub fn aie2_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
extern "C" {
    pub fn aie2_resume_fw(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
extern "C" {
    pub fn aie2_set_runtime_cfg(ndev: *mut amdxdna_dev_hdl, type: u32, value: u64) -> c_int;
}
extern "C" {
    pub fn aie2_get_runtime_cfg(ndev: *mut amdxdna_dev_hdl, type: u32, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn aie2_assign_mgmt_pasid(ndev: *mut amdxdna_dev_hdl, pasid: u16) -> c_int;
}
extern "C" {
    pub fn aie2_get_dev_revision(ndev: *mut amdxdna_dev_hdl, rev: *mut aie2_dev_revision) -> c_int;
}
extern "C" {
    pub fn aie2_create_context(ndev: *mut amdxdna_dev_hdl, hwctx: *mut amdxdna_hwctx) -> c_int;
}
extern "C" {
    pub fn aie2_destroy_context(ndev: *mut amdxdna_dev_hdl, hwctx: *mut amdxdna_hwctx) -> c_int;
}
extern "C" {
    pub fn aie2_map_host_buf(ndev: *mut amdxdna_dev_hdl, context_id: u32, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn aie2_add_host_buf(ndev: *mut amdxdna_dev_hdl, context_id: u32, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn aie2_query_status(ndev: *mut amdxdna_dev_hdl, buf: *mut char __user, size: u32, cols_filled: *mut u32) -> c_int;
}
extern "C" {
    pub fn aie2_update_prop_time_quota(ndev: *mut amdxdna_dev_hdl, us: u32) -> c_int;
}
// aie2_hwctx.c
extern "C" {
    pub fn aie2_hwctx_init(hwctx: *mut amdxdna_hwctx) -> c_int;
}
extern "C" {
    pub fn aie2_hwctx_fini(hwctx: *mut amdxdna_hwctx);
}
extern "C" {
    pub fn aie2_hwctx_config(hwctx: *mut amdxdna_hwctx, type: u32, value: u64, buf: *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn aie2_hwctx_sync_debug_bo(hwctx: *mut amdxdna_hwctx, debug_bo_hdl: u32) -> c_int;
}
extern "C" {
    pub fn aie2_hwctx_suspend(client: *mut amdxdna_client);
}
extern "C" {
    pub fn aie2_hwctx_resume(client: *mut amdxdna_client) -> c_int;
}
extern "C" {
    pub fn aie2_cmd_submit(hwctx: *mut amdxdna_hwctx, job: *mut amdxdna_sched_job, seq: *mut u64) -> c_int;
}
extern "C" {
    pub fn aie2_hmm_invalidate(abo: *mut amdxdna_gem_obj, cur_seq: c_ulong);
}
extern "C" {
    pub fn aie2_hwctx_heap_expand(hwctx: *mut amdxdna_hwctx, heap: *mut amdxdna_gem_obj) -> c_int;
}
