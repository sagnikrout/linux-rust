//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_dspp.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
//
// struct dpu_hw_pcc_coeff - PCC coefficient structure for each color
// component.
// @r: red coefficient.
// @g: green coefficient.
// @b: blue coefficient.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pcc_coeff {
    pub r: __u32,
    pub g: __u32,
    pub b: __u32,
}

//
// struct dpu_hw_pcc_cfg - pcc feature structure
// @r: red coefficients.
// @g: green coefficients.
// @b: blue coefficients.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_pcc_cfg {
    pub r: dpu_hw_pcc_coeff,
    pub g: dpu_hw_pcc_coeff,
    pub b: dpu_hw_pcc_coeff,
}

pub const DPU_GAMMA_LUT_SIZE: c_int = 1024;
pub const PGC_TBL_LEN: c_int = 512;

//
// struct dpu_hw_gc_lut - gc lut feature structure
// @flags: flags for the feature values can be:
// - PGC_8B_ROUND
// @c0: color0 component lut
// @c1: color1 component lut
// @c2: color2 component lut
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_gc_lut {
    pub flags: __u64,
    pub c0: [__u32; PGC_TBL_LEN],
    pub c1: [__u32; PGC_TBL_LEN],
    pub c2: [__u32; PGC_TBL_LEN],
}

//
// struct dpu_hw_dspp_ops - interface to the dspp hardware driver functions
// Caller must call the init function to get the dspp context for each dspp
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_dspp_ops {
//
// @setup_pcc: setup_pcc - setup dspp pcc
// @ctx: Pointer to dspp context
// @cfg: Pointer to configuration
//
    pub cfg): *mut *mut *mut void (setup_pcc)(struct dpu_hw_dspp ctx, struct dpu_hw_pcc_cfg,
//
// setup_gc - setup dspp gc
// @ctx: Pointer to dspp context
// @gc_lut: Pointer to lut content
//
    pub gc_lut): *mut *mut *mut void (setup_gc)(struct dpu_hw_dspp ctx, struct dpu_hw_gc_lut,
}

//
// struct dpu_hw_dspp - dspp description
// @base: Hardware block base structure
// @hw: Block hardware details
// @idx: DSPP index
// @cap: Pointer to layer_cfg
// @ops: Pointer to operations possible for this DSPP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_dspp {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
// dspp
    pub idx: c_int,
    pub cap: *const dpu_dspp_cfg,
// Ops
    pub ops: dpu_hw_dspp_ops,
}

//
// to_dpu_hw_dspp - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_dspp: struct, _arg: base) -> return;
}
