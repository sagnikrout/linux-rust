//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_eswitch.h
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
// Copyright (C) 2019-2021, Intel Corporation.

extern "C" {
    pub fn ice_eswitch_detach_vf(pf: *mut ice_pf, vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_eswitch_detach_sf(pf: *mut ice_pf, sf: *mut ice_dynamic_port);
}
extern "C" {
    pub fn ice_eswitch_attach_vf(pf: *mut ice_pf, vf: *mut ice_vf) -> c_int;
}
extern "C" {
    pub fn ice_eswitch_attach_sf(pf: *mut ice_pf, sf: *mut ice_dynamic_port) -> c_int;
}
extern "C" {
    pub fn ice_eswitch_mode_get(devlink: *mut devlink, mode: *mut u16) -> c_int;
}
extern "C" {
    pub fn ice_is_eswitch_mode_switchdev(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_eswitch_update_repr(repr_id: *mut c_ulong, vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_eswitch_stop_all_tx_queues(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_eswitch_cfg_vsi(vsi: *mut ice_vsi, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn ice_eswitch_decfg_vsi(vsi: *mut ice_vsi, mac: *const u8);
}

