//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/siw-abi.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Copyright (c) 2008-2019, IBM Corporation

pub const SIW_ABI_VERSION: c_int = 1;
pub const SIW_MAX_SGE: c_int = 6;
pub const SIW_UOBJ_MAX_KEY: c_uint = 0x08FFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_uresp_create_cq {
    pub cq_id: __u32,
    pub num_cqe: __u32,
    pub cq_key: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_uresp_create_qp {
    pub qp_id: __u32,
    pub num_sqe: __u32,
    pub num_rqe: __u32,
    pub pad: __u32,
    pub sq_key: __aligned_u64,
    pub rq_key: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_ureq_reg_mr {
    pub stag_key: __u8,
    pub reserved: [__u8; 3],
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_uresp_reg_mr {
    pub stag: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_uresp_create_srq {
    pub num_rqe: __u32,
    pub pad: __u32,
    pub srq_key: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_uresp_alloc_ctx {
    pub dev_id: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_opcode {
    SIW_OP_WRITE,
    SIW_OP_READ,
    SIW_OP_READ_LOCAL_INV,
    SIW_OP_SEND,
    SIW_OP_SEND_WITH_IMM,
    SIW_OP_SEND_REMOTE_INV,

// Unsupported
    SIW_OP_FETCH_AND_ADD,
    SIW_OP_COMP_AND_SWAP,

    SIW_OP_RECEIVE,
// provider internal SQE
    SIW_OP_READ_RESPONSE,
//
// below opcodes valid for
// in-kernel clients only
//
    SIW_OP_INVAL_STAG,
    SIW_OP_REG_MR,
    SIW_NUM_OPCODES
}

// Keep it same as ibv_sge to allow for memcpy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_sge {
    pub laddr: __aligned_u64,
    pub length: __u32,
    pub lkey: __u32,
}

//
// Inline data are kept within the work request itself occupying
// the space of sge[1] .. sge[n]. Therefore, inline data cannot be
// supported if SIW_MAX_SGE is below 2 elements.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_wqe_flags {
    SIW_WQE_VALID = 1,
    SIW_WQE_INLINE = (1 << 1),
    SIW_WQE_SIGNALLED = (1 << 2),
    SIW_WQE_SOLICITED = (1 << 3),
    SIW_WQE_READ_FENCE = (1 << 4),
    SIW_WQE_REM_INVAL = (1 << 5),
    SIW_WQE_COMPLETED = (1 << 6)
}

// Send Queue Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_sqe {
    pub id: __aligned_u64,
    pub flags: __u16,
    pub num_sge: __u8,
// Contains enum siw_opcode values
    pub opcode: __u8,
    pub rkey: __u32,
    pub raddr: __aligned_u64,
    pub base_mr: __aligned_u64,
}

// Receive Queue Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_rqe {
    pub id: __aligned_u64,
    pub flags: __u16,
    pub num_sge: __u8,
//
// only used by kernel driver,
// ignored if set by user
//
    pub opcode: __u8,
    pub unused: __u32,
    pub sge: [siw_sge; SIW_MAX_SGE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_notify_flags {
    SIW_NOTIFY_NOT = (0),
    SIW_NOTIFY_SOLICITED = (1 << 0),
    SIW_NOTIFY_NEXT_COMPLETION = (1 << 1),
    SIW_NOTIFY_MISSED_EVENTS = (1 << 2),
    SIW_NOTIFY_ALL = SIW_NOTIFY_SOLICITED | SIW_NOTIFY_NEXT_COMPLETION |
    SIW_NOTIFY_MISSED_EVENTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum siw_wc_status {
    SIW_WC_SUCCESS,
    SIW_WC_LOC_LEN_ERR,
    SIW_WC_LOC_PROT_ERR,
    SIW_WC_LOC_QP_OP_ERR,
    SIW_WC_WR_FLUSH_ERR,
    SIW_WC_BAD_RESP_ERR,
    SIW_WC_LOC_ACCESS_ERR,
    SIW_WC_REM_ACCESS_ERR,
    SIW_WC_REM_INV_REQ_ERR,
    SIW_WC_GENERAL_ERR,
    SIW_NUM_WC_STATUS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_cqe {
    pub id: __aligned_u64,
    pub flags: __u8,
    pub opcode: __u8,
    pub status: __u16,
    pub bytes: __u32,
    pub imm_data: __aligned_u64,
    pub inval_stag: __u32,
}

// QP number or QP pointer
//
// Shared structure between user and kernel
// to control CQ arming.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siw_cq_ctrl {
    pub flags: __u32,
    pub pad: __u32,
}
