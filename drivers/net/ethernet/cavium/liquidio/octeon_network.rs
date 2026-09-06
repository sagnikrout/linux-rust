//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_network.h
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
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more
// details.
//
// !  \file  octeon_network.h
// \brief Host NIC Driver: Structure and Macro definitions used by NIC Module.
//

// Bit mask values for lio->ifstate
pub const LIO_IFSTATE_DROQ_OPS: c_uint = 0x01;
pub const LIO_IFSTATE_REGISTERED: c_uint = 0x02;
pub const LIO_IFSTATE_RUNNING: c_uint = 0x04;
pub const LIO_IFSTATE_RX_TIMESTAMP_ENABLED: c_uint = 0x08;
pub const LIO_IFSTATE_RESETTING: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liquidio_if_cfg_resp {
    pub rh: u64,
    pub cfg_info: liquidio_if_cfg_info,
    pub status: u64,
}

pub const LIQUIDIO_NDEV_STATS_POLL_TIME_MS: c_int = 200;
// Structure of a node in list of gather components maintained by
// NIC driver for each network device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octnic_gather {
// List manipulation. Next and prev pointers.
    pub list: list_head,
// Size of the gather component at sg in bytes.
    pub sg_size: c_int,
// Number of bytes that sg was adjusted to make it 8B-aligned.
    pub adjust: c_int,
