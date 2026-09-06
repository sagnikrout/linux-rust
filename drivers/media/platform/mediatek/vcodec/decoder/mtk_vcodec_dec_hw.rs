//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/mtk_vcodec_dec_hw.h
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
// Copyright (c) 2021 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//

pub const VDEC_HW_ACTIVE_ADDR: c_uint = 0x0;

pub const VDEC_IRQ_CFG: c_uint = 0x11;
pub const VDEC_IRQ_CLR: c_uint = 0x10;
pub const VDEC_IRQ_CFG_REG: c_uint = 0xa4;

//
// enum mtk_vdec_hw_reg_idx - subdev hardware register base index
// @VDEC_HW_SYS : vdec soc register index
// @VDEC_HW_MISC: vdec misc register index
// @VDEC_HW_MAX : vdec supported max register index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_hw_reg_idx {
    VDEC_HW_SYS,
    VDEC_HW_MISC,
    VDEC_HW_MAX
}

//
// struct mtk_vdec_hw_dev - vdec hardware driver data
// @plat_dev: platform device
// @main_dev: main device
// @reg_base: mapped address of MTK Vcodec registers.
//
// @curr_ctx: the context that is waiting for codec hardware
//
// @dec_irq : decoder irq resource
// @pm      : power management control
// @hw_idx  : each hardware index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vdec_hw_dev {
    pub plat_dev: *mut platform_device,
    pub main_dev: *mut mtk_vcodec_dec_dev,
    pub reg_base: [*mut void __iomem; VDEC_HW_MAX],
    pub curr_ctx: *mut mtk_vcodec_dec_ctx,
    pub dec_irq: c_int,
    pub pm: mtk_vcodec_pm,
    pub hw_idx: c_int,
}
