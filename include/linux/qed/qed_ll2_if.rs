//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_ll2_if.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_conn_type {
    QED_LL2_TYPE_FCOE,
    QED_LL2_TYPE_TCP_ULP,
    QED_LL2_TYPE_TEST,
    QED_LL2_TYPE_OOO,
    QED_LL2_TYPE_RESERVED2,
    QED_LL2_TYPE_ROCE,
    QED_LL2_TYPE_IWARP,
    QED_LL2_TYPE_RESERVED3,
    MAX_QED_LL2_CONN_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_rx_conn_type {
    QED_LL2_RX_TYPE_LEGACY,
    QED_LL2_RX_TYPE_CTX,
    MAX_QED_LL2_RX_CONN_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_roce_flavor_type {
    QED_LL2_ROCE,
    QED_LL2_RROCE,
    MAX_QED_LL2_ROCE_FLAVOR_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_tx_dest {
    QED_LL2_TX_DEST_NW, /* Light L2 TX Destination to the Network */
    QED_LL2_TX_DEST_LB, /* Light L2 TX Destination to the Loopback */
    QED_LL2_TX_DEST_DROP, /* Light L2 Drop the TX packet */
    QED_LL2_TX_DEST_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_error_handle {
    QED_LL2_DROP_PACKET,
    QED_LL2_DO_NOTHING,
    QED_LL2_ASSERT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_stats {
    pub gsi_invalid_hdr: u64,
    pub gsi_invalid_pkt_length: u64,
    pub gsi_unsupported_pkt_typ: u64,
    pub gsi_crcchksm_error: u64,
    pub packet_too_big_discard: u64,
    pub no_buff_discard: u64,
    pub rcv_ucast_bytes: u64,
    pub rcv_mcast_bytes: u64,
    pub rcv_bcast_bytes: u64,
    pub rcv_ucast_pkts: u64,
    pub rcv_mcast_pkts: u64,
    pub rcv_bcast_pkts: u64,
    pub sent_ucast_bytes: u64,
    pub sent_mcast_bytes: u64,
    pub sent_bcast_bytes: u64,
    pub sent_ucast_pkts: u64,
    pub sent_mcast_pkts: u64,
    pub sent_bcast_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_comp_rx_data {
    pub cookie: *mut c_void,
    pub rx_buf_addr: dma_addr_t,
    pub parse_flags: u16,
    pub err_flags: u16,
    pub vlan: u16,
    pub b_last_packet: bool,
    pub connection_handle: u8,
    pub packet_length: u16,
    pub data_length: u16,
    pub length: },
    pub opaque_data_0: u32,
    pub opaque_data_1: u32,
// GSI only
    pub src_qp: u32,
    pub qp_id: u16,
    pub placement_offset: u8,
    pub data_length_error: u8,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_cbs {
    pub rx_comp_cb: qed_ll2_complete_rx_packet_cb,
    pub rx_release_cb: qed_ll2_release_rx_packet_cb,
    pub tx_comp_cb: qed_ll2_complete_tx_packet_cb,
    pub tx_release_cb: qed_ll2_release_tx_packet_cb,
    pub slowpath_cb: qed_ll2_slowpath_cb,
    pub cookie: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_acquire_data_inputs {
    pub rx_conn_type: qed_ll2_rx_conn_type,
    pub conn_type: qed_ll2_conn_type,
    pub mtu: u16,
    pub rx_num_desc: u16,
    pub rx_num_ooo_buffers: u16,
    pub rx_drop_ttl0_flg: u8,
    pub rx_vlan_removal_en: u8,
    pub tx_num_desc: u16,
    pub tx_max_bds_per_packet: u8,
    pub tx_tc: u8,
    pub tx_dest: qed_ll2_tx_dest,
    pub ai_err_packet_too_big: qed_ll2_error_handle,
    pub ai_err_no_buf: qed_ll2_error_handle,
    pub secondary_queue: bool,
    pub gsi_enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_acquire_data {
    pub input: qed_ll2_acquire_data_inputs,
    pub cbs: *const qed_ll2_cbs,
// Output container for LL2 connection's handle
    pub p_connection_handle: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_tx_pkt_info {
    pub cookie: *mut c_void,
    pub first_frag: dma_addr_t,
    pub tx_dest: qed_ll2_tx_dest,
    pub qed_roce_flavor: qed_ll2_roce_flavor_type,
    pub vlan: u16,
    pub /: *mut *mut u16 l4_hdr_offset_w; / from start of packet,
    pub first_frag_len: u16,
    pub num_of_bds: u8,
    pub bd_flags: u8,
    pub enable_ip_cksum: bool,
    pub enable_l4_cksum: bool,
    pub calc_ip_len: bool,
    pub remove_stag: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_cb_ops {
    pub u32): *mut *mut *mut *mut int (rx_cb)(void , struct sk_buff , u32,,
    pub bool): *mut *mut *mut *mut int (tx_cb)(void , struct sk_buff ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_params {
    pub mtu: u16,
    pub drop_ttl0_packets: bool,
    pub rx_vlan_stripping: bool,
    pub tx_tc: u8,
    pub frags_mapped: bool,
    pub ll2_mac_address: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ll2_xmit_flags {
// FIP discovery packet
    QED_LL2_XMIT_FLAGS_FIP_DISCOVERY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_ops {
//
// start(): Initializes ll2.
//
// @cdev: Qed dev pointer.
// @params: Protocol driver configuration for the ll2.
//
// Return: 0 on success, otherwise error value.
//
    pub params): *mut *mut *mut int (start)(struct qed_dev cdev, struct qed_ll2_params,
//
// stop(): Stops the ll2
//
// @cdev: Qed dev pointer.
//
// Return: 0 on success, otherwise error value.
//
    pub cdev): *mut *mut int (stop)(struct qed_dev,
//
// start_xmit(): Transmits an skb over the ll2 interface
//
// @cdev: Qed dev pointer.
// @skb: SKB.
// @xmit_flags: Transmit options defined by the enum qed_ll2_xmit_flags.
//
// Return: 0 on success, otherwise error value.
//
    pub xmit_flags): c_ulong,
//
// register_cb_ops(): Protocol driver register the callback for Rx/Tx
// packets. Should be called before `start'.
//
// @cdev: Qed dev pointer.
// @cookie: to be passed to the callback functions.
// @ops: the callback functions to register for Rx / Tx.
//
// Return: 0 on success, otherwise error value.
//
    pub cookie): *mut c_void,
//
// get_stats(): Get LL2 related statistics.
//
// @cdev: Qed dev pointer.
// @stats: Pointer to struct that would be filled with stats.
//
// Return: 0 on success, error otherwise.
//
    pub stats): *mut *mut *mut int (get_stats)(struct qed_dev cdev, struct qed_ll2_stats,
}

extern "C" {
    pub fn qed_ll2_alloc_if(: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_ll2_dealloc_if(: *mut qed_dev);
}

