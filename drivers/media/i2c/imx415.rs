//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/imx415.c
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
// Driver for the Sony IMX415 CMOS Image Sensor.
//
// Copyright (C) 2023 WolfVision GmbH.
//

pub const IMX415_PIXEL_ARRAY_TOP: c_int = 0;
pub const IMX415_PIXEL_ARRAY_LEFT: c_int = 0;
pub const IMX415_PIXEL_ARRAY_WIDTH: c_int = 3864;
pub const IMX415_PIXEL_ARRAY_HEIGHT: c_int = 2192;
pub const IMX415_PIXEL_ARRAY_VBLANK: c_int = 58;
pub const IMX415_EXPOSURE_OFFSET: c_int = 8;
pub const IMX415_PIXEL_RATE_74_25MHZ: c_int = 891000000;
pub const IMX415_PIXEL_RATE_72MHZ: c_int = 864000000;
pub const IMX415_NUM_CLK_PARAM_REGS: c_int = 11;

pub const IMX415_VMAX_MAX: c_uint = 0xfffff;

pub const IMX415_HMAX_MAX: c_uint = 0xffff;
pub const IMX415_HMAX_MULTIPLIER: c_int = 12;

pub const IMX415_AGAIN_MIN: c_int = 0;
pub const IMX415_AGAIN_MAX: c_int = 100;
pub const IMX415_AGAIN_STEP: c_int = 1;

pub const IMX415_BLKLEVEL_DEFAULT: c_int = 50;

pub const IMX415_SENSOR_INFO_MASK: c_uint = 0xfff;
pub const IMX415_CHIP_ID: c_uint = 0x514;

pub const IMX415_LANEMODE_2: c_int = 1;
pub const IMX415_LANEMODE_4: c_int = 3;

    static const char *const imx415_supply_names[] = {
    "dvdd",
    "ovdd",
    "avdd",
    };
//
// The IMX415 data sheet uses lane rates but v4l2 uses link frequency to
// describe MIPI CSI-2 speed. This driver uses lane rates wherever possible
// and converts them to link frequencies by a factor of two when needed.
//
    static const s64 link_freq_menu_items[] = {
    594000000 / 2,	720000000 / 2,	891000000 / 2,
    1440000000 / 2, 1485000000 / 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx415_clk_params {
    pub lane_rate: u64,
    pub inck: u64,
    pub regs: [cci_reg_sequence; IMX415_NUM_CLK_PARAM_REGS],
}

// INCK Settings - includes all lane rate and INCK dependent registers
    static const struct imx415_clk_params imx415_clk_params[] = {
    {
    .lane_rate = 594000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x7 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x084 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 594000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x7 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x080 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0984 },
    },
    {
    .lane_rate = 594000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x7 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x080 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    {
    .lane_rate = 720000000UL,
    .inck = 24000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x054 },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x03B },
    .regs[2] = { IMX415_SYS_MODE, 0x9 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0B4 },
    .regs[6] = { IMX415_INCKSEL4, 0x0FC },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0600 },
    },
    {
    .lane_rate = 720000000UL,
    .inck = 72000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0F8 },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B0 },
    .regs[2] = { IMX415_SYS_MODE, 0x9 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0A0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1200 },
    },
    {
    .lane_rate = 891000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x5 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C6 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 891000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x5 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0948 },
    },
    {
    .lane_rate = 891000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x5 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x0 },
    .regs[9] = { IMX415_INCKSEL7, 0x1 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    {
    .lane_rate = 1440000000UL,
    .inck = 24000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x054 },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x03B },
    .regs[2] = { IMX415_SYS_MODE, 0x8 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0B4 },
    .regs[6] = { IMX415_INCKSEL4, 0x0FC },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0600 },
    },
    {
    .lane_rate = 1440000000UL,
    .inck = 72000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0F8 },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B0 },
    .regs[2] = { IMX415_SYS_MODE, 0x8 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0A0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1200 },
    },
    {
    .lane_rate = 1485000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x8 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0A5 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 1485000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x8 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x0A0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0948 },
    },
    {
    .lane_rate = 1485000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x8 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0A0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    {
    .lane_rate = 1782000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x4 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C6 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 1782000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x4 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0948 },
    },
    {
    .lane_rate = 1782000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x4 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0C0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    {
    .lane_rate = 2079000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x2 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x0E7 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 2079000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x2 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x0E0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0948 },
    },
    {
    .lane_rate = 2079000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x2 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x0E0 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    {
    .lane_rate = 2376000000UL,
    .inck = 27000000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x05D },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x042 },
    .regs[2] = { IMX415_SYS_MODE, 0x0 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x23 },
    .regs[5] = { IMX415_INCKSEL3, 0x108 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E7 },
    .regs[7] = { IMX415_INCKSEL5, 0x23 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x06C0 },
    },
    {
    .lane_rate = 2376000000UL,
    .inck = 37125000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x07F },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x05B },
    .regs[2] = { IMX415_SYS_MODE, 0x0 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x24 },
    .regs[5] = { IMX415_INCKSEL3, 0x100 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x24 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x0948 },
    },
    {
    .lane_rate = 2376000000UL,
    .inck = 74250000,
    .regs[0] = { IMX415_BCWAIT_TIME, 0x0FF },
    .regs[1] = { IMX415_CPWAIT_TIME, 0x0B6 },
    .regs[2] = { IMX415_SYS_MODE, 0x0 },
    .regs[3] = { IMX415_INCKSEL1, 0x00 },
    .regs[4] = { IMX415_INCKSEL2, 0x28 },
    .regs[5] = { IMX415_INCKSEL3, 0x100 },
    .regs[6] = { IMX415_INCKSEL4, 0x0E0 },
    .regs[7] = { IMX415_INCKSEL5, 0x28 },
    .regs[8] = { IMX415_INCKSEL6, 0x1 },
    .regs[9] = { IMX415_INCKSEL7, 0x0 },
    .regs[10] = { IMX415_TXCLKESC_FREQ, 0x1290 },
    },
    };
