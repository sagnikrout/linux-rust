//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_sriov.h
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
// Copyright (c) 2018, Intel Corporation.

// Static VF transaction/status register def
pub const VF_DEVICE_STATUS: c_uint = 0xAA;
pub const VF_TRANS_PENDING_M: c_uint = 0x20;
// wait defines for polling PF_PCI_CIAD register status
pub const ICE_PCI_CIAD_WAIT_COUNT: c_int = 100;
pub const ICE_PCI_CIAD_WAIT_DELAY_US: c_int = 1;
// VF resource constraints
pub const ICE_MIN_QS_PER_VF: c_int = 1;
pub const ICE_NONQ_VECS_VF: c_int = 1;
pub const ICE_NUM_VF_MSIX_MED: c_int = 17;
pub const ICE_NUM_VF_MSIX_SMALL: c_int = 5;
pub const ICE_NUM_VF_MSIX_MULTIQ_MIN: c_int = 3;

pub const ICE_MAX_VF_RESET_TRIES: c_int = 40;
pub const ICE_MAX_VF_RESET_SLEEP_MS: c_int = 20;

extern "C" {
    pub fn ice_process_vflr_event(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn __ice_set_vf_mac(pf: *mut ice_pf, vf_id: u16, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn ice_set_vf_mac(netdev: *mut net_device, vf_id: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_free_vfs(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_restore_all_vfs_msi_state(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_set_vf_trust(netdev: *mut net_device, vf_id: c_int, trusted: bool) -> c_int;
}
extern "C" {
    pub fn ice_set_vf_link_state(netdev: *mut net_device, vf_id: c_int, link_state: c_int) -> c_int;
}
extern "C" {
    pub fn ice_set_vf_spoofchk(netdev: *mut net_device, vf_id: c_int, ena: bool) -> c_int;
}
extern "C" {
    pub fn ice_calc_vf_reg_idx(vf: *mut ice_vf, q_vector: *mut ice_q_vector);
}
extern "C" {
    pub fn ice_print_vfs_mdd_events(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_print_vf_rx_mdd_event(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_print_vf_tx_mdd_event(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_sriov_get_vf_total_msix(pdev: *mut pci_dev) -> u32;
}
extern "C" {
    pub fn ice_sriov_set_msix_vec_count(vf_dev: *mut pci_dev, msix_vec_count: c_int) -> c_int;
}
extern "C" {
    pub fn ice_vf_vsi_dis_single_txq(vf: *mut ice_vf, vsi: *mut ice_vsi, q_id: u16) -> c_int;
}

