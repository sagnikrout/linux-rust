//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_ptp.h
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
// Copyright (C) 2024 Intel Corporation

//
// struct idpf_ptp_cmd - PTP command masks
// @exec_cmd_mask: mask to trigger command execution
// @shtime_enable_mask: mask to enable shadow time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_cmd {
    pub exec_cmd_mask: u32,
    pub shtime_enable_mask: u32,
}

// struct idpf_ptp_dev_clk_regs - PTP device registers
// @dev_clk_ns_l: low part of the device clock register
// @dev_clk_ns_h: high part of the device clock register
// @phy_clk_ns_l: low part of the PHY clock register
// @phy_clk_ns_h: high part of the PHY clock register
// @sys_time_ns_l: low part of the system time register
// @sys_time_ns_h: high part of the system time register
// @incval_l: low part of the increment value register
// @incval_h: high part of the increment value register
// @shadj_l: low part of the shadow adjust register
// @shadj_h: high part of the shadow adjust register
// @phy_incval_l: low part of the PHY increment value register
// @phy_incval_h: high part of the PHY increment value register
// @phy_shadj_l: low part of the PHY shadow adjust register
// @phy_shadj_h: high part of the PHY shadow adjust register
// @cmd: PTP command register
// @phy_cmd: PHY command register
// @cmd_sync: PTP command synchronization register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_dev_clk_regs {
// Main clock
    pub dev_clk_ns_l: *mut void __iomem,
    pub dev_clk_ns_h: *mut void __iomem,
// PHY timer
    pub phy_clk_ns_l: *mut void __iomem,
    pub phy_clk_ns_h: *mut void __iomem,
// System time
    pub sys_time_ns_l: *mut void __iomem,
    pub sys_time_ns_h: *mut void __iomem,
// Main timer adjustments
    pub incval_l: *mut void __iomem,
    pub incval_h: *mut void __iomem,
    pub shadj_l: *mut void __iomem,
    pub shadj_h: *mut void __iomem,
// PHY timer adjustments
    pub phy_incval_l: *mut void __iomem,
    pub phy_incval_h: *mut void __iomem,
    pub phy_shadj_l: *mut void __iomem,
    pub phy_shadj_h: *mut void __iomem,
// Command
    pub cmd: *mut void __iomem,
    pub phy_cmd: *mut void __iomem,
    pub cmd_sync: *mut void __iomem,
}

//
// enum idpf_ptp_access - the type of access to PTP operations
// @IDPF_PTP_NONE: no access
// @IDPF_PTP_DIRECT: direct access through BAR registers
// @IDPF_PTP_MAILBOX: access through mailbox messages
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_ptp_access {
    IDPF_PTP_NONE = 0,
    IDPF_PTP_DIRECT,
    IDPF_PTP_MAILBOX,
}

//
// struct idpf_ptp_secondary_mbx - PTP secondary mailbox
// @peer_mbx_q_id: PTP mailbox queue ID
// @peer_id: Peer ID for PTP Device Control daemon
// @valid: indicates whether secondary mailblox is supported by the Control
// Plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_secondary_mbx {
    pub peer_mbx_q_id: u16,
    pub peer_id: u16,
    pub valid:1: bool,
}

//
// enum idpf_ptp_tx_tstamp_state - Tx timestamp states
// @IDPF_PTP_FREE: Tx timestamp index free to use
// @IDPF_PTP_REQUEST: Tx timestamp index set to the Tx descriptor
// @IDPF_PTP_READ_VALUE: Tx timestamp value ready to be read
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_ptp_tx_tstamp_state {
    IDPF_PTP_FREE,
    IDPF_PTP_REQUEST,
    IDPF_PTP_READ_VALUE,
}

//
// struct idpf_ptp_tx_tstamp_status - Parameters to track Tx timestamp
// @skb: the pointer to the SKB that received the completion tag
// @state: the state of the Tx timestamp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_tx_tstamp_status {
    pub skb: *mut sk_buff,
    pub state: idpf_ptp_tx_tstamp_state,
}

//
// struct idpf_ptp_tx_tstamp - Parameters for Tx timestamping
// @list_member: the list member structure
// @tx_latch_reg_offset_l: Tx tstamp latch low register offset
// @tx_latch_reg_offset_h: Tx tstamp latch high register offset
// @skb: the pointer to the SKB for this timestamp request
// @tstamp: the Tx tstamp value
// @idx: the index of the Tx tstamp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_tx_tstamp {
    pub list_member: list_head,
    pub tx_latch_reg_offset_l: u32,
    pub tx_latch_reg_offset_h: u32,
    pub skb: *mut sk_buff,
    pub tstamp: u64,
    pub idx: u32,
}

