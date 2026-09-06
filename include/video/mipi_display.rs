//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/mipi_display.h
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
// Defines for Mobile Industry Processor Interface (MIPI(R))
// Display Working Group standards: DSI, DCS, DBI, DPI
//
// Copyright (C) 2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2006 Nokia Corporation
// Author: Imre Deak <imre.deak@nokia.com>
//
// MIPI DSI Processor-to-Peripheral transaction types
// MIPI DSI Peripheral-to-Processor transaction types
// MIPI DCS commands
// MIPI DCS pixel formats
pub const MIPI_DCS_PIXEL_FMT_24BIT: c_int = 7;
pub const MIPI_DCS_PIXEL_FMT_18BIT: c_int = 6;
pub const MIPI_DCS_PIXEL_FMT_16BIT: c_int = 5;
pub const MIPI_DCS_PIXEL_FMT_12BIT: c_int = 3;
pub const MIPI_DCS_PIXEL_FMT_8BIT: c_int = 2;
pub const MIPI_DCS_PIXEL_FMT_3BIT: c_int = 1;
