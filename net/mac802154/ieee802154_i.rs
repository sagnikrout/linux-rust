//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac802154/ieee802154_i.h
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
//
// Copyright (C) 2007-2012 Siemens AG
//
// Written by:
// Pavel Smolenskiy <pavel.smolenskiy@gmail.com>
// Maxim Gorbachyov <maxim.gorbachev@siemens.com>
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_ongoing {
    IEEE802154_IS_SCANNING = BIT(0),
    IEEE802154_IS_BEACONING = BIT(1),
    IEEE802154_IS_ASSOCIATING = BIT(2),
}

// mac802154 device private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_local {
    pub hw: ieee802154_hw,
    pub ops: *const ieee802154_ops,
// hardware address filter
    pub addr_filt: ieee802154_hw_addr_filt,
// ieee802154 phy
    pub phy: *mut wpan_phy,
    pub open_count: c_int,
// As in mac80211 slaves list is modified:
// 1) under the RTNL
// 2) protected by slaves_mtx;
// 3) in an RCU manner
//
// So atomic readers can use any of this protection methods.
//
    pub interfaces: list_head,
    pub iflist_mtx: mutex,
// Data related workqueue
    pub workqueue: *mut workqueue_struct,
// MAC commands related workqueue
    pub mac_wq: *mut workqueue_struct,
    pub ifs_timer: hrtimer,
// Scanning
    pub scan_page: u8,
    pub scan_channel: u8,
    pub scan_beacon_req: ieee802154_beacon_req_frame,
    pub scan_req: *mut cfg802154_scan_request __rcu,
    pub scan_work: delayed_work,
// Beaconing
    pub beacon_interval: c_uint,
    pub beacon: ieee802154_beacon_frame,
    pub beacon_req: *mut cfg802154_beacon_request __rcu,
    pub beacon_work: delayed_work,
// Asynchronous tasks
    pub rx_beacon_list: list_head,
    pub rx_beacon_work: work_struct,
    pub rx_mac_cmd_list: list_head,
    pub rx_mac_cmd_work: work_struct,
// Association
// assoc_lock protects assoc_dev_extended_addr, assoc_addr,
// assoc_status, the assoc_done reinit/complete pairing and the
// IEEE802154_IS_ASSOCIATING bit in @ongoing.
//
    pub assoc_lock: spinlock_t,
    pub assoc_dev_extended_addr: __le64,
    pub assoc_done: completion,
    pub assoc_addr: __le16,
    pub assoc_status: u8,
    pub assoc_work: work_struct,
    pub started: bool,
    pub suspended: bool,
    pub ongoing: c_ulong,
    pub tasklet: tasklet_struct,
    pub skb_queue: sk_buff_head,
    pub tx_skb: *mut sk_buff,
    pub sync_tx_work: work_struct,
// A negative Linux error code or a null/positive MLME error status
    pub tx_result: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_sdata_state_bits {
    SDATA_STATE_RUNNING,
}

// Slave interface definition.
//
// Slaves represent typical network interfaces available from userspace.
// Each ieee802154 device/transceiver may have several slaves and able
// to be associated with several networks at the same time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_sub_if_data {
    pub /: *mut *mut list_head list; / the ieee802154_priv->slaves list,
    pub wpan_dev: wpan_dev,
    pub local: *mut ieee802154_local,
    pub dev: *mut net_device,
// Each interface starts and works in nominal state at a given filtering
// level given by iface_default_filtering, which is set once for all at
// the interface creation and should not evolve over time. For some MAC
// operations however, the filtering level may change temporarily, as
// reflected in the required_filtering field. The actual filtering at
// the PHY level may be different and is shown in struct wpan_phy.
//
    pub iface_default_filtering: ieee802154_filtering_level,
    pub required_filtering: ieee802154_filtering_level,
    pub state: c_ulong,
    pub name: [c_char; IFNAMSIZ],
// protects sec from concurrent access by netlink. access by
// encrypt/decrypt/header_create safe without additional protection.
//
    pub sec_mtx: mutex,
    pub sec: mac802154_llsec,
}

