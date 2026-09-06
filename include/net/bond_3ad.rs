//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bond_3ad.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright(c) 1999 - 2004 Intel Corporation. All rights reserved.
//

// General definitions

pub const AD_LACP_SLOW: c_int = 0;
pub const AD_LACP_FAST: c_int = 1;
// rx machine states(43.4.11 in the 802.3ad standard)
// periodic machine states(43.4.12 in the 802.3ad standard)
// mux machine states(43.4.13 in the 802.3ad standard)
// tx machine states(43.4.15 in the 802.3ad standard)
// churn machine states(43.4.17 in the 802.3ad standard)
// rx indication types
// rx marker indication types
// timers types(43.4.9 in the 802.3ad standard)

// Link Aggregation Control Protocol(LACP) data unit structure(43.4.2.2 in the 802.3ad standard)
// Marker Protocol Data Unit(PDU) structure(43.5.3.2 in the 802.3ad standard)
// = 0x02  (marker response information)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_3ad_stats {
    pub lacpdu_rx: core::sync::atomic::AtomicI64,
    pub lacpdu_tx: core::sync::atomic::AtomicI64,
    pub lacpdu_unknown_rx: core::sync::atomic::AtomicI64,
    pub lacpdu_illegal_rx: core::sync::atomic::AtomicI64,
    pub marker_rx: core::sync::atomic::AtomicI64,
    pub marker_tx: core::sync::atomic::AtomicI64,
    pub marker_resp_rx: core::sync::atomic::AtomicI64,
    pub marker_resp_tx: core::sync::atomic::AtomicI64,
    pub marker_unknown_rx: core::sync::atomic::AtomicI64,
}

// aggregator structure(43.4.5 in the 802.3ad standard)
// ****** PRIVATE PARAMETERS ******
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_params {
    pub system: mac_addr,
    pub system_priority: u16,
    pub key: u16,
    pub port_number: u16,
    pub port_priority: u16,
    pub port_state: u16,
}

// port structure(43.4.6 in the 802.3ad standard)
// ****** PRIVATE PARAMETERS ******
// (always on - enter to transmit
// state 3 time per second)
//
// system structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_system {
    pub sys_priority: u16,
    pub sys_mac_addr: mac_addr,
}

// ========== AD Exported structures to the main bonding code ==========

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_bond_info {
    pub /: *mut *mut ad_system system; / 802.3ad system structure,
    pub stats: bond_3ad_stats,
    pub /: *mut *mut atomic_t agg_select_timer; / Timer to select aggregator after all adapter's hand shakes,
    pub aggregator_identifier: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_slave_info {
    pub /: *mut *mut aggregator aggregator; / 802.3ad aggregator structure,
    pub /: *mut *mut port port; / 802.3ad port structure,
    pub stats: bond_3ad_stats,
    pub id: u16,
    pub port_priority: u16,
}

// ========== AD Exported functions to the main bonding code ==========
extern "C" {
    pub fn bond_3ad_initialize(bond: *mut bonding);
}
extern "C" {
    pub fn bond_3ad_bind_slave(slave: *mut slave);
}
extern "C" {
    pub fn bond_3ad_unbind_slave(slave: *mut slave);
}
extern "C" {
    pub fn bond_3ad_state_machine_handler(: *mut work_struct);
}
extern "C" {
    pub fn bond_3ad_initiate_agg_selection(bond: *mut bonding, timeout: c_int);
}
extern "C" {
    pub fn bond_3ad_adapter_speed_duplex_changed(slave: *mut slave);
}
extern "C" {
    pub fn bond_3ad_handle_link_change(slave: *mut slave, link: c_char);
}
extern "C" {
    pub fn bond_3ad_get_active_agg_info(bond: *const bonding, ad_info: *mut ad_info) -> c_int;
}
extern "C" {
    pub fn bond_3ad_set_carrier(bond: *mut bonding) -> c_int;
}
extern "C" {
    pub fn bond_3ad_update_lacp_rate(bond: *mut bonding);
}
extern "C" {
    pub fn bond_3ad_update_lacp_active(bond: *mut bonding);
}
extern "C" {
    pub fn bond_3ad_update_ad_actor_settings(bond: *mut bonding);
}
extern "C" {
    pub fn bond_3ad_stats_fill(skb: *mut sk_buff, stats: *mut bond_3ad_stats) -> c_int;
}
extern "C" {
    pub fn bond_3ad_stats_size() -> usize;
}
