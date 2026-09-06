//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_fcbuild.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//
// fcbuild.h - FC link service frame building and parsing routines
//

//
// Utility Macros/functions
//

//
// Given the fc response length, this routine will return
// the length of the actual payload bytes following the CT header.
//
// Assumes the input response length does not include the crc, eof, etc.
//
// Convert bfa speed to rpsc speed value.
//
// Convert RPSC speed to bfa speed value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_parse_status {
    FC_PARSE_OK = 0,
    FC_PARSE_FAILURE = 1,
    FC_PARSE_BUSY = 2,
    FC_PARSE_LEN_INVAL,
    FC_PARSE_ACC_INVAL,
    FC_PARSE_PWWN_NOT_EQUAL,
    FC_PARSE_NWWN_NOT_EQUAL,
    FC_PARSE_RXSZ_INVAL,
    FC_PARSE_NOT_FCP,
    FC_PARSE_OPAFLAG_INVAL,
    FC_PARSE_RPAFLAG_INVAL,
    FC_PARSE_OPA_INVAL,
    FC_PARSE_RPA_INVAL,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_templates_s {
    pub fc_els_req: fchs_s,
    pub fc_bls_req: fchs_s,
    pub plogi: fc_logi_s,
    pub rrq: fc_rrq_s,
}

extern "C" {
    pub fn fcbuild_init();
}
extern "C" {
    pub fn fc_plogi_parse(fchs: *mut fchs_s) -> fc_parse_status;
}
extern "C" {
    pub fn fc_prli_rsp_parse(prli: *mut fc_prli_s, len: c_int) -> fc_parse_status;
}
extern "C" {
    pub fn fc_gmal_req_build(fchs: *mut fchs_s, pyld: *mut c_void, s_id: u32, wwn: wwn_t) -> u16;
}
extern "C" {
    pub fn fc_gfn_req_build(fchs: *mut fchs_s, pyld: *mut c_void, s_id: u32, wwn: wwn_t) -> u16;
}
extern "C" {
    pub fn fc_get_fc4type_bitmask(fc4_type: u8, bit_mask: *mut u8);
}
extern "C" {
    pub fn fc_logout_params_pages(fc_frame: *mut fchs_s, els_code: u8) -> c_int;
}
