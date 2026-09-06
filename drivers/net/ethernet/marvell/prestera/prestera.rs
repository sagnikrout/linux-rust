//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2019-2020 Marvell International Ltd. All rights reserved.

pub const PRESTERA_DEFAULT_VID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_rev {
    pub maj: u16,
    pub min: u16,
    pub sub: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_flood_domain {
    pub sw: *mut prestera_switch,
    pub flood_domain_port_list: list_head,
    pub idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_mdb_entry {
    pub sw: *mut prestera_switch,
    pub flood_domain: *mut prestera_flood_domain,
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_flood_domain_port {
    pub flood_domain: *mut prestera_flood_domain,
    pub dev: *mut net_device,
    pub flood_domain_port_node: list_head,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_stats {
    pub good_octets_received: u64,
    pub bad_octets_received: u64,
    pub mac_trans_error: u64,
    pub broadcast_frames_received: u64,
    pub multicast_frames_received: u64,
    pub frames_64_octets: u64,
    pub frames_65_to_127_octets: u64,
    pub frames_128_to_255_octets: u64,
    pub frames_256_to_511_octets: u64,
    pub frames_512_to_1023_octets: u64,
    pub frames_1024_to_max_octets: u64,
    pub excessive_collision: u64,
    pub multicast_frames_sent: u64,
    pub broadcast_frames_sent: u64,
    pub fc_sent: u64,
    pub fc_received: u64,
    pub buffer_overrun: u64,
    pub undersize: u64,
    pub fragments: u64,
    pub oversize: u64,
    pub jabber: u64,
    pub rx_error_frame_received: u64,
    pub bad_crc: u64,
    pub collisions: u64,
    pub late_collision: u64,
    pub unicast_frames_received: u64,
    pub unicast_frames_sent: u64,
    pub sent_multiple: u64,
    pub sent_deferred: u64,
    pub good_octets_sent: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_caps {
    pub supp_link_modes: u64,
    pub supp_fec: u8,
    pub type: u8,
    pub transceiver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_lag {
    pub dev: *mut net_device,
    pub members: list_head,
    pub member_count: u16,
    pub lag_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_mac_state {
    pub valid: bool,
    pub mode: u32,
    pub speed: u32,
    pub oper: bool,
    pub duplex: u8,
    pub fc: u8,
    pub fec: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_phy_state {
    pub lmode_bmap: u64,
    pub pause: bool,
    pub asym_pause: bool,
    pub remote_fc: },
    pub mdix: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_mac_config {
    pub mode: u32,
    pub speed: u32,
    pub admin: bool,
    pub inband: u8,
    pub duplex: u8,
    pub fec: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_phy_config {
    pub mode: u32,
    pub admin: bool,
    pub mdix: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port {
    pub dev: *mut net_device,
    pub sw: *mut prestera_switch,
    pub ingress_flow_block: *mut prestera_flow_block,
    pub egress_flow_block: *mut prestera_flow_block,
    pub dl_port: devlink_port,
    pub lag_member: list_head,
    pub lag: *mut prestera_lag,
    pub id: u32,
    pub hw_id: u32,
    pub dev_id: u32,
    pub fp_id: u16,
    pub pvid: u16,
    pub autoneg: bool,
    pub adver_link_modes: u64,
    pub adver_fec: u8,
    pub caps: prestera_port_caps,
    pub list: list_head,
    pub vlans_list: list_head,
    pub stats: prestera_port_stats,
    pub caching_dw: delayed_work,
    pub cached_hw_stats: },
    pub cfg_mac: prestera_port_mac_config,
    pub cfg_phy: prestera_port_phy_config,
    pub state_mac: prestera_port_mac_state,
    pub state_phy: prestera_port_phy_state,
    pub phy_config: phylink_config,
    pub phy_link: *mut phylink,
    pub phylink_pcs: phylink_pcs,
// protects state_mac
    pub state_mac_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_device {
    pub dev: *mut device,
    pub ctl_regs: *mut u8 __iomem,
    pub pp_regs: *mut u8 __iomem,
    pub fw_rev: prestera_fw_rev,
    pub priv: *mut c_void,
// called by device driver to handle received packets
    pub dev): *mut *mut void (recv_pkt)(struct prestera_device,
// called by device driver to pass event up to the higher layer
    pub size): *mut *mut *mut *mut int (recv_msg)(struct prestera_device dev, void msg, size_t,
// called by higher layer to send request to the firmware
    pub wait): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_event_type {
    PRESTERA_EVENT_TYPE_UNSPEC,

    PRESTERA_EVENT_TYPE_PORT,
    PRESTERA_EVENT_TYPE_FDB,
    PRESTERA_EVENT_TYPE_RXTX,

    PRESTERA_EVENT_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_rxtx_event_id {
    PRESTERA_RXTX_EVENT_UNSPEC,
    PRESTERA_RXTX_EVENT_RCV_PKT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_port_event_id {
    PRESTERA_PORT_EVENT_UNSPEC,
    PRESTERA_PORT_EVENT_MAC_STATE_CHANGED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_port_event {
    pub port_id: u32,
    pub mode: u32,
    pub speed: u32,
    pub oper: u8,
    pub duplex: u8,
    pub fc: u8,
    pub fec: u8,
    pub mac: },
    pub lmode_bmap: u64,
    pub pause: bool,
    pub asym_pause: bool,
    pub remote_fc: },
    pub mdix: u8,
    pub phy: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_fdb_entry_type {
    PRESTERA_FDB_ENTRY_TYPE_REG_PORT,
    PRESTERA_FDB_ENTRY_TYPE_LAG,
    PRESTERA_FDB_ENTRY_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_fdb_event_id {
    PRESTERA_FDB_EVENT_UNSPEC,
    PRESTERA_FDB_EVENT_LEARNED,
    PRESTERA_FDB_EVENT_AGED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fdb_event {
    pub type: prestera_fdb_entry_type,
    pub port_id: u32,
    pub lag_id: u16,
    pub dest: },
    pub vid: u32,
    pub mac: [u8; ETH_ALEN],
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_event {
    pub id: u16,
    pub port_evt: prestera_port_event,
    pub fdb_evt: prestera_fdb_event,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_if_type {
// the interface is of port type (dev,port)
    PRESTERA_IF_PORT_E = 0,

// the interface is of lag type (lag-id)
    PRESTERA_IF_LAG_E = 1,

// the interface is of Vid type (vlan-id)
    PRESTERA_IF_VID_E = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_iface {
    pub type: prestera_if_type,
    pub hw_dev_num: u32,
    pub port_num: u32,
    pub dev_port: },
    pub hw_dev_num: u32,
    pub vr_id: u16,
    pub lag_id: u16,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_switch {
    pub dev: *mut prestera_device,
    pub swdev: *mut prestera_switchdev,
    pub rxtx: *mut prestera_rxtx,
    pub acl: *mut prestera_acl,
    pub span: *mut prestera_span,
    pub event_handlers: list_head,
    pub netdev_nb: notifier_block,
    pub trap_data: *mut prestera_trap_data,
    pub base_mac: [c_char; ETH_ALEN],
    pub port_list: list_head,
    pub port_list_lock: rwlock_t,
    pub port_count: u32,
    pub mtu_min: u32,
    pub mtu_max: u32,
    pub id: u8,
    pub np: *mut device_node,
    pub router: *mut prestera_router,
    pub lags: *mut prestera_lag,
    pub counter: *mut prestera_counter,
    pub lag_member_max: u8,
    pub lag_max: u8,
    pub size_tbl_router_nexthop: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_router {
    pub sw: *mut prestera_switch,
    pub vr_list: list_head,
    pub rif_entry_list: list_head,
    pub nh_neigh_ht: rhashtable,
    pub nexthop_group_ht: rhashtable,
    pub fib_ht: rhashtable,
    pub kern_neigh_cache_ht: rhashtable,
    pub kern_fib_cache_ht: rhashtable,
    pub inetaddr_nb: notifier_block,
    pub inetaddr_valid_nb: notifier_block,
    pub fib_nb: notifier_block,
    pub netevent_nb: notifier_block,
    pub /: *mut *mut *mut u8 nhgrp_hw_state_cache; / Bitmap cached hw state of nhs,
    pub /: *mut *mut unsigned long nhgrp_hw_cache_kick; / jiffies,
    pub dw: delayed_work,
    pub neighs_update: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_rxtx_params {
    pub use_sdma: bool,
    pub map_addr: u32,
}

extern "C" {
    pub fn readl(reg: sw->dev->pp_regs +) -> return;
}
extern "C" {
    pub fn prestera_device_register(dev: *mut prestera_device) -> c_int;
}
extern "C" {
    pub fn prestera_device_unregister(dev: *mut prestera_device);
}
extern "C" {
    pub fn prestera_port_autoneg_set(port: *mut prestera_port, link_modes: u64) -> c_int;
}
extern "C" {
    pub fn prestera_router_init(sw: *mut prestera_switch) -> c_int;
}
extern "C" {
    pub fn prestera_router_fini(sw: *mut prestera_switch);
}
extern "C" {
    pub fn prestera_queue_work(work: *mut work_struct);
}
extern "C" {
    pub fn prestera_queue_delayed_work(work: *mut delayed_work, delay: c_ulong);
}
extern "C" {
    pub fn prestera_queue_drain();
}
extern "C" {
    pub fn prestera_port_learning_set(port: *mut prestera_port, learn_enable: bool) -> c_int;
}
extern "C" {
    pub fn prestera_port_uc_flood_set(port: *mut prestera_port, flood: bool) -> c_int;
}
extern "C" {
    pub fn prestera_port_mc_flood_set(port: *mut prestera_port, flood: bool) -> c_int;
}
extern "C" {
    pub fn prestera_port_br_locked_set(port: *mut prestera_port, br_locked: bool) -> c_int;
}
extern "C" {
    pub fn prestera_port_pvid_set(port: *mut prestera_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn prestera_netdev_check(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn prestera_is_valid_mac_addr(port: *mut prestera_port, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn prestera_port_is_lag_member(port: *const prestera_port) -> bool;
}
extern "C" {
    pub fn prestera_port_lag_id(port: *const prestera_port) -> u16;
}
extern "C" {
    pub fn prestera_mdb_entry_destroy(mdb_entry: *mut prestera_mdb_entry);
}
extern "C" {
    pub fn prestera_flood_domain_destroy(flood_domain: *mut prestera_flood_domain);
}
