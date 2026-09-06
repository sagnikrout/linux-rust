//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_enet.h
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
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const HNS_DEBUG_OFFSET: c_int = 6;
pub const HNS_SRV_OFFSET: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_nic_state {
    NIC_STATE_TESTING = 0,
    NIC_STATE_RESETTING,
    NIC_STATE_REINITING,
    NIC_STATE_DOWN,
    NIC_STATE_DISABLED,
    NIC_STATE_REMOVING,
    NIC_STATE_SERVICE_INITED,
    NIC_STATE_SERVICE_SCHED,
    NIC_STATE2_RESET_REQUESTED,
    NIC_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_nic_ring_data {
    pub ring: *mut hnae_ring,
    pub napi: napi_struct,
    pub /: *mut *mut cpumask_t mask; / affinity mask,
    pub queue_index: u32,
    pub ): *mut *mut *mut int (poll_one)(struct hns_nic_ring_data , int, void,
    pub ): *mut *mut *mut void (ex_process)(struct hns_nic_ring_data , struct sk_buff,
    pub ): *mut *mut bool (fini_process)(struct hns_nic_ring_data,
}

// compatible the difference between two versions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_nic_ops {
    pub is_gso): bool,
    pub ring): *mut *mut int bnum, struct hnae_ring,
    pub out_bnum): *mut *mut void (get_rxd_bnum)(u32 bnum_flag, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_nic_priv {
    pub fwnode: *const fwnode_handle,
    pub enet_ver: u32,
    pub port_id: u32,
    pub phy_mode: c_int,
    pub phy_led_val: c_int,
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub ae_handle: *mut hnae_handle,
    pub ops: hns_nic_ops,
// the cb for nic to manage the ring buffer, the first half of the
// array is for tx_ring and vice versa for the second half
//
    pub ring_data: *mut hns_nic_ring_data,
// The most recently read link state
    pub link: c_int,
    pub tx_timeout_count: u64,
    pub state: c_ulong,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub notifier_block: notifier_block,
}

extern "C" {
    pub fn hns_ethtool_set_ops(ndev: *mut net_device);
}
extern "C" {
    pub fn hns_nic_net_reset(ndev: *mut net_device);
}
extern "C" {
    pub fn hns_nic_net_reinit(netdev: *mut net_device);
}
extern "C" {
    pub fn hns_nic_init_phy(ndev: *mut net_device, h: *mut hnae_handle) -> c_int;
}
