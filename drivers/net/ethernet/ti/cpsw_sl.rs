//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/cpsw_sl.h
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
// Texas Instruments Ethernet Switch media-access-controller (MAC) submodule
// Ethernet MAC Sliver (CPGMAC_SL) APIs
//
// Copyright (C) 2019 Texas Instruments
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpsw_sl_regs {
    CPSW_SL_IDVER,
    CPSW_SL_MACCONTROL,
    CPSW_SL_MACSTATUS,
    CPSW_SL_SOFT_RESET,
    CPSW_SL_RX_MAXLEN,
    CPSW_SL_BOFFTEST,
    CPSW_SL_RX_PAUSE,
    CPSW_SL_TX_PAUSE,
    CPSW_SL_EMCONTROL,
    CPSW_SL_RX_PRI_MAP,
    CPSW_SL_TX_GAP,
}

extern "C" {
    pub fn cpsw_sl_reset(sl: *mut cpsw_sl, tmo: c_ulong);
}
extern "C" {
    pub fn cpsw_sl_ctl_set(sl: *mut cpsw_sl, ctl_funcs: u32) -> u32;
}
extern "C" {
    pub fn cpsw_sl_ctl_clr(sl: *mut cpsw_sl, ctl_funcs: u32) -> u32;
}
extern "C" {
    pub fn cpsw_sl_ctl_reset(sl: *mut cpsw_sl);
}
extern "C" {
    pub fn cpsw_sl_wait_for_idle(sl: *mut cpsw_sl, tmo: c_ulong) -> c_int;
}
extern "C" {
    pub fn cpsw_sl_reg_read(sl: *mut cpsw_sl, reg: cpsw_sl_regs) -> u32;
}
extern "C" {
    pub fn cpsw_sl_reg_write(sl: *mut cpsw_sl, reg: cpsw_sl_regs, val: u32);
}
