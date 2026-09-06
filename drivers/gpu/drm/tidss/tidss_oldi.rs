//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_oldi.h
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
// Copyright (C) 2025 - Texas Instruments Incorporated
//
// Aradhya Bhatia <a-bhatia1@ti.com>
//

// OLDI PORTS
pub const OLDI_INPUT_PORT: c_int = 0;
pub const OLDI_OUTPUT_PORT: c_int = 1;
// Control MMR Registers
// Register offsets
pub const OLDI_PD_CTRL: c_uint = 0x100;
pub const OLDI_LB_CTRL: c_uint = 0x104;
// Power control bits

// LVDS Bandgap reference Enable/Disable

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tidss_oldi_link_type {
    OLDI_MODE_UNSUPPORTED,
    OLDI_MODE_SINGLE_LINK,
    OLDI_MODE_CLONE_SINGLE_LINK,
    OLDI_MODE_SECONDARY_CLONE_SINGLE_LINK,
    OLDI_MODE_DUAL_LINK,
    OLDI_MODE_SECONDARY_DUAL_LINK,
}

extern "C" {
    pub fn tidss_oldi_init(tidss: *mut tidss_device) -> c_int;
}
extern "C" {
    pub fn tidss_oldi_deinit(tidss: *mut tidss_device);
}
