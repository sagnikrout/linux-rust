//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra20_spdif.h
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
// tegra20_spdif.h - Definitions for Tegra20 SPDIF driver
//
// Author: Stephen Warren <swarren@nvidia.com>
// Copyright (C) 2011 - NVIDIA, Inc.
//
// Based on code copyright/by:
// Copyright (c) 2008-2009, NVIDIA Corporation
//

// Offsets from TEGRA20_SPDIF_BASE
pub const TEGRA20_SPDIF_CTRL: c_uint = 0x0;
pub const TEGRA20_SPDIF_STATUS: c_uint = 0x4;
pub const TEGRA20_SPDIF_STROBE_CTRL: c_uint = 0x8;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR: c_uint = 0x0C;
pub const TEGRA20_SPDIF_DATA_OUT: c_uint = 0x40;
pub const TEGRA20_SPDIF_DATA_IN: c_uint = 0x80;
pub const TEGRA20_SPDIF_CH_STA_RX_A: c_uint = 0x100;
pub const TEGRA20_SPDIF_CH_STA_RX_B: c_uint = 0x104;
pub const TEGRA20_SPDIF_CH_STA_RX_C: c_uint = 0x108;
pub const TEGRA20_SPDIF_CH_STA_RX_D: c_uint = 0x10C;
pub const TEGRA20_SPDIF_CH_STA_RX_E: c_uint = 0x110;
pub const TEGRA20_SPDIF_CH_STA_RX_F: c_uint = 0x114;
pub const TEGRA20_SPDIF_CH_STA_TX_A: c_uint = 0x140;
pub const TEGRA20_SPDIF_CH_STA_TX_B: c_uint = 0x144;
pub const TEGRA20_SPDIF_CH_STA_TX_C: c_uint = 0x148;
pub const TEGRA20_SPDIF_CH_STA_TX_D: c_uint = 0x14C;
pub const TEGRA20_SPDIF_CH_STA_TX_E: c_uint = 0x150;
pub const TEGRA20_SPDIF_CH_STA_TX_F: c_uint = 0x154;
pub const TEGRA20_SPDIF_USR_STA_RX_A: c_uint = 0x180;
pub const TEGRA20_SPDIF_USR_DAT_TX_A: c_uint = 0x1C0;
// Fields in TEGRA20_SPDIF_CTRL
// Start capturing from 0=right, 1=left channel

// SPDIF receiver(RX) enable

// SPDIF Transmitter(TX) enable

// Transmit Channel status

// Transmit user Data

// Interrupt on transmit error

// Interrupt on receive error

// Interrupt on invalid preamble

// Interrupt on "B" preamble

// Interrupt when block of channel status received

// Interrupt when a valid information unit (IU) is received

// Interrupt when RX user FIFO attention level is reached

// Interrupt when TX user FIFO attention level is reached

// Interrupt when RX data FIFO attention level is reached

// Interrupt when TX data FIFO attention level is reached

// Loopback test mode enable

//
// Pack data mode:
// 0 = Single data (16 bit needs to be  padded to match the
// interface data bit size).
// 1 = Packeted left/right channel data into a single word.
//

//
// 00 = 16bit data
// 01 = 20bit data
// 10 = 24bit data
// 11 = raw data
//
pub const TEGRA20_SPDIF_BIT_MODE_16BIT: c_int = 0;
pub const TEGRA20_SPDIF_BIT_MODE_20BIT: c_int = 1;
pub const TEGRA20_SPDIF_BIT_MODE_24BIT: c_int = 2;
pub const TEGRA20_SPDIF_BIT_MODE_RAW: c_int = 3;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT: c_int = 12;

// Fields in TEGRA20_SPDIF_STATUS
//
// Note: IS_P, IS_B, IS_C, and IS_U are sticky bits. Software must
// write a 1 to the corresponding bit location to clear the status.
//
// Receiver(RX) shifter is busy receiving data.
// This bit is asserted when the receiver first locked onto the
// preamble of the data stream after RX_EN is asserted. This bit is
// deasserted when either,
// (a) the end of a frame is reached after RX_EN is deeasserted, or
// (b) the SPDIF data stream becomes inactive.
//

//
// Transmitter(TX) shifter is busy transmitting data.
// This bit is asserted when TX_EN is asserted.
// This bit is deasserted when the end of a frame is reached after
// TX_EN is deasserted.
//

//
// TX is busy shifting out channel status.
// This bit is asserted when both TX_EN and TC_EN are asserted and
// data from CH_STA_TX_A register is loaded into the internal shifter.
// This bit is deasserted when either,
// (a) the end of a frame is reached after TX_EN is deasserted, or
// (b) CH_STA_TX_F register is loaded into the internal shifter.
//

//
// TX User data FIFO busy.
// This bit is asserted when TX_EN and TXU_EN are asserted and
// there's data in the TX user FIFO.  This bit is deassert when either,
// (a) the end of a frame is reached after TX_EN is deasserted, or
// (b) there's no data left in the TX user FIFO.
//

// TX FIFO Underrun error status

// RX FIFO Overrun error status

// Preamble status: 0=Preamble OK, 1=bad/missing preamble

// B-preamble detection status: 0=not detected, 1=B-preamble detected

//
// RX channel block data receive status:
// 0=entire block not received yet.
// 1=received entire block of channel status,
//

// RX User Data Valid flag:  1=valid IU detected, 0 = no IU detected.

//
// RX User FIFO Status:
// 1=attention level reached, 0=attention level not reached.
//

//
// TX User FIFO Status:
// 1=attention level reached, 0=attention level not reached.
//

//
// RX Data FIFO Status:
// 1=attention level reached, 0=attention level not reached.
//

//
// TX Data FIFO Status:
// 1=attention level reached, 0=attention level not reached.
//

