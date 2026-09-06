//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dc/dc-de.h
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

pub const DC_DISPLAYS: c_int = 2;
pub const DC_FRAMEGEN_MAX_FRAME_INDEX: c_uint = 0x3ffff;
pub const DC_FRAMEGEN_MAX_CLOCK_KHZ: c_int = 300000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_fg {
    pub dev: *mut device,
    pub reg: *mut regmap,
    pub clk_disp: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_tc {
    pub dev: *mut device,
    pub reg: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_de {
    pub dev: *mut device,
    pub reg_top: *mut regmap,
    pub fg: *mut dc_fg,
    pub tc: *mut dc_tc,
    pub irq_shdload: c_int,
    pub irq_framecomplete: c_int,
    pub irq_seqcomplete: c_int,
}

// Frame Generator Unit
extern "C" {
    pub fn dc_fg_cfg_videomode(fg: *mut dc_fg, m: *mut drm_display_mode);
}
extern "C" {
    pub fn dc_fg_enable(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_disable(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_shdtokgen(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_get_frame_index(fg: *mut dc_fg) -> u32;
}
extern "C" {
    pub fn dc_fg_get_line_index(fg: *mut dc_fg) -> u32;
}
extern "C" {
    pub fn dc_fg_wait_for_frame_index_moving(fg: *mut dc_fg) -> bool;
}
extern "C" {
    pub fn dc_fg_secondary_requests_to_read_empty_fifo(fg: *mut dc_fg) -> bool;
}
extern "C" {
    pub fn dc_fg_secondary_clear_channel_status(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_wait_for_secondary_syncup(fg: *mut dc_fg) -> c_int;
}
extern "C" {
    pub fn dc_fg_enable_clock(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_disable_clock(fg: *mut dc_fg);
}
extern "C" {
    pub fn dc_fg_check_clock(fg: *mut dc_fg, clk_khz: c_int) -> drm_mode_status;
}
extern "C" {
    pub fn dc_fg_init(fg: *mut dc_fg);
}
// Timing Controller Unit
extern "C" {
    pub fn dc_tc_init(tc: *mut dc_tc);
}
