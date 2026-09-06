//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/regulator.h
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
// linux/mfd/wm831x/regulator.h -- Regulator definitons for wm831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// R16462 (0x404E) - Current Sink 1
//
pub const WM831X_CS1_ENA: c_uint = 0x8000  /* CS1_ENA */;
pub const WM831X_CS1_ENA_MASK: c_uint = 0x8000  /* CS1_ENA */;

pub const WM831X_CS1_DRIVE: c_uint = 0x4000  /* CS1_DRIVE */;
pub const WM831X_CS1_DRIVE_MASK: c_uint = 0x4000  /* CS1_DRIVE */;

pub const WM831X_CS1_SLPENA: c_uint = 0x1000  /* CS1_SLPENA */;
pub const WM831X_CS1_SLPENA_MASK: c_uint = 0x1000  /* CS1_SLPENA */;

pub const WM831X_CS1_OFF_RAMP_MASK: c_uint = 0x0C00  /* CS1_OFF_RAMP - [11:10] */;

pub const WM831X_CS1_ON_RAMP_MASK: c_uint = 0x0300  /* CS1_ON_RAMP - [9:8] */;

pub const WM831X_CS1_ISEL_MASK: c_uint = 0x003F  /* CS1_ISEL - [5:0] */;

//
// R16463 (0x404F) - Current Sink 2
//
pub const WM831X_CS2_ENA: c_uint = 0x8000  /* CS2_ENA */;
pub const WM831X_CS2_ENA_MASK: c_uint = 0x8000  /* CS2_ENA */;

pub const WM831X_CS2_DRIVE: c_uint = 0x4000  /* CS2_DRIVE */;
pub const WM831X_CS2_DRIVE_MASK: c_uint = 0x4000  /* CS2_DRIVE */;

pub const WM831X_CS2_SLPENA: c_uint = 0x1000  /* CS2_SLPENA */;
pub const WM831X_CS2_SLPENA_MASK: c_uint = 0x1000  /* CS2_SLPENA */;

pub const WM831X_CS2_OFF_RAMP_MASK: c_uint = 0x0C00  /* CS2_OFF_RAMP - [11:10] */;

pub const WM831X_CS2_ON_RAMP_MASK: c_uint = 0x0300  /* CS2_ON_RAMP - [9:8] */;

pub const WM831X_CS2_ISEL_MASK: c_uint = 0x003F  /* CS2_ISEL - [5:0] */;

//
// R16464 (0x4050) - DCDC Enable
//
pub const WM831X_EPE2_ENA: c_uint = 0x0080  /* EPE2_ENA */;
pub const WM831X_EPE2_ENA_MASK: c_uint = 0x0080  /* EPE2_ENA */;

pub const WM831X_EPE1_ENA: c_uint = 0x0040  /* EPE1_ENA */;
pub const WM831X_EPE1_ENA_MASK: c_uint = 0x0040  /* EPE1_ENA */;

pub const WM831X_DC4_ENA: c_uint = 0x0008  /* DC4_ENA */;
pub const WM831X_DC4_ENA_MASK: c_uint = 0x0008  /* DC4_ENA */;

pub const WM831X_DC3_ENA: c_uint = 0x0004  /* DC3_ENA */;
pub const WM831X_DC3_ENA_MASK: c_uint = 0x0004  /* DC3_ENA */;

pub const WM831X_DC2_ENA: c_uint = 0x0002  /* DC2_ENA */;
pub const WM831X_DC2_ENA_MASK: c_uint = 0x0002  /* DC2_ENA */;

pub const WM831X_DC1_ENA: c_uint = 0x0001  /* DC1_ENA */;
pub const WM831X_DC1_ENA_MASK: c_uint = 0x0001  /* DC1_ENA */;

//
// R16465 (0x4051) - LDO Enable
//
pub const WM831X_LDO11_ENA: c_uint = 0x0400  /* LDO11_ENA */;
pub const WM831X_LDO11_ENA_MASK: c_uint = 0x0400  /* LDO11_ENA */;

pub const WM831X_LDO10_ENA: c_uint = 0x0200  /* LDO10_ENA */;
pub const WM831X_LDO10_ENA_MASK: c_uint = 0x0200  /* LDO10_ENA */;

pub const WM831X_LDO9_ENA: c_uint = 0x0100  /* LDO9_ENA */;
pub const WM831X_LDO9_ENA_MASK: c_uint = 0x0100  /* LDO9_ENA */;

pub const WM831X_LDO8_ENA: c_uint = 0x0080  /* LDO8_ENA */;
pub const WM831X_LDO8_ENA_MASK: c_uint = 0x0080  /* LDO8_ENA */;

pub const WM831X_LDO7_ENA: c_uint = 0x0040  /* LDO7_ENA */;
pub const WM831X_LDO7_ENA_MASK: c_uint = 0x0040  /* LDO7_ENA */;

pub const WM831X_LDO6_ENA: c_uint = 0x0020  /* LDO6_ENA */;
pub const WM831X_LDO6_ENA_MASK: c_uint = 0x0020  /* LDO6_ENA */;

pub const WM831X_LDO5_ENA: c_uint = 0x0010  /* LDO5_ENA */;
pub const WM831X_LDO5_ENA_MASK: c_uint = 0x0010  /* LDO5_ENA */;

pub const WM831X_LDO4_ENA: c_uint = 0x0008  /* LDO4_ENA */;
pub const WM831X_LDO4_ENA_MASK: c_uint = 0x0008  /* LDO4_ENA */;

pub const WM831X_LDO3_ENA: c_uint = 0x0004  /* LDO3_ENA */;
pub const WM831X_LDO3_ENA_MASK: c_uint = 0x0004  /* LDO3_ENA */;

pub const WM831X_LDO2_ENA: c_uint = 0x0002  /* LDO2_ENA */;
pub const WM831X_LDO2_ENA_MASK: c_uint = 0x0002  /* LDO2_ENA */;

pub const WM831X_LDO1_ENA: c_uint = 0x0001  /* LDO1_ENA */;
pub const WM831X_LDO1_ENA_MASK: c_uint = 0x0001  /* LDO1_ENA */;

//
// R16466 (0x4052) - DCDC Status
//
pub const WM831X_EPE2_STS: c_uint = 0x0080  /* EPE2_STS */;
pub const WM831X_EPE2_STS_MASK: c_uint = 0x0080  /* EPE2_STS */;

pub const WM831X_EPE1_STS: c_uint = 0x0040  /* EPE1_STS */;
pub const WM831X_EPE1_STS_MASK: c_uint = 0x0040  /* EPE1_STS */;

