//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun4i_frontend.h
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
// Copyright (C) 2017 Free Electrons
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_FRONTEND_EN_REG: c_uint = 0x000;

pub const SUN4I_FRONTEND_FRM_CTRL_REG: c_uint = 0x004;

pub const SUN4I_FRONTEND_BYPASS_REG: c_uint = 0x008;

pub const SUN4I_FRONTEND_BUF_ADDR0_REG: c_uint = 0x020;
pub const SUN4I_FRONTEND_BUF_ADDR1_REG: c_uint = 0x024;
pub const SUN4I_FRONTEND_BUF_ADDR2_REG: c_uint = 0x028;
pub const SUN4I_FRONTEND_TB_OFF0_REG: c_uint = 0x030;
pub const SUN4I_FRONTEND_TB_OFF1_REG: c_uint = 0x034;
pub const SUN4I_FRONTEND_TB_OFF2_REG: c_uint = 0x038;

pub const SUN4I_FRONTEND_LINESTRD0_REG: c_uint = 0x040;
pub const SUN4I_FRONTEND_LINESTRD1_REG: c_uint = 0x044;
pub const SUN4I_FRONTEND_LINESTRD2_REG: c_uint = 0x048;
//
// In tiled mode, the stride is defined as the distance between the start of the
// end line of the current tile and the start of the first line in the next
// vertical tile.
//
// Tiles are represented in row-major order, thus the end line of current tile
// starts at: 31 * 32 (31 lines of 32 cols), the next vertical tile starts at:
// 32-bit-aligned-width * 32 and the distance is:
// 32 * (32-bit-aligned-width - 31).
//

pub const SUN4I_FRONTEND_INPUT_FMT_REG: c_uint = 0x04c;

pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_UYVY: c_int = 0;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_YUYV: c_int = 1;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_VYUY: c_int = 2;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_YVYU: c_int = 3;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_UV: c_int = 0;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_VU: c_int = 1;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_BGRX: c_int = 0;
pub const SUN4I_FRONTEND_INPUT_FMT_DATA_PS_XRGB: c_int = 1;
pub const SUN4I_FRONTEND_OUTPUT_FMT_REG: c_uint = 0x05c;
pub const SUN4I_FRONTEND_OUTPUT_FMT_DATA_FMT_BGRX8888: c_int = 1;
pub const SUN4I_FRONTEND_OUTPUT_FMT_DATA_FMT_XRGB8888: c_int = 2;

pub const SUN4I_FRONTEND_CH0_INSIZE_REG: c_uint = 0x100;

pub const SUN4I_FRONTEND_CH0_OUTSIZE_REG: c_uint = 0x104;

pub const SUN4I_FRONTEND_CH0_HORZFACT_REG: c_uint = 0x108;

pub const SUN4I_FRONTEND_CH0_VERTFACT_REG: c_uint = 0x10c;

pub const SUN4I_FRONTEND_CH0_HORZPHASE_REG: c_uint = 0x110;
pub const SUN4I_FRONTEND_CH0_VERTPHASE0_REG: c_uint = 0x114;
pub const SUN4I_FRONTEND_CH0_VERTPHASE1_REG: c_uint = 0x118;
pub const SUN4I_FRONTEND_CH1_INSIZE_REG: c_uint = 0x200;
pub const SUN4I_FRONTEND_CH1_OUTSIZE_REG: c_uint = 0x204;
pub const SUN4I_FRONTEND_CH1_HORZFACT_REG: c_uint = 0x208;
pub const SUN4I_FRONTEND_CH1_VERTFACT_REG: c_uint = 0x20c;
pub const SUN4I_FRONTEND_CH1_HORZPHASE_REG: c_uint = 0x210;
pub const SUN4I_FRONTEND_CH1_VERTPHASE0_REG: c_uint = 0x214;
pub const SUN4I_FRONTEND_CH1_VERTPHASE1_REG: c_uint = 0x218;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_frontend_data {
    pub has_coef_access_ctrl: bool,
    pub has_coef_rdy: bool,
    pub ch_phase: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_frontend {
    pub list: list_head,
    pub dev: *mut device,
    pub node: *mut device_node,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub ram_clk: *mut clk,
    pub regs: *mut regmap,
    pub reset: *mut reset_control,
    pub data: *const sun4i_frontend_data,
}

extern "C" {
    pub fn sun4i_frontend_init(frontend: *mut sun4i_frontend) -> c_int;
}
extern "C" {
    pub fn sun4i_frontend_exit(frontend: *mut sun4i_frontend);
}
extern "C" {
    pub fn sun4i_frontend_enable(frontend: *mut sun4i_frontend) -> c_int;
}
extern "C" {
    pub fn sun4i_frontend_format_is_supported(fmt: u32, modifier: u64) -> bool;
}
