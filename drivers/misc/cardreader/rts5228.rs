//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/cardreader/rts5228.h
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
// Driver for Realtek PCI-Express card reader
//
// Copyright(c) 2018-2019 Realtek Semiconductor Corp. All rights reserved.
//
// Author:
// Ricky WU <ricky_wu@realtek.com>
// Rui FENG <rui_feng@realsil.com.cn>
// Wei WANG <wei_wang@realsil.com.cn>
//
pub const RTS5228_AUTOLOAD_CFG0: c_uint = 0xFF7B;
pub const RTS5228_AUTOLOAD_CFG1: c_uint = 0xFF7C;
pub const RTS5228_AUTOLOAD_CFG2: c_uint = 0xFF7D;
pub const RTS5228_AUTOLOAD_CFG3: c_uint = 0xFF7E;
pub const RTS5228_AUTOLOAD_CFG4: c_uint = 0xFF7F;
pub const RTS5228_REG_VREF: c_uint = 0xFE97;

pub const RTS5228_PAD_H3L1: c_uint = 0xFF79;

// SSC_CTL2 0xFC12
pub const RTS5228_SSC_DEPTH_MASK: c_uint = 0x07;
pub const RTS5228_SSC_DEPTH_DISALBE: c_uint = 0x00;
pub const RTS5228_SSC_DEPTH_8M: c_uint = 0x01;
pub const RTS5228_SSC_DEPTH_4M: c_uint = 0x02;
pub const RTS5228_SSC_DEPTH_2M: c_uint = 0x03;
pub const RTS5228_SSC_DEPTH_1M: c_uint = 0x04;
pub const RTS5228_SSC_DEPTH_512K: c_uint = 0x05;
pub const RTS5228_SSC_DEPTH_256K: c_uint = 0x06;
pub const RTS5228_SSC_DEPTH_128K: c_uint = 0x07;
// DMACTL 0xFE2C
pub const RTS5228_DMA_PACK_SIZE_MASK: c_uint = 0xF0;
pub const RTS5228_REG_LDO12_CFG: c_uint = 0xFF6E;

pub const RTS5228_REG_LDO12_L12: c_uint = 0xFF6F;

// LDO control register
pub const RTS5228_CARD_PWR_CTL: c_uint = 0xFD50;

pub const RTS5228_LDO1233318_POW_CTL: c_uint = 0xFF70;

pub const RTS5228_DV3318_CFG: c_uint = 0xFF71;

pub const RTS5228_LDO1_CFG0: c_uint = 0xFF72;

pub const RTS5228_LDO1_CFG1: c_uint = 0xFF73;

pub const RTS5228_AUXCLK_GAT_CTL: c_uint = 0xFF74;
pub const RTS5228_REG_RREF_CTL_0: c_uint = 0xFF75;

pub const RTS5228_REG_RREF_CTL_1: c_uint = 0xFF76;
pub const RTS5228_REG_RREF_CTL_2: c_uint = 0xFF77;

pub const RTS5228_REG_PME_FORCE_CTL: c_uint = 0xFF78;
pub const FORCE_PM_CONTROL: c_uint = 0x20;
pub const FORCE_PM_VALUE: c_uint = 0x10;
// Single LUN, support SD
pub const DEFAULT_SINGLE: c_int = 0;
pub const SD_LUN: c_int = 1;
// For Change_FPGA_SSCClock Function
pub const MULTIPLY_BY_1: c_uint = 0x00;
pub const MULTIPLY_BY_2: c_uint = 0x01;
pub const MULTIPLY_BY_3: c_uint = 0x02;
pub const MULTIPLY_BY_4: c_uint = 0x03;
pub const MULTIPLY_BY_5: c_uint = 0x04;
pub const MULTIPLY_BY_6: c_uint = 0x05;
pub const MULTIPLY_BY_7: c_uint = 0x06;
pub const MULTIPLY_BY_8: c_uint = 0x07;
pub const MULTIPLY_BY_9: c_uint = 0x08;
pub const MULTIPLY_BY_10: c_uint = 0x09;
pub const DIVIDE_BY_2: c_uint = 0x01;
pub const DIVIDE_BY_3: c_uint = 0x02;
pub const DIVIDE_BY_4: c_uint = 0x03;
pub const DIVIDE_BY_5: c_uint = 0x04;
pub const DIVIDE_BY_6: c_uint = 0x05;
pub const DIVIDE_BY_7: c_uint = 0x06;
pub const DIVIDE_BY_8: c_uint = 0x07;
pub const DIVIDE_BY_9: c_uint = 0x08;
pub const DIVIDE_BY_10: c_uint = 0x09;
