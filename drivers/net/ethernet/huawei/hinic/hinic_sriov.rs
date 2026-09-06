//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_sriov.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_sriov_state {
    HINIC_SRIOV_DISABLE,
    HINIC_SRIOV_ENABLE,
    HINIC_FUNC_REMOVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sriov_info {
    pub pdev: *mut pci_dev,
    pub hwdev: *mut hinic_hwdev,
    pub sriov_enabled: bool,
    pub num_vfs: c_uint,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_data_storage {
    pub vf_mac_addr: [u8; ETH_ALEN],
    pub registered: bool,
    pub pf_set_mac: bool,
    pub pf_vlan: u16,
    pub pf_qos: u8,
    pub max_rate: u32,
    pub min_rate: u32,
    pub link_forced: bool,
    pub /: *mut *mut bool link_up; / only valid if VF link is forced,
    pub spoofchk: bool,
    pub trust: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_register_vf {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_mac_update {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub vlan_id: u16,
    pub rsvd1: u16,
    pub old_mac: [u8; ETH_ALEN],
    pub rsvd2: u16,
    pub new_mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_vf_vlan_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub vlan_id: u16,
    pub qos: u8,
    pub rsvd1: [u8; 7],
}

extern "C" {
    pub fn hinic_ndo_set_vf_mac(netdev: *mut net_device, vf: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic_ndo_set_vf_trust(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn hinic_ndo_set_vf_spoofchk(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn hinic_ndo_set_vf_link_state(netdev: *mut net_device, vf_id: c_int, link: c_int) -> c_int;
}
extern "C" {
    pub fn hinic_pci_sriov_disable(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hinic_vf_func_init(hwdev: *mut hinic_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic_vf_func_free(hwdev: *mut hinic_hwdev);
}
extern "C" {
    pub fn hinic_pci_sriov_configure(dev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
