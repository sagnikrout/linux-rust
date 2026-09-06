//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/cardreader/rts5264.h
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
// Ricky Wu <ricky_wu@realtek.com>
//
// New add

pub const RTS5264_AUTOLOAD_CFG0: c_uint = 0xFF7B;
pub const RTS5264_AUTOLOAD_CFG1: c_uint = 0xFF7C;
pub const RTS5264_AUTOLOAD_CFG3: c_uint = 0xFF7E;
pub const RTS5264_AUTOLOAD_CFG4: c_uint = 0xFF7F;

// SSC_CTL2 0xFC12
pub const RTS5264_SSC_DEPTH_MASK: c_uint = 0x07;
pub const RTS5264_SSC_DEPTH_DISALBE: c_uint = 0x00;
pub const RTS5264_SSC_DEPTH_8M: c_uint = 0x01;
pub const RTS5264_SSC_DEPTH_4M: c_uint = 0x02;
pub const RTS5264_SSC_DEPTH_2M: c_uint = 0x03;
pub const RTS5264_SSC_DEPTH_1M: c_uint = 0x04;
pub const RTS5264_SSC_DEPTH_512K: c_uint = 0x05;
pub const RTS5264_SSC_DEPTH_256K: c_uint = 0x06;
pub const RTS5264_SSC_DEPTH_128K: c_uint = 0x07;
pub const RTS5264_CARD_CLK_SRC2: c_uint = 0xFC2F;
pub const RTS5264_REG_BIG_KVCO_A: c_uint = 0x20;
// efuse control register
pub const RTS5264_EFUSE_CTL: c_uint = 0xFC30;
pub const RTS5264_EFUSE_ENABLE: c_uint = 0x80;
// EFUSE_MODE: 0=READ 1=PROGRAM
pub const RTS5264_EFUSE_MODE_MASK: c_uint = 0x40;
pub const RTS5264_EFUSE_PROGRAM: c_uint = 0x40;
pub const RTS5264_EFUSE_ADDR: c_uint = 0xFC31;
pub const RTS5264_EFUSE_ADDR_MASK: c_uint = 0x3F;
pub const RTS5264_EFUSE_WRITE_DATA: c_uint = 0xFC32;
pub const RTS5264_EFUSE_READ_DATA: c_uint = 0xFC34;
pub const RTS5264_SYS_DUMMY_1: c_uint = 0xFC35;
pub const RTS5264_REG_BIG_KVCO: c_uint = 0x04;
// DMACTL 0xFE2C
pub const RTS5264_DMA_PACK_SIZE_MASK: c_uint = 0x70;
pub const RTS5264_FW_CFG_INFO2: c_uint = 0xFF52;
pub const RTS5264_FW_CFG1: c_uint = 0xFF55;

// FW status register
pub const RTS5264_FW_STATUS: c_uint = 0xFF56;

// FW control register
pub const RTS5264_FW_CTL: c_uint = 0xFF5F;

pub const RTS5264_REG_FPDCTL: c_uint = 0xFF60;
pub const RTS5264_REG_LDO12_CFG: c_uint = 0xFF6E;

// LDO control register
pub const RTS5264_CARD_PWR_CTL: c_uint = 0xFD50;

