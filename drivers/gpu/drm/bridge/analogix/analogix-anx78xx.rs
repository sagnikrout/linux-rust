//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/analogix-anx78xx.h
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
// Copyright(c) 2016, Analogix Semiconductor. All rights reserved.
//

// Macro flag: #define __ANX78xx_H

//
// Register definitions for RX_PO
//
// System Control and Status
//
// Software Reset Register 1
pub const SP_SOFTWARE_RESET1_REG: c_uint = 0x11;

// System Status Register
pub const SP_SYSTEM_STATUS_REG: c_uint = 0x14;

// HDMI Status Register
pub const SP_HDMI_STATUS_REG: c_uint = 0x15;

// HDMI Mute Control Register
pub const SP_HDMI_MUTE_CTRL_REG: c_uint = 0x16;

// System Power Down Register 1
pub const SP_SYSTEM_POWER_DOWN1_REG: c_uint = 0x18;

//
// Audio and Video Auto Control
//
// Auto Audio and Video Control register
pub const SP_AUDVID_CTRL_REG: c_uint = 0x20;

// Audio Exception Enable Registers

// Bits for Audio Exception Enable Register 3

//
// Interrupt
//
// Interrupt Status Register 1
pub const SP_INT_STATUS1_REG: c_uint = 0x31;
// Bits for Interrupt Status Register 1

// Bits for Interrupt Status Register 2

// Bits for Interrupt Status Register 3

// Bits for Interrupt Status Register 5

// Bits for Interrupt Status Register 6
pub const SP_INT_STATUS6_REG: c_uint = 0x36;

// Bits for Interrupt Status Register 7

// Interrupt Mask 1 Status Registers
pub const SP_INT_MASK1_REG: c_uint = 0x41;
// HDMI US TIMER Control Register
pub const SP_HDMI_US_TIMER_CTRL_REG: c_uint = 0x49;
pub const SP_MS_TIMER_MARGIN_10_8_MASK: c_uint = 0x07;
//
// TMDS Control
//
// TMDS Control Registers

// Bits for TMDS Control Register 7

//
// Video Control
//
// Video Status Register
pub const SP_VIDEO_STATUS_REG: c_uint = 0x70;
pub const SP_COLOR_DEPTH_MASK: c_uint = 0xf0;
pub const SP_COLOR_DEPTH_SHIFT: c_int = 4;

// Video Data Range Control Register
pub const SP_VID_DATA_RANGE_CTRL_REG: c_uint = 0x83;

// Pixel Clock High Resolution Counter Registers

//
// Audio Control
//
// Number of Audio Channels Status Registers
pub const SP_AUD_CH_STATUS_REG_NUM: c_int = 6;
// Audio IN S/PDIF Channel Status Registers
pub const SP_AUD_SPDIF_CH_STATUS_BASE: c_uint = 0xc7;
// Audio IN S/PDIF Channel Status Register 4
pub const SP_FS_FREQ_MASK: c_uint = 0x0f;

//
// Micellaneous Control Block
//
// CHIP Control Register
pub const SP_CHIP_CTRL_REG: c_uint = 0xe3;

// Packet Receiving Status Register
pub const SP_PACKET_RECEIVING_STATUS_REG: c_uint = 0xf3;

//
// Register definitions for RX_P1
//
// HDCP BCAPS Shadow Register
pub const SP_HDCP_BCAPS_SHADOW_REG: c_uint = 0x2a;

// HDCP Status Register
pub const SP_RX_HDCP_STATUS_REG: c_uint = 0x3f;

//
// InfoFrame and Control Packet Registers
//
// AVI InfoFrame packet checksum
pub const SP_AVI_INFOFRAME_CHECKSUM: c_uint = 0xa3;
// AVI InfoFrame Registers
pub const SP_AVI_INFOFRAME_DATA_BASE: c_uint = 0xa4;
pub const SP_AVI_COLOR_F_MASK: c_uint = 0x60;
pub const SP_AVI_COLOR_F_SHIFT: c_int = 5;
// Audio InfoFrame Registers
pub const SP_AUD_INFOFRAME_DATA_BASE: c_uint = 0xc4;
pub const SP_AUD_INFOFRAME_LAYOUT_MASK: c_uint = 0x0f;
// MPEG/HDMI Vendor Specific InfoFrame Packet type code
pub const SP_MPEG_VS_INFOFRAME_TYPE_REG: c_uint = 0xe0;
// MPEG/HDMI Vendor Specific InfoFrame Packet length
pub const SP_MPEG_VS_INFOFRAME_LEN_REG: c_uint = 0xe2;
// MPEG/HDMI Vendor Specific InfoFrame Packet version number
pub const SP_MPEG_VS_INFOFRAME_VER_REG: c_uint = 0xe1;
// MPEG/HDMI Vendor Specific InfoFrame Packet content
pub const SP_MPEG_VS_INFOFRAME_DATA_BASE: c_uint = 0xe4;
// General Control Packet Register
pub const SP_GENERAL_CTRL_PACKET_REG: c_uint = 0x9f;

//
// Register definitions for TX_P1
//
// DP TX Link Training Control Register
pub const SP_DP_TX_LT_CTRL0_REG: c_uint = 0x30;
// PD 1.2 Lint Training 80bit Pattern Register
pub const SP_DP_LT_80BIT_PATTERN0_REG: c_uint = 0x80;
pub const SP_DP_LT_80BIT_PATTERN_REG_NUM: c_int = 10;
// Audio Interface Control Register 0
pub const SP_AUD_INTERFACE_CTRL0_REG: c_uint = 0x5f;
pub const SP_AUD_INTERFACE_DISABLE: c_uint = 0x80;
// Audio Interface Control Register 2
pub const SP_AUD_INTERFACE_CTRL2_REG: c_uint = 0x60;
pub const SP_M_AUD_ADJUST_ST: c_uint = 0x04;
// Audio Interface Control Register 3
pub const SP_AUD_INTERFACE_CTRL3_REG: c_uint = 0x62;
// Audio Interface Control Register 4
pub const SP_AUD_INTERFACE_CTRL4_REG: c_uint = 0x67;
// Audio Interface Control Register 5
pub const SP_AUD_INTERFACE_CTRL5_REG: c_uint = 0x68;
// Audio Interface Control Register 6
pub const SP_AUD_INTERFACE_CTRL6_REG: c_uint = 0x69;
// Firmware Version Register
pub const SP_FW_VER_REG: c_uint = 0xb7;