// Fields in TEGRA20_SPDIF_STROBE_CTRL
//
// Indicates the approximate number of detected SPDIFIN clocks within a
// bi-phase period.
//
pub const TEGRA20_SPDIF_STROBE_CTRL_PERIOD_SHIFT: c_int = 16;

// Data strobe mode: 0=Auto-locked 1=Manual locked

//
// Manual data strobe time within the bi-phase clock period (in terms of
// the number of over-sampling clocks).
//
pub const TEGRA20_SPDIF_STROBE_CTRL_DATA_STROBES_SHIFT: c_int = 8;

//
// Manual SPDIFIN bi-phase clock period (in terms of the number of
// over-sampling clocks).
//
pub const TEGRA20_SPDIF_STROBE_CTRL_CLOCK_PERIOD_SHIFT: c_int = 0;

// Fields in SPDIF_DATA_FIFO_CSR
// Clear Receiver User FIFO (RX USR.FIFO)

pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_ONE_SLOT: c_int = 0;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_TWO_SLOTS: c_int = 1;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_THREE_SLOTS: c_int = 2;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_FOUR_SLOTS: c_int = 3;
// RU FIFO attention level
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT: c_int = 29;

// Number of RX USR.FIFO levels with valid data.
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_FULL_COUNT_SHIFT: c_int = 24;

// Clear Transmitter User FIFO (TX USR.FIFO)

// TU FIFO attention level
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT: c_int = 21;

// Number of TX USR.FIFO levels that could be filled.
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_EMPTY_COUNT_SHIFT: c_int = 16;

// Clear Receiver Data FIFO (RX DATA.FIFO)

pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_ONE_SLOT: c_int = 0;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_FOUR_SLOTS: c_int = 1;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_EIGHT_SLOTS: c_int = 2;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_TWELVE_SLOTS: c_int = 3;
// RU FIFO attention level
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT: c_int = 13;

// Number of RX DATA.FIFO levels with valid data.
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_FULL_COUNT_SHIFT: c_int = 8;

// Clear Transmitter Data FIFO (TX DATA.FIFO)

// TU FIFO attention level
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT: c_int = 5;

// Number of TX DATA.FIFO levels that could be filled.
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_EMPTY_COUNT_SHIFT: c_int = 0;

// Fields in TEGRA20_SPDIF_DATA_OUT
//
// This register has 5 different formats:
// 16-bit        (BIT_MODE=00, PACK=0)
// 20-bit        (BIT_MODE=01, PACK=0)
// 24-bit        (BIT_MODE=10, PACK=0)
// raw           (BIT_MODE=11, PACK=0)
// 16-bit packed (BIT_MODE=00, PACK=1)
//
pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_20_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_24_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_DATA_SHIFT: c_int = 8;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_AUX_SHIFT: c_int = 4;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_PREAMBLE_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_RIGHT_SHIFT: c_int = 16;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_LEFT_SHIFT: c_int = 0;

// Fields in TEGRA20_SPDIF_DATA_IN
//
// This register has 5 different formats:
// 16-bit        (BIT_MODE=00, PACK=0)
// 20-bit        (BIT_MODE=01, PACK=0)
// 24-bit        (BIT_MODE=10, PACK=0)
// raw           (BIT_MODE=11, PACK=0)
// 16-bit packed (BIT_MODE=00, PACK=1)
//
// Bits 31:24 are common to all modes except 16-bit packed
//

pub const TEGRA20_SPDIF_DATA_IN_DATA_PREAMBLE_SHIFT: c_int = 24;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_IN_DATA_20_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_IN_DATA_24_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_DATA_SHIFT: c_int = 8;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_AUX_SHIFT: c_int = 4;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_PREAMBLE_SHIFT: c_int = 0;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_RIGHT_SHIFT: c_int = 16;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_LEFT_SHIFT: c_int = 0;

// Fields in TEGRA20_SPDIF_CH_STA_RX_A
// Fields in TEGRA20_SPDIF_CH_STA_RX_B
// Fields in TEGRA20_SPDIF_CH_STA_RX_C
// Fields in TEGRA20_SPDIF_CH_STA_RX_D
// Fields in TEGRA20_SPDIF_CH_STA_RX_E
// Fields in TEGRA20_SPDIF_CH_STA_RX_F
//
// The 6-word receive channel data page buffer holds a block (192 frames) of
// channel status information. The order of receive is from LSB to MSB
// bit, and from CH_STA_RX_A to CH_STA_RX_F then back to CH_STA_RX_A.
//
// Fields in TEGRA20_SPDIF_CH_STA_TX_A
// Fields in TEGRA20_SPDIF_CH_STA_TX_B
// Fields in TEGRA20_SPDIF_CH_STA_TX_C
// Fields in TEGRA20_SPDIF_CH_STA_TX_D
// Fields in TEGRA20_SPDIF_CH_STA_TX_E
// Fields in TEGRA20_SPDIF_CH_STA_TX_F
//
// The 6-word transmit channel data page buffer holds a block (192 frames) of
// channel status information. The order of transmission is from LSB to MSB
// bit, and from CH_STA_TX_A to CH_STA_TX_F then back to CH_STA_TX_A.
//
// Fields in TEGRA20_SPDIF_USR_STA_RX_A
//
// This 4-word deep FIFO receives user FIFO field information. The order of
// receive is from LSB to MSB bit.
//
// Fields in TEGRA20_SPDIF_USR_DAT_TX_A
//
// This 4-word deep FIFO transmits user FIFO field information. The order of
// transmission is from LSB to MSB bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra20_spdif {
    pub clk_spdif_out: *mut clk,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub regmap: *mut regmap,
    pub reset: *mut reset_control,
}
