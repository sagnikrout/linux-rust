//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_nic_dev.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_flags {
    HINIC3_INTF_UP,
    HINIC3_MAC_FILTER_CHANGED,
    HINIC3_RSS_ENABLE,
    HINIC3_UPDATE_MAC_FILTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_event_work_flags {
    HINIC3_EVENT_WORK_TX_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_rx_mode_state {
    HINIC3_HW_PROMISC_ON,
    HINIC3_HW_ALLMULTI_ON,
    HINIC3_PROMISC_FORCE_ON,
    HINIC3_ALLMULTI_FORCE_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_mac_filter_state {
    HINIC3_MAC_WAIT_HW_SYNC,
    HINIC3_MAC_HW_SYNCING,
    HINIC3_MAC_HW_SYNCED,
    HINIC3_MAC_WAIT_HW_UNSYNC,
    HINIC3_MAC_HW_UNSYNCED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_mac_filter {
    pub list: list_head,
    pub addr: [u8; ETH_ALEN],
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_rss_hash_type {
    HINIC3_RSS_HASH_ENGINE_TYPE_XOR  = 0,
    HINIC3_RSS_HASH_ENGINE_TYPE_TOEP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_rss_type {
    pub tcp_ipv6_ext: u8,
    pub ipv6_ext: u8,
    pub tcp_ipv6: u8,
    pub ipv6: u8,
    pub tcp_ipv4: u8,
    pub ipv4: u8,
    pub udp_ipv6: u8,
    pub udp_ipv4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_irq_cfg {
    pub netdev: *mut net_device,
    pub msix_entry_idx: u16,
// provided by OS
    pub irq_id: u32,
    pub 16]: char irq_name[IFNAMSIZ +,
    pub napi: napi_struct,
    pub affinity_mask: cpumask_t,
    pub txq: *mut hinic3_txq,
    pub rxq: *mut hinic3_rxq,
    pub total_events: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dyna_txrxq_params {
    pub num_qps: u16,
    pub sq_depth: u32,
    pub rq_depth: u32,
    pub txqs_res: *mut hinic3_dyna_txq_res,
    pub rxqs_res: *mut hinic3_dyna_rxq_res,
    pub irq_cfg: *mut hinic3_irq_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_intr_coal_info {
    pub pending_limit: u8,
    pub coalesce_timer_cfg: u8,
    pub resend_timer_cfg: u8,
    pub rx_pending_limit_low: u8,
    pub rx_pending_limit_high: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_dev {
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub hwdev: *mut hinic3_hwdev,
    pub nic_io: *mut hinic3_nic_io,
    pub msg_enable: u32,
    pub max_qps: u16,
    pub rx_buf_len: u16,
    pub lro_replenish_thld: u32,
    pub vlan_bitmap: *mut c_ulong,
    pub flags: c_ulong,
    pub nic_svc_cap: hinic3_nic_service_cap,
    pub q_params: hinic3_dyna_txrxq_params,
    pub txqs: *mut hinic3_txq,
    pub rxqs: *mut hinic3_rxq,
    pub rss_hash_type: hinic3_rss_hash_type,
    pub rss_type: hinic3_rss_type,
    pub rss_hkey: *mut u8,
    pub rss_indir: *mut u16,
    pub num_qp_irq: u16,
    pub qps_msix_entries: *mut msix_entry,
    pub intr_coalesce: *mut hinic3_intr_coal_info,
    pub adaptive_rx_coal: u32,
    pub workq: *mut workqueue_struct,
    pub periodic_work: delayed_work,
    pub rx_mode_work: work_struct,
// lock for enable/disable port
    pub port_state_mutex: mutex,
    pub uc_filter_list: list_head,
    pub mc_filter_list: list_head,
    pub rx_mod_state: c_ulong,
    pub netdev_uc_cnt: c_int,
    pub netdev_mc_cnt: c_int,
// flag bits defined by hinic3_event_work_flags
    pub event_flag: c_ulong,
    pub link_status_up: bool,
}

extern "C" {
    pub fn hinic3_set_netdev_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic3_set_hw_features(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hinic3_qps_irq_init(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hinic3_qps_irq_uninit(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic3_set_rx_mode_work(work: *mut work_struct);
}
extern "C" {
    pub fn hinic3_clean_mac_list_filter(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic3_set_ethtool_ops(netdev: *mut net_device);
}
