//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef100_rep.h
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
// Copyright 2019 Solarflare Communications Inc.
// Copyright 2020-2022 Xilinx Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
// Handling for ef100 representor netdevs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rep_sw_stats {
    pub tx_packets: atomic64_t rx_packets,,
    pub tx_bytes: atomic64_t rx_bytes,,
    pub tx_errors: atomic64_t rx_dropped,,
}

//
// struct efx_rep - Private data for an Efx representor
//
// @parent: the efx PF which manages this representor
// @net_dev: representor netdevice
// @msg_enable: log message enable flags
// @mport: m-port ID of corresponding VF
// @idx: VF index
// @write_index: number of packets enqueued to @rx_list
// @read_index: number of packets consumed from @rx_list
// @rx_pring_size: max length of RX list
// @dflt: default-rule for MAE switching
// @list: entry on efx->vf_reps
// @rx_list: list of SKBs queued for receive in NAPI poll
// @rx_lock: protects @rx_list
// @napi: NAPI control structure
// @stats: software traffic counters for netdev stats
// @dl_port: devlink port associated to this netdev representor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_rep {
    pub parent: *mut efx_nic,
    pub net_dev: *mut net_device,
    pub msg_enable: u32,
    pub mport: u32,
    pub idx: c_uint,
    pub read_index: unsigned int write_index,,
    pub rx_pring_size: c_uint,
    pub dflt: efx_tc_flow_rule,
    pub list: list_head,
    pub rx_list: list_head,
    pub rx_lock: spinlock_t,
    pub napi: napi_struct,
    pub stats: efx_rep_sw_stats,
    pub dl_port: *mut devlink_port,
}

extern "C" {
    pub fn efx_ef100_vfrep_create(efx: *mut efx_nic, i: c_uint) -> c_int;
}
extern "C" {
    pub fn efx_ef100_vfrep_destroy(efx: *mut efx_nic, efv: *mut efx_rep);
}
extern "C" {
    pub fn efx_ef100_fini_vfreps(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ef100_rep_rx_packet(efv: *mut efx_rep, rx_buf: *mut efx_rx_buffer);
}
// Returns the representor corresponding to a VF m-port, or NULL
// @mport is an m-port label, *not* an m-port ID!
// Caller must hold rcu_read_lock().
//
extern "C" {
    pub fn efx_ef100_init_reps(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ef100_fini_reps(efx: *mut efx_nic);
}
extern "C" {
    pub fn ef100_mport_is_vf(mport_desc: *mut mae_mport_desc) -> bool;
}
