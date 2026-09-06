//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/ad525x_dpot.h
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
// Driver for the Analog Devices digital potentiometers
//
// Copyright (C) 2010 Michael Hennerich, Analog Devices Inc.
//

pub const MAX_RDACS: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpot_devid {
    AD5258_ID = DPOT_CONF(F_RDACS_RW_TOL, BRDAC0, 6, 0), /* I2C */
    AD5259_ID = DPOT_CONF(F_RDACS_RW_TOL, BRDAC0, 8, 1),
    AD5251_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC1 | BRDAC3, 6, 2),
    AD5252_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC1 | BRDAC3, 8, 3),
    AD5253_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 6, 4),
    AD5254_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 8, 5),
    AD5255_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC0 | BRDAC1 | BRDAC2, 9, 6),
    AD5160_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 7), /* SPI */
    AD5161_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 8),
    AD5162_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1, 8, 9),
    AD5165_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 10),
    AD5200_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 11),
    AD5201_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 5, 12),
    AD5203_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 6, 13),
    AD5204_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 8, 14),
    AD5206_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3 | BRDAC4 | BRDAC5,
    8, 15),
    AD5207_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1, 8, 16),
    AD5231_ID = DPOT_CONF(F_RDACS_RW_EEP | F_CMD_INC | F_SPI_24BIT,
    BRDAC0, 10, 17),
    AD5232_ID = DPOT_CONF(F_RDACS_RW_EEP | F_CMD_INC | F_SPI_16BIT,
    BRDAC0 | BRDAC1, 8, 18),
    AD5233_ID = DPOT_CONF(F_RDACS_RW_EEP | F_CMD_INC | F_SPI_16BIT,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 6, 19),
    AD5235_ID = DPOT_CONF(F_RDACS_RW_EEP | F_CMD_INC | F_SPI_24BIT,
    BRDAC0 | BRDAC1, 10, 20),
    AD5260_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 21),
    AD5262_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1, 8, 22),
    AD5263_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1 | BRDAC2 | BRDAC3, 8, 23),
    AD5290_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 8, 24),
    AD5291_ID = DPOT_CONF(F_RDACS_RW | F_SPI_16BIT | F_CMD_OTP,
    BRDAC0, 8, 25),
    AD5292_ID = DPOT_CONF(F_RDACS_RW | F_SPI_16BIT | F_CMD_OTP,
    BRDAC0, 10, 26),
    AD5293_ID = DPOT_CONF(F_RDACS_RW | F_SPI_16BIT, BRDAC0, 10, 27),
    AD7376_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_8BIT,
    BRDAC0, 7, 28),
    AD8400_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0, 8, 29),
    AD8402_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1, 8, 30),
    AD8403_ID = DPOT_CONF(F_RDACS_WONLY | F_AD_APPDATA | F_SPI_16BIT,
    BRDAC0 | BRDAC1 | BRDAC2, 8, 31),
    ADN2850_ID = DPOT_CONF(F_RDACS_RW_EEP | F_CMD_INC | F_SPI_24BIT,
    BRDAC0 | BRDAC1, 10, 32),
    AD5241_ID = DPOT_CONF(F_RDACS_RW, BRDAC0, 8, 33),
    AD5242_ID = DPOT_CONF(F_RDACS_RW, BRDAC0 | BRDAC1, 8, 34),
    AD5243_ID = DPOT_CONF(F_RDACS_RW, BRDAC0 | BRDAC1, 8, 35),
    AD5245_ID = DPOT_CONF(F_RDACS_RW, BRDAC0, 8, 36),
    AD5246_ID = DPOT_CONF(F_RDACS_RW, BRDAC0, 7, 37),
    AD5247_ID = DPOT_CONF(F_RDACS_RW, BRDAC0, 7, 38),
    AD5248_ID = DPOT_CONF(F_RDACS_RW, BRDAC0 | BRDAC1, 8, 39),
    AD5280_ID = DPOT_CONF(F_RDACS_RW, BRDAC0, 8, 40),
    AD5282_ID = DPOT_CONF(F_RDACS_RW, BRDAC0 | BRDAC1, 8, 41),
    ADN2860_ID = DPOT_CONF(F_RDACS_RW_TOL | F_CMD_INC,
    BRDAC0 | BRDAC1 | BRDAC2, 9, 42),
    AD5273_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0, 6, 43),
    AD5171_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0, 6, 44),
    AD5170_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0, 8, 45),
    AD5172_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0 | BRDAC1, 8, 46),
    AD5173_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0 | BRDAC1, 8, 47),
    AD5270_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP | F_SPI_16BIT,
    BRDAC0, 10, 48),
    AD5271_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP | F_SPI_16BIT,
    BRDAC0, 8, 49),
    AD5272_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0, 10, 50),
    AD5274_ID = DPOT_CONF(F_RDACS_RW | F_CMD_OTP, BRDAC0, 8, 51),
}

