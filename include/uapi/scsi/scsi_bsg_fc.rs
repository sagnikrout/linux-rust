//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/scsi_bsg_fc.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// FC Transport BSG Interface
//
// Copyright (C) 2008   James Smart, Emulex Corporation
//

//
// This file intended to be included by both kernel and user space
//
// FC Transport SGIO v4 BSG Message Support
//
// Default BSG request timeout (in seconds)

//
// Request Message Codes supported by the FC Transport
//
// define the class masks for the message codes
pub const FC_BSG_CLS_MASK: c_uint = 0xF0000000	/* find object class */;
pub const FC_BSG_HST_MASK: c_uint = 0x80000000	/* fc host class */;
pub const FC_BSG_RPT_MASK: c_uint = 0x40000000	/* fc rport class */;
// fc_host Message Codes

// fc_rport Message Codes

//
// FC Address Identifiers in Message Structures :
//
// Whenever a command payload contains a FC Address Identifier
// (aka port_id), the value is effectively in big-endian
// order, thus the array elements are decoded as follows:
// element [0] is bits 23:16 of the FC Address Identifier
// element [1] is bits 15:8 of the FC Address Identifier
// element [2] is bits 7:0 of the FC Address Identifier
//
// FC Host Messages
//
// FC_BSG_HST_ADDR_PORT :
// Request:
// This message requests the FC host to login to the remote port
// at the specified N_Port_Id.  The remote port is to be enumerated
// with the transport upon completion of the login.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_add_rport {
    pub reserved: __u8,
// FC Address Identier of the remote port to login to
    pub port_id: [__u8; 3],
}

// Response:
// There is no additional response data - fc_bsg_reply->result is sufficient
//
// FC_BSG_HST_DEL_RPORT :
// Request:
// This message requests the FC host to remove an enumerated
// remote port and to terminate the login to it.
//
// Note: The driver is free to reject this request if it desires to
// remain logged in with the remote port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_del_rport {
    pub reserved: __u8,
// FC Address Identier of the remote port to logout of
    pub port_id: [__u8; 3],
}

// Response:
// There is no additional response data - fc_bsg_reply->result is sufficient
//
// FC_BSG_HST_ELS_NOLOGIN :
// Request:
// This message requests the FC_Host to send an ELS to a specific
// N_Port_ID. The host does not need to log into the remote port,
// nor does it need to enumerate the rport for further traffic
// (although, the FC host is free to do so if it desires).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_els {
//
// ELS Command Code being sent (must be the same as byte 0
// of the payload)
//
    pub command_code: __u8,
// FC Address Identier of the remote port to send the ELS to
    pub port_id: [__u8; 3],
}

// Response:
//
// fc_bsg_ctels_reply->status values
pub const FC_CTELS_STATUS_OK: c_uint = 0x00000000;
pub const FC_CTELS_STATUS_REJECT: c_uint = 0x00000001;
pub const FC_CTELS_STATUS_P_RJT: c_uint = 0x00000002;
pub const FC_CTELS_STATUS_F_RJT: c_uint = 0x00000003;
pub const FC_CTELS_STATUS_P_BSY: c_uint = 0x00000004;
pub const FC_CTELS_STATUS_F_BSY: c_uint = 0x00000006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_ctels_reply {
//
// Note: An ELS LS_RJT may be reported in 2 ways:
// a) A status of FC_CTELS_STATUS_OK is returned. The caller
// is to look into the ELS receive payload to determine
// LS_ACC or LS_RJT (by contents of word 0). The reject
// data will be in word 1.
// b) A status of FC_CTELS_STATUS_REJECT is returned, The
// rjt_data field will contain valid data.
//
// Note: ELS LS_ACC is determined by an FC_CTELS_STATUS_OK, and
// the receive payload word 0 indicates LS_ACC
// (e.g. value is 0x02xxxxxx).
//
// Note: Similarly, a CT Reject may be reported in 2 ways:
// a) A status of FC_CTELS_STATUS_OK is returned. The caller
// is to look into the CT receive payload to determine
// Accept or Reject (by contents of word 2). The reject
// data will be in word 3.
// b) A status of FC_CTELS_STATUS_REJECT is returned, The
// rjt_data field will contain valid data.
//
// Note: x_RJT/BSY status will indicae that the rjt_data field
// is valid and contains the reason/explanation values.
//
    pub /: *mut *mut __u32 status; / See FC_CTELS_STATUS_xxx,
// valid if status is not FC_CTELS_STATUS_OK
    pub /: *mut *mut __u8 action; / fragment_id for CT REJECT,
    pub reason_code: __u8,
    pub reason_explanation: __u8,
    pub vendor_unique: __u8,
    pub rjt_data: },
}