pub const WM831X_DC4_STS: c_uint = 0x0008  /* DC4_STS */;
pub const WM831X_DC4_STS_MASK: c_uint = 0x0008  /* DC4_STS */;

pub const WM831X_DC3_STS: c_uint = 0x0004  /* DC3_STS */;
pub const WM831X_DC3_STS_MASK: c_uint = 0x0004  /* DC3_STS */;

pub const WM831X_DC2_STS: c_uint = 0x0002  /* DC2_STS */;
pub const WM831X_DC2_STS_MASK: c_uint = 0x0002  /* DC2_STS */;

pub const WM831X_DC1_STS: c_uint = 0x0001  /* DC1_STS */;
pub const WM831X_DC1_STS_MASK: c_uint = 0x0001  /* DC1_STS */;

//
// R16467 (0x4053) - LDO Status
//
pub const WM831X_LDO11_STS: c_uint = 0x0400  /* LDO11_STS */;
pub const WM831X_LDO11_STS_MASK: c_uint = 0x0400  /* LDO11_STS */;

pub const WM831X_LDO10_STS: c_uint = 0x0200  /* LDO10_STS */;
pub const WM831X_LDO10_STS_MASK: c_uint = 0x0200  /* LDO10_STS */;

pub const WM831X_LDO9_STS: c_uint = 0x0100  /* LDO9_STS */;
pub const WM831X_LDO9_STS_MASK: c_uint = 0x0100  /* LDO9_STS */;

pub const WM831X_LDO8_STS: c_uint = 0x0080  /* LDO8_STS */;
pub const WM831X_LDO8_STS_MASK: c_uint = 0x0080  /* LDO8_STS */;

pub const WM831X_LDO7_STS: c_uint = 0x0040  /* LDO7_STS */;
pub const WM831X_LDO7_STS_MASK: c_uint = 0x0040  /* LDO7_STS */;

pub const WM831X_LDO6_STS: c_uint = 0x0020  /* LDO6_STS */;
pub const WM831X_LDO6_STS_MASK: c_uint = 0x0020  /* LDO6_STS */;

pub const WM831X_LDO5_STS: c_uint = 0x0010  /* LDO5_STS */;
pub const WM831X_LDO5_STS_MASK: c_uint = 0x0010  /* LDO5_STS */;

pub const WM831X_LDO4_STS: c_uint = 0x0008  /* LDO4_STS */;
pub const WM831X_LDO4_STS_MASK: c_uint = 0x0008  /* LDO4_STS */;

pub const WM831X_LDO3_STS: c_uint = 0x0004  /* LDO3_STS */;
pub const WM831X_LDO3_STS_MASK: c_uint = 0x0004  /* LDO3_STS */;

pub const WM831X_LDO2_STS: c_uint = 0x0002  /* LDO2_STS */;
pub const WM831X_LDO2_STS_MASK: c_uint = 0x0002  /* LDO2_STS */;

pub const WM831X_LDO1_STS: c_uint = 0x0001  /* LDO1_STS */;
pub const WM831X_LDO1_STS_MASK: c_uint = 0x0001  /* LDO1_STS */;

//
// R16468 (0x4054) - DCDC UV Status
//
pub const WM831X_DC2_OV_STS: c_uint = 0x2000  /* DC2_OV_STS */;
pub const WM831X_DC2_OV_STS_MASK: c_uint = 0x2000  /* DC2_OV_STS */;

pub const WM831X_DC1_OV_STS: c_uint = 0x1000  /* DC1_OV_STS */;
pub const WM831X_DC1_OV_STS_MASK: c_uint = 0x1000  /* DC1_OV_STS */;

pub const WM831X_DC2_HC_STS: c_uint = 0x0200  /* DC2_HC_STS */;
pub const WM831X_DC2_HC_STS_MASK: c_uint = 0x0200  /* DC2_HC_STS */;

pub const WM831X_DC1_HC_STS: c_uint = 0x0100  /* DC1_HC_STS */;
pub const WM831X_DC1_HC_STS_MASK: c_uint = 0x0100  /* DC1_HC_STS */;

pub const WM831X_DC4_UV_STS: c_uint = 0x0008  /* DC4_UV_STS */;
pub const WM831X_DC4_UV_STS_MASK: c_uint = 0x0008  /* DC4_UV_STS */;

pub const WM831X_DC3_UV_STS: c_uint = 0x0004  /* DC3_UV_STS */;
pub const WM831X_DC3_UV_STS_MASK: c_uint = 0x0004  /* DC3_UV_STS */;

pub const WM831X_DC2_UV_STS: c_uint = 0x0002  /* DC2_UV_STS */;
pub const WM831X_DC2_UV_STS_MASK: c_uint = 0x0002  /* DC2_UV_STS */;

pub const WM831X_DC1_UV_STS: c_uint = 0x0001  /* DC1_UV_STS */;
pub const WM831X_DC1_UV_STS_MASK: c_uint = 0x0001  /* DC1_UV_STS */;

//
// R16469 (0x4055) - LDO UV Status
//
pub const WM831X_INTLDO_UV_STS: c_uint = 0x8000  /* INTLDO_UV_STS */;
pub const WM831X_INTLDO_UV_STS_MASK: c_uint = 0x8000  /* INTLDO_UV_STS */;

pub const WM831X_LDO10_UV_STS: c_uint = 0x0200  /* LDO10_UV_STS */;
pub const WM831X_LDO10_UV_STS_MASK: c_uint = 0x0200  /* LDO10_UV_STS */;

pub const WM831X_LDO9_UV_STS: c_uint = 0x0100  /* LDO9_UV_STS */;
pub const WM831X_LDO9_UV_STS_MASK: c_uint = 0x0100  /* LDO9_UV_STS */;

pub const WM831X_LDO8_UV_STS: c_uint = 0x0080  /* LDO8_UV_STS */;
pub const WM831X_LDO8_UV_STS_MASK: c_uint = 0x0080  /* LDO8_UV_STS */;

pub const WM831X_LDO7_UV_STS: c_uint = 0x0040  /* LDO7_UV_STS */;
pub const WM831X_LDO7_UV_STS_MASK: c_uint = 0x0040  /* LDO7_UV_STS */;

pub const WM831X_LDO6_UV_STS: c_uint = 0x0020  /* LDO6_UV_STS */;
pub const WM831X_LDO6_UV_STS_MASK: c_uint = 0x0020  /* LDO6_UV_STS */;

pub const WM831X_LDO5_UV_STS: c_uint = 0x0010  /* LDO5_UV_STS */;
pub const WM831X_LDO5_UV_STS_MASK: c_uint = 0x0010  /* LDO5_UV_STS */;

