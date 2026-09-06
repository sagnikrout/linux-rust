//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_nic.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_nic.h: Declaration of common code for NIC.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_fc_mode {
    AQ_NIC_FC_OFF = 0,
    AQ_NIC_FC_TX,
    AQ_NIC_FC_RX,
    AQ_NIC_FC_FULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_fc_info {
    pub req: aq_fc_mode,
    pub cur: aq_fc_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_nic_cfg_s {
    pub aq_hw_caps: *const aq_hw_caps_s,
    pub features: u64,
    pub /: *mut *mut u32 rxds; / rx ring size, descriptors #,
    pub /: *mut *mut u32 txds; / tx ring size, descriptors #,
    pub /: *mut *mut u32 vecs; / allocated rx/tx vectors,
    pub link_irq_vec: u32,
    pub irq_type: u32,
    pub itr: u32,
    pub rx_itr: u16,
    pub tx_itr: u16,
    pub rxpageorder: u32,
    pub num_rss_queues: u32,
    pub mtu: u32,
    pub fc: aq_fc_info,
    pub link_speed_msk: u32,
    pub wol: u32,
    pub is_vlan_rx_strip: u8,
    pub is_vlan_tx_insert: u8,
    pub is_vlan_force_promisc: bool,
    pub is_mc_list_enabled: u16,
    pub mc_list_count: u16,
    pub is_autoneg: bool,
    pub is_polling: bool,
    pub is_rss: bool,
    pub is_lro: bool,
    pub is_qos: bool,
    pub is_ptp: bool,
    pub is_media_detect: bool,
    pub downshift_counter: c_int,
    pub tc_mode: aq_tc_mode,
    pub priv_flags: u32,
    pub tcs: u8,
    pub prio_tc_map: [u8; 8],
    pub tc_max_rate: [u32; AQ_CFG_TCS_MAX],
    pub tc_min_rate_msk: c_ulong,
    pub tc_min_rate: [u32; AQ_CFG_TCS_MAX],
    pub aq_rss: aq_rss_parameters,
    pub eee_speeds: u32,
}

pub const AQ_NIC_FLAG_STARTED: c_uint = 0x00000004U;
pub const AQ_NIC_FLAG_STOPPING: c_uint = 0x00000008U;
pub const AQ_NIC_FLAG_RESETTING: c_uint = 0x00000010U;
pub const AQ_NIC_FLAG_CLOSING: c_uint = 0x00000020U;
pub const AQ_NIC_PTP_DPATH_UP: c_uint = 0x02000000U;
pub const AQ_NIC_LINK_DOWN: c_uint = 0x04000000U;
pub const AQ_NIC_FLAG_ERR_UNPLUG: c_uint = 0x40000000U;
pub const AQ_NIC_FLAG_ERR_HW: c_uint = 0x80000000U;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_rx_fl2 {
    pub aq_vlans: [aq_rx_filter_vlan; AQ_VLAN_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_rx_fl3l4 {
    pub active_ipv4: u8,
    pub active_ipv6:2: u8,
    pub is_ipv6: u8,
    pub reserved_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_rx_fltrs_s {
    pub filter_list: hlist_head,
    pub active_filters: u16,
    pub fl2: aq_hw_rx_fl2,
    pub fl3l4: aq_hw_rx_fl3l4,
// filter ether type
    pub fet_reserved_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_nic_s {
    pub flags: core::sync::atomic::AtomicI32,
    pub msg_enable: u32,
    pub aq_vec: [*mut aq_vec_s; AQ_CFG_VECS_MAX],
    pub aq_ring_tx: [*mut aq_ring_s; AQ_HW_QUEUES_MAX],
    pub aq_hw: *mut aq_hw_s,
    pub xdp_prog: *mut bpf_prog,
    pub ndev: *mut net_device,
    pub aq_vecs: c_uint,
    pub packet_filter: c_uint,
    pub power_state: c_uint,
    pub port: u8,
    pub aq_hw_ops: *const aq_hw_ops,
    pub aq_fw_ops: *const aq_fw_ops,
    pub aq_nic_cfg: aq_nic_cfg_s,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub polling_timer: timer_list,
    pub link_status: aq_hw_link_status_s,
    pub count: u32,
    pub ar: [u8; AQ_HW_MULTICAST_ADDRESS_MAX][ETH_ALEN],
    pub mc_list: },
// Bitmask of currently assigned vlans from linux
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub pdev: *mut pci_dev,
    pub msix_entry_mask: c_uint,
    pub irqvecs: u32,
// mutex to serialize FW interface access operations
    pub fwreq_mutex: mutex,

    pub macsec_cfg: *mut aq_macsec_cfg,
// mutex to protect data in macsec_cfg
    pub macsec_mutex: mutex,

// PTP support
    pub aq_ptp: *mut aq_ptp_s,
    pub aq_hw_rx_fltrs: aq_hw_rx_fltrs_s,
}

extern "C" {
    pub fn aq_nic_ndev_init(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_init(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_cfg_start(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_ndev_register(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_ndev_free(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_start(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_xmit(self: *mut aq_nic_s, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn aq_nic_get_regs(self: *mut aq_nic_s, regs: *mut ethtool_regs, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn aq_nic_get_regs_count(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_stop(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_deinit(self: *mut aq_nic_s, link_down: bool);
}
extern "C" {
    pub fn aq_nic_set_power(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_free_hot_resources(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_free_vectors(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_realloc_vectors(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_mtu(self: *mut aq_nic_s, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_mac(self: *mut aq_nic_s, ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_packet_filter(self: *mut aq_nic_s, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_multicast_list(self: *mut aq_nic_s, ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn aq_nic_get_link_speed(self: *mut aq_nic_s) -> c_uint;
}
extern "C" {
    pub fn aq_nic_get_fw_version(self: *mut aq_nic_s) -> u32;
}
extern "C" {
    pub fn aq_nic_set_loopback(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_downshift(self: *mut aq_nic_s, val: c_int) -> c_int;
}
extern "C" {
    pub fn aq_nic_set_media_detect(self: *mut aq_nic_s, val: c_int) -> c_int;
}
extern "C" {
    pub fn aq_nic_update_interrupt_moderation_settings(self: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_nic_shutdown(self: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_nic_reserve_filter(self: *mut aq_nic_s, type: aq_rx_filter_type) -> u8;
}
extern "C" {
    pub fn aq_nic_setup_tc_mqprio(self: *mut aq_nic_s, tcs: u32, prio_tc_map: *mut u8) -> c_int;
}
