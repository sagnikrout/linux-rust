//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_capture_abi.h
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
// Copyright © 2024 Intel Corporation
//

// Capture List Index
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_capture_list_index_type {
    GUC_CAPTURE_LIST_INDEX_PF = 0,
    GUC_CAPTURE_LIST_INDEX_VF = 1,
}

// Register-types of GuC capture register lists
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_state_capture_type {
    GUC_STATE_CAPTURE_TYPE_GLOBAL = 0,
    GUC_STATE_CAPTURE_TYPE_ENGINE_CLASS,
    GUC_STATE_CAPTURE_TYPE_ENGINE_INSTANCE
}

// Class indices for capture_class and capture_instance arrays
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_capture_list_class_type {
    GUC_CAPTURE_LIST_CLASS_RENDER_COMPUTE = 0,
    GUC_CAPTURE_LIST_CLASS_VIDEO = 1,
    GUC_CAPTURE_LIST_CLASS_VIDEOENHANCE = 2,
    GUC_CAPTURE_LIST_CLASS_BLITTER = 3,
    GUC_CAPTURE_LIST_CLASS_GSC_OTHER = 4,
    GUC_CAPTURE_LIST_CLASS_PAGING = 5,
}

//
// struct guc_mmio_reg - GuC MMIO reg state struct
//
// GuC MMIO reg state struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_mmio_reg {
// @offset: MMIO Offset - filled in by Host
    pub offset: u32,
// @value: MMIO Value - Used by Firmware to store value
    pub value: u32,
// @flags: Flags for accessing the MMIO
    pub flags: u32,
// @mask: Value of a mask to apply if mask with value is set
    pub mask: u32,

    pub __packed: },
//
// struct guc_mmio_reg_set - GuC register sets
//
// GuC register sets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_mmio_reg_set {
// @address: register address
    pub address: u32,
// @count: register count
    pub count: u16,
// @reserved: reserved
    pub reserved: u16,
    pub __packed: },
//
// struct guc_debug_capture_list_header - Debug capture list header.
//
// Debug capture list header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_debug_capture_list_header {
// @info: contains number of MMIO descriptors in the capture list.
    pub info: u32,

    pub __packed: },
//
// struct guc_debug_capture_list - Debug capture list
//
// As part of ADS registration, these header structures (followed by
// an array of 'struct guc_mmio_reg' entries) are used to register with
// GuC microkernel the list of registers we want it to dump out prior
// to a engine reset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_debug_capture_list {
// @header: Debug capture list header.
    pub header: guc_debug_capture_list_header,
// @regs: MMIO descriptors in the capture list.
    pub regs: [guc_mmio_reg; ],
    pub __packed: },
//
// struct guc_state_capture_header_t - State capture header.
//
// Prior to resetting engines that have hung or faulted, GuC microkernel
// reports the engine error-state (register values that was read) by
// logging them into the shared GuC log buffer using these hierarchy
// of structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_state_capture_header_t {
//
// @owner: VFID
// BR[ 7: 0] MBZ when SRIOV is disabled. When SRIOV is enabled
// VFID is an integer in range [0, 63] where 0 means the state capture
// is corresponding to the PF and an integer N in range [1, 63] means
// the state capture is for VF N.
//
    pub owner: u32,

// @info: Engine class/instance and capture type info
    pub info: u32,

//
// @lrca: logical ring context address.
// if type-instance, LRCA (address) that hung, else set to ~0
//
    pub lrca: u32,
//
// @guc_id: context_index.
// if type-instance, context index of hung context, else set to ~0
//
    pub guc_id: u32,
// @num_mmio_entries: Number of captured MMIO entries.
    pub num_mmio_entries: u32,

    pub __packed: },
//
// struct guc_state_capture_t - State capture.
//
// State capture
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_state_capture_t {
// @header: State capture header.
    pub header: guc_state_capture_header_t,
// @mmio_entries: Array of captured guc_mmio_reg entries.
    pub mmio_entries: [guc_mmio_reg; ],
    pub __packed: },
// State Capture Group Type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum guc_state_capture_group_type {
    GUC_STATE_CAPTURE_GROUP_TYPE_FULL = 0,
    GUC_STATE_CAPTURE_GROUP_TYPE_PARTIAL
}

//
// struct guc_state_capture_group_header_t - State capture group header
//
// State capture group header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_state_capture_group_header_t {
// @owner: VFID
    pub owner: u32,

// @info: Engine class/instance and capture type info
    pub info: u32,

    pub __packed: },
//
// struct guc_state_capture_group_t - State capture group.
//
// this is the top level structure where an error-capture dump starts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_state_capture_group_t {
// @grp_header: State capture group header.
    pub grp_header: guc_state_capture_group_header_t,
// @capture_entries: Array of state captures
    pub capture_entries: [guc_state_capture_t; ],
    pub __packed: },
