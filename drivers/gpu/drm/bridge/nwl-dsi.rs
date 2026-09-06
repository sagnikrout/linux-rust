//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/nwl-dsi.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NWL MIPI DSI host driver
//
// Copyright (C) 2017 NXP
// Copyright (C) 2019 Purism SPC
//
// DSI HOST registers
pub const NWL_DSI_CFG_NUM_LANES: c_uint = 0x0;
pub const NWL_DSI_CFG_NONCONTINUOUS_CLK: c_uint = 0x4;
pub const NWL_DSI_CFG_T_PRE: c_uint = 0x8;
pub const NWL_DSI_CFG_T_POST: c_uint = 0xc;
pub const NWL_DSI_CFG_TX_GAP: c_uint = 0x10;
pub const NWL_DSI_CFG_AUTOINSERT_EOTP: c_uint = 0x14;
pub const NWL_DSI_CFG_EXTRA_CMDS_AFTER_EOTP: c_uint = 0x18;
pub const NWL_DSI_CFG_HTX_TO_COUNT: c_uint = 0x1c;
pub const NWL_DSI_CFG_LRX_H_TO_COUNT: c_uint = 0x20;
pub const NWL_DSI_CFG_BTA_H_TO_COUNT: c_uint = 0x24;
pub const NWL_DSI_CFG_TWAKEUP: c_uint = 0x28;
pub const NWL_DSI_CFG_STATUS_OUT: c_uint = 0x2c;
pub const NWL_DSI_RX_ERROR_STATUS: c_uint = 0x30;
// DSI DPI registers
pub const NWL_DSI_PIXEL_PAYLOAD_SIZE: c_uint = 0x200;
pub const NWL_DSI_PIXEL_FIFO_SEND_LEVEL: c_uint = 0x204;
pub const NWL_DSI_INTERFACE_COLOR_CODING: c_uint = 0x208;
pub const NWL_DSI_PIXEL_FORMAT: c_uint = 0x20c;
pub const NWL_DSI_VSYNC_POLARITY: c_uint = 0x210;
pub const NWL_DSI_VSYNC_POLARITY_ACTIVE_LOW: c_int = 0;

pub const NWL_DSI_HSYNC_POLARITY: c_uint = 0x214;
pub const NWL_DSI_HSYNC_POLARITY_ACTIVE_LOW: c_int = 0;

pub const NWL_DSI_VIDEO_MODE: c_uint = 0x218;
pub const NWL_DSI_HFP: c_uint = 0x21c;
pub const NWL_DSI_HBP: c_uint = 0x220;
pub const NWL_DSI_HSA: c_uint = 0x224;
pub const NWL_DSI_ENABLE_MULT_PKTS: c_uint = 0x228;
pub const NWL_DSI_VBP: c_uint = 0x22c;
pub const NWL_DSI_VFP: c_uint = 0x230;
pub const NWL_DSI_BLLP_MODE: c_uint = 0x234;
pub const NWL_DSI_USE_NULL_PKT_BLLP: c_uint = 0x238;
pub const NWL_DSI_VACTIVE: c_uint = 0x23c;
pub const NWL_DSI_VC: c_uint = 0x240;
// DSI APB PKT control
pub const NWL_DSI_TX_PAYLOAD: c_uint = 0x280;
pub const NWL_DSI_PKT_CONTROL: c_uint = 0x284;
pub const NWL_DSI_SEND_PACKET: c_uint = 0x288;
pub const NWL_DSI_PKT_STATUS: c_uint = 0x28c;
pub const NWL_DSI_PKT_FIFO_WR_LEVEL: c_uint = 0x290;
pub const NWL_DSI_PKT_FIFO_RD_LEVEL: c_uint = 0x294;
pub const NWL_DSI_RX_PAYLOAD: c_uint = 0x298;
pub const NWL_DSI_RX_PKT_HEADER: c_uint = 0x29c;
// DSI IRQ handling
pub const NWL_DSI_IRQ_STATUS: c_uint = 0x2a0;

pub const NWL_DSI_IRQ_STATUS2: c_uint = 0x2a4;

pub const NWL_DSI_IRQ_MASK: c_uint = 0x2a8;

pub const NWL_DSI_IRQ_MASK2: c_uint = 0x2ac;

//
// PKT_CONTROL format:
// [15: 0] - word count
// [17:16] - virtual channel
// [23:18] - data type
// [24]	   - LP or HS select (0 - LP, 1 - HS)
// [25]	   - perform BTA after packet is sent
// [26]	   - perform BTA only, no packet tx
//

//
// RX_PKT_HEADER format:
// [15: 0] - word count
// [21:16] - data type
// [23:22] - virtual channel
//

// DSI Video mode
pub const NWL_DSI_VM_BURST_MODE_WITH_SYNC_PULSES: c_int = 0;

// * DPI color coding
pub const NWL_DSI_DPI_16_BIT_565_PACKED: c_int = 0;
pub const NWL_DSI_DPI_16_BIT_565_ALIGNED: c_int = 1;
pub const NWL_DSI_DPI_16_BIT_565_SHIFTED: c_int = 2;
pub const NWL_DSI_DPI_18_BIT_PACKED: c_int = 3;
pub const NWL_DSI_DPI_18_BIT_ALIGNED: c_int = 4;
pub const NWL_DSI_DPI_24_BIT: c_int = 5;
// * DPI Pixel format
pub const NWL_DSI_PIXEL_FORMAT_16: c_int = 0;

