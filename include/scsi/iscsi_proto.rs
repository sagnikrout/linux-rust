//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/iscsi_proto.h
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
// RFC 3720 (iSCSI) protocol data types
//
// Copyright (C) 2005 Dmitry Yusupov
// Copyright (C) 2005 Alex Aizman
// maintained by open-iscsi@googlegroups.com
//

pub const ISCSI_DRAFT20_VERSION: c_uint = 0x00;
// default iSCSI listen port for incoming connections
pub const ISCSI_LISTEN_PORT: c_int = 3260;
// iSCSI header length
pub const ISCSI_HDR_LEN: c_int = 48;
// iSCSI CRC32C length
pub const ISCSI_CRC_LEN: c_int = 4;
// Padding word length
pub const ISCSI_PAD_LEN: c_int = 4;
//
// Serial Number Arithmetic, 32 bits, RFC1982
//
// useful common(control and data paths) macro
//

// initiator tags; opaque for target
pub type itt_t = uint32_t ;
// below makes sense only for initiator that created this tag

//
// iSCSI Template Message Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_hdr {
    pub opcode: u8,
    pub /: *mut *mut uint8_t flags; / Final bit,
    pub rsvd2: [u8; 2],
    pub /: *mut *mut uint8_t hlength; / AHSs total length,
    pub /: *mut *mut uint8_t dlength[3]; / Data length,
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag, opaque for target,
    pub /: *mut *mut __be32 ttt; / Target Task Tag,
    pub statsn: __be32,
    pub exp_statsn: __be32,
    pub max_statsn: __be32,
    pub other: [u8; 12],
}

// RFC 3720 Begin
pub const ISCSI_RESERVED_TAG: c_uint = 0xffffffff;
// Opcode encoding bits
pub const ISCSI_OP_RETRY: c_uint = 0x80;
pub const ISCSI_OP_IMMEDIATE: c_uint = 0x40;
pub const ISCSI_OPCODE_MASK: c_uint = 0x3F;
// Initiator Opcode values
pub const ISCSI_OP_NOOP_OUT: c_uint = 0x00;
pub const ISCSI_OP_SCSI_CMD: c_uint = 0x01;
pub const ISCSI_OP_SCSI_TMFUNC: c_uint = 0x02;
pub const ISCSI_OP_LOGIN: c_uint = 0x03;
pub const ISCSI_OP_TEXT: c_uint = 0x04;
pub const ISCSI_OP_SCSI_DATA_OUT: c_uint = 0x05;
pub const ISCSI_OP_LOGOUT: c_uint = 0x06;
pub const ISCSI_OP_SNACK: c_uint = 0x10;
pub const ISCSI_OP_VENDOR1_CMD: c_uint = 0x1c;
pub const ISCSI_OP_VENDOR2_CMD: c_uint = 0x1d;
pub const ISCSI_OP_VENDOR3_CMD: c_uint = 0x1e;
pub const ISCSI_OP_VENDOR4_CMD: c_uint = 0x1f;
// Target Opcode values
pub const ISCSI_OP_NOOP_IN: c_uint = 0x20;
pub const ISCSI_OP_SCSI_CMD_RSP: c_uint = 0x21;
pub const ISCSI_OP_SCSI_TMFUNC_RSP: c_uint = 0x22;
pub const ISCSI_OP_LOGIN_RSP: c_uint = 0x23;
pub const ISCSI_OP_TEXT_RSP: c_uint = 0x24;
pub const ISCSI_OP_SCSI_DATA_IN: c_uint = 0x25;
pub const ISCSI_OP_LOGOUT_RSP: c_uint = 0x26;
pub const ISCSI_OP_R2T: c_uint = 0x31;
pub const ISCSI_OP_ASYNC_EVENT: c_uint = 0x32;
pub const ISCSI_OP_REJECT: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_ahs_hdr {
    pub ahslength: __be16,
    pub ahstype: u8,
    pub ahspec: [u8; 5],
}

pub const ISCSI_AHSTYPE_CDB: c_int = 1;
pub const ISCSI_AHSTYPE_RLENGTH: c_int = 2;
pub const ISCSI_CDB_SIZE: c_int = 16;
// iSCSI PDU Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_scsi_req {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: __be16,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub data_length: __be32,
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub /: *mut *mut uint8_t cdb[ISCSI_CDB_SIZE]; / SCSI Command Block,
// Additional Data (Command Dependent)
}

