//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/mediatek/mt8189-pm-domains.h
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
// Copyright (c) 2025 MediaTek Inc.
// Author: Qiqi Wang <qiqi.wang@mediatek.com>
//

//
// MT8189 power domain support
//
pub const MT8189_SPM_PWR_STATUS: c_uint = 0x0f40;
pub const MT8189_SPM_PWR_STATUS_2ND: c_uint = 0x0f44;
pub const MT8189_SPM_PWR_STATUS_MSB: c_uint = 0x0f48;
pub const MT8189_SPM_PWR_STATUS_MSB_2ND: c_uint = 0x0f4c;
pub const MT8189_SPM_XPU_PWR_STATUS: c_uint = 0x0f50;
pub const MT8189_SPM_XPU_PWR_STATUS_2ND: c_uint = 0x0f54;
pub const MT8189_PROT_EN_EMICFG_GALS_SLP_SET: c_uint = 0x0084;
pub const MT8189_PROT_EN_EMICFG_GALS_SLP_CLR: c_uint = 0x0088;
pub const MT8189_PROT_EN_EMICFG_GALS_SLP_RDY: c_uint = 0x008c;
pub const MT8189_PROT_EN_MMSYS_STA_0_SET: c_uint = 0x0c14;
pub const MT8189_PROT_EN_MMSYS_STA_0_CLR: c_uint = 0x0c18;
pub const MT8189_PROT_EN_MMSYS_STA_0_RDY: c_uint = 0x0c1c;
pub const MT8189_PROT_EN_MMSYS_STA_1_SET: c_uint = 0x0c24;
pub const MT8189_PROT_EN_MMSYS_STA_1_CLR: c_uint = 0x0c28;
pub const MT8189_PROT_EN_MMSYS_STA_1_RDY: c_uint = 0x0c2c;
pub const MT8189_PROT_EN_INFRASYS_STA_0_SET: c_uint = 0x0c44;
pub const MT8189_PROT_EN_INFRASYS_STA_0_CLR: c_uint = 0x0c48;
pub const MT8189_PROT_EN_INFRASYS_STA_0_RDY: c_uint = 0x0c4c;
pub const MT8189_PROT_EN_INFRASYS_STA_1_SET: c_uint = 0x0c54;
pub const MT8189_PROT_EN_INFRASYS_STA_1_CLR: c_uint = 0x0c58;
pub const MT8189_PROT_EN_INFRASYS_STA_1_RDY: c_uint = 0x0c5c;
pub const MT8189_PROT_EN_PERISYS_STA_0_SET: c_uint = 0x0c84;
pub const MT8189_PROT_EN_PERISYS_STA_0_CLR: c_uint = 0x0c88;
pub const MT8189_PROT_EN_PERISYS_STA_0_RDY: c_uint = 0x0c8c;
pub const MT8189_PROT_EN_MCU_STA_0_SET: c_uint = 0x0c94;
pub const MT8189_PROT_EN_MCU_STA_0_CLR: c_uint = 0x0c98;
pub const MT8189_PROT_EN_MCU_STA_0_RDY: c_uint = 0x0c9c;
pub const MT8189_PROT_EN_MD_STA_0_SET: c_uint = 0x0ca4;
pub const MT8189_PROT_EN_MD_STA_0_CLR: c_uint = 0x0ca8;
pub const MT8189_PROT_EN_MD_STA_0_RDY: c_uint = 0x0cac;

