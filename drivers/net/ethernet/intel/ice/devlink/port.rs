//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/devlink/port.h
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
// Copyright (c) 2024, Intel Corporation.

//
// struct ice_dynamic_port - Track dynamically added devlink port instance
// @hw_addr: the HW address for this port
// @active: true if the port has been activated
// @attached: true if the prot is attached
// @devlink_port: the associated devlink port structure
// @pf: pointer to the PF private structure
// @vsi: the VSI associated with this port
// @repr_id: the representor ID
// @sfnum: the subfunction ID
// @sf_dev: pointer to the subfunction device
//
// An instance of a dynamically added devlink port. Each port flavour
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_dynamic_port {
    pub hw_addr: [u8; ETH_ALEN],
    pub 1: u8 active:,
    pub 1: u8 attached:,
    pub devlink_port: devlink_port,
    pub pf: *mut ice_pf,
    pub vsi: *mut ice_vsi,
    pub repr_id: c_ulong,
    pub sfnum: u32,
// Flavour-specific implementation data
    pub sf_dev: *mut ice_sf_dev,
}

extern "C" {
    pub fn ice_dealloc_all_dynamic_ports(pf: *mut ice_pf);
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
    pub fn ice_devlink_create_sf_port(dyn_port: *mut ice_dynamic_port) -> c_int;
}
extern "C" {
    pub fn ice_devlink_destroy_sf_port(dyn_port: *mut ice_dynamic_port);
}
extern "C" {
    pub fn ice_devlink_create_sf_dev_port(sf_dev: *mut ice_sf_dev) -> c_int;
}
extern "C" {
    pub fn ice_devlink_destroy_sf_dev_port(sf_dev: *mut ice_sf_dev);
}

