//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/verisilicon/vs_bridge.h
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
// Copyright (C) 2025 Icenowy Zheng <uwu@icenowy.me>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vs_bridge_output_interface {
    VSDC_OUTPUT_INTERFACE_DPI = 0,
    VSDC_OUTPUT_INTERFACE_DP = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_bridge {
    pub base: drm_bridge,
    pub enc: *mut drm_encoder,
    pub conn: *mut drm_connector,
    pub crtc: *mut vs_crtc,
    pub next_bridge: *mut drm_bridge,
    pub intf: vs_bridge_output_interface,
}

extern "C" {
    pub fn container_of(_arg: bridge, vs_bridge: struct, _arg: base) -> return;
}
