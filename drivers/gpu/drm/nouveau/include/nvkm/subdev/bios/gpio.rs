//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/gpio.h
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


// SPDX-License-Identifier: MIT
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_gpio_func_name {
    DCB_GPIO_PANEL_POWER = 0x01,
    DCB_GPIO_FAN = 0x09,
    DCB_GPIO_TVDAC0 = 0x0c,
    DCB_GPIO_THERM_EXT_POWER_EVENT = 0x10,
    DCB_GPIO_TVDAC1 = 0x2d,
    DCB_GPIO_FAN_SENSE = 0x3d,
    DCB_GPIO_POWER_ALERT = 0x4c,
    DCB_GPIO_EXT_POWER_LOW = 0x79,
    DCB_GPIO_LOGO_LED_PWM = 0x84,
    DCB_GPIO_UNUSED = 0xff,
    DCB_GPIO_VID0 = 0x04,
    DCB_GPIO_VID1 = 0x05,
    DCB_GPIO_VID2 = 0x06,
    DCB_GPIO_VID3 = 0x1a,
    DCB_GPIO_VID4 = 0x73,
    DCB_GPIO_VID5 = 0x74,
    DCB_GPIO_VID6 = 0x75,
    DCB_GPIO_VID7 = 0x76,
    DCB_GPIO_VID_PWM = 0x81,
}

pub const DCB_GPIO_LOG_DIR: c_uint = 0x02;
pub const DCB_GPIO_LOG_DIR_OUT: c_uint = 0x00;
pub const DCB_GPIO_LOG_DIR_IN: c_uint = 0x02;
pub const DCB_GPIO_LOG_VAL: c_uint = 0x01;
pub const DCB_GPIO_LOG_VAL_LO: c_uint = 0x00;
pub const DCB_GPIO_LOG_VAL_HI: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_gpio_func {
    pub func: u8,
    pub line: u8,
    pub log: [u8; 2],
// so far, "param" seems to only have an influence on PWM-related
// GPIOs such as FAN_CONTROL and PANEL_BACKLIGHT_LEVEL.
// if param equals 1, hardware PWM is available
// if param equals 0, the host should toggle the GPIO itself
//
    pub param: u8,
}

extern "C" {
    pub fn dcb_gpio_table(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, cnt: *mut u8, len: *mut u8) -> u16;
}
extern "C" {
    pub fn dcb_gpio_entry(: *mut nvkm_bios, idx: c_int, ent: c_int, ver: *mut u8, len: *mut u8) -> u16;
}
