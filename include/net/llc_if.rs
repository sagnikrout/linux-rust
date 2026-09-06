//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_if.h
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
// Defines LLC interface to network layer
// Available primitives

pub const LLC_DATAUNIT_PRIM: c_int = 1;
pub const LLC_CONN_PRIM: c_int = 2;
pub const LLC_DATA_PRIM: c_int = 3;
pub const LLC_DISC_PRIM: c_int = 4;
pub const LLC_RESET_PRIM: c_int = 5;

pub const LLC_DISABLE_PRIM: c_int = 7;
pub const LLC_XID_PRIM: c_int = 8;
pub const LLC_TEST_PRIM: c_int = 9;
pub const LLC_SAP_ACTIVATION: c_int = 10;
pub const LLC_SAP_DEACTIVATION: c_int = 11;
pub const LLC_NBR_PRIMITIVES: c_int = 11;
pub const LLC_IND: c_int = 1;
pub const LLC_CONFIRM: c_int = 2;
// Primitive type
pub const LLC_PRIM_TYPE_REQ: c_int = 1;
pub const LLC_PRIM_TYPE_IND: c_int = 2;
pub const LLC_PRIM_TYPE_RESP: c_int = 3;
pub const LLC_PRIM_TYPE_CONFIRM: c_int = 4;
// Reset reasons, remote entity or local LLC
pub const LLC_RESET_REASON_REMOTE: c_int = 1;
pub const LLC_RESET_REASON_LOCAL: c_int = 2;
// Disconnect reasons
pub const LLC_DISC_REASON_RX_DM_RSP_PDU: c_int = 0;
pub const LLC_DISC_REASON_RX_DISC_CMD_PDU: c_int = 1;
pub const LLC_DISC_REASON_ACK_TMR_EXP: c_int = 2;
// Confirm reasons

extern "C" {
    pub fn llc_build_and_send_pkt(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_send_disc(sk: *mut sock) -> c_int;
}
