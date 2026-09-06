//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192cu/mac.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const LLT_LAST_ENTRY_OF_TX_PKT_BUFFER: c_int = 255;
pub const DRIVER_EARLY_INT_TIME: c_uint = 0x05;
pub const BCN_DMA_ATIME_INT_TIME: c_uint = 0x02;
extern "C" {
    pub fn rtl92c_read_chip_version(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_llt_write(hw: *mut ieee80211_hw, address: u32, data: u32) -> bool;
}
extern "C" {
    pub fn rtl92c_init_llt_table(hw: *mut ieee80211_hw, boundary: u32) -> bool;
}
extern "C" {
    pub fn rtl92c_enable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_disable_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_set_qos(hw: *mut ieee80211_hw, aci: c_int);
}
// ---------------------------------------------------------------
// Hardware init functions
// ---------------------------------------------------------------
extern "C" {
    pub fn rtl92c_init_interrupt(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_driver_info_size(hw: *mut ieee80211_hw, size: u8);
}
extern "C" {
    pub fn rtl92c_set_network_type(hw: *mut ieee80211_hw, type: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn rtl92c_init_network_type(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_adaptive_ctrl(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_rate_fallback(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_edca(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_ampdu_aggregation(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_beacon_max_error(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_rdg_setting(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_init_retry_function(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_disable_fast_edca(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl92c_set_min_space(hw: *mut ieee80211_hw, is2T: bool);
}
extern "C" {
    pub fn rtl92c_get_txdma_status(hw: *mut ieee80211_hw) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_fwinfo_92c {
    pub gain_trsw: [u8; 4],
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
    pub cfotail: [u8; 4],
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 4],
    pub pdsnr: [u8; 2],
    pub csi_current: [u8; 2],
    pub csi_target: [u8; 2],
    pub sigevm: u8,
    pub max_ex_pwr: u8,
    pub ex_intf_flag:1: u8,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub reserve:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc_92c {
    pub length:14: u32,
    pub crc32:1: u32,
    pub icverror:1: u32,
    pub drv_infosize:4: u32,
    pub security:3: u32,
    pub qos:1: u32,
    pub shift:2: u32,
    pub phystatus:1: u32,
    pub swdec:1: u32,
    pub lastseg:1: u32,
    pub firstseg:1: u32,
    pub eor:1: u32,
    pub own:1: u32,
    pub /: *mut *mut u32 macid:5; / word 1,
    pub tid:4: u32,
    pub hwrsvd:5: u32,
    pub paggr:1: u32,
    pub faggr:1: u32,
    pub a1_fit:4: u32,
    pub a2_fit:4: u32,
    pub pam:1: u32,
    pub pwr:1: u32,
    pub moredata:1: u32,
    pub morefrag:1: u32,
    pub type:2: u32,
    pub mc:1: u32,
    pub bc:1: u32,
    pub /: *mut *mut u32 seq:12; / word 2,
    pub frag:4: u32,
    pub nextpktlen:14: u32,
    pub nextind:1: u32,
    pub rsvd:1: u32,
    pub /: *mut *mut u32 rxmcs:6; / word 3,
    pub rxht:1: u32,
    pub amsdu:1: u32,
    pub splcp:1: u32,
    pub bandwidth:1: u32,
    pub htc:1: u32,
    pub tcpchk_rpt:1: u32,
    pub ipcchk_rpt:1: u32,
    pub tcpchk_valid:1: u32,
    pub hwpcerr:1: u32,
    pub hwpcind:1: u32,
    pub iv0:16: u32,
    pub /: *mut *mut u32 iv1; / word 4,
    pub /: *mut *mut u32 tsfl; / word 5,
    pub /: *mut *mut u32 bufferaddress; / word 6,
    pub /: *mut *mut u32 bufferaddress64; / word 7,
    pub __packed: },
    pub p_drvinfo): *mut rx_fwinfo_92c,
// ---------------------------------------------------------------
// Card disable functions
// ---------------------------------------------------------------
