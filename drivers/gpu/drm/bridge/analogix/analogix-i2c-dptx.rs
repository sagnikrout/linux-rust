//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/analogix-i2c-dptx.h
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
// Copyright(c) 2016, Analogix Semiconductor.
//
// Based on anx7808 driver obtained from chromeos with copyright:
// Copyright(c) 2013, Google Inc.
//
// Register definitions for TX_P0
//
// HDCP Status Register
pub const SP_TX_HDCP_STATUS_REG: c_uint = 0x00;

// HDCP Control Register 0
pub const SP_HDCP_CTRL0_REG: c_uint = 0x01;

// HDCP Function Enabled

// HDCP Receiver BSTATUS Register 0
pub const SP_HDCP_RX_BSTATUS0_REG: c_uint = 0x1b;
// HDCP Receiver BSTATUS Register 1
pub const SP_HDCP_RX_BSTATUS1_REG: c_uint = 0x1c;
// HDCP Embedded "Blue Screen" Content Registers
pub const SP_HDCP_VID0_BLUE_SCREEN_REG: c_uint = 0x2c;
pub const SP_HDCP_VID1_BLUE_SCREEN_REG: c_uint = 0x2d;
pub const SP_HDCP_VID2_BLUE_SCREEN_REG: c_uint = 0x2e;
// HDCP Wait R0 Timing Register
pub const SP_HDCP_WAIT_R0_TIME_REG: c_uint = 0x40;
// HDCP Link Integrity Check Timer Register
pub const SP_HDCP_LINK_CHECK_TIMER_REG: c_uint = 0x41;
// HDCP Repeater Ready Wait Timer Register
pub const SP_HDCP_RPTR_RDY_WAIT_TIME_REG: c_uint = 0x42;
// HDCP Auto Timer Register
pub const SP_HDCP_AUTO_TIMER_REG: c_uint = 0x51;
// HDCP Key Status Register
pub const SP_HDCP_KEY_STATUS_REG: c_uint = 0x5e;
// HDCP Key Command Register
pub const SP_HDCP_KEY_COMMAND_REG: c_uint = 0x5f;

// OTP Memory Key Protection Registers
pub const SP_OTP_KEY_PROTECT1_REG: c_uint = 0x60;
pub const SP_OTP_KEY_PROTECT2_REG: c_uint = 0x61;
pub const SP_OTP_KEY_PROTECT3_REG: c_uint = 0x62;
pub const SP_OTP_PSW1: c_uint = 0xa2;
pub const SP_OTP_PSW2: c_uint = 0x7e;
pub const SP_OTP_PSW3: c_uint = 0xc6;
// DP System Control Registers

// Bits for DP System Control Register 2

// Bits for DP System Control Register 3

// Bits for DP System Control Register 4

// DP Video Control Register
pub const SP_DP_VIDEO_CTRL_REG: c_uint = 0x84;
pub const SP_COLOR_F_MASK: c_uint = 0x06;
pub const SP_COLOR_F_SHIFT: c_int = 1;
pub const SP_BPC_MASK: c_uint = 0xe0;
pub const SP_BPC_SHIFT: c_int = 5;

// DP Audio Control Register
pub const SP_DP_AUDIO_CTRL_REG: c_uint = 0x87;

// 10us Pulse Generate Timer Registers
pub const SP_I2C_GEN_10US_TIMER0_REG: c_uint = 0x88;
pub const SP_I2C_GEN_10US_TIMER1_REG: c_uint = 0x89;
// Packet Send Control Register
pub const SP_PACKET_SEND_CTRL_REG: c_uint = 0x90;

// DP HDCP Control Register
pub const SP_DP_HDCP_CTRL_REG: c_uint = 0x92;

// DP Main Link Bandwidth Setting Register
pub const SP_DP_MAIN_LINK_BW_SET_REG: c_uint = 0xa0;
pub const SP_LINK_BW_SET_MASK: c_uint = 0x1f;

// DP Lane Count Setting Register
pub const SP_DP_LANE_COUNT_SET_REG: c_uint = 0xa1;
// DP Training Pattern Set Register
pub const SP_DP_TRAINING_PATTERN_SET_REG: c_uint = 0xa2;
// DP Lane 0 Link Training Control Register
pub const SP_DP_LANE0_LT_CTRL_REG: c_uint = 0xa3;
pub const SP_TX_SW_SET_MASK: c_uint = 0x1b;

