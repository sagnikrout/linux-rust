//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/mt6311-regulator.h
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
// Copyright (c) 2015 MediaTek Inc.
// Author: Henry Chen <henryc.chen@mediatek.com>
//
pub const MT6311_SWCID: c_uint = 0x01;
pub const MT6311_TOP_INT_CON: c_uint = 0x18;
pub const MT6311_TOP_INT_MON: c_uint = 0x19;
pub const MT6311_VDVFS11_CON0: c_uint = 0x87;
pub const MT6311_VDVFS11_CON7: c_uint = 0x88;
pub const MT6311_VDVFS11_CON8: c_uint = 0x89;
pub const MT6311_VDVFS11_CON9: c_uint = 0x8A;
pub const MT6311_VDVFS11_CON10: c_uint = 0x8B;
pub const MT6311_VDVFS11_CON11: c_uint = 0x8C;
pub const MT6311_VDVFS11_CON12: c_uint = 0x8D;
pub const MT6311_VDVFS11_CON13: c_uint = 0x8E;
pub const MT6311_VDVFS11_CON14: c_uint = 0x8F;
pub const MT6311_VDVFS11_CON15: c_uint = 0x90;
pub const MT6311_VDVFS11_CON16: c_uint = 0x91;
pub const MT6311_VDVFS11_CON17: c_uint = 0x92;
pub const MT6311_VDVFS11_CON18: c_uint = 0x93;
pub const MT6311_VDVFS11_CON19: c_uint = 0x94;
pub const MT6311_LDO_CON0: c_uint = 0xCC;
pub const MT6311_LDO_OCFB0: c_uint = 0xCD;
pub const MT6311_LDO_CON2: c_uint = 0xCE;
pub const MT6311_LDO_CON3: c_uint = 0xCF;
pub const MT6311_LDO_CON4: c_uint = 0xD0;
pub const MT6311_FQMTR_CON0: c_uint = 0xD1;
pub const MT6311_FQMTR_CON1: c_uint = 0xD2;
pub const MT6311_FQMTR_CON2: c_uint = 0xD3;
pub const MT6311_FQMTR_CON3: c_uint = 0xD4;
pub const MT6311_FQMTR_CON4: c_uint = 0xD5;
pub const MT6311_PMIC_RG_INT_POL_MASK: c_uint = 0x1;
pub const MT6311_PMIC_RG_INT_EN_MASK: c_uint = 0x2;
pub const MT6311_PMIC_RG_BUCK_OC_INT_STATUS_MASK: c_uint = 0x10;
pub const MT6311_PMIC_VDVFS11_EN_CTRL_MASK: c_uint = 0x1;
pub const MT6311_PMIC_VDVFS11_VOSEL_CTRL_MASK: c_uint = 0x2;
pub const MT6311_PMIC_VDVFS11_EN_SEL_MASK: c_uint = 0x3;
pub const MT6311_PMIC_VDVFS11_VOSEL_SEL_MASK: c_uint = 0xc;
pub const MT6311_PMIC_VDVFS11_EN_MASK: c_uint = 0x1;
pub const MT6311_PMIC_VDVFS11_VOSEL_MASK: c_uint = 0x7F;
pub const MT6311_PMIC_VDVFS11_VOSEL_ON_MASK: c_uint = 0x7F;
pub const MT6311_PMIC_VDVFS11_VOSEL_SLEEP_MASK: c_uint = 0x7F;
pub const MT6311_PMIC_NI_VDVFS11_VOSEL_MASK: c_uint = 0x7F;
pub const MT6311_PMIC_RG_VBIASN_EN_MASK: c_uint = 0x1;
