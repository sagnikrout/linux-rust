//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/base.h
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


// -
// Copyright (c) 2002-2007 Sam Leffler, Errno Consulting
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// similar to the "NO WARRANTY" disclaimer below ("Disclaimer") and any
// redistribution must be conditioned upon including a substantially
// similar Disclaimer requirement for further binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF NONINFRINGEMENT, MERCHANTIBILITY
// AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY,
// OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER
// IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
// THE POSSIBILITY OF SUCH DAMAGES.
//
// Definitions for the Atheros Wireless LAN controller driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_srev_type {
    AR5K_VERSION_MAC,
    AR5K_VERSION_RAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_srev_name {
    pub sr_name: *const c_char,
    pub sr_type: ath5k_srev_type,
    pub sr_val: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_buf {
    pub list: list_head,
    pub /: *mut *mut *mut ath5k_desc desc; / virtual addr of desc,
    pub /: *mut *mut dma_addr_t daddr; / physical addr of desc,
    pub /: *mut *mut *mut sk_buff skb; / skbuff for buf,
    pub /: *mut *mut dma_addr_t skbaddr; / physical addr of skb data,
    pub /: *mut *mut ieee80211_tx_rate rates[4]; / number of multi-rate stages,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_vif {
    pub /: *mut *mut bool assoc; / are we associated or not,
    pub opmode: nl80211_iftype,
    pub bslot: c_int,
    pub /: *mut *mut *mut ath5k_buf bbuf; / beacon buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_vif_iter_data {
    pub hw_macaddr: *const u8,
    pub mask: [u8; ETH_ALEN],
    pub /: *mut *mut u8 active_mac[ETH_ALEN]; / first active MAC,
    pub need_set_hw_addr: bool,
    pub found_active: bool,
    pub any_assoc: bool,
    pub opmode: nl80211_iftype,
    pub n_stas: c_int,
}

extern "C" {
    pub fn ath5k_vif_iter(data: *mut c_void, mac: *mut u8, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn ath5k_any_vif_assoc(ah: *mut ath5k_hw) -> bool;
}
extern "C" {
    pub fn ath5k_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn ath5k_beacon_update_timers(ah: *mut ath5k_hw, bc_tsf: u64);
}
extern "C" {
    pub fn ath5k_beacon_update(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn ath5k_beacon_config(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_set_beacon_filter(hw: *mut ieee80211_hw, enable: bool);
}
extern "C" {
    pub fn ath5k_chan_set(ah: *mut ath5k_hw, chandef: *mut cfg80211_chan_def) -> c_int;
}
extern "C" {
    pub fn ath5k_txbuf_free_skb(ah: *mut ath5k_hw, bf: *mut ath5k_buf);
}
extern "C" {
    pub fn ath5k_rxbuf_free_skb(ah: *mut ath5k_hw, bf: *mut ath5k_buf);
}
extern "C" {
    pub fn ath5k_init_ah(ah: *mut ath5k_hw, bus_ops: *const ath_bus_ops) -> c_int;
}
extern "C" {
    pub fn ath5k_deinit_ah(ah: *mut ath5k_hw);
}
// Check whether BSSID mask is supported

// Check whether virtual EOL is supported

