//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/mediatek/mt8196-pm-domains.h
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
// Copyright (c) 2025 Collabora Ltd
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

//
// MT8196 and MT6991 power domain support
//
// INFRA TOP_AXI registers
pub const MT8196_TOP_AXI_PROT_EN_SET: c_uint = 0x4;
pub const MT8196_TOP_AXI_PROT_EN_CLR: c_uint = 0x8;
pub const MT8196_TOP_AXI_PROT_EN_STA: c_uint = 0xc;

pub const MT8196_TOP_AXI_PROT_EN_1_SET: c_uint = 0x24;
pub const MT8196_TOP_AXI_PROT_EN_1_CLR: c_uint = 0x28;
pub const MT8196_TOP_AXI_PROT_EN_1_STA: c_uint = 0x2c;

// SPM BUS_PROTECT registers
pub const MT8196_SPM_BUS_PROTECT_CON_SET: c_uint = 0xdc;
pub const MT8196_SPM_BUS_PROTECT_CON_CLR: c_uint = 0xe0;
pub const MT8196_SPM_BUS_PROTECT_RDY: c_uint = 0x208;

// PWR_CON registers

// Note: This is not managing powerdown (pdn), but sleep instead (slp)
