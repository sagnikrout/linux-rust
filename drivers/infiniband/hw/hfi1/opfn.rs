//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/opfn.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2018 Intel Corporation.
//
// DOC: Omni Path Feature Negotion (OPFN)
//
// OPFN is a discovery protocol for Intel Omni-Path fabric that
// allows two RC QPs to negotiate a common feature that both QPs
// can support. Currently, the only OPA feature that OPFN
// supports is TID RDMA.
//
// Architecture
//
// OPFN involves the communication between two QPs on the HFI
// level on an Omni-Path fabric, and ULPs have no knowledge of
// OPFN at all.
//
// Implementation
//
// OPFN extends the existing IB RC protocol with the following
// changes:
// -- Uses Bit 24 (reserved) of DWORD 1 of Base Transport
// Header (BTH1) to indicate that the RC QP supports OPFN;
// -- Uses a combination of RC COMPARE_SWAP opcode (0x13) and
// the address U64_MAX (0xFFFFFFFFFFFFFFFF) as an OPFN
// request; The 64-bit data carried with the request/response
// contains the parameters for negotiation and will be
// defined in tid_rdma.c file;
// -- Defines IB_WR_RESERVED3 as IB_WR_OPFN.
//
// The OPFN communication will be triggered when an RC QP
// receives a request with Bit 24 of BTH1 set. The responder QP
// will then post send an OPFN request with its local
// parameters, which will be sent to the requester QP once all
// existing requests on the responder QP side have been sent.
// Once the requester QP receives the OPFN request, it will
// keep a copy of the responder QP's parameters, and return a
// response packet with its own local parameters. The responder
// QP receives the response packet and keeps a copy of the requester
// QP's parameters. After this exchange, each side has the parameters
// for both sides and therefore can select the right parameters
// for future transactions
//

// STL Verbs Extended
pub const IB_BTHE_E_SHIFT: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi1_opfn_codes {
    STL_VERBS_EXTD_NONE = 0,
    STL_VERBS_EXTD_TID_RDMA,
    STL_VERBS_EXTD_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_opfn_data {
    pub extended: u8,
    pub requested: u16,
    pub completed: u16,
    pub curr: hfi1_opfn_codes,
// serialize opfn function calls
    pub lock: spinlock_t,
    pub opfn_work: work_struct,
}

// WR opcode for OPFN

extern "C" {
    pub fn opfn_send_conn_request(work: *mut work_struct);
}
extern "C" {
    pub fn opfn_conn_reply(qp: *mut rvt_qp, data: u64);
}
extern "C" {
    pub fn opfn_conn_error(qp: *mut rvt_qp);
}
extern "C" {
    pub fn opfn_qp_init(qp: *mut rvt_qp, attr: *mut ib_qp_attr, attr_mask: c_int);
}
extern "C" {
    pub fn opfn_trigger_conn_request(qp: *mut rvt_qp, bth1: u32);
}
extern "C" {
    pub fn opfn_init() -> c_int;
}
extern "C" {
    pub fn opfn_exit();
}
