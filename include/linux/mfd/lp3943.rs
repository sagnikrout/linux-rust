//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lp3943.h
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
// TI/National Semiconductor LP3943 Device
//
// Copyright 2013 Texas Instruments
//
// Author: Milo Kim <milo.kim@ti.com>
//

// Registers
pub const LP3943_REG_GPIO_A: c_uint = 0x00;
pub const LP3943_REG_GPIO_B: c_uint = 0x01;
pub const LP3943_REG_PRESCALE0: c_uint = 0x02;
pub const LP3943_REG_PWM0: c_uint = 0x03;
pub const LP3943_REG_PRESCALE1: c_uint = 0x04;
pub const LP3943_REG_PWM1: c_uint = 0x05;
pub const LP3943_REG_MUX0: c_uint = 0x06;
pub const LP3943_REG_MUX1: c_uint = 0x07;
pub const LP3943_REG_MUX2: c_uint = 0x08;
pub const LP3943_REG_MUX3: c_uint = 0x09;
// Bit description for LP3943_REG_MUX0 ~ 3
pub const LP3943_GPIO_IN: c_uint = 0x00;
pub const LP3943_GPIO_OUT_HIGH: c_uint = 0x00;
pub const LP3943_GPIO_OUT_LOW: c_uint = 0x01;
pub const LP3943_DIM_PWM0: c_uint = 0x02;
pub const LP3943_DIM_PWM1: c_uint = 0x03;
pub const LP3943_NUM_PWMS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3943_pwm_output {
    LP3943_PWM_OUT0,
    LP3943_PWM_OUT1,
    LP3943_PWM_OUT2,
    LP3943_PWM_OUT3,
    LP3943_PWM_OUT4,
    LP3943_PWM_OUT5,
    LP3943_PWM_OUT6,
    LP3943_PWM_OUT7,
    LP3943_PWM_OUT8,
    LP3943_PWM_OUT9,
    LP3943_PWM_OUT10,
    LP3943_PWM_OUT11,
    LP3943_PWM_OUT12,
    LP3943_PWM_OUT13,
    LP3943_PWM_OUT14,
    LP3943_PWM_OUT15,
}

//
// struct lp3943_pwm_map
// @output: Output pins which are mapped to each PWM channel
// @num_outputs: Number of outputs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3943_pwm_map {
    pub output: *mut lp3943_pwm_output,
    pub num_outputs: c_int,
}

//
// struct lp3943_platform_data
// @pwms: Output channel definitions for PWM channel 0 and 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3943_platform_data {
    pub pwms: [*mut lp3943_pwm_map; LP3943_NUM_PWMS],
}

//
// struct lp3943_reg_cfg
// @reg: Register address
// @mask: Register bit mask to be updated
// @shift: Register bit shift
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3943_reg_cfg {
    pub reg: u8,
    pub mask: u8,
    pub shift: u8,
}

//
// struct lp3943
// @dev: Parent device pointer
// @regmap: Used for I2C communication on accessing registers
// @pdata: LP3943 platform specific data
// @mux_cfg: Register configuration for pin MUX
// @pin_used: Bit mask for output pin used.
// This bitmask is used for pin assignment management.
// 1 = pin used, 0 = available.
// Only LSB 16 bits are used, but it is unsigned long type
// for atomic bitwise operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3943 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pdata: *mut lp3943_platform_data,
    pub mux_cfg: *const lp3943_reg_cfg,
    pub pin_used: c_ulong,
}

extern "C" {
    pub fn lp3943_read_byte(lp3943: *mut lp3943, reg: u8, read: *mut u8) -> c_int;
}
extern "C" {
    pub fn lp3943_write_byte(lp3943: *mut lp3943, reg: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn lp3943_update_bits(lp3943: *mut lp3943, reg: u8, mask: u8, data: u8) -> c_int;
}
