//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc_pf_common.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2024 NXP

extern "C" {
    pub fn enetc_set_si_hw_addr(pf: *mut enetc_pf, si: c_int, mac_addr: *const u8);
}
extern "C" {
    pub fn enetc_pf_set_mac_addr(ndev: *mut net_device, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn enetc_setup_mac_addresses(np: *mut device_node, pf: *mut enetc_pf) -> c_int;
}
extern "C" {
    pub fn enetc_mdiobus_create(pf: *mut enetc_pf, node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn enetc_mdiobus_destroy(pf: *mut enetc_pf);
}
extern "C" {
    pub fn enetc_phylink_destroy(priv: *mut enetc_ndev_priv);
}
extern "C" {
    pub fn enetc_set_default_rss_key(pf: *mut enetc_pf);
}
extern "C" {
    pub fn enetc_vlan_rx_add_vid(ndev: *mut net_device, prot: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn enetc_vlan_rx_del_vid(ndev: *mut net_device, prot: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn enetc_init_sriov_resources(pf: *mut enetc_pf) -> c_int;
}
extern "C" {
    pub fn enetc_set_si_uc_promisc(si: *mut enetc_si, si_id: c_int, promisc: bool);
}
extern "C" {
    pub fn enetc_set_si_mc_promisc(si: *mut enetc_si, si_id: c_int, promisc: bool);
}
extern "C" {
    pub fn enetc_set_si_uc_hash_filter(si: *mut enetc_si, si_id: c_int, hash: u64);
}
extern "C" {
    pub fn enetc_set_si_mc_hash_filter(si: *mut enetc_si, si_id: c_int, hash: u64);
}
extern "C" {
    pub fn enetc_set_si_vlan_promisc(si: *mut enetc_si, si_id: c_int, promisc: bool);
}

extern "C" {
    pub fn enetc_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}

