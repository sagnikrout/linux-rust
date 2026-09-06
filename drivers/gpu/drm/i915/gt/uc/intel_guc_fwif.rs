//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_fwif.h
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
// Copyright © 2014-2019 Intel Corporation
//

// Payload length only i.e. don't include G2H header length
pub const G2H_LEN_DW_SCHED_CONTEXT_MODE_SET: c_int = 2;
pub const G2H_LEN_DW_DEREGISTER_CONTEXT: c_int = 1;
pub const G2H_LEN_DW_INVALIDATE_TLB: c_int = 1;
pub const GUC_CONTEXT_DISABLE: c_int = 0;
pub const GUC_CONTEXT_ENABLE: c_int = 1;
pub const GUC_CLIENT_PRIORITY_KMD_HIGH: c_int = 0;
pub const GUC_CLIENT_PRIORITY_HIGH: c_int = 1;
pub const GUC_CLIENT_PRIORITY_KMD_NORMAL: c_int = 2;
pub const GUC_CLIENT_PRIORITY_NORMAL: c_int = 3;
pub const GUC_CLIENT_PRIORITY_NUM: c_int = 4;
pub const GUC_MAX_CONTEXT_ID: c_int = 65535;

pub const GUC_RENDER_CLASS: c_int = 0;
pub const GUC_VIDEO_CLASS: c_int = 1;
pub const GUC_VIDEOENHANCE_CLASS: c_int = 2;
pub const GUC_BLITTER_CLASS: c_int = 3;
pub const GUC_COMPUTE_CLASS: c_int = 4;
pub const GUC_GSC_OTHER_CLASS: c_int = 5;

pub const GUC_MAX_ENGINE_CLASSES: c_int = 16;
pub const GUC_MAX_INSTANCES_PER_CLASS: c_int = 32;
pub const GUC_DOORBELL_INVALID: c_int = 256;
//
// Work queue item header definitions
//
// Work queue is circular buffer used to submit complex (multi-lrc) submissions
// to the GuC. A work queue item is an entry in the circular buffer.
//
pub const WQ_STATUS_ACTIVE: c_int = 1;
pub const WQ_STATUS_SUSPENDED: c_int = 2;
pub const WQ_STATUS_CMD_ERROR: c_int = 3;
pub const WQ_STATUS_ENGINE_ID_NOT_USED: c_int = 4;
pub const WQ_STATUS_SUSPENDED_FROM_RESET: c_int = 5;
pub const WQ_TYPE_BATCH_BUF: c_uint = 0x1;
pub const WQ_TYPE_PSEUDO: c_uint = 0x2;
pub const WQ_TYPE_INORDER: c_uint = 0x3;
pub const WQ_TYPE_NOOP: c_uint = 0x4;
pub const WQ_TYPE_MULTI_LRC: c_uint = 0x5;

pub const GUC_CTL_LOG_PARAMS: c_int = 0;

pub const GUC_LOG_CRASH_SHIFT: c_int = 4;

pub const GUC_LOG_DEBUG_SHIFT: c_int = 6;

pub const GUC_LOG_CAPTURE_SHIFT: c_int = 10;

pub const GUC_LOG_BUF_ADDR_SHIFT: c_int = 12;
pub const GUC_CTL_WA: c_int = 1;

pub const GUC_CTL_FEATURE: c_int = 2;

pub const GUC_CTL_DEBUG: c_int = 3;
pub const GUC_LOG_VERBOSITY_SHIFT: c_int = 0;

// Verbosity range-check limits, without the shift
pub const GUC_LOG_VERBOSITY_MIN: c_int = 0;
pub const GUC_LOG_VERBOSITY_MAX: c_int = 3;
pub const GUC_LOG_VERBOSITY_MASK: c_uint = 0x0000000f;

pub const GUC_CTL_ADS: c_int = 4;
pub const GUC_ADS_ADDR_SHIFT: c_int = 1;

pub const GUC_CTL_DEVID: c_int = 5;

// Generic GT SysInfo data types
pub const GUC_GENERIC_GT_SYSINFO_SLICE_ENABLED: c_int = 0;
pub const GUC_GENERIC_GT_SYSINFO_VDBOX_SFC_SUPPORT_MASK: c_int = 1;
pub const GUC_GENERIC_GT_SYSINFO_DOORBELL_COUNT_PER_SQIDI: c_int = 2;
pub const GUC_GENERIC_GT_SYSINFO_MAX: c_int = 16;
//
// The class goes in bits [0..2] of the GuC ID, the instance in bits [3..6].
// Bit 7 can be used for operations that apply to all engine classes&instances.
//
pub const GUC_ENGINE_CLASS_SHIFT: c_int = 0;

