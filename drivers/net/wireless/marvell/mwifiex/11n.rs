//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/11n.h
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
// NXP Wireless LAN device driver: 802.11n
//
// Copyright 2011-2020 NXP
//

// tx_tbl);
extern "C" {
    pub fn mwifiex_11n_delete_all_tx_ba_stream_tbl(priv: *mut mwifiex_private);
}
// priv, int tid,
extern "C" {
    pub fn mwifiex_send_addba(priv: *mut mwifiex_private, tid: c_int, peer_mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mwifiex_11n_delete_ba_stream(priv: *mut mwifiex_private, del_ba: *mut u8);
}
extern "C" {
    pub fn mwifiex_del_tx_ba_stream_tbl_by_ra(priv: *mut mwifiex_private, ra: *mut u8);
}
extern "C" {
    pub fn mwifiex_get_sec_chan_offset(chan: c_int) -> u8;
}
// This function checks whether AMPDU is allowed or not for a particular TID.
extern "C" {
    pub fn mwifiex_is_station_ampdu_allowed(_arg: priv, _arg: ptr, _arg: tid) -> return;
}
extern "C" {
    pub fn mwifiex_is_station_ampdu_allowed(_arg: priv, _arg: ptr, _arg: tid) -> return;
}
//
// This function checks whether AMSDU is allowed or not for a particular TID.
//
// This function checks whether a space is available for new BA stream or not.
//
// This function finds the correct Tx BA stream to delete.
//
// Upon successfully locating, both the TID and the RA are returned.
//
// ptid = tx_tbl->tid;
//
// This function checks whether associated station is 11n enabled
//