pub const RTS5264_OCP_VDD3_CTL: c_uint = 0xFD89;
pub const SD_VDD3_DETECT_EN: c_uint = 0x08;
pub const SD_VDD3_OCP_INT_EN: c_uint = 0x04;
pub const SD_VDD3_OCP_INT_CLR: c_uint = 0x02;
pub const SD_VDD3_OC_CLR: c_uint = 0x01;
pub const RTS5264_OCP_VDD3_STS: c_uint = 0xFD8A;
pub const SD_VDD3_OCP_DETECT: c_uint = 0x08;
pub const SD_VDD3_OC_NOW: c_uint = 0x04;
pub const SD_VDD3_OC_EVER: c_uint = 0x02;
pub const RTS5264_OVP_CTL: c_uint = 0xFD8D;
pub const RTS5264_OVP_TIME_MASK: c_uint = 0xF0;
pub const RTS5264_OVP_TIME_DFT: c_uint = 0x50;
pub const RTS5264_OVP_DETECT_EN: c_uint = 0x08;
pub const RTS5264_OVP_INT_EN: c_uint = 0x04;
pub const RTS5264_OVP_INT_CLR: c_uint = 0x02;
pub const RTS5264_OVP_CLR: c_uint = 0x01;
pub const RTS5264_OVP_STS: c_uint = 0xFD8E;
pub const RTS5264_OVP_GLTCH_TIME_MASK: c_uint = 0xF0;
pub const RTS5264_OVP_GLTCH_TIME_DFT: c_uint = 0x50;
pub const RTS5264_VOVER_DET: c_uint = 0x08;
pub const RTS5264_OVP_NOW: c_uint = 0x04;
pub const RTS5264_OVP_EVER: c_uint = 0x02;
pub const RTS5264_CMD_OE_START_EARLY: c_uint = 0xFDCB;
pub const RTS5264_CMD_OE_EARLY_LEAVE: c_uint = 0x08;
pub const RTS5264_CMD_OE_EARLY_CYCLE_MASK: c_uint = 0x06;
pub const RTS5264_CMD_OE_EARLY_4CYCLE: c_uint = 0x06;
pub const RTS5264_CMD_OE_EARLY_3CYCLE: c_uint = 0x04;
pub const RTS5264_CMD_OE_EARLY_2CYCLE: c_uint = 0x02;
pub const RTS5264_CMD_OE_EARLY_1CYCLE: c_uint = 0x00;
pub const RTS5264_CMD_OE_EARLY_EN: c_uint = 0x01;
pub const RTS5264_DAT_OE_START_EARLY: c_uint = 0xFDCC;
pub const RTS5264_DAT_OE_EARLY_LEAVE: c_uint = 0x08;
pub const RTS5264_DAT_OE_EARLY_CYCLE_MASK: c_uint = 0x06;
pub const RTS5264_DAT_OE_EARLY_4CYCLE: c_uint = 0x06;
pub const RTS5264_DAT_OE_EARLY_3CYCLE: c_uint = 0x04;
pub const RTS5264_DAT_OE_EARLY_2CYCLE: c_uint = 0x02;
pub const RTS5264_DAT_OE_EARLY_1CYCLE: c_uint = 0x00;
pub const RTS5264_DAT_OE_EARLY_EN: c_uint = 0x01;
pub const RTS5264_LDO1233318_POW_CTL: c_uint = 0xFF70;

pub const RTS5264_DV3318_CFG: c_uint = 0xFF71;

pub const RTS5264_LDO1_CFG0: c_uint = 0xFF72;

pub const RTS5264_LDO1_CFG1: c_uint = 0xFF73;

pub const RTS5264_LDO2_CFG0: c_uint = 0xFF74;

pub const RTS5264_LDO2_CFG1: c_uint = 0xFF75;

pub const RTS5264_LDO3_CFG0: c_uint = 0xFF76;

pub const RTS5264_LDO3_CFG1: c_uint = 0xFF77;

pub const RTS5264_REG_PME_FORCE_CTL: c_uint = 0xFF78;
pub const FORCE_PM_CONTROL: c_uint = 0x20;
pub const FORCE_PM_VALUE: c_uint = 0x10;
pub const REG_EFUSE_BYPASS: c_uint = 0x08;
pub const REG_EFUSE_POR: c_uint = 0x04;
pub const REG_EFUSE_POWER_MASK: c_uint = 0x03;
pub const REG_EFUSE_POWERON: c_uint = 0x03;
pub const REG_EFUSE_POWEROFF: c_uint = 0x00;
pub const RTS5264_PWR_CUT: c_uint = 0xFF81;
pub const RTS5264_CFG_MEM_PD: c_uint = 0xF0;
pub const RTS5264_OVP_DET: c_uint = 0xFF8A;
pub const RTS5264_POW_VDET: c_uint = 0x04;
pub const RTS5264_TUNE_VROV_MASK: c_uint = 0x03;
pub const RTS5264_TUNE_VROV_2V: c_uint = 0x03;
pub const RTS5264_TUNE_VROV_1V8: c_uint = 0x02;
pub const RTS5264_TUNE_VROV_1V6: c_uint = 0x01;
pub const RTS5264_TUNE_VROV_1V4: c_uint = 0x00;
pub const RTS5264_CKMUX_MBIAS_PWR: c_uint = 0xFF8B;
pub const RTS5264_NON_XTAL_SEL: c_uint = 0x80;
pub const RTS5264_POW_CKMUX: c_uint = 0x40;
pub const RTS5264_LVD_MASK: c_uint = 0x04;
pub const RTS5264_POW_PSW_MASK: c_uint = 0x03;
pub const RTS5264_POW_PSW_DFT: c_uint = 0x03;
// Single LUN, support SD/SD EXPRESS
pub const DEFAULT_SINGLE: c_int = 0;
pub const SD_LUN: c_int = 1;
pub const SD_EXPRESS_LUN: c_int = 2;
pub const RTS5264_IC_VER_A: c_int = 0;
pub const RTS5264_IC_VER_B: c_int = 2;
pub const RTS5264_IC_VER_C: c_int = 3;
