//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/devlink/devlink.h
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
    pub fn ice_devlink_register(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_unregister(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_register_params(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_devlink_unregister_params(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_create_pf_port(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_devlink_destroy_pf_port(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_create_vf_port(vf: *mut ice_vf) -> c_int;
}
extern "C" {
    pub fn ice_devlink_destroy_vf_port(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_devlink_init_regions(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_destroy_regions(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_rate_init_tx_topology(devlink: *mut devlink, vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_tear_down_devlink_rate_tree(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_devlink_rate_clear_tx_topology(vsi: *mut ice_vsi);
}
