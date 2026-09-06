//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_dcb_lib.h
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
    pub fn ice_dcb_rebuild(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_dcb_sw_dflt_cfg(pf: *mut ice_pf, ets_willing: bool, locked: bool) -> c_int;
}
extern "C" {
    pub fn ice_dcb_get_num_tc(dcbcfg: *mut ice_dcbx_cfg) -> u8;
}
extern "C" {
    pub fn ice_vsi_set_dcb_tc_cfg(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_is_pfc_causing_hung_q(pf: *mut ice_pf, txqueue: c_uint) -> bool;
}
extern "C" {
    pub fn ice_dcb_get_tc(vsi: *mut ice_vsi, queue_index: c_int) -> u8;
}
extern "C" {
    pub fn ice_dcb_bwchk(pf: *mut ice_pf, dcbcfg: *mut ice_dcbx_cfg) -> c_int;
}
extern "C" {
    pub fn ice_pf_dcb_recfg(pf: *mut ice_pf, locked: bool);
}
extern "C" {
    pub fn ice_vsi_cfg_dcb_rings(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_init_pf_dcb(pf: *mut ice_pf, locked: bool) -> c_int;
}
extern "C" {
    pub fn ice_update_dcb_stats(pf: *mut ice_pf);
}
//
// ice_find_q_in_range
// @low: start of queue range for a TC i.e. offset of TC
// @high: start of queue for next TC
// @tx_q: hung_queue/tx_queue
//
// finds if queue 'tx_q' falls between the two offsets of any given TC
//

