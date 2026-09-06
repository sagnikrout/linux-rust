//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bnxt_re/hw_counters.h
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


//
// Broadcom NetXtreme-E RoCE driver.
//
// Copyright (c) 2016 - 2017, Broadcom. All rights reserved.  The term
// Broadcom refers to Broadcom Limited and/or its subsidiaries.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS''
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS
// BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
// BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
// OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN
// IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Description: Statistics (header)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_re_hw_stats {
    BNXT_RE_RX_PKTS,
    BNXT_RE_RX_BYTES,
    BNXT_RE_TX_PKTS,
    BNXT_RE_TX_BYTES,
    BNXT_RE_RECOVERABLE_ERRORS,
    BNXT_RE_TX_ERRORS,
    BNXT_RE_TX_DISCARDS,
    BNXT_RE_RX_ERRORS,
    BNXT_RE_RX_DISCARDS,
    BNXT_RE_TO_RETRANSMITS,
    BNXT_RE_SEQ_ERR_NAKS_RCVD,
    BNXT_RE_MAX_RETRY_EXCEEDED,
    BNXT_RE_RNR_NAKS_RCVD,
    BNXT_RE_MISSING_RESP,
    BNXT_RE_UNRECOVERABLE_ERR,
    BNXT_RE_BAD_RESP_ERR,
    BNXT_RE_LOCAL_QP_OP_ERR,
    BNXT_RE_LOCAL_PROTECTION_ERR,
    BNXT_RE_MEM_MGMT_OP_ERR,
    BNXT_RE_REMOTE_INVALID_REQ_ERR,
    BNXT_RE_REMOTE_ACCESS_ERR,
    BNXT_RE_REMOTE_OP_ERR,
    BNXT_RE_DUP_REQ,
    BNXT_RE_RES_EXCEED_MAX,
    BNXT_RE_RES_LENGTH_MISMATCH,
    BNXT_RE_RES_EXCEEDS_WQE,
    BNXT_RE_RES_OPCODE_ERR,
    BNXT_RE_RES_RX_INVALID_RKEY,
    BNXT_RE_RES_RX_DOMAIN_ERR,
    BNXT_RE_RES_RX_NO_PERM,
    BNXT_RE_RES_RX_RANGE_ERR,
    BNXT_RE_RES_TX_INVALID_RKEY,
    BNXT_RE_RES_TX_DOMAIN_ERR,
    BNXT_RE_RES_TX_NO_PERM,
    BNXT_RE_RES_TX_RANGE_ERR,
    BNXT_RE_RES_IRRQ_OFLOW,
    BNXT_RE_RES_UNSUP_OPCODE,
    BNXT_RE_RES_UNALIGNED_ATOMIC,
    BNXT_RE_RES_REM_INV_ERR,
    BNXT_RE_RES_MEM_ERROR,
    BNXT_RE_RES_SRQ_ERR,
    BNXT_RE_RES_CMP_ERR,
    BNXT_RE_RES_INVALID_DUP_RKEY,
    BNXT_RE_RES_WQE_FORMAT_ERR,
    BNXT_RE_RES_CQ_LOAD_ERR,
    BNXT_RE_RES_SRQ_LOAD_ERR,
    BNXT_RE_RES_TX_PCI_ERR,
    BNXT_RE_RES_RX_PCI_ERR,
    BNXT_RE_REQ_CQE_ERROR,
    BNXT_RE_RESP_CQE_ERROR,
    BNXT_RE_RESP_REMOTE_ACCESS_ERRS,
    BNXT_RE_OUT_OF_SEQ_ERR,
    BNXT_RE_TX_ATOMIC_REQ,
    BNXT_RE_TX_READ_REQ,
    BNXT_RE_TX_READ_RES,
    BNXT_RE_TX_WRITE_REQ,
    BNXT_RE_TX_SEND_REQ,
    BNXT_RE_TX_ROCE_PKTS,
    BNXT_RE_TX_ROCE_BYTES,
    BNXT_RE_RX_ATOMIC_REQ,
    BNXT_RE_RX_READ_REQ,
    BNXT_RE_RX_READ_RESP,
    BNXT_RE_RX_WRITE_REQ,
    BNXT_RE_RX_SEND_REQ,
    BNXT_RE_RX_ROCE_PKTS,
    BNXT_RE_RX_ROCE_BYTES,
    BNXT_RE_RX_ROCE_GOOD_PKTS,
    BNXT_RE_RX_ROCE_GOOD_BYTES,
    BNXT_RE_OOB,
    BNXT_RE_TX_CNP,
    BNXT_RE_RX_CNP,
    BNXT_RE_RX_ECN,
    BNXT_RE_NUM_EXT_COUNTERS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_db_pacing_stats {
    pub resched: u64,
    pub complete: u64,
    pub alerts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_res_cntrs {
    pub qp_count: core::sync::atomic::AtomicI32,
    pub rc_qp_count: core::sync::atomic::AtomicI32,
    pub ud_qp_count: core::sync::atomic::AtomicI32,
    pub cq_count: core::sync::atomic::AtomicI32,
    pub srq_count: core::sync::atomic::AtomicI32,
    pub mr_count: core::sync::atomic::AtomicI32,
    pub mw_count: core::sync::atomic::AtomicI32,
    pub ah_count: core::sync::atomic::AtomicI32,
    pub pd_count: core::sync::atomic::AtomicI32,
    pub resize_count: core::sync::atomic::AtomicI32,
    pub qp_watermark: u64,
    pub rc_qp_watermark: u64,
    pub ud_qp_watermark: u64,
    pub cq_watermark: u64,
    pub srq_watermark: u64,
    pub mr_watermark: u64,
    pub mw_watermark: u64,
    pub ah_watermark: u64,
    pub pd_watermark: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_rstat {
    pub errs: bnxt_qplib_roce_stats,
    pub ext_stat: bnxt_qplib_ext_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_re_stats {
    pub rstat: bnxt_re_rstat,
    pub res: bnxt_re_res_cntrs,
    pub pacing: bnxt_re_db_pacing_stats,
}
