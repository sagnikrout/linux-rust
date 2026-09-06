//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/debug.h
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
// Copyright (c) 2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATH6K_DEBUG_MASK {
    ATH6KL_DBG_CREDIT	= BIT(0),
// hole
    ATH6KL_DBG_WLAN_TX      = BIT(2),     /* wlan tx */
    ATH6KL_DBG_WLAN_RX      = BIT(3),     /* wlan rx */
    ATH6KL_DBG_BMI		= BIT(4),     /* bmi tracing */
    ATH6KL_DBG_HTC		= BIT(5),
    ATH6KL_DBG_HIF		= BIT(6),
    ATH6KL_DBG_IRQ		= BIT(7),     /* interrupt processing */
// hole
    ATH6KL_DBG_WMI          = BIT(10),    /* wmi tracing */
    ATH6KL_DBG_TRC	        = BIT(11),    /* generic func tracing */
    ATH6KL_DBG_SCATTER	= BIT(12),    /* hif scatter tracing */
    ATH6KL_DBG_WLAN_CFG     = BIT(13),    /* cfg80211 i/f file tracing */
    ATH6KL_DBG_RAW_BYTES    = BIT(14),    /* dump tx/rx frames */
    ATH6KL_DBG_AGGR		= BIT(15),    /* aggregation */
    ATH6KL_DBG_SDIO		= BIT(16),
    ATH6KL_DBG_SDIO_DUMP	= BIT(17),
    ATH6KL_DBG_BOOT		= BIT(18),    /* driver init and fw boot */
    ATH6KL_DBG_WMI_DUMP	= BIT(19),
    ATH6KL_DBG_SUSPEND	= BIT(20),
    ATH6KL_DBG_USB		= BIT(21),
    ATH6KL_DBG_USB_BULK	= BIT(22),
    ATH6KL_DBG_RECOVERY	= BIT(23),
    ATH6KL_DBG_ANY	        = 0xffffffff  /* enable all logs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_war {
    ATH6KL_WAR_INVALID_RATE,
}

extern "C" {
    pub fn ath6kl_read_tgt_stats(ar: *mut ath6kl, vif: *mut ath6kl_vif) -> c_int;
}

extern "C" {
    pub fn ath6kl_dbg(mask: ATH6K_DEBUG_MASK, fmt: *const c_char, ...);
}
extern "C" {
    pub fn dump_cred_dist_stats(target: *mut htc_target);
}
extern "C" {
    pub fn ath6kl_debug_fwlog_event(ar: *mut ath6kl, buf: *const c_void, len: usize);
}
extern "C" {
    pub fn ath6kl_debug_war(ar: *mut ath6kl, war: ath6kl_war);
}
extern "C" {
    pub fn ath6kl_debug_set_keepalive(ar: *mut ath6kl, keepalive: u8);
}
extern "C" {
    pub fn ath6kl_debug_set_disconnect_timeout(ar: *mut ath6kl, timeout: u8);
}
extern "C" {
    pub fn ath6kl_debug_init(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_debug_init_fs(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_debug_cleanup(ar: *mut ath6kl);
}

