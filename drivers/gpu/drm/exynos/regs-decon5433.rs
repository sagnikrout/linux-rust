//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-decon5433.h
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
// Copyright (C) 2014 Samsung Electronics Co.Ltd
//
// Exynos543X DECON
pub const DECON_VIDCON0: c_uint = 0x0000;
pub const DECON_VIDOUTCON0: c_uint = 0x0010;

pub const DECON_SHADOWCON: c_uint = 0x00A0;

pub const DECON_VIDINTCON0: c_uint = 0x0220;
pub const DECON_VIDINTCON1: c_uint = 0x0224;

pub const DECON_QOSLUT07_00: c_uint = 0x02C0;
pub const DECON_QOSLUT15_08: c_uint = 0x02C4;
pub const DECON_QOSCTRL: c_uint = 0x02C8;

pub const DECON_BLENDCON: c_uint = 0x0310;

pub const DECON_FRAMEFIFO_REG7: c_uint = 0x051C;
pub const DECON_FRAMEFIFO_REG8: c_uint = 0x0520;
pub const DECON_FRAMEFIFO_STATUS: c_uint = 0x0524;
pub const DECON_CMU: c_uint = 0x1404;
pub const DECON_UPDATE: c_uint = 0x1410;
pub const DECON_CRFMID: c_uint = 0x1414;
pub const DECON_UPDATE_SCHEME: c_uint = 0x1438;
pub const DECON_VIDCON1: c_uint = 0x2000;
pub const DECON_VIDCON2: c_uint = 0x2004;
pub const DECON_VIDCON3: c_uint = 0x2008;
pub const DECON_VIDCON4: c_uint = 0x200C;
pub const DECON_VIDTCON2: c_uint = 0x2028;
pub const DECON_FRAME_SIZE: c_uint = 0x2038;
pub const DECON_LINECNT_OP_THRESHOLD: c_uint = 0x203C;
pub const DECON_TRIGCON: c_uint = 0x2040;
pub const DECON_TRIGSKIP: c_uint = 0x2050;
pub const DECON_CRCRDATA: c_uint = 0x20B0;
pub const DECON_CRCCTRL: c_uint = 0x20B4;
// Exynos5430 DECON
pub const DECON_VIDTCON0: c_uint = 0x2020;
pub const DECON_VIDTCON1: c_uint = 0x2024;
// Exynos5433 DECON
pub const DECON_VIDTCON00: c_uint = 0x2010;
pub const DECON_VIDTCON01: c_uint = 0x2014;
pub const DECON_VIDTCON10: c_uint = 0x2018;
pub const DECON_VIDTCON11: c_uint = 0x201C;
// Exynos543X DECON Internal
pub const DECON_W013DSTREOCON: c_uint = 0x0320;
pub const DECON_W233DSTREOCON: c_uint = 0x0324;
pub const DECON_FRAMEFIFO_REG0: c_uint = 0x0500;
pub const DECON_ENHANCER_CTRL: c_uint = 0x2100;
// Exynos543X DECON TV
pub const DECON_VCLKCON0: c_uint = 0x0014;
pub const DECON_VIDINTCON2: c_uint = 0x0228;
pub const DECON_VIDINTCON3: c_uint = 0x022C;
// VIDCON0

// VIDOUTCON0

// WINCONx

// SHADOWCON

// VIDOSDxC

// VIDOSDxD

// VIDINTCON0

// VIDINTCON1

// DECON_CMU

// DECON_UPDATE

// DECON_VIDCON1

// DECON_VIDTCON00

// DECON_VIDTCON01

// DECON_VIDTCON10

// DECON_VIDTCON11

// DECON_VIDTCON2

// TRIGCON

// DECON_CRCCTRL

// BLENDCON

// BLENDERQx
pub const BLENDERQ_ZERO: c_uint = 0x0;
pub const BLENDERQ_ONE: c_uint = 0x1;
pub const BLENDERQ_ALPHA_A: c_uint = 0x2;
pub const BLENDERQ_ONE_MINUS_ALPHA_A: c_uint = 0x3;
pub const BLENDERQ_ALPHA0: c_uint = 0x6;

// BLENDCON

