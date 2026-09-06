//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/rx.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

pub const RSSI_LEVEL_BITMASK: c_uint = 0x7F;

pub const PLCP_HEADER_LENGTH: c_int = 8;
pub const RX_DESC_PACKETID_SHIFT: c_int = 11;
pub const RX_MAX_PACKET_ID: c_int = 3;
pub const RX_DESC_VALID_FCS: c_uint = 0x0001;
pub const RX_DESC_MATCH_RXADDR1: c_uint = 0x0002;
pub const RX_DESC_MCAST: c_uint = 0x0004;
pub const RX_DESC_STAINTIM: c_uint = 0x0008;
pub const RX_DESC_VIRTUAL_BM: c_uint = 0x0010;
pub const RX_DESC_BCAST: c_uint = 0x0020;
pub const RX_DESC_MATCH_SSID: c_uint = 0x0040;
pub const RX_DESC_MATCH_BSSID: c_uint = 0x0080;
pub const RX_DESC_ENCRYPTION_MASK: c_uint = 0x0300;
pub const RX_DESC_MEASURMENT: c_uint = 0x0400;
pub const RX_DESC_SEQNUM_MASK: c_uint = 0x1800;
pub const RX_DESC_MIC_FAIL: c_uint = 0x2000;
pub const RX_DESC_DECRYPT_FAIL: c_uint = 0x4000;
//
// RX Descriptor flags:
//
// Bits 0-1 - band
// Bit  2   - STBC
// Bit  3   - A-MPDU
// Bit  4   - HT
// Bits 5-7 - encryption
//
pub const WL1271_RX_DESC_BAND_MASK: c_uint = 0x03;
pub const WL1271_RX_DESC_ENCRYPT_MASK: c_uint = 0xE0;
pub const WL1271_RX_DESC_BAND_BG: c_uint = 0x00;
pub const WL1271_RX_DESC_BAND_J: c_uint = 0x01;
pub const WL1271_RX_DESC_BAND_A: c_uint = 0x02;

pub const WL1271_RX_DESC_ENCRYPT_WEP: c_uint = 0x20;
pub const WL1271_RX_DESC_ENCRYPT_TKIP: c_uint = 0x40;
pub const WL1271_RX_DESC_ENCRYPT_AES: c_uint = 0x60;
pub const WL1271_RX_DESC_ENCRYPT_GEM: c_uint = 0x80;
//
// RX Descriptor status
//
// Bits 0-2 - error code
// Bits 3-5 - process_id tag (AP mode FW)
// Bits 6-7 - reserved
//
pub const WL1271_RX_DESC_STATUS_MASK: c_uint = 0x07;
pub const WL1271_RX_DESC_SUCCESS: c_uint = 0x00;
pub const WL1271_RX_DESC_DECRYPT_FAIL: c_uint = 0x01;
pub const WL1271_RX_DESC_MIC_FAIL: c_uint = 0x02;
pub const RX_MEM_BLOCK_MASK: c_uint = 0xFF;
pub const RX_BUF_SIZE_MASK: c_uint = 0xFFF00;
pub const RX_BUF_SIZE_SHIFT_DIV: c_int = 6;
pub const ALIGNED_RX_BUF_SIZE_MASK: c_uint = 0xFFFF00;
pub const ALIGNED_RX_BUF_SIZE_SHIFT: c_int = 8;
// If set, the start of IP payload is not 4 bytes aligned

// If set, the buffer was padded by the FW to be 4 bytes aligned

//
// Account for the padding inserted by the FW in case of RX_ALIGNMENT
// or for fixing alignment in case the packet wasn't aligned.
//
pub const RX_BUF_ALIGN: c_int = 2;
// Describes the alignment state of a Rx buffer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl_rx_buf_align {
    WLCORE_RX_BUF_ALIGNED,
    WLCORE_RX_BUF_UNALIGNED,
    WLCORE_RX_BUF_PADDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_rx_descriptor {
    pub length: __le16,
    pub status: u8,
    pub flags: u8,
    pub rate: u8,
    pub channel: u8,
    pub rssi: i8,
    pub snr: u8,
    pub timestamp: __le32,
    pub packet_class: u8,
    pub hlid: u8,
    pub pad_len: u8,
    pub reserved: u8,
    pub __packed: },
    pub status): *mut *mut int wlcore_rx(struct wl1271 wl, struct wl_fw_status,
    pub band): u8 wl1271_rate_to_idx(int rate, enum nl80211_band,
    pub filter): *mut wl12xx_rx_filter,
    pub wl): *mut int wl1271_rx_filter_clear_all(struct wl1271,
