//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/auxadc.h
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
// include/linux/mfd/wm831x/auxadc.h -- Auxiliary ADC interface for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// R16429 (0x402D) - AuxADC Data
//
pub const WM831X_AUX_DATA_SRC_MASK: c_uint = 0xF000  /* AUX_DATA_SRC - [15:12] */;

pub const WM831X_AUX_DATA_MASK: c_uint = 0x0FFF  /* AUX_DATA - [11:0] */;

//
// R16430 (0x402E) - AuxADC Control
//
pub const WM831X_AUX_ENA: c_uint = 0x8000  /* AUX_ENA */;
pub const WM831X_AUX_ENA_MASK: c_uint = 0x8000  /* AUX_ENA */;

pub const WM831X_AUX_CVT_ENA: c_uint = 0x4000  /* AUX_CVT_ENA */;
pub const WM831X_AUX_CVT_ENA_MASK: c_uint = 0x4000  /* AUX_CVT_ENA */;

pub const WM831X_AUX_SLPENA: c_uint = 0x1000  /* AUX_SLPENA */;
pub const WM831X_AUX_SLPENA_MASK: c_uint = 0x1000  /* AUX_SLPENA */;

pub const WM831X_AUX_FRC_ENA: c_uint = 0x0800  /* AUX_FRC_ENA */;
pub const WM831X_AUX_FRC_ENA_MASK: c_uint = 0x0800  /* AUX_FRC_ENA */;

pub const WM831X_AUX_RATE_MASK: c_uint = 0x003F  /* AUX_RATE - [5:0] */;

//
// R16431 (0x402F) - AuxADC Source
//
pub const WM831X_AUX_CAL_SEL: c_uint = 0x8000  /* AUX_CAL_SEL */;
pub const WM831X_AUX_CAL_SEL_MASK: c_uint = 0x8000  /* AUX_CAL_SEL */;

pub const WM831X_AUX_BKUP_BATT_SEL: c_uint = 0x0400  /* AUX_BKUP_BATT_SEL */;
pub const WM831X_AUX_BKUP_BATT_SEL_MASK: c_uint = 0x0400  /* AUX_BKUP_BATT_SEL */;

pub const WM831X_AUX_WALL_SEL: c_uint = 0x0200  /* AUX_WALL_SEL */;
pub const WM831X_AUX_WALL_SEL_MASK: c_uint = 0x0200  /* AUX_WALL_SEL */;

pub const WM831X_AUX_BATT_SEL: c_uint = 0x0100  /* AUX_BATT_SEL */;
pub const WM831X_AUX_BATT_SEL_MASK: c_uint = 0x0100  /* AUX_BATT_SEL */;

pub const WM831X_AUX_USB_SEL: c_uint = 0x0080  /* AUX_USB_SEL */;
pub const WM831X_AUX_USB_SEL_MASK: c_uint = 0x0080  /* AUX_USB_SEL */;

pub const WM831X_AUX_SYSVDD_SEL: c_uint = 0x0040  /* AUX_SYSVDD_SEL */;
pub const WM831X_AUX_SYSVDD_SEL_MASK: c_uint = 0x0040  /* AUX_SYSVDD_SEL */;

pub const WM831X_AUX_BATT_TEMP_SEL: c_uint = 0x0020  /* AUX_BATT_TEMP_SEL */;
pub const WM831X_AUX_BATT_TEMP_SEL_MASK: c_uint = 0x0020  /* AUX_BATT_TEMP_SEL */;

pub const WM831X_AUX_CHIP_TEMP_SEL: c_uint = 0x0010  /* AUX_CHIP_TEMP_SEL */;
pub const WM831X_AUX_CHIP_TEMP_SEL_MASK: c_uint = 0x0010  /* AUX_CHIP_TEMP_SEL */;

pub const WM831X_AUX_AUX4_SEL: c_uint = 0x0008  /* AUX_AUX4_SEL */;
pub const WM831X_AUX_AUX4_SEL_MASK: c_uint = 0x0008  /* AUX_AUX4_SEL */;

pub const WM831X_AUX_AUX3_SEL: c_uint = 0x0004  /* AUX_AUX3_SEL */;
pub const WM831X_AUX_AUX3_SEL_MASK: c_uint = 0x0004  /* AUX_AUX3_SEL */;

pub const WM831X_AUX_AUX2_SEL: c_uint = 0x0002  /* AUX_AUX2_SEL */;
pub const WM831X_AUX_AUX2_SEL_MASK: c_uint = 0x0002  /* AUX_AUX2_SEL */;

pub const WM831X_AUX_AUX1_SEL: c_uint = 0x0001  /* AUX_AUX1_SEL */;
pub const WM831X_AUX_AUX1_SEL_MASK: c_uint = 0x0001  /* AUX_AUX1_SEL */;

//
// R16432 (0x4030) - Comparator Control
//
pub const WM831X_DCOMP4_STS: c_uint = 0x0800  /* DCOMP4_STS */;
pub const WM831X_DCOMP4_STS_MASK: c_uint = 0x0800  /* DCOMP4_STS */;

