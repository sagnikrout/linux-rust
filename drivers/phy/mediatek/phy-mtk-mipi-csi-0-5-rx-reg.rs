//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/mediatek/phy-mtk-mipi-csi-0-5-rx-reg.h
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
// Copyright (c) 2023, MediaTek Inc.
// Copyright (c) 2023, BayLibre Inc.
//
// CSI1 and CSI2 are identical, and similar to CSI0. All CSIX macros are
// applicable to the three PHYs. Where differences exist, they are denoted by
// macro names using CSI0 and CSI1, the latter being applicable to CSI1 and
// CSI2 alike.
//
pub const MIPI_RX_ANA00_CSIXA: c_uint = 0x0000;

pub const MIPI_RX_ANA18_CSIXA: c_uint = 0x0018;

pub const MIPI_RX_ANA1C_CSIXA: c_uint = 0x001c;
pub const MIPI_RX_ANA20_CSI0A: c_uint = 0x0020;
pub const MIPI_RX_ANA24_CSIXA: c_uint = 0x0024;

pub const MIPI_RX_ANA40_CSIXA: c_uint = 0x0040;

pub const MIPI_RX_WRAPPER80_CSIXA: c_uint = 0x0080;

pub const MIPI_RX_ANAA8_CSIXA: c_uint = 0x00a8;

