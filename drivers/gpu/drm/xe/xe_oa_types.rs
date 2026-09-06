//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_oa_types.h
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
// Copyright © 2023-2024 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_oa_report_header {
    HDR_32_BIT = 0,
    HDR_64_BIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_oa_format_name {
    XE_OA_FORMAT_C4_B8,

// Gen8+
    XE_OA_FORMAT_A12,
    XE_OA_FORMAT_A12_B8_C8,
    XE_OA_FORMAT_A32u40_A4u32_B8_C8,

// DG2
    XE_OAR_FORMAT_A32u40_A4u32_B8_C8,
    XE_OA_FORMAT_A24u40_A14u32_B8_C8,

// DG2/MTL OAC
    XE_OAC_FORMAT_A24u64_B8_C8,
    XE_OAC_FORMAT_A22u32_R2u32_B8_C8,

// MTL OAM
    XE_OAM_FORMAT_MPEC8u64_B8_C8,
    XE_OAM_FORMAT_MPEC8u32_B8_C8,

// Xe2+
    XE_OA_FORMAT_PEC64u64,
    XE_OA_FORMAT_PEC64u64_B8_C8,
    XE_OA_FORMAT_PEC64u32,
    XE_OA_FORMAT_PEC32u64_G1,
    XE_OA_FORMAT_PEC32u32_G1,
    XE_OA_FORMAT_PEC32u64_G2,
    XE_OA_FORMAT_PEC32u32_G2,
    XE_OA_FORMAT_PEC36u64_G1_32_G2_4,
    XE_OA_FORMAT_PEC36u64_G1_4_G2_32,

    __XE_OA_FORMAT_MAX,
}

//
// struct xe_oa_format - Format fields for supported OA formats. OA format
// properties are specified in PRM/Bspec 52198 and 60942
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_format {
// @counter_select: counter select value (see Bspec 52198/60942)
    pub counter_select: u32,
// @size: record size as written by HW (multiple of 64 byte cachelines)
    pub size: c_int,
// @type: of enum drm_xe_oa_format_type
    pub type: c_int,
// @header: 32 or 64 bit report headers
    pub header: xe_oa_report_header,
// @counter_size: counter size value (see Bspec 60942)
    pub counter_size: u16,
// @bc_report: BC report value (see Bspec 60942)
    pub bc_report: u16,
}

// struct xe_oa_regs - Registers for each OA unit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_regs {
    pub base: u32,
    pub oa_head_ptr: xe_reg,
    pub oa_tail_ptr: xe_reg,
    pub oa_buffer: xe_reg,
    pub oa_ctx_ctrl: xe_reg,
    pub oa_ctrl: xe_reg,
    pub oa_debug: xe_reg,
    pub oa_status: xe_reg,
    pub oa_mmio_trg: xe_reg,
    pub oa_ctrl_counter_select_mask: u32,
}

//
// struct xe_oa_unit - Hardware OA unit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_unit {
// @oa_unit_id: identifier for the OA unit
    pub oa_unit_id: u16,
// @gt: gt associated with the OA unit
    pub gt: *mut xe_gt,
// @type: Type of OA unit - OAM, OAG etc.
    pub type: drm_xe_oa_unit_type,
// @regs: OA registers for programming the OA unit
    pub regs: xe_oa_regs,
// @num_engines: number of engines attached to this OA unit
    pub num_engines: u32,
// @exclusive_stream: The stream currently using the OA unit
    pub exclusive_stream: *mut xe_oa_stream,
}

//
// struct xe_oa_gt - OA per-gt information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_gt {
// @gt_lock: lock protecting create/destroy OA streams
    pub gt_lock: mutex,
// @num_oa_units: number of oa units for each gt
    pub num_oa_units: u32,
// @oa_unit: array of oa_units
    pub oa_unit: *mut xe_oa_unit,
// @whitelist_count: number of open streams for which oa registers are whitelisted
    pub whitelist_count: u32,
}

//
// struct xe_oa - OA device level information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa {
// @xe: back pointer to xe device
    pub xe: *mut xe_device,
// @metrics_kobj: kobj for metrics sysfs
    pub metrics_kobj: *mut kobject,
