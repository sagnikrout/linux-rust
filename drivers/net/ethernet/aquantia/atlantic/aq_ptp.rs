//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_ptp.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_ptp.h: Declaration of PTP functions.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_ptp_state {
    AQ_PTP_NO_LINK = 0,
    AQ_PTP_FIRST_INIT = 1,
    AQ_PTP_LINK_UP = 2,
}

pub const PTP_8TC_RING_IDX: c_int = 8;
pub const PTP_4TC_RING_IDX: c_int = 16;
pub const PTP_HWST_RING_IDX: c_int = 31;
// Index must to be 8 (8 TCs) or 16 (4 TCs).
// It depends from Traffic Class mode.
//

// Common functions
extern "C" {
    pub fn aq_ptp_init(aq_nic: *mut aq_nic_s, idx_ptp_vec: c_uint) -> c_int;
}
extern "C" {
    pub fn aq_ptp_unregister(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_free(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_irq_alloc(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_ptp_irq_free(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_ring_alloc(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_ptp_ring_free(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_ring_init(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_ptp_ring_start(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_ptp_ring_stop(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_ring_deinit(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_service_task(aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ptp_tm_offset_set(aq_nic: *mut aq_nic_s, mbps: c_uint);
}
extern "C" {
    pub fn aq_ptp_clock_init(aq_nic: *mut aq_nic_s, state: aq_ptp_state);
}
extern "C" {
    pub fn aq_ptp_tx_skb_drop_head(aq_nic: *mut aq_nic_s);
}
// Traffic processing functions
extern "C" {
    pub fn aq_ptp_xmit(aq_nic: *mut aq_nic_s, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn aq_ptp_tx_hwtstamp(aq_nic: *mut aq_nic_s, timestamp: u64);
}
// Must be to check available of PTP before call
// Return either ring is belong to PTP or not
extern "C" {
    pub fn aq_ptp_ring(aq_nic: *mut aq_nic_s, ring: *mut aq_ring_s) -> bool;
}
extern "C" {
    pub fn aq_ptp_link_change(aq_nic: *mut aq_nic_s) -> c_int;
}
// PTP ring statistics
extern "C" {
    pub fn aq_ptp_get_ring_cnt(aq_nic: *mut aq_nic_s, ring_type: atl_ring_type) -> c_int;
}

