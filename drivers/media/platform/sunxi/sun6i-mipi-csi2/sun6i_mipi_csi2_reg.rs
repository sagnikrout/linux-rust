//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-mipi-csi2/sun6i_mipi_csi2_reg.h
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
// Copyright 2020-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//
pub const SUN6I_MIPI_CSI2_CTL_REG: c_uint = 0x0;

pub const SUN6I_MIPI_CSI2_CFG_REG: c_uint = 0x4;

pub const SUN6I_MIPI_CSI2_VCDT_RX_REG: c_uint = 0x8;

pub const SUN6I_MIPI_CSI2_RX_PKT_NUM_REG: c_uint = 0xc;
pub const SUN6I_MIPI_CSI2_VERSION_REG: c_uint = 0x3c;
pub const SUN6I_MIPI_CSI2_CH_CFG_REG: c_uint = 0x40;
pub const SUN6I_MIPI_CSI2_CH_INT_EN_REG: c_uint = 0x50;

pub const SUN6I_MIPI_CSI2_CH_INT_PD_REG: c_uint = 0x58;
pub const SUN6I_MIPI_CSI2_CH_INT_PD_CLEAR: c_uint = 0xff;

pub const SUN6I_MIPI_CSI2_CH_DT_TRIGGER_REG: c_uint = 0x60;
pub const SUN6I_MIPI_CSI2_CH_CUR_PH_REG: c_uint = 0x70;
pub const SUN6I_MIPI_CSI2_CH_ECC_REG: c_uint = 0x74;
pub const SUN6I_MIPI_CSI2_CH_CKS_REG: c_uint = 0x78;
pub const SUN6I_MIPI_CSI2_CH_FRAME_NUM_REG: c_uint = 0x7c;
pub const SUN6I_MIPI_CSI2_CH_LINE_NUM_REG: c_uint = 0x80;
pub const SUN6I_MIPI_CSI2_CH_OFFSET: c_uint = 0x100;