// 720 Mbps CSI configuration
    static const struct cci_reg_sequence imx415_linkrate_720mbps[] = {
    { IMX415_TCLKPOST, 0x006F },
    { IMX415_TCLKPREPARE, 0x002F },
    { IMX415_TCLKTRAIL, 0x002F },
    { IMX415_TCLKZERO, 0x00BF },
    { IMX415_THSPREPARE, 0x002F },
    { IMX415_THSZERO, 0x0057 },
    { IMX415_THSTRAIL, 0x002F },
    { IMX415_THSEXIT, 0x004F },
    { IMX415_TLPX, 0x0027 },
    };
// 1440 Mbps CSI configuration
    static const struct cci_reg_sequence imx415_linkrate_1440mbps[] = {
    { IMX415_TCLKPOST, 0x009F },
    { IMX415_TCLKPREPARE, 0x0057 },
    { IMX415_TCLKTRAIL, 0x0057 },
    { IMX415_TCLKZERO, 0x0187 },
    { IMX415_THSPREPARE, 0x005F },
    { IMX415_THSZERO, 0x00A7 },
    { IMX415_THSTRAIL, 0x005F },
    { IMX415_THSEXIT, 0x0097 },
    { IMX415_TLPX, 0x004F },
    };
// 891 Mbps CSI configuration
    static const struct cci_reg_sequence imx415_linkrate_891mbps[] = {
    { IMX415_TCLKPOST, 0x007F },
    { IMX415_TCLKPREPARE, 0x0037 },
    { IMX415_TCLKTRAIL, 0x0037 },
    { IMX415_TCLKZERO, 0x00F7 },
    { IMX415_THSPREPARE, 0x003F },
    { IMX415_THSZERO, 0x006F },
    { IMX415_THSTRAIL, 0x003F },
    { IMX415_THSEXIT, 0x005F },
    { IMX415_TLPX, 0x002F },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx415_mode_reg_list {
    pub num_of_regs: u32,
    pub regs: *const cci_reg_sequence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx415_mode {
    pub lane_rate: u64,
    pub hmax_min: [u32; 2],
    pub reg_list: imx415_mode_reg_list,
}

// mode configs
    static const struct imx415_mode supported_modes[] = {
    {
    .lane_rate = 720000000,
    .hmax_min = { 2032, 1066 },
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(imx415_linkrate_720mbps),
    .regs = imx415_linkrate_720mbps,
    },
    },
    {
    .lane_rate = 1440000000,
    .hmax_min = { 1066, 533 },
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(imx415_linkrate_1440mbps),
    .regs = imx415_linkrate_1440mbps,
    },
    },
    {
    .lane_rate = 891000000,
    .hmax_min = { 2200, 1100 },
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(imx415_linkrate_891mbps),
    .regs = imx415_linkrate_891mbps,
    },
    },
    };
    static const char *const imx415_test_pattern_menu[] = {
    "disabled",
    "solid black",
    "solid white",
    "solid dark gray",
    "solid light gray",
    "stripes light/dark grey",
    "stripes dark/light grey",
    "stripes black/dark grey",
    "stripes dark grey/black",
    "stripes black/white",
    "stripes white/black",
    "horizontal color bar",
    "vertical color bar",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx415 {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub pixel_rate: c_ulong,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(imx415_supply_names)],
    pub reset: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub clk_params: *const imx415_clk_params,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub ctrls: v4l2_ctrl_handler,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub cur_mode: c_uint,
    pub num_data_lanes: c_uint,
}

