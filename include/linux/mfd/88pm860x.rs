//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/88pm860x.h
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
// Marvell 88PM860x Interface
//
// Copyright (C) 2009 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

// 8606 Registers

// Power Up Log Register

// Charger Control Registers

// Backlight Registers

// LED Registers

// Bit definitions of PM8606 registers

// 8607 chip ID is 0x40 or 0x50

// Interrupt Registers

// Regulator Control Registers

// Vibrator Control Registers

// GPADC Registers

// bit definitions of  MEAS_EN1

// Battery Monitor Registers

// RTC Control Registers

// Misc Registers

// bit definitions of Status Query Interface

// bit definitions of BUCK3

// bit definitions of Misc1

// bits definitions of GPADC

pub const PM8606_REF_GP_OSC_OFF: c_int = 0;
pub const PM8606_REF_GP_OSC_ON: c_int = 1;
pub const PM8606_REF_GP_OSC_UNKNOWN: c_int = 2;
// Clients of reference group and 8MHz oscillator in 88PM8606
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm8606_ref_gp_and_osc_clients {
    REF_GP_NO_CLIENTS       = 0,
    WLED1_DUTY              = (1<<0), /*PF 0x02.7:0*/
    WLED2_DUTY              = (1<<1), /*PF 0x04.7:0*/
    WLED3_DUTY              = (1<<2), /*PF 0x06.7:0*/
    RGB1_ENABLE             = (1<<3), /*PF 0x07.1*/
    RGB2_ENABLE             = (1<<4), /*PF 0x07.2*/
    LDO_VBR_EN              = (1<<5), /*PF 0x12.0*/
    REF_GP_MAX_CLIENT       = 0xFFFF
}

// Interrupt Number in 88PM8607
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_chip {
    pub dev: *mut device,
    pub irq_lock: mutex,
    pub osc_lock: mutex,
    pub client: *mut i2c_client,
    pub /: *mut *mut *mut i2c_client companion; / companion chip client,
    pub regmap: *mut regmap,
    pub regmap_companion: *mut regmap,
    pub /: *mut *mut int buck3_double; / DVC ramp slope double,
    pub companion_addr: c_int,
    pub osc_vote: c_ushort,
    pub id: c_int,
    pub irq_mode: c_int,
    pub irq_base: c_int,
    pub core_irq: c_int,
    pub chip_version: c_uchar,
    pub osc_status: c_uchar,
    pub wakeup_flag: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_backlight_pdata {
    pub pwm: c_int,
    pub iset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_led_pdata {
    pub iset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_rtc_pdata {
    pub ticks): *mut *mut int (sync)(unsigned int,
    pub vrtc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_touch_pdata {
    pub gpadc_prebias: c_int,
    pub slot_cycle: c_int,
    pub off_scale: c_int,
    pub sw_cal: c_int,
    pub /: *mut *mut int tsi_prebias; / time, slot,
    pub /: *mut *mut int pen_prebias; / time, slot,
    pub /: *mut *mut int pen_prechg; / time, slot,
    pub /: *mut *mut int res_x; / resistor of Xplate,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_power_pdata {
    pub max_capacity: c_int,
    pub resistor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_platform_data {
    pub backlight: *mut pm860x_backlight_pdata,
    pub led: *mut pm860x_led_pdata,
    pub rtc: *mut pm860x_rtc_pdata,
    pub touch: *mut pm860x_touch_pdata,
    pub power: *mut pm860x_power_pdata,
    pub buck1: *mut regulator_init_data,
    pub buck2: *mut regulator_init_data,
    pub buck3: *mut regulator_init_data,
    pub ldo1: *mut regulator_init_data,
    pub ldo2: *mut regulator_init_data,
    pub ldo3: *mut regulator_init_data,
    pub ldo4: *mut regulator_init_data,
    pub ldo5: *mut regulator_init_data,
    pub ldo6: *mut regulator_init_data,
    pub ldo7: *mut regulator_init_data,
    pub ldo8: *mut regulator_init_data,
    pub ldo9: *mut regulator_init_data,
    pub ldo10: *mut regulator_init_data,
    pub ldo12: *mut regulator_init_data,
    pub ldo_vibrator: *mut regulator_init_data,
    pub ldo14: *mut regulator_init_data,
    pub chg_desc: *mut charger_desc,
    pub /: *mut *mut int companion_addr; / I2C address of companion chip,
    pub /: *mut *mut int i2c_port; / Controlled by GI2C or PI2C,
    pub /: *mut *mut int irq_mode; / Clear interrupt by read/write(0/1),
    pub /: *mut *mut int irq_base; / IRQ base number of 88pm860x,
    pub num_leds: c_int,
    pub num_backlights: c_int,
}

extern "C" {
    pub fn pm8606_osc_enable(: *mut pm860x_chip, short: unsigned) -> c_int;
}
extern "C" {
    pub fn pm8606_osc_disable(: *mut pm860x_chip, short: unsigned) -> c_int;
}
extern "C" {
    pub fn pm860x_reg_read(: *mut i2c_client, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn pm860x_reg_write(: *mut i2c_client, _arg: c_int, char: unsigned) -> c_int;
}
extern "C" {
    pub fn pm860x_bulk_read(: *mut i2c_client, _arg: c_int, _arg: c_int, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn pm860x_bulk_write(: *mut i2c_client, _arg: c_int, _arg: c_int, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn pm860x_page_reg_write(: *mut i2c_client, _arg: c_int, char: unsigned) -> c_int;
}
