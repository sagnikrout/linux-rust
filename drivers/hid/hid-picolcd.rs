//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-picolcd.h
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
// Copyright (C) 2010-2012 by Bruno Prémont <bonbons@linux-vserver.org>
//
// Based on Logitech G13 driver (v0.4)
// Copyright (C) 2009 by Rick L. Vinyard, Jr. <rvinyard@cs.nmsu.edu>
//

// Report numbers
pub const REPORT_ERROR_CODE: c_uint = 0x10 /* LCD: IN[16]  */;
pub const ERR_SUCCESS: c_uint = 0x00;
pub const ERR_PARAMETER_MISSING: c_uint = 0x01;
pub const ERR_DATA_MISSING: c_uint = 0x02;
pub const ERR_BLOCK_READ_ONLY: c_uint = 0x03;
pub const ERR_BLOCK_NOT_ERASABLE: c_uint = 0x04;
pub const ERR_BLOCK_TOO_BIG: c_uint = 0x05;
pub const ERR_SECTION_OVERFLOW: c_uint = 0x06;
pub const ERR_INVALID_CMD_LEN: c_uint = 0x07;
pub const ERR_INVALID_DATA_LEN: c_uint = 0x08;
pub const REPORT_KEY_STATE: c_uint = 0x11 /* LCD: IN[2]   */;
pub const REPORT_IR_DATA: c_uint = 0x21 /* LCD: IN[63]  */;
pub const REPORT_EE_DATA: c_uint = 0x32 /* LCD: IN[63]  */;
pub const REPORT_MEMORY: c_uint = 0x41 /* LCD: IN[63]  */;
pub const REPORT_LED_STATE: c_uint = 0x81 /* LCD: OUT[1]  */;
pub const REPORT_BRIGHTNESS: c_uint = 0x91 /* LCD: OUT[1]  */;
pub const REPORT_CONTRAST: c_uint = 0x92 /* LCD: OUT[1]  */;
pub const REPORT_RESET: c_uint = 0x93 /* LCD: OUT[2]  */;
pub const REPORT_LCD_CMD: c_uint = 0x94 /* LCD: OUT[63] */;
pub const REPORT_LCD_DATA: c_uint = 0x95 /* LCD: OUT[63] */;
pub const REPORT_LCD_CMD_DATA: c_uint = 0x96 /* LCD: OUT[63] */;
pub const REPORT_EE_READ: c_uint = 0xa3 /* LCD: OUT[63] */;
pub const REPORT_EE_WRITE: c_uint = 0xa4 /* LCD: OUT[63] */;
pub const REPORT_ERASE_MEMORY: c_uint = 0xb2 /* LCD: OUT[2]  */;
pub const REPORT_READ_MEMORY: c_uint = 0xb3 /* LCD: OUT[3]  */;
pub const REPORT_WRITE_MEMORY: c_uint = 0xb4 /* LCD: OUT[63] */;
pub const REPORT_SPLASH_RESTART: c_uint = 0xc1 /* LCD: OUT[1]  */;
pub const REPORT_EXIT_KEYBOARD: c_uint = 0xef /* LCD: OUT[2]  */;
pub const REPORT_VERSION: c_uint = 0xf1 /* LCD: IN[2],OUT[1]    Bootloader: IN[2],OUT[1]   */;
pub const REPORT_BL_ERASE_MEMORY: c_uint = 0xf2 /*                      Bootloader: IN[36],OUT[4]  */;
pub const REPORT_BL_READ_MEMORY: c_uint = 0xf3 /*                      Bootloader: IN[36],OUT[4]  */;
pub const REPORT_BL_WRITE_MEMORY: c_uint = 0xf4 /*                      Bootloader: IN[36],OUT[36] */;
pub const REPORT_DEVID: c_uint = 0xf5 /* LCD: IN[5], OUT[1]   Bootloader: IN[5],OUT[1]   */;
pub const REPORT_SPLASH_SIZE: c_uint = 0xf6 /* LCD: IN[4], OUT[1]   */;
pub const REPORT_HOOK_VERSION: c_uint = 0xf7 /* LCD: IN[2], OUT[1]   */;
pub const REPORT_EXIT_FLASHER: c_uint = 0xff /*                      Bootloader: OUT[2]         */;
// Description of in-progress IO operation, used for operations
// that trigger response from device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct picolcd_pending {
    pub out_report: *mut hid_report,
    pub in_report: *mut hid_report,
    pub ready: completion,
    pub raw_size: c_int,
    pub raw_data: [u8; 64],
}

