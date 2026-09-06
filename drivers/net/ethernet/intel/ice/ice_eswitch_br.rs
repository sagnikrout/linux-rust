//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_eswitch_br.h
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
// Copyright (C) 2023, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_fdb_data {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_flow {
    pub fwd_rule: *mut ice_rule_query_data,
    pub guard_rule: *mut ice_rule_query_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_fdb_entry {
    pub data: ice_esw_br_fdb_data,
    pub ht_node: rhash_head,
    pub list: list_head,
    pub flags: c_int,
    pub dev: *mut net_device,
    pub br_port: *mut ice_esw_br_port,
    pub flow: *mut ice_esw_br_flow,
    pub last_use: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_esw_br_port_type {
    ICE_ESWITCH_BR_UPLINK_PORT = 0,
    ICE_ESWITCH_BR_VF_REPR_PORT = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_port {
    pub bridge: *mut ice_esw_br,
    pub vsi: *mut ice_vsi,
    pub type: ice_esw_br_port_type,
    pub vsi_idx: u16,
    pub pvid: u16,
    pub repr_id: u32,
    pub vlans: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br {
    pub br_offloads: *mut ice_esw_br_offloads,
    pub ports: xarray,
    pub fdb_ht: rhashtable,
    pub fdb_list: list_head,
    pub ifindex: c_int,
    pub flags: u32,
    pub ageing_time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_offloads {
    pub pf: *mut ice_pf,
    pub bridge: *mut ice_esw_br,
    pub netdev_nb: notifier_block,
    pub switchdev_blk: notifier_block,
    pub switchdev_nb: notifier_block,
    pub wq: *mut workqueue_struct,
    pub update_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_fdb_work {
    pub work: work_struct,
    pub fdb_info: switchdev_notifier_fdb_info,
    pub dev: *mut net_device,
    pub event: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_esw_br_vlan {
    pub vid: u16,
    pub flags: u16,
}

// In trunk VLAN mode, for untagged traffic the bridge sends requests
// to offload VLAN 1 with pvid and untagged flags set. Since these
// flags are not supported, add a MAC filter instead.
//
extern "C" {
    pub fn ice_eswitch_br_fdb_flush(bridge: *mut ice_esw_br);
}