pub const WM831X_LDO4_UV_STS: c_uint = 0x0008  /* LDO4_UV_STS */;
pub const WM831X_LDO4_UV_STS_MASK: c_uint = 0x0008  /* LDO4_UV_STS */;

pub const WM831X_LDO3_UV_STS: c_uint = 0x0004  /* LDO3_UV_STS */;
pub const WM831X_LDO3_UV_STS_MASK: c_uint = 0x0004  /* LDO3_UV_STS */;

pub const WM831X_LDO2_UV_STS: c_uint = 0x0002  /* LDO2_UV_STS */;
pub const WM831X_LDO2_UV_STS_MASK: c_uint = 0x0002  /* LDO2_UV_STS */;

pub const WM831X_LDO1_UV_STS: c_uint = 0x0001  /* LDO1_UV_STS */;
pub const WM831X_LDO1_UV_STS_MASK: c_uint = 0x0001  /* LDO1_UV_STS */;

//
// R16470 (0x4056) - DC1 Control 1
//
pub const WM831X_DC1_RATE_MASK: c_uint = 0xC000  /* DC1_RATE - [15:14] */;

pub const WM831X_DC1_PHASE: c_uint = 0x1000  /* DC1_PHASE */;
pub const WM831X_DC1_PHASE_MASK: c_uint = 0x1000  /* DC1_PHASE */;

pub const WM831X_DC1_FREQ_MASK: c_uint = 0x0300  /* DC1_FREQ - [9:8] */;

pub const WM831X_DC1_FLT: c_uint = 0x0080  /* DC1_FLT */;
pub const WM831X_DC1_FLT_MASK: c_uint = 0x0080  /* DC1_FLT */;

pub const WM831X_DC1_SOFT_START_MASK: c_uint = 0x0030  /* DC1_SOFT_START - [5:4] */;

pub const WM831X_DC1_CAP_MASK: c_uint = 0x0003  /* DC1_CAP - [1:0] */;

//
// R16471 (0x4057) - DC1 Control 2
//
pub const WM831X_DC1_ERR_ACT_MASK: c_uint = 0xC000  /* DC1_ERR_ACT - [15:14] */;

pub const WM831X_DC1_HWC_SRC_MASK: c_uint = 0x1800  /* DC1_HWC_SRC - [12:11] */;

pub const WM831X_DC1_HWC_VSEL: c_uint = 0x0400  /* DC1_HWC_VSEL */;
pub const WM831X_DC1_HWC_VSEL_MASK: c_uint = 0x0400  /* DC1_HWC_VSEL */;

pub const WM831X_DC1_HWC_MODE_MASK: c_uint = 0x0300  /* DC1_HWC_MODE - [9:8] */;

pub const WM831X_DC1_HC_THR_MASK: c_uint = 0x0070  /* DC1_HC_THR - [6:4] */;

pub const WM831X_DC1_HC_IND_ENA: c_uint = 0x0001  /* DC1_HC_IND_ENA */;
pub const WM831X_DC1_HC_IND_ENA_MASK: c_uint = 0x0001  /* DC1_HC_IND_ENA */;

//
// R16472 (0x4058) - DC1 ON Config
//
pub const WM831X_DC1_ON_SLOT_MASK: c_uint = 0xE000  /* DC1_ON_SLOT - [15:13] */;

pub const WM831X_DC1_ON_MODE_MASK: c_uint = 0x0300  /* DC1_ON_MODE - [9:8] */;

pub const WM831X_DC1_ON_VSEL_MASK: c_uint = 0x007F  /* DC1_ON_VSEL - [6:0] */;

//
// R16473 (0x4059) - DC1 SLEEP Control
//
pub const WM831X_DC1_SLP_SLOT_MASK: c_uint = 0xE000  /* DC1_SLP_SLOT - [15:13] */;

pub const WM831X_DC1_SLP_MODE_MASK: c_uint = 0x0300  /* DC1_SLP_MODE - [9:8] */;

pub const WM831X_DC1_SLP_VSEL_MASK: c_uint = 0x007F  /* DC1_SLP_VSEL - [6:0] */;

//
// R16474 (0x405A) - DC1 DVS Control
//
pub const WM831X_DC1_DVS_SRC_MASK: c_uint = 0x1800  /* DC1_DVS_SRC - [12:11] */;

pub const WM831X_DC1_DVS_VSEL_MASK: c_uint = 0x007F  /* DC1_DVS_VSEL - [6:0] */;

//
// R16475 (0x405B) - DC2 Control 1
//
pub const WM831X_DC2_RATE_MASK: c_uint = 0xC000  /* DC2_RATE - [15:14] */;

pub const WM831X_DC2_PHASE: c_uint = 0x1000  /* DC2_PHASE */;
pub const WM831X_DC2_PHASE_MASK: c_uint = 0x1000  /* DC2_PHASE */;

pub const WM831X_DC2_FREQ_MASK: c_uint = 0x0300  /* DC2_FREQ - [9:8] */;

pub const WM831X_DC2_FLT: c_uint = 0x0080  /* DC2_FLT */;
pub const WM831X_DC2_FLT_MASK: c_uint = 0x0080  /* DC2_FLT */;

pub const WM831X_DC2_SOFT_START_MASK: c_uint = 0x0030  /* DC2_SOFT_START - [5:4] */;

pub const WM831X_DC2_CAP_MASK: c_uint = 0x0003  /* DC2_CAP - [1:0] */;

//
// R16476 (0x405C) - DC2 Control 2
//
pub const WM831X_DC2_ERR_ACT_MASK: c_uint = 0xC000  /* DC2_ERR_ACT - [15:14] */;

pub const WM831X_DC2_HWC_SRC_MASK: c_uint = 0x1800  /* DC2_HWC_SRC - [12:11] */;

pub const WM831X_DC2_HWC_VSEL: c_uint = 0x0400  /* DC2_HWC_VSEL */;
pub const WM831X_DC2_HWC_VSEL_MASK: c_uint = 0x0400  /* DC2_HWC_VSEL */;

pub const WM831X_DC2_HWC_MODE_MASK: c_uint = 0x0300  /* DC2_HWC_MODE - [9:8] */;

pub const WM831X_DC2_HC_THR_MASK: c_uint = 0x0070  /* DC2_HC_THR - [6:4] */;

pub const WM831X_DC2_HC_IND_ENA: c_uint = 0x0001  /* DC2_HC_IND_ENA */;
pub const WM831X_DC2_HC_IND_ENA_MASK: c_uint = 0x0001  /* DC2_HC_IND_ENA */;

//
// R16477 (0x405D) - DC2 ON Config
//
pub const WM831X_DC2_ON_SLOT_MASK: c_uint = 0xE000  /* DC2_ON_SLOT - [15:13] */;

