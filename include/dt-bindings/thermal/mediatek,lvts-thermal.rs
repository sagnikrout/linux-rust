//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/thermal/mediatek,lvts-thermal.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2023 MediaTek Inc.
// Author: Balsam CHIHI <bchihi@baylibre.com>
//
pub const MT7987_CPU: c_int = 0;
pub const MT7987_ETH2P5G: c_int = 1;
pub const MT7988_CPU_0: c_int = 0;
pub const MT7988_CPU_1: c_int = 1;
pub const MT7988_ETH2P5G_0: c_int = 2;
pub const MT7988_ETH2P5G_1: c_int = 3;
pub const MT7988_TOPS_0: c_int = 4;
pub const MT7988_TOPS_1: c_int = 5;
pub const MT7988_ETHWARP_0: c_int = 6;
pub const MT7988_ETHWARP_1: c_int = 7;
pub const MT8186_LITTLE_CPU0: c_int = 0;
pub const MT8186_LITTLE_CPU1: c_int = 1;
pub const MT8186_LITTLE_CPU2: c_int = 2;
pub const MT8186_CAM: c_int = 3;
pub const MT8186_BIG_CPU0: c_int = 4;
pub const MT8186_BIG_CPU1: c_int = 5;
pub const MT8186_NNA: c_int = 6;
pub const MT8186_ADSP: c_int = 7;
pub const MT8186_GPU: c_int = 8;
pub const MT8188_MCU_LITTLE_CPU0: c_int = 0;
pub const MT8188_MCU_LITTLE_CPU1: c_int = 1;
pub const MT8188_MCU_LITTLE_CPU2: c_int = 2;
pub const MT8188_MCU_LITTLE_CPU3: c_int = 3;
pub const MT8188_MCU_BIG_CPU0: c_int = 4;
pub const MT8188_MCU_BIG_CPU1: c_int = 5;
pub const MT8188_AP_APU: c_int = 0;
pub const MT8188_AP_GPU0: c_int = 1;
pub const MT8188_AP_GPU1: c_int = 2;
pub const MT8188_AP_ADSP: c_int = 3;
pub const MT8188_AP_VDO: c_int = 4;
pub const MT8188_AP_INFRA: c_int = 5;
pub const MT8188_AP_CAM1: c_int = 6;
pub const MT8188_AP_CAM2: c_int = 7;
pub const MT8195_MCU_BIG_CPU0: c_int = 0;
pub const MT8195_MCU_BIG_CPU1: c_int = 1;
pub const MT8195_MCU_BIG_CPU2: c_int = 2;
pub const MT8195_MCU_BIG_CPU3: c_int = 3;
pub const MT8195_MCU_LITTLE_CPU0: c_int = 4;
pub const MT8195_MCU_LITTLE_CPU1: c_int = 5;
pub const MT8195_MCU_LITTLE_CPU2: c_int = 6;
pub const MT8195_MCU_LITTLE_CPU3: c_int = 7;
pub const MT8195_AP_VPU0: c_int = 8;
pub const MT8195_AP_VPU1: c_int = 9;
pub const MT8195_AP_GPU0: c_int = 10;
pub const MT8195_AP_GPU1: c_int = 11;
pub const MT8195_AP_VDEC: c_int = 12;
pub const MT8195_AP_IMG: c_int = 13;
pub const MT8195_AP_INFRA: c_int = 14;
pub const MT8195_AP_CAM0: c_int = 15;
pub const MT8195_AP_CAM1: c_int = 16;
pub const MT8192_MCU_BIG_CPU0: c_int = 0;
pub const MT8192_MCU_BIG_CPU1: c_int = 1;
pub const MT8192_MCU_BIG_CPU2: c_int = 2;
pub const MT8192_MCU_BIG_CPU3: c_int = 3;
pub const MT8192_MCU_LITTLE_CPU0: c_int = 4;
pub const MT8192_MCU_LITTLE_CPU1: c_int = 5;
pub const MT8192_MCU_LITTLE_CPU2: c_int = 6;
pub const MT8192_MCU_LITTLE_CPU3: c_int = 7;
pub const MT8192_AP_VPU0: c_int = 8;
pub const MT8192_AP_VPU1: c_int = 9;
pub const MT8192_AP_GPU0: c_int = 10;
pub const MT8192_AP_GPU1: c_int = 11;
pub const MT8192_AP_INFRA: c_int = 12;
pub const MT8192_AP_CAM: c_int = 13;
pub const MT8192_AP_MD0: c_int = 14;
pub const MT8192_AP_MD1: c_int = 15;
pub const MT8192_AP_MD2: c_int = 16;
pub const MT8196_MCU_MEDIUM_CPU6_0: c_int = 0;
pub const MT8196_MCU_MEDIUM_CPU6_1: c_int = 1;
pub const MT8196_MCU_DSU2: c_int = 2;
pub const MT8196_MCU_DSU3: c_int = 3;
pub const MT8196_MCU_LITTLE_CPU3: c_int = 4;
pub const MT8196_MCU_LITTLE_CPU0: c_int = 5;
pub const MT8196_MCU_LITTLE_CPU1: c_int = 6;
pub const MT8196_MCU_LITTLE_CPU2: c_int = 7;
pub const MT8196_MCU_MEDIUM_CPU4_0: c_int = 8;
pub const MT8196_MCU_MEDIUM_CPU4_1: c_int = 9;
pub const MT8196_MCU_MEDIUM_CPU5_0: c_int = 10;
pub const MT8196_MCU_MEDIUM_CPU5_1: c_int = 11;
pub const MT8196_MCU_DSU0: c_int = 12;
pub const MT8196_MCU_DSU1: c_int = 13;
pub const MT8196_MCU_BIG_CPU7_0: c_int = 14;
pub const MT8196_MCU_BIG_CPU7_1: c_int = 15;
pub const MT8196_AP_TOP0: c_int = 0;
pub const MT8196_AP_TOP1: c_int = 1;
pub const MT8196_AP_TOP2: c_int = 2;
pub const MT8196_AP_TOP3: c_int = 3;
pub const MT8196_AP_BOT0: c_int = 4;
pub const MT8196_AP_BOT1: c_int = 5;
pub const MT8196_AP_BOT2: c_int = 6;
pub const MT8196_AP_BOT3: c_int = 7;
