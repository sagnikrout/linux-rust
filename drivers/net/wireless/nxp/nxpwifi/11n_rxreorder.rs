//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/11n_rxreorder.h
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
// NXP Wireless LAN device driver: 802.11n RX Re-ordering
//
// Copyright 2011-2024 NXP
//
pub const MIN_FLUSH_TIMER_MS: c_int = 50;
pub const MIN_FLUSH_TIMER_15_MS: c_int = 15;
pub const NXPWIFI_BA_WIN_SIZE_32: c_int = 32;
pub const PKT_TYPE_BAR: c_uint = 0xE7;

pub const BLOCKACKPARAM_TID_POS: c_int = 2;
pub const BLOCKACKPARAM_WINSIZE_POS: c_int = 6;
pub const DELBA_TID_POS: c_int = 12;
pub const DELBA_INITIATOR_POS: c_int = 11;
pub const TYPE_DELBA_SENT: c_int = 1;
pub const TYPE_DELBA_RECEIVE: c_int = 2;
pub const IMMEDIATE_BLOCK_ACK: c_uint = 0x2;
pub const ADDBA_RSP_STATUS_ACCEPT: c_int = 0;
pub const NXPWIFI_DEF_11N_RX_SEQ_NUM: c_uint = 0xffff;
pub const BA_SETUP_MAX_PACKET_THRESHOLD: c_int = 16;
pub const BA_SETUP_PACKET_OFFSET: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxpwifi_rxreor_flags {
    RXREOR_FORCE_NO_DROP		= 1 << 0,
    RXREOR_INIT_WINDOW_SHIFT	= 1 << 1,
}

// resp);
// cmd_addba_req);
extern "C" {
    pub fn nxpwifi_11n_cleanup_reorder_tbl(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_11n_del_rx_reorder_tbl_by_ta(priv: *mut nxpwifi_private, ta: *mut u8);
}
extern "C" {
    pub fn nxpwifi_update_rxreor_flags(adapter: *mut nxpwifi_adapter, flags: u8);
}
