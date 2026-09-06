//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/hidp/hidp.h
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

// HIDP header masks
pub const HIDP_HEADER_TRANS_MASK: c_uint = 0xf0;
pub const HIDP_HEADER_PARAM_MASK: c_uint = 0x0f;
// HIDP transaction types
pub const HIDP_TRANS_HANDSHAKE: c_uint = 0x00;
pub const HIDP_TRANS_HID_CONTROL: c_uint = 0x10;
pub const HIDP_TRANS_GET_REPORT: c_uint = 0x40;
pub const HIDP_TRANS_SET_REPORT: c_uint = 0x50;
pub const HIDP_TRANS_GET_PROTOCOL: c_uint = 0x60;
pub const HIDP_TRANS_SET_PROTOCOL: c_uint = 0x70;
pub const HIDP_TRANS_GET_IDLE: c_uint = 0x80;
pub const HIDP_TRANS_SET_IDLE: c_uint = 0x90;
pub const HIDP_TRANS_DATA: c_uint = 0xa0;
pub const HIDP_TRANS_DATC: c_uint = 0xb0;
// HIDP handshake results
pub const HIDP_HSHK_SUCCESSFUL: c_uint = 0x00;
pub const HIDP_HSHK_NOT_READY: c_uint = 0x01;
pub const HIDP_HSHK_ERR_INVALID_REPORT_ID: c_uint = 0x02;
pub const HIDP_HSHK_ERR_UNSUPPORTED_REQUEST: c_uint = 0x03;
pub const HIDP_HSHK_ERR_INVALID_PARAMETER: c_uint = 0x04;
pub const HIDP_HSHK_ERR_UNKNOWN: c_uint = 0x0e;
pub const HIDP_HSHK_ERR_FATAL: c_uint = 0x0f;
// HIDP control operation parameters
pub const HIDP_CTRL_NOP: c_uint = 0x00;
pub const HIDP_CTRL_HARD_RESET: c_uint = 0x01;
pub const HIDP_CTRL_SOFT_RESET: c_uint = 0x02;
pub const HIDP_CTRL_SUSPEND: c_uint = 0x03;
pub const HIDP_CTRL_EXIT_SUSPEND: c_uint = 0x04;
pub const HIDP_CTRL_VIRTUAL_CABLE_UNPLUG: c_uint = 0x05;
// HIDP data transaction headers
pub const HIDP_DATA_RTYPE_MASK: c_uint = 0x03;
pub const HIDP_DATA_RSRVD_MASK: c_uint = 0x0c;
pub const HIDP_DATA_RTYPE_OTHER: c_uint = 0x00;
pub const HIDP_DATA_RTYPE_INPUT: c_uint = 0x01;
pub const HIDP_DATA_RTYPE_OUPUT: c_uint = 0x02;
pub const HIDP_DATA_RTYPE_FEATURE: c_uint = 0x03;
// HIDP protocol header parameters
pub const HIDP_PROTO_BOOT: c_uint = 0x00;
pub const HIDP_PROTO_REPORT: c_uint = 0x01;
// HIDP ioctl defines

pub const HIDP_VIRTUAL_CABLE_UNPLUG: c_int = 0;
pub const HIDP_BOOT_PROTOCOL_MODE: c_int = 1;
pub const HIDP_BLUETOOTH_VENDOR_ID: c_int = 9;
pub const HIDP_WAITING_FOR_RETURN: c_int = 10;
pub const HIDP_WAITING_FOR_SEND_ACK: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidp_connadd_req {
    pub /: *mut *mut int ctrl_sock; / Connected control socket,
    pub /: *mut *mut int intr_sock; / Connected interrupt socket,
    pub parser: __u16,
    pub rd_size: __u16,
    pub rd_data: *mut __u8 __user,
    pub country: __u8,
    pub subclass: __u8,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
    pub flags: __u32,
    pub idle_to: __u32,
    pub name: [c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidp_conndel_req {
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidp_conninfo {
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
    pub state: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
    pub name: [c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidp_connlist_req {
    pub cnum: __u32,
    pub ci: *mut hidp_conninfo __user,
}

extern "C" {
    pub fn hidp_connection_add(req: *const hidp_connadd_req, ctrl_sock: *mut socket, intr_sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn hidp_connection_del(req: *mut hidp_conndel_req) -> c_int;
}
extern "C" {
    pub fn hidp_get_connlist(req: *mut hidp_connlist_req) -> c_int;
}
extern "C" {
    pub fn hidp_get_conninfo(ci: *mut hidp_conninfo) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hidp_session_state {
    HIDP_SESSION_IDLING,
    HIDP_SESSION_PREPARING,
    HIDP_SESSION_RUNNING,
}

// HIDP session defines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidp_session {
    pub list: list_head,
    pub ref: kref,
// runtime management
    pub state: core::sync::atomic::AtomicI32,
    pub state_queue: wait_queue_head_t,
    pub terminate: core::sync::atomic::AtomicI32,
    pub task: *mut task_struct,
    pub flags: c_ulong,
// connection management
    pub bdaddr: bdaddr_t,
    pub conn: *mut l2cap_conn,
    pub user: l2cap_user,
    pub ctrl_sock: *mut socket,
    pub intr_sock: *mut socket,
    pub ctrl_transmit: sk_buff_head,
    pub intr_transmit: sk_buff_head,
    pub ctrl_mtu: c_uint,
    pub intr_mtu: c_uint,
    pub idle_to: c_ulong,
// device management
    pub dev_init: work_struct,
    pub input: *mut input_dev,
    pub hid: *mut hid_device,
    pub timer: timer_list,
// Report descriptor
    pub rd_data: *mut __u8,
    pub rd_size: c_uint,
// session data
    pub keys: [c_uchar; 8],
    pub leds: c_uchar,
// Used in hidp_get_raw_report()
    pub /: *mut *mut *mut int waiting_report_type; / HIDP_DATA_RTYPE_,
    pub /: *mut *mut int waiting_report_number; / -1 for not numbered,
    pub report_mutex: mutex,
    pub report_return: *mut sk_buff,
    pub report_queue: wait_queue_head_t,
// Used in hidp_output_raw_report()
    pub /: *mut *mut int output_report_success; / boolean,
// temporary input buffer
    pub input_buf: [u8; HID_MAX_BUFFER_SIZE],
}

// HIDP init defines
extern "C" {
    pub fn hidp_init_sockets() -> int __init;
}
extern "C" {
    pub fn hidp_cleanup_sockets() -> void __exit;
}
