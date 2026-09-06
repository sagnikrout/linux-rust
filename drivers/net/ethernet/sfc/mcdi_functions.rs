//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi_functions.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2018 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
extern "C" {
    pub fn efx_mcdi_free_vis(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_ev_probe(channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_ev_init(channel: *mut efx_channel, v1_cut_thru: bool, v2: bool) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_ev_remove(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_mcdi_ev_fini(channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_mcdi_tx_init(tx_queue: *mut efx_tx_queue) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_tx_remove(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_mcdi_tx_fini(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn efx_mcdi_rx_probe(rx_queue: *mut efx_rx_queue) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_rx_init(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_mcdi_rx_remove(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_mcdi_rx_fini(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_fini_dmaq(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_window_mode_to_stride(efx: *mut efx_nic, vi_window_mode: u8) -> c_int;
}
extern "C" {
    pub fn efx_get_pf_index(efx: *mut efx_nic, pf_index: *mut c_uint) -> c_int;
}
