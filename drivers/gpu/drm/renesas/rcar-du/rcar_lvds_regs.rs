//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_lvds_regs.h
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


// SPDX-License-Identifier: GPL-2.0
//
// R-Car LVDS Interface Registers Definitions
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//
pub const LVDCR0: c_uint = 0x0000;

pub const LVDCR0_LVMD_SHIFT: c_int = 8;

pub const LVDCR1: c_uint = 0x0004;

pub const LVDPLLCR: c_uint = 0x0008;
// Gen2 & V3M

// Gen3 but V3M,D3 and E3

// D3 and E3

pub const LVDCTRCR: c_uint = 0x000c;

pub const LVDCHCR: c_uint = 0x0010;

// All registers below are specific to D3 and E3
pub const LVDSTRIPE: c_uint = 0x0014;

pub const LVDSCR: c_uint = 0x0018;

pub const LVDDIV: c_uint = 0x001c;

