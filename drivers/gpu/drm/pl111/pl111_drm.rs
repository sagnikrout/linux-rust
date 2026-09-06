//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/pl111/pl111_drm.h
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
// (C) COPYRIGHT 2012-2013 ARM Limited. All rights reserved.
//
// Parts of this file were based on sources as follows:
//
// Copyright (c) 2006-2008 Intel Corporation
// Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
// Copyright (C) 2011 Texas Instruments
//

//
// CLCD Controller Internal Register addresses
//
pub const CLCD_TIM0: c_uint = 0x00000000;
pub const CLCD_TIM1: c_uint = 0x00000004;
pub const CLCD_TIM2: c_uint = 0x00000008;
pub const CLCD_TIM3: c_uint = 0x0000000c;
pub const CLCD_UBAS: c_uint = 0x00000010;
pub const CLCD_LBAS: c_uint = 0x00000014;
pub const CLCD_PL110_IENB: c_uint = 0x00000018;
pub const CLCD_PL110_CNTL: c_uint = 0x0000001c;
pub const CLCD_PL110_STAT: c_uint = 0x00000020;
pub const CLCD_PL110_INTR: c_uint = 0x00000024;
pub const CLCD_PL110_UCUR: c_uint = 0x00000028;
pub const CLCD_PL110_LCUR: c_uint = 0x0000002C;
pub const CLCD_PL111_CNTL: c_uint = 0x00000018;
pub const CLCD_PL111_IENB: c_uint = 0x0000001c;
pub const CLCD_PL111_RIS: c_uint = 0x00000020;
pub const CLCD_PL111_MIS: c_uint = 0x00000024;
pub const CLCD_PL111_ICR: c_uint = 0x00000028;
pub const CLCD_PL111_UCUR: c_uint = 0x0000002c;
pub const CLCD_PL111_LCUR: c_uint = 0x00000030;
pub const CLCD_PALL: c_uint = 0x00000200;
pub const CLCD_PALETTE: c_uint = 0x00000200;

pub const TIM2_PCD_LO_BITS: c_int = 5;

pub const TIM2_PCD_HI_BITS: c_int = 5;
pub const TIM2_PCD_HI_SHIFT: c_int = 27;

// ST Microelectronics variant bits
pub const CNTL_ST_1XBPP_444: c_uint = 0x0;

pub const CNTL_ST_CDWID_12: c_uint = 0x0;

//
// struct pl111_variant_data - encodes IP differences
// @name: the name of this variant
// @is_pl110: this is the early PL110 variant
// @is_lcdc: this is the ST Microelectronics Nomadik LCDC variant
// @external_bgr: this is the Versatile Pl110 variant with external
// BGR/RGB routing
// @broken_clockdivider: the clock divider is broken and we need to
// use the supplied clock directly
// @broken_vblank: the vblank IRQ is broken on this variant
// @st_bitmux_control: this variant is using the ST Micro bitmux
// extensions to the control register
// @formats: array of supported pixel formats on this variant
// @nformats: the length of the array of supported pixel formats
// @fb_depth: desired depth per pixel on the default framebuffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl111_variant_data {
    pub name: *const c_char,
    pub is_pl110: bool,
    pub is_lcdc: bool,
    pub external_bgr: bool,
    pub broken_clockdivider: bool,
    pub broken_vblank: bool,
    pub st_bitmux_control: bool,
    pub formats: *const u32,
    pub nformats: c_uint,
    pub fb_depth: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl111_drm_dev_private {
    pub drm: *mut drm_device,
    pub connector: *mut drm_connector,
    pub panel: *mut drm_panel,
    pub bridge: *mut drm_bridge,
    pub pipe: drm_simple_display_pipe,
    pub regs: *mut c_void,
    pub memory_bw: u32,
    pub ienb: u32,
    pub ctrl: u32,
// The pixel clock (a reference to our clock divider off of CLCDCLK).
    pub clk: *mut clk,
// pl111's internal clock divider.
    pub clk_div: clk_hw,
// Lock to sync access to CLCD_TIM2 between the common clock
// subsystem and pl111_display_enable().
//
    pub tim2_lock: spinlock_t,
    pub variant: *const pl111_variant_data,
    pub format): *mut *mut *mut void (variant_display_enable) (struct drm_device drm, u32,
    pub drm): *mut *mut void (variant_display_disable) (struct drm_device,
    pub use_device_memory: bool,
}

extern "C" {
    pub fn pl111_display_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn pl111_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn pl111_debugfs_init(minor: *mut drm_minor);
}
