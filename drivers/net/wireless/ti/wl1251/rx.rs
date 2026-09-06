//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/rx.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

//
// RX PATH
//
// The Rx path uses a double buffer and an rx_contro structure, each located
// at a fixed address in the device memory. The host keeps track of which
// buffer is available and alternates between them on a per packet basis.
// The size of each of the two buffers is large enough to hold the longest
// 802.3 packet.
// The RX path goes like that:
// 1) The target generates an interrupt each time a new packet is received.
// There are 2 RX interrupts, one for each buffer.
// 2) The host reads the received packet from one of the double buffers.
// 3) The host triggers a target interrupt.
// 4) The target prepares the next RX packet.
//

pub const WL1251_RX_ALIGN_TO: c_int = 4;

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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_rx_descriptor {
    pub /: *mut *mut u32 timestamp; / In microseconds,
    pub /: *mut *mut u16 length; / Paylod length, including headers,
    pub flags: u16,
//
// 0 - 802.11
// 1 - 802.3
// 2 - IP
// 3 - Raw Codec
//
    pub type: u8,
//
// Received Rate:
// 0x0A - 1MBPS
// 0x14 - 2MBPS
// 0x37 - 5_5MBPS
// 0x0B - 6MBPS
// 0x0F - 9MBPS
// 0x6E - 11MBPS
// 0x0A - 12MBPS
// 0x0E - 18MBPS
// 0xDC - 22MBPS
// 0x09 - 24MBPS
// 0x0D - 36MBPS
// 0x08 - 48MBPS
// 0x0C - 54MBPS
//
    pub rate: u8,
    pub /: *mut *mut u8 mod_pre; / Modulation and preamble,
    pub channel: u8,
//
// 0 - 2.4 Ghz
// 1 - 5 Ghz
//
    pub band: u8,
    pub /: *mut *mut s8 rssi; / in dB,
    pub /: *mut *mut u8 rcpi; / in dB,
    pub /: *mut *mut u8 snr; / in dB,
    pub __packed: },
    pub wl): *mut void wl1251_rx(struct wl1251,