//
// This table includes fixed register settings and a bunch of undocumented
// registers that have to be set to another value than default.
//
    static const struct cci_reg_sequence imx415_init_table[] = {
// use all-pixel readout mode, no flip
    { IMX415_WINMODE, 0x00 },
    { IMX415_ADDMODE, 0x00 },
    { IMX415_REVERSE, 0x00 },
// use RAW 10-bit mode
    { IMX415_ADBIT, 0x00 },
    { IMX415_MDBIT, 0x00 },
// output VSYNC on XVS and low on XHS
    { IMX415_OUTSEL, 0x22 },
    { IMX415_DRV, 0x00 },
// SONY magic registers
    { CCI_REG8(0x32D4), 0x21 },
    { CCI_REG8(0x32EC), 0xA1 },
    { CCI_REG8(0x3452), 0x7F },
    { CCI_REG8(0x3453), 0x03 },
    { CCI_REG8(0x358A), 0x04 },
    { CCI_REG8(0x35A1), 0x02 },
    { CCI_REG8(0x36BC), 0x0C },
    { CCI_REG8(0x36CC), 0x53 },
    { CCI_REG8(0x36CD), 0x00 },
    { CCI_REG8(0x36CE), 0x3C },
    { CCI_REG8(0x36D0), 0x8C },
    { CCI_REG8(0x36D1), 0x00 },
    { CCI_REG8(0x36D2), 0x71 },
    { CCI_REG8(0x36D4), 0x3C },
    { CCI_REG8(0x36D6), 0x53 },
    { CCI_REG8(0x36D7), 0x00 },
    { CCI_REG8(0x36D8), 0x71 },
    { CCI_REG8(0x36DA), 0x8C },
    { CCI_REG8(0x36DB), 0x00 },
    { CCI_REG8(0x3724), 0x02 },
    { CCI_REG8(0x3726), 0x02 },
    { CCI_REG8(0x3732), 0x02 },
    { CCI_REG8(0x3734), 0x03 },
    { CCI_REG8(0x3736), 0x03 },
    { CCI_REG8(0x3742), 0x03 },
    { CCI_REG8(0x3862), 0xE0 },
    { CCI_REG8(0x38CC), 0x30 },
    { CCI_REG8(0x38CD), 0x2F },
    { CCI_REG8(0x395C), 0x0C },
    { CCI_REG8(0x3A42), 0xD1 },
    { CCI_REG8(0x3A4C), 0x77 },
    { CCI_REG8(0x3AE0), 0x02 },
    { CCI_REG8(0x3AEC), 0x0C },
    { CCI_REG8(0x3B00), 0x2E },
    { CCI_REG8(0x3B06), 0x29 },
    { CCI_REG8(0x3B98), 0x25 },
    { CCI_REG8(0x3B99), 0x21 },
    { CCI_REG8(0x3B9B), 0x13 },
    { CCI_REG8(0x3B9C), 0x13 },
    { CCI_REG8(0x3B9D), 0x13 },
    { CCI_REG8(0x3B9E), 0x13 },
    { CCI_REG8(0x3BA1), 0x00 },
    { CCI_REG8(0x3BA2), 0x06 },
    { CCI_REG8(0x3BA3), 0x0B },
    { CCI_REG8(0x3BA4), 0x10 },
    { CCI_REG8(0x3BA5), 0x14 },
    { CCI_REG8(0x3BA6), 0x18 },
    { CCI_REG8(0x3BA7), 0x1A },
    { CCI_REG8(0x3BA8), 0x1A },
    { CCI_REG8(0x3BA9), 0x1A },
    { CCI_REG8(0x3BAC), 0xED },
    { CCI_REG8(0x3BAD), 0x01 },
    { CCI_REG8(0x3BAE), 0xF6 },
    { CCI_REG8(0x3BAF), 0x02 },
    { CCI_REG8(0x3BB0), 0xA2 },
    { CCI_REG8(0x3BB1), 0x03 },
    { CCI_REG8(0x3BB2), 0xE0 },
    { CCI_REG8(0x3BB3), 0x03 },
    { CCI_REG8(0x3BB4), 0xE0 },
    { CCI_REG8(0x3BB5), 0x03 },
    { CCI_REG8(0x3BB6), 0xE0 },
    { CCI_REG8(0x3BB7), 0x03 },
    { CCI_REG8(0x3BB8), 0xE0 },
    { CCI_REG8(0x3BBA), 0xE0 },
    { CCI_REG8(0x3BBC), 0xDA },
    { CCI_REG8(0x3BBE), 0x88 },
    { CCI_REG8(0x3BC0), 0x44 },
    { CCI_REG8(0x3BC2), 0x7B },
    { CCI_REG8(0x3BC4), 0xA2 },
    { CCI_REG8(0x3BC8), 0xBD },
    { CCI_REG8(0x3BCA), 0xBD },
    };
    static inline struct imx415 *to_imx415(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct imx415, subdev);
    }
