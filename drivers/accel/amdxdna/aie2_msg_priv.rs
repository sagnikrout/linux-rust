//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie2_msg_priv.h
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
pub enum aie2_msg_opcode {
    MSG_OP_CREATE_CONTEXT              = 0x2,
    MSG_OP_DESTROY_CONTEXT             = 0x3,
    MSG_OP_GET_TELEMETRY               = 0x4,
    MSG_OP_SYNC_BO                     = 0x7,
    MSG_OP_EXECUTE_BUFFER_CF           = 0xC,
    MSG_OP_QUERY_COL_STATUS            = 0xD,
    MSG_OP_QUERY_AIE_TILE_INFO         = 0xE,
    MSG_OP_QUERY_AIE_VERSION           = 0xF,
    MSG_OP_EXEC_DPU                    = 0x10,
    MSG_OP_CONFIG_CU                   = 0x11,
    MSG_OP_CHAIN_EXEC_BUFFER_CF        = 0x12,
    MSG_OP_CHAIN_EXEC_DPU              = 0x13,
    MSG_OP_CONFIG_DEBUG_BO             = 0x14,
    MSG_OP_CHAIN_EXEC_NPU              = 0x18,
    MSG_OP_MAX_XRT_OPCODE,
    MSG_OP_SUSPEND                     = 0x101,
    MSG_OP_RESUME                      = 0x102,
    MSG_OP_ASSIGN_MGMT_PASID           = 0x103,
    MSG_OP_INVOKE_SELF_TEST            = 0x104,
    MSG_OP_MAP_HOST_BUFFER             = 0x106,
    MSG_OP_GET_FIRMWARE_VERSION        = 0x108,
    MSG_OP_SET_RUNTIME_CONFIG          = 0x10A,
    MSG_OP_GET_RUNTIME_CONFIG          = 0x10B,
    MSG_OP_REGISTER_ASYNC_EVENT_MSG    = 0x10C,
    MSG_OP_UPDATE_PROPERTY             = 0x113,
    MSG_OP_GET_APP_HEALTH              = 0x114,
    MSG_OP_ADD_HOST_BUFFER             = 0x115,
    MSG_OP_GET_DEV_REVISION            = 0x117,
    MSG_OP_MAX_DRV_OPCODE,
    MSG_OP_GET_PROTOCOL_VERSION        = 0x301,
    MSG_OP_MAX_OPCODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie2_msg_status {
    AIE2_STATUS_SUCCESS				= 0x0,
// AIE Error codes
    AIE2_STATUS_AIE_SATURATION_ERROR		= 0x1000001,
    AIE2_STATUS_AIE_FP_ERROR			= 0x1000002,
    AIE2_STATUS_AIE_STREAM_ERROR			= 0x1000003,
    AIE2_STATUS_AIE_ACCESS_ERROR			= 0x1000004,
    AIE2_STATUS_AIE_BUS_ERROR			= 0x1000005,
    AIE2_STATUS_AIE_INSTRUCTION_ERROR		= 0x1000006,
    AIE2_STATUS_AIE_ECC_ERROR			= 0x1000007,
    AIE2_STATUS_AIE_LOCK_ERROR			= 0x1000008,
    AIE2_STATUS_AIE_DMA_ERROR			= 0x1000009,
    AIE2_STATUS_AIE_MEM_PARITY_ERROR		= 0x100000a,
    AIE2_STATUS_AIE_PWR_CFG_ERROR			= 0x100000b,
    AIE2_STATUS_AIE_BACKTRACK_ERROR			= 0x100000c,
    AIE2_STATUS_MAX_AIE_STATUS_CODE,
// MGMT ERT Error codes
    AIE2_STATUS_MGMT_ERT_SELF_TEST_FAILURE		= 0x2000001,
    AIE2_STATUS_MGMT_ERT_HASH_MISMATCH,
    AIE2_STATUS_MGMT_ERT_NOAVAIL,
    AIE2_STATUS_MGMT_ERT_INVALID_PARAM,
    AIE2_STATUS_MGMT_ERT_ENTER_SUSPEND_FAILURE,
    AIE2_STATUS_MGMT_ERT_BUSY,
    AIE2_STATUS_MGMT_ERT_APPLICATION_ACTIVE,
    MAX_MGMT_ERT_STATUS_CODE,
// APP ERT Error codes
    AIE2_STATUS_APP_ERT_FIRST_ERROR			= 0x3000001,
    AIE2_STATUS_APP_INVALID_INSTR,
    AIE2_STATUS_APP_LOAD_PDI_FAIL,
    MAX_APP_ERT_STATUS_CODE,
// NPU RTOS Error Codes
    AIE2_STATUS_INVALID_INPUT_BUFFER		= 0x4000001,
    AIE2_STATUS_INVALID_COMMAND,
    AIE2_STATUS_INVALID_PARAM,
    AIE2_STATUS_INVALID_OPERATION			= 0x4000006,
    AIE2_STATUS_ASYNC_EVENT_MSGS_FULL,
    AIE2_STATUS_MAX_RTOS_STATUS_CODE,
    MAX_AIE2_STATUS_CODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct assign_mgmt_pasid_req {
    pub pasid: __u16,
    pub reserved: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assign_mgmt_pasid_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_host_buffer_req {
    pub context_id: __u32,
    pub buf_addr: __u64,
    pub buf_size: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_host_buffer_resp {
    pub status: aie2_msg_status,
    pub __packed: },
pub const MAX_CQ_PAIRS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_info {
    pub head_addr: __u32,
    pub tail_addr: __u32,
    pub buf_addr: __u32,
    pub buf_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_pair {
    pub x2i_q: cq_info,
    pub i2x_q: cq_info,
}

pub const PRIORITY_REALTIME: c_int = 1;
pub const PRIORITY_HIGH: c_int = 2;
pub const PRIORITY_NORMAL: c_int = 3;
pub const PRIORITY_LOW: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_ctx_req {
    pub aie_type: __u32,
    pub start_col: __u8,
    pub num_col: __u8,
    pub num_unused_col: __u8,
    pub reserved: __u8,
    pub num_cq_pairs_requested: __u8,
    pub reserved1: __u8,
    pub pasid: __u16,
    pub pad: [__u32; 2],
    pub sec_comm_target_type: __u32,
    pub context_priority: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_ctx_resp {
    pub status: aie2_msg_status,
    pub context_id: __u32,
    pub msix_id: __u16,
    pub num_cq_pairs_allocated: __u8,
    pub reserved: __u8,
    pub cq_pair: [cq_pair; MAX_CQ_PAIRS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct destroy_ctx_req {
    pub context_id: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct destroy_ctx_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum telemetry_type {
    TELEMETRY_TYPE_DISABLED,
    TELEMETRY_TYPE_HEALTH,
    TELEMETRY_TYPE_ERROR_INFO,
    TELEMETRY_TYPE_PROFILING,
    TELEMETRY_TYPE_DEBUG,
    MAX_TELEMETRY_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_telemetry_req {
    pub type: telemetry_type,
    pub buf_addr: __u64,
    pub buf_size: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_telemetry_resp {
    pub major: __u32,
    pub minor: __u32,
    pub size: __u32,
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct execute_buffer_req {
    pub cu_idx: __u32,
    pub payload: [__u32; 19],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exec_dpu_req {
    pub inst_buf_addr: __u64,
    pub inst_size: __u32,
    pub inst_prop_cnt: __u32,
    pub cu_idx: __u32,
    pub payload: [__u32; 35],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exec_npu_type {
    EXEC_NPU_TYPE_NON_ELF		= 0x1,
    EXEC_NPU_TYPE_PARTIAL_ELF	= 0x2,
    EXEC_NPU_TYPE_PREEMPT		= 0x3,
    EXEC_NPU_TYPE_ELF		= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union exec_req {
    pub ebuf: execute_buffer_req,
    pub dpu_req: exec_dpu_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execute_buffer_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_tile_info {
    pub size: __u32,
    pub major: __u16,
    pub minor: __u16,
    pub cols: __u16,
    pub rows: __u16,
    pub core_rows: __u16,
    pub mem_rows: __u16,
    pub shim_rows: __u16,
    pub core_row_start: __u16,
    pub mem_row_start: __u16,
    pub shim_row_start: __u16,
    pub core_dma_channels: __u16,
    pub mem_dma_channels: __u16,
    pub shim_dma_channels: __u16,
    pub core_locks: __u16,
    pub mem_locks: __u16,
    pub shim_locks: __u16,
    pub core_events: __u16,
    pub mem_events: __u16,
    pub shim_events: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_tile_info_req {
    pub reserved: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_tile_info_resp {
    pub status: aie2_msg_status,
    pub info: aie_tile_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_version_info_req {
    pub reserved: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_version_info_resp {
    pub status: aie2_msg_status,
    pub major: __u16,
    pub minor: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_column_info_req {
    pub dump_buff_addr: __u64,
    pub dump_buff_size: __u32,
    pub num_cols: __u32,
    pub aie_bitmap: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_column_info_resp {
    pub status: aie2_msg_status,
    pub size: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_req {
    pub place_holder: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resume_req {
    pub place_holder: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resume_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct check_header_hash_req {
    pub hash_high: __u64,
    pub hash_low: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct check_header_hash_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct query_error_req {
    pub buf_addr: __u64,
    pub buf_size: __u32,
    pub next_row: __u32,
    pub next_column: __u32,
    pub next_module: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct query_error_resp {
    pub status: aie2_msg_status,
    pub num_err: __u32,
    pub has_next_err: __u32,
    pub next_row: __u32,
    pub next_column: __u32,
    pub next_module: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protocol_version_req {
    pub reserved: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protocol_version_resp {
    pub status: aie2_msg_status,
    pub major: __u32,
    pub minor: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware_version_req {
    pub reserved: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware_version_resp {
    pub status: aie2_msg_status,
    pub major: __u32,
    pub minor: __u32,
    pub sub: __u32,
    pub build: __u32,
    pub __packed: },
pub const MAX_NUM_CUS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_cu_req {
    pub num_cus: __u32,
    pub cfgs: [__u32; MAX_NUM_CUS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_cu_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_runtime_cfg_req {
    pub type: __u32,
    pub value: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_runtime_cfg_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_runtime_cfg_req {
    pub type: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_runtime_cfg_resp {
    pub status: aie2_msg_status,
    pub value: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum async_event_type {
    ASYNC_EVENT_TYPE_AIE_ERROR,
    ASYNC_EVENT_TYPE_EXCEPTION,
    MAX_ASYNC_EVENT_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_event_msg_req {
    pub buf_addr: __u64,
    pub buf_size: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_event_msg_resp {
    pub status: aie2_msg_status,
    pub type: async_event_type,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_slot_execbuf_cf {
    pub cu_idx: __u32,
    pub arg_cnt: __u32,
    pub __counted_by(arg_cnt): __u32 args[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_slot_dpu {
    pub inst_buf_addr: __u64,
    pub inst_size: __u32,
    pub inst_prop_cnt: __u32,
    pub cu_idx: __u32,
    pub arg_cnt: __u32,

    pub __counted_by(arg_cnt): __u32 args[],
}

pub const AIE2_EXEC_BUFFER_KERNEL_OP_TXN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_slot_npu {
    pub type: exec_npu_type,
    pub inst_buf_addr: u64,
    pub save_buf_addr: u64,
    pub restore_buf_addr: u64,
    pub inst_size: u32,
    pub save_size: u32,
    pub restore_size: u32,
    pub inst_prop_cnt: u32,
    pub cu_idx: u32,
    pub arg_cnt: u32,
    pub __counted_by(arg_cnt): u32 args[],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_req {
    pub buf_addr: __u64,
    pub buf_size: __u32,
    pub count: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_npu_req {
    pub flags: u32,
    pub reserved: u32,
    pub buf_addr: u64,
    pub buf_size: u32,
    pub count: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union exec_chain_req {
    pub npu_req: cmd_chain_npu_req,
    pub req: cmd_chain_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_chain_resp {
    pub status: aie2_msg_status,
    pub fail_cmd_idx: __u32,
    pub fail_cmd_status: aie2_msg_status,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_bo_req {
    pub src_addr: __u64,
    pub dst_addr: __u64,
    pub size: __u32,
pub const SYNC_BO_DEV_MEM: c_int = 0;
pub const SYNC_BO_HOST_MEM: c_int = 2;
    pub type: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_bo_resp {
    pub status: aie2_msg_status,
    pub __packed: },
pub const DEBUG_BO_UNREGISTER: c_int = 0;
pub const DEBUG_BO_REGISTER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_debug_bo_req {
    pub offset: __u64,
    pub size: __u64,
//
// config operations.
// DEBUG_BO_REGISTER: Register debug buffer
// DEBUG_BO_UNREGISTER: Unregister debug buffer
//
    pub config: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_debug_bo_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fatal_error_info {
    pub /: *mut *mut __u32 fatal_type; / Fatal error type,
    pub /: *mut *mut __u32 exception_type; / Only valid if fatal_type is a specific value,
    pub /: *mut *mut __u32 exception_argument; / Argument based on exception type,
    pub /: *mut *mut __u32 exception_pc; / Program Counter at the time of the exception,
    pub /: *mut *mut __u32 app_module; / Error module name,
    pub /: *mut *mut __u32 task_index; / Index of the task in which the error occurred,
    pub reserved: [__u32; 127],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct app_health_report {
    pub major: __u16,
    pub minor: __u16,
    pub size: __u32,
    pub context_id: __u32,
//
// Program Counter (PC) of the last initiated DPU opcode, as reported by the ERT
// application. Before execution begins or after successful completion, the value is set
// to UINT_MAX. If execution halts prematurely due to an error, this field retains the
// opcode's PC value.
// Note: To optimize performance, the ERT may simplify certain aspects of reporting.
// Proper interpretation requires familiarity with the implementation details.
//
    pub dpu_pc: __u32,
//
// Index of the last initiated TXN opcode.
// Before execution starts or after successful completion, the value is set to UINT_MAX.
// If execution halts prematurely due to an error, this field retains the opcode's ID.
// Note: To optimize performance, the ERT may simplify certain aspects of reporting.
// Proper interpretation requires familiarity with the implementation details.
//
    pub txn_op_id: __u32,
// The PC of the context at the time of the report
    pub ctx_pc: __u32,
    pub fatal_info: fatal_error_info,
// Index of the most recently executed run list entry.
    pub run_list_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_app_health_req {
    pub context_id: __u32,
    pub buf_size: __u32,
    pub buf_addr: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_app_health_resp {
    pub status: aie2_msg_status,
    pub required_buffer_size: __u32,
    pub reserved: [__u32; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_property_req {
pub const UPDATE_PROPERTY_TIME_QUOTA: c_int = 0;
    pub type: __u32,
pub const AIE2_UPDATE_PROPERTY_ALL_CTX: c_uint = 0xFF;
    pub context_id: __u8,
    pub reserved: [__u8; 7],
    pub time_quota_us: __u32,
    pub reserved1: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_property_resp {
    pub status: aie2_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie2_dev_revision {
    AIE2_DEV_REVISION_STXA = 1,
    AIE2_DEV_REVISION_STXB,
    AIE2_DEV_REVISION_KRK1,
    AIE2_DEV_REVISION_KRK2,
    AIE2_DEV_REVISION_HALO,
    AIE2_DEV_REVISION_GPT1,
    AIE2_DEV_REVISION_GPT2,
    AIE2_DEV_REVISION_GPT3,
    AIE2_DEV_REVISION_UNKN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_dev_revision_req {
    pub place_holder: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_dev_revision_resp {
    pub status: aie2_msg_status,
    pub rev: aie2_dev_revision,
    pub raw_fuse_data: __u32,
    pub __packed: },
