//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mxsfb/mxsfb_regs.h
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
// Copyright (C) 2010 Juergen Beisert, Pengutronix
// Copyright (C) 2016 Marek Vasut <marex@denx.de>
//
// i.MX23/i.MX28/i.MX6SX MXSFB LCD controller driver.
//
pub const REG_SET: c_int = 4;
pub const REG_CLR: c_int = 8;
pub const LCDC_CTRL: c_uint = 0x00;
pub const LCDC_CTRL1: c_uint = 0x10;
pub const LCDC_V3_TRANSFER_COUNT: c_uint = 0x20;
pub const LCDC_V4_CTRL2: c_uint = 0x20;
pub const LCDC_V4_TRANSFER_COUNT: c_uint = 0x30;
pub const LCDC_V4_CUR_BUF: c_uint = 0x40;
pub const LCDC_V4_NEXT_BUF: c_uint = 0x50;
pub const LCDC_V3_CUR_BUF: c_uint = 0x30;
pub const LCDC_V3_NEXT_BUF: c_uint = 0x40;
pub const LCDC_VDCTRL0: c_uint = 0x70;
pub const LCDC_VDCTRL1: c_uint = 0x80;
pub const LCDC_VDCTRL2: c_uint = 0x90;
pub const LCDC_VDCTRL3: c_uint = 0xa0;
pub const LCDC_VDCTRL4: c_uint = 0xb0;
pub const LCDC_V4_CRC_STAT: c_uint = 0x1a0;
pub const LCDC_V4_DEBUG0: c_uint = 0x1d0;
pub const LCDC_V3_DEBUG0: c_uint = 0x1f0;
pub const LCDC_AS_CTRL: c_uint = 0x210;
pub const LCDC_AS_BUF: c_uint = 0x220;
pub const LCDC_AS_NEXT_BUF: c_uint = 0x230;
pub const LCDC_AS_CLRKEYLOW: c_uint = 0x240;
pub const LCDC_AS_CLRKEYHIGH: c_uint = 0x250;

pub const CTRL2_SET_OUTSTANDING_REQS_1: c_int = 0;

pub const MXSFB_MIN_XRES: c_int = 120;
pub const MXSFB_MIN_YRES: c_int = 120;
pub const MXSFB_MAX_XRES: c_uint = 0xffff;
pub const MXSFB_MAX_YRES: c_uint = 0xffff;
