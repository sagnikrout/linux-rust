//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/mes_v11_api_def.h
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


//
// Copyright 2022 Advanced Micro Devices, Inc.
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

pub const MES_API_VERSION: c_int = 1;
// Maximum log buffer size for MES. Needs to be updated if MES expands MES_EVT_INTR_HIST_LOG
pub const AMDGPU_MES_LOG_BUFFER_SIZE: c_uint = 0x4000;
// Driver submits one API(cmd) as a single Frame and this command size is same
// for all API to ease the debugging and parsing of ring buffer.
//
// To avoid command in scheduler context to be overwritten whenenver mutilple
// interrupts come in, this creates another queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_API_TYPE {
    MES_API_TYPE_SCHEDULER = 1,
    MES_API_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_SCH_API_OPCODE {
    MES_SCH_API_SET_HW_RSRC			= 0,
    MES_SCH_API_SET_SCHEDULING_CONFIG	= 1, /* agreegated db, quantums, etc */
    MES_SCH_API_ADD_QUEUE			= 2,
    MES_SCH_API_REMOVE_QUEUE		= 3,
    MES_SCH_API_PERFORM_YIELD		= 4,
    MES_SCH_API_SET_GANG_PRIORITY_LEVEL	= 5,
    MES_SCH_API_SUSPEND			= 6,
    MES_SCH_API_RESUME			= 7,
    MES_SCH_API_RESET			= 8,
    MES_SCH_API_SET_LOG_BUFFER		= 9,
    MES_SCH_API_CHANGE_GANG_PRORITY		= 10,
    MES_SCH_API_QUERY_SCHEDULER_STATUS	= 11,
    MES_SCH_API_PROGRAM_GDS			= 12,
    MES_SCH_API_SET_DEBUG_VMID		= 13,
    MES_SCH_API_MISC			= 14,
    MES_SCH_API_UPDATE_ROOT_PAGE_TABLE      = 15,
    MES_SCH_API_AMD_LOG                     = 16,
    MES_SCH_API_SET_HW_RSRC_1               = 19,
    MES_SCH_API_MAX				= 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MES_API_HEADER {
    pub /: *mut *mut uint32_t type : 4; / 0 - Invalid; 1 - Scheduling; 2 - TBD,
    pub 8: uint32_t opcode :,
    pub /: *mut *mut uint32_t dwsize : 8; / including header,
    pub 12: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_AMD_PRIORITY_LEVEL {
    AMD_PRIORITY_LEVEL_LOW		= 0,
    AMD_PRIORITY_LEVEL_NORMAL	= 1,
    AMD_PRIORITY_LEVEL_MEDIUM	= 2,
    AMD_PRIORITY_LEVEL_HIGH		= 3,
    AMD_PRIORITY_LEVEL_REALTIME	= 4,
    AMD_PRIORITY_NUM_LEVELS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_QUEUE_TYPE {
    MES_QUEUE_TYPE_GFX,
    MES_QUEUE_TYPE_COMPUTE,
    MES_QUEUE_TYPE_SDMA,
    MES_QUEUE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_API_STATUS {
    pub api_completion_fence_addr: u64,
    pub api_completion_fence_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VM_HUB_TYPE {
    VM_HUB_TYPE_GC = 0,
    VM_HUB_TYPE_MM = 1,
    VM_HUB_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SET_DEBUG_VMID_OPERATIONS {
    DEBUG_VMID_OP_PROGRAM = 0,
    DEBUG_VMID_OP_ALLOCATE = 1,
    DEBUG_VMID_OP_RELEASE = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_LOG_OPERATION {
    MES_LOG_OPERATION_CONTEXT_STATE_CHANGE = 0,
    MES_LOG_OPERATION_QUEUE_NEW_WORK = 1,
    MES_LOG_OPERATION_QUEUE_UNWAIT_SYNC_OBJECT = 2,
    MES_LOG_OPERATION_QUEUE_NO_MORE_WORK = 3,
    MES_LOG_OPERATION_QUEUE_WAIT_SYNC_OBJECT = 4,
    MES_LOG_OPERATION_QUEUE_INVALID = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_LOG_CONTEXT_STATE {
    MES_LOG_CONTEXT_STATE_IDLE		= 0,
    MES_LOG_CONTEXT_STATE_RUNNING		= 1,
    MES_LOG_CONTEXT_STATE_READY		= 2,
    MES_LOG_CONTEXT_STATE_READY_STANDBY	= 3,
    MES_LOG_CONTEXT_STATE_INVALID           = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_CONTEXT_STATE_CHANGE {
    pub h_context: *mut c_void,
    pub new_context_state: MES_LOG_CONTEXT_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_QUEUE_NEW_WORK {
    pub h_queue: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT {
    pub h_queue: u64,
    pub h_sync_object: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_QUEUE_NO_MORE_WORK {
    pub h_queue: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_QUEUE_WAIT_SYNC_OBJECT {
    pub h_queue: u64,
    pub h_sync_object: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_ENTRY_HEADER {
    pub first_free_entry_index: u32,
    pub wraparound_count: u32,
    pub number_of_entries: u64,
    pub reserved: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_ENTRY_DATA {
    pub gpu_time_stamp: u64,
    pub /: *mut *mut uint32_t operation_type; / operation_type is of MES_LOG_OPERATION type,
    pub reserved_operation_type_bits: u32,
    pub context_state_change: MES_LOG_CONTEXT_STATE_CHANGE,
    pub queue_new_work: MES_LOG_QUEUE_NEW_WORK,
    pub queue_unwait_sync_object: MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT,
    pub queue_no_more_work: MES_LOG_QUEUE_NO_MORE_WORK,
    pub queue_wait_sync_object: MES_LOG_QUEUE_WAIT_SYNC_OBJECT,
    pub all: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_LOG_BUFFER {
    pub header: MES_LOG_ENTRY_HEADER,
    pub entries: [MES_LOG_ENTRY_DATA; 1],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_SWIP_TO_HWIP_DEF {
    MES_MAX_HWIP_SEGMENT = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI_SET_HW_RESOURCES {
    pub header: MES_API_HEADER,
    pub vmid_mask_mmhub: u32,
    pub vmid_mask_gfxhub: u32,
    pub gds_size: u32,
    pub paging_vmid: u32,
    pub compute_hqd_mask: [u32; MAX_COMPUTE_PIPES],
    pub gfx_hqd_mask: [u32; MAX_GFX_PIPES],
    pub sdma_hqd_mask: [u32; MAX_SDMA_PIPES],
    pub aggregated_doorbells: [u32; AMD_PRIORITY_NUM_LEVELS],
    pub g_sch_ctx_gpu_mc_ptr: u64,
    pub query_status_fence_gpu_mc_ptr: u64,
    pub gc_base: [u32; MES_MAX_HWIP_SEGMENT],
    pub mmhub_base: [u32; MES_MAX_HWIP_SEGMENT],
    pub osssys_base: [u32; MES_MAX_HWIP_SEGMENT],
    pub api_status: MES_API_STATUS,
    pub 1: uint32_t disable_reset :,
    pub 1: uint32_t use_different_vmid_compute :,
    pub 1: uint32_t disable_mes_log :,
    pub 1: uint32_t apply_mmhub_pgvm_invalidate_ack_loss_wa :,
    pub 1: uint32_t apply_grbm_remote_register_dummy_read_wa :,
    pub 1: uint32_t second_gfx_pipe_enabled :,
    pub 1: uint32_t enable_level_process_quantum_check :,
    pub 1: uint32_t legacy_sch_mode :,
    pub 1: uint32_t disable_add_queue_wptr_mc_addr :,
    pub 1: uint32_t enable_mes_event_int_logging :,
    pub 1: uint32_t enable_reg_active_poll :,
    pub 1: uint32_t use_disable_queue_in_legacy_uq_preemption :,
    pub 1: uint32_t send_write_data :,
    pub 1: uint32_t os_tdr_timeout_override :,
    pub 1: uint32_t use_rs64mem_for_proc_gang_ctx :,
    pub 1: uint32_t use_add_queue_unmap_flag_addr :,
    pub 1: uint32_t enable_mes_sch_stb_log :,
    pub 1: uint32_t limit_single_process :,
    pub :1: uint32_t is_strix_tmz_wa_enabled,
    pub 2: uint32_t enable_lr_compute_wa :,
    pub 1: uint32_t enable_compute_pipe_reset :,
    pub 10: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI_SET_HW_RESOURCES_1 {
    pub header: MES_API_HEADER,
    pub api_status: MES_API_STATUS,
    pub timestamp: u64,
    pub 1: uint32_t enable_mes_info_ctx :,
    pub 31: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__ADD_QUEUE {
    pub header: MES_API_HEADER,
    pub process_id: u32,
    pub page_table_base_addr: u64,
    pub process_va_start: u64,
    pub process_va_end: u64,
    pub process_quantum: u64,
    pub process_context_addr: u64,
    pub gang_quantum: u64,
    pub gang_context_addr: u64,
    pub inprocess_gang_priority: u32,
    pub gang_global_priority_level: MES_AMD_PRIORITY_LEVEL,
    pub doorbell_offset: u32,
    pub mqd_addr: u64,
    pub wptr_addr: u64,
    pub h_context: u64,
    pub h_queue: u64,
    pub queue_type: MES_QUEUE_TYPE,
    pub gds_base: u32,
    pub gds_size: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub oa_mask: u32,
    pub trap_handler_addr: u64,
    pub vm_context_cntl: u32,
    pub 1: uint32_t paging :,
    pub 4: uint32_t debug_vmid :,
    pub 1: uint32_t program_gds :,
    pub 1: uint32_t is_gang_suspended :,
    pub 1: uint32_t is_tmz_queue :,
    pub 1: uint32_t map_kiq_utility_queue :,
    pub 1: uint32_t is_kfd_process :,
    pub 1: uint32_t trap_en :,
    pub 1: uint32_t is_aql_queue :,
    pub 1: uint32_t skip_process_ctx_clear :,
    pub 1: uint32_t map_legacy_kq :,
    pub 1: uint32_t exclusively_scheduled :,
    pub 1: uint32_t is_long_running :,
    pub 1: uint32_t is_dwm_queue :,
    pub 1: uint32_t is_video_blit_queue :,
    pub 14: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__REMOVE_QUEUE {
    pub header: MES_API_HEADER,
    pub doorbell_offset: u32,
    pub gang_context_addr: u64,
    pub 1: uint32_t unmap_legacy_gfx_queue :,
    pub 1: uint32_t unmap_kiq_utility_queue :,
    pub 1: uint32_t preempt_legacy_gfx_queue :,
    pub 1: uint32_t unmap_legacy_queue :,
    pub 1: uint32_t remove_queue_after_reset :,
    pub 27: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__SET_SCHEDULING_CONFIG {
    pub header: MES_API_HEADER,
// Grace period when preempting another priority band for this
// priority band. The value for idle priority band is ignored,
// as it never preempts other bands.
//
    pub grace_period_other_levels: [u64; AMD_PRIORITY_NUM_LEVELS],
// Default quantum for scheduling across processes within
// a priority band.
//
    pub process_quantum_for_level: [u64; AMD_PRIORITY_NUM_LEVELS],
// Default grace period for processes that preempt each other
// within a priority band.
//
    pub process_grace_period_same_level: [u64; AMD_PRIORITY_NUM_LEVELS],
// For normal level this field specifies the target GPU
// percentage in situations when it's starved by the high level.
// Valid values are between 0 and 50, with the default being 10.
//
    pub normal_yield_percent: u32,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__PERFORM_YIELD {
    pub header: MES_API_HEADER,
    pub dummy: u32,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__CHANGE_GANG_PRIORITY_LEVEL {
    pub header: MES_API_HEADER,
    pub inprocess_gang_priority: u32,
    pub gang_global_priority_level: MES_AMD_PRIORITY_LEVEL,
    pub gang_quantum: u64,
    pub gang_context_addr: u64,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__SUSPEND {
    pub header: MES_API_HEADER,
// false - suspend all gangs; true - specific gang
    pub 1: uint32_t suspend_all_gangs :,
    pub 31: uint32_t reserved :,
}

// gang_context_addr is valid only if suspend_all = false
#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__RESUME {
    pub header: MES_API_HEADER,
// false - resume all gangs; true - specified gang
    pub 1: uint32_t resume_all_gangs :,
    pub 31: uint32_t reserved :,
}

// valid only if resume_all_gangs = false
#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__RESET {
    pub header: MES_API_HEADER,
// Only reset the queue given by doorbell_offset (not entire gang)
    pub 1: uint32_t reset_queue_only :,
// Hang detection first then reset any queues that are hung
    pub 1: uint32_t hang_detect_then_reset :,
// Only do hang detection (no reset)
    pub 1: uint32_t hang_detect_only :,
// Rest HP and LP kernel queues not managed by MES
    pub 1: uint32_t reset_legacy_gfx :,
    pub 28: uint32_t reserved :,
}

// valid only if reset_queue_only = true
// valid only if hang_detect_then_reset = true
// valid only if reset_legacy_gfx = true
#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__SET_LOGGING_BUFFER {
    pub header: MES_API_HEADER,
// There are separate log buffers for each queue type
    pub log_type: MES_QUEUE_TYPE,
// Log buffer GPU Address
    pub logging_buffer_addr: u64,
// number of entries in the log buffer
    pub number_of_entries: u32,
// Entry index at which CPU interrupt needs to be signalled
    pub interrupt_entry: u32,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MES_API_QUERY_MES_OPCODE {
    MES_API_QUERY_MES__GET_CTX_ARRAY_SIZE,
    MES_API_QUERY_MES__GET_CAPS = MES_API_QUERY_MES__GET_CTX_ARRAY_SIZE,
    MES_API_QUERY_MES__CHECK_HEALTHY,
    MES_API_QUERY_MES__MAX,
}

//
// Obsolete
// to be removed once KMD stopped refering to it.
// use MES_API_QUERY_MES__CAPS instead
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_API_QUERY_MES__CTX_ARRAY_SIZE {
    pub proc_ctx_array_size_addr: u64,
    pub gang_ctx_array_size_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_API_QUERY_MES__HEALTHY_CHECK {
    pub healthy_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MES_API_QUERY_MES__CAPS {
    pub proc_ctx_array_size_addr: u64,
    pub gang_ctx_array_size_addr: u64,
    pub features_enablement_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__QUERY_MES_STATUS {
    pub header: MES_API_HEADER,
    pub subopcode: MES_API_QUERY_MES_OPCODE,
    pub api_status: MES_API_STATUS,
    pub timestamp: u64,
    pub ctx_array_size: MES_API_QUERY_MES__CTX_ARRAY_SIZE,
    pub caps: MES_API_QUERY_MES__CAPS,
    pub healthy_check: MES_API_QUERY_MES__HEALTHY_CHECK,
    pub data: [u32; QUERY_MES_MAX_SIZE_IN_DWORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__PROGRAM_GDS {
    pub header: MES_API_HEADER,
    pub process_context_addr: u64,
    pub gds_base: u32,
    pub gds_size: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub oa_mask: u32,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__SET_DEBUG_VMID {
    pub header: MES_API_HEADER,
    pub api_status: MES_API_STATUS,
    pub 1: uint32_t use_gds :,
    pub 2: uint32_t operation :,
    pub 29: uint32_t reserved :,
    pub flags: },
    pub u32All: u32,
}

// output addr of the acquired vmid value
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MESAPI_MISC_OPCODE {
    MESAPI_MISC__WRITE_REG,
    MESAPI_MISC__INV_GART,
    MESAPI_MISC__QUERY_STATUS,
    MESAPI_MISC__READ_REG,
    MESAPI_MISC__WAIT_REG_MEM,
    MESAPI_MISC__SET_SHADER_DEBUGGER,
    MESAPI_MISC__NOTIFY_WORK_ON_UNMAPPED_QUEUE,
    MESAPI_MISC__NOTIFY_TO_UNMAP_PROCESSES,
    MESAPI_MISC__CHANGE_CONFIG,
    MESAPI_MISC__LAUNCH_CLEANER_SHADER,

    MESAPI_MISC__MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WRITE_REG {
    pub reg_offset: u32,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct READ_REG {
    pub reg_offset: u32,
    pub buffer_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WRM_OPERATION {
    WRM_OPERATION__WAIT_REG_MEM,
    WRM_OPERATION__WR_WAIT_WR_REG,
    WRM_OPERATION__MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WAIT_REG_MEM {
    pub op: WRM_OPERATION,
    pub reference: u32,
    pub mask: u32,
    pub reg_offset1: u32,
    pub reg_offset2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INV_GART {
    pub inv_range_va_start: u64,
    pub inv_range_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QUERY_STATUS {
    pub context_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SET_SHADER_DEBUGGER {
    pub process_context_addr: u64,
    pub /: *mut *mut uint32_t single_memop : 1; / SQ_DEBUG.single_memop,
    pub /: *mut *mut uint32_t single_alu_op : 1; / SQ_DEBUG.single_alu_op,
    pub 29: uint32_t reserved :,
    pub 1: uint32_t process_ctx_flush :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MESAPI_MISC__CHANGE_CONFIG_OPTION {
    MESAPI_MISC__CHANGE_CONFIG_OPTION_LIMIT_SINGLE_PROCESS = 0,
    MESAPI_MISC__CHANGE_CONFIG_OPTION_ENABLE_HWS_LOGGING_BUFFER = 1,
    MESAPI_MISC__CHANGE_CONFIG_OPTION_CHANGE_TDR_CONFIG    = 2,

    MESAPI_MISC__CHANGE_CONFIG_OPTION_MAX = 0x1F
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHANGE_CONFIG {
    pub opcode: MESAPI_MISC__CHANGE_CONFIG_OPTION,
    pub 1: uint32_t limit_single_process :,
    pub 1: uint32_t enable_hws_logging_buffer :,
    pub 31: uint32_t reserved :,
    pub bits: },
    pub all: u32,
    pub option: },
    pub tdr_level: u32,
    pub tdr_delay: u32,
    pub tdr_config: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__MISC {
    pub header: MES_API_HEADER,
    pub opcode: MESAPI_MISC_OPCODE,
    pub api_status: MES_API_STATUS,
    pub write_reg: WRITE_REG,
    pub inv_gart: INV_GART,
    pub query_status: QUERY_STATUS,
    pub read_reg: READ_REG,
    pub wait_reg_mem: WAIT_REG_MEM,
    pub set_shader_debugger: SET_SHADER_DEBUGGER,
    pub queue_sch_level: MES_AMD_PRIORITY_LEVEL,
    pub change_config: CHANGE_CONFIG,
    pub data: [u32; MISC_DATA_MAX_SIZE_IN_DWORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI__UPDATE_ROOT_PAGE_TABLE {
    pub header: MES_API_HEADER,
    pub page_table_base_addr: u64,
    pub process_context_addr: u64,
    pub api_status: MES_API_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MESAPI_AMD_LOG {
    pub header: MES_API_HEADER,
    pub p_buffer_memory: u64,
    pub p_buffer_size_used: u64,
    pub api_status: MES_API_STATUS,
}

