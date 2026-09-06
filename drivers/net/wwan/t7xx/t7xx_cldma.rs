//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_cldma.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const CLDMA_TXQ_NUM: c_int = 8;
pub const CLDMA_RXQ_NUM: c_int = 8;

// Interrupt status bits

pub const EQ_STA_BIT_OFFSET: c_int = 8;
pub const L2_INT_BIT_COUNT: c_int = 16;

pub const CLDMA0_AO_BASE: c_uint = 0x10049000;
pub const CLDMA0_PD_BASE: c_uint = 0x1021d000;
pub const CLDMA1_AO_BASE: c_uint = 0x1004b000;
pub const CLDMA1_PD_BASE: c_uint = 0x1021f000;
pub const CLDMA_R_AO_BASE: c_uint = 0x10023000;
pub const CLDMA_R_PD_BASE: c_uint = 0x1023d000;
// CLDMA TX
pub const REG_CLDMA_UL_START_ADDRL_0: c_uint = 0x0004;
pub const REG_CLDMA_UL_START_ADDRH_0: c_uint = 0x0008;
pub const REG_CLDMA_UL_CURRENT_ADDRL_0: c_uint = 0x0044;
pub const REG_CLDMA_UL_CURRENT_ADDRH_0: c_uint = 0x0048;
pub const REG_CLDMA_UL_STATUS: c_uint = 0x0084;
pub const REG_CLDMA_UL_START_CMD: c_uint = 0x0088;
pub const REG_CLDMA_UL_RESUME_CMD: c_uint = 0x008c;
pub const REG_CLDMA_UL_STOP_CMD: c_uint = 0x0090;
pub const REG_CLDMA_UL_ERROR: c_uint = 0x0094;
pub const REG_CLDMA_UL_CFG: c_uint = 0x0098;

pub const REG_CLDMA_UL_MEM: c_uint = 0x009c;

// CLDMA RX
pub const REG_CLDMA_DL_START_CMD: c_uint = 0x05bc;
pub const REG_CLDMA_DL_RESUME_CMD: c_uint = 0x05c0;
pub const REG_CLDMA_DL_STOP_CMD: c_uint = 0x05c4;
pub const REG_CLDMA_DL_MEM: c_uint = 0x0508;

pub const REG_CLDMA_DL_CFG: c_uint = 0x0404;

pub const REG_CLDMA_DL_START_ADDRL_0: c_uint = 0x0478;
pub const REG_CLDMA_DL_START_ADDRH_0: c_uint = 0x047c;
pub const REG_CLDMA_DL_CURRENT_ADDRL_0: c_uint = 0x04b8;
pub const REG_CLDMA_DL_CURRENT_ADDRH_0: c_uint = 0x04bc;
pub const REG_CLDMA_DL_STATUS: c_uint = 0x04f8;
// CLDMA MISC
pub const REG_CLDMA_L2TISAR0: c_uint = 0x0810;
pub const REG_CLDMA_L2TISAR1: c_uint = 0x0814;
pub const REG_CLDMA_L2TIMR0: c_uint = 0x0818;
pub const REG_CLDMA_L2TIMR1: c_uint = 0x081c;
pub const REG_CLDMA_L2TIMCR0: c_uint = 0x0820;
pub const REG_CLDMA_L2TIMCR1: c_uint = 0x0824;
pub const REG_CLDMA_L2TIMSR0: c_uint = 0x0828;
pub const REG_CLDMA_L2TIMSR1: c_uint = 0x082c;
pub const REG_CLDMA_L3TISAR0: c_uint = 0x0830;
pub const REG_CLDMA_L3TISAR1: c_uint = 0x0834;
pub const REG_CLDMA_L2RISAR0: c_uint = 0x0850;
pub const REG_CLDMA_L2RISAR1: c_uint = 0x0854;
pub const REG_CLDMA_L3RISAR0: c_uint = 0x0870;
pub const REG_CLDMA_L3RISAR1: c_uint = 0x0874;
pub const REG_CLDMA_IP_BUSY: c_uint = 0x08b4;

// CLDMA MISC
pub const REG_CLDMA_L2RIMR0: c_uint = 0x0858;
pub const REG_CLDMA_L2RIMR1: c_uint = 0x085c;
pub const REG_CLDMA_L2RIMCR0: c_uint = 0x0860;
pub const REG_CLDMA_L2RIMCR1: c_uint = 0x0864;
pub const REG_CLDMA_L2RIMSR0: c_uint = 0x0868;
pub const REG_CLDMA_L2RIMSR1: c_uint = 0x086c;
pub const REG_CLDMA_BUSY_MASK: c_uint = 0x0954;

pub const REG_CLDMA_INT_MASK: c_uint = 0x0960;
// CLDMA RESET
pub const REG_INFRA_RST4_SET: c_uint = 0x0730;

pub const REG_INFRA_RST4_CLR: c_uint = 0x0734;

pub const REG_INFRA_RST2_SET: c_uint = 0x0140;

pub const REG_INFRA_RST2_CLR: c_uint = 0x0144;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_txrx {
    MTK_TX,
    MTK_RX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_hw_mode {
    MODE_BIT_32,
    MODE_BIT_36,
    MODE_BIT_40,
    MODE_BIT_64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_cldma_hw {
    pub hw_mode: t7xx_hw_mode,
    pub ap_ao_base: *mut void __iomem,
    pub ap_pdn_base: *mut void __iomem,
    pub phy_interrupt_id: u32,
}

extern "C" {
    pub fn t7xx_cldma_hw_irq_en_eq(hw_info: *mut t7xx_cldma_hw, qno: c_uint, tx_rx: mtk_txrx);
}
extern "C" {
    pub fn t7xx_cldma_hw_init(hw_info: *mut t7xx_cldma_hw);
}
extern "C" {
    pub fn t7xx_cldma_hw_start(hw_info: *mut t7xx_cldma_hw);
}
extern "C" {
    pub fn t7xx_cldma_hw_tx_done(hw_info: *mut t7xx_cldma_hw, bitmask: c_uint);
}
extern "C" {
    pub fn t7xx_cldma_hw_rx_done(hw_info: *mut t7xx_cldma_hw, bitmask: c_uint);
}
extern "C" {
    pub fn t7xx_cldma_hw_stop_all_qs(hw_info: *mut t7xx_cldma_hw, tx_rx: mtk_txrx);
}
extern "C" {
    pub fn t7xx_cldma_hw_reset(ao_base: *mut void __iomem);
}
extern "C" {
    pub fn t7xx_cldma_hw_stop(hw_info: *mut t7xx_cldma_hw, tx_rx: mtk_txrx);
}
extern "C" {
    pub fn t7xx_cldma_hw_restore(hw_info: *mut t7xx_cldma_hw);
}
extern "C" {
    pub fn t7xx_cldma_clear_ip_busy(hw_info: *mut t7xx_cldma_hw);
}
extern "C" {
    pub fn t7xx_cldma_tx_addr_is_set(hw_info: *mut t7xx_cldma_hw, qno: c_uint) -> bool;
}
