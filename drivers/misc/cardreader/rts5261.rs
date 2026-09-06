//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/cardreader/rts5261.h
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
// Rui FENG <rui_feng@realsil.com.cn>
// Wei WANG <wei_wang@realsil.com.cn>
//
// New add

pub const RTS5261_AUTOLOAD_CFG0: c_uint = 0xFF7B;
pub const RTS5261_AUTOLOAD_CFG1: c_uint = 0xFF7C;
pub const RTS5261_AUTOLOAD_CFG2: c_uint = 0xFF7D;
pub const RTS5261_AUTOLOAD_CFG3: c_uint = 0xFF7E;
pub const RTS5261_AUTOLOAD_CFG4: c_uint = 0xFF7F;

pub const RTS5261_REG_VREF: c_uint = 0xFE97;

pub const RTS5261_PAD_H3L1: c_uint = 0xFF79;

// SSC_CTL2 0xFC12
pub const RTS5261_SSC_DEPTH_MASK: c_uint = 0x07;
pub const RTS5261_SSC_DEPTH_DISALBE: c_uint = 0x00;
pub const RTS5261_SSC_DEPTH_8M: c_uint = 0x01;
pub const RTS5261_SSC_DEPTH_4M: c_uint = 0x02;
pub const RTS5261_SSC_DEPTH_2M: c_uint = 0x03;
pub const RTS5261_SSC_DEPTH_1M: c_uint = 0x04;
pub const RTS5261_SSC_DEPTH_512K: c_uint = 0x05;
pub const RTS5261_SSC_DEPTH_256K: c_uint = 0x06;
pub const RTS5261_SSC_DEPTH_128K: c_uint = 0x07;
// efuse control register
pub const RTS5261_EFUSE_CTL: c_uint = 0xFC30;
pub const RTS5261_EFUSE_ENABLE: c_uint = 0x80;
// EFUSE_MODE: 0=READ 1=PROGRAM
pub const RTS5261_EFUSE_MODE_MASK: c_uint = 0x40;
pub const RTS5261_EFUSE_PROGRAM: c_uint = 0x40;
pub const RTS5261_EFUSE_ADDR: c_uint = 0xFC31;
pub const RTS5261_EFUSE_ADDR_MASK: c_uint = 0x3F;
pub const RTS5261_EFUSE_WRITE_DATA: c_uint = 0xFC32;
pub const RTS5261_EFUSE_READ_DATA: c_uint = 0xFC34;
// DMACTL 0xFE2C
pub const RTS5261_DMA_PACK_SIZE_MASK: c_uint = 0xF0;
// FW status register
pub const RTS5261_FW_STATUS: c_uint = 0xFF56;

// FW control register
pub const RTS5261_FW_CTL: c_uint = 0xFF5F;

pub const RTS5261_REG_FPDCTL: c_uint = 0xFF60;
pub const RTS5261_REG_LDO12_CFG: c_uint = 0xFF6E;

// LDO control register
pub const RTS5261_CARD_PWR_CTL: c_uint = 0xFD50;

pub const RTS5261_LDO1233318_POW_CTL: c_uint = 0xFF70;

pub const RTS5261_DV3318_CFG: c_uint = 0xFF71;

// CRD6603-433 190319 request changed

pub const RTS5261_LDO1_CFG1: c_uint = 0xFF73;

pub const RTS5261_LDO2_CFG0: c_uint = 0xFF74;

pub const RTS5261_LDO2_CFG1: c_uint = 0xFF75;

pub const RTS5261_LDO3_CFG0: c_uint = 0xFF76;

pub const RTS5261_LDO3_CFG1: c_uint = 0xFF77;

pub const RTS5261_REG_PME_FORCE_CTL: c_uint = 0xFF78;
pub const FORCE_PM_CONTROL: c_uint = 0x20;
pub const FORCE_PM_VALUE: c_uint = 0x10;
pub const REG_EFUSE_BYPASS: c_uint = 0x08;
pub const REG_EFUSE_POR: c_uint = 0x04;
pub const REG_EFUSE_POWER_MASK: c_uint = 0x03;
pub const REG_EFUSE_POWERON: c_uint = 0x03;
pub const REG_EFUSE_POWEROFF: c_uint = 0x00;
// Single LUN, support SD/SD EXPRESS
pub const DEFAULT_SINGLE: c_int = 0;
pub const SD_LUN: c_int = 1;
pub const SD_EXPRESS_LUN: c_int = 2;
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
