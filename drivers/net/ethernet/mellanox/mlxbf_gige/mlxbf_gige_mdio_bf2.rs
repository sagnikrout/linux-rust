//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxbf_gige/mlxbf_gige_mdio_bf2.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
// MDIO support for Mellanox Gigabit Ethernet driver
//
// Copyright (c) 2022 NVIDIA CORPORATION & AFFILIATES, ALL RIGHTS RESERVED.
//
// This software product is a proprietary product of NVIDIA CORPORATION &
// AFFILIATES (the "Company") and all right, title, and interest in and to the
// software product, including all associated intellectual property rights, are
// and shall remain exclusively with the Company.
//
// This software product is governed by the End User License Agreement
// provided with the software product.
//

pub const MLXBF2_GIGE_MDIO_GW_OFFSET: c_uint = 0x0;
pub const MLXBF2_GIGE_MDIO_CFG_OFFSET: c_uint = 0x4;
// MDIO GW register bits

pub const MLXBF2_GIGE_MDIO_GW_AD_SHIFT: c_int = 0;
pub const MLXBF2_GIGE_MDIO_GW_DEVAD_SHIFT: c_int = 16;
pub const MLXBF2_GIGE_MDIO_GW_PARTAD_SHIFT: c_int = 21;
pub const MLXBF2_GIGE_MDIO_GW_OPCODE_SHIFT: c_int = 26;
pub const MLXBF2_GIGE_MDIO_GW_ST1_SHIFT: c_int = 28;
pub const MLXBF2_GIGE_MDIO_GW_BUSY_SHIFT: c_int = 30;
// MDIO config register bits

