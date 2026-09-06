//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipvlan/ipvlan.h
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
// Copyright (c) 2014 Mahesh Bandewar <maheshb@google.com>
//

pub const IPVLAN_MAC_FILTER_BITS: c_int = 8;

pub const IPVLAN_QBACKLOG_LIMIT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvl_pcpu_stats {
    pub rx_pkts: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_mcast: u64_stats_t,
    pub tx_pkts: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub rx_errs: u32,
    pub tx_drps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvl_dev {
    pub dev: *mut net_device,
    pub pnode: list_head,
    pub port: *mut ipvl_port,
    pub phy_dev: *mut net_device,
    pub addrs: list_head,
    pub pcpu_stats: *mut ipvl_pcpu_stats __percpu,
    pub IPVLAN_MAC_FILTER_SIZE): DECLARE_BITMAP(mac_filters,,
    pub sfeatures: netdev_features_t,
    pub msg_enable: u32,
    pub dying: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvl_addr {
    pub /: *mut *mut *mut ipvl_dev master; / Back pointer to master,
    pub /: *mut *mut in6_addr ip6; / IPv6 address on logical interface,
    pub /: *mut *mut in_addr ip4; / IPv4 address on logical interface,
    pub ipu: },

    pub /: *mut *mut hlist_node hlnode; / Hash-table linkage,
    pub /: *mut *mut list_head anode; / logical-interface linkage,
    pub atype: ipvl_hdr_type,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvl_port {
    pub dev: *mut net_device,
    pub pnet: possible_net_t,
    pub hlhead: [hlist_head; IPVLAN_HASH_SIZE],
    pub /: *mut *mut spinlock_t addrs_lock; / guards hash-table and addrs,
    pub ipvlans: list_head,
    pub pnodes_lock: mutex,
    pub mode: u16,
    pub flags: u16,
    pub dev_id_start: u16,
    pub wq: work_struct,
    pub backlog: sk_buff_head,
    pub count: refcount_t,
    pub ida: ida,
    pub dev_tracker: netdevice_tracker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipvl_skb_cb {
    pub tx_pkt: bool,
}

extern "C" {
    pub fn rcu_dereference(_arg: d->rx_handler_data) -> return;
}
extern "C" {
    pub fn rcu_dereference_bh(_arg: d->rx_handler_data) -> return;
}
extern "C" {
    pub fn rtnl_dereference(_arg: d->rx_handler_data) -> return;
}
extern "C" {
    pub fn ipvlan_init_secret();
}
extern "C" {
    pub fn ipvlan_mac_hash(addr: *const c_uchar) -> c_uint;
}
extern "C" {
    pub fn ipvlan_handle_frame(pskb: *mut sk_buff) -> rx_handler_result_t;
}
extern "C" {
    pub fn ipvlan_process_multicast(work: *mut work_struct);
}
extern "C" {
    pub fn ipvlan_queue_xmit(skb: *mut sk_buff, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipvlan_ht_addr_add(ipvlan: *mut ipvl_dev, addr: *mut ipvl_addr);
}
extern "C" {
    pub fn ipvlan_addr_busy(port: *mut ipvl_port, iaddr: *mut c_void, is_v6: bool) -> bool;
}
extern "C" {
    pub fn ipvlan_ht_addr_del(addr: *mut ipvl_addr);
}
extern "C" {
    pub fn ipvlan_link_setup(dev: *mut net_device);
}
extern "C" {
    pub fn ipvlan_link_register(ops: *mut rtnl_link_ops) -> c_int;
}

extern "C" {
    pub fn ipvlan_l3s_register(port: *mut ipvl_port) -> c_int;
}
extern "C" {
    pub fn ipvlan_l3s_unregister(port: *mut ipvl_port);
}
extern "C" {
    pub fn ipvlan_migrate_l3s_hook(oldnet: *mut net, newnet: *mut net);
}
extern "C" {
    pub fn ipvlan_l3s_init() -> c_int;
}
extern "C" {
    pub fn ipvlan_l3s_cleanup();
}

