//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_ptp.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2017 Chelsio Communications, Inc. All rights reserved.
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
// Maximum parts-per-billion adjustment that is acceptable
pub const MAX_PTP_FREQ_ADJ: c_int = 1000000;

pub const PTP_MIN_LENGTH: c_int = 63;
pub const PTP_IN_TRANSMIT_PACKET_MAXNUM: c_int = 240;
pub const PTP_EVENT_PORT: c_int = 319;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_rx_filter_mode {
    PTP_TS_NONE = 0,
    PTP_TS_L2,
    PTP_TS_L4,
    PTP_TS_L2_L4
}

extern "C" {
    pub fn cxgb4_ptp_init(adap: *mut adapter);
}
extern "C" {
    pub fn cxgb4_ptp_stop(adap: *mut adapter);
}
extern "C" {
    pub fn cxgb4_ptp_is_ptp_tx(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn cxgb4_ptp_is_ptp_rx(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn cxgb4_ptprx_timestamping(pi: *mut port_info, port: u8, mode: u16) -> c_int;
}
extern "C" {
    pub fn cxgb4_ptp_redirect_rx_packet(adap: *mut adapter, pi: *mut port_info) -> c_int;
}
extern "C" {
    pub fn cxgb4_ptp_txtype(adap: *mut adapter, port_id: u8) -> c_int;
}
extern "C" {
    pub fn cxgb4_ptp_read_hwstamp(adap: *mut adapter, pi: *mut port_info);
}
extern "C" {
    pub fn is_ptp_enabled(skb: *mut sk_buff, dev: *mut net_device) -> bool;
}
