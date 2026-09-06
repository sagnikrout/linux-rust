//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/sbp/sbp_target.h
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

pub const SBP_NAMELEN: c_int = 32;
pub const SBP_ORB_FETCH_SIZE: c_int = 8;
pub const MANAGEMENT_AGENT_STATE_IDLE: c_int = 0;
pub const MANAGEMENT_AGENT_STATE_BUSY: c_int = 1;

pub const MANAGEMENT_ORB_FUNCTION_LOGIN: c_uint = 0x0;
pub const MANAGEMENT_ORB_FUNCTION_QUERY_LOGINS: c_uint = 0x1;
pub const MANAGEMENT_ORB_FUNCTION_RECONNECT: c_uint = 0x3;
pub const MANAGEMENT_ORB_FUNCTION_SET_PASSWORD: c_uint = 0x4;
pub const MANAGEMENT_ORB_FUNCTION_LOGOUT: c_uint = 0x7;
pub const MANAGEMENT_ORB_FUNCTION_ABORT_TASK: c_uint = 0xb;
pub const MANAGEMENT_ORB_FUNCTION_ABORT_TASK_SET: c_uint = 0xc;
pub const MANAGEMENT_ORB_FUNCTION_LOGICAL_UNIT_RESET: c_uint = 0xe;
pub const MANAGEMENT_ORB_FUNCTION_TARGET_RESET: c_uint = 0xf;

pub const STATUS_SRC_ORB_CONTINUING: c_int = 0;
pub const STATUS_SRC_ORB_FINISHED: c_int = 1;
pub const STATUS_SRC_UNSOLICITED: c_int = 2;
pub const STATUS_RESP_REQUEST_COMPLETE: c_int = 0;
pub const STATUS_RESP_TRANSPORT_FAILURE: c_int = 1;
pub const STATUS_RESP_ILLEGAL_REQUEST: c_int = 2;
pub const STATUS_RESP_VENDOR_DEPENDENT: c_int = 3;
pub const SBP_STATUS_OK: c_int = 0;
pub const SBP_STATUS_REQ_TYPE_NOTSUPP: c_int = 1;
pub const SBP_STATUS_SPEED_NOTSUPP: c_int = 2;
pub const SBP_STATUS_PAGE_SIZE_NOTSUPP: c_int = 3;
pub const SBP_STATUS_ACCESS_DENIED: c_int = 4;
pub const SBP_STATUS_LUN_NOTSUPP: c_int = 5;
pub const SBP_STATUS_PAYLOAD_TOO_SMALL: c_int = 6;
// 7 is reserved
pub const SBP_STATUS_RESOURCES_UNAVAIL: c_int = 8;
pub const SBP_STATUS_FUNCTION_REJECTED: c_int = 9;
pub const SBP_STATUS_LOGIN_ID_UNKNOWN: c_int = 10;
pub const SBP_STATUS_DUMMY_ORB_COMPLETE: c_int = 11;
pub const SBP_STATUS_REQUEST_ABORTED: c_int = 12;
pub const SBP_STATUS_UNSPECIFIED_ERROR: c_uint = 0xff;
pub const AGENT_STATE_RESET: c_int = 0;
pub const AGENT_STATE_ACTIVE: c_int = 1;
pub const AGENT_STATE_SUSPENDED: c_int = 2;
pub const AGENT_STATE_DEAD: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp2_pointer {
    pub high: __be32,
    pub low: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_command_block_orb {
    pub next_orb: sbp2_pointer,
    pub data_descriptor: sbp2_pointer,
    pub misc: __be32,
    pub command_block: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_page_table_entry {
    pub segment_length: __be16,
    pub segment_base_hi: __be16,
    pub segment_base_lo: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_management_orb {
    pub ptr1: sbp2_pointer,
    pub ptr2: sbp2_pointer,
    pub misc: __be32,
    pub length: __be32,
    pub status_fifo: sbp2_pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_status_block {
    pub status: __be32,
    pub orb_low: __be32,
    pub data: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_login_response_block {
    pub misc: __be32,
    pub command_block_agent: sbp2_pointer,
    pub reconnect_hold: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_login_descriptor {
    pub sess: *mut sbp_session,
    pub link: list_head,
    pub login_lun: u32,
    pub status_fifo_addr: u64,
    pub exclusive: c_int,
    pub login_id: u16,
    pub tgt_agt: *mut sbp_target_agent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_session {
    pub lock: spinlock_t,
    pub se_sess: *mut se_session,
    pub login_list: list_head,
    pub maint_work: delayed_work,
    pub /: *mut *mut u64 guid; / login_owner_EUI_64,
    pub /: *mut *mut int node_id; / login_owner_ID,
    pub card: *mut fw_card,
    pub generation: c_int,
    pub speed: c_int,
    pub reconnect_hold: c_int,
    pub reconnect_expires: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_tpg {
// Target portal group tag for TCM
    pub tport_tpgt: u16,
// Pointer back to sbp_tport
    pub tport: *mut sbp_tport,
// Returned by sbp_make_tpg()
    pub se_tpg: se_portal_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_tport {
// Target Unit Identifier (EUI-64)
    pub guid: u64,
// Target port name
    pub tport_name: [c_char; SBP_NAMELEN],
// Returned by sbp_make_tport()
    pub tport_wwn: se_wwn,
    pub tpg: *mut sbp_tpg,
// FireWire unit directory
    pub unit_directory: fw_descriptor,
// SBP Management Agent
    pub mgt_agt: *mut sbp_management_agent,
// Parameters
    pub enable: c_int,
    pub directory_id: i32,
    pub mgt_orb_timeout: c_int,
    pub max_reconnect_timeout: c_int,
    pub max_logins_per_lun: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_target_agent {
    pub lock: spinlock_t,
    pub handler: fw_address_handler,
    pub login: *mut sbp_login_descriptor,
    pub state: c_int,
    pub work: work_struct,
    pub orb_pointer: u64,
    pub doorbell: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_target_request {
    pub login: *mut sbp_login_descriptor,
    pub orb_pointer: u64,
    pub orb: sbp_command_block_orb,
    pub status: sbp_status_block,
    pub work: work_struct,
    pub se_cmd: se_cmd,
    pub pg_tbl: *mut sbp_page_table_entry,
    pub cmd_buf: *mut c_void,
    pub sense_buf: [c_uchar; TRANSPORT_SENSE_BUFFER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_management_agent {
    pub lock: spinlock_t,
    pub tport: *mut sbp_tport,
    pub handler: fw_address_handler,
    pub state: c_int,
    pub work: work_struct,
    pub orb_offset: u64,
    pub request: *mut sbp_management_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbp_management_request {
    pub orb: sbp_management_orb,
    pub status: sbp_status_block,
    pub card: *mut fw_card,
    pub generation: c_int,
    pub node_addr: c_int,
    pub speed: c_int,
}
