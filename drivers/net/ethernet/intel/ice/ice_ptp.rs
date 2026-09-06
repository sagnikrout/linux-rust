//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_ptp.h
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
// Copyright (C) 2021, Intel Corporation.

// The ice hardware captures Tx hardware timestamps in the PHY. The timestamp
// is stored in a buffer of registers. Depending on the specific hardware,
// this buffer might be shared across multiple PHY ports.
//
// On transmit of a packet to be timestamped, software is responsible for
// selecting an open index. Hardware makes no attempt to lock or prevent
// re-use of an index for multiple packets.
//
// To handle this, timestamp indexes must be tracked by software to ensure
// that an index is not re-used for multiple transmitted packets. The
// structures and functions declared in this file track the available Tx
// register indexes, as well as provide storage for the SKB pointers.
//
// To allow multiple ports to access the shared register block independently,
// the blocks are split up so that indexes are assigned to each port based on
// hardware logical port number.
//
// The timestamp blocks are handled differently for E810- and E822-based
// devices. In E810 devices, each port has its own block of timestamps, while in
// E822 there is a need to logically break the block of registers into smaller
// chunks based on the port number to avoid collisions.
//
// Example for port 5 in E810:
// +--------+--------+--------+--------+--------+--------+--------+--------+
// |register|register|register|register|register|register|register|register|
// | block  | block  | block  | block  | block  | block  | block  | block  |
// |  for   |  for   |  for   |  for   |  for   |  for   |  for   |  for   |
// | port 0 | port 1 | port 2 | port 3 | port 4 | port 5 | port 6 | port 7 |
// +--------+--------+--------+--------+--------+--------+--------+--------+
// ^^
// ||
// |---  quad offset is always 0
// ---- quad number
//
// Example for port 5 in E822:
// +-----------------------------+-----------------------------+
// |  register block for quad 0  |  register block for quad 1  |
// |+------+------+------+------+|+------+------+------+------+|
// ||port 0|port 1|port 2|port 3|||port 0|port 1|port 2|port 3||
// |+------+------+------+------+|+------+------+------+------+|
// +-----------------------------+-------^---------------------+
// ^      |
// |      --- quad offset
// ---- quad number
//
// * PHY port 5 is port 1 in quad 1
//
// struct ice_tx_tstamp - Tracking for a single Tx timestamp
// @skb: pointer to the SKB for this timestamp request
// @start: jiffies when the timestamp was first requested
// @cached_tstamp: last read timestamp
//
// This structure tracks a single timestamp request. The SKB pointer is
// provided when initiating a request. The start time is used to ensure that
// we discard old requests that were not fulfilled within a 2 second time
// window.
// Timestamp values in the PHY are read only and do not get cleared except at
// hardware reset or when a new timestamp value is captured.
//
// Some PHY types do not provide a "ready" bitmap indicating which timestamp
// indexes are valid. In these cases, we use a cached_tstamp to keep track of
// the last timestamp we read for a given index. If the current timestamp
// value is the same as the cached value, we assume a new timestamp hasn't
// been captured. This avoids reporting stale timestamps to the stack. This is
// only done if the has_ready_bitmap flag is not set in ice_ptp_tx structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_tstamp {
    pub skb: *mut sk_buff,
    pub start: c_ulong,
    pub cached_tstamp: u64,
}

//
// enum ice_tx_tstamp_work - Status of Tx timestamp work function
// @ICE_TX_TSTAMP_WORK_DONE: Tx timestamp processing is complete
// @ICE_TX_TSTAMP_WORK_PENDING: More Tx timestamps are pending
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_tstamp_work {
    ICE_TX_TSTAMP_WORK_DONE = 0,
    ICE_TX_TSTAMP_WORK_PENDING,
}

