//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_bsg_iscsi.h
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
// iSCSI Transport BSG Interface
//
// Copyright (C) 2009   James Smart, Emulex Corporation
//
// This file intended to be included by both kernel and user space
//

//
// iSCSI Transport SGIO v4 BSG Message Support
//
// Default BSG request timeout (in seconds)

//
// Request Message Codes supported by the iSCSI Transport
//
// define the class masks for the message codes
pub const ISCSI_BSG_CLS_MASK: c_uint = 0xF0000000      /* find object class */;
pub const ISCSI_BSG_HST_MASK: c_uint = 0x80000000      /* iscsi host class */;
// iscsi host Message Codes

//
// iSCSI Host Messages
//
// ISCSI_BSG_HST_VENDOR :
// Request:
// Note: When specifying vendor_id, be sure to read the Vendor Type and ID
// formatting requirements specified in scsi_netlink.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bsg_host_vendor {
//
// Identifies the vendor that the message is formatted for. This
// should be the recipient of the message.
//
    pub vendor_id: u64,
// start of vendor command area
    pub vendor_cmd: [u32; ],
}

// Response:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bsg_host_vendor_reply {
// start of vendor response area
    pub vendor_rsp): DECLARE_FLEX_ARRAY(uint32_t,,
}

// request (CDB) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bsg_request {
    pub msgcode: u32,
    pub h_vendor: iscsi_bsg_host_vendor,
    pub rqst_data: },
    pub __attribute__((packed)): },
// response (request sense data) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_bsg_reply {
//
// The completion result. Result exists in two forms:
// if negative, it is an -Exxx system errno value. There will
// be no further reply information supplied.
// else, it's the 4-byte scsi error result, with driver, host,
// msg and status fields. The per-msgcode reply structure
// will contain valid data.
//
    pub result: u32,
// If there was reply_payload, how much was received ?
    pub reply_payload_rcv_len: u32,
    pub vendor_reply: iscsi_bsg_host_vendor_reply,
    pub reply_data: },
}