// Command PDU flags
pub const ISCSI_FLAG_CMD_FINAL: c_uint = 0x80;
pub const ISCSI_FLAG_CMD_READ: c_uint = 0x40;
pub const ISCSI_FLAG_CMD_WRITE: c_uint = 0x20;
pub const ISCSI_FLAG_CMD_ATTR_MASK: c_uint = 0x07	/* 3 bits */;
// SCSI Command Attribute values
pub const ISCSI_ATTR_UNTAGGED: c_int = 0;
pub const ISCSI_ATTR_SIMPLE: c_int = 1;
pub const ISCSI_ATTR_ORDERED: c_int = 2;
pub const ISCSI_ATTR_HEAD_OF_QUEUE: c_int = 3;
pub const ISCSI_ATTR_ACA: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_rlength_ahdr {
    pub ahslength: __be16,
    pub ahstype: u8,
    pub reserved: u8,
    pub read_length: __be32,
}

// Extended CDB AHS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_ecdb_ahdr {
    pub /: *mut *mut __be16 ahslength; / CDB length - 15, including reserved byte,
    pub ahstype: u8,
    pub reserved: u8,
// 4-byte aligned extended CDB spillover
    pub ISCSI_CDB_SIZE]: uint8_t ecdb[SCSI_MAX_VARLEN_CDB_SIZE -,
}

// SCSI Response Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_scsi_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub response: u8,
    pub cmd_status: u8,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd: [u8; 8],
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub rsvd1: __be32,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub exp_datasn: __be32,
    pub bi_residual_count: __be32,
    pub residual_count: __be32,
// Response or Sense Data (optional)
}

// Command Response PDU flags
pub const ISCSI_FLAG_CMD_BIDI_OVERFLOW: c_uint = 0x10;
pub const ISCSI_FLAG_CMD_BIDI_UNDERFLOW: c_uint = 0x08;
pub const ISCSI_FLAG_CMD_OVERFLOW: c_uint = 0x04;
pub const ISCSI_FLAG_CMD_UNDERFLOW: c_uint = 0x02;
// iSCSI Status values. Valid if Rsp Selector bit is not set
pub const ISCSI_STATUS_CMD_COMPLETED: c_int = 0;
pub const ISCSI_STATUS_TARGET_FAILURE: c_int = 1;
pub const ISCSI_STATUS_SUBSYS_FAILURE: c_int = 2;
// Asynchronous Event Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_async {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub rsvd3: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub rsvd4: [u8; 8],
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub async_event: u8,
    pub async_vcode: u8,
    pub param1: __be16,
    pub param2: __be16,
    pub param3: __be16,
    pub rsvd5: [u8; 4],
}

// iSCSI Event Codes
pub const ISCSI_ASYNC_MSG_SCSI_EVENT: c_int = 0;
pub const ISCSI_ASYNC_MSG_REQUEST_LOGOUT: c_int = 1;
pub const ISCSI_ASYNC_MSG_DROPPING_CONNECTION: c_int = 2;
pub const ISCSI_ASYNC_MSG_DROPPING_ALL_CONNECTIONS: c_int = 3;
pub const ISCSI_ASYNC_MSG_PARAM_NEGOTIATION: c_int = 4;
pub const ISCSI_ASYNC_MSG_VENDOR_SPECIFIC: c_int = 255;
// NOP-Out Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_nopout {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: __be16,
    pub rsvd3: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub /: *mut *mut __be32 ttt; / Target Transfer Tag,
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub rsvd4: [u8; 16],
}

// NOP-In Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_nopin {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: __be16,
    pub rsvd3: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub /: *mut *mut __be32 ttt; / Target Transfer Tag,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub rsvd4: [u8; 12],
}

// SCSI Task Management Message Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tm {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd1: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub /: *mut *mut itt_t rtt; / Reference Task Tag,
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub refcmdsn: __be32,
    pub exp_datasn: __be32,
    pub rsvd2: [u8; 8],
}

pub const ISCSI_FLAG_TM_FUNC_MASK: c_uint = 0x7F;
// Function values
pub const ISCSI_TM_FUNC_ABORT_TASK: c_int = 1;
pub const ISCSI_TM_FUNC_ABORT_TASK_SET: c_int = 2;
pub const ISCSI_TM_FUNC_CLEAR_ACA: c_int = 3;
pub const ISCSI_TM_FUNC_CLEAR_TASK_SET: c_int = 4;
pub const ISCSI_TM_FUNC_LOGICAL_UNIT_RESET: c_int = 5;
pub const ISCSI_TM_FUNC_TARGET_WARM_RESET: c_int = 6;
pub const ISCSI_TM_FUNC_TARGET_COLD_RESET: c_int = 7;
pub const ISCSI_TM_FUNC_TASK_REASSIGN: c_int = 8;

