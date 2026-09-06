//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mxsfb/lcdif_regs.h
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
// Copyright (C) 2022 Marek Vasut <marex@denx.de>
//
// i.MX8MP/i.MXRT LCDIF LCD controller driver.
//
pub const REG_SET: c_int = 4;
pub const REG_CLR: c_int = 8;
// V8 register set
pub const LCDC_V8_CTRL: c_uint = 0x00;
pub const LCDC_V8_DISP_PARA: c_uint = 0x10;
pub const LCDC_V8_DISP_SIZE: c_uint = 0x14;
pub const LCDC_V8_HSYN_PARA: c_uint = 0x18;
pub const LCDC_V8_VSYN_PARA: c_uint = 0x1c;
pub const LCDC_V8_VSYN_HSYN_WIDTH: c_uint = 0x20;
pub const LCDC_V8_INT_STATUS_D0: c_uint = 0x24;
pub const LCDC_V8_INT_ENABLE_D0: c_uint = 0x28;
pub const LCDC_V8_INT_STATUS_D1: c_uint = 0x30;
pub const LCDC_V8_INT_ENABLE_D1: c_uint = 0x34;
pub const LCDC_V8_CTRLDESCL0_1: c_uint = 0x200;
pub const LCDC_V8_CTRLDESCL0_3: c_uint = 0x208;
pub const LCDC_V8_CTRLDESCL_LOW0_4: c_uint = 0x20c;
pub const LCDC_V8_CTRLDESCL_HIGH0_4: c_uint = 0x210;
pub const LCDC_V8_CTRLDESCL0_5: c_uint = 0x214;
pub const LCDC_V8_CSC0_CTRL: c_uint = 0x21c;
pub const LCDC_V8_CSC0_COEF0: c_uint = 0x220;
pub const LCDC_V8_CSC0_COEF1: c_uint = 0x224;
pub const LCDC_V8_CSC0_COEF2: c_uint = 0x228;
pub const LCDC_V8_CSC0_COEF3: c_uint = 0x22c;
pub const LCDC_V8_CSC0_COEF4: c_uint = 0x230;
pub const LCDC_V8_CSC0_COEF5: c_uint = 0x234;
pub const LCDC_V8_PANIC0_THRES: c_uint = 0x238;

pub const CTRL2_SET_OUTSTANDING_REQS_1: c_int = 0;

// V8 register set

pub const CTRL_FETCH_START_OPTION_FPV: c_int = 0;

pub const PANIC0_THRES_MAX: c_int = 511;
pub const LCDIF_MIN_XRES: c_int = 120;
pub const LCDIF_MIN_YRES: c_int = 120;
pub const LCDIF_MAX_XRES: c_uint = 0xffff;
pub const LCDIF_MAX_YRES: c_uint = 0xffff;
