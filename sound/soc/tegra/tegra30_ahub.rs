//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra30_ahub.h
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
// tegra30_ahub.h - Definitions for Tegra30 AHUB driver
//
// Copyright (c) 2011,2012, NVIDIA CORPORATION.  All rights reserved.
//
// Fields in *_CIF_RX/TX_CTRL; used by AHUB FIFOs, and all other audio modules
pub const TEGRA30_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT: c_int = 28;
pub const TEGRA30_AUDIOCIF_CTRL_FIFO_THRESHOLD_MASK_US: c_uint = 0xf;

pub const TEGRA124_AUDIOCIF_CTRL_FIFO_THRESHOLD_SHIFT: c_int = 24;
pub const TEGRA124_AUDIOCIF_CTRL_FIFO_THRESHOLD_MASK_US: c_uint = 0x3f;

// Channel count minus 1
pub const TEGRA30_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT: c_int = 24;
pub const TEGRA30_AUDIOCIF_CTRL_AUDIO_CHANNELS_MASK_US: c_int = 7;

// Channel count minus 1
pub const TEGRA124_AUDIOCIF_CTRL_AUDIO_CHANNELS_SHIFT: c_int = 20;
pub const TEGRA124_AUDIOCIF_CTRL_AUDIO_CHANNELS_MASK_US: c_uint = 0xf;

// Channel count minus 1
pub const TEGRA30_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT: c_int = 16;
pub const TEGRA30_AUDIOCIF_CTRL_CLIENT_CHANNELS_MASK_US: c_int = 7;

// Channel count minus 1
pub const TEGRA124_AUDIOCIF_CTRL_CLIENT_CHANNELS_SHIFT: c_int = 16;
pub const TEGRA124_AUDIOCIF_CTRL_CLIENT_CHANNELS_MASK_US: c_uint = 0xf;

pub const TEGRA30_AUDIOCIF_BITS_4: c_int = 0;
pub const TEGRA30_AUDIOCIF_BITS_8: c_int = 1;
pub const TEGRA30_AUDIOCIF_BITS_12: c_int = 2;
pub const TEGRA30_AUDIOCIF_BITS_16: c_int = 3;
pub const TEGRA30_AUDIOCIF_BITS_20: c_int = 4;
pub const TEGRA30_AUDIOCIF_BITS_24: c_int = 5;
pub const TEGRA30_AUDIOCIF_BITS_28: c_int = 6;
pub const TEGRA30_AUDIOCIF_BITS_32: c_int = 7;
pub const TEGRA30_AUDIOCIF_CTRL_AUDIO_BITS_SHIFT: c_int = 12;

pub const TEGRA30_AUDIOCIF_CTRL_CLIENT_BITS_SHIFT: c_int = 8;

pub const TEGRA30_AUDIOCIF_EXPAND_ZERO: c_int = 0;
pub const TEGRA30_AUDIOCIF_EXPAND_ONE: c_int = 1;
pub const TEGRA30_AUDIOCIF_EXPAND_LFSR: c_int = 2;
pub const TEGRA30_AUDIOCIF_CTRL_EXPAND_SHIFT: c_int = 6;

pub const TEGRA30_AUDIOCIF_STEREO_CONV_CH0: c_int = 0;
pub const TEGRA30_AUDIOCIF_STEREO_CONV_CH1: c_int = 1;
pub const TEGRA30_AUDIOCIF_STEREO_CONV_AVG: c_int = 2;
pub const TEGRA30_AUDIOCIF_CTRL_STEREO_CONV_SHIFT: c_int = 4;

pub const TEGRA30_AUDIOCIF_CTRL_REPLICATE_SHIFT: c_int = 3;
pub const TEGRA30_AUDIOCIF_DIRECTION_TX: c_int = 0;
pub const TEGRA30_AUDIOCIF_DIRECTION_RX: c_int = 1;
pub const TEGRA30_AUDIOCIF_CTRL_DIRECTION_SHIFT: c_int = 2;

pub const TEGRA30_AUDIOCIF_TRUNCATE_ROUND: c_int = 0;
pub const TEGRA30_AUDIOCIF_TRUNCATE_CHOP: c_int = 1;
pub const TEGRA30_AUDIOCIF_CTRL_TRUNCATE_SHIFT: c_int = 1;

pub const TEGRA30_AUDIOCIF_MONO_CONV_ZERO: c_int = 0;
pub const TEGRA30_AUDIOCIF_MONO_CONV_COPY: c_int = 1;
pub const TEGRA30_AUDIOCIF_CTRL_MONO_CONV_SHIFT: c_int = 0;