// SCSI Task Management Response Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_tm_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub /: *mut *mut uint8_t response; / see Response values below,
    pub qualifier: u8,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd2: [u8; 8],
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub /: *mut *mut itt_t rtt; / Reference Task Tag,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub rsvd3: [u8; 12],
}

// Response values
pub const ISCSI_TMF_RSP_COMPLETE: c_uint = 0x00;
pub const ISCSI_TMF_RSP_NO_TASK: c_uint = 0x01;
pub const ISCSI_TMF_RSP_NO_LUN: c_uint = 0x02;
pub const ISCSI_TMF_RSP_TASK_ALLEGIANT: c_uint = 0x03;
pub const ISCSI_TMF_RSP_NO_FAILOVER: c_uint = 0x04;
pub const ISCSI_TMF_RSP_NOT_SUPPORTED: c_uint = 0x05;
pub const ISCSI_TMF_RSP_AUTH_FAILED: c_uint = 0x06;
pub const ISCSI_TMF_RSP_REJECTED: c_uint = 0xff;
// Ready To Transfer Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_r2t_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub /: *mut *mut __be32 ttt; / Target Transfer Tag,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub r2tsn: __be32,
    pub data_offset: __be32,
    pub data_length: __be32,
}

// SCSI Data Hdr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_data {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub rsvd3: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub itt: itt_t,
    pub ttt: __be32,
    pub rsvd4: __be32,
    pub exp_statsn: __be32,
    pub rsvd5: __be32,
    pub datasn: __be32,
    pub offset: __be32,
    pub rsvd6: __be32,
// Payload
}

// SCSI Data Response Hdr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_data_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: u8,
    pub cmd_status: u8,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub lun: scsi_lun,
    pub itt: itt_t,
    pub ttt: __be32,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub datasn: __be32,
    pub offset: __be32,
    pub residual_count: __be32,
}

// Data Response PDU flags
pub const ISCSI_FLAG_DATA_ACK: c_uint = 0x40;
pub const ISCSI_FLAG_DATA_OVERFLOW: c_uint = 0x04;
pub const ISCSI_FLAG_DATA_UNDERFLOW: c_uint = 0x02;
pub const ISCSI_FLAG_DATA_STATUS: c_uint = 0x01;
// Text Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_text {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd4: [u8; 8],
    pub itt: itt_t,
    pub ttt: __be32,
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub rsvd5: [u8; 16],
// Text - key=value pairs
}

pub const ISCSI_FLAG_TEXT_CONTINUE: c_uint = 0x40;
// Text Response Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_text_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd4: [u8; 8],
    pub itt: itt_t,
    pub ttt: __be32,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub rsvd5: [u8; 12],
// Text Response - key:value pairs
}

// Login Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_req {
    pub opcode: u8,
    pub flags: u8,
    pub /: *mut *mut uint8_t max_version; / Max. version supported,
    pub /: *mut *mut uint8_t min_version; / Min. version supported,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub /: *mut *mut uint8_t isid[6]; / Initiator Session ID,
    pub /: *mut *mut __be16 tsih; / Target Session Handle,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub cid: __be16,
    pub rsvd3: __be16,
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub rsvd5: [u8; 16],
}

// Login PDU flags
pub const ISCSI_FLAG_LOGIN_TRANSIT: c_uint = 0x80;
pub const ISCSI_FLAG_LOGIN_CONTINUE: c_uint = 0x40;
pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE_MASK: c_uint = 0x0C	/* 2 bits */;
pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE1: c_uint = 0x04;
pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE2: c_uint = 0x08;
pub const ISCSI_FLAG_LOGIN_CURRENT_STAGE3: c_uint = 0x0C;
pub const ISCSI_FLAG_LOGIN_NEXT_STAGE_MASK: c_uint = 0x03	/* 2 bits */;
pub const ISCSI_FLAG_LOGIN_NEXT_STAGE1: c_uint = 0x01;
pub const ISCSI_FLAG_LOGIN_NEXT_STAGE2: c_uint = 0x02;
pub const ISCSI_FLAG_LOGIN_NEXT_STAGE3: c_uint = 0x03;

