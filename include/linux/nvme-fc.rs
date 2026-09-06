//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvme-fc.h
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
// Copyright (c) 2016 Avago Technologies.  All rights reserved.
//
// This file contains definitions relative to FC-NVME-2 r1.08
// (T11-2019-00210-v004).
//
pub const _NVME_FC_H: c_int = 1;

pub const NVME_CMD_FORMAT_ID: c_uint = 0xFD;

// FC-NVME Cmd IU Flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fc_cmd_iu {
    pub format_id: __u8,
    pub fc_id: __u8,
    pub iu_len: __be16,
    pub rsvd4: [__u8; 2],
    pub rsv_cat: __u8,
    pub flags: __u8,
    pub connection_id: __be64,
    pub csn: __be32,
    pub data_len: __be32,
    pub sqe: nvme_command,
    pub dps: __u8,
    pub lbads: __u8,
    pub ms: __be16,
    pub rsvd92: __be32,
}

pub const NVME_FC_SIZEOF_ZEROS_RSP: c_int = 12;
// reserved			  2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fc_ersp_iu {
    pub ersp_result: __u8,
    pub rsvd1: __u8,
    pub iu_len: __be16,
    pub rsn: __be32,
    pub xfrd_len: __be32,
    pub rsvd12: __be32,
    pub cqe: nvme_completion,
// for now - no additional payload
}

pub const FCNVME_NVME_SR_OPCODE: c_uint = 0x01;
pub const FCNVME_NVME_SR_RSP_OPCODE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fc_nvme_sr_iu {
    pub fc_id: __u8,
    pub opcode: __u8,
    pub rsvd2: __u8,
    pub retry_rctl: __u8,
    pub rsvd4: __be32,
}

// reserved			  0x1
// reserved			  0x2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fc_nvme_sr_rsp_iu {
    pub fc_id: __u8,
    pub opcode: __u8,
    pub rsvd2: __u8,
    pub status: __u8,
    pub rsvd4: __be32,
}

// FC-NVME Link Services - LS cmd values (w0 bits 31:24)
// FC-NVME Link Service Descriptors
// ********** start of Link Service Descriptors **********
//
// fills in length of a descriptor. Struture minus descriptor header
//
extern "C" {
    pub fn cpu_to_be32(sizeof(u32)): *mut *mut sz - (2) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_rqst_w0 {
    pub /: *mut *mut u8 ls_cmd; / FCNVME_LS_xxx,
    pub zeros: [u8; 3],
}

// FCNVME_LSDESC_RQST
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_rqst {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub w0: fcnvme_ls_rqst_w0,
    pub rsvd12: __be32,
}

// FC-NVME LS RJT reason_code values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcnvme_ls_rjt_reason {
    FCNVME_RJT_RC_NONE		= 0,
// no reason - not to be sent

    FCNVME_RJT_RC_INVAL		= 0x01,
// invalid NVMe_LS command code

    FCNVME_RJT_RC_LOGIC		= 0x03,
// logical error

    FCNVME_RJT_RC_UNAB		= 0x09,
// unable to perform command request

    FCNVME_RJT_RC_UNSUP		= 0x0b,
// command not supported

    FCNVME_RJT_RC_INV_ASSOC		= 0x40,
// Invalid Association ID

    FCNVME_RJT_RC_INV_CONN		= 0x41,
// Invalid Connection ID

    FCNVME_RJT_RC_INV_PARAM		= 0x42,
// Invalid Parameters

    FCNVME_RJT_RC_INSUF_RES		= 0x43,
// Insufficient Resources

    FCNVME_RJT_RC_VENDOR		= 0xff,
// vendor specific error
}

// FC-NVME LS RJT reason_explanation values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcnvme_ls_rjt_explan {
    FCNVME_RJT_EXP_NONE		= 0x00,
// No additional explanation

    FCNVME_RJT_EXP_OXID_RXID	= 0x17,
// invalid OX_ID-RX_ID combination

    FCNVME_RJT_EXP_UNAB_DATA	= 0x2a,
// unable to supply requested data

    FCNVME_RJT_EXP_INV_LEN		= 0x2d,
// Invalid payload length

    FCNVME_RJT_EXP_INV_ERSP_RAT	= 0x40,
// Invalid NVMe_ERSP Ratio

    FCNVME_RJT_EXP_INV_CTLR_ID	= 0x41,
// Invalid Controller ID

    FCNVME_RJT_EXP_INV_QUEUE_ID	= 0x42,
// Invalid Queue ID

    FCNVME_RJT_EXP_INV_SQSIZE	= 0x43,
// Invalid Submission Queue Size

    FCNVME_RJT_EXP_INV_HOSTID	= 0x44,
// Invalid HOST ID

    FCNVME_RJT_EXP_INV_HOSTNQN	= 0x45,
