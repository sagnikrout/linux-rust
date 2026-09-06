//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_ll2.h
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

// LL2 queues handles will be split as follows:
// first will be legacy queues, and then the ctx based queues.
//

pub const QED_LL2_LEGACY_CONN_BASE_PF: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_rx_packet {
    pub list_entry: list_head,
    pub rxq_bd: *mut core_rx_bd_with_buff_len,
    pub rx_buf_addr: dma_addr_t,
    pub buf_length: u16,
    pub cookie: *mut c_void,
    pub placement_offset: u8,
    pub parse_flags: u16,
    pub packet_length: u16,
    pub vlan: u16,
    pub opaque_data: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_tx_packet {
    pub list_entry: list_head,
    pub bd_used: u16,
    pub notify_fw: bool,
    pub cookie: *mut c_void,
// Flexible Array of bds_set determined by max_bds_per_packet
    pub txq_bd: *mut core_tx_bd,
    pub tx_frag: dma_addr_t,
    pub frag_len: u16,
    pub bds_set: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_rx_queue {
// Lock protecting the Rx queue manipulation
    pub lock: spinlock_t,
    pub rxq_chain: qed_chain,
    pub rcq_chain: qed_chain,
    pub rx_sb_index: u8,
    pub ctx_based: u8,
    pub b_cb_registered: bool,
    pub p_fw_cons: *mut __le16,
    pub active_descq: list_head,
    pub free_descq: list_head,
    pub posting_descq: list_head,
    pub descq_array: *mut qed_ll2_rx_packet,
    pub set_prod_addr: *mut void __iomem,
    pub db_data: core_pwm_prod_update_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_tx_queue {
// Lock protecting the Tx queue manipulation
    pub lock: spinlock_t,
    pub txq_chain: qed_chain,
    pub tx_sb_index: u8,
    pub b_cb_registered: bool,
    pub p_fw_cons: *mut __le16,
    pub active_descq: list_head,
    pub free_descq: list_head,
    pub sending_descq: list_head,
    pub cur_completing_bd_idx: u16,
    pub doorbell_addr: *mut void __iomem,
    pub db_msg: core_db_data,
    pub bds_idx: u16,
    pub cur_send_frag_num: u16,
    pub cur_completing_frag_num: u16,
    pub b_completing_packet: bool,
    pub qed_ll2_tx_packet*/: *mut *mut *mut void descq_mem; / memory for variable sized,
    pub cur_send_packet: *mut qed_ll2_tx_packet,
    pub cur_completing_packet: qed_ll2_tx_packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ll2_info {
// Lock protecting the state of LL2
    pub mutex: mutex,
    pub input: qed_ll2_acquire_data_inputs,
    pub cid: u32,
    pub my_id: u8,
    pub queue_id: u8,
    pub tx_stats_id: u8,
    pub b_active: bool,
    pub tx_dest: core_tx_dest,
    pub tx_stats_en: u8,
    pub main_func_queue: bool,
    pub cbs: qed_ll2_cbs,
    pub rx_queue: qed_ll2_rx_queue,
    pub tx_queue: qed_ll2_tx_queue,
}

//
// qed_ll2_acquire_connection(): Allocate resources,
// starts rx & tx (if relevant) queues pair.
// Provides connecion handler as output
// parameter.
//
// @cxt: Pointer to the hw-function [opaque to some].
// @data: Describes connection parameters.
//
// Return: Int.
//
extern "C" {
    pub fn qed_ll2_acquire_connection(cxt: *mut c_void, data: *mut qed_ll2_acquire_data) -> c_int;
}
//
// qed_ll2_establish_connection(): start previously allocated LL2 queues pair
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
//
// Return: 0 on success, failure otherwise.
//
extern "C" {
    pub fn qed_ll2_establish_connection(cxt: *mut c_void, connection_handle: u8) -> c_int;
}
//
// qed_ll2_post_rx_buffer(): Submit buffers to LL2 Rx queue.
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
// @addr: RX (physical address) buffers to submit.
// @buf_len: Buffer Len.
// @cookie: Cookie.
// @notify_fw: Produce corresponding Rx BD immediately.
//
// Return: 0 on success, failure otherwise.
//
// qed_ll2_prepare_tx_packet(): Request for start Tx BD
// to prepare Tx packet submission to FW.
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: Connection handle.
// @pkt: Info regarding the tx packet.
// @notify_fw: Issue doorbell to fw for this packet.
//
// Return: 0 on success, failure otherwise.
//
// qed_ll2_release_connection(): Releases resources allocated for LL2
// connection.
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
//
// Return: Void.
//
extern "C" {
    pub fn qed_ll2_release_connection(cxt: *mut c_void, connection_handle: u8);
}
//
// qed_ll2_set_fragment_of_tx_packet(): Provides fragments to fill
// Tx BD of BDs requested by
// qed_ll2_prepare_tx_packet
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
// @addr: Address.
// @nbytes: Number of bytes.
//
// Return: 0 on success, failure otherwise.
//
// qed_ll2_terminate_connection(): Stops Tx/Rx queues
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
//
// Return: 0 on success, failure otherwise.
//
extern "C" {
    pub fn qed_ll2_terminate_connection(cxt: *mut c_void, connection_handle: u8) -> c_int;
}
//
// qed_ll2_get_stats(): Get LL2 queue's statistics
//
// @cxt: Pointer to the hw-function [opaque to some].
// @connection_handle: LL2 connection's handle obtained from
// qed_ll2_require_connection.
// @p_stats: Pointer Status.
//
// Return: 0 on success, failure otherwise.
//
// qed_ll2_alloc(): Allocates LL2 connections set.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_ll2_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_ll2_setup(): Inits LL2 connections set.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_ll2_setup(p_hwfn: *mut qed_hwfn);
}
//
// qed_ll2_free(): Releases LL2 connections set
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_ll2_free(p_hwfn: *mut qed_hwfn);
}
