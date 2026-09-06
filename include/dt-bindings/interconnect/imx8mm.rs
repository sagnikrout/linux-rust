//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/imx8mm.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Interconnect framework driver for i.MX SoC
//
// Copyright (c) 2019, BayLibre
// Copyright (c) 2019-2020, NXP
// Author: Alexandre Bailon <abailon@baylibre.com>
//
pub const IMX8MM_ICN_NOC: c_int = 1;
pub const IMX8MM_ICS_DRAM: c_int = 2;
pub const IMX8MM_ICS_OCRAM: c_int = 3;
pub const IMX8MM_ICM_A53: c_int = 4;
pub const IMX8MM_ICM_VPU_H1: c_int = 5;
pub const IMX8MM_ICM_VPU_G1: c_int = 6;
pub const IMX8MM_ICM_VPU_G2: c_int = 7;
pub const IMX8MM_ICN_VIDEO: c_int = 8;
pub const IMX8MM_ICM_GPU2D: c_int = 9;
pub const IMX8MM_ICM_GPU3D: c_int = 10;
pub const IMX8MM_ICN_GPU: c_int = 11;
pub const IMX8MM_ICM_CSI: c_int = 12;
pub const IMX8MM_ICM_LCDIF: c_int = 13;
pub const IMX8MM_ICN_MIPI: c_int = 14;
pub const IMX8MM_ICM_USB1: c_int = 15;
pub const IMX8MM_ICM_USB2: c_int = 16;
pub const IMX8MM_ICM_PCIE: c_int = 17;
pub const IMX8MM_ICN_HSIO: c_int = 18;
pub const IMX8MM_ICM_SDMA2: c_int = 19;
pub const IMX8MM_ICM_SDMA3: c_int = 20;
pub const IMX8MM_ICN_AUDIO: c_int = 21;
pub const IMX8MM_ICN_ENET: c_int = 22;
pub const IMX8MM_ICM_ENET: c_int = 23;
pub const IMX8MM_ICN_MAIN: c_int = 24;
pub const IMX8MM_ICM_NAND: c_int = 25;
pub const IMX8MM_ICM_SDMA1: c_int = 26;
pub const IMX8MM_ICM_USDHC1: c_int = 27;
pub const IMX8MM_ICM_USDHC2: c_int = 28;
pub const IMX8MM_ICM_USDHC3: c_int = 29;