// DP Link Training Control Register
pub const SP_DP_LT_CTRL_REG: c_uint = 0xa8;
pub const SP_DP_LT_INPROGRESS: c_uint = 0x80;
pub const SP_LT_ERROR_TYPE_MASK: c_uint = 0x70;

// DP CEP Training Control Registers
pub const SP_DP_CEP_TRAINING_CTRL0_REG: c_uint = 0xa9;
pub const SP_DP_CEP_TRAINING_CTRL1_REG: c_uint = 0xaa;
// DP Debug Register 1
pub const SP_DP_DEBUG1_REG: c_uint = 0xb0;

// DP Polling Control Register
pub const SP_DP_POLLING_CTRL_REG: c_uint = 0xb4;

// DP Link Debug Control Register
pub const SP_DP_LINK_DEBUG_CTRL_REG: c_uint = 0xb8;

// AUX Misc control Register
pub const SP_AUX_MISC_CTRL_REG: c_uint = 0xbf;
// DP PLL control Register
pub const SP_DP_PLL_CTRL_REG: c_uint = 0xc7;

// DP Analog Power Down Register
pub const SP_DP_ANALOG_POWER_DOWN_REG: c_uint = 0xc8;

// DP Misc Control Register
pub const SP_DP_MISC_CTRL_REG: c_uint = 0xcd;

// DP Extra I2C Device Address Register
pub const SP_DP_EXTRA_I2C_DEV_ADDR_REG: c_uint = 0xce;

pub const SP_I2C_EXTRA_ADDR: c_uint = 0x50;
// DP Downspread Control Register 1
pub const SP_DP_DOWNSPREAD_CTRL1_REG: c_uint = 0xd0;
// DP M Value Calculation Control Register
pub const SP_DP_M_CALCULATION_CTRL_REG: c_uint = 0xd9;

// AUX Channel Access Status Register
pub const SP_AUX_CH_STATUS_REG: c_uint = 0xe0;
pub const SP_AUX_STATUS: c_uint = 0x0f;
// AUX Channel DEFER Control Register
pub const SP_AUX_DEFER_CTRL_REG: c_uint = 0xe2;

// DP Buffer Data Count Register
pub const SP_BUF_DATA_COUNT_REG: c_uint = 0xe4;
pub const SP_BUF_DATA_COUNT_MASK: c_uint = 0x1f;

// DP AUX Channel Control Register 1
pub const SP_DP_AUX_CH_CTRL1_REG: c_uint = 0xe5;
pub const SP_AUX_TX_COMM_MASK: c_uint = 0x0f;
pub const SP_AUX_LENGTH_MASK: c_uint = 0xf0;
pub const SP_AUX_LENGTH_SHIFT: c_int = 4;
// DP AUX CH Address Register 0
pub const SP_AUX_ADDR_7_0_REG: c_uint = 0xe6;
// DP AUX CH Address Register 1
pub const SP_AUX_ADDR_15_8_REG: c_uint = 0xe7;
// DP AUX CH Address Register 2
pub const SP_AUX_ADDR_19_16_REG: c_uint = 0xe8;
pub const SP_AUX_ADDR_19_16_MASK: c_uint = 0x0f;
// DP AUX Channel Control Register 2
pub const SP_DP_AUX_CH_CTRL2_REG: c_uint = 0xe9;

// DP Video Stream Control InfoFrame Register
pub const SP_DP_3D_VSC_CTRL_REG: c_uint = 0xea;

// DP Video Stream Data Byte 1 Register
pub const SP_DP_VSC_DB1_REG: c_uint = 0xeb;
// DP AUX Channel Control Register 3
pub const SP_DP_AUX_CH_CTRL3_REG: c_uint = 0xec;
pub const SP_WAIT_COUNTER_7_0_MASK: c_uint = 0xff;
// DP AUX Channel Control Register 4
pub const SP_DP_AUX_CH_CTRL4_REG: c_uint = 0xed;
// DP AUX Buffer Data Registers
pub const SP_DP_BUF_DATA0_REG: c_uint = 0xf0;