pub const DPOT_RDAC0: c_int = 0;
pub const DPOT_RDAC1: c_int = 1;
pub const DPOT_RDAC2: c_int = 2;
pub const DPOT_RDAC3: c_int = 3;
pub const DPOT_RDAC4: c_int = 4;
pub const DPOT_RDAC5: c_int = 5;
pub const DPOT_RDAC_MASK: c_uint = 0x1F;
pub const DPOT_REG_TOL: c_uint = 0x18;

// RDAC-to-EEPROM Interface Commands

pub const DPOT_SPI_RDAC: c_uint = 0xB0;
pub const DPOT_SPI_EEPROM: c_uint = 0x30;
pub const DPOT_SPI_READ_RDAC: c_uint = 0xA0;
pub const DPOT_SPI_READ_EEPROM: c_uint = 0x90;
pub const DPOT_SPI_DEC_ALL_6DB: c_uint = 0x50;
pub const DPOT_SPI_INC_ALL_6DB: c_uint = 0xD0;
pub const DPOT_SPI_DEC_ALL: c_uint = 0x70;
pub const DPOT_SPI_INC_ALL: c_uint = 0xF0;
// AD5291/2/3 use special commands
pub const DPOT_AD5291_RDAC: c_uint = 0x01;
pub const DPOT_AD5291_READ_RDAC: c_uint = 0x02;
pub const DPOT_AD5291_STORE_XTPM: c_uint = 0x03;
pub const DPOT_AD5291_CTRLREG: c_uint = 0x06;
pub const DPOT_AD5291_UNLOCK_CMD: c_uint = 0x03;
// AD5270/1/2/4 use special commands
pub const DPOT_AD5270_1_2_4_RDAC: c_uint = 0x01;
pub const DPOT_AD5270_1_2_4_READ_RDAC: c_uint = 0x02;
pub const DPOT_AD5270_1_2_4_STORE_XTPM: c_uint = 0x03;
pub const DPOT_AD5270_1_2_4_CTRLREG: c_uint = 0x07;
pub const DPOT_AD5270_1_2_4_UNLOCK_CMD: c_uint = 0x03;
pub const DPOT_AD5282_RDAC_AB: c_uint = 0x80;
pub const DPOT_AD5273_FUSE: c_uint = 0x80;
pub const DPOT_AD5170_2_3_FUSE: c_uint = 0x20;
pub const DPOT_AD5170_2_3_OW: c_uint = 0x08;
pub const DPOT_AD5172_3_A0: c_uint = 0x08;
pub const DPOT_AD5170_2FUSE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_dpot_bus_ops {
    pub client): *mut *mut int (read_d8)(void,
    pub reg): *mut *mut *mut int (read_r8d8)(void client, u8,
    pub reg): *mut *mut *mut int (read_r8d16)(void client, u8,
    pub val): *mut *mut *mut int (write_d8)(void client, u8,
    pub val): *mut *mut *mut int (write_r8d8)(void client, u8 reg, u8,
    pub val): *mut *mut *mut int (write_r8d16)(void client, u8 reg, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_dpot_bus_data {
    pub client: *mut c_void,
    pub bops: *const ad_dpot_bus_ops,
}

extern "C" {
    pub fn ad_dpot_remove(dev: *mut device);
}