pub const WM831X_DCOMP3_STS: c_uint = 0x0400  /* DCOMP3_STS */;
pub const WM831X_DCOMP3_STS_MASK: c_uint = 0x0400  /* DCOMP3_STS */;

pub const WM831X_DCOMP2_STS: c_uint = 0x0200  /* DCOMP2_STS */;
pub const WM831X_DCOMP2_STS_MASK: c_uint = 0x0200  /* DCOMP2_STS */;

pub const WM831X_DCOMP1_STS: c_uint = 0x0100  /* DCOMP1_STS */;
pub const WM831X_DCOMP1_STS_MASK: c_uint = 0x0100  /* DCOMP1_STS */;

pub const WM831X_DCMP4_ENA: c_uint = 0x0008  /* DCMP4_ENA */;
pub const WM831X_DCMP4_ENA_MASK: c_uint = 0x0008  /* DCMP4_ENA */;

pub const WM831X_DCMP3_ENA: c_uint = 0x0004  /* DCMP3_ENA */;
pub const WM831X_DCMP3_ENA_MASK: c_uint = 0x0004  /* DCMP3_ENA */;

pub const WM831X_DCMP2_ENA: c_uint = 0x0002  /* DCMP2_ENA */;
pub const WM831X_DCMP2_ENA_MASK: c_uint = 0x0002  /* DCMP2_ENA */;

pub const WM831X_DCMP1_ENA: c_uint = 0x0001  /* DCMP1_ENA */;
pub const WM831X_DCMP1_ENA_MASK: c_uint = 0x0001  /* DCMP1_ENA */;

//
// R16433 (0x4031) - Comparator 1
//
pub const WM831X_DCMP1_SRC_MASK: c_uint = 0xE000  /* DCMP1_SRC - [15:13] */;

pub const WM831X_DCMP1_GT: c_uint = 0x1000  /* DCMP1_GT */;
pub const WM831X_DCMP1_GT_MASK: c_uint = 0x1000  /* DCMP1_GT */;

pub const WM831X_DCMP1_THR_MASK: c_uint = 0x0FFF  /* DCMP1_THR - [11:0] */;

//
// R16434 (0x4032) - Comparator 2
//
pub const WM831X_DCMP2_SRC_MASK: c_uint = 0xE000  /* DCMP2_SRC - [15:13] */;

pub const WM831X_DCMP2_GT: c_uint = 0x1000  /* DCMP2_GT */;
pub const WM831X_DCMP2_GT_MASK: c_uint = 0x1000  /* DCMP2_GT */;

pub const WM831X_DCMP2_THR_MASK: c_uint = 0x0FFF  /* DCMP2_THR - [11:0] */;

//
// R16435 (0x4033) - Comparator 3
//
pub const WM831X_DCMP3_SRC_MASK: c_uint = 0xE000  /* DCMP3_SRC - [15:13] */;

pub const WM831X_DCMP3_GT: c_uint = 0x1000  /* DCMP3_GT */;
pub const WM831X_DCMP3_GT_MASK: c_uint = 0x1000  /* DCMP3_GT */;

pub const WM831X_DCMP3_THR_MASK: c_uint = 0x0FFF  /* DCMP3_THR - [11:0] */;

//
// R16436 (0x4034) - Comparator 4
//
pub const WM831X_DCMP4_SRC_MASK: c_uint = 0xE000  /* DCMP4_SRC - [15:13] */;

pub const WM831X_DCMP4_GT: c_uint = 0x1000  /* DCMP4_GT */;
pub const WM831X_DCMP4_GT_MASK: c_uint = 0x1000  /* DCMP4_GT */;

pub const WM831X_DCMP4_THR_MASK: c_uint = 0x0FFF  /* DCMP4_THR - [11:0] */;

pub const WM831X_AUX_CAL_FACTOR: c_uint = 0xfff;
pub const WM831X_AUX_CAL_NOMINAL: c_uint = 0x222;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm831x_auxadc {
    WM831X_AUX_CAL = 15,
    WM831X_AUX_BKUP_BATT = 10,
    WM831X_AUX_WALL = 9,
    WM831X_AUX_BATT = 8,
    WM831X_AUX_USB = 7,
    WM831X_AUX_SYSVDD = 6,
    WM831X_AUX_BATT_TEMP = 5,
    WM831X_AUX_CHIP_TEMP = 4,
    WM831X_AUX_AUX4 = 3,
    WM831X_AUX_AUX3 = 2,
    WM831X_AUX_AUX2 = 1,
    WM831X_AUX_AUX1 = 0,
}

extern "C" {
    pub fn wm831x_auxadc_read(wm831x: *mut wm831x, input: wm831x_auxadc) -> c_int;
}
extern "C" {
    pub fn wm831x_auxadc_read_uv(wm831x: *mut wm831x, input: wm831x_auxadc) -> c_int;
}
