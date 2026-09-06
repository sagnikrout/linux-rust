//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/psoc_pci_pll_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PSOC_PCI_PLL (Prototype: PLL)
//
pub const mmPSOC_PCI_PLL_NR: c_uint = 0xC72100;
pub const mmPSOC_PCI_PLL_NF: c_uint = 0xC72104;
pub const mmPSOC_PCI_PLL_OD: c_uint = 0xC72108;
pub const mmPSOC_PCI_PLL_NB: c_uint = 0xC7210C;
pub const mmPSOC_PCI_PLL_CFG: c_uint = 0xC72110;
pub const mmPSOC_PCI_PLL_LOSE_MASK: c_uint = 0xC72120;
pub const mmPSOC_PCI_PLL_LOCK_INTR: c_uint = 0xC72128;
pub const mmPSOC_PCI_PLL_LOCK_BYPASS: c_uint = 0xC7212C;
pub const mmPSOC_PCI_PLL_DATA_CHNG: c_uint = 0xC72130;
pub const mmPSOC_PCI_PLL_RST: c_uint = 0xC72134;
pub const mmPSOC_PCI_PLL_SLIP_WD_CNTR: c_uint = 0xC72150;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_0: c_uint = 0xC72200;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_1: c_uint = 0xC72204;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_2: c_uint = 0xC72208;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_3: c_uint = 0xC7220C;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_CMD_0: c_uint = 0xC72220;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_CMD_1: c_uint = 0xC72224;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_CMD_2: c_uint = 0xC72228;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_CMD_3: c_uint = 0xC7222C;
pub const mmPSOC_PCI_PLL_DIV_SEL_0: c_uint = 0xC72280;
pub const mmPSOC_PCI_PLL_DIV_SEL_1: c_uint = 0xC72284;
pub const mmPSOC_PCI_PLL_DIV_SEL_2: c_uint = 0xC72288;
pub const mmPSOC_PCI_PLL_DIV_SEL_3: c_uint = 0xC7228C;
pub const mmPSOC_PCI_PLL_DIV_EN_0: c_uint = 0xC722A0;
pub const mmPSOC_PCI_PLL_DIV_EN_1: c_uint = 0xC722A4;
pub const mmPSOC_PCI_PLL_DIV_EN_2: c_uint = 0xC722A8;
pub const mmPSOC_PCI_PLL_DIV_EN_3: c_uint = 0xC722AC;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_BUSY_0: c_uint = 0xC722C0;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_BUSY_1: c_uint = 0xC722C4;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_BUSY_2: c_uint = 0xC722C8;
pub const mmPSOC_PCI_PLL_DIV_FACTOR_BUSY_3: c_uint = 0xC722CC;
pub const mmPSOC_PCI_PLL_CLK_GATER: c_uint = 0xC72300;
pub const mmPSOC_PCI_PLL_CLK_RLX_0: c_uint = 0xC72310;
pub const mmPSOC_PCI_PLL_CLK_RLX_1: c_uint = 0xC72314;
pub const mmPSOC_PCI_PLL_CLK_RLX_2: c_uint = 0xC72318;
pub const mmPSOC_PCI_PLL_CLK_RLX_3: c_uint = 0xC7231C;
pub const mmPSOC_PCI_PLL_REF_CNTR_PERIOD: c_uint = 0xC72400;
pub const mmPSOC_PCI_PLL_REF_LOW_THRESHOLD: c_uint = 0xC72410;
pub const mmPSOC_PCI_PLL_REF_HIGH_THRESHOLD: c_uint = 0xC72420;
pub const mmPSOC_PCI_PLL_PLL_NOT_STABLE: c_uint = 0xC72430;
pub const mmPSOC_PCI_PLL_FREQ_CALC_EN: c_uint = 0xC72440;
