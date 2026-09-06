//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/hw-ops.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
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

// Hardware core and driver accessible callbacks
extern "C" {
    pub fn ath9k_hw_ops(_arg: ah)->calibrate(ah, _arg: chan, _arg: rxchainmask, _arg: longcal) -> return;
}
extern "C" {
    pub fn ath9k_hw_ops(_arg: ah)->get_isr(ah, _arg: masked, _arg: sync_cause_p) -> return;
}
extern "C" {
    pub fn ath9k_hw_ops(_arg: ah)->set_txdesc(ah, _arg: ds, _arg: i) -> return;
}
extern "C" {
    pub fn ath9k_hw_ops(_arg: ah)->proc_txdesc(ah, _arg: ds, _arg: ts) -> return;
}
extern "C" {
    pub fn ath9k_hw_ops(_arg: ah)->get_duration(ah, _arg: ds, _arg: index) -> return;
}

extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->is_aic_enabled(ah) -> return;
}

// Private hardware call ops
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->detect_mac_hang(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->detect_bb_hang(ah) -> return;
}
// PHY ops
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->rf_set_freq(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->set_rf_regs(ah, _arg: chan, _arg: modesIndex) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->init_bb(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->set_channel_regs(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->process_ini(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->olc_init(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->set_rfmode(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->mark_phy_inactive(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->set_delta_slope(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->rfbus_req(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->rfbus_done(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->restore_chainmask(ah) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->ani_control(ah, _arg: cmd, _arg: param) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->init_cal(ah, _arg: chan) -> return;
}
extern "C" {
    pub fn ath9k_hw_private_ops(_arg: ah)->compute_pll_control(ah, _arg: chan) -> return;
}
