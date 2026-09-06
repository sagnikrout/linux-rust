//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_vsi_vlan_ops.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi_vlan_ops {
    pub vlan): *mut *mut *mut int (add_vlan)(struct ice_vsi vsi, struct ice_vlan,
    pub vlan): *mut *mut *mut int (del_vlan)(struct ice_vsi vsi, struct ice_vlan,
    pub tpid): *const *const *const int (ena_stripping)(struct ice_vsi vsi, u16,
    pub vsi): *mut *mut int (dis_stripping)(struct ice_vsi,
    pub tpid): *const *const *const int (ena_insertion)(struct ice_vsi vsi, u16,
    pub vsi): *mut *mut int (dis_insertion)(struct ice_vsi,
    pub vsi): *mut *mut int (ena_rx_filtering)(struct ice_vsi,
    pub vsi): *mut *mut int (dis_rx_filtering)(struct ice_vsi,
    pub vsi): *mut *mut int (ena_tx_filtering)(struct ice_vsi,
    pub vsi): *mut *mut int (dis_tx_filtering)(struct ice_vsi,
    pub vlan): *mut *mut *mut int (set_port_vlan)(struct ice_vsi vsi, struct ice_vlan,
    pub vsi): *mut *mut int (clear_port_vlan)(struct ice_vsi,
}

extern "C" {
    pub fn ice_vsi_init_vlan_ops(vsi: *mut ice_vsi);
}
