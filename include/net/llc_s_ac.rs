//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_s_ac.h
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
// SAP component actions
pub const SAP_ACT_UNITDATA_IND: c_int = 1;
pub const SAP_ACT_SEND_UI: c_int = 2;
pub const SAP_ACT_SEND_XID_C: c_int = 3;
pub const SAP_ACT_SEND_XID_R: c_int = 4;
pub const SAP_ACT_SEND_TEST_C: c_int = 5;
pub const SAP_ACT_SEND_TEST_R: c_int = 6;
pub const SAP_ACT_REPORT_STATUS: c_int = 7;
pub const SAP_ACT_XID_IND: c_int = 8;
pub const SAP_ACT_TEST_IND: c_int = 9;
// All action functions must look like this
extern "C" {
    pub fn int(sap: *mut *mut llc_sap_action_t)(struct llc_sap, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn llc_sap_action_unitdata_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_send_ui(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_send_xid_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_send_xid_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_send_test_c(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_send_test_r(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_report_status(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_xid_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_sap_action_test_ind(sap: *mut llc_sap, skb: *mut sk_buff) -> c_int;
}