pub const WM831X_DC2_ON_MODE_MASK: c_uint = 0x0300  /* DC2_ON_MODE - [9:8] */;

pub const WM831X_DC2_ON_VSEL_MASK: c_uint = 0x007F  /* DC2_ON_VSEL - [6:0] */;

//
// R16478 (0x405E) - DC2 SLEEP Control
//
pub const WM831X_DC2_SLP_SLOT_MASK: c_uint = 0xE000  /* DC2_SLP_SLOT - [15:13] */;

pub const WM831X_DC2_SLP_MODE_MASK: c_uint = 0x0300  /* DC2_SLP_MODE - [9:8] */;

pub const WM831X_DC2_SLP_VSEL_MASK: c_uint = 0x007F  /* DC2_SLP_VSEL - [6:0] */;

//
// R16479 (0x405F) - DC2 DVS Control
//
pub const WM831X_DC2_DVS_SRC_MASK: c_uint = 0x1800  /* DC2_DVS_SRC - [12:11] */;

pub const WM831X_DC2_DVS_VSEL_MASK: c_uint = 0x007F  /* DC2_DVS_VSEL - [6:0] */;

//
// R16480 (0x4060) - DC3 Control 1
//
pub const WM831X_DC3_PHASE: c_uint = 0x1000  /* DC3_PHASE */;
pub const WM831X_DC3_PHASE_MASK: c_uint = 0x1000  /* DC3_PHASE */;

pub const WM831X_DC3_FLT: c_uint = 0x0080  /* DC3_FLT */;
pub const WM831X_DC3_FLT_MASK: c_uint = 0x0080  /* DC3_FLT */;

pub const WM831X_DC3_SOFT_START_MASK: c_uint = 0x0030  /* DC3_SOFT_START - [5:4] */;

pub const WM831X_DC3_STNBY_LIM_MASK: c_uint = 0x000C  /* DC3_STNBY_LIM - [3:2] */;

pub const WM831X_DC3_CAP_MASK: c_uint = 0x0003  /* DC3_CAP - [1:0] */;

//
// R16481 (0x4061) - DC3 Control 2
//
pub const WM831X_DC3_ERR_ACT_MASK: c_uint = 0xC000  /* DC3_ERR_ACT - [15:14] */;

pub const WM831X_DC3_HWC_SRC_MASK: c_uint = 0x1800  /* DC3_HWC_SRC - [12:11] */;

pub const WM831X_DC3_HWC_VSEL: c_uint = 0x0400  /* DC3_HWC_VSEL */;
pub const WM831X_DC3_HWC_VSEL_MASK: c_uint = 0x0400  /* DC3_HWC_VSEL */;

pub const WM831X_DC3_HWC_MODE_MASK: c_uint = 0x0300  /* DC3_HWC_MODE - [9:8] */;

pub const WM831X_DC3_OVP: c_uint = 0x0080  /* DC3_OVP */;
pub const WM831X_DC3_OVP_MASK: c_uint = 0x0080  /* DC3_OVP */;

//
// R16482 (0x4062) - DC3 ON Config
//
pub const WM831X_DC3_ON_SLOT_MASK: c_uint = 0xE000  /* DC3_ON_SLOT - [15:13] */;

pub const WM831X_DC3_ON_MODE_MASK: c_uint = 0x0300  /* DC3_ON_MODE - [9:8] */;

pub const WM831X_DC3_ON_VSEL_MASK: c_uint = 0x007F  /* DC3_ON_VSEL - [6:0] */;

//
// R16483 (0x4063) - DC3 SLEEP Control
//
pub const WM831X_DC3_SLP_SLOT_MASK: c_uint = 0xE000  /* DC3_SLP_SLOT - [15:13] */;

pub const WM831X_DC3_SLP_MODE_MASK: c_uint = 0x0300  /* DC3_SLP_MODE - [9:8] */;

pub const WM831X_DC3_SLP_VSEL_MASK: c_uint = 0x007F  /* DC3_SLP_VSEL - [6:0] */;

//
// R16484 (0x4064) - DC4 Control
//
pub const WM831X_DC4_ERR_ACT_MASK: c_uint = 0xC000  /* DC4_ERR_ACT - [15:14] */;

pub const WM831X_DC4_HWC_SRC_MASK: c_uint = 0x1800  /* DC4_HWC_SRC - [12:11] */;

pub const WM831X_DC4_HWC_MODE: c_uint = 0x0100  /* DC4_HWC_MODE */;
pub const WM831X_DC4_HWC_MODE_MASK: c_uint = 0x0100  /* DC4_HWC_MODE */;

pub const WM831X_DC4_RANGE_MASK: c_uint = 0x000C  /* DC4_RANGE - [3:2] */;

pub const WM831X_DC4_FBSRC: c_uint = 0x0001  /* DC4_FBSRC */;
pub const WM831X_DC4_FBSRC_MASK: c_uint = 0x0001  /* DC4_FBSRC */;

//
// R16485 (0x4065) - DC4 SLEEP Control
//
pub const WM831X_DC4_SLPENA: c_uint = 0x0100  /* DC4_SLPENA */;
pub const WM831X_DC4_SLPENA_MASK: c_uint = 0x0100  /* DC4_SLPENA */;

//
// R16488 (0x4068) - LDO1 Control
//
pub const WM831X_LDO1_ERR_ACT_MASK: c_uint = 0xC000  /* LDO1_ERR_ACT - [15:14] */;

pub const WM831X_LDO1_HWC_SRC_MASK: c_uint = 0x1800  /* LDO1_HWC_SRC - [12:11] */;

pub const WM831X_LDO1_HWC_VSEL: c_uint = 0x0400  /* LDO1_HWC_VSEL */;
pub const WM831X_LDO1_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO1_HWC_VSEL */;

pub const WM831X_LDO1_HWC_MODE_MASK: c_uint = 0x0300  /* LDO1_HWC_MODE - [9:8] */;

pub const WM831X_LDO1_FLT: c_uint = 0x0080  /* LDO1_FLT */;
pub const WM831X_LDO1_FLT_MASK: c_uint = 0x0080  /* LDO1_FLT */;

pub const WM831X_LDO1_SWI: c_uint = 0x0040  /* LDO1_SWI */;
pub const WM831X_LDO1_SWI_MASK: c_uint = 0x0040  /* LDO1_SWI */;

pub const WM831X_LDO1_LP_MODE: c_uint = 0x0001  /* LDO1_LP_MODE */;
pub const WM831X_LDO1_LP_MODE_MASK: c_uint = 0x0001  /* LDO1_LP_MODE */;