#[no_mangle]
unsafe extern "C" fn imx415_set_testpattern(sensor: *mut imx415, val: c_int) -> c_int {
    static int imx415_set_testpattern(struct imx415 *sensor, int val)
    {
    let mut ret: c_int = 0;
    if (val) {
    cci_write(sensor.regmap, IMX415_BLKLEVEL, 0x00, &ret);
    cci_write(sensor.regmap, IMX415_TPG_EN_DUOUT, 0x01, &ret);
    cci_write(sensor.regmap, IMX415_TPG_PATSEL_DUOUT,
    val - 1, &ret);
    cci_write(sensor.regmap, IMX415_TPG_COLORWIDTH, 0x01, &ret);
    cci_write(sensor.regmap, IMX415_TESTCLKEN_MIPI, 0x20, &ret);
    cci_write(sensor.regmap, IMX415_DIG_CLP_MODE, 0x00, &ret);
    cci_write(sensor.regmap, IMX415_WRJ_OPEN, 0x00, &ret);
    } else {
    cci_write(sensor.regmap, IMX415_BLKLEVEL,
    IMX415_BLKLEVEL_DEFAULT, &ret);
    cci_write(sensor.regmap, IMX415_TPG_EN_DUOUT, 0x00, &ret);
    cci_write(sensor.regmap, IMX415_TESTCLKEN_MIPI, 0x00, &ret);
    cci_write(sensor.regmap, IMX415_DIG_CLP_MODE, 0x01, &ret);
    cci_write(sensor.regmap, IMX415_WRJ_OPEN, 0x01, &ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int imx415_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct imx415 *sensor = container_of(ctrl.handler, struct imx415,
    ctrls);
    const struct v4l2_mbus_framefmt *format;
    struct v4l2_subdev_state *state;
    u32 exposure_max;
    unsigned int vmax;
    unsigned int flip;
    int ret;
    state = v4l2_subdev_get_locked_active_state(&sensor.subdev);
    format = v4l2_subdev_state_get_format(state, 0);
    if (ctrl.id == V4L2_CID_VBLANK) {
    exposure_max = format.height + ctrl.val -
    IMX415_EXPOSURE_OFFSET;
    __v4l2_ctrl_modify_range(sensor.exposure,
    sensor.exposure.minimum,
    exposure_max, sensor.exposure.step,
    sensor.exposure.default_value);
    }
    if (!pm_runtime_get_if_in_use(sensor.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_VBLANK:
    ret = cci_write(sensor.regmap, IMX415_VMAX,
    format.height + ctrl.val, core::ptr::null_mut());
    if (ret)
    break;
//
// Exposure is set based on VMAX which has just changed, so
// program exposure register as well
//
    ctrl = sensor.exposure;
    fallthrough;
    case V4L2_CID_EXPOSURE:
// clamp the exposure value to VMAX.
    vmax = format.height + sensor.vblank.cur.val;
    ctrl.val = min_t(int, ctrl.val, vmax);
    ret = cci_write(sensor.regmap, IMX415_SHR0,
    vmax - ctrl.val, core::ptr::null_mut());
    break;
    case V4L2_CID_ANALOGUE_GAIN:
// analogue gain in 0.3 dB step size
    ret = cci_write(sensor.regmap, IMX415_GAIN_PCG_0,
    ctrl.val, core::ptr::null_mut());
    break;
    case V4L2_CID_HFLIP:
    case V4L2_CID_VFLIP:
    flip = (sensor.hflip.val << IMX415_HREVERSE_SHIFT) |
    (sensor.vflip.val << IMX415_VREVERSE_SHIFT);
    ret = cci_write(sensor.regmap, IMX415_REVERSE, flip, core::ptr::null_mut());
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = imx415_set_testpattern(sensor, ctrl.val);
    break;
    case V4L2_CID_HBLANK:
    ret = cci_write(sensor.regmap, IMX415_HMAX,
    (format.width + ctrl.val) /
    IMX415_HMAX_MULTIPLIER,
    core::ptr::null_mut());
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(sensor.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops imx415_ctrl_ops = {
    .s_ctrl = imx415_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn imx415_ctrls_init(sensor: *mut imx415) -> c_int {
    static int imx415_ctrls_init(struct imx415 *sensor)
    {
    struct v4l2_fwnode_device_properties props;
    struct v4l2_ctrl *ctrl;
    const struct imx415_mode *cur_mode = &supported_modes[sensor.cur_mode];
    let mut lane_rate: u64 = cur_mode.lane_rate;
    u32 exposure_max = IMX415_PIXEL_ARRAY_HEIGHT +
    IMX415_PIXEL_ARRAY_VBLANK -
    IMX415_EXPOSURE_OFFSET;
    u32 hblank_min, hblank_max;
    unsigned int i;
    int ret;
    ret = v4l2_fwnode_device_parse(sensor.dev, &props);
    if (ret < 0)
    return ret;
    v4l2_ctrl_handler_init(&sensor.ctrls, 10);
    for (i = 0; i < ARRAY_SIZE(link_freq_menu_items); ++i) {
    if (lane_rate == link_freq_menu_items[i] * 2)
    break;
    }
    if (i == ARRAY_SIZE(link_freq_menu_items)) {
    return dev_err_probe(sensor.dev, -EINVAL,
    "lane rate %llu not supported\n",
    lane_rate);
    }
    ctrl = v4l2_ctrl_new_int_menu(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(link_freq_menu_items) - 1, i,
    link_freq_menu_items);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    sensor.exposure = v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_EXPOSURE, 4,
    exposure_max, 1, exposure_max);
    v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_ANALOGUE_GAIN, IMX415_AGAIN_MIN,
    IMX415_AGAIN_MAX, IMX415_AGAIN_STEP,
    IMX415_AGAIN_MIN);
    hblank_min = (cur_mode.hmax_min[sensor.num_data_lanes == 2 ? 0 : 1] *
    IMX415_HMAX_MULTIPLIER) - IMX415_PIXEL_ARRAY_WIDTH;
    hblank_max = (IMX415_HMAX_MAX * IMX415_HMAX_MULTIPLIER) -
    IMX415_PIXEL_ARRAY_WIDTH;
    ctrl = v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_HBLANK, hblank_min,
    hblank_max, IMX415_HMAX_MULTIPLIER,
    hblank_min);
    sensor.vblank = v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_VBLANK,
    IMX415_PIXEL_ARRAY_VBLANK,
    IMX415_VMAX_MAX - IMX415_PIXEL_ARRAY_HEIGHT,
    1, IMX415_PIXEL_ARRAY_VBLANK);
    v4l2_ctrl_new_std(&sensor.ctrls, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE,
    sensor.pixel_rate, sensor.pixel_rate, 1,
    sensor.pixel_rate);
    sensor.hflip = v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    sensor.vflip = v4l2_ctrl_new_std(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std_menu_items(&sensor.ctrls, &imx415_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(imx415_test_pattern_menu) - 1,
    0, 0, imx415_test_pattern_menu);
    v4l2_ctrl_new_fwnode_properties(&sensor.ctrls, &imx415_ctrl_ops,
    &props);
    if (sensor.ctrls.error) {
    dev_err_probe(sensor.dev, sensor.ctrls.error,
    "failed to add controls\n");
    v4l2_ctrl_handler_free(&sensor.ctrls);
    return sensor.ctrls.error;
    }
    sensor.subdev.ctrl_handler = &sensor.ctrls;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx415_set_mode(sensor: *mut imx415, mode: c_int) -> c_int {
    static int imx415_set_mode(struct imx415 *sensor, int mode)
    {
    let mut ret: c_int = 0;
    if (mode >= ARRAY_SIZE(supported_modes)) {
    dev_err(sensor.dev, "Mode %d not supported\n", mode);
    return -EINVAL;
    }
    cci_multi_reg_write(sensor.regmap,
    supported_modes[mode].reg_list.regs,
    supported_modes[mode].reg_list.num_of_regs,
    &ret);
    cci_multi_reg_write(sensor.regmap,
    sensor.clk_params.regs,
    IMX415_NUM_CLK_PARAM_REGS,
    &ret);
    ret = cci_write(sensor.regmap, IMX415_LANEMODE,
    sensor.num_data_lanes == 2 ? IMX415_LANEMODE_2 :
    IMX415_LANEMODE_4,
    core::ptr::null_mut());
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_setup(sensor: *mut imx415, state: *mut v4l2_subdev_state) -> c_int {
    static int imx415_setup(struct imx415 *sensor, struct v4l2_subdev_state *state)
    {
    int ret;
    ret = cci_multi_reg_write(sensor.regmap,
    imx415_init_table,
    ARRAY_SIZE(imx415_init_table),
    core::ptr::null_mut());
    if (ret)
    return ret;
    return imx415_set_mode(sensor, sensor.cur_mode);
    }
#[no_mangle]
unsafe extern "C" fn imx415_wakeup(sensor: *mut imx415) -> c_int {
    static int imx415_wakeup(struct imx415 *sensor)
    {
    int ret;
    ret = cci_write(sensor.regmap, IMX415_MODE,
    IMX415_MODE_OPERATING, core::ptr::null_mut());
    if (ret)
    return ret;
//
// According to the datasheet we have to wait at least 63 us after
// leaving standby mode. But this doesn't work even after 30 ms.
// So probably this should be 63 ms and therefore we wait for 80 ms.
//
    msleep(80);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx415_stream_on(sensor: *mut imx415) -> c_int {
    static int imx415_stream_on(struct imx415 *sensor)
    {
    int ret;
    ret = imx415_wakeup(sensor);
    return cci_write(sensor.regmap, IMX415_XMSTA,
    IMX415_XMSTA_START, &ret);
    }
#[no_mangle]
unsafe extern "C" fn imx415_stream_off(sensor: *mut imx415) -> c_int {
    static int imx415_stream_off(struct imx415 *sensor)
    {
    int ret;
    ret = cci_write(sensor.regmap, IMX415_XMSTA,
    IMX415_XMSTA_STOP, core::ptr::null_mut());
    return cci_write(sensor.regmap, IMX415_MODE,
    IMX415_MODE_STANDBY, &ret);
    }
#[no_mangle]
unsafe extern "C" fn imx415_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int imx415_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct imx415 *sensor = to_imx415(sd);
    struct v4l2_subdev_state *state;
    int ret;
    state = v4l2_subdev_lock_and_get_active_state(sd);
    if (!enable) {
    ret = imx415_stream_off(sensor);
    pm_runtime_put_autosuspend(sensor.dev);
    goto unlock;
    }
    ret = pm_runtime_resume_and_get(sensor.dev);
    if (ret < 0)
    goto unlock;
    ret = imx415_setup(sensor, state);
    if (ret)
    goto err_pm;
    ret = __v4l2_ctrl_handler_setup(&sensor.ctrls);
    if (ret < 0)
    goto err_pm;
    ret = imx415_stream_on(sensor);
    if (ret)
    goto err_pm;
    ret = 0;
    unlock:
    v4l2_subdev_unlock_state(state);
    return ret;
    err_pm:
//
// In case of error, turn the power off synchronously as the device
// likely has no other chance to recover.
//
    pm_runtime_put_sync(sensor.dev);
    goto unlock;
    }
    static int imx415_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index != 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SGBRG10_1X10;
    return 0;
    }
    static int imx415_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    const struct v4l2_mbus_framefmt *format;
    format = v4l2_subdev_state_get_format(state, fse.pad);
    if (fse.index > 0 || fse.code != format.code)
    return -EINVAL;
    fse.min_width = IMX415_PIXEL_ARRAY_WIDTH;
    fse.max_width = fse.min_width;
    fse.min_height = IMX415_PIXEL_ARRAY_HEIGHT;
    fse.max_height = fse.min_height;
    return 0;
    }
    static int imx415_set_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *fmt)
    {
    struct v4l2_mbus_framefmt *format;
    format = v4l2_subdev_state_get_format(state, fmt.pad);
    format.width = fmt.format.width;
    format.height = fmt.format.height;
    format.code = MEDIA_BUS_FMT_SGBRG10_1X10;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_RAW;
    format.ycbcr_enc = V4L2_YCBCR_ENC_DEFAULT;
    format.quantization = V4L2_QUANTIZATION_DEFAULT;
    format.xfer_func = V4L2_XFER_FUNC_NONE;
    fmt.format = *format;
    return 0;
    }
    static int imx415_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = IMX415_PIXEL_ARRAY_TOP;
    sel.r.left = IMX415_PIXEL_ARRAY_LEFT;
    sel.r.width = IMX415_PIXEL_ARRAY_WIDTH;
    sel.r.height = IMX415_PIXEL_ARRAY_HEIGHT;
    return 0;
    }
    return -EINVAL;
    }
    static int imx415_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct v4l2_subdev_format format = {
    .format = {
    .width = IMX415_PIXEL_ARRAY_WIDTH,
    .height = IMX415_PIXEL_ARRAY_HEIGHT,
    },
    };
    imx415_set_format(sd, state, &format);
    return 0;
    }
    static const struct v4l2_subdev_video_ops imx415_subdev_video_ops = {
    .s_stream = imx415_s_stream,
    };
    static const struct v4l2_subdev_pad_ops imx415_subdev_pad_ops = {
    .enum_mbus_code = imx415_enum_mbus_code,
    .enum_frame_size = imx415_enum_frame_size,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = imx415_set_format,
    .get_selection = imx415_get_selection,
    };
    static const struct v4l2_subdev_ops imx415_subdev_ops = {
    .video = &imx415_subdev_video_ops,
    .pad = &imx415_subdev_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops imx415_internal_ops = {
    .init_state = imx415_init_state,
    };
#[no_mangle]
unsafe extern "C" fn imx415_subdev_init(sensor: *mut imx415) -> c_int {
    static int imx415_subdev_init(struct imx415 *sensor)
    {
    struct i2c_client *client = to_i2c_client(sensor.dev);
    int ret;
    v4l2_i2c_subdev_init(&sensor.subdev, client, &imx415_subdev_ops);
    sensor.subdev.internal_ops = &imx415_internal_ops;
    ret = imx415_ctrls_init(sensor);
    if (ret)
    return ret;
    sensor.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    sensor.pad.flags = MEDIA_PAD_FL_SOURCE;
    sensor.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&sensor.subdev.entity, 1, &sensor.pad);
    if (ret < 0) {
    v4l2_ctrl_handler_free(&sensor.ctrls);
    return ret;
    }
    sensor.subdev.state_lock = sensor.subdev.ctrl_handler.lock;
    v4l2_subdev_init_finalize(&sensor.subdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx415_subdev_cleanup(sensor: *mut imx415) {
    static void imx415_subdev_cleanup(struct imx415 *sensor)
    {
    media_entity_cleanup(&sensor.subdev.entity);
    v4l2_ctrl_handler_free(&sensor.ctrls);
    }
#[no_mangle]
unsafe extern "C" fn imx415_power_on(sensor: *mut imx415) -> c_int {
    static int imx415_power_on(struct imx415 *sensor)
    {
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(sensor.supplies),
    sensor.supplies);
    if (ret < 0)
    return ret;
    gpiod_set_value_cansleep(sensor.reset, 0);
    udelay(1);
    ret = clk_prepare_enable(sensor.clk);
    if (ret < 0)
    goto err_reset;
//
// Data sheet states that 20 us are required before communication start,
// but this doesn't work in all cases. Use 100 us to be on the safe
// side.
//
    usleep_range(100, 200);
    return 0;
    err_reset:
    gpiod_set_value_cansleep(sensor.reset, 1);
    regulator_bulk_disable(ARRAY_SIZE(sensor.supplies), sensor.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_power_off(sensor: *mut imx415) {
    static void imx415_power_off(struct imx415 *sensor)
    {
    clk_disable_unprepare(sensor.clk);
    gpiod_set_value_cansleep(sensor.reset, 1);
    regulator_bulk_disable(ARRAY_SIZE(sensor.supplies), sensor.supplies);
    }
#[no_mangle]
unsafe extern "C" fn imx415_identify_model(sensor: *mut imx415) -> c_int {
    static int imx415_identify_model(struct imx415 *sensor)
    {
    int model, ret;
    u64 chip_id;
//
// While most registers can be read when the sensor is in standby, this
// is not the case of the sensor info register :-(
//
    ret = imx415_wakeup(sensor);
    if (ret)
    return dev_err_probe(sensor.dev, ret,
    "failed to get sensor out of standby\n");
    ret = cci_read(sensor.regmap, IMX415_SENSOR_INFO, &chip_id, core::ptr::null_mut());
    if (ret < 0) {
    dev_err_probe(sensor.dev, ret,
    "failed to read sensor information\n");
    goto done;
    }
    model = chip_id & IMX415_SENSOR_INFO_MASK;
    switch (model) {
    case IMX415_CHIP_ID:
    dev_info(sensor.dev, "Detected IMX415 image sensor\n");
    break;
    default:
    ret = dev_err_probe(sensor.dev, -ENODEV,
    "invalid device model 0x%04x\n", model);
    goto done;
    }
    ret = 0;
    done:
    cci_write(sensor.regmap, IMX415_MODE, IMX415_MODE_STANDBY, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_check_inck(inck: c_ulong, link_frequency: u64) -> c_int {
    static int imx415_check_inck(unsigned long inck, u64 link_frequency)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(imx415_clk_params); ++i) {
    if ((imx415_clk_params[i].lane_rate == link_frequency * 2) &&
    imx415_clk_params[i].inck == inck)
    break;
    }
    if (i == ARRAY_SIZE(imx415_clk_params))
    return -EINVAL;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx415_parse_hw_config(sensor: *mut imx415) -> c_int {
    static int imx415_parse_hw_config(struct imx415 *sensor)
    {
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY,
    };
    struct fwnode_handle *ep;
    u64 lane_rate;
    unsigned long inck;
    unsigned int i, j;
    int ret;
    for (i = 0; i < ARRAY_SIZE(sensor.supplies); ++i)
    sensor.supplies[i].supply = imx415_supply_names[i];
    ret = devm_regulator_bulk_get(sensor.dev, ARRAY_SIZE(sensor.supplies),
    sensor.supplies);
    if (ret)
    return dev_err_probe(sensor.dev, ret,
    "failed to get supplies\n");
    sensor.reset = devm_gpiod_get_optional(sensor.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(sensor.reset))
    return dev_err_probe(sensor.dev, PTR_ERR(sensor.reset),
    "failed to get reset GPIO\n");
    sensor.clk = devm_v4l2_sensor_clk_get(sensor.dev, core::ptr::null_mut());
    if (IS_ERR(sensor.clk))
    return dev_err_probe(sensor.dev, PTR_ERR(sensor.clk),
    "failed to get clock\n");
    ep = fwnode_graph_get_next_endpoint(dev_fwnode(sensor.dev), core::ptr::null_mut());
    if (!ep)
    return -ENXIO;
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    fwnode_handle_put(ep);
    if (ret)
    return ret;
    switch (bus_cfg.bus.mipi_csi2.num_data_lanes) {
    case 2:
    case 4:
    sensor.num_data_lanes = bus_cfg.bus.mipi_csi2.num_data_lanes;
    break;
    default:
    ret = dev_err_probe(sensor.dev, -EINVAL,
    "invalid number of CSI2 data lanes %d\n",
    bus_cfg.bus.mipi_csi2.num_data_lanes);
    goto done_endpoint_free;
    }
    if (!bus_cfg.nr_of_link_frequencies) {
    ret = dev_err_probe(sensor.dev, -EINVAL,
    "no link frequencies defined");
    goto done_endpoint_free;
    }
//
// Check if there exists a sensor mode defined for current INCK,
// number of lanes and given lane rates.
//
    inck = clk_get_rate(sensor.clk);
    for (i = 0; i < bus_cfg.nr_of_link_frequencies; ++i) {
    if (imx415_check_inck(inck, bus_cfg.link_frequencies[i])) {
    dev_dbg(sensor.dev,
    "INCK %lu Hz not supported for this link freq",
    inck);
    continue;
    }
    for (j = 0; j < ARRAY_SIZE(supported_modes); ++j) {
    if (bus_cfg.link_frequencies[i] * 2 !=
    supported_modes[j].lane_rate)
    continue;
    sensor.cur_mode = j;
    break;
    }
    if (j < ARRAY_SIZE(supported_modes))
    break;
    }
    if (i == bus_cfg.nr_of_link_frequencies) {
    ret = dev_err_probe(sensor.dev, -EINVAL,
    "no valid sensor mode defined\n");
    goto done_endpoint_free;
    }
    switch (inck) {
    case 27000000:
    case 37125000:
    case 74250000:
    sensor.pixel_rate = IMX415_PIXEL_RATE_74_25MHZ;
    break;
    case 24000000:
    case 72000000:
    sensor.pixel_rate = IMX415_PIXEL_RATE_72MHZ;
    break;
    }
    lane_rate = supported_modes[sensor.cur_mode].lane_rate;
    for (i = 0; i < ARRAY_SIZE(imx415_clk_params); ++i) {
    if (lane_rate == imx415_clk_params[i].lane_rate &&
    inck == imx415_clk_params[i].inck) {
    sensor.clk_params = &imx415_clk_params[i];
    break;
    }
    }
    if (i == ARRAY_SIZE(imx415_clk_params)) {
    ret = dev_err_probe(sensor.dev, -EINVAL,
    "Mode %d not supported\n",
    sensor.cur_mode);
    goto done_endpoint_free;
    }
    ret = 0;
    dev_dbg(sensor.dev, "clock: %lu Hz, lane_rate: %llu bps, lanes: %d\n",
    inck, lane_rate, sensor.num_data_lanes);
    done_endpoint_free:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_probe(client: *mut i2c_client) -> c_int {
    static int imx415_probe(struct i2c_client *client)
    {
    struct imx415 *sensor;
    int ret;
    sensor = devm_kzalloc(&client.dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.dev = &client.dev;
    ret = imx415_parse_hw_config(sensor);
    if (ret)
    return ret;
    sensor.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(sensor.regmap))
    return PTR_ERR(sensor.regmap);
//
// Enable power management. The driver supports runtime PM, but needs to
// work when runtime PM is disabled in the kernel. To that end, power
// the sensor on manually here, identify it, and fully initialize it.
//
    ret = imx415_power_on(sensor);
    if (ret)
    return ret;
    ret = imx415_identify_model(sensor);
    if (ret)
    goto err_power;
    ret = imx415_subdev_init(sensor);
    if (ret)
    goto err_power;
//
// Enable runtime PM. As the device has been powered manually, mark it
// as active, and increase the usage count without resuming the device.
//
    pm_runtime_set_active(sensor.dev);
    pm_runtime_get_noresume(sensor.dev);
    pm_runtime_enable(sensor.dev);
    ret = v4l2_async_register_subdev_sensor(&sensor.subdev);
    if (ret < 0)
    goto err_pm;
//
// Finally, enable autosuspend and decrease the usage count. The device
// will get suspended after the autosuspend delay, turning the power
// off.
//
    pm_runtime_set_autosuspend_delay(sensor.dev, 1000);
    pm_runtime_use_autosuspend(sensor.dev);
    pm_runtime_put_autosuspend(sensor.dev);
    return 0;
    err_pm:
    pm_runtime_disable(sensor.dev);
    pm_runtime_put_noidle(sensor.dev);
    imx415_subdev_cleanup(sensor);
    err_power:
    imx415_power_off(sensor);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx415_remove(client: *mut i2c_client) {
    static void imx415_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct imx415 *sensor = to_imx415(subdev);
    v4l2_async_unregister_subdev(subdev);
    imx415_subdev_cleanup(sensor);
//
// Disable runtime PM. In case runtime PM is disabled in the kernel,
// make sure to turn power off manually.
//
    pm_runtime_disable(sensor.dev);
    if (!pm_runtime_status_suspended(sensor.dev))
    imx415_power_off(sensor);
    pm_runtime_set_suspended(sensor.dev);
    }
#[no_mangle]
unsafe extern "C" fn imx415_runtime_resume(dev: *mut device) -> c_int {
    static int imx415_runtime_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct imx415 *sensor = to_imx415(subdev);
    return imx415_power_on(sensor);
    }
#[no_mangle]
unsafe extern "C" fn imx415_runtime_suspend(dev: *mut device) -> c_int {
    static int imx415_runtime_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct imx415 *sensor = to_imx415(subdev);
    imx415_power_off(sensor);
    return 0;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(imx415_pm_ops, imx415_runtime_suspend,
    imx415_runtime_resume, core::ptr::null_mut());
    static const struct of_device_id imx415_of_match[] = {
    { .compatible = "sony,imx415" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx415_of_match);
    static struct i2c_driver imx415_driver = {
    .probe = imx415_probe,
    .remove = imx415_remove,
    .driver = {
    .name = "imx415",
    .of_match_table = imx415_of_match,
    .pm = pm_ptr(&imx415_pm_ops),
    },
    };
    module_i2c_driver(imx415_driver);
    MODULE_DESCRIPTION("Sony IMX415 image sensor driver");
    MODULE_AUTHOR("Gerald Loacker <gerald.loacker@wolfvision.net>");
    MODULE_AUTHOR("Michael Riesch <michael.riesch@wolfvision.net>");
    MODULE_LICENSE("GPL");
