//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/leds/rgb/leds-lp5860.h
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
// Copyright (c) 2025 Pengutronix
//
// Author: Steffen Trumtrar <kernel@pengutronix.de>
//

pub const LP5860_REG_CHIP_EN: c_uint = 0x00;
pub const LP5860_REG_DEV_INITIAL: c_uint = 0x01;
pub const LP5860_REG_DEV_CONFIG1: c_uint = 0x02;
pub const LP5860_REG_DEV_CONFIG2: c_uint = 0x03;
pub const LP5860_REG_DEV_CONFIG3: c_uint = 0x04;
pub const LP5860_REG_GLOBAL_BRI: c_uint = 0x05;
pub const LP5860_REG_GROUP0_BRI: c_uint = 0x06;
pub const LP5860_REG_GROUP1_BRI: c_uint = 0x07;
pub const LP5860_REG_GROUP2_BRI: c_uint = 0x08;
pub const LP5860_REG_R_CURRENT_SET: c_uint = 0x09;
pub const LP5860_REG_G_CURRENT_SET: c_uint = 0x0A;
pub const LP5860_REG_B_CURRENT_SET: c_uint = 0x0B;
pub const LP5860_REG_GRP_SEL_START: c_uint = 0x0C;
pub const LP5860_REG_DOT_ONOFF_START: c_uint = 0x43;
pub const LP5860_REG_DOT_ONOFF_MAX: c_uint = 0x63;
pub const LP5860_REG_FAULT_STATE: c_uint = 0x64;
pub const LP5860_REG_DOT_LOD_START: c_uint = 0x65;
pub const LP5860_REG_DOT_LSD_START: c_uint = 0x86;
pub const LP5860_REG_LOD_CLEAR: c_uint = 0xA7;
pub const LP5860_REG_LSD_CLEAR: c_uint = 0xA8;
pub const LP5860_REG_RESET: c_uint = 0xA9;
pub const LP5860_REG_DC_START: c_uint = 0x0100;
pub const LP5860_REG_PWM_BRI_START: c_uint = 0x0200;
pub const LP5860_MAX_REG: c_uint = 0x038B;
// Register chip_enable value
pub const LP5860_CHIP_SHIFT: c_int = 0;

// Register dev_initial value
pub const LP5860_MAX_LINE_SHIFT: c_int = 3;

pub const LP5860_MAX_LINE_11: c_uint = 0x0B;
pub const LP5860_MAX_LINE_10: c_uint = 0x0A;
pub const LP5860_MAX_LINE_9: c_uint = 0x09;
pub const LP5860_MAX_LINE_8: c_uint = 0x08;
pub const LP5860_MAX_LINE_7: c_uint = 0x07;
pub const LP5860_MAX_LINE_6: c_uint = 0x06;
pub const LP5860_MAX_LINE_5: c_uint = 0x05;
pub const LP5860_MAX_LINE_4: c_uint = 0x04;
pub const LP5860_MAX_LINE_3: c_uint = 0x03;
pub const LP5860_MAX_LINE_2: c_uint = 0x02;
pub const LP5860_MAX_LINE_1: c_uint = 0x01;
pub const LP5860_MODE_SHIFT: c_int = 1;

pub const LP5860_MODE_3_1: c_uint = 0x03;
pub const LP5860_MODE_3: c_uint = 0x02;
pub const LP5860_MODE_2: c_uint = 0x01;
pub const LP5860_MODE_1: c_uint = 0x00;
pub const LP5860_PWM_FREQUENCY_SHIFT: c_int = 0;

pub const LP5860_PWM_FREQUENCY_62_5K: c_uint = 0x01;
pub const LP5860_PWM_FREQUENCY_125K: c_uint = 0x00;
// Register dev_config1 value
pub const LP5860_SW_BLK_SHIFT: c_int = 3;

pub const LP5860_SW_BLK_05US: c_uint = 0x01;
pub const LP5860_SW_BLK_1US: c_uint = 0x00;
pub const LP5860_PWM_SCALE_MODE_SHIFT: c_int = 2;

pub const LP5860_PWM_SCALE_EXPONENTIAL: c_uint = 0x01;
pub const LP5860_PWM_SCALE_LINEAR: c_uint = 0x00;
pub const LP5860_PWM_PHASESHIFT_SHIFT: c_int = 1;

pub const LP5860_PWM_PHASESHIFT_ON: c_uint = 0x01;
pub const LP5860_PWM_PHASESHIFT_OFF: c_uint = 0x00;
pub const LP5860_CS_ON_SHIFT_SHIFT: c_int = 0;

