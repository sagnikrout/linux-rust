//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra210_admaif.h
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
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_admaif.h - Tegra ADMAIF registers
//
pub const TEGRA_ADMAIF_CHANNEL_REG_STRIDE: c_uint = 0x40;
// Tegra210 specific
pub const TEGRA210_ADMAIF_LAST_REG: c_uint = 0x75f;
pub const TEGRA210_ADMAIF_CHANNEL_COUNT: c_int = 10;
pub const TEGRA210_ADMAIF_RX_BASE: c_uint = 0x0;
pub const TEGRA210_ADMAIF_TX_BASE: c_uint = 0x300;
pub const TEGRA210_ADMAIF_GLOBAL_BASE: c_uint = 0x700;
pub const TEGRA210_ADMAIF_MAX_CHANNEL: c_int = 16;
// Tegra186 specific
pub const TEGRA186_ADMAIF_LAST_REG: c_uint = 0xd5f;
pub const TEGRA186_ADMAIF_CHANNEL_COUNT: c_int = 20;
pub const TEGRA186_ADMAIF_RX_BASE: c_uint = 0x0;
pub const TEGRA186_ADMAIF_TX_BASE: c_uint = 0x500;
pub const TEGRA186_ADMAIF_GLOBAL_BASE: c_uint = 0xd00;
pub const TEGRA186_ADMAIF_MAX_CHANNEL: c_int = 16;
// Tegra264 specific
pub const TEGRA264_ADMAIF_LAST_REG: c_uint = 0x205f;
pub const TEGRA264_ADMAIF_CHANNEL_COUNT: c_int = 32;
pub const TEGRA264_ADMAIF_RX_BASE: c_uint = 0x0;
pub const TEGRA264_ADMAIF_TX_BASE: c_uint = 0x1000;
pub const TEGRA264_ADMAIF_GLOBAL_BASE: c_uint = 0x2000;
pub const TEGRA264_ADMAIF_MAX_CHANNEL: c_int = 32;
// Global registers
pub const TEGRA_ADMAIF_GLOBAL_ENABLE: c_uint = 0x0;
pub const TEGRA_ADMAIF_GLOBAL_CG_0: c_uint = 0x8;
pub const TEGRA_ADMAIF_GLOBAL_STATUS: c_uint = 0x10;
pub const TEGRA_ADMAIF_GLOBAL_RX_ENABLE_STATUS: c_uint = 0x20;
pub const TEGRA_ADMAIF_GLOBAL_TX_ENABLE_STATUS: c_uint = 0x24;
// RX channel registers
pub const TEGRA_ADMAIF_RX_ENABLE: c_uint = 0x0;
pub const TEGRA_ADMAIF_RX_SOFT_RESET: c_uint = 0x4;
pub const TEGRA_ADMAIF_RX_STATUS: c_uint = 0xc;
pub const TEGRA_ADMAIF_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA_ADMAIF_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA_ADMAIF_RX_INT_SET: c_uint = 0x18;
pub const TEGRA_ADMAIF_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA_ADMAIF_CH_ACIF_RX_CTRL: c_uint = 0x20;
pub const TEGRA_ADMAIF_RX_FIFO_CTRL: c_uint = 0x28;
pub const TEGRA_ADMAIF_RX_FIFO_READ: c_uint = 0x2c;
// TX channel registers
pub const TEGRA_ADMAIF_TX_ENABLE: c_uint = 0x0;
pub const TEGRA_ADMAIF_TX_SOFT_RESET: c_uint = 0x4;
pub const TEGRA_ADMAIF_TX_STATUS: c_uint = 0xc;
pub const TEGRA_ADMAIF_TX_INT_STATUS: c_uint = 0x10;
pub const TEGRA_ADMAIF_TX_INT_MASK: c_uint = 0x14;
pub const TEGRA_ADMAIF_TX_INT_SET: c_uint = 0x18;
pub const TEGRA_ADMAIF_TX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA_ADMAIF_CH_ACIF_TX_CTRL: c_uint = 0x20;
pub const TEGRA_ADMAIF_TX_FIFO_CTRL: c_uint = 0x28;
pub const TEGRA_ADMAIF_TX_FIFO_WRITE: c_uint = 0x2c;
// Bit fields
pub const PACK8_EN_SHIFT: c_int = 31;

pub const PACK16_EN_SHIFT: c_int = 30;

pub const TX_ENABLE_SHIFT: c_int = 0;

pub const RX_ENABLE_SHIFT: c_int = 0;

