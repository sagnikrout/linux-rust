//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/fsl-dcu/fsl_dcu_drm_drv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2015 Freescale Semiconductor, Inc.
//
// Freescale DCU drm device driver
//

pub const DCU_DCU_MODE: c_uint = 0x0010;

pub const DCU_MODE_DCU_MODE_MASK: c_uint = 0x03;
pub const DCU_MODE_OFF: c_int = 0;
pub const DCU_MODE_NORMAL: c_int = 1;
pub const DCU_MODE_TEST: c_int = 2;
pub const DCU_MODE_COLORBAR: c_int = 3;
pub const DCU_BGND: c_uint = 0x0014;

pub const DCU_DISP_SIZE: c_uint = 0x0018;

// Regisiter value 1/16 of horizontal resolution

pub const DCU_HSYN_PARA: c_uint = 0x001c;

pub const DCU_VSYN_PARA: c_uint = 0x0020;

pub const DCU_SYN_POL: c_uint = 0x0024;

pub const DCU_THRESHOLD: c_uint = 0x0028;

pub const BF_VS_VAL: c_uint = 0x03;
pub const BUF_MAX_VAL: c_uint = 0x78;
pub const BUF_MIN_VAL: c_uint = 0x0a;
pub const DCU_INT_STATUS: c_uint = 0x002C;

pub const DCU_INT_MASK: c_uint = 0x0030;

pub const DCU_DIV_RATIO: c_uint = 0x0054;
pub const DCU_UPDATE_MODE: c_uint = 0x00cc;

pub const DCU_DCFB_MAX: c_uint = 0x300;

pub const DCU_LAYER_AB_NONE: c_int = 0;
pub const DCU_LAYER_AB_CHROMA_KEYING: c_int = 1;
pub const DCU_LAYER_AB_WHOLE_FRAME: c_int = 2;

pub const FSL_DCU_RGB565: c_int = 4;
pub const FSL_DCU_RGB888: c_int = 5;
pub const FSL_DCU_ARGB8888: c_int = 6;
pub const FSL_DCU_ARGB1555: c_int = 11;
pub const FSL_DCU_ARGB4444: c_int = 12;
pub const FSL_DCU_YUV422: c_int = 14;
pub const SCFG_PIXCLKCR: c_uint = 0x28;

pub const VF610_LAYER_REG_NUM: c_int = 9;
pub const LS1021A_LAYER_REG_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dcu_soc_data {
    pub name: *const c_char,
// total layer number
    pub total_layer: c_uint,
// max layer number DCU supported
    pub max_layer: c_uint,
    pub layer_regs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dcu_drm_device {
    pub dev: *mut device,
    pub np: *mut device_node,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub clk: *mut clk,
    pub pix_clk: *mut clk,
    pub tcon: *mut fsl_tcon,
// protects hardware register
    pub irq_lock: spinlock_t,
    pub drm: *mut drm_device,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: fsl_dcu_drm_connector,
    pub soc: *const fsl_dcu_soc_data,
}

extern "C" {
    pub fn fsl_dcu_drm_modeset_init(fsl_dev: *mut fsl_dcu_drm_device) -> c_int;
}
