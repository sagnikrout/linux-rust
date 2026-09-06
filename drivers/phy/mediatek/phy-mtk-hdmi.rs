//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/mediatek/phy-mtk-hdmi.h
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
// Copyright (c) 2018 MediaTek Inc.
// Author: Chunhui Dai <chunhui.dai@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi_phy_conf {
    pub flags: c_ulong,
    pub pll_default_off: bool,
    pub hdmi_phy_regulator_desc: *const regulator_desc,
    pub hdmi_phy_clk_ops: *const clk_ops,
    pub hdmi_phy): *mut *mut void (hdmi_phy_enable_tmds)(struct mtk_hdmi_phy,
    pub hdmi_phy): *mut *mut void (hdmi_phy_disable_tmds)(struct mtk_hdmi_phy,
    pub opts): *mut *mut *mut int (hdmi_phy_configure)(struct phy phy, union phy_configure_opts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi_phy {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub conf: *mut mtk_hdmi_phy_conf,
    pub pll: *mut clk,
    pub pll_hw: clk_hw,
    pub rdev: *mut regulator_dev,
    pub pll_rate: c_ulong,
    pub drv_imp_clk: c_uchar,
    pub drv_imp_d2: c_uchar,
    pub drv_imp_d1: c_uchar,
    pub drv_imp_d0: c_uchar,
    pub ibias: c_uint,
    pub ibias_up: c_uint,
    pub tmds_over_340M: bool,
}
