//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77693.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// max77693.h - Driver for the Maxim 77693
//
// Copyright (C) 2012 Samsung Electronics
// SangYoung Son <hello.son@samsung.com>
//
// This program is not provided / owned by Maxim Integrated Products.
//
// This driver is based on max8997.h
//
// MAX77693 has PMIC, Charger, Flash LED, Haptic, MUIC devices.
// The devices share the same I2C bus and included in
// this mfd driver.
//
// MAX77693 regulator IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_regulators {
    MAX77693_ESAFEOUT1 = 0,
    MAX77693_ESAFEOUT2,
    MAX77693_CHARGER,
    MAX77693_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77693_reg_data {
    pub addr: u8,
    pub data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77693_muic_platform_data {
    pub init_data: *mut max77693_reg_data,
    pub num_init_data: c_int,
    pub detcable_delay_ms: c_int,
//
// Default usb/uart path whether UART/USB or AUX_UART/AUX_USB
// h/w path of COMP2/COMN1 on CONTROL1 register.
//
    pub path_usb: c_int,
    pub path_uart: c_int,
}

// MAX77693 led flash
// triggers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_led_trigger {
    MAX77693_LED_TRIG_OFF,
    MAX77693_LED_TRIG_FLASH,
    MAX77693_LED_TRIG_TORCH,
    MAX77693_LED_TRIG_EXT,
    MAX77693_LED_TRIG_SOFT,
}

// trigger types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_led_trigger_type {
    MAX77693_LED_TRIG_TYPE_EDGE,
    MAX77693_LED_TRIG_TYPE_LEVEL,
}

// boost modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77693_led_boost_mode {
    MAX77693_LED_BOOST_NONE,
    MAX77693_LED_BOOST_ADAPTIVE,
    MAX77693_LED_BOOST_FIXED,
}

// MAX77693
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77693_platform_data {
// muic data
    pub muic_data: *mut max77693_muic_platform_data,
    pub led_data: *mut max77693_led_platform_data,
}