//
// R16489 (0x4069) - LDO1 ON Control
//
pub const WM831X_LDO1_ON_SLOT_MASK: c_uint = 0xE000  /* LDO1_ON_SLOT - [15:13] */;

pub const WM831X_LDO1_ON_MODE: c_uint = 0x0100  /* LDO1_ON_MODE */;
pub const WM831X_LDO1_ON_MODE_MASK: c_uint = 0x0100  /* LDO1_ON_MODE */;

pub const WM831X_LDO1_ON_VSEL_MASK: c_uint = 0x001F  /* LDO1_ON_VSEL - [4:0] */;

//
// R16490 (0x406A) - LDO1 SLEEP Control
//
pub const WM831X_LDO1_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO1_SLP_SLOT - [15:13] */;

pub const WM831X_LDO1_SLP_MODE: c_uint = 0x0100  /* LDO1_SLP_MODE */;
pub const WM831X_LDO1_SLP_MODE_MASK: c_uint = 0x0100  /* LDO1_SLP_MODE */;

pub const WM831X_LDO1_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO1_SLP_VSEL - [4:0] */;

//
// R16491 (0x406B) - LDO2 Control
//
pub const WM831X_LDO2_ERR_ACT_MASK: c_uint = 0xC000  /* LDO2_ERR_ACT - [15:14] */;

pub const WM831X_LDO2_HWC_SRC_MASK: c_uint = 0x1800  /* LDO2_HWC_SRC - [12:11] */;

pub const WM831X_LDO2_HWC_VSEL: c_uint = 0x0400  /* LDO2_HWC_VSEL */;
pub const WM831X_LDO2_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO2_HWC_VSEL */;

pub const WM831X_LDO2_HWC_MODE_MASK: c_uint = 0x0300  /* LDO2_HWC_MODE - [9:8] */;

pub const WM831X_LDO2_FLT: c_uint = 0x0080  /* LDO2_FLT */;
pub const WM831X_LDO2_FLT_MASK: c_uint = 0x0080  /* LDO2_FLT */;

pub const WM831X_LDO2_SWI: c_uint = 0x0040  /* LDO2_SWI */;
pub const WM831X_LDO2_SWI_MASK: c_uint = 0x0040  /* LDO2_SWI */;

pub const WM831X_LDO2_LP_MODE: c_uint = 0x0001  /* LDO2_LP_MODE */;
pub const WM831X_LDO2_LP_MODE_MASK: c_uint = 0x0001  /* LDO2_LP_MODE */;

//
// R16492 (0x406C) - LDO2 ON Control
//
pub const WM831X_LDO2_ON_SLOT_MASK: c_uint = 0xE000  /* LDO2_ON_SLOT - [15:13] */;

pub const WM831X_LDO2_ON_MODE: c_uint = 0x0100  /* LDO2_ON_MODE */;
pub const WM831X_LDO2_ON_MODE_MASK: c_uint = 0x0100  /* LDO2_ON_MODE */;

pub const WM831X_LDO2_ON_VSEL_MASK: c_uint = 0x001F  /* LDO2_ON_VSEL - [4:0] */;

//
// R16493 (0x406D) - LDO2 SLEEP Control
//
pub const WM831X_LDO2_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO2_SLP_SLOT - [15:13] */;

pub const WM831X_LDO2_SLP_MODE: c_uint = 0x0100  /* LDO2_SLP_MODE */;
pub const WM831X_LDO2_SLP_MODE_MASK: c_uint = 0x0100  /* LDO2_SLP_MODE */;

pub const WM831X_LDO2_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO2_SLP_VSEL - [4:0] */;

//
// R16494 (0x406E) - LDO3 Control
//
pub const WM831X_LDO3_ERR_ACT_MASK: c_uint = 0xC000  /* LDO3_ERR_ACT - [15:14] */;

pub const WM831X_LDO3_HWC_SRC_MASK: c_uint = 0x1800  /* LDO3_HWC_SRC - [12:11] */;

pub const WM831X_LDO3_HWC_VSEL: c_uint = 0x0400  /* LDO3_HWC_VSEL */;
pub const WM831X_LDO3_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO3_HWC_VSEL */;

pub const WM831X_LDO3_HWC_MODE_MASK: c_uint = 0x0300  /* LDO3_HWC_MODE - [9:8] */;

pub const WM831X_LDO3_FLT: c_uint = 0x0080  /* LDO3_FLT */;
pub const WM831X_LDO3_FLT_MASK: c_uint = 0x0080  /* LDO3_FLT */;

pub const WM831X_LDO3_SWI: c_uint = 0x0040  /* LDO3_SWI */;
pub const WM831X_LDO3_SWI_MASK: c_uint = 0x0040  /* LDO3_SWI */;

pub const WM831X_LDO3_LP_MODE: c_uint = 0x0001  /* LDO3_LP_MODE */;
pub const WM831X_LDO3_LP_MODE_MASK: c_uint = 0x0001  /* LDO3_LP_MODE */;

//
// R16495 (0x406F) - LDO3 ON Control
//
pub const WM831X_LDO3_ON_SLOT_MASK: c_uint = 0xE000  /* LDO3_ON_SLOT - [15:13] */;

pub const WM831X_LDO3_ON_MODE: c_uint = 0x0100  /* LDO3_ON_MODE */;
pub const WM831X_LDO3_ON_MODE_MASK: c_uint = 0x0100  /* LDO3_ON_MODE */;

pub const WM831X_LDO3_ON_VSEL_MASK: c_uint = 0x001F  /* LDO3_ON_VSEL - [4:0] */;

//
// R16496 (0x4070) - LDO3 SLEEP Control
//
pub const WM831X_LDO3_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO3_SLP_SLOT - [15:13] */;

pub const WM831X_LDO3_SLP_MODE: c_uint = 0x0100  /* LDO3_SLP_MODE */;
pub const WM831X_LDO3_SLP_MODE_MASK: c_uint = 0x0100  /* LDO3_SLP_MODE */;

pub const WM831X_LDO3_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO3_SLP_VSEL - [4:0] */;

//
// R16497 (0x4071) - LDO4 Control
//
pub const WM831X_LDO4_ERR_ACT_MASK: c_uint = 0xC000  /* LDO4_ERR_ACT - [15:14] */;

pub const WM831X_LDO4_HWC_SRC_MASK: c_uint = 0x1800  /* LDO4_HWC_SRC - [12:11] */;

pub const WM831X_LDO4_HWC_VSEL: c_uint = 0x0400  /* LDO4_HWC_VSEL */;
pub const WM831X_LDO4_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO4_HWC_VSEL */;

pub const WM831X_LDO4_HWC_MODE_MASK: c_uint = 0x0300  /* LDO4_HWC_MODE - [9:8] */;

