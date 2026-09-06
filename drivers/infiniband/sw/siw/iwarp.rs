//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/siw/iwarp.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Copyright (c) 2008-2019, IBM Corporation

pub const RDMAP_VERSION: c_int = 1;
pub const DDP_VERSION: c_int = 1;
pub const MPA_REVISION_1: c_int = 1;
pub const MPA_REVISION_2: c_int = 2;

pub const MPA_IRD_ORD_MASK: c_uint = 0x3fff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_rr_params {
    pub bits: __be16,
    pub pd_len: __be16,
}

//
// MPA request/response header bits & fields
//
// MPA request/reply header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_rr {
    pub key: [__u8; 16],
    pub params: mpa_rr_params,
}

// bits = (*bits & ~MPA_RR_MASK_REVISION) |
extern "C" {
    pub fn be16_to_cpu(_arg: rev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpa_v2_ctrl {
    MPA_V2_PEER_TO_PEER = cpu_to_be16(0x8000),
    MPA_V2_ZERO_LENGTH_RTR = cpu_to_be16(0x4000),
    MPA_V2_RDMA_WRITE_RTR = cpu_to_be16(0x8000),
    MPA_V2_RDMA_READ_RTR = cpu_to_be16(0x4000),
    MPA_V2_RDMA_NO_RTR = cpu_to_be16(0x0000),
    MPA_V2_MASK_IRD_ORD = cpu_to_be16(0x3fff)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_v2_data {
    pub ird: __be16,
    pub ord: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_marker {
    pub rsvd: __be16,
    pub /: *mut *mut __be16 fpdu_hmd; / FPDU header-marker distance (= MPA's FPDUPTR),
}

//
// maximum MPA trailer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_trailer {
    pub pad: [__u8; 4],
    pub crc: __be32,
}

pub const MPA_HDR_SIZE: c_int = 2;
pub const MPA_CRC_SIZE: c_int = 4;
//
// Common portion of iWARP headers (MPA, DDP, RDMAP)
// for any FPDU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_ctrl {
    pub mpa_len: __be16,
    pub ddp_rdmap_ctrl: __be16,
}

//
// DDP/RDMAP Hdr bits & fields
//
extern "C" {
    pub fn be16_to_cpu(RDMAP_MASK_OPCODE: ctrl->ddp_rdmap_ctrl &) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_rdma_write {
    pub ctrl: iwarp_ctrl,
    pub sink_stag: __be32,
    pub sink_to: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_rdma_rreq {
    pub ctrl: iwarp_ctrl,
    pub rsvd: __be32,
    pub ddp_qn: __be32,
    pub ddp_msn: __be32,
    pub ddp_mo: __be32,
    pub sink_stag: __be32,
    pub sink_to: __be64,
    pub read_size: __be32,
    pub source_stag: __be32,
    pub source_to: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_rdma_rresp {
    pub ctrl: iwarp_ctrl,
    pub sink_stag: __be32,
    pub sink_to: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_send {
    pub ctrl: iwarp_ctrl,
    pub rsvd: __be32,
    pub ddp_qn: __be32,
    pub ddp_msn: __be32,
    pub ddp_mo: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_send_inv {
    pub ctrl: iwarp_ctrl,
    pub inval_stag: __be32,
    pub ddp_qn: __be32,
    pub ddp_msn: __be32,
    pub ddp_mo: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_terminate {
    pub ctrl: iwarp_ctrl,
    pub rsvd: __be32,
    pub ddp_qn: __be32,
    pub ddp_msn: __be32,
    pub ddp_mo: __be32,

    pub 4: __be32 layer :,
    pub 4: __be32 etype :,
    pub 8: __be32 ecode :,
    pub 1: __be32 flag_m :,
    pub 1: __be32 flag_d :,
    pub 1: __be32 flag_r :,
    pub 13: __be32 reserved :,

    pub 13: __be32 reserved :,
    pub 1: __be32 flag_r :,
    pub 1: __be32 flag_d :,
    pub 1: __be32 flag_m :,
    pub 8: __be32 ecode :,
    pub 4: __be32 etype :,
    pub 4: __be32 layer :,

}

//
// Terminate Hdr bits & fields
//
// Common portion of iWARP headers (MPA, DDP, RDMAP)
// for an FPDU carrying an untagged DDP segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_ctrl_untagged {
    pub ctrl: iwarp_ctrl,
    pub rsvd: __be32,
    pub ddp_qn: __be32,
    pub ddp_msn: __be32,
    pub ddp_mo: __be32,
}

//
// Common portion of iWARP headers (MPA, DDP, RDMAP)
// for an FPDU carrying a tagged DDP segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_ctrl_tagged {
    pub ctrl: iwarp_ctrl,
    pub ddp_stag: __be32,
    pub ddp_to: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union iwarp_hdr {
    pub ctrl: iwarp_ctrl,
    pub c_untagged: iwarp_ctrl_untagged,
    pub c_tagged: iwarp_ctrl_tagged,
    pub rwrite: iwarp_rdma_write,
    pub rreq: iwarp_rdma_rreq,
    pub rresp: iwarp_rdma_rresp,
    pub terminate: iwarp_terminate,
    pub send: iwarp_send,
    pub send_inv: iwarp_send_inv,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum term_elayer {
    TERM_ERROR_LAYER_RDMAP = 0x00,
    TERM_ERROR_LAYER_DDP = 0x01,
    TERM_ERROR_LAYER_LLP = 0x02 /* eg., MPA */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddp_etype {
    DDP_ETYPE_CATASTROPHIC = 0x0,
    DDP_ETYPE_TAGGED_BUF = 0x1,
    DDP_ETYPE_UNTAGGED_BUF = 0x2,
    DDP_ETYPE_RSVD = 0x3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ddp_ecode {
// unspecified, set to zero
    DDP_ECODE_CATASTROPHIC = 0x00,
// Tagged Buffer Errors
    DDP_ECODE_T_INVALID_STAG = 0x00,
    DDP_ECODE_T_BASE_BOUNDS = 0x01,
    DDP_ECODE_T_STAG_NOT_ASSOC = 0x02,
    DDP_ECODE_T_TO_WRAP = 0x03,
    DDP_ECODE_T_VERSION = 0x04,
// Untagged Buffer Errors
    DDP_ECODE_UT_INVALID_QN = 0x01,
    DDP_ECODE_UT_INVALID_MSN_NOBUF = 0x02,
    DDP_ECODE_UT_INVALID_MSN_RANGE = 0x03,
    DDP_ECODE_UT_INVALID_MO = 0x04,
    DDP_ECODE_UT_MSG_TOOLONG = 0x05,
    DDP_ECODE_UT_VERSION = 0x06
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdmap_untagged_qn {
    RDMAP_UNTAGGED_QN_SEND = 0,
    RDMAP_UNTAGGED_QN_RDMA_READ = 1,
    RDMAP_UNTAGGED_QN_TERMINATE = 2,
    RDMAP_UNTAGGED_QN_COUNT = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdmap_etype {
    RDMAP_ETYPE_CATASTROPHIC = 0x0,
    RDMAP_ETYPE_REMOTE_PROTECTION = 0x1,
    RDMAP_ETYPE_REMOTE_OPERATION = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdmap_ecode {
    RDMAP_ECODE_INVALID_STAG = 0x00,
    RDMAP_ECODE_BASE_BOUNDS = 0x01,
    RDMAP_ECODE_ACCESS_RIGHTS = 0x02,
    RDMAP_ECODE_STAG_NOT_ASSOC = 0x03,
    RDMAP_ECODE_TO_WRAP = 0x04,
    RDMAP_ECODE_VERSION = 0x05,
    RDMAP_ECODE_OPCODE = 0x06,
    RDMAP_ECODE_CATASTROPHIC_STREAM = 0x07,
    RDMAP_ECODE_CATASTROPHIC_GLOBAL = 0x08,
    RDMAP_ECODE_CANNOT_INVALIDATE = 0x09,
    RDMAP_ECODE_UNSPECIFIED = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum llp_ecode {
    LLP_ECODE_TCP_STREAM_LOST = 0x01, /* How to transfer this ?? */
    LLP_ECODE_RECEIVED_CRC = 0x02,
    LLP_ECODE_FPDU_START = 0x03,
    LLP_ECODE_INVALID_REQ_RESP = 0x04,

// Errors for Enhanced Connection Establishment only
    LLP_ECODE_LOCAL_CATASTROPHIC = 0x05,
    LLP_ECODE_INSUFFICIENT_IRD = 0x06,
    LLP_ECODE_NO_MATCHING_RTR = 0x07
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum llp_etype {

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_opcode {
    RDMAP_RDMA_WRITE = 0x0,
    RDMAP_RDMA_READ_REQ = 0x1,
    RDMAP_RDMA_READ_RESP = 0x2,
    RDMAP_SEND = 0x3,
    RDMAP_SEND_INVAL = 0x4,
    RDMAP_SEND_SE = 0x5,
    RDMAP_SEND_SE_INVAL = 0x6,
    RDMAP_TERMINATE = 0x7,
    RDMAP_NOT_SUPPORTED = RDMAP_TERMINATE + 1
}