// Registers within TEGRA30_AUDIO_CLUSTER_BASE
// TEGRA30_AHUB_CHANNEL_CTRL
pub const TEGRA30_AHUB_CHANNEL_CTRL: c_uint = 0x0;
pub const TEGRA30_AHUB_CHANNEL_CTRL_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CHANNEL_CTRL_COUNT: c_int = 4;

pub const TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_SHIFT: c_int = 16;
pub const TEGRA30_AHUB_CHANNEL_CTRL_TX_THRESHOLD_MASK_US: c_uint = 0xff;

pub const TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_SHIFT: c_int = 8;
pub const TEGRA30_AHUB_CHANNEL_CTRL_RX_THRESHOLD_MASK_US: c_uint = 0xff;

pub const TEGRA30_PACK_8_4: c_int = 2;
pub const TEGRA30_PACK_16: c_int = 3;
pub const TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_SHIFT: c_int = 4;
pub const TEGRA30_AHUB_CHANNEL_CTRL_TX_PACK_MASK_US: c_int = 3;

pub const TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_SHIFT: c_int = 0;
pub const TEGRA30_AHUB_CHANNEL_CTRL_RX_PACK_MASK_US: c_int = 3;

// TEGRA30_AHUB_CHANNEL_CLEAR
pub const TEGRA30_AHUB_CHANNEL_CLEAR: c_uint = 0x4;
pub const TEGRA30_AHUB_CHANNEL_CLEAR_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CHANNEL_CLEAR_COUNT: c_int = 4;

// TEGRA30_AHUB_CHANNEL_STATUS
pub const TEGRA30_AHUB_CHANNEL_STATUS: c_uint = 0x8;
pub const TEGRA30_AHUB_CHANNEL_STATUS_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CHANNEL_STATUS_COUNT: c_int = 4;
pub const TEGRA30_AHUB_CHANNEL_STATUS_TX_FREE_SHIFT: c_int = 24;
pub const TEGRA30_AHUB_CHANNEL_STATUS_TX_FREE_MASK_US: c_uint = 0xff;

pub const TEGRA30_AHUB_CHANNEL_STATUS_RX_FREE_SHIFT: c_int = 16;
pub const TEGRA30_AHUB_CHANNEL_STATUS_RX_FREE_MASK_US: c_uint = 0xff;

// TEGRA30_AHUB_CHANNEL_TXFIFO
pub const TEGRA30_AHUB_CHANNEL_TXFIFO: c_uint = 0xc;
pub const TEGRA30_AHUB_CHANNEL_TXFIFO_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CHANNEL_TXFIFO_COUNT: c_int = 4;
// TEGRA30_AHUB_CHANNEL_RXFIFO
pub const TEGRA30_AHUB_CHANNEL_RXFIFO: c_uint = 0x10;
pub const TEGRA30_AHUB_CHANNEL_RXFIFO_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CHANNEL_RXFIFO_COUNT: c_int = 4;
// TEGRA30_AHUB_CIF_TX_CTRL
pub const TEGRA30_AHUB_CIF_TX_CTRL: c_uint = 0x14;
pub const TEGRA30_AHUB_CIF_TX_CTRL_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CIF_TX_CTRL_COUNT: c_int = 4;
// Uses field from TEGRA30_AUDIOCIF_CTRL_*
// TEGRA30_AHUB_CIF_RX_CTRL
pub const TEGRA30_AHUB_CIF_RX_CTRL: c_uint = 0x18;
pub const TEGRA30_AHUB_CIF_RX_CTRL_STRIDE: c_uint = 0x20;
pub const TEGRA30_AHUB_CIF_RX_CTRL_COUNT: c_int = 4;
// Uses field from TEGRA30_AUDIOCIF_CTRL_*
// TEGRA30_AHUB_CONFIG_LINK_CTRL
pub const TEGRA30_AHUB_CONFIG_LINK_CTRL: c_uint = 0x80;
pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_MASTER_FIFO_FULL_CNT_SHIFT: c_int = 28;
pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_MASTER_FIFO_FULL_CNT_MASK_US: c_uint = 0xf;

pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_TIMEOUT_CNT_SHIFT: c_int = 16;
pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_TIMEOUT_CNT_MASK_US: c_uint = 0xfff;

pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_IDLE_CNT_SHIFT: c_int = 4;
pub const TEGRA30_AHUB_CONFIG_LINK_CTRL_IDLE_CNT_MASK_US: c_uint = 0xfff;

// TEGRA30_AHUB_MISC_CTRL
pub const TEGRA30_AHUB_MISC_CTRL: c_uint = 0x84;

pub const TEGRA30_AHUB_MISC_CTRL_AUDIO_OBS_SEL_SHIFT: c_int = 0;

