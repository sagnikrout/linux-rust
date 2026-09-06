//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_plog.h
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

pub const BFA_PL_NLOG_ENTS: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_plog_log_type {
    BFA_PL_LOG_TYPE_INVALID	= 0,
    BFA_PL_LOG_TYPE_INT	= 1,
    BFA_PL_LOG_TYPE_STRING	= 2,
}

//
// the (fixed size) record format for each entry in the portlog
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_plog_rec_s {
    pub /: *mut *mut u64 tv; / timestamp,
    pub /: *mut *mut u8 port; / Source port that logged this entry,
    pub /: *mut *mut u8 mid; / module id,
    pub /: *mut *mut u8 eid; / indicates Rx, Tx, IOCTL, etc. bfa_plog_eid,
    pub /: *mut *mut u8 log_type; / string/integer log, bfa_plog_log_type_t,
    pub log_num_ints: u8,
//
// interpreted only if log_type is INT_LOG. indicates number of
// integers in the int_log[] (0-PL_INT_LOG_SZ).
//
    pub rsvd: u8,
    pub /: *mut *mut u16 misc; / can be used to indicate fc frame length,
    pub string_log: [c_char; BFA_PL_STRING_LOG_SZ],
    pub int_log: [u32; BFA_PL_INT_LOG_SZ],
    pub log_entry: },
}

//
// the following #defines will be used by the logging entities to indicate
// their module id. BFAL will convert the integer value to string format
//
// process to be used while changing the following #defines:
// - Always add new entries at the end
// - define corresponding string in BFAL
// - Do not remove any entry or rearrange the order.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_plog_mid {
    BFA_PL_MID_INVALID	= 0,
    BFA_PL_MID_DEBUG	= 1,
    BFA_PL_MID_DRVR		= 2,
    BFA_PL_MID_HAL		= 3,
    BFA_PL_MID_HAL_FCXP	= 4,
    BFA_PL_MID_HAL_UF	= 5,
    BFA_PL_MID_FCS		= 6,
    BFA_PL_MID_LPS		= 7,
    BFA_PL_MID_MAX		= 8
}

pub const BFA_PL_MID_STRLEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_plog_mid_strings_s {
    pub m_str: [c_char; BFA_PL_MID_STRLEN],
}

//
// the following #defines will be used by the logging entities to indicate
// their event type. BFAL will convert the integer value to string format
//
// process to be used while changing the following #defines:
// - Always add new entries at the end
// - define corresponding string in BFAL
// - Do not remove any entry or rearrange the order.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_plog_eid {
    BFA_PL_EID_INVALID		= 0,
    BFA_PL_EID_IOC_DISABLE		= 1,
    BFA_PL_EID_IOC_ENABLE		= 2,
    BFA_PL_EID_PORT_DISABLE		= 3,
    BFA_PL_EID_PORT_ENABLE		= 4,
    BFA_PL_EID_PORT_ST_CHANGE	= 5,
    BFA_PL_EID_TX			= 6,
    BFA_PL_EID_TX_ACK1		= 7,
    BFA_PL_EID_TX_RJT		= 8,
    BFA_PL_EID_TX_BSY		= 9,
    BFA_PL_EID_RX			= 10,
    BFA_PL_EID_RX_ACK1		= 11,
    BFA_PL_EID_RX_RJT		= 12,
    BFA_PL_EID_RX_BSY		= 13,
    BFA_PL_EID_CT_IN		= 14,
    BFA_PL_EID_CT_OUT		= 15,
    BFA_PL_EID_DRIVER_START		= 16,
    BFA_PL_EID_RSCN			= 17,
    BFA_PL_EID_DEBUG		= 18,
    BFA_PL_EID_MISC			= 19,
    BFA_PL_EID_FIP_FCF_DISC		= 20,
    BFA_PL_EID_FIP_FCF_CVL		= 21,
    BFA_PL_EID_LOGIN		= 22,
    BFA_PL_EID_LOGO			= 23,
    BFA_PL_EID_TRUNK_SCN		= 24,
    BFA_PL_EID_MAX
}

pub const BFA_PL_ENAME_STRLEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_plog_eid_strings_s {
    pub e_str: [c_char; BFA_PL_ENAME_STRLEN],
}

pub const BFA_PL_SIG_LEN: c_int = 8;

//
// per port circular log buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_plog_s {
    pub /: *mut *mut char plog_sig[BFA_PL_SIG_LEN]; / Start signature,
    pub plog_enabled: u8,
    pub rsvd: [u8; 7],
    pub ticks: u32,
    pub head: u16,
    pub tail: u16,
    pub plog_recs: [bfa_plog_rec_s; BFA_PL_NLOG_ENTS],
}

extern "C" {
    pub fn bfa_plog_init(plog: *mut bfa_plog_s);
}
