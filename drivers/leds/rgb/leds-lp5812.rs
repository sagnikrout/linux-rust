//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/leds/rgb/leds-lp5812.h
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
// LP5812 Driver Header
//
// Copyright (C) 2025 Texas Instruments
//
// Author: Jared Zhou <jared-zhou@ti.com>
//

pub const LP5812_REG_ENABLE: c_uint = 0x0000;
pub const LP5812_REG_RESET: c_uint = 0x0023;
pub const LP5812_DEV_CONFIG0: c_uint = 0x0001;
pub const LP5812_DEV_CONFIG1: c_uint = 0x0002;
pub const LP5812_DEV_CONFIG2: c_uint = 0x0003;
pub const LP5812_DEV_CONFIG3: c_uint = 0x0004;
pub const LP5812_DEV_CONFIG4: c_uint = 0x0005;
pub const LP5812_DEV_CONFIG5: c_uint = 0x0006;
pub const LP5812_DEV_CONFIG6: c_uint = 0x0007;
pub const LP5812_DEV_CONFIG7: c_uint = 0x0008;
pub const LP5812_DEV_CONFIG8: c_uint = 0x0009;
pub const LP5812_DEV_CONFIG9: c_uint = 0x000A;
pub const LP5812_DEV_CONFIG10: c_uint = 0x000B;
pub const LP5812_DEV_CONFIG11: c_uint = 0x000c;
pub const LP5812_DEV_CONFIG12: c_uint = 0x000D;
pub const LP5812_CMD_UPDATE: c_uint = 0x0010;
pub const LP5812_LED_EN_1: c_uint = 0x0020;
pub const LP5812_LED_EN_2: c_uint = 0x0021;
pub const LP5812_FAULT_CLEAR: c_uint = 0x0022;
pub const LP5812_MANUAL_DC_BASE: c_uint = 0x0030;
pub const LP5812_AUTO_DC_BASE: c_uint = 0x0050;
pub const LP5812_MANUAL_PWM_BASE: c_uint = 0x0040;
pub const LP5812_TSD_CONFIG_STATUS: c_uint = 0x0300;
pub const LP5812_LOD_STATUS: c_uint = 0x0301;
pub const LP5812_LSD_STATUS: c_uint = 0x0303;
pub const LP5812_ENABLE: c_uint = 0x01;
pub const LP5812_DISABLE: c_uint = 0x00;
pub const FAULT_CLEAR_ALL: c_uint = 0x07;
pub const TSD_CLEAR_VAL: c_uint = 0x04;
pub const LSD_CLEAR_VAL: c_uint = 0x02;
pub const LOD_CLEAR_VAL: c_uint = 0x01;
pub const LP5812_RESET: c_uint = 0x66;
pub const LP5812_DEV_CONFIG12_DEFAULT: c_uint = 0x08;
pub const LP5812_UPDATE_CMD_VAL: c_uint = 0x55;
pub const LP5812_REG_ADDR_HIGH_SHIFT: c_int = 8;
pub const LP5812_REG_ADDR_BIT_8_9_MASK: c_uint = 0x03;
pub const LP5812_REG_ADDR_LOW_MASK: c_uint = 0xFF;
pub const LP5812_CHIP_ADDR_SHIFT: c_int = 2;
pub const LP5812_DATA_LENGTH: c_int = 2;
pub const LP5812_DATA_BYTE_0_IDX: c_int = 0;
pub const LP5812_DATA_BYTE_1_IDX: c_int = 1;
pub const LP5812_READ_MSG_LENGTH: c_int = 2;
pub const LP5812_MSG_0_IDX: c_int = 0;
pub const LP5812_MSG_1_IDX: c_int = 1;
pub const LP5812_CFG_ERR_STATUS_MASK: c_uint = 0x01;
pub const LP5812_CFG_TSD_STATUS_SHIFT: c_int = 1;
pub const LP5812_CFG_TSD_STATUS_MASK: c_uint = 0x01;
pub const LP5812_FAULT_CLEAR_LOD: c_int = 0;
pub const LP5812_FAULT_CLEAR_LSD: c_int = 1;
pub const LP5812_FAULT_CLEAR_TSD: c_int = 2;
pub const LP5812_FAULT_CLEAR_ALL: c_int = 3;
pub const LP5812_NUMBER_LED_IN_REG: c_int = 8;
pub const LP5812_WAIT_DEVICE_STABLE_MIN: c_int = 1000;
pub const LP5812_WAIT_DEVICE_STABLE_MAX: c_int = 1100;
pub const LP5812_LSD_LOD_START_UP: c_uint = 0x0B;
pub const LP5812_MODE_NAME_MAX_LEN: c_int = 20;

pub const LP5812_MODE_DIRECT_VALUE: c_int = 0;
pub const LP5812_MODE_MIX_SELECT_LED_0: c_int = 0;
pub const LP5812_MODE_MIX_SELECT_LED_1: c_int = 1;
pub const LP5812_MODE_MIX_SELECT_LED_2: c_int = 2;
pub const LP5812_MODE_MIX_SELECT_LED_3: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum control_mode {
    LP5812_MODE_MANUAL = 0,
    LP5812_MODE_AUTONOMOUS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dimming_type {
    LP5812_DIMMING_ANALOG,
    LP5812_DIMMING_PWM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lp5812_scan_order {
    pub order0:2: u8,
    pub order1:2: u8,
    pub order2:2: u8,
    pub order3:2: u8,
    pub bits: },
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lp5812_drive_mode {
    pub mix_sel_led_0:1: u8,
    pub mix_sel_led_1:1: u8,
    pub mix_sel_led_2:1: u8,
    pub mix_sel_led_3:1: u8,
    pub led_mode:3: u8,
    pub pwm_fre:1: u8,
    pub bits: },
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5812_reg {
    pub addr: u16,
    pub val: u8,
    pub mask: u8,
    pub shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5812_mode_mapping {
    pub mode_name: [c_char; LP5812_MODE_NAME_MAX_LEN],
    pub mode: u8,
    pub scan_order_0: u8,
    pub scan_order_1: u8,
    pub scan_order_2: u8,
    pub scan_order_3: u8,
    pub selection_led: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5812_led_config {
    pub is_sc_led: bool,
    pub name: *const c_char,
    pub color_id: [u8; LED_COLOR_ID_MAX],
    pub max_current: [u32; LED_COLOR_ID_MAX],
    pub chan_nr: c_int,
    pub num_colors: c_int,
    pub led_id: [c_int; LED_COLOR_ID_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5812_chip {
    pub num_channels: u8,
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / Protects register access,
    pub led_config: *mut lp5812_led_config,
    pub label: *const c_char,
    pub scan_mode: *const c_char,
    pub scan_order: lp5812_scan_order,
    pub drive_mode: lp5812_drive_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5812_led {
    pub brightness: u8,
    pub chan_nr: c_int,
    pub cdev: led_classdev,
    pub mc_cdev: led_classdev_mc,
    pub chip: *mut lp5812_chip,
}
