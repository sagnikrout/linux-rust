//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/wl18xx.h
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
//
// This file is part of wl18xx
//
// Copyright (C) 2011 Texas Instruments Inc.
//

// minimum FW required for driver
pub const WL18XX_CHIP_VER: c_int = 8;
pub const WL18XX_IFTYPE_VER: c_int = 9;
pub const WL18XX_MAJOR_VER: c_int = 0;

pub const WL18XX_MINOR_VER: c_int = 58;
pub const WL18XX_CMD_MAX_SIZE: c_int = 740;

pub const WL18XX_NUM_TX_DESCRIPTORS: c_int = 32;
pub const WL18XX_NUM_RX_DESCRIPTORS: c_int = 32;
pub const WL18XX_NUM_MAC_ADDRESSES: c_int = 2;
pub const WL18XX_RX_BA_MAX_SESSIONS: c_int = 13;
pub const WL18XX_MAX_AP_STATIONS: c_int = 10;
pub const WL18XX_MAX_LINKS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_priv {
// buffer for sending commands to FW
    pub cmd_buf: [u8; WL18XX_CMD_MAX_SIZE],
    pub conf: wl18xx_priv_conf,
// Index of last released Tx desc in FW
    pub last_fw_rls_idx: u8,
// number of keys requiring extra spare mem-blocks
    pub extra_spare_key_count: c_int,
}

pub const WL18XX_FW_MAX_TX_STATUS_DESC: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_fw_status_priv {
//
// Index in released_tx_desc for first byte that holds
// released tx host desc
//
    pub fw_release_idx: u8,
//
// Array of host Tx descriptors, where fw_release_idx
// indicated the first released idx.
//
    pub released_tx_desc: [u8; WL18XX_FW_MAX_TX_STATUS_DESC],
// A bitmap representing the currently suspended links. The suspend
// is short lived, for multi-channel Tx requirements.
//
    pub link_suspend_bitmap: __le32,
// packet threshold for an "almost empty" AC,
// for Tx schedulng purposes
//
    pub tx_ac_threshold: u8,
// number of packets to queue up for a link in PS
    pub tx_ps_threshold: u8,
// number of packet to queue up for a suspended link
    pub tx_suspend_threshold: u8,
// Should have less than this number of packets in queue of a slow
// link to qualify as high priority link
//
    pub tx_slow_link_prio_threshold: u8,
// Should have less than this number of packets in queue of a fast
// link to qualify as high priority link
//
    pub tx_fast_link_prio_threshold: u8,
// Should have less than this number of packets in queue of a slow
// link before we stop queuing up packets for it.
//
    pub tx_slow_stop_threshold: u8,
// Should have less than this number of packets in queue of a fast
// link before we stop queuing up packets for it.
//
    pub tx_fast_stop_threshold: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_fw_packet_counters {
// Cumulative counter of released packets per AC
    pub tx_released_pkts: [u8; NUM_TX_QUEUES],
// Cumulative counter of freed packets per HLID
    pub tx_lnk_free_pkts: [u8; WL18XX_MAX_LINKS],
// Cumulative counter of released Voice memory blocks
    pub tx_voice_released_blks: u8,
// Tx rate of the last transmitted packet
    pub tx_last_rate: u8,
// Tx rate or Tx rate estimate pre-calculated by fw in mbps units
    pub tx_last_rate_mbps: u8,
// hlid for which the rates were reported
    pub hlid: u8,
    pub __packed: },
// FW status registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_fw_status {
    pub intr: __le32,
    pub fw_rx_counter: u8,
    pub drv_rx_counter: u8,
    pub reserved: u8,
    pub tx_results_counter: u8,
    pub rx_pkt_descs: [__le32; WL18XX_NUM_RX_DESCRIPTORS],
    pub fw_localtime: __le32,
//
// A bitmap (where each bit represents a single HLID)
// to indicate if the station is in PS mode.
//
    pub link_ps_bitmap: __le32,
//
// A bitmap (where each bit represents a single HLID) to indicate
// if the station is in Fast mode
//
    pub link_fast_bitmap: __le32,
// Cumulative counter of total released mem blocks since FW-reset
    pub total_released_blks: __le32,
// Size (in Memory Blocks) of TX pool
    pub tx_total: __le32,
    pub counters: wl18xx_fw_packet_counters,
    pub log_start_addr: __le32,
// Private status to be used by the lower drivers
    pub priv: wl18xx_fw_status_priv,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_fw_packet_counters_8_9_1 {
// Cumulative counter of released packets per AC
    pub tx_released_pkts: [u8; NUM_TX_QUEUES],
// Cumulative counter of freed packets per HLID
    pub tx_lnk_free_pkts: [u8; WL18XX_MAX_LINKS],
// PN16 of last TKIP/AES seq-num per HLID
    pub tx_lnk_sec_pn16: [__le16; WL18XX_MAX_LINKS],
// Cumulative counter of released Voice memory blocks
    pub tx_voice_released_blks: u8,
// Tx rate of the last transmitted packet
    pub tx_last_rate: u8,
// Tx rate or Tx rate estimate pre-calculated by fw in mbps units
    pub tx_last_rate_mbps: u8,
// hlid for which the rates were reported
    pub hlid: u8,
    pub __packed: },
// FW status registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_fw_status_8_9_1 {
    pub intr: __le32,
    pub fw_rx_counter: u8,
    pub drv_rx_counter: u8,
    pub reserved: u8,
    pub tx_results_counter: u8,
    pub rx_pkt_descs: [__le32; WL18XX_NUM_RX_DESCRIPTORS],
    pub fw_localtime: __le32,
//
// A bitmap (where each bit represents a single HLID)
// to indicate if the station is in PS mode.
//
    pub link_ps_bitmap: __le32,
//
// A bitmap (where each bit represents a single HLID) to indicate
// if the station is in Fast mode
//
    pub link_fast_bitmap: __le32,
// Cumulative counter of total released mem blocks since FW-reset
    pub total_released_blks: __le32,
// Size (in Memory Blocks) of TX pool
    pub tx_total: __le32,
    pub counters: wl18xx_fw_packet_counters_8_9_1,
    pub log_start_addr: __le32,
// Private status to be used by the lower drivers
    pub priv: wl18xx_fw_status_priv,
    pub __packed: },
pub const WL18XX_PHY_VERSION_MAX_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_static_data_priv {
    pub phy_version: [c_char; WL18XX_PHY_VERSION_MAX_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_clk_cfg {
    pub n: u32,
    pub m: u32,
    pub p: u32,
    pub q: u32,
    pub swallow: bool,
}
