//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/ibmveth.h
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
// IBM Power Virtual Ethernet Device Driver
//
// Copyright (C) IBM Corporation, 2003, 2010
//
// Authors: Dave Larson <larson1@us.ibm.com>
// Santiago Leon <santil@linux.vnet.ibm.com>
// Brian King <brking@linux.vnet.ibm.com>
// Robert Jennings <rcj@linux.vnet.ibm.com>
// Anton Blanchard <anton@au.ibm.com>
//
// constants for H_MULTICAST_CTRL
pub const IbmVethMcastReceptionModifyBit: c_uint = 0x80000UL;
pub const IbmVethMcastReceptionEnableBit: c_uint = 0x20000UL;
pub const IbmVethMcastFilterModifyBit: c_uint = 0x40000UL;
pub const IbmVethMcastFilterEnableBit: c_uint = 0x10000UL;

pub const IbmVethMcastAddFilter: c_uint = 0x1UL;
pub const IbmVethMcastRemoveFilter: c_uint = 0x2UL;
pub const IbmVethMcastClearFilterTable: c_uint = 0x3UL;
pub const IBMVETH_ILLAN_RX_MULTI_BUFF_SUPPORT: c_uint = 0x0000000000040000UL;
pub const IBMVETH_ILLAN_LRG_SR_ENABLED: c_uint = 0x0000000000010000UL;
pub const IBMVETH_ILLAN_LRG_SND_SUPPORT: c_uint = 0x0000000000008000UL;
pub const IBMVETH_ILLAN_PADDED_PKT_CSUM: c_uint = 0x0000000000002000UL;
pub const IBMVETH_ILLAN_TRUNK_PRI_MASK: c_uint = 0x0000000000000F00UL;
pub const IBMVETH_ILLAN_IPV6_TCP_CSUM: c_uint = 0x0000000000000004UL;
pub const IBMVETH_ILLAN_IPV4_TCP_CSUM: c_uint = 0x0000000000000002UL;
pub const IBMVETH_ILLAN_ACTIVE_TRUNK: c_uint = 0x0000000000000001UL;

// hcall macros

// FW allows us to send 6 descriptors but we only use one so mark
// the other 5 as unused (0)
//
// corellator_out = retbuf[0];
// ret_attributes = retbuf[0];

pub const IBMVETH_NUM_BUFF_POOLS: c_int = 5;

pub const IBMVETH_MIN_MTU: c_int = 68;
pub const IBMVETH_MAX_POOL_COUNT: c_int = 4096;
pub const IBMVETH_BUFF_LIST_SIZE: c_int = 4096;
pub const IBMVETH_FILT_LIST_SIZE: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmveth_buff_pool {
    pub size: u32,
    pub index: u32,
    pub buff_size: u32,
    pub threshold: u32,
    pub available: core::sync::atomic::AtomicI32,
    pub consumer_index: u32,
    pub producer_index: u32,
    pub free_map: *mut u16,
    pub dma_addr: *mut dma_addr_t,
    pub skbuff: *mut sk_buff,
    pub active: c_int,
    pub kobj: kobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmveth_rx_q {
    pub index: u64,
    pub num_slots: u64,
    pub toggle: u64,
    pub queue_dma: dma_addr_t,
    pub queue_len: u32,
    pub queue_addr: *mut ibmveth_rx_q_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmveth_adapter {
    pub vdev: *mut vio_dev,
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub work: work_struct,
    pub mcastFilterSize: c_uint,
    pub buffer_list_addr: *mut c_void,
    pub filter_list_addr: *mut c_void,
    pub tx_ltb_ptr: [*mut c_void; IBMVETH_MAX_QUEUES],
    pub tx_ltb_size: c_uint,
    pub tx_ltb_dma: [dma_addr_t; IBMVETH_MAX_QUEUES],
    pub buffer_list_dma: dma_addr_t,
    pub filter_list_dma: dma_addr_t,
    pub rx_buff_pool: [ibmveth_buff_pool; IBMVETH_NUM_BUFF_POOLS],
    pub rx_queue: ibmveth_rx_q,
    pub rx_csum: c_int,
    pub large_send: c_int,
    pub is_active_trunk: bool,
    pub rx_buffers_per_hcall: c_uint,
    pub fw_ipv6_csum_support: u64,
    pub fw_ipv4_csum_support: u64,
    pub fw_large_send_support: u64,
// adapter specific stats
    pub replenish_task_cycles: u64,
    pub replenish_no_mem: u64,
    pub replenish_add_buff_failure: u64,
    pub replenish_add_buff_success: u64,
    pub rx_invalid_buffer: u64,
    pub rx_no_buffer: u64,
    pub tx_map_failed: u64,
    pub tx_send_failed: u64,
    pub tx_large_packets: u64,
    pub rx_large_packets: u64,
// Ethtool settings
    pub duplex: u8,
    pub speed: u32,
}

//
// We pass struct ibmveth_buf_desc_fields to the hypervisor in registers,
// so we don't need to byteswap the two elements. However since we use
// a union (ibmveth_buf_desc) to convert from the struct to a u64 we
// do end up with endian specific ordering of the elements and that
// needs correcting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmveth_buf_desc_fields {

    pub flags_len: u32,
    pub address: u32,

    pub address: u32,
    pub flags_len: u32,

pub const IBMVETH_BUF_VALID: c_uint = 0x80000000;
pub const IBMVETH_BUF_TOGGLE: c_uint = 0x40000000;
pub const IBMVETH_BUF_LRG_SND: c_uint = 0x04000000;
pub const IBMVETH_BUF_NO_CSUM: c_uint = 0x02000000;
pub const IBMVETH_BUF_CSUM_GOOD: c_uint = 0x01000000;
pub const IBMVETH_BUF_LEN_MASK: c_uint = 0x00FFFFFF;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmveth_buf_desc {
    pub desc: u64,
    pub fields: ibmveth_buf_desc_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmveth_rx_q_entry {
    pub flags_off: __be32,
pub const IBMVETH_RXQ_TOGGLE: c_uint = 0x80000000;
pub const IBMVETH_RXQ_TOGGLE_SHIFT: c_int = 31;
pub const IBMVETH_RXQ_VALID: c_uint = 0x40000000;
pub const IBMVETH_RXQ_LRG_PKT: c_uint = 0x04000000;
pub const IBMVETH_RXQ_NO_CSUM: c_uint = 0x02000000;
pub const IBMVETH_RXQ_CSUM_GOOD: c_uint = 0x01000000;
pub const IBMVETH_RXQ_OFF_MASK: c_uint = 0x0000FFFF;
    pub length: __be32,
// correlator is only used by the OS, no need to byte swap
    pub correlator: u64,
}
