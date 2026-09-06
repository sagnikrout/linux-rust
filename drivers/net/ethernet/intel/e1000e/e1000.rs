//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/e1000.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
// Linux PRO/1000 Ethernet Driver main header file

// Interrupt modes, as used by the IntMode parameter
pub const E1000E_INT_MODE_LEGACY: c_int = 0;
pub const E1000E_INT_MODE_MSI: c_int = 1;
pub const E1000E_INT_MODE_MSIX: c_int = 2;
// Tx/Rx descriptor defines
pub const E1000_DEFAULT_TXD: c_int = 256;
pub const E1000_MAX_TXD: c_int = 4096;
pub const E1000_MIN_TXD: c_int = 64;
pub const E1000_DEFAULT_RXD: c_int = 256;
pub const E1000_MAX_RXD: c_int = 4096;
pub const E1000_MIN_RXD: c_int = 64;

pub const E1000_FC_PAUSE_TIME: c_uint = 0x0680 /* 858 usec */;
// How many Tx Descriptors do we need to call netif_wake_queue ?
// How many Rx Buffers do we bundle into one write to the hardware ?

pub const AUTO_ALL_MODES: c_int = 0;
pub const E1000_EEPROM_APME: c_uint = 0x0400;
pub const E1000_MNG_VLAN_NONE: c_uint = 0xFFFF;
pub const DEFAULT_JUMBO: c_int = 9234;
// Time to wait before putting the device into D3 if there's no link (in ms).
pub const LINK_TIMEOUT: c_int = 100;
// Count for polling __E1000_RESET condition every 10-20msec.
// Experimentation has shown the reset can take approximately 210msec.
//
pub const E1000_CHECK_RESET_COUNT: c_int = 25;
pub const PCICFG_DESC_RING_STATUS: c_uint = 0xe4;
pub const FLUSH_DESC_REQUIRED: c_uint = 0x100;
// in the case of WTHRESH, it appears at least the 82571/2 hardware
// writes back 4 descriptors when WTHRESH=5, and 3 descriptors when
// WTHRESH=4, so a setting of 5 gives the most efficient bus
// utilization but to avoid possible Tx stalls, set it to 1
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_boards {
    board_82571,
    board_82572,
    board_82573,
    board_82574,
    board_82583,
    board_80003es2lan,
    board_ich8lan,
    board_ich9lan,
    board_ich10lan,
    board_pchlan,
    board_pch2lan,
    board_pch_lpt,
    board_pch_spt,
    board_pch_cnp,
    board_pch_tgp,
    board_pch_adp,
    board_pch_mtp,
    board_pch_ptp
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ps_page {
    pub page: *mut page,
    pub /: *mut *mut u64 dma; / must be u64 - written to hw,
}

// wrappers around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_buffer {
    pub dma: dma_addr_t,
    pub skb: *mut sk_buff,
// Tx
    pub time_stamp: c_ulong,
    pub length: u16,
    pub next_to_watch: u16,
    pub segs: c_uint,
    pub bytecount: c_uint,
    pub mapped_as_page: u16,
}

// Rx
// arrays of page information for packet split
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_ring {
    pub /: *mut *mut *mut e1000_adapter adapter; / back pointer to adapter,
    pub /: *mut *mut *mut void desc; / pointer to ring memory,
    pub /: *mut *mut dma_addr_t dma; / phys address of ring,
    pub /: *mut *mut unsigned int size; / length of ring in bytes,
    pub /: *mut *mut unsigned int count; / number of desc. in ring,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub head: *mut void __iomem,
    pub tail: *mut void __iomem,
// array of buffer information structs
    pub buffer_info: *mut e1000_buffer,
    pub 5]: char name[IFNAMSIZ +,
    pub ims_val: u32,
    pub itr_val: u32,
    pub itr_register: *mut void __iomem,
    pub set_itr: c_int,
    pub rx_skb_top: *mut sk_buff,
}

