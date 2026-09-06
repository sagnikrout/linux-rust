//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igbvf/igbvf.h
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
// Copyright(c) 2009 - 2018 Intel Corporation.
// Linux PRO/1000 Ethernet Driver main header file

// Forward declarations
// Interrupt defines

pub const IGBVF_4K_ITR: c_int = 980;
pub const IGBVF_20K_ITR: c_int = 196;
pub const IGBVF_70K_ITR: c_int = 56;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum latency_range {
    lowest_latency = 0,
    low_latency = 1,
    bulk_latency = 2,
    latency_invalid = 255
}

// Interrupt modes, as used by the IntMode parameter
pub const IGBVF_INT_MODE_LEGACY: c_int = 0;
pub const IGBVF_INT_MODE_MSI: c_int = 1;
pub const IGBVF_INT_MODE_MSIX: c_int = 2;
// Tx/Rx descriptor defines
pub const IGBVF_DEFAULT_TXD: c_int = 256;
pub const IGBVF_MAX_TXD: c_int = 4096;
pub const IGBVF_MIN_TXD: c_int = 64;
pub const IGBVF_DEFAULT_RXD: c_int = 256;
pub const IGBVF_MAX_RXD: c_int = 4096;
pub const IGBVF_MIN_RXD: c_int = 64;

// RX descriptor control thresholds.
// PTHRESH - MAC will consider prefetch if it has fewer than this number of
// descriptors available in its onboard memory.
// Setting this to 0 disables RX descriptor prefetch.
// HTHRESH - MAC will only prefetch if there are at least this many descriptors
// available in host memory.
// If PTHRESH is 0, this should also be 0.
// WTHRESH - RX descriptor writeback threshold - MAC will delay writing back
// descriptors until either it has this many to write back, or the
// ITR timer expires.
//
pub const IGBVF_RX_PTHRESH: c_int = 16;
pub const IGBVF_RX_HTHRESH: c_int = 8;
pub const IGBVF_RX_WTHRESH: c_int = 1;
// this is the size past which hardware will drop packets when setting LPE=0
pub const MAXIMUM_ETHERNET_VLAN_SIZE: c_int = 1522;
pub const IGBVF_FC_PAUSE_TIME: c_uint = 0x0680 /* 858 usec */;
// How many Tx Descriptors do we need to call netif_wake_queue ?
pub const IGBVF_TX_QUEUE_WAKE: c_int = 32;
// How many Rx Buffers do we bundle into one write to the hardware ?

pub const AUTO_ALL_MODES: c_int = 0;
pub const IGBVF_EEPROM_APME: c_uint = 0x0400;

pub const IGBVF_MAX_MAC_FILTERS: c_int = 3;
// Number of packet split data buffers (not including the header buffer)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igbvf_boards {
    board_vf,
    board_i350_vf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igbvf_queue_stats {
    pub packets: u64,
    pub bytes: u64,
}

// wrappers around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igbvf_buffer {
    pub dma: dma_addr_t,
    pub skb: *mut sk_buff,
// Tx
    pub time_stamp: c_ulong,
    pub next_to_watch: *mut e1000_adv_tx_desc,
    pub length: u16,
    pub mapped_as_page: u16,
}

// Rx
#[repr(C)]
#[derive(Copy, Clone)]
pub union igbvf_desc {
    pub rx_desc: e1000_adv_rx_desc,
    pub tx_desc: e1000_adv_tx_desc,
    pub tx_context_desc: e1000_adv_tx_context_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igbvf_ring {
    pub /: *mut *mut *mut igbvf_adapter adapter; / backlink,
    pub /: *mut *mut *mut igbvf_desc desc; / pointer to ring memory,
    pub /: *mut *mut dma_addr_t dma; / phys address of ring,
    pub /: *mut *mut unsigned int size; / length of ring in bytes,
    pub /: *mut *mut unsigned int count; / number of desc. in ring,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub head: u16,
    pub tail: u16,
// array of buffer information structs
    pub buffer_info: *mut igbvf_buffer,
    pub napi: napi_struct,
    pub 5]: char name[IFNAMSIZ +,
    pub eims_value: u32,
    pub itr_val: u32,
    pub itr_range: latency_range,
    pub itr_register: u16,
    pub set_itr: c_int,
    pub rx_skb_top: *mut sk_buff,
    pub stats: igbvf_queue_stats,
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igbvf_adapter {
    pub watchdog_timer: timer_list,
    pub reset_task: work_struct,
    pub watchdog_task: work_struct,
    pub ei: *const igbvf_info,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub rx_buffer_len: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
// track device up/down/testing state
    pub state: c_ulong,
// Interrupt Throttle Rate
    pub /: *mut *mut u32 requested_itr; / ints/sec or adaptive,
    pub /: *mut *mut u32 current_itr; / Actual ITR register value, not ints/sec,
// Tx
    pub restart_queue: c_uint,
    pub txd_cmd: u32,
    pub total_tx_bytes: c_uint,
    pub total_tx_packets: c_uint,
    pub total_rx_bytes: c_uint,
    pub total_rx_packets: c_uint,
// Tx stats
    pub tx_timeout_count: u32,
// Rx
    pub rx_ring: *mut igbvf_ring,
// Rx stats
    pub hw_csum_err: u64,
    pub hw_csum_good: u64,
    pub rx_hdr_split: u64,
    pub alloc_rx_buff_failed: u32,
    pub rx_ps_hdr_size: c_uint,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
// structs defined in e1000_hw.h
    pub hw: e1000_hw,
// The VF counters don't clear on read so we have to get a base
// count on driver start up and always subtract that base on
// the first update, thus the flag..
//
    pub stats: e1000_vf_stats,
    pub zero_base: u64,
    pub msg_enable: u32,
    pub msix_entries: *mut msix_entry,
    pub eims_enable_mask: u32,
    pub eims_other: u32,
    pub wol: u32,
    pub pba: u32,
    pub flags: c_uint,
    pub last_reset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct igbvf_info {
    pub mac: e1000_mac_type,
    pub flags: c_uint,
    pub pba: u32,
    pub ): *mut *mut void (init_ops)(struct e1000_hw,
    pub ): *mut *mut s32 (get_variants)(struct igbvf_adapter,
}

// hardware capability, feature, and workaround flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igbvf_state_t {
    __IGBVF_TESTING,
    __IGBVF_RESETTING,
    __IGBVF_DOWN
}

extern "C" {
    pub fn igbvf_set_ethtool_ops(: *mut net_device);
}
extern "C" {
    pub fn igbvf_up(: *mut igbvf_adapter) -> c_int;
}
extern "C" {
    pub fn igbvf_down(: *mut igbvf_adapter);
}
extern "C" {
    pub fn igbvf_reinit_locked(: *mut igbvf_adapter);
}
extern "C" {
    pub fn igbvf_setup_rx_resources(: *mut igbvf_adapter, : *mut igbvf_ring) -> c_int;
}
extern "C" {
    pub fn igbvf_setup_tx_resources(: *mut igbvf_adapter, : *mut igbvf_ring) -> c_int;
}
extern "C" {
    pub fn igbvf_free_rx_resources(: *mut igbvf_ring);
}
extern "C" {
    pub fn igbvf_free_tx_resources(: *mut igbvf_ring);
}
extern "C" {
    pub fn igbvf_update_stats(: *mut igbvf_adapter);
}