// @metrics_lock: lock protecting add/remove configs
    pub metrics_lock: mutex,
// @metrics_idr: List of dynamic configurations (struct xe_oa_config)
    pub metrics_idr: idr,
// @oa_formats: tracks all OA formats across platforms
    pub oa_formats: *const xe_oa_format,
// @format_mask: tracks valid OA formats for a platform
    pub format_mask: [c_ulong; BITS_TO_LONGS(__XE_OA_FORMAT_MAX)],
// @oa_unit_ids: tracks oa unit ids assigned across gt's
    pub oa_unit_ids: u16,
}

//
// struct xe_oa_buffer - State of the stream OA buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_buffer {
// @format: data format
    pub format: *const xe_oa_format,
// @bo: xe_bo backing the OA buffer
    pub bo: *mut xe_bo,
// @bounce: bounce buffer used with xe_map layer
    pub bounce: *mut c_void,
// @ptr_lock: Lock protecting reads/writes to head/tail pointers
    pub ptr_lock: spinlock_t,
// @head: Cached head to read from
    pub head: u32,
// @tail: The last verified cached tail where HW has completed writing
    pub tail: u32,
// @circ_size: The effective circular buffer size, for Xe2+
    pub circ_size: u32,
}

//
// struct xe_oa_stream - state for a single open stream FD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_oa_stream {
// @oa: xe_oa backpointer
    pub oa: *mut xe_oa,
// @gt: gt associated with the oa stream
    pub gt: *mut xe_gt,
// @oa_unit: oa unit for this stream
    pub oa_unit: *mut xe_oa_unit,
// @hwe: hardware engine associated with this oa stream
    pub hwe: *mut xe_hw_engine,
// @stream_lock: Lock serializing stream operations
    pub stream_lock: mutex,
// @sample: true if DRM_XE_OA_PROP_SAMPLE_OA is provided
    pub sample: bool,
// @exec_q: Exec queue corresponding to DRM_XE_OA_PROPERTY_EXEC_QUEUE_ID
    pub exec_q: *mut xe_exec_queue,
// @k_exec_q: kernel exec_q used for OA programming batch submissions
    pub k_exec_q: *mut xe_exec_queue,
// @enabled: Whether the stream is currently enabled
    pub enabled: bool,
// @oa_config: OA configuration used by the stream
    pub oa_config: *mut xe_oa_config,
// @oa_config_bos: List of struct @xe_oa_config_bo's
    pub oa_config_bos: llist_head,
// @poll_check_timer: Timer to periodically check for data in the OA buffer
    pub poll_check_timer: hrtimer,
// @poll_wq: Wait queue for waiting for OA data to be available
    pub poll_wq: wait_queue_head_t,
// @pollin: Whether there is data available to read
    pub pollin: bool,
// @wait_num_reports: Number of reports to wait for before signalling pollin
    pub wait_num_reports: c_int,
// @periodic: Whether periodic sampling is currently enabled
    pub periodic: bool,
// @period_exponent: OA unit sampling frequency is derived from this
    pub period_exponent: c_int,
// @oa_buffer: OA buffer for the stream
    pub oa_buffer: xe_oa_buffer,
// @poll_period_ns: hrtimer period for checking OA buffer for available data
    pub poll_period_ns: u64,
// @oa_status: temporary storage for oa_status register value
    pub oa_status: u32,
// @no_preempt: Whether preemption and timeslicing is disabled for stream exec_q
    pub no_preempt: u32,
// @xef: xe_file with which the stream was opened
    pub xef: *mut xe_file,
// @ufence_syncobj: User fence syncobj
    pub ufence_syncobj: *mut drm_syncobj,
// @ufence_timeline_value: User fence timeline value
    pub ufence_timeline_value: u64,
// @last_fence: fence to use in stream destroy when needed
    pub last_fence: *mut dma_fence,
// @num_syncs: size of @syncs array
    pub num_syncs: u32,
// @syncs: syncs to wait on and to signal
    pub syncs: *mut xe_sync_entry,
// @fw_ref: Forcewake reference
    pub fw_ref: c_uint,
}
