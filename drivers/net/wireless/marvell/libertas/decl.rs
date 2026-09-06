//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/decl.h
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
// This file contains declaration referring to
// functions defined in other source files
//

// Should be terminated by a NULL entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbs_fw_table {
    pub model: c_int,
    pub helper: *const c_char,
    pub fwname: *const c_char,
}

// ethtool.c
// tx.c
extern "C" {
    pub fn lbs_send_tx_feedback(priv: *mut lbs_private, try_count: u32);
}
// rx.c
extern "C" {
    pub fn lbs_process_rxed_packet(priv: *mut lbs_private, : *mut sk_buff) -> c_int;
}
// main.c
extern "C" {
    pub fn lbs_remove_card(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_start_card(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_stop_card(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_host_to_card_done(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_start_iface(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_stop_iface(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_set_iface_type(priv: *mut lbs_private, type: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn lbs_rtap_supported(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_set_mac_address(dev: *mut net_device, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn lbs_set_multicast_list(dev: *mut net_device);
}
extern "C" {
    pub fn lbs_update_mcast(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_suspend(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_resume(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_queue_event(priv: *mut lbs_private, event: u32);
}
extern "C" {
    pub fn lbs_notify_command_response(priv: *mut lbs_private, resp_idx: u8);
}
extern "C" {
    pub fn lbs_fw_index_to_data_rate(index: u8) -> u32;
}
extern "C" {
    pub fn lbs_wait_for_firmware_load(priv: *mut lbs_private);
}
