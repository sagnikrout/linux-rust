//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/fsl,imx8mp.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Interconnect framework driver for i.MX SoC
//
// Copyright 2022 NXP
// Peng Fan <peng.fan@nxp.com>
//
pub const IMX8MP_ICN_NOC: c_int = 0;
pub const IMX8MP_ICN_MAIN: c_int = 1;
pub const IMX8MP_ICS_DRAM: c_int = 2;
pub const IMX8MP_ICS_OCRAM: c_int = 3;
pub const IMX8MP_ICM_A53: c_int = 4;
pub const IMX8MP_ICM_SUPERMIX: c_int = 5;
pub const IMX8MP_ICM_GIC: c_int = 6;
pub const IMX8MP_ICM_MLMIX: c_int = 7;
pub const IMX8MP_ICN_AUDIO: c_int = 8;
pub const IMX8MP_ICM_DSP: c_int = 9;
pub const IMX8MP_ICM_SDMA2PER: c_int = 10;
pub const IMX8MP_ICM_SDMA2BURST: c_int = 11;
pub const IMX8MP_ICM_SDMA3PER: c_int = 12;
pub const IMX8MP_ICM_SDMA3BURST: c_int = 13;
pub const IMX8MP_ICM_EDMA: c_int = 14;
pub const IMX8MP_ICN_GPU: c_int = 15;
pub const IMX8MP_ICM_GPU2D: c_int = 16;
pub const IMX8MP_ICM_GPU3D: c_int = 17;
pub const IMX8MP_ICN_HDMI: c_int = 18;
pub const IMX8MP_ICM_HRV: c_int = 19;
pub const IMX8MP_ICM_LCDIF_HDMI: c_int = 20;
pub const IMX8MP_ICM_HDCP: c_int = 21;
pub const IMX8MP_ICN_HSIO: c_int = 22;
pub const IMX8MP_ICM_NOC_PCIE: c_int = 23;
pub const IMX8MP_ICM_USB1: c_int = 24;
pub const IMX8MP_ICM_USB2: c_int = 25;
pub const IMX8MP_ICM_PCIE: c_int = 26;
pub const IMX8MP_ICN_MEDIA: c_int = 27;
pub const IMX8MP_ICM_LCDIF_RD: c_int = 28;
pub const IMX8MP_ICM_LCDIF_WR: c_int = 29;
pub const IMX8MP_ICM_ISI0: c_int = 30;
pub const IMX8MP_ICM_ISI1: c_int = 31;
pub const IMX8MP_ICM_ISI2: c_int = 32;
pub const IMX8MP_ICM_ISP0: c_int = 33;
pub const IMX8MP_ICM_ISP1: c_int = 34;
pub const IMX8MP_ICM_DWE: c_int = 35;
pub const IMX8MP_ICN_VIDEO: c_int = 36;
pub const IMX8MP_ICM_VPU_G1: c_int = 37;
pub const IMX8MP_ICM_VPU_G2: c_int = 38;
pub const IMX8MP_ICM_VPU_H1: c_int = 39;
