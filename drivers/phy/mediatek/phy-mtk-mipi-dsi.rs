//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/mediatek/phy-mtk-mipi-dsi.h
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
// Copyright (c) 2019 MediaTek Inc.
// Author: Jitao Shi <jitao.shi@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mipitx_data {
    pub mppll_preserve: u32,
    pub mipi_tx_clk_ops: *const clk_ops,
    pub phy): *mut *mut void (mipi_tx_enable_signal)(struct phy,
    pub phy): *mut *mut void (mipi_tx_disable_signal)(struct phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mipi_tx {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub data_rate: u32,
    pub mipitx_drive: u32,
    pub rt_code: [u32; 5],
    pub driver_data: *const mtk_mipitx_data,
    pub pll_hw: clk_hw,
}