pub const GUC_ENGINE_INSTANCE_SHIFT: c_int = 3;

// the GuC arrays don't include OTHER_CLASS
// Work item for submitting workloads into work queue of GuC.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_wq_item {
    pub header: u32,
    pub context_desc: u32,
    pub submit_element_info: u32,
    pub fence_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_process_desc_v69 {
    pub stage_id: u32,
    pub db_base_addr: u64,
    pub head: u32,
    pub tail: u32,
    pub error_offset: u32,
    pub wq_base_addr: u64,
    pub wq_size_bytes: u32,
    pub wq_status: u32,
    pub engine_presence: u32,
    pub priority: u32,
    pub reserved: [u32; 36],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_sched_wq_desc {
    pub head: u32,
    pub tail: u32,
    pub error_offset: u32,
    pub wq_status: u32,
    pub reserved: [u32; 28],
    pub __packed: },
// Helper for context registration H2G
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ctxt_registration_info {
    pub flags: u32,
    pub context_idx: u32,
    pub engine_class: u32,
    pub engine_submit_mask: u32,
    pub wq_desc_lo: u32,
    pub wq_desc_hi: u32,
    pub wq_base_lo: u32,
    pub wq_base_hi: u32,
    pub wq_size: u32,
    pub hwlrca_lo: u32,
    pub hwlrca_hi: u32,
}

// Preempt to idle on quantum expiry

//
// GuC Context registration descriptor.
// FIXME: This is only required to exist during context registration.
// The current 1:1 between guc_lrc_desc and LRCs for the lifetime of the LRC
// is not required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_lrc_desc_v69 {
    pub hw_context_desc: u32,
    pub /: *mut *mut u32 slpm_perf_mode_hint; / SPLC v1 only,
    pub slpm_freq_hint: u32,
    pub /: *mut *mut u32 engine_submit_mask; / In logical space,
    pub engine_class: u8,
    pub reserved0: [u8; 3],
    pub priority: u32,
    pub process_desc: u32,
    pub wq_addr: u32,
    pub wq_size: u32,
    pub /: *mut *mut *mut u32 context_flags; / CONTEXT_REGISTRATION_,
// Time for one workload to execute. (in micro seconds)
    pub execution_quantum: u32,
// Time to wait for a preemption request to complete before issuing a
// reset. (in micro seconds).
//
    pub preemption_timeout: u32,
    pub /: *mut *mut *mut u32 policy_flags; / CONTEXT_POLICY_,
    pub reserved1: [u32; 19],
    pub __packed: },
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
pub struct guc_update_context_policy_header {
    pub action: u32,
    pub ctx_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_update_context_policy {
    pub header: guc_update_context_policy_header,
    pub klv: [guc_klv_generic_dw_t; GUC_CONTEXT_POLICIES_KLV_NUM_IDS],
    pub __packed: },
// Format of the UPDATE_SCHEDULING_POLICIES H2G data packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_update_scheduling_policy_header {
    pub action: u32,
    pub __packed: },
//
// Can't dynamically allocate memory for the scheduling policy KLV because
// it will be sent from within the reset path. Need a fixed size lump on
// the stack instead :(.
//
// Currently, there is only one KLV defined, which has 1 word of KL + 2 words of V.
//
pub const MAX_SCHEDULING_POLICY_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_update_scheduling_policy {
    pub header: guc_update_scheduling_policy_header,
    pub data: [u32; MAX_SCHEDULING_POLICY_SIZE],
    pub __packed: },
pub const GUC_POWER_UNSPECIFIED: c_int = 0;
pub const GUC_POWER_D0: c_int = 1;
pub const GUC_POWER_D1: c_int = 2;
pub const GUC_POWER_D2: c_int = 3;
pub const GUC_POWER_D3: c_int = 4;
// Scheduling policy settings

pub const GLOBAL_POLICY_MAX_NUM_WI: c_int = 15;
// Don't reset an engine upon preemption failure

