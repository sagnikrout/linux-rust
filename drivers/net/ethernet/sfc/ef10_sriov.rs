//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef10_sriov.h
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
// Copyright 2015 Solarflare Communications Inc.
//

//
// struct ef10_vf - PF's store of VF data
// @efx: efx_nic struct for the current VF
// @pci_dev: the pci_dev struct for the VF, retained while the VF is assigned
// @vport_id: vport ID for the VF
// @vport_assigned: record whether the vport is currently assigned to the VF
// @mac: MAC address for the VF, zero when address is removed from the vport
// @vlan: Default VLAN for the VF or #EFX_EF10_NO_VLAN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ef10_vf {
    pub efx: *mut efx_nic,
    pub pci_dev: *mut pci_dev,
    pub vport_id: c_uint,
    pub vport_assigned: c_uint,
    pub mac: [u8; ETH_ALEN],
    pub vlan: u16,
pub const EFX_EF10_NO_VLAN: c_int = 0;
}

extern "C" {
    pub fn efx_ef10_sriov_configure(efx: *mut efx_nic, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn efx_ef10_sriov_init(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef10_sriov_fini(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ef10_sriov_set_vf_mac(efx: *mut efx_nic, vf: c_int, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vswitching_probe_pf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vswitching_probe_vf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vswitching_restore_pf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vswitching_restore_vf(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vswitching_remove_pf(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ef10_vswitching_remove_vf(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ef10_vadaptor_alloc(efx: *mut efx_nic, port_id: c_uint) -> c_int;
}
extern "C" {
    pub fn efx_ef10_vadaptor_free(efx: *mut efx_nic, port_id: c_uint) -> c_int;
}
