//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/armada/armada_crtc.h
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
// Copyright (C) 2012 Russell King
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_regs {
    pub offset: u32,
    pub mask: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_crtc {
    pub crtc: drm_crtc,
    pub variant: *const armada_variant,
    pub variant_data: *mut c_void,
    pub num: unsigned,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub spu_v_h_total: u32,
    pub spu_v_porch: u32,
    pub spu_adv_reg: u32,
    pub v: [}; 2],
    pub interlaced: bool,
    pub cursor_update: bool,
    pub cursor_obj: *mut armada_gem_object,
    pub cursor_x: c_int,
    pub cursor_y: c_int,
    pub cursor_hw_pos: u32,
    pub cursor_hw_sz: u32,
    pub cursor_w: u32,
    pub cursor_h: u32,
    pub cfg_dumb_ctrl: u32,
    pub spu_iopad_ctrl: u32,
    pub irq_lock: spinlock_t,
    pub irq_ena: u32,
    pub update_pending: bool,
    pub event: *mut drm_pending_vblank_event,
    pub atomic_regs: [armada_regs; 32],
    pub regs: *mut armada_regs,
    pub regs_idx: c_uint,
}

extern "C" {
    pub fn armada_drm_crtc_update_regs(: *mut armada_crtc, : *mut armada_regs);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_clocking_params {
    pub permillage_min: c_ulong,
    pub permillage_max: c_ulong,
    pub settable: u32,
    pub div_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_clk_result {
    pub desired_clk_hz: c_ulong,
    pub clk: *mut clk,
    pub div: u32,
}
