//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mae.h
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
// MCDI interface for the ef100 Match-Action Engine

extern "C" {
    pub fn efx_mae_allocate_mport(efx: *mut efx_nic, id: *mut u32, label: *mut u32) -> c_int;
}
extern "C" {
    pub fn efx_mae_free_mport(efx: *mut efx_nic, id: u32) -> c_int;
}
extern "C" {
    pub fn efx_mae_mport_wire(efx: *mut efx_nic, out: *mut u32);
}
extern "C" {
    pub fn efx_mae_mport_uplink(efx: *mut efx_nic, out: *mut u32);
}
extern "C" {
    pub fn efx_mae_mport_mport(efx: *mut efx_nic, mport_id: u32, out: *mut u32);
}
extern "C" {
    pub fn efx_mae_lookup_mport(efx: *mut efx_nic, selector: u32, id: *mut u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mae_mport_desc {
    pub mport_id: u32,
    pub flags: u32,
    pub /: *mut *mut u32 caller_flags; / enum mae_mport_desc_caller_flags,
    pub /: *mut *mut *mut u32 mport_type; / MAE_MPORT_DESC_MPORT_TYPE_,
    pub /: *mut *mut u32 port_idx; / for mport_type == NET_PORT,
    pub /: *mut *mut u32 alias_mport_id; / for mport_type == ALIAS,
    pub /: *mut *mut *mut u32 vnic_client_type; / MAE_MPORT_DESC_VNIC_CLIENT_TYPE_,
    pub interface_idx: u32,
    pub pf_idx: u16,
    pub vf_idx: u16,
}

extern "C" {
    pub fn efx_mae_enumerate_mports(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mae_put_mport(efx: *mut efx_nic, desc: *mut mae_mport_desc);
}
//
// struct efx_mae - MAE information
//
// @efx: The associated NIC
// @mports_ht: m-port descriptions from MC_CMD_MAE_MPORT_READ_JOURNAL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mae {
    pub efx: *mut efx_nic,
    pub mports_ht: rhashtable,
}

extern "C" {
    pub fn efx_mae_start_counters(efx: *mut efx_nic, rx_queue: *mut efx_rx_queue) -> c_int;
}
extern "C" {
    pub fn efx_mae_stop_counters(efx: *mut efx_nic, rx_queue: *mut efx_rx_queue) -> c_int;
}
extern "C" {
    pub fn efx_mae_counters_grant_credits(work: *mut work_struct);
}
extern "C" {
    pub fn efx_mae_get_tables(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mae_free_tables(efx: *mut efx_nic);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mae_caps {
    pub match_field_count: u32,
    pub encap_types: u32,
    pub action_prios: u32,
    pub action_rule_fields: [u8; MAE_NUM_FIELDS],
    pub outer_rule_fields: [u8; MAE_NUM_FIELDS],
}

extern "C" {
    pub fn efx_mae_get_caps(efx: *mut efx_nic, caps: *mut mae_caps) -> c_int;
}
extern "C" {
    pub fn efx_mae_allocate_counter(efx: *mut efx_nic, cnt: *mut efx_tc_counter) -> c_int;
}
extern "C" {
    pub fn efx_mae_free_counter(efx: *mut efx_nic, cnt: *mut efx_tc_counter) -> c_int;
}
extern "C" {
    pub fn efx_mae_alloc_action_set(efx: *mut efx_nic, act: *mut efx_tc_action_set) -> c_int;
}
extern "C" {
    pub fn efx_mae_free_action_set(efx: *mut efx_nic, fw_id: u32) -> c_int;
}
extern "C" {
    pub fn efx_mae_remove_lhs_rule(efx: *mut efx_nic, rule: *mut efx_tc_lhs_rule) -> c_int;
}
extern "C" {
    pub fn efx_mae_insert_ct(efx: *mut efx_nic, conn: *mut efx_tc_ct_entry) -> c_int;
}
extern "C" {
    pub fn efx_mae_remove_ct(efx: *mut efx_nic, conn: *mut efx_tc_ct_entry) -> c_int;
}
extern "C" {
    pub fn efx_mae_update_rule(efx: *mut efx_nic, acts_id: u32, id: u32) -> c_int;
}
extern "C" {
    pub fn efx_mae_delete_rule(efx: *mut efx_nic, id: u32) -> c_int;
}
extern "C" {
    pub fn efx_init_mae(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_fini_mae(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mae_remove_mport(desc: *mut c_void, arg: *mut c_void);
}
extern "C" {
    pub fn efx_mae_fw_lookup_mport(efx: *mut efx_nic, selector: u32, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn efx_mae_lookup_mport(efx: *mut efx_nic, vf: u32, id: *mut u32) -> c_int;
}
