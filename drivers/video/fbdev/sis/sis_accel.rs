//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/sis_accel.h
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
// SiS 300/540/630[S]/730[S],
// SiS 315[E|PRO]/550/[M]650/651/[M]661[F|M]X/740/[M]741[GX]/330/[M]760[GX],
// XGI V3XT/V5/V8, Z7
// frame buffer driver for Linux kernels >= 2.4.14 and >=2.6.3
//
// 2D acceleration part
//
// Based on the X driver's sis300_accel.h which is
// Copyright (C) 2001-2004 by Thomas Winischhofer, Vienna, Austria
// and sis310_accel.h which is
// Copyright (C) 2001-2004 by Thomas Winischhofer, Vienna, Austria
//
// Author:   Thomas Winischhofer <thomas@winischhofer.net>:
// (see http://www.winischhofer.net
// for more information and updates)
//
// Guard accelerator accesses with spin_lock_irqsave? Works well without.

// Macro flag: #define CRITBEGIN
// Macro flag: #define CRITEND
// Macro flag: #define CRITFLAGS

// Definitions for the SIS engine communication.

// SiS300 engine commands
pub const BITBLT: c_uint = 0x00000000  /* Blit */;
pub const COLOREXP: c_uint = 0x00000001  /* Color expand */;
pub const ENCOLOREXP: c_uint = 0x00000002  /* Enhanced color expand */;
pub const MULTIPLE_SCANLINE: c_uint = 0x00000003  /* ? */;
pub const LINE: c_uint = 0x00000004  /* Draw line */;
pub const TRAPAZOID_FILL: c_uint = 0x00000005  /* Fill trapezoid */;
pub const TRANSPARENT_BITBLT: c_uint = 0x00000006  /* Transparent Blit */;
// Additional engine commands for 315
pub const ALPHA_BLEND: c_uint = 0x00000007  /* Alpha blend ? */;
pub const A3D_FUNCTION: c_uint = 0x00000008  /* 3D command ? */;
pub const CLEAR_Z_BUFFER: c_uint = 0x00000009  /* ? */;
pub const GRADIENT_FILL: c_uint = 0x0000000A  /* Gradient fill */;
// source select
pub const SRCVIDEO: c_uint = 0x00000000  /* source is video RAM */;
pub const SRCSYSTEM: c_uint = 0x00000010  /* source is system memory */;

pub const SRCAGP: c_uint = 0x00000020  /* source is AGP memory (?) */;
// Pattern flags
pub const PATFG: c_uint = 0x00000000  /* foreground color */;
pub const PATPATREG: c_uint = 0x00000040  /* pattern in pattern buffer (0x8300) */;
pub const PATMONO: c_uint = 0x00000080  /* mono pattern */;
// blitting direction (300 series only)
pub const X_INC: c_uint = 0x00010000;
pub const X_DEC: c_uint = 0x00000000;
pub const Y_INC: c_uint = 0x00020000;
pub const Y_DEC: c_uint = 0x00000000;
// Clipping flags
pub const NOCLIP: c_uint = 0x00000000;
pub const NOMERGECLIP: c_uint = 0x04000000;
pub const CLIPENABLE: c_uint = 0x00040000;
pub const CLIPWITHOUTMERGE: c_uint = 0x04040000;
// Transparency
pub const OPAQUE: c_uint = 0x00000000;
pub const TRANSPARENT: c_uint = 0x00100000;
// ?
pub const DSTAGP: c_uint = 0x02000000;
pub const DSTVIDEO: c_uint = 0x02000000;
// Subfunctions for Color/Enhanced Color Expansion (315 only)
pub const COLOR_TO_MONO: c_uint = 0x00100000;
pub const AA_TEXT: c_uint = 0x00200000;
// Some general registers for 315 series
pub const SRC_ADDR: c_uint = 0x8200;
pub const SRC_PITCH: c_uint = 0x8204;
pub const AGP_BASE: c_uint = 0x8206 /* color-depth dependent value */;
pub const SRC_Y: c_uint = 0x8208;
pub const SRC_X: c_uint = 0x820A;
pub const DST_Y: c_uint = 0x820C;
pub const DST_X: c_uint = 0x820E;
pub const DST_ADDR: c_uint = 0x8210;
pub const DST_PITCH: c_uint = 0x8214;
pub const DST_HEIGHT: c_uint = 0x8216;
pub const RECT_WIDTH: c_uint = 0x8218;
pub const RECT_HEIGHT: c_uint = 0x821A;
pub const PAT_FGCOLOR: c_uint = 0x821C;
pub const PAT_BGCOLOR: c_uint = 0x8220;
pub const SRC_FGCOLOR: c_uint = 0x8224;
pub const SRC_BGCOLOR: c_uint = 0x8228;
pub const MONO_MASK: c_uint = 0x822C;
pub const LEFT_CLIP: c_uint = 0x8234;
pub const TOP_CLIP: c_uint = 0x8236;
pub const RIGHT_CLIP: c_uint = 0x8238;
pub const BOTTOM_CLIP: c_uint = 0x823A;
pub const COMMAND_READY: c_uint = 0x823C;
pub const FIRE_TRIGGER: c_uint = 0x8240;
pub const PATTERN_REG: c_uint = 0x8300  /* 384 bytes pattern buffer */;
// Transparent bitblit registers

// Store queue length in par

// ------------- SiS 300 series --------------
// BR(16) (0x8240):
//

// (do three times, because 2D engine seems quite unsure about whether or not it's idle)

// 0x8224 src colorkey high
// 0x8228 src colorkey low
// 0x821c dest colorkey high
// 0x8220 dest colorkey low

// General

// -------------- SiS 315/330 series ---------------
// Q_STATUS:
//

extern "C" {
    pub fn sisfb_initaccel(ivideo: *mut sis_video_info) -> c_int;
}
extern "C" {
    pub fn sisfb_syncaccel(ivideo: *mut sis_video_info);
}
extern "C" {
    pub fn fbcon_sis_sync(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fbcon_sis_fillrect(info: *mut fb_info, rect: *const fb_fillrect);
}
extern "C" {
    pub fn fbcon_sis_copyarea(info: *mut fb_info, area: *const fb_copyarea);
}
