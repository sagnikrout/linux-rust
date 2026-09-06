//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mana/counters.h
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
// Copyright (c) 2024 Microsoft Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_port_counters {
    MANA_IB_REQUESTER_TIMEOUT,
    MANA_IB_REQUESTER_OOS_NAK,
    MANA_IB_REQUESTER_RNR_NAK,
    MANA_IB_RESPONDER_RNR_NAK,
    MANA_IB_RESPONDER_OOS,
    MANA_IB_RESPONDER_DUP_REQUEST,
    MANA_IB_REQUESTER_IMPLICIT_NAK,
    MANA_IB_REQUESTER_READRESP_PSN_MISMATCH,
    MANA_IB_NAK_INV_REQ,
    MANA_IB_NAK_ACCESS_ERR,
    MANA_IB_NAK_OPP_ERR,
    MANA_IB_NAK_INV_READ,
    MANA_IB_RESPONDER_LOCAL_LEN_ERR,
    MANA_IB_REQUESTOR_LOCAL_PROT_ERR,
    MANA_IB_RESPONDER_REM_ACCESS_ERR,
    MANA_IB_RESPONDER_LOCAL_QP_ERR,
    MANA_IB_RESPONDER_MALFORMED_WQE,
    MANA_IB_GENERAL_HW_ERR,
    MANA_IB_REQUESTER_RNR_NAK_RETRIES_EXCEEDED,
    MANA_IB_REQUESTER_RETRIES_EXCEEDED,
    MANA_IB_TOTAL_FATAL_ERR,
    MANA_IB_RECEIVED_CNPS,
    MANA_IB_NUM_QPS_CONGESTED,
    MANA_IB_RATE_INC_EVENTS,
    MANA_IB_NUM_QPS_RECOVERED,
    MANA_IB_CURRENT_RATE,
    MANA_IB_DUP_RX_REQ,
    MANA_IB_TX_BYTES,
    MANA_IB_RX_BYTES,
    MANA_IB_RX_SEND_REQ,
    MANA_IB_RX_WRITE_REQ,
    MANA_IB_RX_READ_REQ,
    MANA_IB_TX_PKT,
    MANA_IB_RX_PKT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_ib_device_counters {
    MANA_IB_SENT_CNPS,
    MANA_IB_RECEIVED_ECNS,
    MANA_IB_RECEIVED_CNP_COUNT,
    MANA_IB_QP_CONGESTED_EVENTS,
    MANA_IB_QP_RECOVERED_EVENTS,
    MANA_IB_DEV_RATE_INC_EVENTS,
}