pub const WM831X_LDO4_FLT: c_uint = 0x0080  /* LDO4_FLT */;
pub const WM831X_LDO4_FLT_MASK: c_uint = 0x0080  /* LDO4_FLT */;

pub const WM831X_LDO4_SWI: c_uint = 0x0040  /* LDO4_SWI */;
pub const WM831X_LDO4_SWI_MASK: c_uint = 0x0040  /* LDO4_SWI */;

pub const WM831X_LDO4_LP_MODE: c_uint = 0x0001  /* LDO4_LP_MODE */;
pub const WM831X_LDO4_LP_MODE_MASK: c_uint = 0x0001  /* LDO4_LP_MODE */;

//
// R16498 (0x4072) - LDO4 ON Control
//
pub const WM831X_LDO4_ON_SLOT_MASK: c_uint = 0xE000  /* LDO4_ON_SLOT - [15:13] */;

pub const WM831X_LDO4_ON_MODE: c_uint = 0x0100  /* LDO4_ON_MODE */;
pub const WM831X_LDO4_ON_MODE_MASK: c_uint = 0x0100  /* LDO4_ON_MODE */;

pub const WM831X_LDO4_ON_VSEL_MASK: c_uint = 0x001F  /* LDO4_ON_VSEL - [4:0] */;

//
// R16499 (0x4073) - LDO4 SLEEP Control
//
pub const WM831X_LDO4_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO4_SLP_SLOT - [15:13] */;

pub const WM831X_LDO4_SLP_MODE: c_uint = 0x0100  /* LDO4_SLP_MODE */;
pub const WM831X_LDO4_SLP_MODE_MASK: c_uint = 0x0100  /* LDO4_SLP_MODE */;

pub const WM831X_LDO4_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO4_SLP_VSEL - [4:0] */;

//
// R16500 (0x4074) - LDO5 Control
//
pub const WM831X_LDO5_ERR_ACT_MASK: c_uint = 0xC000  /* LDO5_ERR_ACT - [15:14] */;

pub const WM831X_LDO5_HWC_SRC_MASK: c_uint = 0x1800  /* LDO5_HWC_SRC - [12:11] */;

pub const WM831X_LDO5_HWC_VSEL: c_uint = 0x0400  /* LDO5_HWC_VSEL */;
pub const WM831X_LDO5_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO5_HWC_VSEL */;

pub const WM831X_LDO5_HWC_MODE_MASK: c_uint = 0x0300  /* LDO5_HWC_MODE - [9:8] */;

pub const WM831X_LDO5_FLT: c_uint = 0x0080  /* LDO5_FLT */;
pub const WM831X_LDO5_FLT_MASK: c_uint = 0x0080  /* LDO5_FLT */;

pub const WM831X_LDO5_SWI: c_uint = 0x0040  /* LDO5_SWI */;
pub const WM831X_LDO5_SWI_MASK: c_uint = 0x0040  /* LDO5_SWI */;

pub const WM831X_LDO5_LP_MODE: c_uint = 0x0001  /* LDO5_LP_MODE */;
pub const WM831X_LDO5_LP_MODE_MASK: c_uint = 0x0001  /* LDO5_LP_MODE */;

//
// R16501 (0x4075) - LDO5 ON Control
//
pub const WM831X_LDO5_ON_SLOT_MASK: c_uint = 0xE000  /* LDO5_ON_SLOT - [15:13] */;

pub const WM831X_LDO5_ON_MODE: c_uint = 0x0100  /* LDO5_ON_MODE */;
pub const WM831X_LDO5_ON_MODE_MASK: c_uint = 0x0100  /* LDO5_ON_MODE */;

pub const WM831X_LDO5_ON_VSEL_MASK: c_uint = 0x001F  /* LDO5_ON_VSEL - [4:0] */;

//
// R16502 (0x4076) - LDO5 SLEEP Control
//
pub const WM831X_LDO5_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO5_SLP_SLOT - [15:13] */;

pub const WM831X_LDO5_SLP_MODE: c_uint = 0x0100  /* LDO5_SLP_MODE */;
pub const WM831X_LDO5_SLP_MODE_MASK: c_uint = 0x0100  /* LDO5_SLP_MODE */;

pub const WM831X_LDO5_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO5_SLP_VSEL - [4:0] */;

//
// R16503 (0x4077) - LDO6 Control
//
pub const WM831X_LDO6_ERR_ACT_MASK: c_uint = 0xC000  /* LDO6_ERR_ACT - [15:14] */;

pub const WM831X_LDO6_HWC_SRC_MASK: c_uint = 0x1800  /* LDO6_HWC_SRC - [12:11] */;

pub const WM831X_LDO6_HWC_VSEL: c_uint = 0x0400  /* LDO6_HWC_VSEL */;
pub const WM831X_LDO6_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO6_HWC_VSEL */;

pub const WM831X_LDO6_HWC_MODE_MASK: c_uint = 0x0300  /* LDO6_HWC_MODE - [9:8] */;

pub const WM831X_LDO6_FLT: c_uint = 0x0080  /* LDO6_FLT */;
pub const WM831X_LDO6_FLT_MASK: c_uint = 0x0080  /* LDO6_FLT */;

pub const WM831X_LDO6_SWI: c_uint = 0x0040  /* LDO6_SWI */;
pub const WM831X_LDO6_SWI_MASK: c_uint = 0x0040  /* LDO6_SWI */;

pub const WM831X_LDO6_LP_MODE: c_uint = 0x0001  /* LDO6_LP_MODE */;
pub const WM831X_LDO6_LP_MODE_MASK: c_uint = 0x0001  /* LDO6_LP_MODE */;

//
// R16504 (0x4078) - LDO6 ON Control
//
pub const WM831X_LDO6_ON_SLOT_MASK: c_uint = 0xE000  /* LDO6_ON_SLOT - [15:13] */;

pub const WM831X_LDO6_ON_MODE: c_uint = 0x0100  /* LDO6_ON_MODE */;
pub const WM831X_LDO6_ON_MODE_MASK: c_uint = 0x0100  /* LDO6_ON_MODE */;

pub const WM831X_LDO6_ON_VSEL_MASK: c_uint = 0x001F  /* LDO6_ON_VSEL - [4:0] */;

//
// R16505 (0x4079) - LDO6 SLEEP Control
//
pub const WM831X_LDO6_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO6_SLP_SLOT - [15:13] */;

pub const WM831X_LDO6_SLP_MODE: c_uint = 0x0100  /* LDO6_SLP_MODE */;
pub const WM831X_LDO6_SLP_MODE_MASK: c_uint = 0x0100  /* LDO6_SLP_MODE */;

pub const WM831X_LDO6_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO6_SLP_VSEL - [4:0] */;

