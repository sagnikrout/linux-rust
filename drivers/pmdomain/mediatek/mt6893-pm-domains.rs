//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/mediatek/mt6893-pm-domains.h
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

pub const MT6893_TOP_AXI_PROT_EN_MCU_STA1: c_uint = 0x2e4;
pub const MT6893_TOP_AXI_PROT_EN_MCU_SET: c_uint = 0x2c4;
pub const MT6893_TOP_AXI_PROT_EN_MCU_CLR: c_uint = 0x2c8;
pub const MT6893_TOP_AXI_PROT_EN_VDNR_1_SET: c_uint = 0xba4;
pub const MT6893_TOP_AXI_PROT_EN_VDNR_1_CLR: c_uint = 0xba8;
pub const MT6893_TOP_AXI_PROT_EN_VDNR_1_STA1: c_uint = 0xbb0;
pub const MT6893_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_SET: c_uint = 0xbb8;
pub const MT6893_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_CLR: c_uint = 0xbbc;
pub const MT6893_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_STA1: c_uint = 0xbc4;

//
// MT6893 Power Domain (MTCMOS) support
//
// The register layout for this IP is very similar to MT8192 so where possible
// the same definitions are reused to avoid duplication.
// Where the bus protection bits are also the same, the entire set is reused.
//
