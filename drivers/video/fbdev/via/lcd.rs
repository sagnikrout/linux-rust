//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/lcd.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//
// Definition TMDS Device ID register
pub const VT1631_DEVICE_ID_REG: c_uint = 0x02;
pub const VT1631_DEVICE_ID: c_uint = 0x92;
pub const VT3271_DEVICE_ID_REG: c_uint = 0x02;
pub const VT3271_DEVICE_ID: c_uint = 0x71;
// Definition DVI Panel ID
// Resolution: 640x480,   Channel: single, Dithering: Enable
pub const LCD_PANEL_ID0_640X480: c_uint = 0x00;
// Resolution: 800x600,   Channel: single, Dithering: Enable
pub const LCD_PANEL_ID1_800X600: c_uint = 0x01;
// Resolution: 1024x768,  Channel: single, Dithering: Enable
pub const LCD_PANEL_ID2_1024X768: c_uint = 0x02;
// Resolution: 1280x768,  Channel: single, Dithering: Enable
pub const LCD_PANEL_ID3_1280X768: c_uint = 0x03;
// Resolution: 1280x1024, Channel: dual,   Dithering: Enable
pub const LCD_PANEL_ID4_1280X1024: c_uint = 0x04;
// Resolution: 1400x1050, Channel: dual,   Dithering: Enable
pub const LCD_PANEL_ID5_1400X1050: c_uint = 0x05;
// Resolution: 1600x1200, Channel: dual,   Dithering: Enable
pub const LCD_PANEL_ID6_1600X1200: c_uint = 0x06;
// Resolution: 1366x768,  Channel: single, Dithering: Disable
pub const LCD_PANEL_ID7_1366X768: c_uint = 0x07;
// Resolution: 1024x600,  Channel: single, Dithering: Enable
pub const LCD_PANEL_ID8_1024X600: c_uint = 0x08;
// Resolution: 1280x800,  Channel: single, Dithering: Enable
pub const LCD_PANEL_ID9_1280X800: c_uint = 0x09;
// Resolution: 800x480,   Channel: single, Dithering: Enable
pub const LCD_PANEL_IDA_800X480: c_uint = 0x0A;
// Resolution: 1360x768,   Channel: single, Dithering: Disable
pub const LCD_PANEL_IDB_1360X768: c_uint = 0x0B;
// Resolution: 480x640,  Channel: single, Dithering: Enable
pub const LCD_PANEL_IDC_480X640: c_uint = 0x0C;
// Resolution: 1200x900,  Channel: single, Dithering: Disable
pub const LCD_PANEL_IDD_1200X900: c_uint = 0x0D;
// plvds_setting_info,
extern "C" {
    pub fn viafb_lcd_disable();
}
extern "C" {
    pub fn viafb_lcd_enable();
}
extern "C" {
    pub fn viafb_init_lcd_size();
}
// plvds_chip_info,
// plvds_setting_info);
extern "C" {
    pub fn viafb_lvds_trasmitter_identify() -> bool;
}
// plvds_chip_info,
// plvds_setting_info);
extern "C" {
    pub fn viafb_lcd_get_mobile_state(mobile: *mut bool) -> bool;
}