// Login Response Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub /: *mut *mut uint8_t max_version; / Max. version supported,
    pub /: *mut *mut uint8_t active_version; / Active version,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub /: *mut *mut uint8_t isid[6]; / Initiator Session ID,
    pub /: *mut *mut __be16 tsih; / Target Session Handle,
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub rsvd3: __be32,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub /: *mut *mut uint8_t status_class; / see Login RSP ststus classes below,
    pub /: *mut *mut uint8_t status_detail; / see Login RSP Status details below,
    pub rsvd4: [u8; 10],
}

// Login stage (phase) codes for CSG, NSG

pub const ISCSI_SECURITY_NEGOTIATION_STAGE: c_int = 0;
pub const ISCSI_OP_PARMS_NEGOTIATION_STAGE: c_int = 1;
pub const ISCSI_FULL_FEATURE_PHASE: c_int = 3;
// Login Status response classes
pub const ISCSI_STATUS_CLS_SUCCESS: c_uint = 0x00;
pub const ISCSI_STATUS_CLS_REDIRECT: c_uint = 0x01;
pub const ISCSI_STATUS_CLS_INITIATOR_ERR: c_uint = 0x02;
pub const ISCSI_STATUS_CLS_TARGET_ERR: c_uint = 0x03;
// Login Status response detail codes
// Class-0 (Success)
pub const ISCSI_LOGIN_STATUS_ACCEPT: c_uint = 0x00;
// Class-1 (Redirection)
pub const ISCSI_LOGIN_STATUS_TGT_MOVED_TEMP: c_uint = 0x01;
pub const ISCSI_LOGIN_STATUS_TGT_MOVED_PERM: c_uint = 0x02;
// Class-2 (Initiator Error)
pub const ISCSI_LOGIN_STATUS_INIT_ERR: c_uint = 0x00;
pub const ISCSI_LOGIN_STATUS_AUTH_FAILED: c_uint = 0x01;
pub const ISCSI_LOGIN_STATUS_TGT_FORBIDDEN: c_uint = 0x02;
pub const ISCSI_LOGIN_STATUS_TGT_NOT_FOUND: c_uint = 0x03;
pub const ISCSI_LOGIN_STATUS_TGT_REMOVED: c_uint = 0x04;
pub const ISCSI_LOGIN_STATUS_NO_VERSION: c_uint = 0x05;
pub const ISCSI_LOGIN_STATUS_ISID_ERROR: c_uint = 0x06;
pub const ISCSI_LOGIN_STATUS_MISSING_FIELDS: c_uint = 0x07;
pub const ISCSI_LOGIN_STATUS_CONN_ADD_FAILED: c_uint = 0x08;
pub const ISCSI_LOGIN_STATUS_NO_SESSION_TYPE: c_uint = 0x09;
pub const ISCSI_LOGIN_STATUS_NO_SESSION: c_uint = 0x0a;
pub const ISCSI_LOGIN_STATUS_INVALID_REQUEST: c_uint = 0x0b;
// Class-3 (Target Error)
pub const ISCSI_LOGIN_STATUS_TARGET_ERROR: c_uint = 0x00;
pub const ISCSI_LOGIN_STATUS_SVC_UNAVAILABLE: c_uint = 0x01;
pub const ISCSI_LOGIN_STATUS_NO_RESOURCES: c_uint = 0x02;
// Logout Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd1: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd2: [u8; 8],
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub cid: __be16,
    pub rsvd3: [u8; 2],
    pub cmdsn: __be32,
    pub exp_statsn: __be32,
    pub rsvd4: [u8; 16],
}

// Logout PDU flags
pub const ISCSI_FLAG_LOGOUT_REASON_MASK: c_uint = 0x7F;
// logout reason_code values
pub const ISCSI_LOGOUT_REASON_CLOSE_SESSION: c_int = 0;
pub const ISCSI_LOGOUT_REASON_CLOSE_CONNECTION: c_int = 1;
pub const ISCSI_LOGOUT_REASON_RECOVERY: c_int = 2;
pub const ISCSI_LOGOUT_REASON_AEN_REQUEST: c_int = 3;
// Logout Response Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout_rsp {
    pub opcode: u8,
    pub flags: u8,
    pub /: *mut *mut uint8_t response; / see Logout response values below,
    pub rsvd2: u8,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd3: [u8; 8],
    pub /: *mut *mut itt_t itt; / Initiator Task Tag,
    pub rsvd4: __be32,
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub rsvd5: __be32,
    pub t2wait: __be16,
    pub t2retain: __be16,
    pub rsvd6: __be32,
}

