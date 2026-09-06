//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy_shim.h
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
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// phy_shim.h: stuff defined in phy_shim.c and included only by the phy
//

// French radar pulse widths
pub const FRA_T1_20MHZ: c_int = 52770;
pub const FRA_T2_20MHZ: c_int = 61538;
pub const FRA_T3_20MHZ: c_int = 66002;
pub const FRA_T1_40MHZ: c_int = 105541;
pub const FRA_T2_40MHZ: c_int = 123077;
pub const FRA_T3_40MHZ: c_int = 132004;
pub const FRA_ERR_20MHZ: c_int = 60;
pub const FRA_ERR_40MHZ: c_int = 120;

// Rx Antenna diversity control values

// values for n_preamble_type
pub const BRCMS_N_PREAMBLE_MIXEDMODE: c_int = 0;
pub const BRCMS_N_PREAMBLE_GF: c_int = 1;
pub const BRCMS_N_PREAMBLE_GF_BRCM: c_int = 2;
pub const WL_TX_POWER_RATES_LEGACY: c_int = 45;
pub const WL_TX_POWER_MCS20_FIRST: c_int = 12;
pub const WL_TX_POWER_MCS20_NUM: c_int = 16;
pub const WL_TX_POWER_MCS40_FIRST: c_int = 28;
pub const WL_TX_POWER_MCS40_NUM: c_int = 17;
pub const WL_TX_POWER_RATES: c_int = 101;
pub const WL_TX_POWER_CCK_FIRST: c_int = 0;
pub const WL_TX_POWER_CCK_NUM: c_int = 4;
// Index for first 20MHz OFDM SISO rate
pub const WL_TX_POWER_OFDM_FIRST: c_int = 4;
// Index for first 20MHz OFDM CDD rate
pub const WL_TX_POWER_OFDM20_CDD_FIRST: c_int = 12;
// Index for first 40MHz OFDM SISO rate
pub const WL_TX_POWER_OFDM40_SISO_FIRST: c_int = 52;
// Index for first 40MHz OFDM CDD rate
pub const WL_TX_POWER_OFDM40_CDD_FIRST: c_int = 60;
pub const WL_TX_POWER_OFDM_NUM: c_int = 8;
// Index for first 20MHz MCS SISO rate
pub const WL_TX_POWER_MCS20_SISO_FIRST: c_int = 20;
// Index for first 20MHz MCS CDD rate
pub const WL_TX_POWER_MCS20_CDD_FIRST: c_int = 28;
// Index for first 20MHz MCS STBC rate
pub const WL_TX_POWER_MCS20_STBC_FIRST: c_int = 36;
// Index for first 20MHz MCS SDM rate
pub const WL_TX_POWER_MCS20_SDM_FIRST: c_int = 44;
// Index for first 40MHz MCS SISO rate
pub const WL_TX_POWER_MCS40_SISO_FIRST: c_int = 68;
// Index for first 40MHz MCS CDD rate
pub const WL_TX_POWER_MCS40_CDD_FIRST: c_int = 76;
// Index for first 40MHz MCS STBC rate
pub const WL_TX_POWER_MCS40_STBC_FIRST: c_int = 84;
// Index for first 40MHz MCS SDM rate
pub const WL_TX_POWER_MCS40_SDM_FIRST: c_int = 92;
pub const WL_TX_POWER_MCS_1_STREAM_NUM: c_int = 8;
pub const WL_TX_POWER_MCS_2_STREAM_NUM: c_int = 8;
// Index for 40MHz rate MCS 32
pub const WL_TX_POWER_MCS_32: c_int = 100;
pub const WL_TX_POWER_MCS_32_NUM: c_int = 1;
// sslpnphy specifics
// Index for first 20MHz MCS SISO rate
pub const WL_TX_POWER_MCS20_SISO_FIRST_SSN: c_int = 12;
// struct tx_power::flags bits
pub const WL_TX_POWER_F_ENABLED: c_int = 1;
pub const WL_TX_POWER_F_HW: c_int = 2;
pub const WL_TX_POWER_F_MIMO: c_int = 4;
pub const WL_TX_POWER_F_SISO: c_int = 8;
// values to force tx/rx chain
pub const BRCMS_N_TXRX_CHAIN0: c_int = 0;
pub const BRCMS_N_TXRX_CHAIN1: c_int = 1;
extern "C" {
    pub fn wlc_phy_shim_detach(physhim: *mut phy_shim_info);
}
// PHY to WL utility functions
extern "C" {
    pub fn wlapi_free_timer(t: *mut wlapi_timer);
}
extern "C" {
    pub fn wlapi_add_timer(t: *mut wlapi_timer, ms: c_uint, periodic: c_int);
}
extern "C" {
    pub fn wlapi_del_timer(t: *mut wlapi_timer) -> bool;
}
extern "C" {
    pub fn wlapi_intrson(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_intrsoff(physhim: *mut phy_shim_info) -> u32;
}
extern "C" {
    pub fn wlapi_intrsrestore(physhim: *mut phy_shim_info, macintmask: u32);
}
extern "C" {
    pub fn wlapi_bmac_write_shm(physhim: *mut phy_shim_info, offset: c_uint, v: u16);
}
extern "C" {
    pub fn wlapi_bmac_read_shm(physhim: *mut phy_shim_info, offset: c_uint) -> u16;
}
extern "C" {
    pub fn wlapi_bmac_corereset(physhim: *mut phy_shim_info, flags: u32);
}
extern "C" {
    pub fn wlapi_suspend_mac_and_wait(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_switch_macfreq(physhim: *mut phy_shim_info, spurmode: u8);
}
extern "C" {
    pub fn wlapi_enable_mac(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_bmac_mctrl(physhim: *mut phy_shim_info, mask: u32, val: u32);
}
extern "C" {
    pub fn wlapi_bmac_phy_reset(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_bmac_bw_set(physhim: *mut phy_shim_info, bw: u16);
}
extern "C" {
    pub fn wlapi_bmac_phyclk_fgc(physhim: *mut phy_shim_info, clk: bool);
}
extern "C" {
    pub fn wlapi_bmac_macphyclk_set(physhim: *mut phy_shim_info, clk: bool);
}
extern "C" {
    pub fn wlapi_bmac_core_phypll_ctl(physhim: *mut phy_shim_info, on: bool);
}
extern "C" {
    pub fn wlapi_bmac_core_phypll_reset(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_bmac_ucode_wake_override_phyreg_set(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_bmac_ucode_wake_override_phyreg_clear(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_bmac_rate_shm_offset(physhim: *mut phy_shim_info, rate: u8) -> u16;
}
extern "C" {
    pub fn wlapi_ucode_sample_init(physhim: *mut phy_shim_info);
}
extern "C" {
    pub fn wlapi_high_update_phy_mode(physhim: *mut phy_shim_info, phy_mode: u32);
}
extern "C" {
    pub fn wlapi_bmac_get_txant(physhim: *mut phy_shim_info) -> u16;
}
