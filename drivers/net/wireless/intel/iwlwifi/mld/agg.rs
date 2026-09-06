//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/agg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024 Intel Corporation
//

// Macro flag: #define __iwl_agg_h__

//
// struct iwl_mld_reorder_buffer - per ra/tid/queue reorder buffer
// @head_sn: reorder window head sequence number
// @num_stored: number of MPDUs stored in the buffer
// @queue: queue of this reorder buffer
// @valid: true if reordering is valid for this queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_reorder_buffer {
    pub head_sn: u16,
    pub num_stored: u16,
    pub queue: c_int,
    pub valid: bool,
    pub ____cacheline_aligned_in_smp: },
//
// struct iwl_mld_reorder_buf_entry - reorder buffer entry per-queue/per-seqno
// @frames: list of skbs stored. a list is necessary because in an A-MSDU,
// all sub-frames share the same sequence number, so they are stored
// together in the same list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_reorder_buf_entry {
    pub frames: sk_buff_head,
// sparse doesn't like this construct: "bad integer constant expression"

//
// struct iwl_mld_baid_data - Block Ack session data
// @rcu_head: RCU head for freeing this data
// @sta_mask: station mask for the BAID
// @tid: tid of the session
// @baid: baid of the session
// @buf_size: the reorder buffer size as set by the last ADDBA request
// @entries_per_queue: number of buffers per queue, this actually gets
// aligned up to avoid cache line sharing between queues
// @timeout: the timeout value specified in the ADDBA request.
// @last_rx_timestamp: timestamp of the last received packet (in jiffies). This
// value is updated only when the configured @timeout has passed since
// the last update to minimize cache bouncing between RX queues.
// @session_timer: timer is set to expire after 2 * @timeout (since we want
// to minimize the cache bouncing by updating @last_rx_timestamp only once
// after @timeout has passed). If no packets are received within this
// period, it informs mac80211 to initiate delBA flow, terminating the
// BA session.
// @rcu_ptr: BA data RCU protected access
// @mld: mld pointer, needed for timer context
// @reorder_buf: reorder buffer, allocated per queue
// @entries: data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_baid_data {
    pub rcu_head: rcu_head,
    pub sta_mask: u32,
    pub tid: u8,
    pub baid: u8,
    pub buf_size: u16,
    pub entries_per_queue: u16,
    pub timeout: u16,
    pub session_timer: timer_list,
    pub last_rx_timestamp: c_ulong,
    pub rcu_ptr: *mut iwl_mld_baid_data __rcu,
    pub mld: *mut iwl_mld,
    pub reorder_buf: [iwl_mld_reorder_buffer; IWL_MAX_RX_HW_QUEUES],
    pub ____cacheline_aligned_in_smp: iwl_mld_reorder_buf_entry entries[],
}

//
// struct iwl_mld_delba_data - RX queue sync data for %IWL_MLD_RXQ_NOTIF_DEL_BA
//
// @baid: Block Ack id, used to identify the BA session to be removed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_delba_data {
    pub baid: u32,
    pub __packed: },
//
// enum iwl_mld_reorder_result - Possible return values for iwl_mld_reorder()
// indicating how the caller should handle the skb based on the result.
//
// @IWL_MLD_PASS_SKB: skb should be passed to upper layer.
// @IWL_MLD_BUFFERED_SKB: skb has been buffered, don't pass it to upper layer.
// @IWL_MLD_DROP_SKB: skb should be dropped and freed by the caller.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_reorder_result {
    IWL_MLD_PASS_SKB,
    IWL_MLD_BUFFERED_SKB,
    IWL_MLD_DROP_SKB
}

    pub timeout): int tid, u16 ssn, u16 buf_size, u16,
    pub tid): c_int,
    pub desc): *mut *mut sk_buff skb, iwl_rx_mpdu_desc,
    pub queue): *mut *mut iwl_rx_packet pkt, int,
    pub queue): c_int,
    pub data): *mut iwl_mld_delba_data,
    pub new_sta_mask): u32,
