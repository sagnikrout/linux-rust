//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun8i-a83t-mipi-csi2/sun8i_a83t_mipi_csi2_reg.h
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
// Copyright 2020 Kévin L'hôpital <kevin.lhopital@bootlin.com>
// Copyright 2020-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//
pub const SUN8I_A83T_MIPI_CSI2_VERSION_REG: c_uint = 0x0;
pub const SUN8I_A83T_MIPI_CSI2_CTRL_REG: c_uint = 0x4;
pub const SUN8I_A83T_MIPI_CSI2_CTRL_INIT_VALUE: c_uint = 0xb8c39bec;

pub const SUN8I_A83T_MIPI_CSI2_RX_PKT_NUM_REG: c_uint = 0x8;
pub const SUN8I_A83T_MIPI_CSI2_RX_PKT_NUM_INIT_VALUE: c_uint = 0xb8d257f8;
pub const SUN8I_A83T_MIPI_CSI2_RSVD0_REG: c_uint = 0xc;
pub const SUN8I_A83T_MIPI_CSI2_RSVD1_REG: c_uint = 0x18;
pub const SUN8I_A83T_MIPI_CSI2_RSVD1_HW_LOCK_VALUE: c_uint = 0xb8c8a30c;
pub const SUN8I_A83T_MIPI_CSI2_RSVD2_REG: c_uint = 0x1c;
pub const SUN8I_A83T_MIPI_CSI2_RSVD2_HW_LOCK_VALUE: c_uint = 0xb8df8ad7;
pub const SUN8I_A83T_MIPI_CSI2_INT_STA0_REG: c_uint = 0x20;

pub const SUN8I_A83T_MIPI_CSI2_INT_STA1_REG: c_uint = 0x24;

pub const SUN8I_A83T_MIPI_CSI2_INT_MSK0_REG: c_uint = 0x28;

pub const SUN8I_A83T_MIPI_CSI2_INT_MSK1_REG: c_uint = 0x2c;

pub const SUN8I_A83T_MIPI_CSI2_CFG_REG: c_uint = 0x100;
pub const SUN8I_A83T_MIPI_CSI2_CFG_INIT_VALUE: c_uint = 0xb8c64f24;

pub const SUN8I_A83T_MIPI_CSI2_VCDT0_REG: c_uint = 0x104;

pub const SUN8I_A83T_MIPI_CSI2_VCDT1_REG: c_uint = 0x108;

