//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/adp5520.h
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
// ADP5520/ADP5501 MFD PMICs (Backlight, LED, GPIO and Keys)
//
// Copyright 2009 Analog Devices Inc.
//
pub const ID_ADP5520: c_int = 5520;
pub const ID_ADP5501: c_int = 5501;
//
// ADP5520/ADP5501 Register Map
//
pub const ADP5520_MODE_STATUS: c_uint = 0x00;
pub const ADP5520_INTERRUPT_ENABLE: c_uint = 0x01;
pub const ADP5520_BL_CONTROL: c_uint = 0x02;
pub const ADP5520_BL_TIME: c_uint = 0x03;
pub const ADP5520_BL_FADE: c_uint = 0x04;
pub const ADP5520_DAYLIGHT_MAX: c_uint = 0x05;
pub const ADP5520_DAYLIGHT_DIM: c_uint = 0x06;
pub const ADP5520_OFFICE_MAX: c_uint = 0x07;
pub const ADP5520_OFFICE_DIM: c_uint = 0x08;
pub const ADP5520_DARK_MAX: c_uint = 0x09;
pub const ADP5520_DARK_DIM: c_uint = 0x0A;
pub const ADP5520_BL_VALUE: c_uint = 0x0B;
pub const ADP5520_ALS_CMPR_CFG: c_uint = 0x0C;
pub const ADP5520_L2_TRIP: c_uint = 0x0D;
pub const ADP5520_L2_HYS: c_uint = 0x0E;
pub const ADP5520_L3_TRIP: c_uint = 0x0F;
pub const ADP5520_L3_HYS: c_uint = 0x10;
pub const ADP5520_LED_CONTROL: c_uint = 0x11;
pub const ADP5520_LED_TIME: c_uint = 0x12;
pub const ADP5520_LED_FADE: c_uint = 0x13;
pub const ADP5520_LED1_CURRENT: c_uint = 0x14;
pub const ADP5520_LED2_CURRENT: c_uint = 0x15;
pub const ADP5520_LED3_CURRENT: c_uint = 0x16;
//
// ADP5520 Register Map
//
pub const ADP5520_GPIO_CFG_1: c_uint = 0x17;
pub const ADP5520_GPIO_CFG_2: c_uint = 0x18;
pub const ADP5520_GPIO_IN: c_uint = 0x19;
pub const ADP5520_GPIO_OUT: c_uint = 0x1A;
pub const ADP5520_GPIO_INT_EN: c_uint = 0x1B;
pub const ADP5520_GPIO_INT_STAT: c_uint = 0x1C;
pub const ADP5520_GPIO_INT_LVL: c_uint = 0x1D;
pub const ADP5520_GPIO_DEBOUNCE: c_uint = 0x1E;
pub const ADP5520_GPIO_PULLUP: c_uint = 0x1F;
pub const ADP5520_KP_INT_STAT_1: c_uint = 0x20;
pub const ADP5520_KP_INT_STAT_2: c_uint = 0x21;
pub const ADP5520_KR_INT_STAT_1: c_uint = 0x22;
pub const ADP5520_KR_INT_STAT_2: c_uint = 0x23;
pub const ADP5520_KEY_STAT_1: c_uint = 0x24;
pub const ADP5520_KEY_STAT_2: c_uint = 0x25;
//
// MODE_STATUS bits
//

//
// INTERRUPT_ENABLE bits
//

//
// BL_CONTROL bits
//

//
// ALS_CMPR_CFG bits
//

pub const ADP5020_MAX_BRIGHTNESS: c_uint = 0x7F;

//
// LEDs subdevice bits and masks
//
pub const ADP5520_01_MAXLEDS: c_int = 3;
pub const ADP5520_FLAG_LED_MASK: c_uint = 0x3;
pub const ADP5520_FLAG_OFFT_SHIFT: c_int = 8;
pub const ADP5520_FLAG_OFFT_MASK: c_uint = 0x3;

//
// GPIO subdevice bits and masks
//
pub const ADP5520_MAXGPIOS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_gpio_platform_data {
    pub gpio_start: unsigned,
    pub gpio_en_mask: u8,
    pub gpio_pullup_mask: u8,
}

//
// Keypad subdevice bits and masks
//
pub const ADP5520_MAXKEYS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_keys_platform_data {
    pub /: *mut *mut int rows_en_mask; / Number of rows,
    pub /: *mut *mut int cols_en_mask; / Number of columns,
    pub /: *const *const *const unsigned short keymap; / Pointer to keymap,
    pub /: *mut *mut unsigned short keymapsize; / Keymap size,
    pub /: *mut *mut unsigned repeat:1; / Enable key repeat,
}

