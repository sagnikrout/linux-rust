//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef100_nic.h
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
// Copyright 2019-2020 Xilinx Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

extern "C" {
    pub fn ef100_probe_netdev_pf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn ef100_probe_vf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn ef100_remove(efx: *mut efx_nic);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef100_nic_data {
    pub efx: *mut efx_nic,
    pub mcdi_buf: efx_buffer,
    pub datapath_caps: u32,
    pub datapath_caps2: u32,
    pub datapath_caps3: u32,
    pub pf_index: c_uint,
    pub warm_boot_count: u16,
    pub port_id: [u8; ETH_ALEN],
    pub EFX_MAX_CHANNELS): DECLARE_BITMAP(evq_phases,,
    pub stats: [u64; EF100_STAT_COUNT],
    pub base_mport: u32,
    pub /: *mut *mut bool have_mport; / base_mport was populated successfully,
    pub own_mport: u32,
    pub /: *mut *mut u32 local_mae_intf; / interface_idx that corresponds to us, in mport enumerate,
    pub /: *mut *mut bool have_own_mport; / own_mport was populated successfully,
    pub /: *mut *mut bool have_local_intf; / local_mae_intf was populated successfully,
    pub /: *mut *mut bool grp_mae; / MAE Privilege,
    pub tso_max_hdr_len: u16,
    pub tso_max_payload_num_segs: u16,
    pub tso_max_frames: u16,
    pub tso_max_payload_len: c_uint,
}

extern "C" {
    pub fn efx_ef100_init_datapath_caps(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn ef100_phy_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn ef100_filter_table_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef100_lookup_client_id(efx: *mut efx_nic, pciefn: efx_qword_t, id: *mut u32) -> c_int;
}