// logout response status values
pub const ISCSI_LOGOUT_SUCCESS: c_int = 0;
pub const ISCSI_LOGOUT_CID_NOT_FOUND: c_int = 1;
pub const ISCSI_LOGOUT_RECOVERY_UNSUPPORTED: c_int = 2;
pub const ISCSI_LOGOUT_CLEANUP_FAILED: c_int = 3;
// SNACK Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_snack {
    pub opcode: u8,
    pub flags: u8,
    pub rsvd2: [u8; 2],
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub lun: [u8; 8],
    pub itt: itt_t,
    pub ttt: __be32,
    pub rsvd3: [u8; 4],
    pub exp_statsn: __be32,
    pub rsvd4: [u8; 8],
    pub begrun: __be32,
    pub runlength: __be32,
}

// SNACK PDU flags
pub const ISCSI_FLAG_SNACK_TYPE_DATA: c_int = 0;
pub const ISCSI_FLAG_SNACK_TYPE_R2T: c_int = 0;
pub const ISCSI_FLAG_SNACK_TYPE_STATUS: c_int = 1;
pub const ISCSI_FLAG_SNACK_TYPE_DATA_ACK: c_int = 2;
pub const ISCSI_FLAG_SNACK_TYPE_RDATA: c_int = 3;
pub const ISCSI_FLAG_SNACK_TYPE_MASK: c_uint = 0x0F	/* 4 bits */;
// Reject Message Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_reject {
    pub opcode: u8,
    pub flags: u8,
    pub reason: u8,
    pub rsvd2: u8,
    pub hlength: u8,
    pub dlength: [u8; 3],
    pub rsvd3: [u8; 8],
    pub ffffffff: __be32,
    pub rsvd4: [u8; 4],
    pub statsn: __be32,
    pub exp_cmdsn: __be32,
    pub max_cmdsn: __be32,
    pub datasn: __be32,
    pub rsvd5: [u8; 8],
// Text - Rejected hdr
}

// Reason for Reject
pub const ISCSI_REASON_CMD_BEFORE_LOGIN: c_int = 1;
pub const ISCSI_REASON_DATA_DIGEST_ERROR: c_int = 2;
pub const ISCSI_REASON_DATA_SNACK_REJECT: c_int = 3;
pub const ISCSI_REASON_PROTOCOL_ERROR: c_int = 4;
pub const ISCSI_REASON_CMD_NOT_SUPPORTED: c_int = 5;
pub const ISCSI_REASON_IMM_CMD_REJECT: c_int = 6;
pub const ISCSI_REASON_TASK_IN_PROGRESS: c_int = 7;
pub const ISCSI_REASON_INVALID_SNACK: c_int = 8;
pub const ISCSI_REASON_BOOKMARK_INVALID: c_int = 9;
pub const ISCSI_REASON_BOOKMARK_NO_RESOURCES: c_int = 10;
pub const ISCSI_REASON_NEGOTIATION_RESET: c_int = 11;
// Max. number of Key=Value pairs in a text message
pub const MAX_KEY_VALUE_PAIRS: c_int = 8192;
// maximum length for text keys/values
pub const KEY_MAXLEN: c_int = 64;
pub const VALUE_MAXLEN: c_int = 255;

pub const ISCSI_DEF_MAX_RECV_SEG_LEN: c_int = 8192;
pub const ISCSI_MIN_MAX_RECV_SEG_LEN: c_int = 512;
pub const ISCSI_MAX_MAX_RECV_SEG_LEN: c_int = 16777215;
pub const ISCSI_DEF_FIRST_BURST_LEN: c_int = 65536;
pub const ISCSI_MIN_FIRST_BURST_LEN: c_int = 512;
pub const ISCSI_MAX_FIRST_BURST_LEN: c_int = 16777215;
pub const ISCSI_DEF_MAX_BURST_LEN: c_int = 262144;
pub const ISCSI_MIN_MAX_BURST_LEN: c_int = 512;
pub const ISCSI_MAX_MAX_BURST_LEN: c_int = 16777215;
pub const ISCSI_DEF_TIME2WAIT: c_int = 2;
pub const ISCSI_NAME_LEN: c_int = 224;
// RFC 3720 End