// PHY register snapshot values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_phy_regs {
    pub /: *mut *mut u16 bmcr; / basic mode control register,
    pub /: *mut *mut u16 bmsr; / basic mode status register,
    pub /: *mut *mut u16 advertise; / auto-negotiation advertisement,
    pub /: *mut *mut u16 lpa; / link partner ability register,
    pub /: *mut *mut u16 expansion; / auto-negotiation expansion reg,
    pub /: *mut *mut u16 ctrl1000; / 1000BASE-T control register,
    pub /: *mut *mut u16 stat1000; / 1000BASE-T status register,
    pub /: *mut *mut u16 estatus; / extended status register,
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_adapter {
    pub watchdog_timer: timer_list,
    pub phy_info_timer: timer_list,
    pub blink_timer: timer_list,
    pub reset_task: work_struct,
    pub watchdog_task: work_struct,
    pub ei: *const e1000_info,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub bd_number: u32,
    pub rx_buffer_len: u32,
    pub mng_vlan_id: u16,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub eeprom_vers: u16,
// track device up/down/testing state
    pub state: c_ulong,
// Interrupt Throttle Rate
    pub itr: u32,
    pub itr_setting: u32,
    pub tx_itr: u16,
    pub rx_itr: u16,
// Tx - one ring per active queue
    pub ____cacheline_aligned_in_smp: *mut *mut e1000_ring tx_ring,
    pub tx_fifo_limit: u32,
    pub napi: napi_struct,
    pub /: *mut *mut unsigned int uncorr_errors; / uncorrectable ECC errors,
    pub /: *mut *mut unsigned int corr_errors; / correctable ECC errors,
    pub restart_queue: c_uint,
    pub txd_cmd: u32,
    pub detect_tx_hung: bool,
    pub tx_hang_recheck: bool,
    pub tx_timeout_factor: u8,
    pub tx_int_delay: u32,
    pub tx_abs_int_delay: u32,
    pub total_tx_bytes: c_uint,
    pub total_tx_packets: c_uint,
    pub total_rx_bytes: c_uint,
    pub total_rx_packets: c_uint,
// Tx stats
    pub tpt_old: u64,
    pub colc_old: u64,
    pub gotc: u32,
    pub gotc_old: u64,
    pub tx_timeout_count: u32,
    pub tx_fifo_head: u32,
    pub tx_head_addr: u32,
    pub tx_fifo_size: u32,
    pub tx_dma_failed: u32,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
// Rx
    pub ____cacheline_aligned_in_smp: int work_to_do),
    pub gfp): gfp_t,
    pub rx_ring: *mut e1000_ring,
    pub rx_int_delay: u32,
    pub rx_abs_int_delay: u32,
// Rx stats
    pub hw_csum_err: u64,
    pub hw_csum_good: u64,
    pub rx_hdr_split: u64,
    pub gorc: u32,
    pub gorc_old: u64,
    pub alloc_rx_buff_failed: u32,
    pub rx_dma_failed: u32,
    pub rx_hwtstamp_cleared: u32,
    pub rx_ps_pages: c_uint,
    pub rx_ps_bsize0: u16,
    pub max_frame_size: u32,
    pub min_frame_size: u32,
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
// structs defined in e1000_hw.h
    pub hw: e1000_hw,
    pub /: *mut *mut spinlock_t stats64_lock; / protects statistics counters,
    pub stats: e1000_hw_stats,
    pub phy_info: e1000_phy_info,
    pub phy_stats: e1000_phy_stats,
// Snapshot of PHY registers
    pub phy_regs: e1000_phy_regs,
    pub test_tx_ring: e1000_ring,
    pub test_rx_ring: e1000_ring,
    pub test_icr: u32,
    pub msg_enable: u32,
    pub num_vectors: c_uint,
    pub msix_entries: *mut msix_entry,
    pub int_mode: c_int,
    pub eiac_mask: u32,
    pub eeprom_wol: u32,
    pub wol: u32,
    pub pba: u32,
    pub max_hw_frame_size: u32,
    pub fc_autoneg: bool,
    pub flags: c_uint,
    pub flags2: c_uint,
    pub downshift_task: work_struct,
    pub update_phy_task: work_struct,
    pub print_hang_task: work_struct,
    pub phy_hang_count: c_int,
    pub tx_ring_count: u16,
    pub rx_ring_count: u16,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub systim_overflow_work: delayed_work,
    pub tx_hwtstamp_skb: *mut sk_buff,
    pub tx_hwtstamp_start: c_ulong,
    pub tx_hwtstamp_work: work_struct,
    pub /: *mut *mut spinlock_t systim_lock; / protects SYSTIML/H regsters,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub pm_qos_req: pm_qos_request,
    pub ptp_delta: c_long,
    pub eee_advert: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_info {
    pub mac: e1000_mac_type,
    pub flags: c_uint,
    pub flags2: c_uint,
    pub pba: u32,
    pub max_hw_frame_size: u32,
    pub ): *mut *mut s32 (get_variants)(struct e1000_adapter,
    pub mac_ops: *const e1000_mac_operations,
    pub phy_ops: *const e1000_phy_operations,
    pub nvm_ops: *const e1000_nvm_operations,
}

