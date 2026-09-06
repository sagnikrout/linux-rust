//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/gl860/gl860.h
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
// GSPCA subdrivers for Genesys Logic webcams with the GL860 chip
// Subdriver declarations
//
// 2009/10/14 Olivier LORIN <o.lorin@laposte.net>
//

pub const ID_MI1320: c_int = 1;
pub const ID_OV2640: c_int = 2;
pub const ID_OV9655: c_int = 4;
pub const ID_MI2020: c_int = 8;

pub const IMAGE_640: c_int = 0;
pub const IMAGE_800: c_int = 1;
pub const IMAGE_1280: c_int = 2;
pub const IMAGE_1600: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_gl860 {
    pub backlight: u16,
    pub brightness: u16,
    pub sharpness: u16,
    pub contrast: u16,
    pub gamma: u16,
    pub hue: u16,
    pub saturation: u16,
    pub whitebal: u16,
    pub mirror: u8,
    pub flip: u8,
    pub AC50Hz: u8,
}

// Specific webcam descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub /: *mut *mut gspca_dev gspca_dev; / !! must be the first item,
    pub vcur: sd_gl860,
    pub vold: sd_gl860,
    pub vmax: sd_gl860,
    pub ): *mut *mut int (dev_configure_alt) (struct gspca_dev,
    pub ): *mut *mut int (dev_init_at_startup)(struct gspca_dev,
    pub ): *mut *mut int (dev_init_pre_alt) (struct gspca_dev,
    pub ): *mut *mut void (dev_post_unset_alt) (struct gspca_dev,
    pub ): *mut *mut int (dev_camera_settings)(struct gspca_dev,
    pub swapRB: u8,
    pub mirrorMask: u8,
    pub sensor: u8,
    pub nbIm: i32,
    pub nbRightUp: i32,
    pub waitSet: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct validx {
    pub val: u16,
    pub idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxdata {
    pub idx: u8,
    pub data: [u8; 3],
}

extern "C" {
    pub fn fetch_validx(gspca_dev: *mut gspca_dev, tbl: *mut validx, len: c_int) -> c_int;
}
extern "C" {
    pub fn fetch_idxdata(gspca_dev: *mut gspca_dev, tbl: *mut idxdata, len: c_int);
}
extern "C" {
    pub fn mi1320_init_settings(: *mut gspca_dev);
}
extern "C" {
    pub fn ov2640_init_settings(: *mut gspca_dev);
}
extern "C" {
    pub fn ov9655_init_settings(: *mut gspca_dev);
}
extern "C" {
    pub fn mi2020_init_settings(: *mut gspca_dev);
}
