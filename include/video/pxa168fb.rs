//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/pxa168fb.h
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
// Copyright (C) 2009 Marvell International Ltd.
//

// Dumb interface
pub const PIN_MODE_DUMB_24: c_int = 0;
pub const PIN_MODE_DUMB_18_SPI: c_int = 1;
pub const PIN_MODE_DUMB_18_GPIO: c_int = 2;
pub const PIN_MODE_DUMB_16_SPI: c_int = 3;
pub const PIN_MODE_DUMB_16_GPIO: c_int = 4;
pub const PIN_MODE_DUMB_12_SPI_GPIO: c_int = 5;
pub const PIN_MODE_SMART_18_SPI: c_int = 6;
pub const PIN_MODE_SMART_16_SPI: c_int = 7;
pub const PIN_MODE_SMART_8_SPI_GPIO: c_int = 8;
// Dumb interface pin allocation
pub const DUMB_MODE_RGB565: c_int = 0;
pub const DUMB_MODE_RGB565_UPPER: c_int = 1;
pub const DUMB_MODE_RGB666: c_int = 2;
pub const DUMB_MODE_RGB666_UPPER: c_int = 3;
pub const DUMB_MODE_RGB444: c_int = 4;
pub const DUMB_MODE_RGB444_UPPER: c_int = 5;
pub const DUMB_MODE_RGB888: c_int = 6;
// default fb buffer size WVGA-32bits

//
// Buffer pixel format
// bit0 is for rb swap.
// bit12 is for Y UorV swap
//
pub const PIX_FMT_RGB565: c_int = 0;
pub const PIX_FMT_BGR565: c_int = 1;
pub const PIX_FMT_RGB1555: c_int = 2;
pub const PIX_FMT_BGR1555: c_int = 3;
pub const PIX_FMT_RGB888PACK: c_int = 4;
pub const PIX_FMT_BGR888PACK: c_int = 5;
pub const PIX_FMT_RGB888UNPACK: c_int = 6;
pub const PIX_FMT_BGR888UNPACK: c_int = 7;
pub const PIX_FMT_RGBA888: c_int = 8;
pub const PIX_FMT_BGRA888: c_int = 9;
pub const PIX_FMT_YUV422PACK: c_int = 10;
pub const PIX_FMT_YVU422PACK: c_int = 11;
pub const PIX_FMT_YUV422PLANAR: c_int = 12;
pub const PIX_FMT_YVU422PLANAR: c_int = 13;
pub const PIX_FMT_YUV420PLANAR: c_int = 14;
pub const PIX_FMT_YVU420PLANAR: c_int = 15;
pub const PIX_FMT_PSEUDOCOLOR: c_int = 20;

//
// PXA LCD controller private state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa168fb_info {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub info: *mut fb_info,
    pub reg_base: *mut void __iomem,
    pub fb_start_dma: dma_addr_t,
    pub pseudo_palette: [u32; 16],
    pub pix_fmt: c_int,
    pub is_blanked:1: unsigned,
    pub panel_rbswap:1: unsigned,
    pub active:1: unsigned,
}

//
// PXA fb machine information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa168fb_mach_info {
    pub id: [c_char; 16],
    pub num_modes: c_int,
    pub modes: *mut fb_videomode,
//
// Pix_fmt
//
    pub pix_fmt: unsigned,
//
// I/O pin allocation.
//
    pub io_pin_allocation_mode:4: unsigned,
//
// Dumb panel -- assignment of R/G/B component info to the 24
// available external data lanes.
//
    pub dumb_mode:4: unsigned,
    pub panel_rgb_reverse_lanes:1: unsigned,
//
// Dumb panel -- GPIO output data.
//
    pub gpio_output_mask:8: unsigned,
    pub gpio_output_data:8: unsigned,
//
// Dumb panel -- configurable output signal polarity.
//
    pub invert_composite_blank:1: unsigned,
    pub invert_pix_val_ena:1: unsigned,
    pub invert_pixclock:1: unsigned,
    pub panel_rbswap:1: unsigned,
    pub active:1: unsigned,
    pub enable_lcd:1: unsigned,
}
