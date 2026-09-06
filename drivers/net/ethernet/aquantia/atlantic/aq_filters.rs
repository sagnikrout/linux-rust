//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_filters.h
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
// Copyright (C) 2014-2017 aQuantia Corporation.
// File aq_filters.h: RX filters related functions.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_rx_filter_type {
    aq_rx_filter_ethertype,
    aq_rx_filter_vlan,
    aq_rx_filter_l3l4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rx_filter {
    pub aq_node: hlist_node,
    pub type: aq_rx_filter_type,
    pub aq_fsp: ethtool_rx_flow_spec,
}

extern "C" {
    pub fn aq_get_rxnfc_count_all_rules(aq_nic: *mut aq_nic_s) -> u16;
}
extern "C" {
    pub fn aq_add_rxnfc_rule(aq_nic: *mut aq_nic_s, cmd: *const ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn aq_del_rxnfc_rule(aq_nic: *mut aq_nic_s, cmd: *const ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn aq_get_rxnfc_rule(aq_nic: *mut aq_nic_s, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn aq_del_fvlan_by_vlan(aq_nic: *mut aq_nic_s, vlan_id: u16) -> c_int;
}
extern "C" {
    pub fn aq_clear_rxnfc_all_rules(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_reapply_rxnfc_all_rules(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_filters_vlans_update(aq_nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_filters_vlan_offload_off(aq_nic: *mut aq_nic_s) -> c_int;
}
