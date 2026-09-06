//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/ipoib/ipoib.h
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


//
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2004 Voltaire, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// constants
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipoib_flush_level {
    IPOIB_FLUSH_LIGHT,
    IPOIB_FLUSH_NORMAL,
    IPOIB_FLUSH_HEAVY
}

//
// For IPOIB_MCAST_FLAG_BUSY
// When set, in flight join and mcast->mc is unreliable
// When clear and mcast->mc IS_ERR_OR_NULL, need to restart or
// haven't started yet
// When clear and mcast->mc is valid pointer, join was successful
//

// structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_header {
    pub proto: __be16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_pseudo_header {
    pub hwaddr: [u8; INFINIBAND_ALEN],
}

//
// only the ipoib header is present now, make room for a dummy
// pseudo header and set skb field accordingly
//
// Used for all multicast joins (broadcast, IPv4 mcast and IPv6 mcast)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_mcast {
    pub mcmember: ib_sa_mcmember_rec,
    pub mc: *mut ib_sa_multicast,
    pub ah: *mut ipoib_ah,
    pub rb_node: rb_node,
    pub list: list_head,
    pub created: c_ulong,
    pub backoff: c_ulong,
    pub delay_until: c_ulong,
    pub flags: c_ulong,
    pub logcount: c_uchar,
    pub neigh_list: list_head,
    pub pkt_queue: sk_buff_head,
    pub dev: *mut net_device,
    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_rx_buf {
    pub skb: *mut sk_buff,
    pub mapping: [u64; IPOIB_UD_RX_SG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_tx_buf {
    pub skb: *mut sk_buff,
    pub 1]: u64 mapping[MAX_SKB_FRAGS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_cm_data {
    pub /: *mut *mut __be32 qpn; / High byte MUST be ignored on receive,
    pub mtu: __be32,
}

//
// Quoting 10.3.1 Queue Pair and EE Context States:
//
// Note, for QPs that are associated with an SRQ, the Consumer should take the
// QP through the Error State before invoking a Destroy QP or a Modify QP to the
// Reset State.  The Consumer may invoke the Destroy QP without first performing
// a Modify QP to the Error State and waiting for the Affiliated Asynchronous
// Last WQE Reached Event. However, if the Consumer does not wait for the
// Affiliated Asynchronous Last WQE Reached Event, then WQE and Data Segment
// leakage may occur. Therefore, it is good programming practice to tear down a
// QP that is associated with an SRQ by using the following process:
//
// - Put the QP in the Error State
// - Wait for the Affiliated Asynchronous Last WQE Reached Event;
// - either:
// drain the CQ by invoking the Poll CQ verb and either wait for CQ
// to be empty or the number of Poll CQ operations has exceeded
// CQ capacity size;
// - or
// post another WR that completes on the same CQ and wait for this
// WR to return as a WC;
// - and then invoke a Destroy QP or Reset QP.
//
// We use the second option and wait for a completion on the
// same CQ before destroying QPs attached to our SRQ.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipoib_cm_state {
    IPOIB_CM_RX_LIVE,
    IPOIB_CM_RX_ERROR, /* Ignored by stale task */
    IPOIB_CM_RX_FLUSH  /* Last WQE Reached event observed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_cm_rx {
    pub id: *mut ib_cm_id,
    pub qp: *mut ib_qp,
    pub rx_ring: *mut ipoib_cm_rx_buf,
    pub list: list_head,
    pub dev: *mut net_device,
    pub jiffies: c_ulong,
    pub state: ipoib_cm_state,
    pub recv_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_cm_tx {
    pub id: *mut ib_cm_id,
    pub qp: *mut ib_qp,
    pub list: list_head,
    pub dev: *mut net_device,
    pub neigh: *mut ipoib_neigh,
    pub tx_ring: *mut ipoib_tx_buf,
    pub tx_head: c_uint,
    pub tx_tail: c_uint,
    pub flags: c_ulong,
    pub mtu: u32,
    pub max_send_sge: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_cm_rx_buf {
    pub skb: *mut sk_buff,
    pub mapping: [u64; IPOIB_CM_RX_SG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_cm_dev_priv {
    pub srq: *mut ib_srq,
    pub srq_ring: *mut ipoib_cm_rx_buf,
    pub id: *mut ib_cm_id,
    pub /: *mut *mut list_head passive_ids; / state: LIVE,
    pub /: *mut *mut list_head rx_error_list; / state: ERROR,
    pub /: *mut *mut list_head rx_flush_list; / state: FLUSH, drain not started,
    pub /: *mut *mut list_head rx_drain_list; / state: FLUSH, drain started,
    pub /: *mut *mut list_head rx_reap_list; / state: FLUSH, drain done,
    pub start_task: work_struct,
    pub reap_task: work_struct,
    pub skb_task: work_struct,
    pub rx_reap_task: work_struct,
    pub stale_task: delayed_work,
    pub skb_queue: sk_buff_head,
    pub start_list: list_head,
    pub reap_list: list_head,
    pub ibwc: [ib_wc; IPOIB_NUM_WC],
    pub rx_sge: [ib_sge; IPOIB_CM_RX_SG],
    pub rx_wr: ib_recv_wr,
    pub nonsrq_conn_qp: c_int,
    pub max_cm_mtu: c_int,
    pub num_frags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_ethtool_st {
    pub coalesce_usecs: u16,
    pub max_coalesced_frames: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_neigh_hash {
    pub ntbl: *mut ipoib_neigh_table,
    pub buckets: *mut ipoib_neigh __rcu,
    pub rcu: rcu_head,
    pub mask: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_neigh_table {
    pub htbl: *mut ipoib_neigh_hash __rcu,
    pub entries: core::sync::atomic::AtomicI32,
    pub flushed: completion,
    pub deleted: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_qp_state_validate {
    pub work: work_struct,
    pub priv: *mut ipoib_dev_priv,
}

//
// Device private locking: network stack tx_lock protects members used
// in TX fast path, lock protects everything else.  lock nests inside
// of tx_lock (ie tx_lock must be acquired first if needed).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_dev_priv {
    pub lock: spinlock_t,
    pub dev: *mut net_device,
    pub dev): *mut *mut void (next_priv_destructor)(struct net_device,
    pub send_napi: napi_struct,
    pub recv_napi: napi_struct,
    pub flags: c_ulong,
    pub mcast_mutex: mutex,
    pub path_tree: rb_root,
    pub path_list: list_head,
    pub ntbl: ipoib_neigh_table,
    pub broadcast: *mut ipoib_mcast,
    pub multicast_list: list_head,
    pub multicast_tree: rb_root,
    pub wq: *mut workqueue_struct,
    pub mcast_task: delayed_work,
    pub carrier_on_task: work_struct,
    pub reschedule_napi_work: work_struct,
    pub flush_light: work_struct,
    pub flush_normal: work_struct,
    pub flush_heavy: work_struct,
    pub restart_task: work_struct,
    pub tx_timeout_work: work_struct,
    pub ah_reap_task: delayed_work,
    pub neigh_reap_task: delayed_work,
    pub ca: *mut ib_device,
    pub port: u8,
    pub pkey: u16,
    pub pkey_index: u16,
    pub pd: *mut ib_pd,
    pub recv_cq: *mut ib_cq,
    pub send_cq: *mut ib_cq,
    pub qp: *mut ib_qp,
    pub qkey: u32,
    pub local_gid: ib_gid,
    pub local_lid: u32,
    pub admin_mtu: c_uint,
    pub mcast_mtu: c_uint,
    pub max_ib_mtu: c_uint,
    pub rx_ring: *mut ipoib_rx_buf,
    pub tx_ring: *mut ipoib_tx_buf,
// cyclic ring variables for managing tx_ring, for UD only
    pub tx_head: c_uint,
    pub tx_tail: c_uint,
// cyclic ring variables for counting overall outstanding send WRs
    pub global_tx_head: c_uint,
    pub global_tx_tail: c_uint,
    pub 1]: ib_sge tx_sge[MAX_SKB_FRAGS +,
    pub tx_wr: ib_ud_wr,
    pub send_wc: [ib_wc; MAX_SEND_CQE],
    pub rx_wr: ib_recv_wr,
    pub rx_sge: [ib_sge; IPOIB_UD_RX_SG],
    pub ibwc: [ib_wc; IPOIB_NUM_WC],
    pub dead_ahs: list_head,
    pub event_handler: ib_event_handler,
    pub parent: *mut net_device,
// 'child_intfs' and 'list' membership of all child devices are
// protected by the netdev instance lock of 'dev'.
//
    pub child_intfs: list_head,
    pub list: list_head,
    pub child_type: c_int,

    pub cm: ipoib_cm_dev_priv,

    pub fs_list: list_head,
    pub mcg_dentry: *mut dentry,
    pub path_dentry: *mut dentry,

    pub hca_caps: u64,
    pub kernel_caps: u64,
    pub ethtool: ipoib_ethtool_st,
    pub max_send_sge: c_uint,
    pub rn_ops: *const net_device_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_ah {
    pub dev: *mut net_device,
    pub ah: *mut ib_ah,
    pub list: list_head,
    pub ref: kref,
    pub last_send: c_uint,
    pub valid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_path {
    pub dev: *mut net_device,
    pub pathrec: sa_path_rec,
    pub ah: *mut ipoib_ah,
    pub queue: sk_buff_head,
    pub neigh_list: list_head,
    pub query_id: c_int,
    pub query: *mut ib_sa_query,
    pub done: completion,
    pub rb_node: rb_node,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_neigh {
    pub ah: *mut ipoib_ah,

    pub cm: *mut ipoib_cm_tx,
    pub daddr: [u8; INFINIBAND_ALEN],
    pub queue: sk_buff_head,
    pub dev: *mut net_device,
    pub list: list_head,
    pub hnext: *mut ipoib_neigh __rcu,
    pub rcu: rcu_head,
    pub refcnt: refcount_t,
    pub alive: c_ulong,
}

extern "C" {
    pub fn ipoib_neigh_dtor(neigh: *mut ipoib_neigh);
}
extern "C" {
    pub fn ipoib_neigh_free(neigh: *mut ipoib_neigh);
}
extern "C" {
    pub fn ipoib_del_neighs_by_gid(dev: *mut net_device, gid: *mut u8);
}
// functions
extern "C" {
    pub fn ipoib_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn ipoib_tx_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn ipoib_ib_rx_completion(cq: *mut ib_cq, ctx_ptr: *mut c_void);
}
extern "C" {
    pub fn ipoib_ib_tx_completion(cq: *mut ib_cq, ctx_ptr: *mut c_void);
}
extern "C" {
    pub fn ipoib_free_ah(kref: *mut kref);
}
extern "C" {
    pub fn ipoib_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_intf_free(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_add_pkey_attr(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_add_umcast_attr(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_reap_ah(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_napi_schedule_work(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_mark_paths_invalid(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_flush_paths(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_ib_dev_flush_light(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_ib_dev_flush_normal(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_ib_dev_flush_heavy(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_ib_tx_timeout_work(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_ib_dev_cleanup(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_ib_dev_open_default(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_ib_dev_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_ib_dev_stop(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_ib_dev_up(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_ib_dev_down(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_ib_dev_stop_default(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_pkey_dev_check_presence(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_mcast_join_task(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_mcast_carrier_on_task(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_mcast_send(dev: *mut net_device, daddr: *mut u8, skb: *mut sk_buff);
}
extern "C" {
    pub fn ipoib_mcast_restart_task(work: *mut work_struct);
}
extern "C" {
    pub fn ipoib_mcast_start_thread(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_mcast_stop_thread(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_mcast_dev_flush(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_dma_map_tx(ca: *mut ib_device, tx_req: *mut ipoib_tx_buf) -> c_int;
}

extern "C" {
    pub fn ipoib_mcast_iter_next(iter: *mut ipoib_mcast_iter) -> c_int;
}
extern "C" {
    pub fn ipoib_path_iter_next(iter: *mut ipoib_path_iter) -> c_int;
}

extern "C" {
    pub fn ipoib_mcast_remove_list(remove_list: *mut list_head);
}
extern "C" {
    pub fn ipoib_init_qp(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_transport_dev_init(dev: *mut net_device, ca: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn ipoib_transport_dev_cleanup(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_vlan_add(pdev: *mut net_device, pkey: c_ushort) -> c_int;
}
extern "C" {
    pub fn ipoib_vlan_delete(pdev: *mut net_device, pkey: c_ushort) -> c_int;
}
extern "C" {
    pub fn ipoib_netlink_init() -> int  __init;
}
extern "C" {
    pub fn ipoib_netlink_fini() -> void __exit;
}
extern "C" {
    pub fn ipoib_set_umcast(ndev: *mut net_device, umcast_val: c_int);
}
extern "C" {
    pub fn ipoib_set_mode(dev: *mut net_device, buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn ipoib_setup_common(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_drain_cq(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_set_ethtool_ops(dev: *mut net_device);
}
pub const IPOIB_FLAGS_RC: c_uint = 0x80;
pub const IPOIB_FLAGS_UC: c_uint = 0x40;
// We don't support UC connections at the moment

extern "C" {
    pub fn test_bit(_arg: IPOIB_FLAG_OPER_UP, _arg: &neigh->cm->flags) -> return;
}
extern "C" {
    pub fn ipoib_cm_send(dev: *mut net_device, skb: *mut sk_buff, tx: *mut ipoib_cm_tx);
}
extern "C" {
    pub fn ipoib_cm_dev_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_cm_dev_stop(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_cm_dev_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_cm_add_mode_attr(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ipoib_cm_dev_cleanup(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_cm_destroy_tx(tx: *mut ipoib_cm_tx);
}
extern "C" {
    pub fn ipoib_cm_handle_rx_wc(dev: *mut net_device, wc: *mut ib_wc);
}
extern "C" {
    pub fn ipoib_cm_handle_tx_wc(dev: *mut net_device, wc: *mut ib_wc);
}

pub const ipoib_max_conn_qp: c_int = 0;

extern "C" {
    pub fn ipoib_create_debug_files(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_delete_debug_files(dev: *mut net_device);
}
extern "C" {
    pub fn ipoib_register_debugfs();
}
extern "C" {
    pub fn ipoib_unregister_debugfs();
}

