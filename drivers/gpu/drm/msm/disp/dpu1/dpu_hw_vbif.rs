//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_vbif.h
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

//
// struct dpu_hw_vbif_ops : Interface to the VBIF hardware driver functions
// Assumption is these functions will be called after clocks are enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_vbif_ops {
//
// @set_limit_conf: set transaction limit config
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @rd: true for read limit; false for write limit
// @limit: outstanding transaction limit
//
    pub limit): u32 xin_id, bool rd, u32,
//
// @get_limit_conf: get transaction limit config
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @rd: true for read limit; false for write limit
// @return: outstanding transaction limit
//
    pub rd): u32 xin_id, bool,
//
// @set_halt_ctrl: set halt control
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @enable: halt control enable
//
    pub enable): u32 xin_id, bool,
//
// @get_halt_ctrl: get halt control
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @return: halt control enable
//
    pub xin_id): u32,
//
// @set_qos_remap: set QoS priority remap
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @level: priority level
// @remap_level: remapped level
//
    pub remap_level): u32 xin_id, u32 level, u32,
//
// @set_mem_type: set memory type
// @vbif: vbif context driver
// @xin_id: client interface identifier
// @value: memory type value
//
    pub value): u32 xin_id, u32,
//
// @clear_errors: clear any vbif errors
// This function clears any detected pending/source errors
// on the VBIF interface, and optionally returns the detected
// error mask(s).
// @vbif: vbif context driver
// @pnd_errors: pointer to pending error reporting variable
// @src_errors: pointer to source error reporting variable
//
    pub src_errors): *mut *mut u32 pnd_errors, u32,
//
// @set_write_gather_en: set write_gather enable
// @vbif: vbif context driver
// @xin_id: client interface identifier
//
    pub xin_id): *mut *mut *mut void (set_write_gather_en)(struct dpu_hw_vbif vbif, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_vbif {
// base
    pub hw: dpu_hw_blk_reg_map,
// vbif
    pub cap: *const dpu_vbif_cfg,
// ops
    pub ops: dpu_hw_vbif_ops,
}