// Invalid HOSTNQN

    FCNVME_RJT_EXP_INV_SUBNQN	= 0x46,
// Invalid SUBNQN
}

// FCNVME_LSDESC_RJT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_rjt {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub rsvd8: u8,
//
// Reject reason and explanaction codes are generic
// to ELs's from LS-3.
//
    pub /: *mut *mut u8 reason_code; / fcnvme_ls_rjt_reason,
    pub /: *mut *mut u8 reason_explanation; / fcnvme_ls_rjt_explan,
    pub vendor: u8,
    pub rsvd12: __be32,
}

pub const FCNVME_ASSOC_HOSTNQN_LEN: c_int = 256;
pub const FCNVME_ASSOC_SUBNQN_LEN: c_int = 256;
// FCNVME_LSDESC_CREATE_ASSOC_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_cr_assoc_cmd {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub ersp_ratio: __be16,
    pub rsvd10: __be16,
    pub rsvd12: [__be32; 9],
    pub cntlid: __be16,
    pub sqsize: __be16,
    pub rsvd52: __be32,
    pub hostid: uuid_t,
    pub hostnqn: [u8; FCNVME_ASSOC_HOSTNQN_LEN],
    pub subnqn: [u8; FCNVME_ASSOC_SUBNQN_LEN],
    pub bytes,: *mut *mut __be32 rsvd584[108]; / pad to 1016,
// which makes overall LS rqst
// payload 1024 bytes
//
}

// FCNVME_LSDESC_CREATE_CONN_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_cr_conn_cmd {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub ersp_ratio: __be16,
    pub rsvd10: __be16,
    pub rsvd12: [__be32; 9],
    pub qid: __be16,
    pub sqsize: __be16,
    pub rsvd52: __be32,
}

// FCNVME_LSDESC_DISCONN_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_disconn_cmd {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub rsvd8: [__be32; 4],
}

// FCNVME_LSDESC_CONN_ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_conn_id {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub connection_id: __be64,
}

// FCNVME_LSDESC_ASSOC_ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_lsdesc_assoc_id {
    pub /: *mut *mut __be32 desc_tag; / FCNVME_LSDESC_xxx,
    pub desc_len: __be32,
    pub association_id: __be64,
}

// r_ctl values
// ********** start of Link Services **********
// FCNVME_LS_RJT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_rjt {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub rqst: fcnvme_lsdesc_rqst,
    pub rjt: fcnvme_lsdesc_rjt,
}

// FCNVME_LS_ACC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_acc_hdr {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub rqst: fcnvme_lsdesc_rqst,
//
// Followed by cmd-specific ACCEPT descriptors, see xxx_acc
// definitions below
//
}

// FCNVME_LS_CREATE_ASSOCIATION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_cr_assoc_rqst {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub assoc_cmd: fcnvme_lsdesc_cr_assoc_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_cr_assoc_acc {
    pub hdr: fcnvme_ls_acc_hdr,
    pub associd: fcnvme_lsdesc_assoc_id,
    pub connectid: fcnvme_lsdesc_conn_id,
}

// FCNVME_LS_CREATE_CONNECTION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_cr_conn_rqst {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub associd: fcnvme_lsdesc_assoc_id,
    pub connect_cmd: fcnvme_lsdesc_cr_conn_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_cr_conn_acc {
    pub hdr: fcnvme_ls_acc_hdr,
    pub connectid: fcnvme_lsdesc_conn_id,
}

// FCNVME_LS_DISCONNECT_ASSOC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_disconnect_assoc_rqst {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub associd: fcnvme_lsdesc_assoc_id,
    pub discon_cmd: fcnvme_lsdesc_disconn_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_disconnect_assoc_acc {
    pub hdr: fcnvme_ls_acc_hdr,
}

// FCNVME_LS_DISCONNECT_CONN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_disconnect_conn_rqst {
    pub w0: fcnvme_ls_rqst_w0,
    pub desc_list_len: __be32,
    pub associd: fcnvme_lsdesc_assoc_id,
    pub connectid: fcnvme_lsdesc_conn_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcnvme_ls_disconnect_conn_acc {
    pub hdr: fcnvme_ls_acc_hdr,
}

//
// Default R_A_TOV is pulled in from fc_fs.h but needs conversion
// from ms to seconds for our use.
//

//
// TRADDR string must be of form "nn-<16hexdigits>:pn-<16hexdigits>"
// the string is allowed to be specified with or without a "0x" prefix
// infront of the <16hexdigits>.  Without is considered the "min" string
// and with is considered the "max" string. The hexdigits may be upper
// or lower case.
// Note: FC-NVME-2 standard requires a "0x" prefix.
//

pub const NVME_FC_TRADDR_HEXNAMELEN: c_int = 16;

