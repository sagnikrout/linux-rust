//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/hub.h
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
// Copyright (C) 2017 NVIDIA CORPORATION.  All rights reserved.
//
pub const TEGRA_HUB_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_windowgroup {
    pub usecount: c_uint,
    pub lock: mutex,
    pub index: c_uint,
    pub parent: *mut host1x_client,
    pub rst: *mut reset_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_shared_plane {
    pub base: tegra_plane,
    pub wgrp: *mut tegra_windowgroup,
}

extern "C" {
    pub fn container_of(_arg: plane, tegra_shared_plane: struct, _arg: base.base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_display_hub_soc {
    pub num_wgrps: c_uint,
    pub supports_dsc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_display_hub {
    pub base: drm_private_obj,
    pub client: host1x_client,
    pub clk_disp: *mut clk,
    pub clk_dsc: *mut clk,
    pub clk_hub: *mut clk,
    pub rst: *mut reset_control,
    pub num_heads: c_uint,
    pub clk_heads: *mut clk,
    pub soc: *const tegra_display_hub_soc,
    pub wgrps: *mut tegra_windowgroup,
}

extern "C" {
    pub fn container_of(_arg: client, tegra_display_hub: struct, _arg: client) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_display_hub_state {
    pub base: drm_private_state,
    pub dc: *mut tegra_dc,
    pub rate: c_ulong,
    pub clk: *mut clk,
}

extern "C" {
    pub fn container_of(_arg: priv, tegra_display_hub_state: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_display_hub_prepare(hub: *mut tegra_display_hub) -> c_int;
}
extern "C" {
    pub fn tegra_display_hub_cleanup(hub: *mut tegra_display_hub);
}
pub const DC_CMD_IHUB_COMMON_MISC_CTL: c_uint = 0x068;

pub const DC_DISP_IHUB_COMMON_DISPLAY_FETCH_METER: c_uint = 0x451;