//
// R16506 (0x407A) - LDO7 Control
//
pub const WM831X_LDO7_ERR_ACT_MASK: c_uint = 0xC000  /* LDO7_ERR_ACT - [15:14] */;

pub const WM831X_LDO7_HWC_SRC_MASK: c_uint = 0x1800  /* LDO7_HWC_SRC - [12:11] */;

pub const WM831X_LDO7_HWC_VSEL: c_uint = 0x0400  /* LDO7_HWC_VSEL */;
pub const WM831X_LDO7_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO7_HWC_VSEL */;

pub const WM831X_LDO7_HWC_MODE_MASK: c_uint = 0x0300  /* LDO7_HWC_MODE - [9:8] */;

pub const WM831X_LDO7_FLT: c_uint = 0x0080  /* LDO7_FLT */;
pub const WM831X_LDO7_FLT_MASK: c_uint = 0x0080  /* LDO7_FLT */;

pub const WM831X_LDO7_SWI: c_uint = 0x0040  /* LDO7_SWI */;
pub const WM831X_LDO7_SWI_MASK: c_uint = 0x0040  /* LDO7_SWI */;

//
// R16507 (0x407B) - LDO7 ON Control
//
pub const WM831X_LDO7_ON_SLOT_MASK: c_uint = 0xE000  /* LDO7_ON_SLOT - [15:13] */;

pub const WM831X_LDO7_ON_MODE: c_uint = 0x0100  /* LDO7_ON_MODE */;
pub const WM831X_LDO7_ON_MODE_MASK: c_uint = 0x0100  /* LDO7_ON_MODE */;

pub const WM831X_LDO7_ON_VSEL_MASK: c_uint = 0x001F  /* LDO7_ON_VSEL - [4:0] */;

//
// R16508 (0x407C) - LDO7 SLEEP Control
//
pub const WM831X_LDO7_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO7_SLP_SLOT - [15:13] */;

pub const WM831X_LDO7_SLP_MODE: c_uint = 0x0100  /* LDO7_SLP_MODE */;
pub const WM831X_LDO7_SLP_MODE_MASK: c_uint = 0x0100  /* LDO7_SLP_MODE */;

pub const WM831X_LDO7_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO7_SLP_VSEL - [4:0] */;

//
// R16509 (0x407D) - LDO8 Control
//
pub const WM831X_LDO8_ERR_ACT_MASK: c_uint = 0xC000  /* LDO8_ERR_ACT - [15:14] */;

pub const WM831X_LDO8_HWC_SRC_MASK: c_uint = 0x1800  /* LDO8_HWC_SRC - [12:11] */;

pub const WM831X_LDO8_HWC_VSEL: c_uint = 0x0400  /* LDO8_HWC_VSEL */;
pub const WM831X_LDO8_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO8_HWC_VSEL */;

pub const WM831X_LDO8_HWC_MODE_MASK: c_uint = 0x0300  /* LDO8_HWC_MODE - [9:8] */;

pub const WM831X_LDO8_FLT: c_uint = 0x0080  /* LDO8_FLT */;
pub const WM831X_LDO8_FLT_MASK: c_uint = 0x0080  /* LDO8_FLT */;

pub const WM831X_LDO8_SWI: c_uint = 0x0040  /* LDO8_SWI */;
pub const WM831X_LDO8_SWI_MASK: c_uint = 0x0040  /* LDO8_SWI */;

//
// R16510 (0x407E) - LDO8 ON Control
//
pub const WM831X_LDO8_ON_SLOT_MASK: c_uint = 0xE000  /* LDO8_ON_SLOT - [15:13] */;

pub const WM831X_LDO8_ON_MODE: c_uint = 0x0100  /* LDO8_ON_MODE */;
pub const WM831X_LDO8_ON_MODE_MASK: c_uint = 0x0100  /* LDO8_ON_MODE */;

pub const WM831X_LDO8_ON_VSEL_MASK: c_uint = 0x001F  /* LDO8_ON_VSEL - [4:0] */;

//
// R16511 (0x407F) - LDO8 SLEEP Control
//
pub const WM831X_LDO8_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO8_SLP_SLOT - [15:13] */;

pub const WM831X_LDO8_SLP_MODE: c_uint = 0x0100  /* LDO8_SLP_MODE */;
pub const WM831X_LDO8_SLP_MODE_MASK: c_uint = 0x0100  /* LDO8_SLP_MODE */;

pub const WM831X_LDO8_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO8_SLP_VSEL - [4:0] */;

//
// R16512 (0x4080) - LDO9 Control
//
pub const WM831X_LDO9_ERR_ACT_MASK: c_uint = 0xC000  /* LDO9_ERR_ACT - [15:14] */;

pub const WM831X_LDO9_HWC_SRC_MASK: c_uint = 0x1800  /* LDO9_HWC_SRC - [12:11] */;

pub const WM831X_LDO9_HWC_VSEL: c_uint = 0x0400  /* LDO9_HWC_VSEL */;
pub const WM831X_LDO9_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO9_HWC_VSEL */;

pub const WM831X_LDO9_HWC_MODE_MASK: c_uint = 0x0300  /* LDO9_HWC_MODE - [9:8] */;

pub const WM831X_LDO9_FLT: c_uint = 0x0080  /* LDO9_FLT */;
pub const WM831X_LDO9_FLT_MASK: c_uint = 0x0080  /* LDO9_FLT */;

pub const WM831X_LDO9_SWI: c_uint = 0x0040  /* LDO9_SWI */;
pub const WM831X_LDO9_SWI_MASK: c_uint = 0x0040  /* LDO9_SWI */;

//
// R16513 (0x4081) - LDO9 ON Control
//
pub const WM831X_LDO9_ON_SLOT_MASK: c_uint = 0xE000  /* LDO9_ON_SLOT - [15:13] */;

pub const WM831X_LDO9_ON_MODE: c_uint = 0x0100  /* LDO9_ON_MODE */;
pub const WM831X_LDO9_ON_MODE_MASK: c_uint = 0x0100  /* LDO9_ON_MODE */;

pub const WM831X_LDO9_ON_VSEL_MASK: c_uint = 0x001F  /* LDO9_ON_VSEL - [4:0] */;

//
// R16514 (0x4082) - LDO9 SLEEP Control
//
pub const WM831X_LDO9_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO9_SLP_SLOT - [15:13] */;

pub const WM831X_LDO9_SLP_MODE: c_uint = 0x0100  /* LDO9_SLP_MODE */;
pub const WM831X_LDO9_SLP_MODE_MASK: c_uint = 0x0100  /* LDO9_SLP_MODE */;

