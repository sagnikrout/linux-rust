//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/amdtee/amdtee_if.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
//
// This file has definitions related to Host and AMD-TEE Trusted OS interface.
// These definitions must match the definitions on the TEE side.
//

//
// TEE Param
//
pub const TEE_MAX_PARAMS: c_int = 4;
//
// struct memref - memory reference structure
// @buf_id:    buffer ID of the buffer mapped by TEE_CMD_ID_MAP_SHARED_MEM
// @offset:    offset in bytes from beginning of the buffer
// @size:      data size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memref {
    pub buf_id: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value {
    pub a: u32,
    pub b: u32,
}

//
// Parameters passed to open_session or invoke_command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union tee_op_param {
    pub mref: memref,
    pub val: value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_operation {
    pub param_types: u32,
    pub params: [tee_op_param; TEE_MAX_PARAMS],
}

// Must be same as in GP TEE specification
pub const TEE_OP_PARAM_TYPE_NONE: c_int = 0;
pub const TEE_OP_PARAM_TYPE_VALUE_INPUT: c_int = 1;
pub const TEE_OP_PARAM_TYPE_VALUE_OUTPUT: c_int = 2;
pub const TEE_OP_PARAM_TYPE_VALUE_INOUT: c_int = 3;
pub const TEE_OP_PARAM_TYPE_INVALID: c_int = 4;
pub const TEE_OP_PARAM_TYPE_MEMREF_INPUT: c_int = 5;
pub const TEE_OP_PARAM_TYPE_MEMREF_OUTPUT: c_int = 6;
pub const TEE_OP_PARAM_TYPE_MEMREF_INOUT: c_int = 7;

//
// TEE Commands
//
// The shared memory between rich world and secure world may be physically
// non-contiguous. Below structures are meant to describe a shared memory region
// via scatter/gather (sg) list
//
// struct tee_sg_desc - sg descriptor for a physically contiguous buffer
// @low_addr: [in] bits[31:0] of buffer's physical address. Must be 4KB aligned
// @hi_addr:  [in] bits[63:32] of the buffer's physical address
// @size:     [in] size in bytes (must be multiple of 4KB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_sg_desc {
    pub low_addr: u32,
    pub hi_addr: u32,
    pub size: u32,
}

//
// struct tee_sg_list - structure describing a scatter/gather list
// @count:   [in] number of sg descriptors
// @size:    [in] total size of all buffers in the list. Must be multiple of 4KB
// @buf:     [in] list of sg buffer descriptors
//
pub const TEE_MAX_SG_DESC: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_sg_list {
    pub count: u32,
    pub size: u32,
    pub buf: [tee_sg_desc; TEE_MAX_SG_DESC],
}

//
// struct tee_cmd_map_shared_mem - command to map shared memory
// @buf_id:    [out] return buffer ID value
// @sg_list:   [in] list describing memory to be mapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_map_shared_mem {
    pub buf_id: u32,
    pub sg_list: tee_sg_list,
}

//
// struct tee_cmd_unmap_shared_mem - command to unmap shared memory
// @buf_id:    [in] buffer ID of memory to be unmapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_unmap_shared_mem {
    pub buf_id: u32,
}

//
// struct tee_cmd_load_ta - load Trusted Application (TA) binary into TEE
// @low_addr:       [in] bits [31:0] of the physical address of the TA binary
// @hi_addr:        [in] bits [63:32] of the physical address of the TA binary
// @size:           [in] size of TA binary in bytes
// @ta_handle:      [out] return handle of the loaded TA
// @return_origin:  [out] origin of return code after TEE processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_load_ta {
    pub low_addr: u32,
    pub hi_addr: u32,
    pub size: u32,
    pub ta_handle: u32,
    pub return_origin: u32,
}

//
// struct tee_cmd_unload_ta - command to unload TA binary from TEE environment
// @ta_handle:    [in] handle of the loaded TA to be unloaded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_unload_ta {
    pub ta_handle: u32,
}

//
// struct tee_cmd_open_session - command to call TA_OpenSessionEntryPoint in TA
// @ta_handle:      [in] handle of the loaded TA
// @session_info:   [out] pointer to TA allocated session data
// @op:             [in/out] operation parameters
// @return_origin:  [out] origin of return code after TEE processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_open_session {
    pub ta_handle: u32,
    pub session_info: u32,
    pub op: tee_operation,
    pub return_origin: u32,
}

//
// struct tee_cmd_close_session - command to call TA_CloseSessionEntryPoint()
// in TA
// @ta_handle:      [in] handle of the loaded TA
// @session_info:   [in] pointer to TA allocated session data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_close_session {
    pub ta_handle: u32,
    pub session_info: u32,
}

//
// struct tee_cmd_invoke_cmd - command to call TA_InvokeCommandEntryPoint() in
// TA
// @ta_handle:     [in] handle of the loaded TA
// @cmd_id:        [in] TA command ID
// @session_info:  [in] pointer to TA allocated session data
// @op:            [in/out] operation parameters
// @return_origin: [out] origin of return code after TEE processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_cmd_invoke_cmd {
    pub ta_handle: u32,
    pub cmd_id: u32,
    pub session_info: u32,
    pub op: tee_operation,
    pub return_origin: u32,
}
