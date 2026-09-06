//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_s_ev.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 1997 by Procom Technology,Inc.
// 2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

// Defines SAP component events
// Types of events (possible values in 'ev->type')
pub const LLC_SAP_EV_TYPE_SIMPLE: c_int = 1;
pub const LLC_SAP_EV_TYPE_CONDITION: c_int = 2;
pub const LLC_SAP_EV_TYPE_PRIM: c_int = 3;

pub const LLC_SAP_EV_TYPE_ACK_TMR: c_int = 5;
pub const LLC_SAP_EV_TYPE_RPT_STATUS: c_int = 6;
pub const LLC_SAP_EV_ACTIVATION_REQ: c_int = 1;
pub const LLC_SAP_EV_RX_UI: c_int = 2;
pub const LLC_SAP_EV_UNITDATA_REQ: c_int = 3;
pub const LLC_SAP_EV_XID_REQ: c_int = 4;
pub const LLC_SAP_EV_RX_XID_C: c_int = 5;
pub const LLC_SAP_EV_RX_XID_R: c_int = 6;
pub const LLC_SAP_EV_TEST_REQ: c_int = 7;
pub const LLC_SAP_EV_RX_TEST_C: c_int = 8;
pub const LLC_SAP_EV_RX_TEST_R: c_int = 9;
pub const LLC_SAP_EV_DEACTIVATION_REQ: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_sap_state_ev {
    pub prim: u8,
    pub prim_type: u8,
    pub type: u8,
    pub reason: u8,
    pub ind_cfm_flag: u8,
    pub saddr: llc_addr,
    pub daddr: llc_addr,
}

extern "C" {
    pub fn int(sap: *mut *mut llc_sap_ev_t)(struct llc_sap, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn llc_sap_ev_activation_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_rx_ui(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_unitdata_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_xid_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_rx_xid_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_rx_xid_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_test_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_rx_test_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_rx_test_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_ev_deactivation_req(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