//
// struct ice_ptp_tx - Tracking structure for all Tx timestamp requests on a port
// @lock: lock to prevent concurrent access to fields of this struct
// @tstamps: array of len to store outstanding requests
// @in_use: bitmap of len to indicate which slots are in use
// @stale: bitmap of len to indicate slots which have stale timestamps
// @block: which memory block (quad or port) the timestamps are captured in
// @offset: offset into timestamp block to get the real index
// @len: length of the tstamps and in_use fields.
// @init: if true, the tracker is initialized;
// @calibrating: if true, the PHY is calibrating the Tx offset. During this
// window, timestamps are temporarily disabled.
// @has_ready_bitmap: if true, the hardware has a valid Tx timestamp ready
// bitmap register. If false, fall back to verifying new
// timestamp values against previously cached copy.
// @last_ll_ts_idx_read: index of the last LL TS read by the FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptp_tx {
    pub /: *mut *mut spinlock_t lock; / lock protecting in_use bitmap,
    pub tstamps: *mut ice_tx_tstamp,
    pub in_use: *mut c_ulong,
    pub stale: *mut c_ulong,
    pub block: u8,
    pub offset: u8,
    pub len: u8,
    pub 1: u8 init :,
    pub 1: u8 calibrating :,
    pub 1: u8 has_ready_bitmap :,
    pub last_ll_ts_idx_read: i8,
}

// Quad and port information for initializing timestamp blocks
pub const INDEX_PER_QUAD: c_int = 64;
pub const INDEX_PER_PORT_E82X: c_int = 16;
pub const INDEX_PER_PORT: c_int = 64;
//
// struct ice_ptp_port - data used to initialize an external port for PTP
//
// This structure contains data indicating whether a single external port is
// ready for PTP functionality. It is used to track the port initialization
// and determine when the port's PHY offset is valid.
//
// @list_node: list member structure
// @tx: Tx timestamp tracking for this port
// @ov_work: delayed work task for tracking when PHY offset is valid
// @ps_lock: mutex used to protect the overall PTP PHY start procedure
// @link_up: indicates whether the link is up
// @tx_fifo_busy_cnt: number of times the Tx FIFO was busy
// @port_num: the port number this structure represents
// @tx_clk: currently active Tx reference clock source
// @tx_clk_req: requested Tx reference clock source (new target)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptp_port {
    pub list_node: list_head,
    pub tx: ice_ptp_tx,
    pub ov_work: kthread_delayed_work,
    pub /: *mut *mut mutex ps_lock; / protects overall PTP PHY start procedure,
    pub link_up: bool,
    pub tx_fifo_busy_cnt: u8,
    pub port_num: u8,
    pub tx_clk: ice_e825c_ref_clk,
    pub tx_clk_req: ice_e825c_ref_clk,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_tx_interrupt {
    ICE_PTP_TX_INTERRUPT_NONE = 0,
    ICE_PTP_TX_INTERRUPT_SELF,
    ICE_PTP_TX_INTERRUPT_ALL,
}