// FC_BSG_HST_CT :
// Request:
// This message requests that a CT Request be performed with the
// indicated N_Port_ID. The driver is responsible for logging in with
// the fabric and/or N_Port_ID, etc as per FC rules. This request does
// not mandate that the driver must enumerate the destination in the
// transport. The driver is allowed to decide whether to enumerate it,
// and whether to tear it down after the request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_ct {
    pub reserved: __u8,
// FC Address Identier of the remote port to send the ELS to
    pub port_id: [__u8; 3],
//
// We need words 0-2 of the generic preamble for the LLD's
//
    pub /: *mut *mut __u32 preamble_word0; / revision & IN_ID,
    pub /: *mut *mut __u32 preamble_word1; / GS_Type, GS_SubType, Options, Rsvd,
    pub /: *mut *mut __u32 preamble_word2; / Cmd Code, Max Size,
}

// Response:
//
// The reply structure is an fc_bsg_ctels_reply structure
//
// FC_BSG_HST_VENDOR :
// Request:
// Note: When specifying vendor_id, be sure to read the Vendor Type and ID
// formatting requirements specified in scsi_netlink.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_vendor {
//
// Identifies the vendor that the message is formatted for. This
// should be the recipient of the message.
//
    pub vendor_id: __u64,
// start of vendor command area
    pub vendor_cmd: [__u32; ],
}

// Response:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_host_vendor_reply {
// start of vendor response area
    pub vendor_rsp): __DECLARE_FLEX_ARRAY(__u32,,
}

//
// FC Remote Port Messages
//
// FC_BSG_RPT_ELS :
// Request:
// This message requests that an ELS be performed with the rport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_rport_els {
//
// ELS Command Code being sent (must be the same as
// byte 0 of the payload)
//
    pub els_code: __u8,
}

// Response:
//
// The reply structure is an fc_bsg_ctels_reply structure
//
// FC_BSG_RPT_CT :
// Request:
// This message requests that a CT Request be performed with the rport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_rport_ct {
//
// We need words 0-2 of the generic preamble for the LLD's
//
    pub /: *mut *mut __u32 preamble_word0; / revision & IN_ID,
    pub /: *mut *mut __u32 preamble_word1; / GS_Type, GS_SubType, Options, Rsvd,
    pub /: *mut *mut __u32 preamble_word2; / Cmd Code, Max Size,
}

// Response:
//
// The reply structure is an fc_bsg_ctels_reply structure
//
// request (CDB) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_request {
    pub msgcode: __u32,
    pub h_addrport: fc_bsg_host_add_rport,
    pub h_delrport: fc_bsg_host_del_rport,
    pub h_els: fc_bsg_host_els,
    pub h_ct: fc_bsg_host_ct,
    pub h_vendor: fc_bsg_host_vendor,
    pub r_els: fc_bsg_rport_els,
    pub r_ct: fc_bsg_rport_ct,
    pub rqst_data: },
    pub __attribute__((packed)): },
// response (request sense data) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_bsg_reply {
//
// The completion result. Result exists in two forms:
// if negative, it is an -Exxx system errno value. There will
// be no further reply information supplied.
// else, it's the 4-byte scsi error result, with driver, host,
// msg and status fields. The per-msgcode reply structure
// will contain valid data.
//
    pub result: __u32,
// If there was reply_payload, how much was recevied ?
    pub reply_payload_rcv_len: __u32,
    pub vendor_reply: fc_bsg_host_vendor_reply,
    pub ctels_reply: fc_bsg_ctels_reply,
    pub reply_data: },
}
