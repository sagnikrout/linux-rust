//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_port.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.

//
// enum nfp_port_type - type of port NFP can switch traffic to
// @NFP_PORT_INVALID:	port is invalid, %NFP_PORT_PHYS_PORT transitions to this
// state when port disappears because of FW fault or config
// change
// @NFP_PORT_PHYS_PORT:	external NIC port
// @NFP_PORT_PF_PORT:	logical port of PCI PF
// @NFP_PORT_VF_PORT:	logical port of PCI VF
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_port_type {
    NFP_PORT_INVALID,
    NFP_PORT_PHYS_PORT,
    NFP_PORT_PF_PORT,
    NFP_PORT_VF_PORT,
}

//
// enum nfp_port_flags - port flags (can be type-specific)
// @NFP_PORT_CHANGED:	port state has changed since last eth table refresh;
// for NFP_PORT_PHYS_PORT, never set otherwise; must hold
// rtnl_lock to clear
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_port_flags {
    NFP_PORT_CHANGED = 0,
}

//
// struct nfp_port - structure representing NFP port
// @netdev:	backpointer to associated netdev
// @type:	what port type does the entity represent
// @flags:	port flags
// @tc_offload_cnt:	number of active TC offloads, how offloads are counted
// is not defined, use as a boolean
// @app:	backpointer to the app structure
// @link_cb:	callback when link status changed
// @dl_port:	devlink port structure
// @eth_id:	for %NFP_PORT_PHYS_PORT port ID in NFP enumeration scheme
// @eth_forced:	for %NFP_PORT_PHYS_PORT port is forced UP or DOWN, don't change
// @eth_port:	for %NFP_PORT_PHYS_PORT translated ETH Table port entry
// @eth_stats:	for %NFP_PORT_PHYS_PORT MAC stats if available
// @speed_bitmap:	for %NFP_PORT_PHYS_PORT supported speed bitmap
// @pf_id:	for %NFP_PORT_PF_PORT, %NFP_PORT_VF_PORT ID of the PCI PF (0-3)
// @vf_id:	for %NFP_PORT_VF_PORT ID of the PCI VF within @pf_id
// @pf_split:	for %NFP_PORT_PF_PORT %true if PCI PF has more than one vNIC
// @pf_split_id:for %NFP_PORT_PF_PORT ID of PCI PF vNIC (valid if @pf_split)
// @vnic:	for %NFP_PORT_PF_PORT, %NFP_PORT_VF_PORT vNIC ctrl memory
// @port_list:	entry on pf's list of ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_port {
    pub netdev: *mut net_device,
    pub type: nfp_port_type,
    pub flags: c_ulong,
    pub tc_offload_cnt: c_ulong,
    pub app: *mut nfp_app,
    pub port): *mut *mut void (link_cb)(struct nfp_port,
    pub dl_port: devlink_port,
// NFP_PORT_PHYS_PORT
    pub eth_id: c_uint,
    pub eth_forced: bool,
    pub eth_port: *mut nfp_eth_table_port,
    pub eth_stats: *mut u8 __iomem,
    pub NFP_SUP_SPEED_NUMBER): DECLARE_BITMAP(speed_bitmap,,
}

// NFP_PORT_PF_PORT, NFP_PORT_VF_PORT
extern "C" {
    pub fn nfp_port_configure(netdev: *mut net_device, configed: bool) -> c_int;
}
extern "C" {
    pub fn nfp_port_free(port: *mut nfp_port);
}
extern "C" {
    pub fn nfp_net_refresh_eth_port(port: *mut nfp_port) -> c_int;
}
extern "C" {
    pub fn nfp_net_refresh_port_table(port: *mut nfp_port);
}
extern "C" {
    pub fn nfp_net_refresh_port_table_sync(pf: *mut nfp_pf) -> c_int;
}
extern "C" {
    pub fn nfp_devlink_port_register(app: *mut nfp_app, port: *mut nfp_port) -> c_int;
}
extern "C" {
    pub fn nfp_devlink_port_unregister(port: *mut nfp_port);
}
// Mac stats (0x0000 - 0x0200)
// all counters are 64bit.
//
pub const NFP_MAC_STATS_BASE: c_uint = 0x0000;
pub const NFP_MAC_STATS_SIZE: c_uint = 0x0200;

// unused 0x008

// unused 0x120
// unused 0x128
// unused 0x130

// unused 0x148

