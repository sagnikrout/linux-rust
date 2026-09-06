//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000/e1000.h
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
// Copyright(c) 1999 - 2006 Intel Corporation.
// Linux PRO/1000 Ethernet Driver main header file

pub const BAR_0: c_int = 0;
pub const BAR_1: c_int = 1;

pub const E1000_MAX_INTR: c_int = 10;
//
// Count for polling __E1000_RESET condition every 10-20msec.
//
pub const E1000_CHECK_RESET_COUNT: c_int = 50;
// TX/RX descriptor defines
pub const E1000_DEFAULT_TXD: c_int = 256;
pub const E1000_MAX_TXD: c_int = 256;
pub const E1000_MIN_TXD: c_int = 48;
pub const E1000_MAX_82544_TXD: c_int = 4096;
pub const E1000_DEFAULT_RXD: c_int = 256;
pub const E1000_MAX_RXD: c_int = 256;
pub const E1000_MIN_RXD: c_int = 48;
pub const E1000_MAX_82544_RXD: c_int = 4096;

// this is the size past which hardware will drop packets when setting LPE=0
pub const MAXIMUM_ETHERNET_VLAN_SIZE: c_int = 1522;
// Supported Rx Buffer Sizes

pub const E1000_RXBUFFER_512: c_int = 512;
pub const E1000_RXBUFFER_1024: c_int = 1024;
pub const E1000_RXBUFFER_2048: c_int = 2048;
pub const E1000_RXBUFFER_4096: c_int = 4096;
pub const E1000_RXBUFFER_8192: c_int = 8192;
pub const E1000_RXBUFFER_16384: c_int = 16384;
// SmartSpeed delimiters
pub const E1000_SMARTSPEED_DOWNSHIFT: c_int = 3;
pub const E1000_SMARTSPEED_MAX: c_int = 15;
// Packet Buffer allocations
pub const E1000_PBA_BYTES_SHIFT: c_uint = 0xA;
pub const E1000_TX_HEAD_ADDR_SHIFT: c_int = 7;
pub const E1000_PBA_TX_MASK: c_uint = 0xFFFF0000;
// Flow Control Watermarks
pub const E1000_FC_HIGH_DIFF: c_uint = 0x1638 /* High: 5688 bytes below Rx FIFO size */;
pub const E1000_FC_LOW_DIFF: c_uint = 0x1640 /* Low:  5696 bytes below Rx FIFO size */;
pub const E1000_FC_PAUSE_TIME: c_uint = 0xFFFF /* pause for the max or until send xon */;
// How many Tx Descriptors do we need to call netif_wake_queue ?
pub const E1000_TX_QUEUE_WAKE: c_int = 16;
// How many Rx Buffers do we bundle into one write to the hardware ?

pub const AUTO_ALL_MODES: c_int = 0;
pub const E1000_EEPROM_82544_APM: c_uint = 0x0004;
pub const E1000_EEPROM_APME: c_uint = 0x0400;

// Switch to override PHY master/slave setting

