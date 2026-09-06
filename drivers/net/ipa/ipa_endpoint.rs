//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_endpoint.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2024 Linaro Ltd.
//

// Non-zero granularity of counter used to implement aggregation timeout

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_endpoint_name {
    IPA_ENDPOINT_AP_COMMAND_TX,
    IPA_ENDPOINT_AP_LAN_RX,
    IPA_ENDPOINT_AP_MODEM_TX,
    IPA_ENDPOINT_AP_MODEM_RX,
    IPA_ENDPOINT_MODEM_COMMAND_TX,
    IPA_ENDPOINT_MODEM_LAN_TX,
    IPA_ENDPOINT_MODEM_LAN_RX,
    IPA_ENDPOINT_MODEM_AP_TX,
    IPA_ENDPOINT_MODEM_AP_RX,
    IPA_ENDPOINT_MODEM_DL_NLO_TX,
    IPA_ENDPOINT_COUNT,	/* Number of names (not an index) */
}

//
// struct ipa_endpoint_tx - Endpoint configuration for TX endpoints
// @seq_type:		primary packet processing sequencer type
// @seq_rep_type:	sequencer type for replication processing
// @status_endpoint:	endpoint to which status elements are sent
//
// The @status_endpoint is only valid if the endpoint's @status_enable
// flag is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_endpoint_tx {
    pub seq_type: ipa_seq_type,
    pub seq_rep_type: ipa_seq_rep_type,
    pub status_endpoint: ipa_endpoint_name,
}

//
// struct ipa_endpoint_rx - Endpoint configuration for RX endpoints
// @buffer_size:	requested receive buffer size (bytes)
// @pad_align:		power-of-2 boundary to which packet payload is aligned
// @aggr_time_limit:	time before aggregation closes (microseconds)
// @aggr_hard_limit:	whether aggregation closes before or after boundary
// @aggr_close_eof:	whether aggregation closes on end-of-frame
// @holb_drop:		whether to drop packets to avoid head-of-line blocking
//
// The actual size of the receive buffer is rounded up if necessary
// to be a power-of-2 number of pages.
//
// With each packet it transfers, the IPA hardware can perform certain
// transformations of its packet data.  One of these is adding pad bytes
// to the end of the packet data so the result ends on a power-of-2 boundary.
//
// It is also able to aggregate multiple packets into a single receive buffer.
// Aggregation is "open" while a buffer is being filled, and "closes" when
// certain criteria are met.
//
// A time limit can be specified to close aggregation.  Aggregation will be
// closed if this period passes after data is first written into a receive
// buffer.  If not specified, no time limit is imposed.
//
// Insufficient space available in the receive buffer can close aggregation.
// The aggregation byte limit defines the point (in units of 1024 bytes) in
// the buffer where aggregation closes.  With a "soft" aggregation limit,
// aggregation closes when a packet written to the buffer *crosses* that
// aggregation limit.  With a "hard" aggregation limit, aggregation will
// close *before* writing a packet that would cross that boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_endpoint_rx {
    pub buffer_size: u32,
    pub pad_align: u32,
    pub aggr_time_limit: u32,
    pub aggr_hard_limit: bool,
    pub aggr_close_eof: bool,
    pub holb_drop: bool,
}

//
// struct ipa_endpoint_config - IPA endpoint hardware configuration
// @resource_group:	resource group to assign endpoint to
// @checksum:		whether checksum offload is enabled
// @qmap:		whether endpoint uses QMAP protocol
// @aggregation:	whether endpoint supports aggregation
// @status_enable:	whether endpoint uses status elements
// @dma_mode:		whether endpoint operates in DMA mode
// @dma_endpoint:	peer endpoint, if operating in DMA mode
// @tx:			TX-specific endpoint information (see above)
// @rx:			RX-specific endpoint information (see above)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_endpoint_config {
    pub resource_group: u32,
    pub checksum: bool,
    pub qmap: bool,
    pub aggregation: bool,
    pub status_enable: bool,
    pub dma_mode: bool,
    pub dma_endpoint: ipa_endpoint_name,
    pub tx: ipa_endpoint_tx,
    pub rx: ipa_endpoint_rx,
}

//
// enum ipa_replenish_flag:	RX buffer replenish flags
//
// @IPA_REPLENISH_ENABLED:	Whether receive buffer replenishing is enabled
// @IPA_REPLENISH_ACTIVE:	Whether replenishing is underway
// @IPA_REPLENISH_COUNT:	Number of defined replenish flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_replenish_flag {
    IPA_REPLENISH_ENABLED,
    IPA_REPLENISH_ACTIVE,
    IPA_REPLENISH_COUNT,	/* Number of flags (must be last) */
}

//
// struct ipa_endpoint - IPA endpoint information
// @ipa:		IPA pointer
// @ee_id:		Execution environmnent endpoint is associated with
// @channel_id:		GSI channel used by the endpoint
// @endpoint_id:	IPA endpoint number
// @toward_ipa:		Endpoint direction (true = TX, false = RX)
// @config:		Default endpoint configuration
// @skb_frag_max:	Maximum allowed number of TX SKB fragments
// @evt_ring_id:	GSI event ring used by the endpoint
// @netdev:		Network device pointer, if endpoint uses one
// @replenish_flags:	Replenishing state flags
// @replenish_count:	Total number of replenish transactions committed
// @replenish_work:	Work item used for repeated replenish failures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_endpoint {
    pub ipa: *mut ipa,
    pub ee_id: gsi_ee_id,
    pub channel_id: u32,
    pub endpoint_id: u32,
    pub toward_ipa: bool,
    pub config: ipa_endpoint_config,
    pub /: *mut *mut u32 skb_frag_max; / Used for netdev TX only,
    pub evt_ring_id: u32,
// Net device this endpoint is associated with, if any
    pub netdev: *mut net_device,
// Receive buffer replenishing for RX endpoints
    pub IPA_REPLENISH_COUNT): DECLARE_BITMAP(replenish_flags,,
    pub replenish_count: u64,
    pub /: *mut *mut delayed_work replenish_work; / global wq,
}

extern "C" {
    pub fn ipa_endpoint_modem_hol_block_clear_all(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_modem_pause_all(ipa: *mut ipa, enable: bool);
}
extern "C" {
    pub fn ipa_endpoint_modem_exception_reset_all(ipa: *mut ipa) -> c_int;
}
extern "C" {
    pub fn ipa_endpoint_skb_tx(endpoint: *mut ipa_endpoint, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ipa_endpoint_enable_one(endpoint: *mut ipa_endpoint) -> c_int;
}
extern "C" {
    pub fn ipa_endpoint_disable_one(endpoint: *mut ipa_endpoint);
}
extern "C" {
    pub fn ipa_endpoint_suspend_one(endpoint: *mut ipa_endpoint);
}
extern "C" {
    pub fn ipa_endpoint_resume_one(endpoint: *mut ipa_endpoint);
}
extern "C" {
    pub fn ipa_endpoint_suspend(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_resume(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_setup(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_teardown(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_config(ipa: *mut ipa) -> c_int;
}
extern "C" {
    pub fn ipa_endpoint_deconfig(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_default_route_set(ipa: *mut ipa, endpoint_id: u32);
}
extern "C" {
    pub fn ipa_endpoint_default_route_clear(ipa: *mut ipa);
}
extern "C" {
    pub fn ipa_endpoint_exit(ipa: *mut ipa);
}
