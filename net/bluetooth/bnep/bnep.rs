//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/bnep/bnep.h
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


// SPDX-License-Identifier: GPL-2.0-only
//

// Limits
pub const BNEP_MAX_PROTO_FILTERS: c_int = 5;
pub const BNEP_MAX_MULTICAST_FILTERS: c_int = 20;
// UUIDs
pub const BNEP_BASE_UUID: c_uint = 0x0000000000001000800000805F9B34FB;
pub const BNEP_UUID16: c_uint = 0x02;
pub const BNEP_UUID32: c_uint = 0x04;
pub const BNEP_UUID128: c_uint = 0x16;
pub const BNEP_SVC_PANU: c_uint = 0x1115;
pub const BNEP_SVC_NAP: c_uint = 0x1116;
pub const BNEP_SVC_GN: c_uint = 0x1117;
// Packet types
pub const BNEP_GENERAL: c_uint = 0x00;
pub const BNEP_CONTROL: c_uint = 0x01;
pub const BNEP_COMPRESSED: c_uint = 0x02;
pub const BNEP_COMPRESSED_SRC_ONLY: c_uint = 0x03;
pub const BNEP_COMPRESSED_DST_ONLY: c_uint = 0x04;
// Control types
pub const BNEP_CMD_NOT_UNDERSTOOD: c_uint = 0x00;
pub const BNEP_SETUP_CONN_REQ: c_uint = 0x01;
pub const BNEP_SETUP_CONN_RSP: c_uint = 0x02;
pub const BNEP_FILTER_NET_TYPE_SET: c_uint = 0x03;
pub const BNEP_FILTER_NET_TYPE_RSP: c_uint = 0x04;
pub const BNEP_FILTER_MULTI_ADDR_SET: c_uint = 0x05;
pub const BNEP_FILTER_MULTI_ADDR_RSP: c_uint = 0x06;
// Extension types
pub const BNEP_EXT_CONTROL: c_uint = 0x00;
// Response messages
pub const BNEP_SUCCESS: c_uint = 0x00;
pub const BNEP_CONN_INVALID_DST: c_uint = 0x01;
pub const BNEP_CONN_INVALID_SRC: c_uint = 0x02;
pub const BNEP_CONN_INVALID_SVC: c_uint = 0x03;
pub const BNEP_CONN_NOT_ALLOWED: c_uint = 0x04;
pub const BNEP_FILTER_UNSUPPORTED_REQ: c_uint = 0x01;
pub const BNEP_FILTER_INVALID_RANGE: c_uint = 0x02;
pub const BNEP_FILTER_INVALID_MCADDR: c_uint = 0x02;
pub const BNEP_FILTER_LIMIT_REACHED: c_uint = 0x03;
pub const BNEP_FILTER_DENIED_SECURITY: c_uint = 0x04;
// L2CAP settings
pub const BNEP_MTU: c_int = 1691;
pub const BNEP_PSM: c_uint = 0x0f;
pub const BNEP_FLUSH_TO: c_uint = 0xffff;
pub const BNEP_CONNECT_TO: c_int = 15;
pub const BNEP_FILTER_TO: c_int = 15;
// Headers
pub const BNEP_TYPE_MASK: c_uint = 0x7f;
pub const BNEP_EXT_HEADER: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_setup_conn_req {
    pub type: __u8,
    pub ctrl: __u8,
    pub uuid_size: __u8,
    pub service: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_set_filter_req {
    pub type: __u8,
    pub ctrl: __u8,
    pub len: __be16,
    pub list: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_control_rsp {
    pub type: __u8,
    pub ctrl: __u8,
    pub resp: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_ext_hdr {
    pub type: __u8,
    pub len: __u8,
    pub data: [__u8; ],
    pub __packed: },
// BNEP ioctl defines

pub const BNEP_SETUP_RESPONSE: c_int = 0;
pub const BNEP_SETUP_RSP_SENT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_connadd_req {
    pub /: *mut *mut int sock; / Connected socket,
    pub flags: __u32,
    pub role: __u16,
    pub /: *mut *mut char device[16]; / Name of the Ethernet device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_conndel_req {
    pub flags: __u32,
    pub dst: [__u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_conninfo {
    pub flags: __u32,
    pub role: __u16,
    pub state: __u16,
    pub dst: [__u8; ETH_ALEN],
    pub device: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_connlist_req {
    pub cnum: __u32,
    pub ci: *mut bnep_conninfo __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_proto_filter {
    pub start: __u16,
    pub end: __u16,
}

extern "C" {
    pub fn bnep_add_connection(req: *mut bnep_connadd_req, sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn bnep_del_connection(req: *mut bnep_conndel_req) -> c_int;
}
extern "C" {
    pub fn bnep_get_connlist(req: *mut bnep_connlist_req) -> c_int;
}
extern "C" {
    pub fn bnep_get_conninfo(ci: *mut bnep_conninfo) -> c_int;
}
// BNEP sessions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnep_session {
    pub list: list_head,
    pub role: c_uint,
    pub state: c_ulong,
    pub flags: c_ulong,
    pub terminate: core::sync::atomic::AtomicI32,
    pub task: *mut task_struct,
    pub eh: ethhdr,
    pub msg: msghdr,
    pub proto_filter: [bnep_proto_filter; BNEP_MAX_PROTO_FILTERS],
    pub mc_filter: c_ulonglong,
    pub sock: *mut socket,
    pub dev: *mut net_device,
}

extern "C" {
    pub fn bnep_net_setup(dev: *mut net_device);
}
extern "C" {
    pub fn bnep_sock_init() -> c_int;
}
extern "C" {
    pub fn bnep_sock_cleanup();
}
