//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl12xx/wl12xx.h
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
// This file is part of wl12xx
//
// Copyright (C) 2011 Texas Instruments Inc.
//

// WiLink 6/7 chip IDs

// FW chip version for wl127x
pub const WL127X_CHIP_VER: c_int = 6;
// minimum single-role FW version for wl127x
pub const WL127X_IFTYPE_SR_VER: c_int = 3;
pub const WL127X_MAJOR_SR_VER: c_int = 10;

pub const WL127X_MINOR_SR_VER: c_int = 133;
// minimum multi-role FW version for wl127x
pub const WL127X_IFTYPE_MR_VER: c_int = 5;
pub const WL127X_MAJOR_MR_VER: c_int = 7;

pub const WL127X_MINOR_MR_VER: c_int = 42;
// FW chip version for wl128x
pub const WL128X_CHIP_VER: c_int = 7;
// minimum single-role FW version for wl128x
pub const WL128X_IFTYPE_SR_VER: c_int = 3;
pub const WL128X_MAJOR_SR_VER: c_int = 10;

pub const WL128X_MINOR_SR_VER: c_int = 133;
// minimum multi-role FW version for wl128x
pub const WL128X_IFTYPE_MR_VER: c_int = 5;
pub const WL128X_MAJOR_MR_VER: c_int = 7;

pub const WL128X_MINOR_MR_VER: c_int = 42;

pub const WL12XX_NUM_TX_DESCRIPTORS: c_int = 16;
pub const WL12XX_NUM_RX_DESCRIPTORS: c_int = 8;
pub const WL12XX_NUM_MAC_ADDRESSES: c_int = 2;
pub const WL12XX_RX_BA_MAX_SESSIONS: c_int = 3;
pub const WL12XX_MAX_AP_STATIONS: c_int = 8;
pub const WL12XX_MAX_LINKS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl127x_rx_mem_pool_addr {
    pub addr: u32,
    pub addr_extra: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_priv {
    pub conf: wl12xx_priv_conf,
    pub ref_clock: c_int,
    pub tcxo_clock: c_int,
    pub rx_mem_addr: *mut wl127x_rx_mem_pool_addr,
}

// Reference clock values
// TCXO clock values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_clock {
    pub freq: u32,
    pub xtal: bool,
    pub hw_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_fw_packet_counters {
// Cumulative counter of released packets per AC
    pub tx_released_pkts: [u8; NUM_TX_QUEUES],
// Cumulative counter of freed packets per HLID
    pub tx_lnk_free_pkts: [u8; WL12XX_MAX_LINKS],
// Cumulative counter of released Voice memory blocks
    pub tx_voice_released_blks: u8,
// Tx rate of the last transmitted packet
    pub tx_last_rate: u8,
    pub padding: [u8; 2],
    pub __packed: },
// FW status registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_fw_status {
    pub intr: __le32,
    pub fw_rx_counter: u8,
    pub drv_rx_counter: u8,
    pub reserved: u8,
    pub tx_results_counter: u8,
    pub rx_pkt_descs: [__le32; WL12XX_NUM_RX_DESCRIPTORS],
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
    pub counters: wl12xx_fw_packet_counters,
    pub log_start_addr: __le32,
    pub __packed: },
