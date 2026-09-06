//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pmdomain/mediatek/mt6858-pm-domains.h
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
// KY Liu <ky.liu@mediatek.com>
// Copyright (c) 2026 Jolla Mobile Ltd
// Nikolai Burov <nikolai.burov@jolla.com>
//

// TOP_AXI registers
pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_0_SET: c_uint = 0x0c14;
pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_0_CLR: c_uint = 0x0c18;
pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_0_RDY: c_uint = 0x0c1c;

pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_1_SET: c_uint = 0x0c24;
pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_1_CLR: c_uint = 0x0c28;
pub const MT6858_TOP_AXI_PROT_EN_MMSYS_STA_1_RDY: c_uint = 0x0c2c;

pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_0_SET: c_uint = 0x0c44;
pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_0_CLR: c_uint = 0x0c48;
pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_0_RDY: c_uint = 0x0c4c;

pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_1_SET: c_uint = 0x0c54;
pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_1_CLR: c_uint = 0x0c58;
pub const MT6858_TOP_AXI_PROT_EN_INFRASYS_STA_1_RDY: c_uint = 0x0c5c;

pub const MT6858_TOP_AXI_PROT_EN_EMISYS_STA_0_SET: c_uint = 0x0c64;
pub const MT6858_TOP_AXI_PROT_EN_EMISYS_STA_0_CLR: c_uint = 0x0c68;
pub const MT6858_TOP_AXI_PROT_EN_EMISYS_STA_0_RDY: c_uint = 0x0c6c;

pub const MT6858_TOP_AXI_PROT_EN_PERISYS_STA_0_SET: c_uint = 0x0c84;
pub const MT6858_TOP_AXI_PROT_EN_PERISYS_STA_0_CLR: c_uint = 0x0c88;
pub const MT6858_TOP_AXI_PROT_EN_PERISYS_STA_0_RDY: c_uint = 0x0c8c;

pub const MT6858_TOP_AXI_PROT_EN_MCU_STA_0_SET: c_uint = 0x0c94;
pub const MT6858_TOP_AXI_PROT_EN_MCU_STA_0_CLR: c_uint = 0x0c98;
pub const MT6858_TOP_AXI_PROT_EN_MCU_STA_0_RDY: c_uint = 0x0c9c;

// {IMG,IPE,CAM}_SUBx registers
pub const MT6858_SUBx_PROT_EN_SET: c_uint = 0x03c4;
pub const MT6858_SUBx_PROT_EN_CLR: c_uint = 0x03c8;
pub const MT6858_SUBx_PROT_EN_STA: c_uint = 0x03cc;

// VLP_AXI registers
pub const MT6858_VLP_AXI_PROT_EN_SET: c_uint = 0x0214;
pub const MT6858_VLP_AXI_PROT_EN_CLR: c_uint = 0x0218;
pub const MT6858_VLP_AXI_PROT_EN_STA: c_uint = 0x021c;

// PWR_CON registers

//
// Note: the PWR_ACK_2ND bit is not used for the modem domain.
// Skip it and fall back to checking the 1st bit twice.
//