// TEGRA30_AHUB_APBDMA_LIVE_STATUS
pub const TEGRA30_AHUB_APBDMA_LIVE_STATUS: c_uint = 0x88;

// TEGRA30_AHUB_I2S_LIVE_STATUS
pub const TEGRA30_AHUB_I2S_LIVE_STATUS: c_uint = 0x8c;

// TEGRA30_AHUB_DAM0_LIVE_STATUS
pub const TEGRA30_AHUB_DAM_LIVE_STATUS: c_uint = 0x90;
pub const TEGRA30_AHUB_DAM_LIVE_STATUS_STRIDE: c_uint = 0x8;
pub const TEGRA30_AHUB_DAM_LIVE_STATUS_COUNT: c_int = 3;

// TEGRA30_AHUB_SPDIF_LIVE_STATUS
pub const TEGRA30_AHUB_SPDIF_LIVE_STATUS: c_uint = 0xa8;

// TEGRA30_AHUB_I2S_INT_MASK
pub const TEGRA30_AHUB_I2S_INT_MASK: c_uint = 0xb0;
// TEGRA30_AHUB_DAM_INT_MASK
pub const TEGRA30_AHUB_DAM_INT_MASK: c_uint = 0xb4;
// TEGRA30_AHUB_SPDIF_INT_MASK
pub const TEGRA30_AHUB_SPDIF_INT_MASK: c_uint = 0xbc;
// TEGRA30_AHUB_APBIF_INT_MASK
pub const TEGRA30_AHUB_APBIF_INT_MASK: c_uint = 0xc0;
// TEGRA30_AHUB_I2S_INT_STATUS
pub const TEGRA30_AHUB_I2S_INT_STATUS: c_uint = 0xc8;
// TEGRA30_AHUB_DAM_INT_STATUS
pub const TEGRA30_AHUB_DAM_INT_STATUS: c_uint = 0xcc;
// TEGRA30_AHUB_SPDIF_INT_STATUS
pub const TEGRA30_AHUB_SPDIF_INT_STATUS: c_uint = 0xd4;
// TEGRA30_AHUB_APBIF_INT_STATUS
pub const TEGRA30_AHUB_APBIF_INT_STATUS: c_uint = 0xd8;
// TEGRA30_AHUB_I2S_INT_SOURCE
pub const TEGRA30_AHUB_I2S_INT_SOURCE: c_uint = 0xe0;
// TEGRA30_AHUB_DAM_INT_SOURCE
pub const TEGRA30_AHUB_DAM_INT_SOURCE: c_uint = 0xe4;
// TEGRA30_AHUB_SPDIF_INT_SOURCE
pub const TEGRA30_AHUB_SPDIF_INT_SOURCE: c_uint = 0xec;
// TEGRA30_AHUB_APBIF_INT_SOURCE
pub const TEGRA30_AHUB_APBIF_INT_SOURCE: c_uint = 0xf0;
// TEGRA30_AHUB_I2S_INT_SET
pub const TEGRA30_AHUB_I2S_INT_SET: c_uint = 0xf8;
// TEGRA30_AHUB_DAM_INT_SET
pub const TEGRA30_AHUB_DAM_INT_SET: c_uint = 0xfc;
// TEGRA30_AHUB_SPDIF_INT_SET
pub const TEGRA30_AHUB_SPDIF_INT_SET: c_uint = 0x100;
// TEGRA30_AHUB_APBIF_INT_SET
pub const TEGRA30_AHUB_APBIF_INT_SET: c_uint = 0x104;
// Registers within TEGRA30_AHUB_BASE
pub const TEGRA30_AHUB_AUDIO_RX: c_uint = 0x0;
pub const TEGRA30_AHUB_AUDIO_RX_STRIDE: c_uint = 0x4;
pub const TEGRA30_AHUB_AUDIO_RX_COUNT: c_int = 17;
// This register repeats once for each entry in enum tegra30_ahub_rxcif
// The fields in this register are 1 bit per entry in tegra30_ahub_txcif
//
// Terminology:
// AHUB: Audio Hub; a cross-bar switch between the audio devices: DMA FIFOs,
// I2S controllers, SPDIF controllers, and DAMs.
// XBAR: The core cross-bar component of the AHUB.
// CIF:  Client Interface; the HW module connecting an audio device to the
// XBAR.
// DAM:  Digital Audio Mixer: A HW module that mixes multiple audio streams,
// possibly including sample-rate conversion.
//
// Each TX CIF transmits data into the XBAR. Each RX CIF can receive audio
// transmitted by a particular TX CIF.
//
// This driver is currently very simplistic; many HW features are not
// exposed; DAMs are not supported, only 16-bit stereo audio is supported,
// etc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra30_ahub_txcif {
    TEGRA30_AHUB_TXCIF_APBIF_TX0,
    TEGRA30_AHUB_TXCIF_APBIF_TX1,
    TEGRA30_AHUB_TXCIF_APBIF_TX2,
    TEGRA30_AHUB_TXCIF_APBIF_TX3,
    TEGRA30_AHUB_TXCIF_I2S0_TX0,
    TEGRA30_AHUB_TXCIF_I2S1_TX0,
    TEGRA30_AHUB_TXCIF_I2S2_TX0,
    TEGRA30_AHUB_TXCIF_I2S3_TX0,
    TEGRA30_AHUB_TXCIF_I2S4_TX0,
    TEGRA30_AHUB_TXCIF_DAM0_TX0,
    TEGRA30_AHUB_TXCIF_DAM1_TX0,
    TEGRA30_AHUB_TXCIF_DAM2_TX0,
    TEGRA30_AHUB_TXCIF_SPDIF_TX0,
    TEGRA30_AHUB_TXCIF_SPDIF_TX1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra30_ahub_rxcif {
    TEGRA30_AHUB_RXCIF_APBIF_RX0,
    TEGRA30_AHUB_RXCIF_APBIF_RX1,
    TEGRA30_AHUB_RXcIF_APBIF_RX2,
    TEGRA30_AHUB_RXCIF_APBIF_RX3,
    TEGRA30_AHUB_RXCIF_I2S0_RX0,
    TEGRA30_AHUB_RXCIF_I2S1_RX0,
    TEGRA30_AHUB_RXCIF_I2S2_RX0,
    TEGRA30_AHUB_RXCIF_I2S3_RX0,
    TEGRA30_AHUB_RXCIF_I2S4_RX0,
    TEGRA30_AHUB_RXCIF_DAM0_RX0,
    TEGRA30_AHUB_RXCIF_DAM0_RX1,
    TEGRA30_AHUB_RXCIF_DAM1_RX0,
    TEGRA30_AHUB_RXCIF_DAM2_RX1,
    TEGRA30_AHUB_RXCIF_DAM3_RX0,
    TEGRA30_AHUB_RXCIF_DAM3_RX1,
    TEGRA30_AHUB_RXCIF_SPDIF_RX0,
    TEGRA30_AHUB_RXCIF_SPDIF_RX1,
}

