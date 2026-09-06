//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/rx_common.h
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
// Preferred number of descriptors to fill at once

// Each packet can consume up to ceil(max_frame_len / buffer_size) buffers

// Number of RX buffers to recycle pages for.  When creating the RX page recycle
// ring, this number is divided by the number of buffers per page to calculate
// the number of pages to store in the RX page recycle ring.
//
pub const EFX_RECYCLE_RING_SIZE_10G: c_int = 256;

extern "C" {
    pub fn __le32_to_cpup(efx->rx_packet_hash_offset): *const *const (__le32 )(eh +) -> return;
}

extern "C" {
    pub fn efx_rx_slow_fill(t: *mut timer_list);
}
extern "C" {
    pub fn efx_probe_rx_queue(rx_queue: *mut efx_rx_queue) -> c_int;
}
extern "C" {
    pub fn efx_init_rx_queue(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_fini_rx_queue(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_remove_rx_queue(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_destroy_rx_queue(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_unmap_rx_buffer(efx: *mut efx_nic, rx_buf: *mut efx_rx_buffer);
}
extern "C" {
    pub fn efx_schedule_slow_fill(rx_queue: *mut efx_rx_queue);
}
extern "C" {
    pub fn efx_rx_config_page_split(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fast_push_rx_descriptors(rx_queue: *mut efx_rx_queue, atomic: bool);
}
extern "C" {
    pub fn efx_set_default_rx_indir_table(efx: *mut efx_nic, indir: *mut u32);
}
extern "C" {
    pub fn efx_filter_is_mc_recipient(spec: *const efx_filter_spec) -> bool;
}
extern "C" {
    pub fn efx_filter_spec_hash(spec: *const efx_filter_spec) -> u32;
}

extern "C" {
    pub fn efx_rps_hash_del(efx: *mut efx_nic, spec: *const efx_filter_spec);
}
extern "C" {
    pub fn __efx_filter_rfs_expire(channel: *mut efx_channel, quota: c_uint) -> bool;
}

extern "C" {
    pub fn efx_probe_filters(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_remove_filters(efx: *mut efx_nic);
}
