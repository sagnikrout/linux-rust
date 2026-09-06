//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/regs.h
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
// Copyright (C) 2019 MediaTek Inc.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_reg_base {
    MT_TOP_CFG_BASE,
    MT_HW_BASE,
    MT_DMA_SHDL_BASE,
    MT_PCIE_REMAP_2,
    MT_ARB_BASE,
    MT_HIF_BASE,
    MT_CSR_BASE,
    MT_PLE_BASE,
    MT_PSE_BASE,
    MT_CFG_BASE,
    MT_AGG_BASE,
    MT_TMAC_BASE,
    MT_RMAC_BASE,
    MT_DMA_BASE,
    MT_PF_BASE,
    MT_WTBL_BASE_ON,
    MT_WTBL_BASE_OFF,
    MT_LPON_BASE,
    MT_MIB_BASE,
    MT_WTBL_BASE_ADDR,
    MT_PCIE_REMAP_BASE2,
    MT_TOP_MISC_BASE,
    MT_EFUSE_ADDR_BASE,
    MT_PP_BASE,
    __MT_BASE_MAX,
}

pub const MT_TOP_OFF_RSV: c_uint = 0x1128;

pub const MT_MCU_BASE: c_uint = 0x2000;

pub const MT_PCIE_REMAP_BASE_1: c_uint = 0x40000;

pub const MT_MCU_CIRQ_BASE: c_uint = 0xc0000;

pub const MT_HIF2_BASE: c_uint = 0xf0000;

pub const MT_WF_PHY_BASE: c_uint = 0x82070000;

pub const MT_ARB_RQCR_BAND_SHIFT: c_int = 16;

pub const MT_WTBL_ENTRY_SIZE: c_int = 256;

pub const MT_DMASHDL_BASE: c_uint = 0x5000a000;
pub const MT_DMASHDL_OPTIONAL: c_uint = 0x008;
pub const MT_DMASHDL_PAGE: c_uint = 0x00c;
pub const MT_DMASHDL_REFILL: c_uint = 0x010;
pub const MT_DMASHDL_PKT_MAX_SIZE: c_uint = 0x01c;

pub const MT_DMASHDL_SCHED_SET0: c_uint = 0x0b0;
pub const MT_DMASHDL_SCHED_SET1: c_uint = 0x0b4;

pub const MT_LED_BASE_PHYS: c_uint = 0x80024000;

pub const MT_PDMA_BUSY: c_uint = 0x82000504;

pub const MT_EFUSE_BASE_CTRL: c_uint = 0x000;

pub const MT_EFUSE_CTRL: c_uint = 0x008;

// INFRACFG host register range on MT7622
pub const MT_INFRACFG_MISC: c_uint = 0x700;

pub const MT_UMAC_BASE: c_uint = 0x7c000000;

pub const MT_MCU_PTA_BASE: c_uint = 0x81060000;

