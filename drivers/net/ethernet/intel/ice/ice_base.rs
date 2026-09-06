//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_base.h
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
// Copyright (c) 2019, Intel Corporation.

extern "C" {
    pub fn ice_vsi_cfg_single_rxq(vsi: *mut ice_vsi, q_idx: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_rxqs(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn __ice_vsi_get_qs(qs_cfg: *mut ice_qs_cfg) -> c_int;
}
extern "C" {
    pub fn ice_vsi_wait_one_rx_ring(vsi: *mut ice_vsi, ena: bool, rxq_idx: u16) -> c_int;
}
extern "C" {
    pub fn ice_vsi_alloc_q_vectors(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_map_rings_to_vectors(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_free_q_vectors(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_cfg_lan_txqs(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_xdp_txqs(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_cfg_itr(hw: *mut ice_hw, q_vector: *mut ice_q_vector);
}
extern "C" {
    pub fn ice_trigger_sw_intr(hw: *mut ice_hw, q_vector: *const ice_q_vector);
}
extern "C" {
    pub fn ice_qp_ena(vsi: *mut ice_vsi, q_idx: u16) -> c_int;
}
extern "C" {
    pub fn ice_qp_dis(vsi: *mut ice_vsi, q_idx: u16) -> c_int;
}
extern "C" {
    pub fn ice_calc_ts_ring_count(tx_ring: *mut ice_tx_ring) -> u16;
}