pub const LP5860_CS_DELAY_ON: c_uint = 0x01;
pub const LP5860_CS_DELAY_OFF: c_uint = 0x00;
// Register dev_config2 value
pub const LP5860_COMP_GROUP3_SHIFT: c_int = 6;

pub const LP5860_COMP_GROUP3_3CLOCK: c_uint = 0x03;
pub const LP5860_COMP_GROUP3_2CLOCK: c_uint = 0x02;
pub const LP5860_COMP_GROUP3_1CLOCK: c_uint = 0x01;
pub const LP5860_COMP_GROUP3_OFF: c_uint = 0x00;
pub const LP5860_COMP_GROUP2_SHIFT: c_int = 4;

pub const LP5860_COMP_GROUP2_3CLOCK: c_uint = 0x03;
pub const LP5860_COMP_GROUP2_2CLOCK: c_uint = 0x02;
pub const LP5860_COMP_GROUP2_1CLOCK: c_uint = 0x01;
pub const LP5860_COMP_GROUP2_OFF: c_uint = 0x00;
pub const LP5860_COMP_GROUP1_SHIFT: c_int = 2;

pub const LP5860_COMP_GROUP1_3CLOCK: c_uint = 0x03;
pub const LP5860_COMP_GROUP1_2CLOCK: c_uint = 0x02;
pub const LP5860_COMP_GROUP1_1CLOCK: c_uint = 0x01;
pub const LP5860_COMP_GROUP1_OFF: c_uint = 0x00;
pub const LP5860_LOD_REMOVAL_SHIFT: c_int = 1;

pub const LP5860_LOD_REMOVAL_EN: c_uint = 0x01;
pub const LP5860_LOD_REMOVAL_OFF: c_uint = 0x00;
pub const LP5860_LSD_REMOVAL_SHIFT: c_int = 0;

pub const LP5860_LSD_REMOVAL_EN: c_uint = 0x01;
pub const LP5860_LSD_REMOVAL_OFF: c_uint = 0x00;
// Register dev_config3 value
pub const LP5860_DOWN_DEGHOST_SHIFT: c_int = 6;

pub const LP5860_DOWN_DEGHOST_STRONG: c_uint = 0x03;
pub const LP5860_DOWN_DEGHOST_MEDIUM: c_uint = 0x02;
pub const LP5860_DOWN_DEGHOST_WEAK: c_uint = 0x01;
pub const LP5860_DOWN_DEGHOST_OFF: c_uint = 0x00;
pub const LP5860_UP_DEGHOST_SHIFT: c_int = 4;

pub const LP5860_UP_DEGHOST_GND: c_uint = 0x03;
pub const LP5860_UP_DEGHOST_3: c_uint = 0x02;
pub const LP5860_UP_DEGHOST_2_5: c_uint = 0x01;
pub const LP5860_UP_DEGHOST_2: c_uint = 0x00;
pub const LP5860_MAXIMUM_CURRENT_SHIFT: c_int = 1;

pub const LP5860_MAXIMUM_CURRENT_50: c_uint = 0x07;
pub const LP5860_MAXIMUM_CURRENT_40: c_uint = 0x06;
pub const LP5860_MAXIMUM_CURRENT_30: c_uint = 0x05;
pub const LP5860_MAXIMUM_CURRENT_20: c_uint = 0x04;
pub const LP5860_MAXIMUM_CURRENT_15: c_uint = 0x03;
pub const LP5860_MAXIMUM_CURRENT_10: c_uint = 0x02;
pub const LP5860_MAXIMUM_CURRENT_5: c_uint = 0x01;
pub const LP5860_MAXIMUM_CURRENT_3: c_uint = 0x00;
pub const LP5860_UP_DEGHOST_ENABLE_SHIFT: c_int = 0;

pub const LP5860_UP_DEGHOST_ENABLE_EN: c_uint = 0x01;
pub const LP5860_UP_DEGHOST_ENABLE_OFF: c_uint = 0x00;
// Register PWM
pub const LP5860_PWM_GLOBAL_MAX: c_uint = 0xff;
pub const LP5860_PWM_GROUP_MAX: c_uint = 0xff;
// Register CC group select

