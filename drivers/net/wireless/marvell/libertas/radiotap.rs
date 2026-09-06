//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/radiotap.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_radiotap_hdr {
    pub hdr: ieee80211_radiotap_header_fixed,
    pub rate: u8,
    pub txpower: u8,
    pub rts_retries: u8,
    pub data_retries: u8,
    pub __packed: },

pub const IEEE80211_FC_VERSION_MASK: c_uint = 0x0003;
pub const IEEE80211_FC_TYPE_MASK: c_uint = 0x000c;
pub const IEEE80211_FC_TYPE_MGT: c_uint = 0x0000;
pub const IEEE80211_FC_TYPE_CTL: c_uint = 0x0004;
pub const IEEE80211_FC_TYPE_DATA: c_uint = 0x0008;
pub const IEEE80211_FC_SUBTYPE_MASK: c_uint = 0x00f0;
pub const IEEE80211_FC_TOFROMDS_MASK: c_uint = 0x0300;
pub const IEEE80211_FC_TODS_MASK: c_uint = 0x0100;
pub const IEEE80211_FC_FROMDS_MASK: c_uint = 0x0200;
pub const IEEE80211_FC_NODS: c_uint = 0x0000;
pub const IEEE80211_FC_TODS: c_uint = 0x0100;
pub const IEEE80211_FC_FROMDS: c_uint = 0x0200;
pub const IEEE80211_FC_DSTODS: c_uint = 0x0300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_radiotap_hdr {
    pub hdr: ieee80211_radiotap_header_fixed,
    pub flags: u8,
    pub rate: u8,
    pub antsignal: u8,
    pub __packed: },
