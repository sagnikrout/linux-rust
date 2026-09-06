//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_tcb.h
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
// This file is part of the Chelsio T4/T5/T6 Ethernet driver for Linux.
//
// Copyright (c) 2017 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const TCB_L2T_IX_W: c_int = 0;
pub const TCB_L2T_IX_S: c_int = 12;
pub const TCB_L2T_IX_M: c_uint = 0xfffULL;

pub const TCB_T_FLAGS_W: c_int = 1;
pub const TCB_T_FLAGS_S: c_int = 0;
pub const TCB_T_FLAGS_M: c_uint = 0xffffffffffffffffULL;

pub const TCB_FIELD_COOKIE_TFLAG: c_int = 1;
pub const TCB_SMAC_SEL_W: c_int = 0;
pub const TCB_SMAC_SEL_S: c_int = 24;
pub const TCB_SMAC_SEL_M: c_uint = 0xffULL;

pub const TCB_T_FLAGS_W: c_int = 1;
pub const TCB_T_FLAGS_S: c_int = 0;
pub const TCB_T_FLAGS_M: c_uint = 0xffffffffffffffffULL;

pub const TF_DROP_S: c_int = 22;
pub const TF_DIRECT_STEER_S: c_int = 23;
pub const TF_LPBK_S: c_int = 59;
pub const TF_CCTRL_ECE_S: c_int = 60;
pub const TF_CCTRL_CWR_S: c_int = 61;
pub const TF_CCTRL_RFR_S: c_int = 62;
pub const TCB_RSS_INFO_W: c_int = 3;
pub const TCB_RSS_INFO_S: c_int = 0;
pub const TCB_RSS_INFO_M: c_uint = 0x3ffULL;

pub const TCB_T_STATE_W: c_int = 3;
pub const TCB_T_STATE_S: c_int = 16;
pub const TCB_T_STATE_M: c_uint = 0xfULL;

pub const TCB_TIMESTAMP_W: c_int = 5;
pub const TCB_TIMESTAMP_S: c_int = 0;
pub const TCB_TIMESTAMP_M: c_uint = 0xffffffffULL;

pub const TCB_RTT_TS_RECENT_AGE_W: c_int = 6;
pub const TCB_RTT_TS_RECENT_AGE_S: c_int = 0;
pub const TCB_RTT_TS_RECENT_AGE_M: c_uint = 0xffffffffULL;

pub const TCB_T_RTSEQ_RECENT_W: c_int = 7;
pub const TCB_T_RTSEQ_RECENT_S: c_int = 0;
pub const TCB_T_RTSEQ_RECENT_M: c_uint = 0xffffffffULL;

pub const TCB_TX_MAX_W: c_int = 9;
pub const TCB_TX_MAX_S: c_int = 0;
pub const TCB_TX_MAX_M: c_uint = 0xffffffffULL;

pub const TCB_SND_UNA_RAW_W: c_int = 10;
pub const TCB_SND_UNA_RAW_S: c_int = 0;
pub const TCB_SND_UNA_RAW_M: c_uint = 0xfffffffULL;

pub const TCB_SND_NXT_RAW_W: c_int = 10;
pub const TCB_SND_NXT_RAW_S: c_int = 28;
pub const TCB_SND_NXT_RAW_M: c_uint = 0xfffffffULL;

pub const TCB_SND_MAX_RAW_W: c_int = 11;
pub const TCB_SND_MAX_RAW_S: c_int = 24;
pub const TCB_SND_MAX_RAW_M: c_uint = 0xfffffffULL;

pub const TCB_RCV_NXT_W: c_int = 16;
pub const TCB_RCV_NXT_S: c_int = 10;
pub const TCB_RCV_NXT_M: c_uint = 0xffffffffULL;

pub const TCB_RCV_WND_W: c_int = 17;
pub const TCB_RCV_WND_S: c_int = 10;
pub const TCB_RCV_WND_M: c_uint = 0xffffffULL;

pub const TCB_RX_FRAG2_PTR_RAW_W: c_int = 27;
pub const TCB_RX_FRAG3_LEN_RAW_W: c_int = 29;
pub const TCB_RX_FRAG3_START_IDX_OFFSET_RAW_W: c_int = 30;
pub const TCB_PDU_HDR_LEN_W: c_int = 31;
pub const TCB_RQ_START_W: c_int = 30;
pub const TCB_RQ_START_S: c_int = 0;
pub const TCB_RQ_START_M: c_uint = 0x3ffffffULL;

pub const TF_RX_PDU_OUT_S: c_int = 49;

pub const TF_CORE_BYPASS_S: c_int = 63;

pub const TF_NON_OFFLOAD_S: c_int = 1;

