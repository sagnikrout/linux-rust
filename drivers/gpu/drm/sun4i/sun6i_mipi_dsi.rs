//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sun6i_mipi_dsi.h
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
// Copyright (c) 2016 Allwinnertech Co., Ltd.
// Copyright (C) 2017-2018 Bootlin
//
// Maxime Ripard <maxime.ripard@bootlin.com>
//

pub const SUN6I_DSI_TCON_DIV: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_dsi_variant {
    pub has_mod_clk: bool,
    pub set_mod_clk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_dsi {
    pub connector: drm_connector,
    pub encoder: drm_encoder,
    pub host: mipi_dsi_host,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub regs: *mut regmap,
    pub regulator: *mut regulator,
    pub reset: *mut reset_control,
    pub dphy: *mut phy,
    pub dev: *mut device,
    pub device: *mut mipi_dsi_device,
    pub drm: *mut drm_device,
    pub panel: *mut drm_panel,
    pub variant: *const sun6i_dsi_variant,
}

extern "C" {
    pub fn container_of(_arg: host, sun6i_dsi: struct, _arg: host) -> return;
}
extern "C" {
    pub fn container_of(_arg: connector, sun6i_dsi: struct, _arg: connector) -> return;
}
extern "C" {
    pub fn container_of(_arg: encoder, sun6i_dsi: struct, _arg: encoder) -> return;
}
