//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun4i_backend.h
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
// Copyright (C) 2015 Free Electrons
// Copyright (C) 2015 NextThing Co
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN4I_BACKEND_MODCTL_REG: c_uint = 0x800;

pub const SUN4I_BACKEND_BACKCOLOR_REG: c_uint = 0x804;

pub const SUN4I_BACKEND_DISSIZE_REG: c_uint = 0x808;

pub const SUN4I_BACKEND_LAYFB_H4ADD_REG: c_uint = 0x860;

pub const SUN4I_BACKEND_REGBUFFCTL_REG: c_uint = 0x870;

pub const SUN4I_BACKEND_CKMAX_REG: c_uint = 0x880;
pub const SUN4I_BACKEND_CKMIN_REG: c_uint = 0x884;
pub const SUN4I_BACKEND_CKCFG_REG: c_uint = 0x888;

pub const SUN4I_BACKEND_DLCDPCTL_REG: c_uint = 0x8b0;
pub const SUN4I_BACKEND_DLCDPFRMBUF_ADDRCTL_REG: c_uint = 0x8b4;
pub const SUN4I_BACKEND_DLCDPCOOR_REG0: c_uint = 0x8b8;
pub const SUN4I_BACKEND_DLCDPCOOR_REG1: c_uint = 0x8bc;
pub const SUN4I_BACKEND_INT_EN_REG: c_uint = 0x8c0;
pub const SUN4I_BACKEND_INT_FLAG_REG: c_uint = 0x8c4;

pub const SUN4I_BACKEND_HWCCTL_REG: c_uint = 0x8d8;
pub const SUN4I_BACKEND_HWCFBCTL_REG: c_uint = 0x8e0;
pub const SUN4I_BACKEND_WBCTL_REG: c_uint = 0x8f0;
pub const SUN4I_BACKEND_WBADD_REG: c_uint = 0x8f4;
pub const SUN4I_BACKEND_WBLINEWIDTH_REG: c_uint = 0x8f8;
pub const SUN4I_BACKEND_SPREN_REG: c_uint = 0x900;
pub const SUN4I_BACKEND_SPRFMTCTL_REG: c_uint = 0x908;
pub const SUN4I_BACKEND_SPRALPHACTL_REG: c_uint = 0x90c;
pub const SUN4I_BACKEND_IYUVCTL_REG: c_uint = 0x920;

pub const SUN4I_BACKEND_YGCONS_REG: c_uint = 0x95c;

pub const SUN4I_BACKEND_URCONS_REG: c_uint = 0x96c;

pub const SUN4I_BACKEND_VBCONS_REG: c_uint = 0x97c;
pub const SUN4I_BACKEND_KSCTL_REG: c_uint = 0x980;
pub const SUN4I_BACKEND_KSBKCOLOR_REG: c_uint = 0x984;
pub const SUN4I_BACKEND_KSFSTLINEWIDTH_REG: c_uint = 0x988;
pub const SUN4I_BACKEND_KSVSCAFCT_REG: c_uint = 0x98c;

pub const SUN4I_BACKEND_OCCTL_REG: c_uint = 0x9c0;

pub const SUN4I_BACKEND_OCRCONS_REG: c_uint = 0x9dc;

pub const SUN4I_BACKEND_OCGCONS_REG: c_uint = 0x9ec;

pub const SUN4I_BACKEND_OCBCONS_REG: c_uint = 0x9fc;

pub const SUN4I_BACKEND_SPRPALTAB_OFF: c_uint = 0x4000;
pub const SUN4I_BACKEND_GAMMATAB_OFF: c_uint = 0x4400;
pub const SUN4I_BACKEND_HWCPATTERN_OFF: c_uint = 0x4800;
pub const SUN4I_BACKEND_HWCCOLORTAB_OFF: c_uint = 0x4c00;

pub const SUN4I_BACKEND_NUM_LAYERS: c_int = 4;
pub const SUN4I_BACKEND_NUM_FRONTEND_LAYERS: c_int = 1;
pub const SUN4I_BACKEND_NUM_YUV_PLANES: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_backend {
    pub engine: sunxi_engine,
    pub frontend: *mut sun4i_frontend,
    pub reset: *mut reset_control,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub ram_clk: *mut clk,
    pub sat_clk: *mut clk,
    pub sat_reset: *mut reset_control,
// Protects against races in the frontend teardown
    pub frontend_lock: spinlock_t,
    pub frontend_teardown: bool,
    pub quirks: *const sun4i_backend_quirks,
}

extern "C" {
    pub fn container_of(_arg: engine, sun4i_backend: struct, _arg: engine) -> return;
}
extern "C" {
    pub fn sun4i_backend_format_is_supported(fmt: u32, modifier: u64) -> bool;
}
