//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_fwif.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

pub const G2H_LEN_DW_SCHED_CONTEXT_MODE_SET: c_int = 4;
pub const G2H_LEN_DW_DEREGISTER_CONTEXT: c_int = 3;
pub const G2H_LEN_DW_TLB_INVALIDATE: c_int = 3;
pub const G2H_LEN_DW_G2G_NOTIFY_MIN: c_int = 3;
pub const G2H_LEN_DW_MULTI_QUEUE_CONTEXT: c_int = 3;
pub const G2H_LEN_DW_PAGE_RECLAMATION: c_int = 3;
// 32-bit KLV structure as used by policy updates and others
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_klv_generic_dw_t {
    pub kl: u32,
    pub value: u32,
    pub __packed: },
// Format of the UPDATE_CONTEXT_POLICIES H2G data packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_update_exec_queue_policy_header {
    pub action: u32,
    pub guc_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_update_exec_queue_policy {
    pub header: guc_update_exec_queue_policy_header,
    pub klv: [guc_klv_generic_dw_t; GUC_CONTEXT_POLICIES_KLV_NUM_IDS],
    pub __packed: },
// GUC_CTL_* - Parameters for loading the GuC
pub const GUC_CTL_LOG_PARAMS: c_int = 0;

pub const GUC_CTL_WA: c_int = 1;

pub const GUC_CTL_FEATURE: c_int = 2;

pub const GUC_CTL_DEBUG: c_int = 3;

pub const GUC_LOG_VERBOSITY_MAX: c_int = 3;

pub const GUC_CTL_ADS: c_int = 4;

pub const GUC_CTL_DEVID: c_int = 5;
pub const GUC_CTL_MAX_DWORDS: c_int = 14;
// Scheduling policy settings
pub const GLOBAL_POLICY_MAX_NUM_WI: c_int = 15;
// Don't reset an engine upon preemption failure

pub const GLOBAL_POLICY_DEFAULT_DPC_PROMOTE_TIME_US: c_int = 500000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_policies {
    pub submission_queue_depth: [u32; GUC_MAX_ENGINE_CLASSES],
//
// In micro seconds. How much time to allow before DPC processing is
// called back via interrupt (to prevent DPC queue drain starving).
// Typically 1000s of micro seconds (example only, not granularity).
//
    pub dpc_promote_time: u32,
// Must be set to take these new values.
    pub is_valid: u32,
//
// Max number of WIs to process per call. A large value may keep CS
// idle.
//
    pub max_num_work_items: u32,
    pub global_flags: u32,
    pub reserved: [u32; 4],
    pub __packed: },
// Generic GT SysInfo data types
pub const GUC_GENERIC_GT_SYSINFO_SLICE_ENABLED: c_int = 0;
pub const GUC_GENERIC_GT_SYSINFO_VDBOX_SFC_SUPPORT_MASK: c_int = 1;
pub const GUC_GENERIC_GT_SYSINFO_DOORBELL_COUNT_PER_SQIDI: c_int = 2;
pub const GUC_GENERIC_GT_SYSINFO_MAX: c_int = 16;
// HW info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_gt_system_info {
    pub mapping_table: [u8; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
    pub engine_enabled_masks: [u32; GUC_MAX_ENGINE_CLASSES],
    pub generic_gt_sysinfo: [u32; GUC_GENERIC_GT_SYSINFO_MAX],
    pub __packed: },
// GuC Additional Data Struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ads {
    pub reg_state_list: [guc_mmio_reg_set; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
    pub reserved0: u32,
    pub scheduler_policies: u32,
    pub gt_system_info: u32,
    pub reserved1: u32,
    pub control_data: u32,
    pub golden_context_lrca: [u32; GUC_MAX_ENGINE_CLASSES],
    pub eng_state_size: [u32; GUC_MAX_ENGINE_CLASSES],
    pub private_data: u32,
    pub um_init_data: u32,
    pub capture_instance: [u32; GUC_CAPTURE_LIST_INDEX_MAX][GUC_MAX_ENGINE_CLASSES],
    pub capture_class: [u32; GUC_CAPTURE_LIST_INDEX_MAX][GUC_MAX_ENGINE_CLASSES],
    pub capture_global: [u32; GUC_CAPTURE_LIST_INDEX_MAX],
    pub wa_klv_addr_lo: u32,
    pub wa_klv_addr_hi: u32,
    pub wa_klv_size: u32,
    pub reserved: [u32; 11],
    pub __packed: },
// Engine usage stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_engine_usage_record {
    pub current_context_index: u32,
    pub last_switch_in_stamp: u32,
    pub reserved0: u32,
    pub total_runtime: u32,
    pub reserved1: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_engine_usage {
    pub engines: [guc_engine_usage_record; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
    pub __packed: },
// Engine Activity stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_engine_activity {
    pub change_num: u16,
    pub quanta_ratio: u16,
    pub last_update_tick: u32,
    pub active_ticks: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_engine_activity_data {
    pub engine_activity: [guc_engine_activity; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_engine_activity_metadata {
    pub guc_tsc_frequency_hz: u32,
    pub lag_latency_usec: u32,
    pub global_change_num: u32,
    pub reserved: u32,
    pub __packed: },
// This action will be programmed in C1BC - SOFT_SCRATCH_15_REG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_guc_recv_message {
    XE_GUC_RECV_MSG_CRASH_DUMP_POSTED = BIT(1),
    XE_GUC_RECV_MSG_EXCEPTION = BIT(30),
}

// Page fault structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_counter_desc {
    pub dw0: u32,

    pub dw1: u32,

    pub dw2: u32,

    pub dw3: u32,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_um_queue_type {
    GUC_UM_HW_QUEUE_PAGE_FAULT = 0,
    GUC_UM_HW_QUEUE_PAGE_FAULT_RESPONSE,
    GUC_UM_HW_QUEUE_ACCESS_COUNTER,
    GUC_UM_HW_QUEUE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_um_queue_params {
    pub base_dpa: u64,
    pub base_ggtt_address: u32,
    pub size_in_bytes: u32,
    pub rsvd: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_um_init_params {
    pub page_response_timeout_in_us: u64,
    pub rsvd: [u32; 6],
    pub queue_params: [guc_um_queue_params; GUC_UM_HW_QUEUE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_guc_fault_reply_type {
    PFR_ACCESS = 0,
    PFR_ENGINE,
    PFR_VFID,
    PFR_ALL,
    PFR_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_guc_response_desc_type {
    TLB_INVALIDATION_DESC = 0,
    FAULT_RESPONSE_DESC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_pagefault_desc {
    pub dw0: u32,

    pub dw1: u32,

pub const PFD_PDATA_HI_SHIFT: c_int = 4;

    pub dw2: u32,

pub const PFD_VIRTUAL_ADDR_LO_SHIFT: c_int = 12;
    pub dw3: u32,

pub const PFD_VIRTUAL_ADDR_HI_SHIFT: c_int = 32;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_pagefault_reply {
    pub dw0: u32,

    pub dw1: u32,

    pub dw2: u32,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_acc_desc {
    pub dw0: u32,

pub const ACC_TRIGGER: c_int = 0;
pub const ACC_NOTIFY: c_int = 1;

    pub dw1: u32,

    pub dw2: u32,

    pub dw3: u32,

    pub __packed: },
