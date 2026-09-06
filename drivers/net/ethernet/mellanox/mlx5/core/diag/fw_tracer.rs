//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/diag/fw_tracer.h
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
// Copyright (c) 2018, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const STRINGS_DB_SECTIONS_NUM: c_int = 8;
pub const STRINGS_DB_READ_SIZE_BYTES: c_int = 256;
pub const STRINGS_DB_LEFTOVER_SIZE_BYTES: c_int = 64;
pub const TRACER_BUFFER_PAGE_NUM: c_int = 64;
pub const TRACER_BUFFER_CHUNK: c_int = 4096;

pub const TRACER_BLOCK_SIZE_BYTE: c_int = 256;
pub const TRACES_PER_BLOCK: c_int = 32;
pub const TRACE_STR_MSG: c_int = 256;
pub const SAVED_TRACES_NUM: c_int = 8192;
pub const TRACER_MAX_PARAMS: c_int = 7;
pub const MESSAGE_HASH_BITS: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fw_trace_data {
    pub timestamp: u64,
    pub lost: bool,
    pub event_id: u8,
    pub msg: [c_char; TRACE_STR_MSG],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fw_tracer_state {
    MLX5_TRACER_STATE_UP = BIT(0),
    MLX5_TRACER_RECREATE_DB = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fw_tracer {
    pub dev: *mut mlx5_core_dev,
    pub nb: mlx5_nb,
    pub owner: bool,
    pub trc_ver: u8,
    pub work_queue: *mut workqueue_struct,
    pub ownership_change_work: work_struct,
    pub read_fw_strings_work: work_struct,
// Strings DB
    pub first_string_trace: u8,
    pub num_string_trace: u8,
    pub num_string_db: u32,
    pub base_address_out: [u32; STRINGS_DB_SECTIONS_NUM],
    pub size_out: [u32; STRINGS_DB_SECTIONS_NUM],
    pub buffer: [*mut c_void; STRINGS_DB_SECTIONS_NUM],
    pub loaded: bool,
    pub str_db: },
// Log Buffer
    pub pdn: u32,
    pub log_buf: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
    pub mkey: u32,
    pub consumer_index: u32,
    pub buff: },
// Saved Traces Array
    pub straces: [mlx5_fw_trace_data; SAVED_TRACES_NUM],
    pub saved_traces_index: u32,
    pub /: *mut *mut mutex lock; / Protect st_arr access,
    pub st_arr: },
    pub last_timestamp: u64,
    pub handle_traces_work: work_struct,
    pub hash: [hlist_head; MESSAGE_HASH_SIZE],
    pub ready_strings_list: list_head,
    pub update_db_work: work_struct,
    pub /: *mut *mut mutex state_lock; / Synchronize update work with reload flows,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_string_format {
    pub string: *mut c_char,
    pub params: [c_int; TRACER_MAX_PARAMS],
    pub num_of_params: c_int,
    pub last_param_num: c_int,
    pub event_id: u8,
    pub tmsn: u32,
    pub hlist: hlist_node,
    pub list: list_head,
    pub timestamp: u32,
    pub lost: bool,
    pub invalid_string: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fw_tracer_ownership_state {
    MLX5_FW_TRACER_RELEASE_OWNERSHIP,
    MLX5_FW_TRACER_ACQUIRE_OWNERSHIP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tracer_ctrl_fields_select {
    TRACE_STATUS = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tracer_event_type {
    TRACER_EVENT_TYPE_STRING,
    TRACER_EVENT_TYPE_TIMESTAMP = 0xFF,
    TRACER_EVENT_TYPE_UNRECOGNIZED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tracing_mode {
    TRACE_TO_MEMORY = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_timestamp_event {
    pub timestamp: u64,
    pub unreliable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_string_event {
    pub timestamp: u32,
    pub tmsn: u32,
    pub tdsn: u32,
    pub string_param: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracer_event {
    pub lost_event: bool,
    pub type: u32,
    pub event_id: u8,
    pub string_event: tracer_string_event,
    pub timestamp_event: tracer_timestamp_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tracer_event_bits {
    pub lost: [u8; 0x1],
    pub timestamp: [u8; 0x7],
    pub event_id: [u8; 0x8],
    pub event_data: [u8; 0x30],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tracer_string_event_bits {
    pub lost: [u8; 0x1],
    pub timestamp: [u8; 0x7],
    pub event_id: [u8; 0x8],
    pub tmsn: [u8; 0xd],
    pub tdsn: [u8; 0x3],
    pub string_param: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tracer_timestamp_event_bits {
    pub timestamp7_0: [u8; 0x8],
    pub event_id: [u8; 0x8],
    pub urts: [u8; 0x3],
    pub timestamp52_40: [u8; 0xd],
    pub timestamp39_8: [u8; 0x20],
}

extern "C" {
    pub fn mlx5_fw_tracer_init(tracer: *mut mlx5_fw_tracer) -> c_int;
}
extern "C" {
    pub fn mlx5_fw_tracer_cleanup(tracer: *mut mlx5_fw_tracer);
}
extern "C" {
    pub fn mlx5_fw_tracer_destroy(tracer: *mut mlx5_fw_tracer);
}
extern "C" {
    pub fn mlx5_fw_tracer_trigger_core_dump_general(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_fw_tracer_reload(tracer: *mut mlx5_fw_tracer) -> c_int;
}
