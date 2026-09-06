//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkcif/rkcif-regs.h
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
// Rockchip Camera Interface (CIF) Driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
// Copyright (C) 2023 Mehdi Djait <mehdi.djait@bootlin.com>
// Copyright (C) 2025 Michael Riesch <michael.riesch@wolfvision.net>
//
pub const RKCIF_REGISTER_NOTSUPPORTED: c_uint = 0x420000;

// DVP register contents

pub const RKCIF_INTSTAT_CLS: c_uint = 0x3ff;

pub const RKCIF_INTSTAT_ERR: c_uint = 0xfc;
pub const RKCIF_FRAME_STAT_CLS: c_uint = 0x00;
pub const RKCIF_FRAME_FRM0_STAT_CLS: c_uint = 0x20;

// GRF register offsets and contents
pub const RK3568_GRF_VI_CON0: c_uint = 0x340;
pub const RK3568_GRF_VI_CON1: c_uint = 0x344;
pub const RK3568_GRF_VI_STATUS0: c_uint = 0x348;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_dvp_register_index {
    RKCIF_DVP_CTRL,
    RKCIF_DVP_INTEN,
    RKCIF_DVP_INTSTAT,
    RKCIF_DVP_FOR,
    RKCIF_DVP_LINE_NUM_ADDR,
    RKCIF_DVP_FRM0_ADDR_Y,
    RKCIF_DVP_FRM0_ADDR_UV,
    RKCIF_DVP_FRM1_ADDR_Y,
    RKCIF_DVP_FRM1_ADDR_UV,
    RKCIF_DVP_VIR_LINE_WIDTH,
    RKCIF_DVP_SET_SIZE,
    RKCIF_DVP_SCL_CTRL,
    RKCIF_DVP_CROP,
    RKCIF_DVP_FRAME_STATUS,
    RKCIF_DVP_LAST_LINE,
    RKCIF_DVP_LAST_PIX,
    RKCIF_DVP_REGISTER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_mipi_register_index {
    RKCIF_MIPI_CTRL,
    RKCIF_MIPI_INTEN,
    RKCIF_MIPI_INTSTAT,
    RKCIF_MIPI_REGISTER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_mipi_id_register_index {
    RKCIF_MIPI_CTRL0,
    RKCIF_MIPI_CTRL1,
    RKCIF_MIPI_FRAME0_ADDR_Y,
    RKCIF_MIPI_FRAME0_ADDR_UV,
    RKCIF_MIPI_FRAME0_VLW_Y,
    RKCIF_MIPI_FRAME0_VLW_UV,
    RKCIF_MIPI_FRAME1_ADDR_Y,
    RKCIF_MIPI_FRAME1_ADDR_UV,
    RKCIF_MIPI_FRAME1_VLW_Y,
    RKCIF_MIPI_FRAME1_VLW_UV,
    RKCIF_MIPI_CROP_START,
    RKCIF_MIPI_ID_REGISTER_MAX
}
