//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/acornfb.h
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
// linux/drivers/video/acornfb.h
//
// Copyright (C) 1998,1999 Russell King
//
// Frame buffer code for Acorn platforms
//

pub const VIDC_PALETTE_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidc20_palette {
    pub red:8: u_int,
    pub green:8: u_int,
    pub blue:8: u_int,
    pub ext:4: u_int,
    pub unused:4: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidc_palette {
    pub red:4: u_int,
    pub green:4: u_int,
    pub blue:4: u_int,
    pub trans:1: u_int,
    pub sbz1:13: u_int,
    pub reg:4: u_int,
    pub sbz2:2: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union palette {
    pub vidc20: vidc20_palette,
    pub vidc: vidc_palette,
    pub p: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acornfb_par {
    pub dev: *mut device,
    pub screen_end: c_ulong,
    pub dram_size: c_uint,
    pub vram_half_sam: c_uint,
    pub palette_size: c_uint,
    pub montype: signed int,
    pub 1: unsigned int using_vram :,
    pub 1: unsigned int dpms :,
    pub palette: [palette; VIDC_PALETTE_SIZE],
    pub pseudo_palette: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidc_timing {
    pub h_cycle: u_int,
    pub h_sync_width: u_int,
    pub h_border_start: u_int,
    pub h_display_start: u_int,
    pub h_display_end: u_int,
    pub h_border_end: u_int,
    pub h_interlace: u_int,
    pub v_cycle: u_int,
    pub v_sync_width: u_int,
    pub v_border_start: u_int,
    pub v_display_start: u_int,
    pub v_display_end: u_int,
    pub v_border_end: u_int,
    pub control: u_int,
// VIDC20 only
    pub pll_ctl: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct modey_params {
    pub y_res: u_int,
    pub u_margin: u_int,
    pub b_margin: u_int,
    pub vsync_len: u_int,
    pub vf: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct modex_params {
    pub x_res: u_int,
    pub l_margin: u_int,
    pub r_margin: u_int,
    pub hsync_len: u_int,
    pub clock: u_int,
    pub hf: u_int,
    pub modey: *const modey_params,
}

//
// VIDC20 registers
//
pub const VIDC20_CTRL: c_uint = 0xe0000000;

pub const VIDC20_ECTL: c_uint = 0xc0000000;

pub const VIDC20_DCTL: c_uint = 0xf0000000;
// 0-9 = number of words in scanline

