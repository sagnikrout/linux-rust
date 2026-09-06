//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dc/dc-fu.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2024 NXP
//

pub const FRAC_OFFSET: c_uint = 0x28;
pub const STATICCONTROL: c_uint = 0x8;
pub const BURSTBUFFERMANAGEMENT: c_uint = 0xc;
// COLORCOMPONENTBITS

// COLORCOMPONENTSHIFT

// LAYERPROPERTY

// FRAMEDIMENSIONS

// CONTROL

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_yuvconversionmode {
    YUVCONVERSIONMODE_OFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_inputselect {
    INPUTSELECT_INACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_rastermode {
    RASTERMODE_NORMAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_fu_frac {
    DC_FETCHUNIT_FRAC0,
    DC_FETCHUNIT_FRAC1,
    DC_FETCHUNIT_FRAC2,
    DC_FETCHUNIT_FRAC3,
    DC_FETCHUNIT_FRAC4,
    DC_FETCHUNIT_FRAC5,
    DC_FETCHUNIT_FRAC6,
    DC_FETCHUNIT_FRAC7,
    DC_FETCHUNIT_FRAC_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_fu_ops {
    pub fu): *mut *mut void (init)(struct dc_fu,
    pub baddr): *mut *mut *mut void (set_burstlength)(struct dc_fu fu, dma_addr_t,
    pub baddr): dma_addr_t,
    pub stride): c_uint,
    pub h): int w, int,
    pub format): *const drm_format_info,
    pub frac): *mut *mut *mut void (enable_src_buf)(struct dc_fu fu, enum dc_fu_frac,
    pub frac): *mut *mut *mut void (disable_src_buf)(struct dc_fu fu, enum dc_fu_frac,
    pub h): *mut *mut *mut void (set_framedimensions)(struct dc_fu fu, int w, int,
    pub lb): *mut *mut *mut void (set_layerblend)(struct dc_fu fu, struct dc_lb,
    pub fu): *mut *mut dc_link_id (get_link_id)(struct dc_fu,
    pub fu): *const *const *const char (get_name)(struct dc_fu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_fu {
    pub reg_pec: *mut regmap,
    pub reg_cfg: *mut regmap,
    pub name: [c_char; 21],
    pub reg_baseaddr: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_sourcebufferattributes: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_sourcebufferdimension: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_layeroffset: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_clipwindowoffset: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_clipwindowdimensions: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_constantcolor: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub reg_layerproperty: [u32; DC_FETCHUNIT_FRAC_NUM],
    pub id: c_uint,
    pub link_id: dc_link_id,
    pub ops: dc_fu_ops,
    pub lb: *mut dc_lb,
}

extern "C" {
    pub fn dc_fu_get_pixel_format_bits(fu: *mut dc_fu, format: u32, bits: *mut u32);
}
extern "C" {
    pub fn dc_fu_get_pixel_format_shifts(fu: *mut dc_fu, format: u32, shifts: *mut u32);
}
extern "C" {
    pub fn dc_fu_shdldreq_sticky(fu: *mut dc_fu, layer_mask: u8);
}
extern "C" {
    pub fn dc_fu_set_src_bpp(fu: *mut dc_fu, frac: dc_fu_frac, bpp: c_uint);
}
extern "C" {
    pub fn dc_fu_common_hw_init(fu: *mut dc_fu);
}
