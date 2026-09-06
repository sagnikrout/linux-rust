//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/aat2870.h
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
// linux/include/linux/mfd/aat2870.h
//
// Copyright (c) 2011, NVIDIA Corporation.
// Author: Jin Park <jinyoungp@nvidia.com>
//

// Register offsets
pub const AAT2870_BL_CH_EN: c_uint = 0x00;
pub const AAT2870_BLM: c_uint = 0x01;
pub const AAT2870_BLS: c_uint = 0x02;
pub const AAT2870_BL1: c_uint = 0x03;
pub const AAT2870_BL2: c_uint = 0x04;
pub const AAT2870_BL3: c_uint = 0x05;
pub const AAT2870_BL4: c_uint = 0x06;
pub const AAT2870_BL5: c_uint = 0x07;
pub const AAT2870_BL6: c_uint = 0x08;
pub const AAT2870_BL7: c_uint = 0x09;
pub const AAT2870_BL8: c_uint = 0x0A;
pub const AAT2870_FLR: c_uint = 0x0B;
pub const AAT2870_FM: c_uint = 0x0C;
pub const AAT2870_FS: c_uint = 0x0D;
pub const AAT2870_ALS_CFG0: c_uint = 0x0E;
pub const AAT2870_ALS_CFG1: c_uint = 0x0F;
pub const AAT2870_ALS_CFG2: c_uint = 0x10;
pub const AAT2870_AMB: c_uint = 0x11;
pub const AAT2870_ALS0: c_uint = 0x12;
pub const AAT2870_ALS1: c_uint = 0x13;
pub const AAT2870_ALS2: c_uint = 0x14;
pub const AAT2870_ALS3: c_uint = 0x15;
pub const AAT2870_ALS4: c_uint = 0x16;
pub const AAT2870_ALS5: c_uint = 0x17;
pub const AAT2870_ALS6: c_uint = 0x18;
pub const AAT2870_ALS7: c_uint = 0x19;
pub const AAT2870_ALS8: c_uint = 0x1A;
pub const AAT2870_ALS9: c_uint = 0x1B;
pub const AAT2870_ALSA: c_uint = 0x1C;
pub const AAT2870_ALSB: c_uint = 0x1D;
pub const AAT2870_ALSC: c_uint = 0x1E;
pub const AAT2870_ALSD: c_uint = 0x1F;
pub const AAT2870_ALSE: c_uint = 0x20;
pub const AAT2870_ALSF: c_uint = 0x21;
pub const AAT2870_SUB_SET: c_uint = 0x22;
pub const AAT2870_SUB_CTRL: c_uint = 0x23;
pub const AAT2870_LDO_AB: c_uint = 0x24;
pub const AAT2870_LDO_CD: c_uint = 0x25;
pub const AAT2870_LDO_EN: c_uint = 0x26;
pub const AAT2870_REG_NUM: c_uint = 0x27;
// Device IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aat2870_id {
    AAT2870_ID_BL,
    AAT2870_ID_LDOA,
    AAT2870_ID_LDOB,
    AAT2870_ID_LDOC,
    AAT2870_ID_LDOD
}

// Backlight channels
pub const AAT2870_BL_CH1: c_uint = 0x01;
pub const AAT2870_BL_CH2: c_uint = 0x02;
pub const AAT2870_BL_CH3: c_uint = 0x04;
pub const AAT2870_BL_CH4: c_uint = 0x08;
pub const AAT2870_BL_CH5: c_uint = 0x10;
pub const AAT2870_BL_CH6: c_uint = 0x20;
pub const AAT2870_BL_CH7: c_uint = 0x40;
pub const AAT2870_BL_CH8: c_uint = 0x80;
pub const AAT2870_BL_CH_ALL: c_uint = 0xFF;
// Backlight current magnitude (mA)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aat2870_current {
    AAT2870_CURRENT_0_45 = 1,
    AAT2870_CURRENT_0_90,
    AAT2870_CURRENT_1_80,
    AAT2870_CURRENT_2_70,
    AAT2870_CURRENT_3_60,
    AAT2870_CURRENT_4_50,
    AAT2870_CURRENT_5_40,
    AAT2870_CURRENT_6_30,
    AAT2870_CURRENT_7_20,
    AAT2870_CURRENT_8_10,
    AAT2870_CURRENT_9_00,
    AAT2870_CURRENT_9_90,
    AAT2870_CURRENT_10_8,
    AAT2870_CURRENT_11_7,
    AAT2870_CURRENT_12_6,
    AAT2870_CURRENT_13_5,
    AAT2870_CURRENT_14_4,
    AAT2870_CURRENT_15_3,
    AAT2870_CURRENT_16_2,
    AAT2870_CURRENT_17_1,
    AAT2870_CURRENT_18_0,
    AAT2870_CURRENT_18_9,
    AAT2870_CURRENT_19_8,
    AAT2870_CURRENT_20_7,
    AAT2870_CURRENT_21_6,
    AAT2870_CURRENT_22_5,
    AAT2870_CURRENT_23_4,
    AAT2870_CURRENT_24_3,
    AAT2870_CURRENT_25_2,
    AAT2870_CURRENT_26_1,
    AAT2870_CURRENT_27_0,
    AAT2870_CURRENT_27_9
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_register {
    pub readable: bool,
    pub writeable: bool,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_data {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub io_lock: mutex,
    pub /: *mut *mut *mut aat2870_register reg_cache; / register cache,
    pub /: *mut *mut int en_pin; / enable GPIO pin (if < 0, ignore this value),
    pub is_enable: bool,
// init and uninit for platform specified
    pub aat2870): *mut *mut int (init)(struct aat2870_data,
    pub aat2870): *mut *mut void (uninit)(struct aat2870_data,
// i2c io funcntions
    pub val): *mut *mut *mut int (read)(struct aat2870_data aat2870, u8 addr, u8,
    pub val): *mut *mut *mut int (write)(struct aat2870_data aat2870, u8 addr, u8,
    pub val): *mut *mut *mut int (update)(struct aat2870_data aat2870, u8 addr, u8 mask, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_subdev_info {
    pub id: c_int,
    pub name: *const c_char,
    pub platform_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_platform_data {
    pub /: *mut *mut int en_pin; / enable GPIO pin (if < 0, ignore this value),
    pub subdevs: *mut aat2870_subdev_info,
    pub num_subdevs: c_int,
// init and uninit for platform specified
    pub aat2870): *mut *mut int (init)(struct aat2870_data,
    pub aat2870): *mut *mut void (uninit)(struct aat2870_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_bl_platform_data {
// backlight channels, default is AAT2870_BL_CH_ALL
    pub channels: c_int,
// backlight current magnitude, default is AAT2870_CURRENT_27_9
    pub max_current: c_int,
// maximum brightness, default is 255
    pub max_brightness: c_int,
}
