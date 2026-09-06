//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_tc_mqprio.h
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
// Copyright (C) 2019 Chelsio Communications.  All rights reserved.

pub const CXGB4_EOSW_TXQ_DEFAULT_DESC_NUM: c_int = 128;
pub const CXGB4_EOHW_TXQ_DEFAULT_DESC_NUM: c_int = 1024;
pub const CXGB4_EOHW_RXQ_DEFAULT_DESC_NUM: c_int = 1024;
pub const CXGB4_EOHW_RXQ_DEFAULT_DESC_SIZE: c_int = 64;
pub const CXGB4_EOHW_RXQ_DEFAULT_INTR_USEC: c_int = 5;
pub const CXGB4_EOHW_RXQ_DEFAULT_PKT_CNT: c_int = 8;
pub const CXGB4_EOHW_FLQ_DEFAULT_DESC_NUM: c_int = 72;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_mqprio_state {
    CXGB4_MQPRIO_STATE_DISABLED = 0,
    CXGB4_MQPRIO_STATE_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_tc_port_mqprio {
    pub /: *mut *mut cxgb4_mqprio_state state; / Current MQPRIO offload state,
    pub /: *mut *mut tc_mqprio_qopt_offload mqprio; / MQPRIO offload params,
    pub /: *mut *mut *mut sge_eosw_txq eosw_txq; / Netdev SW Tx queue array,
    pub /: *mut *mut u8 tc_hwtc_map[TC_QOPT_MAX_QUEUE]; / MQPRIO tc to hardware tc map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_tc_mqprio {
    pub /: *mut *mut refcount_t refcnt; / Refcount for adapter-wide resources,
    pub /: *mut *mut mutex mqprio_mutex; / Lock for accessing MQPRIO info,
    pub /: *mut *mut *mut cxgb4_tc_port_mqprio port_mqprio; / Per port MQPRIO info,
}

extern "C" {
    pub fn cxgb4_mqprio_stop_offload(adap: *mut adapter);
}
extern "C" {
    pub fn cxgb4_init_tc_mqprio(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn cxgb4_cleanup_tc_mqprio(adap: *mut adapter);
}