pub const PICOLCD_KEYS: c_int = 17;
// Per device data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct picolcd_data {
    pub hdev: *mut hid_device,

    pub debug_reset: *mut dentry,
    pub debug_eeprom: *mut dentry,
    pub debug_flash: *mut dentry,
    pub mutex_flash: mutex,
    pub addr_sz: c_int,
    pub version: [u8; 2],
    pub opmode_delay: c_ushort,
// input stuff
    pub pressed_keys: [u8; 2],
    pub input_keys: *mut input_dev,

    pub rc_dev: *mut rc_dev,
    pub keycode: [c_ushort; PICOLCD_KEYS],
// Framebuffer stuff
    pub fb_info: *mut fb_info,

    pub lcd: *mut lcd_device,
    pub lcd_contrast: u8,

    pub backlight: *mut backlight_device,
    pub lcd_brightness: u8,
    pub lcd_power: u8,

// LED stuff
    pub led_state: u8,
    pub led: [*mut led_classdev; 8],
// Housekeeping stuff
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub pending: *mut picolcd_pending,
    pub status: c_int,
pub const PICOLCD_BOOTLOADER: c_int = 1;
pub const PICOLCD_FAILED: c_int = 2;
pub const PICOLCD_CIR_SHUN: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct picolcd_fb_data {
// Framebuffer stuff
    pub lock: spinlock_t,
    pub picolcd: *mut picolcd_data,
    pub update_rate: u8,
    pub bpp: u8,
    pub force: u8,
    pub ready: u8,
    pub /: *mut *mut *mut u8 vbitmap; / local copy of what was sent to PicoLCD,
    pub /: *mut *mut *mut u8 bitmap; / framebuffer,
}

// Find a given report

extern "C" {
    pub fn picolcd_exit_devfs(data: *mut picolcd_data);
}

extern "C" {
    pub fn picolcd_fb_reset(data: *mut picolcd_data, clear: c_int) -> c_int;
}
extern "C" {
    pub fn picolcd_init_framebuffer(data: *mut picolcd_data) -> c_int;
}
extern "C" {
    pub fn picolcd_exit_framebuffer(data: *mut picolcd_data);
}
extern "C" {
    pub fn picolcd_fb_refresh(data: *mut picolcd_data);
}

extern "C" {
    pub fn picolcd_exit_backlight(data: *mut picolcd_data);
}
extern "C" {
    pub fn picolcd_resume_backlight(data: *mut picolcd_data) -> c_int;
}
extern "C" {
    pub fn picolcd_suspend_backlight(data: *mut picolcd_data);
}

extern "C" {
    pub fn picolcd_exit_lcd(data: *mut picolcd_data);
}
extern "C" {
    pub fn picolcd_resume_lcd(data: *mut picolcd_data) -> c_int;
}

extern "C" {
    pub fn picolcd_exit_leds(data: *mut picolcd_data);
}
extern "C" {
    pub fn picolcd_leds_set(data: *mut picolcd_data);
}

extern "C" {
    pub fn picolcd_init_cir(data: *mut picolcd_data, report: *mut hid_report) -> c_int;
}
extern "C" {
    pub fn picolcd_exit_cir(data: *mut picolcd_data);
}

extern "C" {
    pub fn picolcd_reset(hdev: *mut hid_device) -> c_int;
}