pub const WM831X_LDO9_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO9_SLP_VSEL - [4:0] */;

//
// R16515 (0x4083) - LDO10 Control
//
pub const WM831X_LDO10_ERR_ACT_MASK: c_uint = 0xC000  /* LDO10_ERR_ACT - [15:14] */;

pub const WM831X_LDO10_HWC_SRC_MASK: c_uint = 0x1800  /* LDO10_HWC_SRC - [12:11] */;

pub const WM831X_LDO10_HWC_VSEL: c_uint = 0x0400  /* LDO10_HWC_VSEL */;
pub const WM831X_LDO10_HWC_VSEL_MASK: c_uint = 0x0400  /* LDO10_HWC_VSEL */;

pub const WM831X_LDO10_HWC_MODE_MASK: c_uint = 0x0300  /* LDO10_HWC_MODE - [9:8] */;

pub const WM831X_LDO10_FLT: c_uint = 0x0080  /* LDO10_FLT */;
pub const WM831X_LDO10_FLT_MASK: c_uint = 0x0080  /* LDO10_FLT */;

pub const WM831X_LDO10_SWI: c_uint = 0x0040  /* LDO10_SWI */;
pub const WM831X_LDO10_SWI_MASK: c_uint = 0x0040  /* LDO10_SWI */;

//
// R16516 (0x4084) - LDO10 ON Control
//
pub const WM831X_LDO10_ON_SLOT_MASK: c_uint = 0xE000  /* LDO10_ON_SLOT - [15:13] */;

pub const WM831X_LDO10_ON_MODE: c_uint = 0x0100  /* LDO10_ON_MODE */;
pub const WM831X_LDO10_ON_MODE_MASK: c_uint = 0x0100  /* LDO10_ON_MODE */;

pub const WM831X_LDO10_ON_VSEL_MASK: c_uint = 0x001F  /* LDO10_ON_VSEL - [4:0] */;

//
// R16517 (0x4085) - LDO10 SLEEP Control
//
pub const WM831X_LDO10_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO10_SLP_SLOT - [15:13] */;

pub const WM831X_LDO10_SLP_MODE: c_uint = 0x0100  /* LDO10_SLP_MODE */;
pub const WM831X_LDO10_SLP_MODE_MASK: c_uint = 0x0100  /* LDO10_SLP_MODE */;

pub const WM831X_LDO10_SLP_VSEL_MASK: c_uint = 0x001F  /* LDO10_SLP_VSEL - [4:0] */;

//
// R16519 (0x4087) - LDO11 ON Control
//
pub const WM831X_LDO11_ON_SLOT_MASK: c_uint = 0xE000  /* LDO11_ON_SLOT - [15:13] */;

pub const WM831X_LDO11_OFFENA: c_uint = 0x1000  /* LDO11_OFFENA */;
pub const WM831X_LDO11_OFFENA_MASK: c_uint = 0x1000  /* LDO11_OFFENA */;

pub const WM831X_LDO11_VSEL_SRC: c_uint = 0x0080  /* LDO11_VSEL_SRC */;
pub const WM831X_LDO11_VSEL_SRC_MASK: c_uint = 0x0080  /* LDO11_VSEL_SRC */;

pub const WM831X_LDO11_ON_VSEL_MASK: c_uint = 0x000F  /* LDO11_ON_VSEL - [3:0] */;

//
// R16520 (0x4088) - LDO11 SLEEP Control
//
pub const WM831X_LDO11_SLP_SLOT_MASK: c_uint = 0xE000  /* LDO11_SLP_SLOT - [15:13] */;

pub const WM831X_LDO11_SLP_VSEL_MASK: c_uint = 0x000F  /* LDO11_SLP_VSEL - [3:0] */;

//
// R16526 (0x408E) - Power Good Source 1
//
pub const WM831X_DC4_OK: c_uint = 0x0008  /* DC4_OK */;
pub const WM831X_DC4_OK_MASK: c_uint = 0x0008  /* DC4_OK */;

pub const WM831X_DC3_OK: c_uint = 0x0004  /* DC3_OK */;
pub const WM831X_DC3_OK_MASK: c_uint = 0x0004  /* DC3_OK */;

pub const WM831X_DC2_OK: c_uint = 0x0002  /* DC2_OK */;
pub const WM831X_DC2_OK_MASK: c_uint = 0x0002  /* DC2_OK */;

pub const WM831X_DC1_OK: c_uint = 0x0001  /* DC1_OK */;
pub const WM831X_DC1_OK_MASK: c_uint = 0x0001  /* DC1_OK */;

//
// R16527 (0x408F) - Power Good Source 2
//
pub const WM831X_LDO10_OK: c_uint = 0x0200  /* LDO10_OK */;
pub const WM831X_LDO10_OK_MASK: c_uint = 0x0200  /* LDO10_OK */;

pub const WM831X_LDO9_OK: c_uint = 0x0100  /* LDO9_OK */;
pub const WM831X_LDO9_OK_MASK: c_uint = 0x0100  /* LDO9_OK */;

pub const WM831X_LDO8_OK: c_uint = 0x0080  /* LDO8_OK */;
pub const WM831X_LDO8_OK_MASK: c_uint = 0x0080  /* LDO8_OK */;

pub const WM831X_LDO7_OK: c_uint = 0x0040  /* LDO7_OK */;
pub const WM831X_LDO7_OK_MASK: c_uint = 0x0040  /* LDO7_OK */;

pub const WM831X_LDO6_OK: c_uint = 0x0020  /* LDO6_OK */;
pub const WM831X_LDO6_OK_MASK: c_uint = 0x0020  /* LDO6_OK */;

pub const WM831X_LDO5_OK: c_uint = 0x0010  /* LDO5_OK */;
pub const WM831X_LDO5_OK_MASK: c_uint = 0x0010  /* LDO5_OK */;

pub const WM831X_LDO4_OK: c_uint = 0x0008  /* LDO4_OK */;
pub const WM831X_LDO4_OK_MASK: c_uint = 0x0008  /* LDO4_OK */;

pub const WM831X_LDO3_OK: c_uint = 0x0004  /* LDO3_OK */;
pub const WM831X_LDO3_OK_MASK: c_uint = 0x0004  /* LDO3_OK */;

pub const WM831X_LDO2_OK: c_uint = 0x0002  /* LDO2_OK */;
pub const WM831X_LDO2_OK_MASK: c_uint = 0x0002  /* LDO2_OK */;

pub const WM831X_LDO1_OK: c_uint = 0x0001  /* LDO1_OK */;
pub const WM831X_LDO1_OK_MASK: c_uint = 0x0001  /* LDO1_OK */;

pub const WM831X_ISINK_MAX_ISEL: c_int = 55;