// Gather component that can accommodate max sized fragment list
// received from the IP layer.
//
    pub sg: *mut octeon_sg_entry,
    pub sg_dma_ptr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_nic_stats_resp {
    pub rh: u64,
    pub stats: oct_link_stats,
    pub status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_nic_vf_stats_resp {
    pub rh: u64,
    pub spoofmac_cnt: u64,
    pub status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_nic_stats_ctrl {
    pub complete: completion,
    pub netdev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_nic_seapi_resp {
    pub rh: u64,
    pub fec_setting: u32,
    pub speed: u32,
}

// LiquidIO per-interface network private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio {
// State of the interface. Rx/Tx happens only in the RUNNING state.
    pub ifstate: core::sync::atomic::AtomicI32,
// Octeon Interface index number. This device will be represented as
// oct<ifidx> in the system.
//
    pub ifidx: c_int,
// Octeon Input queue to use to transmit for this network interface.
    pub txq: c_int,
// Octeon Output queue from which pkts arrive
// for this network interface.
//
    pub rxq: c_int,
// Guards each glist
    pub glist_lock: *mut spinlock_t,
// Array of gather component linked lists
    pub glist: *mut list_head,
    pub glists_virt_base: *mut c_void,
    pub glists_dma_base: *mut dma_addr_t,
    pub glist_entry_size: u32,
// Pointer to the NIC properties for the Octeon device this network
// interface is associated with.
//
    pub octprops: *mut octdev_props,
// Pointer to the octeon device structure.
    pub oct_dev: *mut octeon_device,
    pub netdev: *mut net_device,
// Link information sent by the core application for this interface.
    pub linfo: oct_link_info,
// counter of link changes
    pub link_changes: u64,
// Size of Tx queue for this octeon device.
    pub tx_qsize: u32,
// Size of Rx queue for this octeon device.
    pub rx_qsize: u32,
// Size of MTU this octeon device.
    pub mtu: u32,
// msg level flag per interface.
    pub msg_enable: u32,
// Copy of Interface capabilities: TSO, TSO6, LRO, Chescksums .
    pub dev_capability: u64,
// Copy of transmit encapsulation capabilities:
// TSO, TSO6, Checksums for this device for Kernel
// 3.10.0 onwards
//
    pub enc_dev_capability: u64,
// Copy of beacaon reg in phy
    pub phy_beacon_val: u32,
// Copy of ctrl reg in phy
    pub led_ctrl_val: u32,
// PTP clock information
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_adjust: i64,
// for atomic access to Octeon PTP reg and data struct
    pub ptp_lock: spinlock_t,
// Interface info
    pub intf_open: u32,
// work queue for  txq status
    pub txq_status_wq: cavium_wq,
// work queue for  rxq oom status
    pub rxq_status_wq: [cavium_wq; MAX_POSSIBLE_OCTEON_OUTPUT_QUEUES],
// work queue for  link status
    pub link_status_wq: cavium_wq,
// work queue to regularly send local time to octeon firmware
    pub sync_octeon_time_wq: cavium_wq,
    pub netdev_uc_count: c_int,
    pub stats_wk: cavium_wk,
}

pub const LIO_MAX_CORES: c_int = 16;
//
// \brief Enable or disable feature
// @param netdev    pointer to network device
// @param cmd       Command that just requires acknowledgment
// @param param1    Parameter to command
//
extern "C" {
    pub fn liquidio_set_feature(netdev: *mut net_device, cmd: c_int, param1: u16) -> c_int;
}
extern "C" {
    pub fn setup_rx_oom_poll_fn(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cleanup_rx_oom_poll_fn(netdev: *mut net_device);
}
//
// \brief Link control command completion callback
// @param nctrl_ptr pointer to control packet structure
//
// This routine is called by the callback function when a ctrl pkt sent to
// core app completes. The nctrl_ptr contains a copy of the command type
// and data sent to the core app. This routine is only called if the ctrl
// pkt was sent successfully to the core app.
//
extern "C" {
    pub fn liquidio_link_ctrl_cmd_completion(nctrl_ptr: *mut c_void);
}
extern "C" {
    pub fn octeon_setup_interrupt(oct: *mut octeon_device, num_ioqs: u32) -> c_int;
}
extern "C" {
    pub fn lio_fetch_stats(work: *mut work_struct);
}
extern "C" {
    pub fn lio_wait_for_clean_oq(oct: *mut octeon_device) -> c_int;
}
//
// \brief Register ethtool operations
// @param netdev    pointer to network device
//
extern "C" {
    pub fn liquidio_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn lio_delete_glists(lio: *mut lio);
}
extern "C" {
    pub fn lio_setup_glists(oct: *mut octeon_device, lio: *mut lio, num_qs: c_int) -> c_int;
}
extern "C" {
    pub fn liquidio_get_speed(lio: *mut lio) -> c_int;
}
extern "C" {
    pub fn liquidio_set_speed(lio: *mut lio, speed: c_int) -> c_int;
}
extern "C" {
    pub fn liquidio_get_fec(lio: *mut lio) -> c_int;
}
extern "C" {
    pub fn liquidio_set_fec(lio: *mut lio, on_off: c_int) -> c_int;
}
//
// \brief Net device change_mtu
// @param netdev network device
//
extern "C" {
    pub fn liquidio_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int;
}
pub const LIO_CHANGE_MTU_SUCCESS: c_int = 1;
pub const LIO_CHANGE_MTU_FAIL: c_int = 2;
pub const SKB_ADJ_MASK: c_uint = 0x3F;

pub const LIO_RXBUFFER_SZ: c_int = 2048;
// recv_buffer_alloc(struct octeon_device *oct,
// Get DMA info
// Mapping failed!!
// recv_buffer_fast_alloc(u32 size)
// Flip to other half of the buffer
// recv_buffer_reuse(struct octeon_device *oct, void *buf)

// Get DMA info
extern "C" {
    pub fn recv_buffer_fast_alloc(_arg: size) -> return;
}
//
// \brief check interface state
// @param lio per-network private data
// @param state_flag flag state to check
//
// \brief set interface state
// @param lio per-network private data
// @param state_flag flag state to set
//
// \brief clear interface state
// @param lio per-network private data
// @param state_flag flag state to clear
//
// \brief wait for all pending requests to complete
// @param oct Pointer to Octeon device
//
// Called during shutdown sequence
//
// \brief Stop Tx queues
// @param netdev network device
//
// \brief Wake Tx queues
// @param netdev network device
//
// \brief Start Tx queues
// @param netdev network device
//
// Remove the node at the head of the list. The list would be empty at
// the end of this call if there are no more nodes in the list.
//
