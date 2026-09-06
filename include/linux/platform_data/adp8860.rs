//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/adp8860.h
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
// Definitions and platform data for Analog Devices
// Backlight drivers ADP8860
//
// Copyright 2009-2010 Analog Devices Inc.
//

pub const ID_ADP8860: c_int = 8860;
pub const ADP8860_MAX_BRIGHTNESS: c_uint = 0x7F;
pub const FLAG_OFFT_SHIFT: c_int = 8;
//
// LEDs subdevice platform data
//

pub const ADP8860_LED_ONT_200ms: c_int = 0;
pub const ADP8860_LED_ONT_600ms: c_int = 1;
pub const ADP8860_LED_ONT_800ms: c_int = 2;
pub const ADP8860_LED_ONT_1200ms: c_int = 3;

//
// Backlight subdevice platform data
//

pub const ADP8860_FADE_T_600ms: c_int = 2;
pub const ADP8860_FADE_T_900ms: c_int = 3;
pub const ADP8860_FADE_T_1200ms: c_int = 4;
pub const ADP8860_FADE_T_1500ms: c_int = 5;
pub const ADP8860_FADE_T_1800ms: c_int = 6;
pub const ADP8860_FADE_T_2100ms: c_int = 7;
pub const ADP8860_FADE_T_2400ms: c_int = 8;
pub const ADP8860_FADE_T_2700ms: c_int = 9;
pub const ADP8860_FADE_T_3000ms: c_int = 10;
pub const ADP8860_FADE_T_3500ms: c_int = 11;
pub const ADP8860_FADE_T_4000ms: c_int = 12;
pub const ADP8860_FADE_T_4500ms: c_int = 13;
pub const ADP8860_FADE_T_5000ms: c_int = 14;

pub const ADP8860_FADE_LAW_LINEAR: c_int = 0;
pub const ADP8860_FADE_LAW_SQUARE: c_int = 1;
pub const ADP8860_FADE_LAW_CUBIC1: c_int = 2;
pub const ADP8860_FADE_LAW_CUBIC2: c_int = 3;

pub const ADP8860_BL_AMBL_FILT_160ms: c_int = 1;
pub const ADP8860_BL_AMBL_FILT_320ms: c_int = 2;
pub const ADP8860_BL_AMBL_FILT_640ms: c_int = 3;
pub const ADP8860_BL_AMBL_FILT_1280ms: c_int = 4;
pub const ADP8860_BL_AMBL_FILT_2560ms: c_int = 5;
pub const ADP8860_BL_AMBL_FILT_5120ms: c_int = 6;

//
// Blacklight current 0..30mA
//

//
// L2 comparator current 0..1106uA
//

//
// L3 comparator current 0..138uA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp8860_backlight_platform_data {
    pub /: *mut *mut u8 bl_led_assign; / 1 = Backlight 0 = Individual LED,
    pub /: *mut *mut u8 bl_fade_in; / Backlight Fade-In Timer,
    pub /: *mut *mut u8 bl_fade_out; / Backlight Fade-Out Timer,
    pub /: *mut *mut u8 bl_fade_law; / fade-on/fade-off transfer characteristic,
    pub /: *mut *mut u8 en_ambl_sens; / 1 = enable ambient light sensor,
    pub /: *mut *mut u8 abml_filt; / Light sensor filter time,
    pub /: *mut *mut u8 l1_daylight_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l1_daylight_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_office_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_office_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l3_dark_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l3_dark_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_trip; / use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA,
    pub /: *mut *mut u8 l2_hyst; / use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA,
    pub /: *mut *mut u8 l3_trip; / use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA,
    pub /: *mut *mut u8 l3_hyst; / use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA,
//
// Independent Current Sinks / LEDS
// Sinks not assigned to the Backlight can be exposed to
// user space using the LEDS CLASS interface
//
    pub num_leds: c_int,
    pub leds: *mut led_info,
    pub /: *mut *mut u8 led_fade_in; / LED Fade-In Timer,
    pub /: *mut *mut u8 led_fade_out; / LED Fade-Out Timer,
    pub /: *mut *mut u8 led_fade_law; / fade-on/fade-off transfer characteristic,
    pub led_on_time: u8,
//
// Gain down disable. Setting this option does not allow the
// charge pump to switch to lower gains. NOT AVAILABLE on ADP8860
// 1 = the charge pump doesn't switch down in gain until all LEDs are 0.
// The charge pump switches up in gain as needed. This feature is
// useful if the ADP8863 charge pump is used to drive an external load.
// This feature must be used when utilizing small fly capacitors
// (0402 or smaller).
// 0 = the charge pump automatically switches up and down in gain.
// This provides optimal efficiency, but is not suitable for driving
// loads that are not connected through the ADP8863 diode drivers.
// Additionally, the charge pump fly capacitors should be low ESR
// and sized 0603 or greater.
//
    pub gdwn_dis: u8,
}
