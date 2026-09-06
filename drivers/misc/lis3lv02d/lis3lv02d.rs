//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/lis3lv02d/lis3lv02d.h
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
// lis3lv02d.h - ST LIS3LV02DL accelerometer driver
//
// Copyright (C) 2007-2008 Yan Burman
// Copyright (C) 2008-2009 Eric Piel
//

//
// This driver tries to support the "digital" accelerometer chips from
// STMicroelectronics such as LIS3LV02DL, LIS302DL, LIS3L02DQ, LIS331DL,
// LIS331DLH, LIS35DE, or LIS202DL. They are very similar in terms of
// programming, with almost the same registers. In addition to differing
// on physical properties, they differ on the number of axes (2/3),
// precision (8/12 bits), and special features (freefall detection,
// click...). Unfortunately, not all the differences can be probed via
// a register. They can be connected either via I²C or SPI.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3_reg {
    WHO_AM_I	= 0x0F,
    OFFSET_X	= 0x16,
    OFFSET_Y	= 0x17,
    OFFSET_Z	= 0x18,
    GAIN_X		= 0x19,
    GAIN_Y		= 0x1A,
    GAIN_Z		= 0x1B,
    CTRL_REG1	= 0x20,
    CTRL_REG2	= 0x21,
    CTRL_REG3	= 0x22,
    CTRL_REG4	= 0x23,
    HP_FILTER_RESET	= 0x23,
    STATUS_REG	= 0x27,
    OUTX_L		= 0x28,
    OUTX_H		= 0x29,
    OUTX		= 0x29,
    OUTY_L		= 0x2A,
    OUTY_H		= 0x2B,
    OUTY		= 0x2B,
    OUTZ_L		= 0x2C,
    OUTZ_H		= 0x2D,
    OUTZ		= 0x2D,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis302d_reg {
    FF_WU_CFG_1	= 0x30,
    FF_WU_SRC_1	= 0x31,
    FF_WU_THS_1	= 0x32,
    FF_WU_DURATION_1 = 0x33,
    FF_WU_CFG_2	= 0x34,
    FF_WU_SRC_2	= 0x35,
    FF_WU_THS_2	= 0x36,
    FF_WU_DURATION_2 = 0x37,
    CLICK_CFG	= 0x38,
    CLICK_SRC	= 0x39,
    CLICK_THSY_X	= 0x3B,
    CLICK_THSZ	= 0x3C,
    CLICK_TIMELIMIT	= 0x3D,
    CLICK_LATENCY	= 0x3E,
    CLICK_WINDOW	= 0x3F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_reg {
    FF_WU_CFG	= 0x30,
    FF_WU_SRC	= 0x31,
    FF_WU_ACK	= 0x32,
    FF_WU_THS_L	= 0x34,
    FF_WU_THS_H	= 0x35,
    FF_WU_DURATION	= 0x36,
    DD_CFG		= 0x38,
    DD_SRC		= 0x39,
    DD_ACK		= 0x3A,
    DD_THSI_L	= 0x3C,
    DD_THSI_H	= 0x3D,
    DD_THSE_L	= 0x3E,
    DD_THSE_H	= 0x3F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3_who_am_i {
    WAI_3DLH	= 0x32,	/* 16 bits: LIS331DLH */
    WAI_3DC		= 0x33,	/* 8 bits: LIS3DC, HP3DC */
    WAI_12B		= 0x3A, /* 12 bits: LIS3LV02D[LQ]... */
    WAI_8B		= 0x3B, /* 8 bits: LIS[23]02D[LQ]... */
    WAI_6B		= 0x52, /* 6 bits: LIS331DLF - not supported */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3_type {
    LIS3LV02D,
    LIS3DC,
    HP3DC,
    LIS2302D,
    LIS331DLF,
    LIS331DLH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl1_12b {
    CTRL1_Xen	= 0x01,
    CTRL1_Yen	= 0x02,
    CTRL1_Zen	= 0x04,
    CTRL1_ST	= 0x08,
    CTRL1_DF0	= 0x10,
    CTRL1_DF1	= 0x20,
    CTRL1_PD0	= 0x40,
    CTRL1_PD1	= 0x80,
}

// Delta to ctrl1_12b version
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl1_8b {
    CTRL1_STM	= 0x08,
    CTRL1_STP	= 0x10,
    CTRL1_FS	= 0x20,
    CTRL1_PD	= 0x40,
    CTRL1_DR	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl1_3dc {
    CTRL1_ODR0	= 0x10,
    CTRL1_ODR1	= 0x20,
    CTRL1_ODR2	= 0x40,
    CTRL1_ODR3	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis331dlh_ctrl1 {
    CTRL1_DR0	= 0x08,
    CTRL1_DR1	= 0x10,
    CTRL1_PM0	= 0x20,
    CTRL1_PM1	= 0x40,
    CTRL1_PM2	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis331dlh_ctrl2 {
    CTRL2_HPEN1	= 0x04,
    CTRL2_HPEN2	= 0x08,
    CTRL2_FDS_3DLH	= 0x10,
    CTRL2_BOOT_3DLH	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis331dlh_ctrl4 {
    CTRL4_STSIGN	= 0x08,
    CTRL4_BLE	= 0x40,
    CTRL4_BDU	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl2 {
    CTRL2_DAS	= 0x01,
    CTRL2_SIM	= 0x02,
    CTRL2_DRDY	= 0x04,
    CTRL2_IEN	= 0x08,
    CTRL2_BOOT	= 0x10,
    CTRL2_BLE	= 0x20,
    CTRL2_BDU	= 0x40, /* Block Data Update */
    CTRL2_FS	= 0x80, /* Full Scale selection */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl4_3dc {
    CTRL4_SIM	= 0x01,
    CTRL4_ST0	= 0x02,
    CTRL4_ST1	= 0x04,
    CTRL4_FS0	= 0x10,
    CTRL4_FS1	= 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis302d_ctrl2 {
    HP_FF_WU2	= 0x08,
    HP_FF_WU1	= 0x04,
    CTRL2_BOOT_8B   = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ctrl3 {
    CTRL3_CFS0	= 0x01,
    CTRL3_CFS1	= 0x02,
    CTRL3_FDS	= 0x10,
    CTRL3_HPFF	= 0x20,
    CTRL3_HPDD	= 0x40,
    CTRL3_ECK	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_status_reg {
    STATUS_XDA	= 0x01,
    STATUS_YDA	= 0x02,
    STATUS_ZDA	= 0x04,
    STATUS_XYZDA	= 0x08,
    STATUS_XOR	= 0x10,
    STATUS_YOR	= 0x20,
    STATUS_ZOR	= 0x40,
    STATUS_XYZOR	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ff_wu_cfg {
    FF_WU_CFG_XLIE	= 0x01,
    FF_WU_CFG_XHIE	= 0x02,
    FF_WU_CFG_YLIE	= 0x04,
    FF_WU_CFG_YHIE	= 0x08,
    FF_WU_CFG_ZLIE	= 0x10,
    FF_WU_CFG_ZHIE	= 0x20,
    FF_WU_CFG_LIR	= 0x40,
    FF_WU_CFG_AOI	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_ff_wu_src {
    FF_WU_SRC_XL	= 0x01,
    FF_WU_SRC_XH	= 0x02,
    FF_WU_SRC_YL	= 0x04,
    FF_WU_SRC_YH	= 0x08,
    FF_WU_SRC_ZL	= 0x10,
    FF_WU_SRC_ZH	= 0x20,
    FF_WU_SRC_IA	= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_dd_cfg {
    DD_CFG_XLIE	= 0x01,
    DD_CFG_XHIE	= 0x02,
    DD_CFG_YLIE	= 0x04,
    DD_CFG_YHIE	= 0x08,
    DD_CFG_ZLIE	= 0x10,
    DD_CFG_ZHIE	= 0x20,
    DD_CFG_LIR	= 0x40,
    DD_CFG_IEND	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_dd_src {
    DD_SRC_XL	= 0x01,
    DD_SRC_XH	= 0x02,
    DD_SRC_YL	= 0x04,
    DD_SRC_YH	= 0x08,
    DD_SRC_ZL	= 0x10,
    DD_SRC_ZH	= 0x20,
    DD_SRC_IA	= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_click_src_8b {
    CLICK_SINGLE_X	= 0x01,
    CLICK_DOUBLE_X	= 0x02,
    CLICK_SINGLE_Y	= 0x04,
    CLICK_DOUBLE_Y	= 0x08,
    CLICK_SINGLE_Z	= 0x10,
    CLICK_DOUBLE_Z	= 0x20,
    CLICK_IA	= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lis3lv02d_reg_state {
    LIS3_REG_OFF	= 0x00,
    LIS3_REG_ON	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union axis_conversion {
    pub z: int x, y,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lis3lv02d {
    pub /: *mut *mut *mut void bus_priv; / used by the bus layer only,
    pub /: *mut *mut *mut device pm_dev; / for pm_runtime purposes,
    pub lis3): *mut *mut int (init) (struct lis3lv02d,
    pub val): *mut *mut *mut int (write) (struct lis3lv02d lis3, int reg, u8,
    pub ret): *mut *mut *mut int (read) (struct lis3lv02d lis3, int reg, u8,
    pub ret): *mut *mut *mut int (blkread) (struct lis3lv02d lis3, int reg, int len, u8,
    pub state): *mut *mut *mut int (reg_ctrl) (struct lis3lv02d lis3, bool,
    pub /: *mut *mut *mut int odrs; / Supported output data rates,
    pub /: *mut *mut *mut u8 regs; / Regs to store / restore,
    pub regs_size: c_int,
    pub reg_cache: *mut u8,
    pub regs_stored: bool,
    pub /: *mut *mut u8 odr_mask; / ODR bit mask,
    pub /: *mut *mut u8 whoami; / indicates measurement precision,
    pub reg): *mut *mut *mut s16 (read_data) (struct lis3lv02d lis3, int,
    pub mdps_max_val: c_int,
    pub pwron_delay: c_int,
    pub /*: *mut int scale;,
// relationship between 1 LBS and mG
// (1/1000th of earth gravity)
//
    pub /: *mut *mut *mut input_dev idev; / input device,
    pub /: *mut *mut *mut faux_device fdev; / faux device,
    pub regulators: [regulator_bulk_data; 2],
    pub /: *mut *mut atomic_t count; / interrupt count after last read,
    pub /: *mut *mut axis_conversion ac; / hw -> logical axis,
    pub mapped_btns: [c_int; 3],
    pub /: *mut *mut u32 irq; / IRQ number,
    pub /: *mut *mut *mut fasync_async_queue; / queue for the misc device,
    pub /: *mut *mut wait_queue_head_t misc_wait; / Wait queue for the misc device,
    pub /: *mut *mut unsigned long misc_opened; / bit0: whether the device is open,
    pub miscdev: miscdevice,
    pub data_ready_count: [c_int; 2],
    pub wake_thread: core::sync::atomic::AtomicI32,
    pub irq_cfg: c_uchar,
    pub shift_adj: c_uint,
    pub /: *mut *mut *mut lis3lv02d_platform_data pdata; / for passing board config,
    pub /: *mut *mut mutex mutex; / Serialize poll and selftest,

    pub of_node: *mut device_node,

}

extern "C" {
    pub fn lis3lv02d_init_device(lis3: *mut lis3lv02d) -> c_int;
}
extern "C" {
    pub fn lis3lv02d_joystick_enable(lis3: *mut lis3lv02d) -> c_int;
}
extern "C" {
    pub fn lis3lv02d_joystick_disable(lis3: *mut lis3lv02d);
}
extern "C" {
    pub fn lis3lv02d_poweroff(lis3: *mut lis3lv02d);
}
extern "C" {
    pub fn lis3lv02d_poweron(lis3: *mut lis3lv02d) -> c_int;
}
extern "C" {
    pub fn lis3lv02d_remove_fs(lis3: *mut lis3lv02d);
}
extern "C" {
    pub fn lis3lv02d_init_dt(lis3: *mut lis3lv02d) -> c_int;
}
