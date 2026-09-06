//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/sh_mobile_lcdc.h
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

// Register definitions
pub const _LDDCKR: c_uint = 0x410;

pub const _LDDCKSTPR: c_uint = 0x414;
pub const _LDINTR: c_uint = 0x468;

pub const _LDSR: c_uint = 0x46c;

pub const _LDCNT1R: c_uint = 0x470;

pub const _LDCNT2R: c_uint = 0x474;

pub const _LDRCNTR: c_uint = 0x478;

pub const _LDDDSR: c_uint = 0x47c;

pub const _LDDWD0R: c_uint = 0x800;

pub const _LDDRDR: c_uint = 0x840;

pub const _LDDWAR: c_uint = 0x900;

pub const _LDDRAR: c_uint = 0x904;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_sys_bus_cfg {
    pub ldmt2r: c_ulong,
    pub ldmt3r: c_ulong,
    pub deferred_io_msec: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_sys_bus_ops {
    pub data): *mut *mut *mut void (write_index)(void handle, unsigned long,
    pub data): *mut *mut *mut void (write_data)(void handle, unsigned long,
    pub handle): *mut *mut unsigned long (read_data)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_panel_cfg {
    pub /: *mut *mut unsigned long width; / Panel width in mm,
    pub /: *mut *mut unsigned long height; / Panel height in mm,
    pub sys_ops): *mut sh_mobile_lcdc_sys_bus_ops,
    pub sys_ops): *mut sh_mobile_lcdc_sys_bus_ops,
    pub (*display_on)(void): *mut c_void,
    pub (*display_off)(void): *mut c_void,
}

// backlight info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_bl_info {
    pub name: *const c_char,
    pub max_brightness: c_int,
    pub brightness): *mut *mut int (set_brightness)(int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_overlay_cfg {
    pub fourcc: c_int,
    pub max_xres: c_uint,
    pub max_yres: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_chan_cfg {
    pub chan: c_int,
    pub fourcc: c_int,
    pub colorspace: c_int,
    pub /: *mut *mut int interface_type; / selects RGBn or SYSn I/F, see above,
    pub clock_divider: c_int,
    pub /: *mut *mut unsigned long flags; / LCDC_FLAGS_...,
    pub lcd_modes: *const fb_videomode,
    pub num_modes: c_int,
    pub panel_cfg: sh_mobile_lcdc_panel_cfg,
    pub bl_info: sh_mobile_lcdc_bl_info,
    pub /: *mut *mut sh_mobile_lcdc_sys_bus_cfg sys_bus_cfg; / only for SYSn I/F,
    pub /: *mut *mut *mut platform_device tx_dev; / HDMI/DSI transmitter device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mobile_lcdc_info {
    pub clock_source: c_int,
    pub ch: [sh_mobile_lcdc_chan_cfg; 2],
    pub overlays: [sh_mobile_lcdc_overlay_cfg; 4],
}
