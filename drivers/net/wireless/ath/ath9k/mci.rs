//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/mci.h
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

pub const ATH_MCI_GPM_MAX_ENTRY: c_int = 16;

pub const ATH_MCI_DEF_BT_PERIOD: c_int = 40;
pub const ATH_MCI_BDR_DUTY_CYCLE: c_int = 20;
pub const ATH_MCI_MAX_DUTY_CYCLE: c_int = 90;

pub const ATH_MCI_MAX_ACL_PROFILE: c_int = 7;
pub const ATH_MCI_MAX_SCO_PROFILE: c_int = 1;

pub const ATH_MCI_INQUIRY_PRIO: c_int = 62;
pub const ATH_MCI_HI_PRIO: c_int = 60;
pub const ATH_MCI_NUM_BT_CHANNELS: c_int = 79;
pub const ATH_MCI_CONCUR_TX_SWITCH: c_int = 5;

// (((u8 *)(_p_gpm)) + MCI_GPM_COEX_B_CHANNEL_MAP + \

// (((u8 *)(_p_gpm)) + MCI_GPM_COEX_B_CHANNEL_MAP + \

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_mci_profile_info {
    pub type: u8,
    pub conn_handle: u8,
    pub start: bool,
    pub master: bool,
    pub edr: bool,
    pub voice_type: u8,
    pub /: *mut *mut u16 T; / Voice: Tvoice, HID: Tsniff, in slots,
    pub /: *mut *mut u8 W; / Voice: Wvoice, HID: Sniff timeout, in slots,
    pub /: *mut *mut u8 A; / HID: Sniff attempt, in slots,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_mci_profile_status {
    pub is_critical: bool,
    pub is_link: bool,
    pub conn_handle: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_mci_profile {
    pub info: list_head,
    pub ATH_MCI_MAX_PROFILE): DECLARE_BITMAP(status,,
    pub aggr_limit: u16,
    pub num_mgmt: u8,
    pub num_sco: u8,
    pub num_a2dp: u8,
    pub num_hid: u8,
    pub num_pan: u8,
    pub num_other_acl: u8,
    pub num_bdr: u8,
    pub voice_priority: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_mci_buf {
    pub /: *mut *mut *mut void bf_addr; / virtual addr of desc,
    pub /: *mut *mut dma_addr_t bf_paddr; / physical addr of buffer,
    pub /: *mut *mut u32 bf_len; / len of data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_mci_coex {
    pub sched_buf: ath_mci_buf,
    pub gpm_buf: ath_mci_buf,
}

extern "C" {
    pub fn ath_mci_flush_profile(mci: *mut ath_mci_profile);
}
extern "C" {
    pub fn ath_mci_setup(sc: *mut ath_softc) -> c_int;
}
extern "C" {
    pub fn ath_mci_cleanup(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_mci_intr(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_mci_update_rssi(sc: *mut ath_softc);
}

extern "C" {
    pub fn ath_mci_enable(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_mci_update_wlan_channels(sc: *mut ath_softc, allow_all: bool);
}

