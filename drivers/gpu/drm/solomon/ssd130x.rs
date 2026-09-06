//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/solomon/ssd130x.h
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
// Header file for:
// DRM driver for Solomon SSD130x OLED displays
//
// Copyright 2022 Red Hat Inc.
// Author: Javier Martinez Canillas <javierm@redhat.com>
//
// Based on drivers/video/fbdev/ssd1307fb.c
// Copyright 2012 Free Electrons
//

pub const SSD13XX_DATA: c_uint = 0x40;
pub const SSD13XX_COMMAND: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssd130x_family_ids {
    SSD130X_FAMILY,
    SSD132X_FAMILY,
    SSD133X_FAMILY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssd130x_variants {
// ssd130x family
    SH1106_ID,
    SSD1305_ID,
    SSD1306_ID,
    SSD1307_ID,
    SSD1309_ID,
// ssd132x family
    SSD1322_ID,
    SSD1325_ID,
    SSD1327_ID,
// ssd133x family
    SSD1331_ID,
    NR_SSD130X_VARIANTS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssd130x_deviceinfo {
    pub default_vcomh: u32,
    pub default_dclk_div: u32,
    pub default_dclk_frq: u32,
    pub default_width: u32,
    pub default_height: u32,
    pub need_pwm: bool,
    pub need_chargepump: bool,
    pub page_mode_only: bool,
    pub family_id: ssd130x_family_ids,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssd130x_device {
    pub drm: drm_device,
    pub dev: *mut device,
    pub mode: drm_display_mode,
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub device_info: *const ssd130x_deviceinfo,
    pub 1: unsigned page_address_mode :,
    pub 1: unsigned area_color_enable :,
    pub 1: unsigned com_invdir :,
    pub 1: unsigned com_lrremap :,
    pub 1: unsigned com_seq :,
    pub 1: unsigned lookup_table_set :,
    pub 1: unsigned low_power :,
    pub 1: unsigned seg_remap :,
    pub com_offset: u32,
    pub contrast: u32,
    pub dclk_div: u32,
    pub dclk_frq: u32,
    pub height: u32,
    pub lookup_table: [u8; 4],
    pub page_offset: u32,
    pub col_offset: u32,
    pub prechargep1: u32,
    pub prechargep2: u32,
    pub bl_dev: *mut backlight_device,
    pub pwm: *mut pwm_device,
    pub reset: *mut gpio_desc,
    pub vcc_reg: *mut regulator,
    pub vcomh: u32,
    pub width: u32,
// Cached address ranges
    pub col_start: u8,
    pub col_end: u8,
    pub page_start: u8,
    pub page_end: u8,
}

extern "C" {
    pub fn ssd130x_remove(ssd130x: *mut ssd130x_device);
}
extern "C" {
    pub fn ssd130x_shutdown(ssd130x: *mut ssd130x_device);
}
