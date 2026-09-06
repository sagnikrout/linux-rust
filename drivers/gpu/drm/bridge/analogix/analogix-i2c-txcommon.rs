//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/analogix-i2c-txcommon.h
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
// Register definitions for TX_P2
//
// Core Register Definitions
//
// Device ID Low Byte Register
pub const SP_DEVICE_IDL_REG: c_uint = 0x02;
// Device ID High Byte Register
pub const SP_DEVICE_IDH_REG: c_uint = 0x03;
// Device version register
pub const SP_DEVICE_VERSION_REG: c_uint = 0x04;
// Power Down Control Register
pub const SP_POWERDOWN_CTRL_REG: c_uint = 0x05;

// Reset Control Register 1
pub const SP_RESET_CTRL1_REG: c_uint = 0x06;

// Reset Control Register 2
pub const SP_RESET_CTRL2_REG: c_uint = 0x07;

// Video Control Register 1
pub const SP_VID_CTRL1_REG: c_uint = 0x08;

// Video Control Register 2
pub const SP_VID_CTRL2_REG: c_uint = 0x09;
pub const SP_IN_COLOR_F_MASK: c_uint = 0x03;

pub const SP_IN_BPC_MASK: c_uint = 0x70;
pub const SP_IN_BPC_SHIFT: c_int = 4;

// Video Control Register 3
pub const SP_VID_CTRL3_REG: c_uint = 0x0a;

// Video Control Register 5
pub const SP_VID_CTRL5_REG: c_uint = 0x0c;

// Video Control Register 6
pub const SP_VID_CTRL6_REG: c_uint = 0x0d;

// Video Control Register 8
pub const SP_VID_CTRL8_REG: c_uint = 0x0f;

// Total Line Status Low Byte Register
pub const SP_TOTAL_LINE_STAL_REG: c_uint = 0x24;
// Total Line Status High Byte Register
pub const SP_TOTAL_LINE_STAH_REG: c_uint = 0x25;
// Active Line Status Low Byte Register
pub const SP_ACT_LINE_STAL_REG: c_uint = 0x26;
// Active Line Status High Byte Register
pub const SP_ACT_LINE_STAH_REG: c_uint = 0x27;
// Vertical Front Porch Status Register
pub const SP_V_F_PORCH_STA_REG: c_uint = 0x28;
// Vertical SYNC Width Status Register
pub const SP_V_SYNC_STA_REG: c_uint = 0x29;
// Vertical Back Porch Status Register
pub const SP_V_B_PORCH_STA_REG: c_uint = 0x2a;
// Total Pixel Status Low Byte Register
pub const SP_TOTAL_PIXEL_STAL_REG: c_uint = 0x2b;
// Total Pixel Status High Byte Register
pub const SP_TOTAL_PIXEL_STAH_REG: c_uint = 0x2c;
// Active Pixel Status Low Byte Register
pub const SP_ACT_PIXEL_STAL_REG: c_uint = 0x2d;
// Active Pixel Status High Byte Register
pub const SP_ACT_PIXEL_STAH_REG: c_uint = 0x2e;
// Horizontal Front Porch Status Low Byte Register
pub const SP_H_F_PORCH_STAL_REG: c_uint = 0x2f;
// Horizontal Front Porch Statys High Byte Register
pub const SP_H_F_PORCH_STAH_REG: c_uint = 0x30;
// Horizontal SYNC Width Status Low Byte Register
pub const SP_H_SYNC_STAL_REG: c_uint = 0x31;
// Horizontal SYNC Width Status High Byte Register
pub const SP_H_SYNC_STAH_REG: c_uint = 0x32;
// Horizontal Back Porch Status Low Byte Register
pub const SP_H_B_PORCH_STAL_REG: c_uint = 0x33;
// Horizontal Back Porch Status High Byte Register
pub const SP_H_B_PORCH_STAH_REG: c_uint = 0x34;
// InfoFrame AVI Packet DB1 Register
pub const SP_INFOFRAME_AVI_DB1_REG: c_uint = 0x70;
// Bit Control Specific Register
pub const SP_BIT_CTRL_SPECIFIC_REG: c_uint = 0x80;
pub const SP_BIT_CTRL_SELECT_SHIFT: c_int = 1;

// InfoFrame Audio Packet DB1 Register
pub const SP_INFOFRAME_AUD_DB1_REG: c_uint = 0x83;
// InfoFrame MPEG Packet DB1 Register
pub const SP_INFOFRAME_MPEG_DB1_REG: c_uint = 0xb0;
// Audio Channel Status Registers
pub const SP_AUD_CH_STATUS_BASE: c_uint = 0xd0;
// Audio Channel Num Register 5
pub const SP_I2S_CHANNEL_NUM_MASK: c_uint = 0xe0;

// Analog Debug Register 1
pub const SP_ANALOG_DEBUG1_REG: c_uint = 0xdc;
// Analog Debug Register 2
pub const SP_ANALOG_DEBUG2_REG: c_uint = 0xdd;
pub const SP_FORCE_SW_OFF_BYPASS: c_uint = 0x20;
pub const SP_XTAL_FRQ: c_uint = 0x1c;

pub const SP_POWERON_TIME_1P5MS: c_uint = 0x03;
// Analog Control 0 Register
pub const SP_ANALOG_CTRL0_REG: c_uint = 0xe1;
// Common Interrupt Status Register 1

pub const SP_PLL_LOCK_CHG: c_uint = 0x40;
// Common Interrupt Status Register 2
pub const SP_COMMON_INT_STATUS2: c_uint = 0xf2;

// Common Interrupt Status Register 4
pub const SP_COMMON_INT_STATUS4_REG: c_uint = 0xf4;

// DP Interrupt Status Register
pub const SP_DP_INT_STATUS1_REG: c_uint = 0xf7;

// Common Interrupt Mask Register

pub const SP_COMMON_INT_MASK4_REG: c_uint = 0xfb;
// DP Interrupts Mask Register
pub const SP_DP_INT_MASK1_REG: c_uint = 0xfe;
// Interrupt Control Register
pub const SP_INT_CTRL_REG: c_uint = 0xff;