pub const E1000_MNG_VLAN_NONE: c_uint = 0xFFFF;
// wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_tx_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub time_stamp: c_ulong,
    pub length: u16,
    pub next_to_watch: u16,
    pub mapped_as_page: bool,
    pub segs: c_ushort,
    pub bytecount: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_rx_buffer {
    pub /: *mut *mut *mut page page; / jumbo: alloc_page,
    pub /: *mut *mut *mut u8 data; / else, netdev_alloc_frag,
    pub rxbuf: },
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_tx_ring {
// pointer to the descriptor ring memory
    pub desc: *mut c_void,
// physical address of the descriptor ring
    pub dma: dma_addr_t,
// length of descriptor ring in bytes
    pub size: c_uint,
// number of descriptors in the ring
    pub count: c_uint,
// next descriptor to associate a buffer with
    pub next_to_use: c_uint,
// next descriptor to check for DD status bit
    pub next_to_clean: c_uint,
// array of buffer information structs
    pub buffer_info: *mut e1000_tx_buffer,
    pub tdh: u16,
    pub tdt: u16,
    pub last_tx_tso: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_rx_ring {
// pointer to the descriptor ring memory
    pub desc: *mut c_void,
// physical address of the descriptor ring
    pub dma: dma_addr_t,
// length of descriptor ring in bytes
    pub size: c_uint,
// number of descriptors in the ring
    pub count: c_uint,
// next descriptor to associate a buffer with
    pub next_to_use: c_uint,
// next descriptor to check for DD status bit
    pub next_to_clean: c_uint,
// array of buffer information structs
    pub buffer_info: *mut e1000_rx_buffer,
    pub rx_skb_top: *mut sk_buff,
// cpu for rx queue
    pub cpu: c_int,
    pub rdh: u16,
    pub rdt: u16,
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_adapter {
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub mng_vlan_id: u16,
    pub bd_number: u32,
    pub rx_buffer_len: u32,
    pub wol: u32,
    pub smartspeed: u32,
    pub en_mng_pt: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub stats_lock: spinlock_t,
    pub total_tx_bytes: c_uint,
    pub total_tx_packets: c_uint,
    pub total_rx_bytes: c_uint,
    pub total_rx_packets: c_uint,
// Interrupt Throttle Rate
    pub itr: u32,
    pub itr_setting: u32,
    pub tx_itr: u16,
    pub rx_itr: u16,
    pub fc_autoneg: u8,
// TX
    pub /: *mut *mut *mut e1000_tx_ring tx_ring; / One per active queue,
    pub restart_queue: c_uint,
    pub txd_cmd: u32,
    pub tx_int_delay: u32,
    pub tx_abs_int_delay: u32,
    pub gotcl: u32,
    pub gotcl_old: u64,
    pub tpt_old: u64,
    pub colc_old: u64,
    pub tx_timeout_count: u32,
    pub tx_fifo_head: u32,
    pub tx_head_addr: u32,
    pub tx_fifo_size: u32,
    pub tx_timeout_factor: u8,
    pub tx_fifo_stall: core::sync::atomic::AtomicI32,
    pub pcix_82544: bool,
    pub detect_tx_hung: bool,
    pub dump_buffers: bool,
// RX
    pub work_to_do): *mut *mut int work_done, int,
    pub cleaned_count): c_int,
    pub /: *mut *mut *mut e1000_rx_ring rx_ring; / One per active queue,
    pub napi: napi_struct,
    pub num_tx_queues: c_int,
    pub num_rx_queues: c_int,
    pub hw_csum_err: u64,
    pub hw_csum_good: u64,
    pub alloc_rx_buff_failed: u32,
    pub rx_int_delay: u32,
    pub rx_abs_int_delay: u32,
    pub rx_csum: bool,
    pub gorcl: u32,
    pub gorcl_old: u64,
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
// structs defined in e1000_hw.h
    pub hw: e1000_hw,
    pub stats: e1000_hw_stats,
    pub phy_info: e1000_phy_info,
    pub phy_stats: e1000_phy_stats,
    pub test_icr: u32,
    pub test_tx_ring: e1000_tx_ring,
    pub test_rx_ring: e1000_rx_ring,
    pub msg_enable: c_int,
// to not mess up cache alignment, always add to the bottom
    pub tso_force: bool,
    pub /: *mut *mut bool smart_power_down; / phy smart power down,
    pub quad_port_a: bool,
    pub flags: c_ulong,
    pub eeprom_wol: u32,
// for ioport free
    pub bars: c_int,
    pub need_ioport: c_int,
    pub discarding: bool,
    pub reset_task: work_struct,
    pub watchdog_task: delayed_work,
    pub fifo_stall_task: delayed_work,
    pub phy_info_task: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_state_t {
    __E1000_TESTING,
    __E1000_RESETTING,
    __E1000_DOWN,
    __E1000_DISABLED
}

extern "C" {
    pub fn e1000_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn e1000_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn e1000_up(adapter: *mut e1000_adapter) -> c_int;
}
extern "C" {
    pub fn e1000_down(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_reinit_locked(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_reset(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_set_spd_dplx(adapter: *mut e1000_adapter, spd: u32, dplx: u8) -> c_int;
}
extern "C" {
    pub fn e1000_setup_all_rx_resources(adapter: *mut e1000_adapter) -> c_int;
}
extern "C" {
    pub fn e1000_setup_all_tx_resources(adapter: *mut e1000_adapter) -> c_int;
}
extern "C" {
    pub fn e1000_free_all_rx_resources(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_free_all_tx_resources(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_update_stats(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_has_link(adapter: *mut e1000_adapter) -> bool;
}
extern "C" {
    pub fn e1000_power_up_phy(: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn e1000_check_options(adapter: *mut e1000_adapter);
}