//
// LEDs subdevice platform data
//

pub const ADP5520_LED_ONT_200ms: c_int = 0;
pub const ADP5520_LED_ONT_600ms: c_int = 1;
pub const ADP5520_LED_ONT_800ms: c_int = 2;
pub const ADP5520_LED_ONT_1200ms: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_leds_platform_data {
    pub num_leds: c_int,
    pub leds: *mut led_info,
    pub /: *mut *mut u8 fade_in; / Backlight Fade-In Timer,
    pub /: *mut *mut u8 fade_out; / Backlight Fade-Out Timer,
    pub led_on_time: u8,
}

//
// Backlight subdevice platform data
//

pub const ADP5520_FADE_T_600ms: c_int = 2;
pub const ADP5520_FADE_T_900ms: c_int = 3;
pub const ADP5520_FADE_T_1200ms: c_int = 4;
pub const ADP5520_FADE_T_1500ms: c_int = 5;
pub const ADP5520_FADE_T_1800ms: c_int = 6;
pub const ADP5520_FADE_T_2100ms: c_int = 7;
pub const ADP5520_FADE_T_2400ms: c_int = 8;
pub const ADP5520_FADE_T_2700ms: c_int = 9;
pub const ADP5520_FADE_T_3000ms: c_int = 10;
pub const ADP5520_FADE_T_3500ms: c_int = 11;
pub const ADP5520_FADE_T_4000ms: c_int = 12;
pub const ADP5520_FADE_T_4500ms: c_int = 13;
pub const ADP5520_FADE_T_5000ms: c_int = 14;

pub const ADP5520_BL_LAW_LINEAR: c_int = 0;
pub const ADP5520_BL_LAW_SQUARE: c_int = 1;
pub const ADP5520_BL_LAW_CUBIC1: c_int = 2;
pub const ADP5520_BL_LAW_CUBIC2: c_int = 3;

pub const ADP5520_BL_AMBL_FILT_160ms: c_int = 1;
pub const ADP5520_BL_AMBL_FILT_320ms: c_int = 2;
pub const ADP5520_BL_AMBL_FILT_640ms: c_int = 3;
pub const ADP5520_BL_AMBL_FILT_1280ms: c_int = 4;
pub const ADP5520_BL_AMBL_FILT_2560ms: c_int = 5;
pub const ADP5520_BL_AMBL_FILT_5120ms: c_int = 6;

//
// Blacklight current 0..30mA
//

//
// L2 comparator current 0..1000uA
//

//
// L3 comparator current 0..127uA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_backlight_platform_data {
    pub /: *mut *mut u8 fade_in; / Backlight Fade-In Timer,
    pub /: *mut *mut u8 fade_out; / Backlight Fade-Out Timer,
    pub /: *mut *mut u8 fade_led_law; / fade-on/fade-off transfer characteristic,
    pub /: *mut *mut u8 en_ambl_sens; / 1 = enable ambient light sensor,
    pub /: *mut *mut u8 abml_filt; / Light sensor filter time,
    pub /: *mut *mut u8 l1_daylight_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l1_daylight_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_office_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_office_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l3_dark_max; / use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l3_dark_dim; / typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA,
    pub /: *mut *mut u8 l2_trip; / use L2_COMP_CURR_uA(I) 0 <= I <= 1000 uA,
    pub /: *mut *mut u8 l2_hyst; / use L2_COMP_CURR_uA(I) 0 <= I <= 1000 uA,
    pub /: *mut *mut u8 l3_trip; / use L3_COMP_CURR_uA(I) 0 <= I <= 127 uA,
    pub /: *mut *mut u8 l3_hyst; / use L3_COMP_CURR_uA(I) 0 <= I <= 127 uA,
}

//
// MFD chip platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_platform_data {
    pub keys: *mut adp5520_keys_platform_data,
    pub gpio: *mut adp5520_gpio_platform_data,
    pub leds: *mut adp5520_leds_platform_data,
    pub backlight: *mut adp5520_backlight_platform_data,
}

//
// MFD chip functions
//
extern "C" {
    pub fn adp5520_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn adp5520_write(dev: *mut device, reg: c_int, val: u8) -> c_int;
}
extern "C" {
    pub fn adp5520_clr_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
extern "C" {
    pub fn adp5520_set_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
}