pub const SW_RESET_MASK: c_int = 1;
pub const SW_RESET: c_int = 1;
// Default values - Tegra210
pub const TEGRA210_ADMAIF_CIF_REG_DEFAULT: c_uint = 0x00007700;
pub const TEGRA210_ADMAIF_RX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000300;
pub const TEGRA210_ADMAIF_RX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000304;
pub const TEGRA210_ADMAIF_RX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000208;
pub const TEGRA210_ADMAIF_RX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000020b;
pub const TEGRA210_ADMAIF_RX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000020e;
pub const TEGRA210_ADMAIF_RX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000211;
pub const TEGRA210_ADMAIF_RX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000214;
pub const TEGRA210_ADMAIF_RX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000217;
pub const TEGRA210_ADMAIF_RX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021a;
pub const TEGRA210_ADMAIF_RX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021d;
pub const TEGRA210_ADMAIF_TX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x02000300;
pub const TEGRA210_ADMAIF_TX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x02000304;
pub const TEGRA210_ADMAIF_TX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800208;
pub const TEGRA210_ADMAIF_TX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180020b;
pub const TEGRA210_ADMAIF_TX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180020e;
pub const TEGRA210_ADMAIF_TX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800211;
pub const TEGRA210_ADMAIF_TX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800214;
pub const TEGRA210_ADMAIF_TX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800217;
pub const TEGRA210_ADMAIF_TX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021a;
pub const TEGRA210_ADMAIF_TX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021d;
// Default values - Tegra186
pub const TEGRA186_ADMAIF_CIF_REG_DEFAULT: c_uint = 0x00007700;
pub const TEGRA186_ADMAIF_RX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000300;
pub const TEGRA186_ADMAIF_RX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000304;
pub const TEGRA186_ADMAIF_RX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000308;
pub const TEGRA186_ADMAIF_RX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000030c;
pub const TEGRA186_ADMAIF_RX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000210;
pub const TEGRA186_ADMAIF_RX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000213;
pub const TEGRA186_ADMAIF_RX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000216;
pub const TEGRA186_ADMAIF_RX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000219;
pub const TEGRA186_ADMAIF_RX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021c;
pub const TEGRA186_ADMAIF_RX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021f;
pub const TEGRA186_ADMAIF_RX11_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000222;
pub const TEGRA186_ADMAIF_RX12_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000225;
pub const TEGRA186_ADMAIF_RX13_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000228;
pub const TEGRA186_ADMAIF_RX14_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000022b;
pub const TEGRA186_ADMAIF_RX15_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000022e;
pub const TEGRA186_ADMAIF_RX16_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000231;
pub const TEGRA186_ADMAIF_RX17_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000234;
pub const TEGRA186_ADMAIF_RX18_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000237;
pub const TEGRA186_ADMAIF_RX19_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000023a;
pub const TEGRA186_ADMAIF_RX20_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000023d;
pub const TEGRA186_ADMAIF_TX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x02000300;
pub const TEGRA186_ADMAIF_TX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x02000304;
pub const TEGRA186_ADMAIF_TX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x02000308;
pub const TEGRA186_ADMAIF_TX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0200030c;
pub const TEGRA186_ADMAIF_TX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800210;
pub const TEGRA186_ADMAIF_TX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800213;
pub const TEGRA186_ADMAIF_TX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800216;
pub const TEGRA186_ADMAIF_TX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800219;
pub const TEGRA186_ADMAIF_TX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021c;
pub const TEGRA186_ADMAIF_TX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021f;
pub const TEGRA186_ADMAIF_TX11_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800222;
pub const TEGRA186_ADMAIF_TX12_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800225;
pub const TEGRA186_ADMAIF_TX13_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800228;
pub const TEGRA186_ADMAIF_TX14_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180022b;
pub const TEGRA186_ADMAIF_TX15_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180022e;
pub const TEGRA186_ADMAIF_TX16_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800231;
pub const TEGRA186_ADMAIF_TX17_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800234;
pub const TEGRA186_ADMAIF_TX18_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800237;
pub const TEGRA186_ADMAIF_TX19_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180023a;
pub const TEGRA186_ADMAIF_TX20_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180023d;
// Default values - Tegra264
pub const TEGRA264_ADMAIF_CIF_REG_DEFAULT: c_uint = 0x00003f00;
pub const TEGRA264_ADMAIF_RX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000200;
pub const TEGRA264_ADMAIF_RX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000203;
pub const TEGRA264_ADMAIF_RX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000206;
pub const TEGRA264_ADMAIF_RX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000209;
pub const TEGRA264_ADMAIF_RX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000020c;
pub const TEGRA264_ADMAIF_RX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000020f;
pub const TEGRA264_ADMAIF_RX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000212;
pub const TEGRA264_ADMAIF_RX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000215;
pub const TEGRA264_ADMAIF_RX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000218;
pub const TEGRA264_ADMAIF_RX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021b;
pub const TEGRA264_ADMAIF_RX11_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000021e;
pub const TEGRA264_ADMAIF_RX12_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000221;
pub const TEGRA264_ADMAIF_RX13_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000224;
pub const TEGRA264_ADMAIF_RX14_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000227;
pub const TEGRA264_ADMAIF_RX15_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000022a;
pub const TEGRA264_ADMAIF_RX16_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000022d;
pub const TEGRA264_ADMAIF_RX17_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000230;
pub const TEGRA264_ADMAIF_RX18_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000233;
pub const TEGRA264_ADMAIF_RX19_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000236;
pub const TEGRA264_ADMAIF_RX20_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000239;
pub const TEGRA264_ADMAIF_RX21_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000023c;
pub const TEGRA264_ADMAIF_RX22_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000023f;
pub const TEGRA264_ADMAIF_RX23_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000242;
pub const TEGRA264_ADMAIF_RX24_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000245;
pub const TEGRA264_ADMAIF_RX25_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000248;
pub const TEGRA264_ADMAIF_RX26_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000024b;
pub const TEGRA264_ADMAIF_RX27_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000024e;
pub const TEGRA264_ADMAIF_RX28_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000251;
pub const TEGRA264_ADMAIF_RX29_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000254;
pub const TEGRA264_ADMAIF_RX30_FIFO_CTRL_REG_DEFAULT: c_uint = 0x00000257;
pub const TEGRA264_ADMAIF_RX31_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000025a;
pub const TEGRA264_ADMAIF_RX32_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0000025d;
pub const TEGRA264_ADMAIF_TX1_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800200;
pub const TEGRA264_ADMAIF_TX2_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800203;
pub const TEGRA264_ADMAIF_TX3_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800206;
pub const TEGRA264_ADMAIF_TX4_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800209;
pub const TEGRA264_ADMAIF_TX5_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180020c;
pub const TEGRA264_ADMAIF_TX6_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180020f;
pub const TEGRA264_ADMAIF_TX7_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800212;
pub const TEGRA264_ADMAIF_TX8_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800215;
pub const TEGRA264_ADMAIF_TX9_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800218;
pub const TEGRA264_ADMAIF_TX10_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021b;
pub const TEGRA264_ADMAIF_TX11_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180021e;
pub const TEGRA264_ADMAIF_TX12_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800221;
pub const TEGRA264_ADMAIF_TX13_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800224;
pub const TEGRA264_ADMAIF_TX14_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800227;
pub const TEGRA264_ADMAIF_TX15_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180022a;
pub const TEGRA264_ADMAIF_TX16_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180022d;
pub const TEGRA264_ADMAIF_TX17_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800230;
pub const TEGRA264_ADMAIF_TX18_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800233;
pub const TEGRA264_ADMAIF_TX19_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800236;
pub const TEGRA264_ADMAIF_TX20_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800239;
pub const TEGRA264_ADMAIF_TX21_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180023c;
pub const TEGRA264_ADMAIF_TX22_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180023f;
pub const TEGRA264_ADMAIF_TX23_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800242;
pub const TEGRA264_ADMAIF_TX24_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800245;
pub const TEGRA264_ADMAIF_TX25_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800248;
pub const TEGRA264_ADMAIF_TX26_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180024b;
pub const TEGRA264_ADMAIF_TX27_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180024e;
pub const TEGRA264_ADMAIF_TX28_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800251;
pub const TEGRA264_ADMAIF_TX29_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800254;
pub const TEGRA264_ADMAIF_TX30_FIFO_CTRL_REG_DEFAULT: c_uint = 0x01800257;
pub const TEGRA264_ADMAIF_TX31_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180025a;
pub const TEGRA264_ADMAIF_TX32_FIFO_CTRL_REG_DEFAULT: c_uint = 0x0180025d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_admaif_soc_data {
    pub cmpnt: *const snd_soc_component_driver,
    pub regmap_conf: *const regmap_config,
    pub dais: *mut snd_soc_dai_driver,
    pub global_base: c_uint,
    pub tx_base: c_uint,
    pub rx_base: c_uint,
    pub num_ch: c_uint,
    pub max_stream_ch: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_admaif {
    pub playback_dma_data: *mut snd_dmaengine_dai_dma_data,
    pub soc_data: *const tegra_admaif_soc_data,
    pub mono_to_stereo: [*mut c_uint; ADMAIF_PATHS],
    pub stereo_to_mono: [*mut c_uint; ADMAIF_PATHS],
    pub regmap: *mut regmap,
    pub adma_isomgr: *mut tegra_adma_isomgr,
    pub capture_dma_data: [snd_dmaengine_dai_dma_data; ],
}
