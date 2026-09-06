//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun8i_tcon_top.h
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
// Copyright (c) 2018 Jernej Skrabec <jernej.skrabec@siol.net>

pub const TCON_TOP_TCON_TV_SETUP_REG: c_uint = 0x00;
pub const TCON_TOP_PORT_SEL_REG: c_uint = 0x1C;

pub const TCON_TOP_PORT_TCON_NUM: c_int = 4;
pub const TCON_TOP_MIXER0_OUT_PORT: c_int = 1;
pub const TCON_TOP_MIXER1_OUT_PORT: c_int = 3;
pub const TCON_TOP_GATE_SRC_REG: c_uint = 0x20;

pub const TCON_TOP_TCON_TV1_GATE: c_int = 24;
pub const TCON_TOP_TCON_TV0_GATE: c_int = 20;
pub const TCON_TOP_TCON_DSI_GATE: c_int = 16;
pub const CLK_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_tcon_top {
    pub bus: *mut clk,
    pub clk_data: *mut clk_hw_onecell_data,
    pub regs: *mut void __iomem,
    pub rst: *mut reset_control,
    pub tcon_map: c_uint,
//
// spinlock is used to synchronize access to same
// register where multiple clock gates can be set.
//
    pub reg_lock: spinlock_t,
}

extern "C" {
    pub fn sun8i_tcon_top_set_hdmi_src(dev: *mut device, tcon: c_int) -> c_int;
}
extern "C" {
    pub fn sun8i_tcon_top_de_config(dev: *mut device, mixer: c_int, tcon: c_int) -> c_int;
}