pub const GLOBAL_POLICY_DEFAULT_DPC_PROMOTE_TIME_US: c_int = 500000;
//
// GuC converts the timeout to clock ticks internally. Different platforms have
// different GuC clocks. Thus, the maximum value before overflow is platform
// dependent. Current worst case scenario is about 110s. So, the spec says to
// limit to 100s to be safe.
//

    pub UINT_MAX): BUILD_BUG_ON(GUC_POLICY_MAX_EXEC_QUANTUM_US >=,
    pub 1000: return GUC_POLICY_MAX_EXEC_QUANTUM_US /,
    pub UINT_MAX): BUILD_BUG_ON(GUC_POLICY_MAX_PREEMPT_TIMEOUT_US >=,
    pub 1000: return GUC_POLICY_MAX_PREEMPT_TIMEOUT_US /,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_policies {
    pub submission_queue_depth: [u32; GUC_MAX_ENGINE_CLASSES],
// In micro seconds. How much time to allow before DPC processing is
// called back via interrupt (to prevent DPC queue drain starving).
// Typically 1000s of micro seconds (example only, not granularity).
    pub dpc_promote_time: u32,
// Must be set to take these new values.
    pub is_valid: u32,
// Max number of WIs to process per call. A large value may keep CS
// idle.
    pub max_num_work_items: u32,
    pub global_flags: u32,
    pub reserved: [u32; 4],
    pub __packed: },
// GuC MMIO reg state struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_mmio_reg {
    pub offset: u32,
    pub value: u32,
    pub flags: u32,

    pub mask: u32,
    pub __packed: },
// GuC register sets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_mmio_reg_set {
    pub address: u32,
    pub count: u16,
    pub reserved: u16,
    pub __packed: },
// HW info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_gt_system_info {
    pub mapping_table: [u8; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
    pub engine_enabled_masks: [u32; GUC_MAX_ENGINE_CLASSES],
    pub generic_gt_sysinfo: [u32; GUC_GENERIC_GT_SYSINFO_MAX],
    pub __packed: },
}

// Register-types of GuC capture register lists
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_capture_type {
    GUC_CAPTURE_LIST_TYPE_GLOBAL = 0,
    GUC_CAPTURE_LIST_TYPE_ENGINE_CLASS,
    GUC_CAPTURE_LIST_TYPE_ENGINE_INSTANCE,
    GUC_CAPTURE_LIST_TYPE_MAX,
}

// Class indices for capture_class and capture_instance arrays
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
    pub reserved2: u32,
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
// GuC logging structures
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_log_buffer_type {
    GUC_DEBUG_LOG_BUFFER,
    GUC_CRASH_DUMP_LOG_BUFFER,
    GUC_CAPTURE_LOG_BUFFER,
    GUC_MAX_LOG_BUFFER
}

//
// struct guc_log_buffer_state - GuC log buffer state
//
// Below state structure is used for coordination of retrieval of GuC firmware
// logs. Separate state is maintained for each log buffer type.
// read_ptr points to the location where i915 read last in log buffer and
// is read only for GuC firmware. write_ptr is incremented by GuC with number
// of bytes written for each log entry and is read only for i915.
// When any type of log buffer becomes half full, GuC sends a flush interrupt.
// GuC firmware expects that while it is writing to 2nd half of the buffer,
// first half would get consumed by Host and then get a flush completed
// acknowledgment from Host, so that it does not end up doing any overwrite
// causing loss of logs. So when buffer gets half filled & i915 has requested
// for interrupt, GuC will set flush_to_file field, set the sampled_write_ptr
// to the value of write_ptr and raise the interrupt.
// On receiving the interrupt i915 should read the buffer, clear flush_to_file
// field and also update read_ptr with the value of sample_write_ptr, before
// sending an acknowledgment to GuC. marker & version fields are for internal
// usage of GuC and opaque to i915. buffer_full_cnt field is incremented every
// time GuC detects the log buffer overflow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_log_buffer_state {
    pub marker: [u32; 2],
    pub read_ptr: u32,
    pub write_ptr: u32,
    pub size: u32,
    pub sampled_write_ptr: u32,
    pub wrap_offset: u32,
    pub flush_to_file:1: u32,
    pub buffer_full_cnt:4: u32,
    pub reserved:27: u32,
}

// This action will be programmed in C1BC - SOFT_SCRATCH_15_REG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_guc_recv_message {
    INTEL_GUC_RECV_MSG_CRASH_DUMP_POSTED = BIT(1),
    INTEL_GUC_RECV_MSG_EXCEPTION = BIT(30),
}