extern "C" {
    pub fn e1000e_get_base_timinca(adapter: *mut e1000_adapter, timinca: *mut u32) -> i32;
}
// The system time is maintained by a 64-bit counter comprised of the 32-bit
// SYSTIMH and SYSTIML registers.  How the counter increments (and therefore
// its resolution) is based on the contents of the TIMINCA register - it
// increments every incperiod (bits 31:24) clock ticks by incvalue (bits 23:0).
// For the best accuracy, the incperiod should be as small as possible.  The
// incvalue is scaled by a factor as large as possible (while still fitting
// in bits 23:0) so that relatively small clock corrections can be made.
//
// As a result, a shift of INCVALUE_SHIFT_n is used to fit a value of
// INCVALUE_n into the TIMINCA register allowing 32+8+(24-INCVALUE_SHIFT_n)
// bits to count nanoseconds leaving the rest for fractional nonseconds.
//
// Any given INCVALUE also has an associated maximum adjustment value. This
// maximum adjustment value is the largest increase (or decrease) which can be
// safely applied without overflowing the INCVALUE. Since INCVALUE has
// a maximum range of 24 bits, its largest value is 0xFFFFFF.
//
// To understand where the maximum value comes from, consider the following
// equation:
//
// new_incval = base_incval + (base_incval * adjustment) / 1billion
//
// To avoid overflow that means:
// max_incval = base_incval + (base_incval * max_adj) / billion
//
// Re-arranging:
// max_adj = floor(((max_incval - base_incval) * 1billion) / 1billion)
//
pub const INCVALUE_96MHZ: c_int = 125;
pub const INCVALUE_SHIFT_96MHZ: c_int = 17;
pub const INCPERIOD_SHIFT_96MHZ: c_int = 2;

pub const INCVALUE_25MHZ: c_int = 40;
pub const INCVALUE_SHIFT_25MHZ: c_int = 18;
pub const INCPERIOD_25MHZ: c_int = 1;

pub const INCVALUE_24MHZ: c_int = 125;
pub const INCVALUE_SHIFT_24MHZ: c_int = 14;
pub const INCPERIOD_24MHZ: c_int = 3;

pub const INCVALUE_38400KHZ: c_int = 26;
pub const INCVALUE_SHIFT_38400KHZ: c_int = 19;
pub const INCPERIOD_38400KHZ: c_int = 1;

// Another drawback of scaling the incvalue by a large factor is the
// 64-bit SYSTIM register overflows more quickly.  This is dealt with
// by simply reading the clock before it overflows.
//
// Clock	ns bits	Overflows after
// ~~~~~~	~~~~~~~	~~~~~~~~~~~~~~~
// 96MHz	47-bit	2^(47-INCPERIOD_SHIFT_96MHz) / 10^9 / 3600 = 9.77 hrs
// 25MHz	46-bit	2^46 / 10^9 / 3600 = 19.55 hours
//

pub const E1000_MAX_82574_SYSTIM_REREADS: c_int = 50;

// hardware capability, feature, and workaround flags

// reserved BIT(4)

// reserved BIT(28)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_state_t {
    __E1000_TESTING,
    __E1000_RESETTING,
    __E1000_ACCESS_SHARED_RESOURCE,
    __E1000_DOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum latency_range {
    lowest_latency = 0,
    low_latency = 1,
    bulk_latency = 2,
    latency_invalid = 255
}

extern "C" {
    pub fn e1000e_check_options(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn e1000e_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn e1000e_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn e1000e_up(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_down(adapter: *mut e1000_adapter, reset: bool);
}
extern "C" {
    pub fn e1000e_reinit_locked(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_reset(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_power_up_phy(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_setup_rx_resources(ring: *mut e1000_ring) -> c_int;
}
extern "C" {
    pub fn e1000e_setup_tx_resources(ring: *mut e1000_ring) -> c_int;
}
extern "C" {
    pub fn e1000e_free_rx_resources(ring: *mut e1000_ring);
}
extern "C" {
    pub fn e1000e_free_tx_resources(ring: *mut e1000_ring);
}
extern "C" {
    pub fn e1000e_set_interrupt_capability(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_reset_interrupt_capability(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_get_hw_control(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_release_hw_control(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_write_itr(adapter: *mut e1000_adapter, itr: u32);
}
extern "C" {
    pub fn e1000e_ptp_init(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_ptp_remove(adapter: *mut e1000_adapter);
}
extern "C" {
    pub fn e1000e_reload_nvm_generic(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_read_mac_addr_generic(_arg: hw) -> return;
}
extern "C" {
    pub fn readl(reg: hw->hw_addr +) -> return;
}

extern "C" {
    pub fn __ew32(hw: *mut e1000_hw, reg: c_ulong, val: u32);
}

