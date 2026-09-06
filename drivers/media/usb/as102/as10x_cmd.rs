//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/as102/as10x_cmd.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2008 Pierrick Hascoet <pierrick.hascoet@abilis.com>
//

//
// MACRO DEFINITIONS
//

pub const SERVICE_PROG_ID: c_uint = 0x0002;
pub const SERVICE_PROG_VERSION: c_uint = 0x0001;
pub const HIER_NONE: c_uint = 0x00;
pub const HIER_LOW_PRIORITY: c_uint = 0x01;

// context request types
pub const GET_CONTEXT_DATA: c_int = 1;
pub const SET_CONTEXT_DATA: c_int = 2;
// ODSP suspend modes
pub const CFG_MODE_ODSP_RESUME: c_int = 0;
pub const CFG_MODE_ODSP_SUSPEND: c_int = 1;
// Dump memory size
pub const DUMP_BLOCK_SIZE_MAX: c_uint = 0x20;
//
// TYPE DEFINITION
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum control_proc {
    CONTROL_PROC_TURNON			= 0x0001,
    CONTROL_PROC_TURNON_RSP			= 0x0100,
    CONTROL_PROC_SET_REGISTER		= 0x0002,
    CONTROL_PROC_SET_REGISTER_RSP		= 0x0200,
    CONTROL_PROC_GET_REGISTER		= 0x0003,
    CONTROL_PROC_GET_REGISTER_RSP		= 0x0300,
    CONTROL_PROC_SETTUNE			= 0x000A,
    CONTROL_PROC_SETTUNE_RSP		= 0x0A00,
    CONTROL_PROC_GETTUNESTAT		= 0x000B,
    CONTROL_PROC_GETTUNESTAT_RSP		= 0x0B00,
    CONTROL_PROC_GETTPS			= 0x000D,
    CONTROL_PROC_GETTPS_RSP			= 0x0D00,
    CONTROL_PROC_SETFILTER			= 0x000E,
    CONTROL_PROC_SETFILTER_RSP		= 0x0E00,
    CONTROL_PROC_REMOVEFILTER		= 0x000F,
    CONTROL_PROC_REMOVEFILTER_RSP		= 0x0F00,
    CONTROL_PROC_GET_IMPULSE_RESP		= 0x0012,
    CONTROL_PROC_GET_IMPULSE_RESP_RSP	= 0x1200,
    CONTROL_PROC_START_STREAMING		= 0x0013,
    CONTROL_PROC_START_STREAMING_RSP	= 0x1300,
    CONTROL_PROC_STOP_STREAMING		= 0x0014,
    CONTROL_PROC_STOP_STREAMING_RSP		= 0x1400,
    CONTROL_PROC_GET_DEMOD_STATS		= 0x0015,
    CONTROL_PROC_GET_DEMOD_STATS_RSP	= 0x1500,
    CONTROL_PROC_ELNA_CHANGE_MODE		= 0x0016,
    CONTROL_PROC_ELNA_CHANGE_MODE_RSP	= 0x1600,
    CONTROL_PROC_ODSP_CHANGE_MODE		= 0x0017,
    CONTROL_PROC_ODSP_CHANGE_MODE_RSP	= 0x1700,
    CONTROL_PROC_AGC_CHANGE_MODE		= 0x0018,
    CONTROL_PROC_AGC_CHANGE_MODE_RSP	= 0x1800,

    CONTROL_PROC_CONTEXT			= 0x00FC,
    CONTROL_PROC_CONTEXT_RSP		= 0xFC00,
    CONTROL_PROC_DUMP_MEMORY		= 0x00FD,
    CONTROL_PROC_DUMP_MEMORY_RSP		= 0xFD00,
    CONTROL_PROC_DUMPLOG_MEMORY		= 0x00FE,
    CONTROL_PROC_DUMPLOG_MEMORY_RSP		= 0xFE00,
    CONTROL_PROC_TURNOFF			= 0x00FF,
    CONTROL_PROC_TURNOFF_RSP		= 0xFF00
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_turn_on {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_turn_off {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub err: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_set_tune {
// request
// request identifier
    pub proc_id: __le16,
// tune params
    pub args: as10x_tune_args,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_get_tune_status {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
// tune status
    pub sts: as10x_tune_status,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_get_tps {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
// tps details
    pub tps: as10x_tps,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_common {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_add_pid_filter {
// request
// request identifier
    pub proc_id: __le16,
// PID to filter
    pub pid: __le16,
// stream type (MPE, PSI/SI or PES )
    pub stream_type: u8,
// PID index in filter table
    pub idx: u8,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
// Filter id
    pub filter_id: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_del_pid_filter {
// request
// request identifier
    pub proc_id: __le16,
// PID to remove
    pub pid: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// response error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_start_streaming {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_stop_streaming {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_get_demod_stats {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
// demod stats
    pub stats: as10x_demod_stats,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_get_impulse_resp {
// request
// request identifier
    pub proc_id: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
// impulse response ready
    pub is_ready: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_fw_context {
// request
// request identifier
    pub proc_id: __le16,
// value to write (for set context)
    pub reg_val: as10x_register_value,
// context tag
    pub tag: __le16,
// context request type
    pub type: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// value read (for get context)
    pub reg_val: as10x_register_value,
// context request type
    pub type: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_set_register {
// request
// response identifier
    pub proc_id: __le16,
// register description
    pub reg_addr: as10x_register_addr,
// register content
    pub reg_val: as10x_register_value,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_get_register {
// request
// response identifier
    pub proc_id: __le16,
// register description
    pub reg_addr: as10x_register_addr,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
// register content
    pub reg_val: as10x_register_value,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_cfg_change_mode {
// request
// request identifier
    pub proc_id: __le16,
// mode
    pub mode: u8,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_cmd_header_t {
    pub req_id: __le16,
    pub prog: __le16,
    pub version: __le16,
    pub data_len: __le16,
    pub __packed: },
pub const DUMP_BLOCK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_dump_memory {
// request
// request identifier
    pub proc_id: __le16,
// dump memory type request
    pub dump_req: u8,
// register description
    pub reg_addr: as10x_register_addr,
// nb blocks to read
    pub num_blocks: __le16,
    pub req: } __packed,
// response
// response identifier
    pub proc_id: __le16,
// error
    pub error: u8,
// dump response
    pub dump_rsp: u8,
// data
    pub data8: [u8; DUMP_BLOCK_SIZE],
    pub sizeof(__le16)]: __le16 data16[DUMP_BLOCK_SIZE /,
    pub sizeof(__le32)]: __le32 data32[DUMP_BLOCK_SIZE /,
    pub u: } __packed,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_dumplog_memory {
// request identifier
    pub proc_id: __le16,
// dump memory type request
    pub dump_req: u8,
    pub req: } __packed,
// request identifier
    pub proc_id: __le16,
// error
    pub error: u8,
// dump response
    pub dump_rsp: u8,
// dump data
    pub data: [u8; DUMP_BLOCK_SIZE],
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_raw_data {
// request
    pub proc_id: __le16,
    pub /]: *mut *mut - 2 / proc_id,
    pub req: } __packed,
// response
    pub proc_id: __le16,
    pub error: u8,
    pub /]: *mut *mut *mut *mut - 2 / proc_id / - 1 / rc,
    pub rsp: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_cmd_t {
    pub header: as10x_cmd_header_t,
    pub turn_on: as10x_turn_on,
    pub turn_off: as10x_turn_off,
    pub set_tune: as10x_set_tune,
    pub get_tune_status: as10x_get_tune_status,
    pub get_tps: as10x_get_tps,
    pub common: as10x_common,
    pub add_pid_filter: as10x_add_pid_filter,
    pub del_pid_filter: as10x_del_pid_filter,
    pub start_streaming: as10x_start_streaming,
    pub stop_streaming: as10x_stop_streaming,
    pub get_demod_stats: as10x_get_demod_stats,
    pub get_impulse_rsp: as10x_get_impulse_resp,
    pub context: as10x_fw_context,
    pub set_register: as10x_set_register,
    pub get_register: as10x_get_register,
    pub cfg_change_mode: as10x_cfg_change_mode,
    pub dump_memory: as10x_dump_memory,
    pub dumplog_memory: as10x_dumplog_memory,
    pub raw_data: as10x_raw_data,
    pub body: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_token_cmd_t {
// token cmd
    pub c: as10x_cmd_t,
// token response
    pub r: as10x_cmd_t,
    pub __packed: },
//
// FUNCTION DECLARATION
//
    pub cmd_len): u16,
    pub proc_id): *mut *mut int as10x_rsp_parse(struct as10x_cmd_t r, uint16_t,
// as10x cmd
    pub adap): *mut int as10x_cmd_turn_on(struct as10x_bus_adapter_t,
    pub adap): *mut int as10x_cmd_turn_off(struct as10x_bus_adapter_t,
    pub ptune): *mut as10x_tune_args,
    pub pstatus): *mut as10x_tune_status,
    pub ptps): *mut as10x_tps,
    pub pdemod_stats): *mut as10x_demod_stats,
    pub is_ready): *mut u8,
// as10x cmd stream
    pub filter): *mut as10x_ts_filter,
    pub pid_value): u16,
    pub adap): *mut int as10x_cmd_start_streaming(struct as10x_bus_adapter_t,
    pub adap): *mut int as10x_cmd_stop_streaming(struct as10x_bus_adapter_t,
// as10x cmd cfg
    pub value): u32,
    pub pvalue): *mut u32,
    pub mode): *mut *mut int as10x_cmd_eLNA_change_mode(struct as10x_bus_adapter_t adap, uint8_t,
    pub proc_id): *mut *mut int as10x_context_rsp_parse(struct as10x_cmd_t prsp, uint16_t,
