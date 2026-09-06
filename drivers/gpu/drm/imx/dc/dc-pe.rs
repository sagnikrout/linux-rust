//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dc/dc-pe.h
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

pub const CLKEN_MASK_SHIFT: c_int = 24;

pub const DC_DISP_FU_CNT: c_int = 2;
pub const DC_LB_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_link_id {
    LINK_ID_NONE		= 0x00,
    LINK_ID_CONSTFRAME0	= 0x0c,
    LINK_ID_CONSTFRAME4	= 0x0e,
    LINK_ID_CONSTFRAME1	= 0x10,
    LINK_ID_CONSTFRAME5	= 0x12,
    LINK_ID_FETCHWARP2	= 0x14,
    LINK_ID_FETCHLAYER0	= 0x1a,
    LINK_ID_LAYERBLEND0	= 0x21,
    LINK_ID_LAYERBLEND1	= 0x22,
    LINK_ID_LAYERBLEND2	= 0x23,
    LINK_ID_LAYERBLEND3	= 0x24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_lb_mode {
    LB_NEUTRAL,	/* Output is same as primary input. */
    LB_BLEND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_pec_clken {
    CLKEN_DISABLE,
    CLKEN_AUTOMATIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cf {
    pub reg_cfg: *mut regmap,
    pub link: dc_link_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_ed {
    pub dev: *mut device,
    pub reg_pec: *mut regmap,
    pub reg_cfg: *mut regmap,
    pub irq_shdload: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_lb {
    pub dev: *mut device,
    pub reg_pec: *mut regmap,
    pub reg_cfg: *mut regmap,
    pub id: c_int,
    pub link: dc_link_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_pe {
    pub dev: *mut device,
    pub clk_axi: *mut clk,
    pub cf_safe: [*mut dc_cf; DC_DISPLAYS],
    pub cf_cont: [*mut dc_cf; DC_DISPLAYS],
    pub ed_safe: [*mut dc_ed; DC_DISPLAYS],
    pub ed_cont: [*mut dc_ed; DC_DISPLAYS],
    pub fu_disp: [*mut dc_fu; DC_DISP_FU_CNT],
    pub lb: [*mut dc_lb; DC_LB_CNT],
}

// Constant Frame Unit
extern "C" {
    pub fn dc_cf_get_link_id(cf: *mut dc_cf) -> dc_link_id;
}
extern "C" {
    pub fn dc_cf_framedimensions(cf: *mut dc_cf, w: c_uint, h: c_uint);
}
extern "C" {
    pub fn dc_cf_constantcolor_black(cf: *mut dc_cf);
}
extern "C" {
    pub fn dc_cf_constantcolor_blue(cf: *mut dc_cf);
}
extern "C" {
    pub fn dc_cf_init(cf: *mut dc_cf);
}
// External Destination Unit
extern "C" {
    pub fn dc_ed_pec_src_sel(ed: *mut dc_ed, src: dc_link_id);
}
extern "C" {
    pub fn dc_ed_pec_sync_trigger(ed: *mut dc_ed);
}
extern "C" {
    pub fn dc_ed_init(ed: *mut dc_ed);
}
// Layer Blend Unit
extern "C" {
    pub fn dc_lb_get_link_id(lb: *mut dc_lb) -> dc_link_id;
}
extern "C" {
    pub fn dc_lb_pec_dynamic_prim_sel(lb: *mut dc_lb, prim: dc_link_id);
}
extern "C" {
    pub fn dc_lb_pec_dynamic_sec_sel(lb: *mut dc_lb, sec: dc_link_id);
}
extern "C" {
    pub fn dc_lb_pec_clken(lb: *mut dc_lb, clken: dc_pec_clken);
}
extern "C" {
    pub fn dc_lb_mode(lb: *mut dc_lb, mode: dc_lb_mode);
}
extern "C" {
    pub fn dc_lb_position(lb: *mut dc_lb, x: c_int, y: c_int);
}
extern "C" {
    pub fn dc_lb_get_id(lb: *mut dc_lb) -> c_int;
}
extern "C" {
    pub fn dc_lb_init(lb: *mut dc_lb);
}
