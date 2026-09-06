//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sm501.h
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
// include/linux/sm501.h
//
// Copyright (c) 2006 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
// Vincent Sanders <vince@simtec.co.uk>
//
// sm501_misc_control
//
// Modify the SM501's MISC_CONTROL register
//
// sm501_modify_reg
//
// Modify a register in the SM501 which may be shared with other
// drivers.
//
// Platform data definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_platdata_fbsub {
    pub def_mode: *mut fb_videomode,
    pub def_bpp: c_uint,
    pub max_mem: c_ulong,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sm501_fb_routing {
    SM501_FB_OWN		= 0,	/* CRT=>CRT, Panel=>Panel */
    SM501_FB_CRT_PANEL	= 1,	/* Panel=>CRT, Panel=>Panel */
}

// sm501_platdata_fb flag field bit definitions

// sm501_platdata_fb
//
// configuration data for the framebuffer driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_platdata_fb {
    pub fb_route: sm501_fb_routing,
    pub flags: c_uint,
    pub fb_crt: *mut sm501_platdata_fbsub,
    pub fb_pnl: *mut sm501_platdata_fbsub,
}

// gpio i2c
//
// Note, we have to pass in the bus number, as the number used will be
// passed to the i2c-gpio driver's platform_device.id, subsequently used
// to register the i2c bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_platdata_gpio_i2c {
    pub bus_num: c_uint,
    pub pin_sda: c_uint,
    pub pin_scl: c_uint,
    pub udelay: c_int,
    pub timeout: c_int,
}

// sm501_initdata
//
// use for initialising values that may not have been setup
// before the driver is loaded.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_reg_init {
    pub set: c_ulong,
    pub mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_initdata {
    pub gpio_low: sm501_reg_init,
    pub gpio_high: sm501_reg_init,
    pub misc_timing: sm501_reg_init,
    pub misc_control: sm501_reg_init,
    pub devices: c_ulong,
    pub /: *mut *mut unsigned long mclk; / non-zero to modify,
    pub /: *mut *mut unsigned long m1xclk; / non-zero to modify,
}

// sm501_init_gpio
//
// default gpio settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_init_gpio {
    pub gpio_data_low: sm501_reg_init,
    pub gpio_data_high: sm501_reg_init,
    pub gpio_ddr_low: sm501_reg_init,
    pub gpio_ddr_high: sm501_reg_init,
}

// sm501_platdata
//
// This is passed with the platform device to allow the board
// to control the behaviour of the SM501 driver(s) which attach
// to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm501_platdata {
    pub init: *mut sm501_initdata,
    pub init_gpiop: *mut sm501_init_gpio,
    pub fb: *mut sm501_platdata_fb,
    pub flags: c_int,
    pub gpio_base: c_int,
    pub dev): *mut *mut int (get_power)(struct device,
    pub on): *mut *mut *mut int (set_power)(struct device dev, unsigned int,
    pub gpio_i2c: *mut sm501_platdata_gpio_i2c,
    pub gpio_i2c_nr: c_uint,
}

