//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/regs.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
pub const MT_HW_REV: c_uint = 0x1000;
pub const MT_HW_CHIPID: c_uint = 0x1008;
pub const MT_TOP_MISC2: c_uint = 0x1134;
pub const MT_MCU_BASE: c_uint = 0x2000;

pub const MT_HIF_BASE: c_uint = 0x4000;

pub const MT_PSE_BASE: c_uint = 0x8000;

pub const MT_WF_PHY_BASE: c_uint = 0x10000;
pub const MT_WF_PHY_OFFSET: c_uint = 0x1000;

pub const MT_WF_AGG_BASE: c_uint = 0x21200;

pub const MT_AGG_BA_SIZE_LIMIT_SHIFT: c_int = 8;

pub const MT_WF_DMA_BASE: c_uint = 0x21c00;

pub const MT_WF_ARB_BASE: c_uint = 0x21400;

pub const MT_WF_ARB_CAB_COUNT_SHIFT: c_int = 4;

pub const MT_WF_TMAC_BASE: c_uint = 0x21600;

pub const MT_WF_RMAC_BASE: c_uint = 0x21800;

pub const MT_WF_SEC_BASE: c_uint = 0x21a00;

pub const MT_WF_CFG_OFF_BASE: c_uint = 0x21e00;

pub const MT_WTBL_OFF_BASE: c_uint = 0x23000;

pub const MT_LPON_BASE: c_uint = 0x24000;

pub const MT_PRE_TBTT_SHIFT: c_int = 8;

pub const MT_INT_WAKEUP_BASE: c_uint = 0x24400;

pub const MT_WTBL1_BASE: c_uint = 0x28000;

pub const MT_MIB_BASE: c_uint = 0x2c000;

pub const MT_PCIE_REMAP_BASE_1: c_uint = 0x40000;
pub const MT_PCIE_REMAP_BASE_2: c_uint = 0x80000;
pub const MT_TX_HW_QUEUE_MGMT: c_int = 4;
pub const MT_TX_HW_QUEUE_MCU: c_int = 5;
pub const MT_TX_HW_QUEUE_BCN: c_int = 7;
pub const MT_TX_HW_QUEUE_BMC: c_int = 8;
pub const MT_LED_BASE_PHYS: c_uint = 0x80024000;

pub const MT_CLIENT_BASE_PHYS_ADDR: c_uint = 0x800c0000;
pub const MT_CLIENT_TMAC_INFO_TEMPLATE: c_uint = 0x040;
pub const MT_CLIENT_STATUS: c_uint = 0x06c;
pub const MT_CLIENT_RESET_TX: c_uint = 0x070;

pub const MT_EFUSE_BASE: c_uint = 0x81070000;
pub const MT_EFUSE_BASE_CTRL: c_uint = 0x000;

pub const MT_EFUSE_CTRL: c_uint = 0x008;

pub const MT_CLIENT_RXINF: c_uint = 0x068;

pub const MT_PSE_BASE_PHYS_ADDR: c_uint = 0xa0000000;
pub const MT_PSE_WTBL_2_PHYS_ADDR: c_uint = 0xa5000000;

pub const MT_WTBL2_W15_BA_WIN_SIZE_SHIFT: c_int = 3;

