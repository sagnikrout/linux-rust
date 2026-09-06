//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/wmm.h
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
// NXP Wireless LAN device driver: WMM
//
// Copyright 2011-2024 NXP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee_types_wmm_aciaifsn_bitmasks {
    NXPWIFI_AIFSN = (BIT(0) | BIT(1) | BIT(2) | BIT(3)),
    NXPWIFI_ACM = BIT(4),
    NXPWIFI_ACI = (BIT(5) | BIT(6)),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee_types_wmm_ecw_bitmasks {
    NXPWIFI_ECW_MIN = (BIT(0) | BIT(1) | BIT(2) | BIT(3)),
    NXPWIFI_ECW_MAX = (BIT(4) | BIT(5) | BIT(6) | BIT(7)),
}

// Retrieve the TID of the given RA list.
extern "C" {
    pub fn nxpwifi_ralist_add(priv: *mut nxpwifi_private, ra: *const u8);
}
extern "C" {
    pub fn nxpwifi_wmm_lists_empty(adapter: *mut nxpwifi_adapter) -> bool;
}
extern "C" {
    pub fn nxpwifi_bypass_txlist_empty(adapter: *mut nxpwifi_adapter) -> bool;
}
extern "C" {
    pub fn nxpwifi_wmm_process_tx(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_process_bypass_tx(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_wmm_init(adapter: *mut nxpwifi_adapter);
}
extern "C" {
    pub fn nxpwifi_wmm_setup_ac_downgrade(priv: *mut nxpwifi_private);
}
extern "C" {
    pub fn nxpwifi_wmm_downgrade_tid(priv: *mut nxpwifi_private, tid: u32) -> u8;
}
// priv, u8 tid, const u8 *ra_addr);
extern "C" {
    pub fn nxpwifi_wmm_init_tos_to_tid_inv(priv: *mut nxpwifi_private);
}
