//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/umsch_mm_4_0_api_def.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const UMSCH_API_VERSION: c_int = 1;
//
// Driver submits one API(cmd) as a single Frame and this command size is same for all API
// to ease the debugging and parsing of ring buffer.
//
// To avoid command in scheduler context to be overwritten whenever multiple interrupts come in,
// this creates another queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_API_TYPE {
    UMSCH_API_TYPE_SCHEDULER = 1,
    UMSCH_API_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_MS_LOG_CONTEXT_STATE {
    UMSCH_LOG_CONTEXT_STATE_IDLE = 0,
    UMSCH_LOG_CONTEXT_STATE_RUNNING = 1,
    UMSCH_LOG_CONTEXT_STATE_READY = 2,
    UMSCH_LOG_CONTEXT_STATE_READY_STANDBY = 3,
    UMSCH_LOG_CONTEXT_STATE_INVALID = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_MS_LOG_OPERATION {
    UMSCH_LOG_OPERATION_CONTEXT_STATE_CHANGE = 0,
    UMSCH_LOG_OPERATION_QUEUE_NEW_WORK = 1,
    UMSCH_LOG_OPERATION_QUEUE_UNWAIT_SYNC_OBJECT = 2,
    UMSCH_LOG_OPERATION_QUEUE_NO_MORE_WORK = 3,
    UMSCH_LOG_OPERATION_QUEUE_WAIT_SYNC_OBJECT = 4,
    UMSCH_LOG_OPERATION_QUEUE_INVALID = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_INSTANCE_DB_OFFSET {
    pub instance_index: u32,
    pub doorbell_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_CONTEXT_STATE_CHANGE {
    pub h_context: u64,
    pub new_context_state: UMSCH_MS_LOG_CONTEXT_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_QUEUE_NEW_WORK {
    pub h_queue: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_QUEUE_UNWAIT_SYNC_OBJECT {
    pub h_queue: u64,
    pub h_sync_object: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_QUEUE_NO_MORE_WORK {
    pub h_queue: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_QUEUE_WAIT_SYNC_OBJECT {
    pub h_queue: u64,
    pub h_sync_object: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_ENTRY_HEADER {
    pub first_free_entry_index: u32,
    pub wraparound_count: u32,
    pub number_of_entries: u64,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_ENTRY_DATA {
    pub gpu_time_stamp: u64,
    pub /: *mut *mut uint32_t operation_type; / operation_type is of UMSCH_LOG_OPERATION type,
    pub reserved_operation_type_bits: u32,
    pub context_state_change: UMSCH_LOG_CONTEXT_STATE_CHANGE,
    pub queue_new_work: UMSCH_LOG_QUEUE_NEW_WORK,
    pub queue_unwait_sync_object: UMSCH_LOG_QUEUE_UNWAIT_SYNC_OBJECT,
    pub queue_no_more_work: UMSCH_LOG_QUEUE_NO_MORE_WORK,
    pub queue_wait_sync_object: UMSCH_LOG_QUEUE_WAIT_SYNC_OBJECT,
    pub all: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_LOG_BUFFER {
    pub header: UMSCH_LOG_ENTRY_HEADER,
    pub entries: [UMSCH_LOG_ENTRY_DATA; 1],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_API_OPCODE {
    UMSCH_API_SET_HW_RSRC = 0x00,
    UMSCH_API_SET_SCHEDULING_CONFIG = 0x1,
    UMSCH_API_ADD_QUEUE = 0x2,
    UMSCH_API_REMOVE_QUEUE = 0x3,
    UMSCH_API_PERFORM_YIELD = 0x4,
    UMSCH_API_SUSPEND = 0x5,
    UMSCH_API_RESUME = 0x6,
    UMSCH_API_RESET = 0x7,
    UMSCH_API_SET_LOG_BUFFER = 0x8,
    UMSCH_API_CHANGE_CONTEXT_PRIORITY = 0x9,
    UMSCH_API_QUERY_SCHEDULER_STATUS = 0xA,
    UMSCH_API_UPDATE_AFFINITY = 0xB,
    UMSCH_API_MAX = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCH_API_HEADER {
    pub /: *mut *mut uint32_t type : 4; / 0 - Invalid; 1 - Scheduling; 2 - TBD,
    pub 8: uint32_t opcode :,
    pub 8: uint32_t dwsize :,
    pub 12: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_AMD_PRIORITY_LEVEL {
    AMD_PRIORITY_LEVEL_IDLE = 0,
    AMD_PRIORITY_LEVEL_NORMAL = 1,
    AMD_PRIORITY_LEVEL_FOCUS = 2,
    AMD_PRIORITY_LEVEL_REALTIME = 3,
    AMD_PRIORITY_NUM_LEVELS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_ENGINE_TYPE {
    UMSCH_ENGINE_TYPE_VCN0 = 0,
    UMSCH_ENGINE_TYPE_VCN1 = 1,
    UMSCH_ENGINE_TYPE_VCN = 2,
    UMSCH_ENGINE_TYPE_VPE = 3,
    UMSCH_ENGINE_TYPE_MAX
}

pub const AFFINITY_DISABLE: c_int = 0;
pub const AFFINITY_ENABLE: c_int = 1;
pub const AFFINITY_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCH_AFFINITY {
    pub /: *mut *mut unsigned int vcn0Affinity : 2; / enable 1 disable 0,
    pub 2: unsigned int vcn1Affinity :,
    pub 28: unsigned int reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UMSCH_API_STATUS {
    pub api_completion_fence_addr: u64,
    pub api_completion_fence_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VM_HUB_TYPE {
    VM_HUB_TYPE_GC = 0,
    VM_HUB_TYPE_MM = 1,
    VM_HUB_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__SET_HW_RESOURCES {
    pub header: UMSCH_API_HEADER,
    pub vmid_mask_mm_vcn: u32,
    pub vmid_mask_mm_vpe: u32,
    pub collaboration_mask_vpe: u32,
    pub engine_mask: u32,
    pub logging_vmid: u32,
    pub vcn0_hqd_mask: [u32; MAX_VCN0_INSTANCES],
    pub vcn1_hqd_mask: [u32; MAX_VCN1_INSTANCES],
    pub vcn_hqd_mask: [u32; MAX_VCN_INSTANCES],
    pub vpe_hqd_mask: [u32; MAX_VPE_INSTANCES],
    pub g_sch_ctx_gpu_mc_ptr: u64,
    pub mmhub_base: [u32; UMSCH_MAX_HWIP_SEGMENT],
    pub mmhub_version: u32,
    pub osssys_base: [u32; UMSCH_MAX_HWIP_SEGMENT],
    pub osssys_version: u32,
    pub vcn_version: u32,
    pub vpe_version: u32,
    pub api_status: UMSCH_API_STATUS,
    pub 1: uint32_t disable_reset :,
    pub 1: uint32_t disable_umsch_log :,
    pub 1: uint32_t enable_level_process_quantum_check :,
    pub 1: uint32_t is_vcn0_enabled :,
    pub 1: uint32_t is_vcn1_enabled :,
    pub 1: uint32_t use_rs64mem_for_proc_ctx_csa :,
    pub 26: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__SET_SCHEDULING_CONFIG {
    pub header: UMSCH_API_HEADER,
//
// Grace period when preempting another priority band for this priority band.
// The value for idle priority band is ignored, as it never preempts other bands.
//
    pub grace_period_other_levels: [u64; AMD_PRIORITY_NUM_LEVELS],
// Default quantum for scheduling across processes within a priority band.
    pub process_quantum_for_level: [u64; AMD_PRIORITY_NUM_LEVELS],
// Default grace period for processes that preempt each other within a priority band.
    pub process_grace_period_same_level: [u64; AMD_PRIORITY_NUM_LEVELS],
//
// For normal level this field specifies the target GPU percentage in situations
// when it's starved by the high level. Valid values are between 0 and 50,
// with the default being 10.
//
    pub normal_yield_percent: u32,
    pub api_status: UMSCH_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__ADD_QUEUE {
    pub header: UMSCH_API_HEADER,
    pub process_id: u32,
    pub page_table_base_addr: u64,
    pub process_va_start: u64,
    pub process_va_end: u64,
    pub process_quantum: u64,
    pub process_csa_addr: u64,
    pub context_quantum: u64,
    pub context_csa_addr: u64,
    pub inprocess_context_priority: u32,
    pub context_global_priority_level: UMSCH_AMD_PRIORITY_LEVEL,
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub affinity: UMSCH_AFFINITY,
    pub mqd_addr: u64,
    pub h_context: u64,
    pub h_queue: u64,
    pub engine_type: UMSCH_ENGINE_TYPE,
    pub vm_context_cntl: u32,
    pub 1: uint32_t is_context_suspended :,
    pub 1: uint32_t collaboration_mode :,
    pub 30: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__REMOVE_QUEUE {
    pub header: UMSCH_API_HEADER,
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub context_csa_addr: u64,
    pub api_status: UMSCH_API_STATUS,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__PERFORM_YIELD {
    pub header: UMSCH_API_HEADER,
    pub dummy: u32,
    pub api_status: UMSCH_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__SUSPEND {
    pub header: UMSCH_API_HEADER,
    pub context_csa_addr: u64,
    pub suspend_fence_addr: u64,
    pub suspend_fence_value: u32,
    pub api_status: UMSCH_API_STATUS,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_RESUME_OPTION {
    CONTEXT_RESUME = 0,
    ENGINE_SCHEDULE_RESUME = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__RESUME {
    pub header: UMSCH_API_HEADER,
    pub resume_option: UMSCH_RESUME_OPTION,
    pub /: *mut *mut uint64_t context_csa_addr; / valid only for UMSCH_SWIP_CONTEXT_RESUME,
    pub engine_type: UMSCH_ENGINE_TYPE,
    pub api_status: UMSCH_API_STATUS,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_RESET_OPTION {
    HANG_DETECT_AND_RESET = 0,
    HANG_DETECT_ONLY = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__RESET {
    pub header: UMSCH_API_HEADER,
    pub reset_option: UMSCH_RESET_OPTION,
    pub doorbell_offset_addr: u64,
    pub engine_type: UMSCH_ENGINE_TYPE,
    pub api_status: UMSCH_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__SET_LOGGING_BUFFER {
    pub header: UMSCH_API_HEADER,
// There are separate log buffers for each queue type
    pub log_type: UMSCH_ENGINE_TYPE,
// Log buffer GPU Address
    pub logging_buffer_addr: u64,
// Number of entries in the log buffer
    pub number_of_entries: u32,
// Entry index at which CPU interrupt needs to be signalled
    pub interrupt_entry: u32,
    pub api_status: UMSCH_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__UPDATE_AFFINITY {
    pub header: UMSCH_API_HEADER,
    pub affinity: UMSCH_AFFINITY,
    pub context_csa_addr: u64,
    pub api_status: UMSCH_API_STATUS,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__CHANGE_CONTEXT_PRIORITY_LEVEL {
    pub header: UMSCH_API_HEADER,
    pub inprocess_context_priority: u32,
    pub context_global_priority_level: UMSCH_AMD_PRIORITY_LEVEL,
    pub context_quantum: u64,
    pub context_csa_addr: u64,
    pub api_status: UMSCH_API_STATUS,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UMSCHAPI__QUERY_UMSCH_STATUS {
    pub header: UMSCH_API_HEADER,
    pub /: *mut *mut bool umsch_mm_healthy; / 0 - not healthy, 1 - healthy,
    pub api_status: UMSCH_API_STATUS,
}

