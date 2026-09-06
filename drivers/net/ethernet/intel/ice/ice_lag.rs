//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_lag.h
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
// Copyright (C) 2018-2021, Intel Corporation.

// LAG roles for netdev
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_lag_role {
    ICE_LAG_NONE,
    ICE_LAG_PRIMARY,
    ICE_LAG_BACKUP,
    ICE_LAG_UNSET
}

pub const ICE_LAG_INVALID_PORT: c_uint = 0xFF;
pub const ICE_LAGP_IDX: c_int = 0;
pub const ICE_LAGS_IDX: c_int = 1;
pub const ICE_LAGP_M: c_uint = 0x1;
pub const ICE_LAGS_M: c_uint = 0x2;
pub const ICE_LAG_RESET_RETRIES: c_int = 5;
pub const ICE_SW_DEFAULT_PROFILE: c_int = 0;
pub const ICE_FV_PROT_MDID: c_int = 255;
pub const ICE_LP_EXT_BUF_OFFSET: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_lag_netdev_list {
    pub node: list_head,
    pub netdev: *mut net_device,
}

// LAG info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_lag {
    pub /: *mut *mut *mut ice_pf pf; / backlink to PF struct,
    pub /: *mut *mut *mut net_device netdev; / this PF's netdev,
    pub /: *mut *mut *mut net_device upper_netdev; / upper bonding netdev,
    pub netdev_head: *mut list_head,
    pub notif_block: notifier_block,
    pub bond_mode: i32,
    pub /: *mut *mut u16 bond_swid; / swid for primary interface,
    pub /: *mut *mut u8 active_port; / lport value for the current active port,
    pub /: *mut *mut u8 bonded:1; / currently bonded,
    pub /: *mut *mut u8 primary:1; / this is primary,
    pub /: *mut *mut u8 bond_aa:1; / is this bond active-active,
    pub /: *mut *mut u8 need_fltr_cfg:1; / fltrs for A/A bond still need to be make,
    pub /: *mut *mut u8 port_bitmap:2; / bitmap of active ports,
    pub /: *mut *mut u8 bond_lport_pri; / lport values for primary PF,
    pub /: *mut *mut u8 bond_lport_sec; / lport values for secondary PF,
// q_home keeps track of which interface the q is currently on
    pub q_home: [u8; ICE_MAX_SRIOV_VFS][ICE_MAX_RSS_QS_PER_VF],
// placeholder VSI for hanging VF queues from on secondary interface
    pub sec_vf: [*mut ice_vsi; ICE_MAX_SRIOV_VFS],
    pub pf_recipe: u16,
    pub lport_recipe: u16,
    pub act_act_recipe: u16,
    pub pf_rx_rule_id: u16,
    pub pf_tx_rule_id: u16,
    pub cp_rule_idx: u16,
    pub lport_rule_idx: u16,
    pub act_act_rule_idx: u16,
    pub role: u8,
}

// LAG workqueue struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_lag_work {
    pub lag_task: work_struct,
    pub netdev_list: ice_lag_netdev_list,
    pub lag: *mut ice_lag,
    pub event: c_ulong,
    pub event_netdev: *mut net_device,
    pub changeupper_info: netdev_notifier_changeupper_info,
    pub bonding_info: netdev_notifier_bonding_info,
    pub notifier_info: netdev_notifier_info,
    pub info: },
}

extern "C" {
    pub fn ice_lag_aa_failover(lag: *mut ice_lag, dest: u8, e_pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_init_lag(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_deinit_lag(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_lag_rebuild(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_lag_is_switchdev_running(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_lag_move_vf_nodes_cfg(lag: *mut ice_lag, src_prt: u8, dst_prt: u8);
}
extern "C" {
    pub fn ice_lag_prepare_vf_reset(lag: *mut ice_lag) -> u8;
}
extern "C" {
    pub fn ice_lag_complete_vf_reset(lag: *mut ice_lag, act_prt: u8);
}