// utility functions/constants
extern "C" {
    pub fn container_of(_arg: hw, ieee802154_local: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: wpan_dev, ieee802154_sub_if_data: struct, _arg: wpan_dev) -> return;
}
extern "C" {
    pub fn test_bit(_arg: SDATA_STATE_RUNNING, _arg: &sdata->state) -> return;
}
// mac_cmd = mac_pl.cmd_id;
extern "C" {
    pub fn ieee802154_rx(local: *mut ieee802154_local, skb: *mut sk_buff);
}
extern "C" {
    pub fn ieee802154_xmit_sync_worker(work: *mut work_struct);
}
extern "C" {
    pub fn ieee802154_sync_and_hold_queue(local: *mut ieee802154_local) -> c_int;
}
extern "C" {
    pub fn ieee802154_mlme_op_pre(local: *mut ieee802154_local) -> c_int;
}
extern "C" {
    pub fn ieee802154_mlme_op_post(local: *mut ieee802154_local);
}
extern "C" {
    pub fn ieee802154_xmit_ifs_timer(timer: *mut hrtimer) -> hrtimer_restart;
}
//
// ieee802154_hold_queue - hold ieee802154 queue
// @local: main mac object
//
// Hold a queue by incrementing an atomic counter and requesting the netif
// queues to be stopped. The queues cannot be woken up while the counter has not
// been reset with as any ieee802154_release_queue() calls as needed.
//
extern "C" {
    pub fn ieee802154_hold_queue(local: *mut ieee802154_local);
}
//
// ieee802154_release_queue - release ieee802154 queue
// @local: main mac object
//
// Release a queue which is held by decrementing an atomic counter and wake it
// up only if the counter reaches 0.
//
extern "C" {
    pub fn ieee802154_release_queue(local: *mut ieee802154_local);
}
//
// ieee802154_disable_queue - disable ieee802154 queue
// @local: main mac object
//
// When trying to sync the Tx queue, we cannot just stop the queue
// (which is basically a bit being set without proper lock handling)
// because it would be racy. We actually need to call netif_tx_disable()
// instead, which is done by this helper. Restarting the queue can
// however still be done with a regular wake call.
//
extern "C" {
    pub fn ieee802154_disable_queue(local: *mut ieee802154_local);
}
// MIB callbacks
extern "C" {
    pub fn mac802154_dev_set_page_channel(dev: *mut net_device, page: u8, chan: u8);
}
extern "C" {
    pub fn mac802154_del_dev(dev: *mut net_device, dev_addr: __le64) -> c_int;
}
extern "C" {
    pub fn mac802154_lock_table(dev: *mut net_device);
}
extern "C" {
    pub fn mac802154_unlock_table(dev: *mut net_device);
}
extern "C" {
    pub fn mac802154_wpan_update_llsec(dev: *mut net_device) -> c_int;
}
// PAN management handling
extern "C" {
    pub fn mac802154_scan_worker(work: *mut work_struct);
}
extern "C" {
    pub fn mac802154_rx_beacon_worker(work: *mut work_struct);
}
extern "C" {
    pub fn test_bit(_arg: IEEE802154_IS_SCANNING, _arg: &local->ongoing) -> return;
}
extern "C" {
    pub fn mac802154_beacon_worker(work: *mut work_struct);
}
extern "C" {
    pub fn test_bit(_arg: IEEE802154_IS_BEACONING, _arg: &local->ongoing) -> return;
}
extern "C" {
    pub fn mac802154_rx_mac_cmd_worker(work: *mut work_struct);
}
extern "C" {
    pub fn test_bit(_arg: IEEE802154_IS_ASSOCIATING, _arg: &local->ongoing) -> return;
}
// interface handling
extern "C" {
    pub fn ieee802154_iface_init() -> c_int;
}
extern "C" {
    pub fn ieee802154_iface_exit();
}
extern "C" {
    pub fn ieee802154_if_remove(sdata: *mut ieee802154_sub_if_data);
}
extern "C" {
    pub fn ieee802154_remove_interfaces(local: *mut ieee802154_local);
}
extern "C" {
    pub fn ieee802154_stop_device(local: *mut ieee802154_local);
}