pub const LP5860_CC_GROUP_MAX: c_uint = 0x7F;
// Register dot group select
pub const LP5860_DOT_0_SHIFT: c_int = 0;
pub const LP5860_DOT_1_SHIFT: c_int = 2;
pub const LP5860_DOT_2_SHIFT: c_int = 4;
pub const LP5860_DOT_3_SHIFT: c_int = 6;
pub const LP5860_DOT_GROUP3: c_uint = 0x03;
pub const LP5860_DOT_GROUP2: c_uint = 0x02;
pub const LP5860_DOT_GROUP1: c_uint = 0x01;
pub const LP5860_DOT_GROUP_NONE: c_uint = 0x00;
pub const LP5860_DOT_ALL_ON: c_uint = 0xff;
pub const LP5860_DOT_ALL_OFF: c_uint = 0x0;
pub const LP5860_PWM_DOT_MAX: c_uint = 0xff;
// Dot onoff value
pub const LP5860_DOT_CS0_SHIFT: c_int = 0;
pub const LP5860_DOT_CS1_SHIFT: c_int = 1;
pub const LP5860_DOT_CS2_SHIFT: c_int = 2;
pub const LP5860_DOT_CS3_SHIFT: c_int = 3;
pub const LP5860_DOT_CS4_SHIFT: c_int = 4;
pub const LP5860_DOT_CS5_SHIFT: c_int = 5;
pub const LP5860_DOT_CS6_SHIFT: c_int = 6;
pub const LP5860_DOT_CS7_SHIFT: c_int = 7;
pub const LP5860_DOT_CS_ON: c_uint = 0x01;
pub const LP5860_DOT_CS_OFF: c_uint = 0x00;
// Dot lod value
pub const LP5860_DOT_LOD0_SHIFT: c_int = 0;
pub const LP5860_DOT_LOD1_SHIFT: c_int = 1;
pub const LP5860_DOT_LOD2_SHIFT: c_int = 2;
pub const LP5860_DOT_LOD3_SHIFT: c_int = 3;
pub const LP5860_DOT_LOD4_SHIFT: c_int = 4;
pub const LP5860_DOT_LOD5_SHIFT: c_int = 5;
pub const LP5860_DOT_LOD6_SHIFT: c_int = 6;
pub const LP5860_DOT_LOD7_SHIFT: c_int = 7;
pub const LP5860_DOT_LOD_ON: c_uint = 0x01;
pub const LP5860_DOT_LOD_OFF: c_uint = 0x00;
// dot lsd value
pub const LP5860_DOT_LSD0_SHIFT: c_int = 0;
pub const LP5860_DOT_LSD1_SHIFT: c_int = 1;
pub const LP5860_DOT_LSD2_SHIFT: c_int = 2;
pub const LP5860_DOT_LSD3_SHIFT: c_int = 3;
pub const LP5860_DOT_LSD4_SHIFT: c_int = 4;
pub const LP5860_DOT_LSD5_SHIFT: c_int = 5;
pub const LP5860_DOT_LSD6_SHIFT: c_int = 6;
pub const LP5860_DOT_LSD7_SHIFT: c_int = 7;
pub const LP5860_DOT_LSD_ON: c_uint = 0x01;
pub const LP5860_DOT_LSD_OFF: c_uint = 0x00;
// Register lod state
pub const LP5860_GLOBAL_LOD_SHIFT: c_int = 1;

pub const LP5860_GLOBAL_LSD_SHIFT: c_int = 0;

pub const LP5860_FAULT_STATE_ON: c_uint = 0x01;
pub const LP5860_FAULT_STATE_OFF: c_uint = 0x00;
pub const LP5860_GLOBAL_LOD_CLEAR: c_uint = 0x00;
pub const LP5860_GLOBAL_LSD_CLEAR: c_uint = 0x00;
pub const LP5860_LOD_CLEAR_EN: c_uint = 0xff;
pub const LP5860_LSD_CLEAR_EN: c_uint = 0xff;
pub const LP5860_RESET_EN: c_uint = 0xff;
pub const LP5860_MAX_BRIGHTNESS: c_int = 255;
pub const LP5860_REG_R_PWM: c_uint = 0x0;
pub const LP5860_REG_G_PWM: c_uint = 0x1;
pub const LP5860_REG_B_PWM: c_uint = 0x2;
pub const LP5860_MAX_LED_CONSTANT: c_int = 18;
pub const LP5860_MAX_LED_SCAN: c_int = 11;

pub const LP5860_MAX_DOT_ONOFF_GROUP_NUM: c_int = 8;
//
// Theoretically, there is no max channel per LED,
// limit this to a reasonable value for RGBW LEDs
//
pub const LP5860_MAX_LED_CHANNELS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5860_led {
    pub chip: *mut lp5860,
    pub mc_cdev: led_classdev_mc,
    pub brightness: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp5860 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub leds): DECLARE_FLEX_ARRAY(struct lp5860_led,,
}

extern "C" {
    pub fn lp5860_device_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn lp5860_device_remove(dev: *mut device);
}