extern "C" {
    pub fn tegra30_ahub_enable_rx_fifo(rxcif: tegra30_ahub_rxcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_disable_rx_fifo(rxcif: tegra30_ahub_rxcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_free_rx_fifo(rxcif: tegra30_ahub_rxcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_enable_tx_fifo(txcif: tegra30_ahub_txcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_disable_tx_fifo(txcif: tegra30_ahub_txcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_free_tx_fifo(txcif: tegra30_ahub_txcif) -> c_int;
}
extern "C" {
    pub fn tegra30_ahub_unset_rx_cif_source(rxcif: tegra30_ahub_rxcif) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra30_ahub_cif_conf {
    pub threshold: c_uint,
    pub audio_channels: c_uint,
    pub client_channels: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
    pub expand: c_uint,
    pub stereo_conv: c_uint,
    pub replicate: c_uint,
    pub direction: c_uint,
    pub truncate: c_uint,
    pub mono_conv: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra30_ahub_soc_data {
    pub num_resets: c_uint,
    pub conf): *mut tegra30_ahub_cif_conf,
//
// FIXME: There are many more differences in HW, such as:
// - More APBIF channels.
// - Extra separate chunks of register address space to represent
// the extra APBIF channels.
// - More units connected to the AHUB, so that tegra30_ahub_[rt]xcif
// need expansion, coupled with there being more defined bits in
// the AHUB routing registers.
// However, the driver doesn't support those new features yet, so we
// don't represent them here yet.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra30_ahub {
    pub soc_data: *const tegra30_ahub_soc_data,
    pub dev: *mut device,
    pub resets: [reset_control_bulk_data; 21],
    pub nresets: c_uint,
    pub clocks: [clk_bulk_data; 2],
    pub nclocks: c_uint,
    pub apbif_addr: resource_size_t,
    pub regmap_apbif: *mut regmap,
    pub regmap_ahub: *mut regmap,
    pub TEGRA30_AHUB_CHANNEL_CTRL_COUNT): DECLARE_BITMAP(rx_usage,,
    pub TEGRA30_AHUB_CHANNEL_CTRL_COUNT): DECLARE_BITMAP(tx_usage,,
}
