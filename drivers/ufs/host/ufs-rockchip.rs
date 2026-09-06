//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-rockchip.h
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
// Rockchip UFS Host Controller driver
//
// Copyright (C) 2025 Rockchip Electronics Co., Ltd.
//
pub const SEL_TX_LANE0: c_uint = 0x0;
pub const SEL_TX_LANE1: c_uint = 0x1;
pub const SEL_TX_LANE2: c_uint = 0x2;
pub const SEL_TX_LANE3: c_uint = 0x3;
pub const SEL_RX_LANE0: c_uint = 0x4;
pub const SEL_RX_LANE1: c_uint = 0x5;
pub const SEL_RX_LANE2: c_uint = 0x6;
pub const SEL_RX_LANE3: c_uint = 0x7;
pub const VND_TX_CLK_PRD: c_uint = 0xAA;
pub const VND_TX_CLK_PRD_EN: c_uint = 0xA9;
pub const VND_TX_LINERESET_PVALUE2: c_uint = 0xAB;
pub const VND_TX_LINERESET_PVALUE1: c_uint = 0xAC;
pub const VND_TX_LINERESET_VALUE: c_uint = 0xAD;
pub const VND_TX_BASE_NVALUE: c_uint = 0x93;
pub const VND_TX_TASE_VALUE: c_uint = 0x94;
pub const VND_TX_POWER_SAVING_CTRL: c_uint = 0x7F;
pub const VND_RX_CLK_PRD: c_uint = 0x12;
pub const VND_RX_CLK_PRD_EN: c_uint = 0x11;
pub const VND_RX_LINERESET_PVALUE2: c_uint = 0x1B;
pub const VND_RX_LINERESET_PVALUE1: c_uint = 0x1C;
pub const VND_RX_LINERESET_VALUE: c_uint = 0x1D;
pub const VND_RX_LINERESET_OPTION: c_uint = 0x25;
pub const VND_RX_POWER_SAVING_CTRL: c_uint = 0x2F;
pub const VND_RX_SAVE_DET_CTRL: c_uint = 0x1E;
pub const CMN_REG23: c_uint = 0x8C;
pub const CMN_REG25: c_uint = 0x94;
pub const TRSV0_REG08: c_uint = 0xE0;
pub const TRSV1_REG08: c_uint = 0x220;
pub const TRSV0_REG14: c_uint = 0x110;
pub const TRSV1_REG14: c_uint = 0x250;
pub const TRSV0_REG15: c_uint = 0x134;
pub const TRSV1_REG15: c_uint = 0x274;
pub const TRSV0_REG16: c_uint = 0x128;
pub const TRSV1_REG16: c_uint = 0x268;
pub const TRSV0_REG17: c_uint = 0x12C;
pub const TRSV1_REG17: c_uint = 0x26c;
pub const TRSV0_REG18: c_uint = 0x120;
pub const TRSV1_REG18: c_uint = 0x260;
pub const TRSV0_REG29: c_uint = 0x164;
pub const TRSV1_REG29: c_uint = 0x2A4;
pub const TRSV0_REG2E: c_uint = 0x178;
pub const TRSV1_REG2E: c_uint = 0x2B8;
pub const TRSV0_REG3C: c_uint = 0x1B0;
pub const TRSV1_REG3C: c_uint = 0x2F0;
pub const TRSV0_REG3D: c_uint = 0x1B4;
pub const TRSV1_REG3D: c_uint = 0x2F4;
pub const MPHY_CFG: c_uint = 0x200;
pub const MPHY_CFG_ENABLE: c_uint = 0x40;
pub const MPHY_CFG_DISABLE: c_uint = 0x0;
pub const MIB_T_DBG_CPORT_TX_ENDIAN: c_uint = 0xc022;
pub const MIB_T_DBG_CPORT_RX_ENDIAN: c_uint = 0xc023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_rockchip_host {
    pub hba: *mut ufs_hba,
    pub ufs_phy_ctrl: *mut void __iomem,
    pub ufs_sys_ctrl: *mut void __iomem,
    pub mphy_base: *mut void __iomem,
    pub rst_gpio: *mut gpio_desc,
    pub rst: *mut reset_control,
    pub ref_out_clk: *mut clk,
    pub clks: *mut clk_bulk_data,
    pub caps: u64,
}

