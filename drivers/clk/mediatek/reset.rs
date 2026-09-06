//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/reset.h
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
// Copyright (c) 2022 MediaTek Inc.
//

pub const RST_NR_PER_BANK: c_int = 32;
// Infra global controller reset set register
pub const INFRA_RST0_SET_OFFSET: c_uint = 0x120;
pub const INFRA_RST1_SET_OFFSET: c_uint = 0x130;
pub const INFRA_RST2_SET_OFFSET: c_uint = 0x140;
pub const INFRA_RST3_SET_OFFSET: c_uint = 0x150;
pub const INFRA_RST4_SET_OFFSET: c_uint = 0x730;
//
// enum mtk_reset_version - Version of MediaTek clock reset controller.
// @MTK_RST_SIMPLE: Use the same registers for bit set and clear.
// @MTK_RST_SET_CLR: Use separate registers for bit set and clear.
// @MTK_RST_MAX: Total quantity of version for MediaTek clock reset controller.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_reset_version {
    MTK_RST_SIMPLE = 0,
    MTK_RST_SET_CLR,
    MTK_RST_MAX,
}

//
// struct mtk_clk_rst_desc - Description of MediaTek clock reset.
// @version: Reset version which is defined in enum mtk_reset_version.
// @rst_bank_ofs: Pointer to an array containing base offsets of the reset register.
// @rst_bank_nr: Quantity of reset bank.
// @rst_idx_map:Pointer to an array containing ids if input argument is index.
// This array is not necessary if our input argument does not mean index.
// @rst_idx_map_nr: Quantity of reset index map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_clk_rst_desc {
    pub version: mtk_reset_version,
    pub rst_bank_ofs: *mut u16,
    pub rst_bank_nr: u32,
    pub rst_idx_map: *mut u16,
    pub rst_idx_map_nr: u32,
}

//
// struct mtk_clk_rst_data - Data of MediaTek clock reset controller.
// @regmap: Pointer to base address of reset register address.
// @rcdev: Reset controller device.
// @desc: Pointer to description of the reset controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_clk_rst_data {
    pub regmap: *mut regmap,
    pub rcdev: reset_controller_dev,
    pub desc: *const mtk_clk_rst_desc,
}

//
// mtk_register_reset_controller - Register mediatek clock reset controller with device
// @np: Pointer to device.
// @desc: Constant pointer to description of clock reset.
//
// Return: 0 on success and errorno otherwise.
//
