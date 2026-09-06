//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/leds/leds-lp55xx-common.h
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
// LP55XX Common Driver Header
//
// Copyright (C) 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//
// Derived from leds-lp5521.c, leds-lp5523.c
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp55xx_engine_index {
    LP55XX_ENGINE_INVALID,
    LP55XX_ENGINE_1,
    LP55XX_ENGINE_2,
    LP55XX_ENGINE_3,
    LP55XX_ENGINE_MAX = LP55XX_ENGINE_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp55xx_engine_mode {
    LP55XX_ENGINE_DISABLED,
    LP55XX_ENGINE_LOAD,
    LP55XX_ENGINE_RUN,
}

//
// struct lp55xx_reg
// @addr : Register address
// @val  : Register value (can also used as mask or shift)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_reg {
    pub addr: u8,
    pub val: u8,
    pub mask: u8,
    pub shift: u8,
}

//
// struct lp55xx_device_config
// @reg_op_mode        : Chip specific OP MODE reg addr
// @engine_busy        : Chip specific engine busy
// (if not supported 153 us sleep)
// @reset              : Chip specific reset command
// @enable             : Chip specific enable command
// @prog_mem_base      : Chip specific base reg address for chip SMEM programming
// @reg_led_pwm_base   : Chip specific base reg address for LED PWM conf
// @reg_led_current_base : Chip specific base reg address for LED current conf
// @reg_master_fader_base : Chip specific base reg address for master fader base
// @reg_led_ctrl_base  : Chip specific base reg address for LED ctrl base
// @pages_per_engine   : Assigned pages for each engine
// (if not set chip doesn't support pages)
// @max_channel        : Maximum number of channels
// @post_init_device   : Chip specific initialization code
// @brightness_fn      : Brightness function
// @multicolor_brightness_fn : Multicolor brightness function
// @set_led_current    : LED current set function
// @firmware_cb        : Call function when the firmware is loaded
// @run_engine         : Run internal engine for pattern
// @dev_attr_group     : Device specific attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_device_config {
    pub /: *const *const lp55xx_reg reg_op_mode; / addr, shift,
    pub /: *const *const lp55xx_reg reg_exec; / addr, shift,
    pub /: *const *const lp55xx_reg engine_busy; / addr, mask,
    pub reset: lp55xx_reg,
    pub enable: lp55xx_reg,
    pub prog_mem_base: lp55xx_reg,
    pub reg_led_pwm_base: lp55xx_reg,
    pub reg_led_current_base: lp55xx_reg,
    pub reg_master_fader_base: lp55xx_reg,
    pub reg_led_ctrl_base: lp55xx_reg,
    pub pages_per_engine: c_int,
    pub max_channel: c_int,
// define if the device has specific initialization process
    pub chip): *mut *mut int (post_init_device) (struct lp55xx_chip,
// set LED brightness
    pub led): *mut *mut int (brightness_fn)(struct lp55xx_led,
// set multicolor LED brightness
    pub led): *mut *mut int (multicolor_brightness_fn)(struct lp55xx_led,
// current setting function
    pub led_current): *mut *mut *mut void (set_led_current) (struct lp55xx_led led, u8,
// access program memory when the firmware is loaded
    pub chip): *mut *mut void (firmware_cb)(struct lp55xx_chip,
// used for running firmware LED patterns
    pub start): *mut *mut *mut void (run_engine) (struct lp55xx_chip chip, bool,
// additional device specific attributes
    pub dev_attr_group: *const attribute_group,
}

//
// struct lp55xx_engine
// @mode       : Engine mode
// @led_mux    : Mux bits for LED selection. Only used in LP5523
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_engine {
    pub mode: lp55xx_engine_mode,
    pub led_mux: u16,
}

//
// struct lp55xx_chip
// @cl         : I2C communication for access registers
// @pdata      : Platform specific data
// @lock       : Lock for user-space interface
// @num_leds   : Number of registered LEDs
// @cfg        : Device specific configuration data
// @engine_idx : Selected engine number
// @engines    : Engine structure for the device attribute R/W interface
// @fw         : Firmware data for running a LED pattern
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_chip {
    pub cl: *mut i2c_client,
    pub pdata: *mut lp55xx_platform_data,
    pub /: *mut *mut mutex lock; / lock for user-space interface,
    pub num_leds: c_int,
    pub cfg: *const lp55xx_device_config,
    pub engine_idx: lp55xx_engine_index,
    pub engines: [lp55xx_engine; LP55XX_ENGINE_MAX],
    pub fw: *const firmware,
}

//
// struct lp55xx_led
// @chan_nr         : Channel number
// @cdev            : LED class device
// @mc_cdev         : Multi color class device
// @color_components: Multi color LED map information
// @led_current     : Current setting at each led channel
// @max_current     : Maximun current at each led channel
// @brightness      : Brightness value
// @chip            : The lp55xx chip data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_led {
    pub chan_nr: c_int,
    pub cdev: led_classdev,
    pub mc_cdev: led_classdev_mc,
    pub led_current: u8,
    pub max_current: u8,
    pub brightness: u8,
    pub chip: *mut lp55xx_chip,
}

// register access
extern "C" {
    pub fn lp55xx_write(chip: *mut lp55xx_chip, reg: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn lp55xx_read(chip: *mut lp55xx_chip, reg: u8, val: *mut u8) -> c_int;
}
// external clock detection
extern "C" {
    pub fn lp55xx_is_extclk_used(chip: *mut lp55xx_chip) -> bool;
}
// common chip functions
extern "C" {
    pub fn lp55xx_stop_all_engine(chip: *mut lp55xx_chip);
}
extern "C" {
    pub fn lp55xx_load_engine(chip: *mut lp55xx_chip);
}
extern "C" {
    pub fn lp55xx_run_engine_common(chip: *mut lp55xx_chip) -> c_int;
}
extern "C" {
    pub fn lp55xx_firmware_loaded_cb(chip: *mut lp55xx_chip);
}
extern "C" {
    pub fn lp55xx_led_brightness(led: *mut lp55xx_led) -> c_int;
}
extern "C" {
    pub fn lp55xx_multicolor_brightness(led: *mut lp55xx_led) -> c_int;
}
extern "C" {
    pub fn lp55xx_set_led_current(led: *mut lp55xx_led, led_current: u8);
}
extern "C" {
    pub fn lp55xx_turn_off_channels(chip: *mut lp55xx_chip);
}
extern "C" {
    pub fn lp55xx_stop_engine(chip: *mut lp55xx_chip);
}
// common probe/remove function
extern "C" {
    pub fn lp55xx_probe(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn lp55xx_remove(client: *mut i2c_client);
}
// common sysfs function