//
// struct idpf_ptp_vport_tx_tstamp_caps - Tx timestamp capabilities
// @vport_id: the vport id
// @num_entries: the number of negotiated Tx timestamp entries
// @tstamp_ns_lo_bit: first bit for nanosecond part of the timestamp
// @latches_lock: the lock to the lists of free/used timestamp indexes
// @status_lock: the lock to the status tracker
// @access: indicates an access to Tx timestamp
// @latches_free: the list of the free Tx timestamps latches
// @latches_in_use: the list of the used Tx timestamps latches
// @tx_tstamp_status: Tx tstamp status tracker
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_vport_tx_tstamp_caps {
    pub vport_id: u32,
    pub num_entries: u16,
    pub tstamp_ns_lo_bit: u16,
    pub latches_lock: spinlock_t,
    pub status_lock: spinlock_t,
    pub access:1: bool,
    pub latches_free: list_head,
    pub latches_in_use: list_head,
    pub tx_tstamp_status: [idpf_ptp_tx_tstamp_status; ],
}

//
// struct idpf_ptp - PTP parameters
// @info: structure defining PTP hardware capabilities
// @clock: pointer to registered PTP clock device
// @adapter: back pointer to the adapter
// @base_incval: base increment value of the PTP clock
// @max_adj: maximum adjustment of the PTP clock
// @cmd: HW specific command masks
// @cached_phc_time: a cached copy of the PHC time for timestamp extension
// @cached_phc_jiffies: jiffies when cached_phc_time was last updated
// @dev_clk_regs: the set of registers to access the device clock
// @caps: PTP capabilities negotiated with the Control Plane
// @get_dev_clk_time_access: access type for getting the device clock time
// @get_cross_tstamp_access: access type for the cross timestamping
// @set_dev_clk_time_access: access type for setting the device clock time
// @adj_dev_clk_time_access: access type for the adjusting the device clock
// @tx_tstamp_access: access type for the Tx timestamp value read
// @rsv: reserved bits
// @secondary_mbx: parameters for using dedicated PTP mailbox
// @read_dev_clk_lock: spinlock protecting access to the device clock read
// operation executed by the HW latch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp {
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub adapter: *mut idpf_adapter,
    pub base_incval: u64,
    pub max_adj: u64,
    pub cmd: idpf_ptp_cmd,
    pub cached_phc_time: u64,
    pub cached_phc_jiffies: c_ulong,
    pub dev_clk_regs: idpf_ptp_dev_clk_regs,
    pub caps: u32,
    pub get_dev_clk_time_access:2: idpf_ptp_access,
    pub get_cross_tstamp_access:2: idpf_ptp_access,
    pub set_dev_clk_time_access:2: idpf_ptp_access,
    pub adj_dev_clk_time_access:2: idpf_ptp_access,
    pub tx_tstamp_access:2: idpf_ptp_access,
    pub rsv: u8,
    pub secondary_mbx: idpf_ptp_secondary_mbx,
    pub read_dev_clk_lock: spinlock_t,
}

//
// idpf_ptp_info_to_adapter - get driver adapter struct from ptp_clock_info
// @info: pointer to ptp_clock_info struct
//
// Return: pointer to the corresponding adapter struct
//
// struct idpf_ptp_dev_timers - System time and device time values
// @sys_time_ns: system time value expressed in nanoseconds
// @dev_clk_time_ns: device clock time value expressed in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_ptp_dev_timers {
    pub sys_time_ns: u64,
    pub dev_clk_time_ns: u64,
}

//
// idpf_ptp_is_vport_tx_tstamp_ena - Verify the Tx timestamping enablement for
// a given vport.
// @vport: Virtual port structure
//
// Tx timestamp capabilities are negotiated with the Control Plane only if the
// device clock value can be read, Tx timestamp access type is different than
// NONE, and the PTP clock for the adapter is created. When all those conditions
// are satisfied, Tx timestamp feature is enabled and tx_tstamp_caps is
// allocated and fulfilled.
//
// Return: true if the Tx timestamping is enabled, false otherwise.
//
// idpf_ptp_is_vport_rx_tstamp_ena - Verify the Rx timestamping enablement for
// a given vport.
// @vport: Virtual port structure
//
// Rx timestamp feature is enabled if the PTP clock for the adapter is created
// and it is possible to read the value of the device clock. The second
// assumption comes from the need to extend the Rx timestamp value to 64 bit
// based on the current device clock time.
//
// Return: true if the Rx timestamping is enabled, false otherwise.
//

extern "C" {
    pub fn idpf_ptp_init(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_release(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_ptp_get_caps(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_get_features_access(adapter: *const idpf_adapter);
}
extern "C" {
    pub fn idpf_ptp_get_txq_tstamp_capability(txq: *mut idpf_tx_queue) -> bool;
}
extern "C" {
    pub fn idpf_ptp_set_dev_clk_time(adapter: *mut idpf_adapter, time: u64) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_adj_dev_clk_fine(adapter: *mut idpf_adapter, incval: u64) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_adj_dev_clk_time(adapter: *mut idpf_adapter, delta: i64) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_get_vport_tstamps_caps(vport: *mut idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_get_tx_tstamp(vport: *mut idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_ptp_extend_ts(vport: *mut idpf_vport, in_tstamp: u64) -> u64;
}
extern "C" {
    pub fn idpf_ptp_tstamp_extend_32b_to_64b(cached_phc_time: u64, in_timestamp: u32) -> u64;
}
extern "C" {
    pub fn idpf_tstamp_task(work: *mut work_struct);
}