pub const GLTSYN_TGT_H_IDX_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_state {
    ICE_PTP_UNINIT = 0,
    ICE_PTP_INITIALIZING,
    ICE_PTP_READY,
    ICE_PTP_RESETTING,
    ICE_PTP_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_pin {
    SDP0 = 0,
    SDP1,
    SDP2,
    SDP3,
    TIME_SYNC,
    ONE_PPS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ptp_pin_nvm {
    GNSS = 0,
    SMA1,
    UFL1,
    SMA2,
    UFL2,
    NUM_PTP_PINS_NVM,
    GPIO_NA = 9
}

// Per-channel register definitions

pub const GLTSYN_EVNT_H_IDX_MAX: c_int = 3;
// Pin definitions for PTP
pub const ICE_N_PINS_MAX: c_int = 6;
//
// struct ice_ptp_pin_desc - hardware pin description data
// @name_idx: index of the name of pin in ice_pin_names
// @gpio: the associated GPIO input and output pins
// @delay: input and output signal delays in nanoseconds
//
// Structure describing a PTP-capable GPIO pin that extends ptp_pin_desc array
// for the device. Device families have separate sets of available pins with
// varying restrictions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptp_pin_desc {
    pub name_idx: c_int,
    pub gpio: [c_int; 2],
    pub delay: [c_uint; 2],
}

//
// struct ice_ptp - data used for integrating with CONFIG_PTP_1588_CLOCK
// @state: current state of PTP state machine
// @tx_interrupt_mode: the TX interrupt mode for the PTP clock
// @port: data for the PHY port initialization procedure
// @work: delayed work function for periodic tasks
// @cached_phc_time: a cached copy of the PHC time for timestamp extension
// @cached_phc_jiffies: jiffies when cached_phc_time was last updated
// @kworker: kwork thread for handling periodic work
// @ext_ts_irq: the external timestamp IRQ in use
// @pin_desc: structure defining pins
// @ice_pin_desc: internal structure describing pin relations
// @perout_rqs: cached periodic output requests
// @extts_rqs: cached external timestamp requests
// @info: structure defining PTP hardware capabilities
// @clock: pointer to registered PTP clock device
// @tstamp_config: hardware timestamping configuration
// @tx_refclks: bitmaps table to store the information about TX reference clocks
// @reset_time: kernel time after clock stop on reset
// @tx_hwtstamp_good: number of completed Tx timestamp requests
// @tx_hwtstamp_skipped: number of Tx time stamp requests skipped
// @tx_hwtstamp_timeouts: number of Tx skbs discarded with no time stamp
// @tx_hwtstamp_flushed: number of Tx skbs flushed due to interface closed
// @tx_hwtstamp_discarded: number of Tx skbs discarded due to cached PHC time
// being too old to correctly extend timestamp
// @late_cached_phc_updates: number of times cached PHC update is late
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptp {
    pub state: ice_ptp_state,
    pub tx_interrupt_mode: ice_ptp_tx_interrupt,
    pub port: ice_ptp_port,
    pub work: kthread_delayed_work,
    pub cached_phc_time: u64,
    pub cached_phc_jiffies: c_ulong,
    pub kworker: *mut kthread_worker,
    pub ext_ts_irq: u8,
    pub pin_desc: [ptp_pin_desc; ICE_N_PINS_MAX],
    pub ice_pin_desc: *const ice_ptp_pin_desc,
    pub perout_rqs: [ptp_perout_request; GLTSYN_TGT_H_IDX_MAX],
    pub extts_rqs: [ptp_extts_request; GLTSYN_EVNT_H_IDX_MAX],
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub tstamp_config: kernel_hwtstamp_config,
    pub tx_refclks: [c_ulong; ICE_E825_MAX_PHYS][ICE_REF_CLK_MAX],
    pub reset_time: u64,
    pub tx_hwtstamp_good: u64,
    pub tx_hwtstamp_skipped: u32,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_flushed: u32,
    pub tx_hwtstamp_discarded: u32,
    pub late_cached_phc_updates: u32,
}

pub const PFTSYN_SEM_BYTES: c_int = 4;

pub const TS_CMD_MASK: c_uint = 0xF;
pub const SYNC_EXEC_CMD: c_uint = 0x3;

pub const FIFO_OK: c_uint = 0xFF;
pub const ICE_PTP_FIFO_NUM_CHECKS: c_int = 5;

extern "C" {
    pub fn ice_ptp_clock_index(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_ptp_restore_timestamp_mode(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_ptp_extts_event(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_ptp_request_ts(tx: *mut ice_ptp_tx, skb: *mut sk_buff) -> i8;
}
extern "C" {
    pub fn ice_ptp_req_tx_single_tstamp(tx: *mut ice_ptp_tx, idx: u8);
}
extern "C" {
    pub fn ice_ptp_complete_tx_single_tstamp(tx: *mut ice_ptp_tx);
}
extern "C" {
    pub fn ice_ptp_process_ts(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_ptp_ts_irq(pf: *mut ice_pf) -> irqreturn_t;
}
extern "C" {
    pub fn ice_ptp_tx_tstamps_pending(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_ptp_rebuild(pf: *mut ice_pf, reset_type: ice_reset_req);
}
extern "C" {
    pub fn ice_ptp_init(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_ptp_release(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_ptp_link_change(pf: *mut ice_pf, linkup: bool);
}
extern "C" {
    pub fn ice_ptp_queue_work(pf: *mut ice_pf);
}

