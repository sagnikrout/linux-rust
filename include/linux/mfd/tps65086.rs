//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65086.h
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
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// Based on the TPS65912 driver
//

// List of registers for TPS65086
pub const TPS65086_DEVICEID1: c_uint = 0x00;
pub const TPS65086_DEVICEID2: c_uint = 0x01;
pub const TPS65086_IRQ: c_uint = 0x02;
pub const TPS65086_IRQ_MASK: c_uint = 0x03;
pub const TPS65086_PMICSTAT: c_uint = 0x04;
pub const TPS65086_SHUTDNSRC: c_uint = 0x05;
pub const TPS65086_BUCK1CTRL: c_uint = 0x20;
pub const TPS65086_BUCK2CTRL: c_uint = 0x21;
pub const TPS65086_BUCK3DECAY: c_uint = 0x22;
pub const TPS65086_BUCK3VID: c_uint = 0x23;
pub const TPS65086_BUCK3SLPCTRL: c_uint = 0x24;
pub const TPS65086_BUCK4CTRL: c_uint = 0x25;
pub const TPS65086_BUCK5CTRL: c_uint = 0x26;
pub const TPS65086_BUCK6CTRL: c_uint = 0x27;
pub const TPS65086_LDOA2CTRL: c_uint = 0x28;
pub const TPS65086_LDOA3CTRL: c_uint = 0x29;
pub const TPS65086_DISCHCTRL1: c_uint = 0x40;
pub const TPS65086_DISCHCTRL2: c_uint = 0x41;
pub const TPS65086_DISCHCTRL3: c_uint = 0x42;
pub const TPS65086_PG_DELAY1: c_uint = 0x43;
pub const TPS65086_FORCESHUTDN: c_uint = 0x91;
pub const TPS65086_BUCK1SLPCTRL: c_uint = 0x92;
pub const TPS65086_BUCK2SLPCTRL: c_uint = 0x93;
pub const TPS65086_BUCK4VID: c_uint = 0x94;
pub const TPS65086_BUCK4SLPVID: c_uint = 0x95;
pub const TPS65086_BUCK5VID: c_uint = 0x96;
pub const TPS65086_BUCK5SLPVID: c_uint = 0x97;
pub const TPS65086_BUCK6VID: c_uint = 0x98;
pub const TPS65086_BUCK6SLPVID: c_uint = 0x99;
pub const TPS65086_LDOA2VID: c_uint = 0x9A;
pub const TPS65086_LDOA3VID: c_uint = 0x9B;
pub const TPS65086_BUCK123CTRL: c_uint = 0x9C;
pub const TPS65086_PG_DELAY2: c_uint = 0x9D;
pub const TPS65086_PIN_EN_MASK1: c_uint = 0x9E;
pub const TPS65086_PIN_EN_MASK2: c_uint = 0x9F;
pub const TPS65086_SWVTT_EN: c_uint = 0x9F;
pub const TPS65086_PIN_EN_OVR1: c_uint = 0xA0;
pub const TPS65086_PIN_EN_OVR2: c_uint = 0xA1;
pub const TPS65086_GPOCTRL: c_uint = 0xA1;
pub const TPS65086_PWR_FAULT_MASK1: c_uint = 0xA2;
pub const TPS65086_PWR_FAULT_MASK2: c_uint = 0xA3;
pub const TPS65086_GPO1PG_CTRL1: c_uint = 0xA4;
pub const TPS65086_GPO1PG_CTRL2: c_uint = 0xA5;
pub const TPS65086_GPO4PG_CTRL1: c_uint = 0xA6;
pub const TPS65086_GPO4PG_CTRL2: c_uint = 0xA7;
pub const TPS65086_GPO2PG_CTRL1: c_uint = 0xA8;
pub const TPS65086_GPO2PG_CTRL2: c_uint = 0xA9;
pub const TPS65086_GPO3PG_CTRL1: c_uint = 0xAA;
pub const TPS65086_GPO3PG_CTRL2: c_uint = 0xAB;
pub const TPS65086_LDOA1CTRL: c_uint = 0xAE;
pub const TPS65086_PG_STATUS1: c_uint = 0xB0;
pub const TPS65086_PG_STATUS2: c_uint = 0xB1;
pub const TPS65086_PWR_FAULT_STATUS1: c_uint = 0xB2;
pub const TPS65086_PWR_FAULT_STATUS2: c_uint = 0xB3;
pub const TPS65086_TEMPCRIT: c_uint = 0xB4;
pub const TPS65086_TEMPHOT: c_uint = 0xB5;
pub const TPS65086_OC_STATUS: c_uint = 0xB6;
// IRQ Register field definitions

// DEVICEID1 Register field definitions
pub const TPS6508640_ID: c_uint = 0x00;
pub const TPS65086401_ID: c_uint = 0x01;
pub const TPS6508641_ID: c_uint = 0x10;
pub const TPS65086470_ID: c_uint = 0x70;
// DEVICEID2 Register field definitions

// VID Masks

// Define the TPS65086 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65086_irqs {
    TPS65086_IRQ_DIETEMP,
    TPS65086_IRQ_SHUTDN,
    TPS65086_IRQ_FAULT,
}

//
// struct tps65086 - state holder for the tps65086 driver
//
// Device data may be used to access the TPS65086 chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65086 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub chip_id: c_uint,
    pub reg_config: *const tps65086_regulator_config,
// IRQ Data
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}
