//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_cwb.h
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
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cwb_mode_input {
    INPUT_MODE_LM_OUT,
    INPUT_MODE_DSPP_OUT,
    INPUT_MODE_MAX
}

//
// struct dpu_hw_cwb_setup_cfg : Describes configuration for CWB mux
// @pp_idx:        Index of the real-time pinpong that the CWB mux will
// feed the CWB mux
// @input:         Input tap point
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cwb_setup_cfg {
    pub pp_idx: dpu_pingpong,
    pub input: cwb_mode_input,
}

//
// struct dpu_hw_cwb_ops : Interface to the cwb hw driver functions
// @config_cwb: configure CWB mux
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cwb_ops {
    pub cwb_cfg): *mut dpu_hw_cwb_setup_cfg,
}

//
// struct dpu_hw_cwb : CWB mux driver object
// @base: Hardware block base structure
// @hw: Block hardware details
// @idx: CWB index
// @ops: handle to operations possible for this CWB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_cwb {
    pub base: dpu_hw_blk,
    pub hw: dpu_hw_blk_reg_map,
    pub idx: dpu_cwb,
    pub ops: dpu_hw_cwb_ops,
}

//
// to_dpu_hw_cwb - convert base object dpu_hw_base to container
// @hw: Pointer to base hardware block
// return: Pointer to hardware block container
//
extern "C" {
    pub fn container_of(_arg: hw, dpu_hw_cwb: struct, _arg: base) -> return;
}
