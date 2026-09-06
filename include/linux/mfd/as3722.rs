//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/as3722.h
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
// as3722 definitions
//
// Copyright (C) 2013 ams
// Copyright (c) 2013, NVIDIA Corporation. All rights reserved.
//
// Author: Florian Lobmaier <florian.lobmaier@ams.com>
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//

// AS3722 registers
pub const AS3722_SD0_VOLTAGE_REG: c_uint = 0x00;
pub const AS3722_SD1_VOLTAGE_REG: c_uint = 0x01;
pub const AS3722_SD2_VOLTAGE_REG: c_uint = 0x02;
pub const AS3722_SD3_VOLTAGE_REG: c_uint = 0x03;
pub const AS3722_SD4_VOLTAGE_REG: c_uint = 0x04;
pub const AS3722_SD5_VOLTAGE_REG: c_uint = 0x05;
pub const AS3722_SD6_VOLTAGE_REG: c_uint = 0x06;
pub const AS3722_GPIO0_CONTROL_REG: c_uint = 0x08;
pub const AS3722_GPIO1_CONTROL_REG: c_uint = 0x09;
pub const AS3722_GPIO2_CONTROL_REG: c_uint = 0x0A;
pub const AS3722_GPIO3_CONTROL_REG: c_uint = 0x0B;
pub const AS3722_GPIO4_CONTROL_REG: c_uint = 0x0C;
pub const AS3722_GPIO5_CONTROL_REG: c_uint = 0x0D;
pub const AS3722_GPIO6_CONTROL_REG: c_uint = 0x0E;
pub const AS3722_GPIO7_CONTROL_REG: c_uint = 0x0F;
pub const AS3722_LDO0_VOLTAGE_REG: c_uint = 0x10;
pub const AS3722_LDO1_VOLTAGE_REG: c_uint = 0x11;
pub const AS3722_LDO2_VOLTAGE_REG: c_uint = 0x12;
pub const AS3722_LDO3_VOLTAGE_REG: c_uint = 0x13;
pub const AS3722_LDO4_VOLTAGE_REG: c_uint = 0x14;
pub const AS3722_LDO5_VOLTAGE_REG: c_uint = 0x15;
pub const AS3722_LDO6_VOLTAGE_REG: c_uint = 0x16;
pub const AS3722_LDO7_VOLTAGE_REG: c_uint = 0x17;
pub const AS3722_LDO9_VOLTAGE_REG: c_uint = 0x19;
pub const AS3722_LDO10_VOLTAGE_REG: c_uint = 0x1A;
pub const AS3722_LDO11_VOLTAGE_REG: c_uint = 0x1B;
pub const AS3722_GPIO_DEB1_REG: c_uint = 0x1E;
pub const AS3722_GPIO_DEB2_REG: c_uint = 0x1F;
pub const AS3722_GPIO_SIGNAL_OUT_REG: c_uint = 0x20;
pub const AS3722_GPIO_SIGNAL_IN_REG: c_uint = 0x21;
pub const AS3722_REG_SEQU_MOD1_REG: c_uint = 0x22;
pub const AS3722_REG_SEQU_MOD2_REG: c_uint = 0x23;
pub const AS3722_REG_SEQU_MOD3_REG: c_uint = 0x24;
pub const AS3722_SD_PHSW_CTRL_REG: c_uint = 0x27;
pub const AS3722_SD_PHSW_STATUS: c_uint = 0x28;
pub const AS3722_SD0_CONTROL_REG: c_uint = 0x29;
pub const AS3722_SD1_CONTROL_REG: c_uint = 0x2A;
pub const AS3722_SDmph_CONTROL_REG: c_uint = 0x2B;
pub const AS3722_SD23_CONTROL_REG: c_uint = 0x2C;
pub const AS3722_SD4_CONTROL_REG: c_uint = 0x2D;
pub const AS3722_SD5_CONTROL_REG: c_uint = 0x2E;
pub const AS3722_SD6_CONTROL_REG: c_uint = 0x2F;
pub const AS3722_SD_DVM_REG: c_uint = 0x30;
pub const AS3722_RESET_REASON_REG: c_uint = 0x31;
pub const AS3722_BATTERY_VOLTAGE_MONITOR_REG: c_uint = 0x32;
pub const AS3722_STARTUP_CONTROL_REG: c_uint = 0x33;
pub const AS3722_RESET_TIMER_REG: c_uint = 0x34;
pub const AS3722_REFERENCE_CONTROL_REG: c_uint = 0x35;
pub const AS3722_RESET_CONTROL_REG: c_uint = 0x36;
pub const AS3722_OVER_TEMP_CONTROL_REG: c_uint = 0x37;
pub const AS3722_WATCHDOG_CONTROL_REG: c_uint = 0x38;
pub const AS3722_REG_STANDBY_MOD1_REG: c_uint = 0x39;
pub const AS3722_REG_STANDBY_MOD2_REG: c_uint = 0x3A;
pub const AS3722_REG_STANDBY_MOD3_REG: c_uint = 0x3B;
pub const AS3722_ENABLE_CTRL1_REG: c_uint = 0x3C;
pub const AS3722_ENABLE_CTRL2_REG: c_uint = 0x3D;
pub const AS3722_ENABLE_CTRL3_REG: c_uint = 0x3E;
pub const AS3722_ENABLE_CTRL4_REG: c_uint = 0x3F;
pub const AS3722_ENABLE_CTRL5_REG: c_uint = 0x40;
pub const AS3722_PWM_CONTROL_L_REG: c_uint = 0x41;
pub const AS3722_PWM_CONTROL_H_REG: c_uint = 0x42;
pub const AS3722_WATCHDOG_TIMER_REG: c_uint = 0x46;
pub const AS3722_WATCHDOG_SOFTWARE_SIGNAL_REG: c_uint = 0x48;
pub const AS3722_IOVOLTAGE_REG: c_uint = 0x49;
pub const AS3722_BATTERY_VOLTAGE_MONITOR2_REG: c_uint = 0x4A;
pub const AS3722_SD_CONTROL_REG: c_uint = 0x4D;
pub const AS3722_LDOCONTROL0_REG: c_uint = 0x4E;
pub const AS3722_LDOCONTROL1_REG: c_uint = 0x4F;
pub const AS3722_SD0_PROTECT_REG: c_uint = 0x50;
pub const AS3722_SD6_PROTECT_REG: c_uint = 0x51;
pub const AS3722_PWM_VCONTROL1_REG: c_uint = 0x52;
pub const AS3722_PWM_VCONTROL2_REG: c_uint = 0x53;
pub const AS3722_PWM_VCONTROL3_REG: c_uint = 0x54;
pub const AS3722_PWM_VCONTROL4_REG: c_uint = 0x55;
pub const AS3722_BB_CHARGER_REG: c_uint = 0x57;
pub const AS3722_CTRL_SEQU1_REG: c_uint = 0x58;
pub const AS3722_CTRL_SEQU2_REG: c_uint = 0x59;
pub const AS3722_OVCURRENT_REG: c_uint = 0x5A;
pub const AS3722_OVCURRENT_DEB_REG: c_uint = 0x5B;
pub const AS3722_SDLV_DEB_REG: c_uint = 0x5C;
pub const AS3722_OC_PG_CTRL_REG: c_uint = 0x5D;
pub const AS3722_OC_PG_CTRL2_REG: c_uint = 0x5E;
pub const AS3722_CTRL_STATUS: c_uint = 0x5F;
pub const AS3722_RTC_CONTROL_REG: c_uint = 0x60;
pub const AS3722_RTC_SECOND_REG: c_uint = 0x61;
pub const AS3722_RTC_MINUTE_REG: c_uint = 0x62;
pub const AS3722_RTC_HOUR_REG: c_uint = 0x63;
pub const AS3722_RTC_DAY_REG: c_uint = 0x64;
pub const AS3722_RTC_MONTH_REG: c_uint = 0x65;
pub const AS3722_RTC_YEAR_REG: c_uint = 0x66;
pub const AS3722_RTC_ALARM_SECOND_REG: c_uint = 0x67;
pub const AS3722_RTC_ALARM_MINUTE_REG: c_uint = 0x68;
pub const AS3722_RTC_ALARM_HOUR_REG: c_uint = 0x69;
pub const AS3722_RTC_ALARM_DAY_REG: c_uint = 0x6A;
pub const AS3722_RTC_ALARM_MONTH_REG: c_uint = 0x6B;
pub const AS3722_RTC_ALARM_YEAR_REG: c_uint = 0x6C;
pub const AS3722_SRAM_REG: c_uint = 0x6D;
pub const AS3722_RTC_ACCESS_REG: c_uint = 0x6F;
pub const AS3722_RTC_STATUS_REG: c_uint = 0x73;
pub const AS3722_INTERRUPT_MASK1_REG: c_uint = 0x74;
pub const AS3722_INTERRUPT_MASK2_REG: c_uint = 0x75;
pub const AS3722_INTERRUPT_MASK3_REG: c_uint = 0x76;
pub const AS3722_INTERRUPT_MASK4_REG: c_uint = 0x77;
pub const AS3722_INTERRUPT_STATUS1_REG: c_uint = 0x78;
pub const AS3722_INTERRUPT_STATUS2_REG: c_uint = 0x79;
pub const AS3722_INTERRUPT_STATUS3_REG: c_uint = 0x7A;
pub const AS3722_INTERRUPT_STATUS4_REG: c_uint = 0x7B;
pub const AS3722_TEMP_STATUS_REG: c_uint = 0x7D;
pub const AS3722_ADC0_CONTROL_REG: c_uint = 0x80;
pub const AS3722_ADC1_CONTROL_REG: c_uint = 0x81;
pub const AS3722_ADC0_MSB_RESULT_REG: c_uint = 0x82;
pub const AS3722_ADC0_LSB_RESULT_REG: c_uint = 0x83;
pub const AS3722_ADC1_MSB_RESULT_REG: c_uint = 0x84;
pub const AS3722_ADC1_LSB_RESULT_REG: c_uint = 0x85;
pub const AS3722_ADC1_THRESHOLD_HI_MSB_REG: c_uint = 0x86;
pub const AS3722_ADC1_THRESHOLD_HI_LSB_REG: c_uint = 0x87;
pub const AS3722_ADC1_THRESHOLD_LO_MSB_REG: c_uint = 0x88;
pub const AS3722_ADC1_THRESHOLD_LO_LSB_REG: c_uint = 0x89;
pub const AS3722_ADC_CONFIGURATION_REG: c_uint = 0x8A;
pub const AS3722_ASIC_ID1_REG: c_uint = 0x90;
pub const AS3722_ASIC_ID2_REG: c_uint = 0x91;
pub const AS3722_LOCK_REG: c_uint = 0x9E;
pub const AS3722_FUSE7_REG: c_uint = 0xA7;
pub const AS3722_MAX_REGISTER: c_uint = 0xF4;
pub const AS3722_SD0_EXT_ENABLE_MASK: c_uint = 0x03;
pub const AS3722_SD1_EXT_ENABLE_MASK: c_uint = 0x0C;
pub const AS3722_SD2_EXT_ENABLE_MASK: c_uint = 0x30;
pub const AS3722_SD3_EXT_ENABLE_MASK: c_uint = 0xC0;
pub const AS3722_SD4_EXT_ENABLE_MASK: c_uint = 0x03;
pub const AS3722_SD5_EXT_ENABLE_MASK: c_uint = 0x0C;
pub const AS3722_SD6_EXT_ENABLE_MASK: c_uint = 0x30;
pub const AS3722_LDO0_EXT_ENABLE_MASK: c_uint = 0x03;
pub const AS3722_LDO1_EXT_ENABLE_MASK: c_uint = 0x0C;
pub const AS3722_LDO2_EXT_ENABLE_MASK: c_uint = 0x30;
pub const AS3722_LDO3_EXT_ENABLE_MASK: c_uint = 0xC0;
pub const AS3722_LDO4_EXT_ENABLE_MASK: c_uint = 0x03;
pub const AS3722_LDO5_EXT_ENABLE_MASK: c_uint = 0x0C;
pub const AS3722_LDO6_EXT_ENABLE_MASK: c_uint = 0x30;
pub const AS3722_LDO7_EXT_ENABLE_MASK: c_uint = 0xC0;
pub const AS3722_LDO9_EXT_ENABLE_MASK: c_uint = 0x0C;
pub const AS3722_LDO10_EXT_ENABLE_MASK: c_uint = 0x30;
pub const AS3722_LDO11_EXT_ENABLE_MASK: c_uint = 0xC0;
pub const AS3722_OVCURRENT_SD0_ALARM_MASK: c_uint = 0x07;
pub const AS3722_OVCURRENT_SD0_ALARM_SHIFT: c_uint = 0x01;
pub const AS3722_OVCURRENT_SD0_TRIP_MASK: c_uint = 0x18;
pub const AS3722_OVCURRENT_SD0_TRIP_SHIFT: c_uint = 0x03;
pub const AS3722_OVCURRENT_SD1_TRIP_MASK: c_uint = 0x60;
pub const AS3722_OVCURRENT_SD1_TRIP_SHIFT: c_uint = 0x05;
pub const AS3722_OVCURRENT_SD6_ALARM_MASK: c_uint = 0x07;
pub const AS3722_OVCURRENT_SD6_ALARM_SHIFT: c_uint = 0x01;
pub const AS3722_OVCURRENT_SD6_TRIP_MASK: c_uint = 0x18;
pub const AS3722_OVCURRENT_SD6_TRIP_SHIFT: c_uint = 0x03;
// AS3722 register bits and bit masks

pub const AS3722_LDO0_VSEL_MASK: c_uint = 0x1F;
pub const AS3722_LDO0_VSEL_MIN: c_uint = 0x01;
pub const AS3722_LDO0_VSEL_MAX: c_uint = 0x12;
pub const AS3722_LDO0_NUM_VOLT: c_uint = 0x12;
pub const AS3722_LDO3_VSEL_MASK: c_uint = 0x3F;
pub const AS3722_LDO3_VSEL_MIN: c_uint = 0x01;
pub const AS3722_LDO3_VSEL_MAX: c_uint = 0x2D;
pub const AS3722_LDO3_NUM_VOLT: c_uint = 0x2D;
pub const AS3722_LDO6_VSEL_BYPASS: c_uint = 0x3F;
pub const AS3722_LDO_VSEL_MASK: c_uint = 0x7F;
pub const AS3722_LDO_VSEL_MIN: c_uint = 0x01;
pub const AS3722_LDO_VSEL_MAX: c_uint = 0x7F;
pub const AS3722_LDO_VSEL_DNU_MIN: c_uint = 0x25;
pub const AS3722_LDO_VSEL_DNU_MAX: c_uint = 0x3F;
pub const AS3722_LDO_NUM_VOLT: c_uint = 0x80;

pub const AS3722_SD_VSEL_MASK: c_uint = 0x7F;
pub const AS3722_SD0_VSEL_MIN: c_uint = 0x01;
pub const AS3722_SD0_VSEL_MAX: c_uint = 0x5A;
pub const AS3722_SD0_VSEL_LOW_VOL_MAX: c_uint = 0x6E;
pub const AS3722_SD2_VSEL_MIN: c_uint = 0x01;
pub const AS3722_SD2_VSEL_MAX: c_uint = 0x7F;

pub const AS3722_ADC_MSB_VAL_MASK: c_uint = 0x7F;
pub const AS3722_ADC_LSB_VAL_MASK: c_uint = 0x07;

pub const AS3722_ADC0_SOURCE_SELECT_MASK: c_uint = 0x1F;

pub const AS3722_ADC1_SOURCE_SELECT_MASK: c_uint = 0x1F;

// GPIO modes
pub const AS3722_GPIO_MODE_MASK: c_uint = 0x07;
pub const AS3722_GPIO_MODE_INPUT: c_uint = 0x00;
pub const AS3722_GPIO_MODE_OUTPUT_VDDH: c_uint = 0x01;
pub const AS3722_GPIO_MODE_IO_OPEN_DRAIN: c_uint = 0x02;
pub const AS3722_GPIO_MODE_ADC_IN: c_uint = 0x03;
pub const AS3722_GPIO_MODE_INPUT_PULL_UP: c_uint = 0x04;
pub const AS3722_GPIO_MODE_INPUT_PULL_DOWN: c_uint = 0x05;
pub const AS3722_GPIO_MODE_IO_OPEN_DRAIN_PULL_UP: c_uint = 0x06;
pub const AS3722_GPIO_MODE_OUTPUT_VDDL: c_uint = 0x07;

pub const AS3722_GPIO_IOSF_MASK: c_uint = 0x78;

pub const AS3722_WATCHDOG_TIMER_MAX: c_uint = 0x7F;

pub const AS3722_EXT_CONTROL_ENABLE1: c_uint = 0x1;
pub const AS3722_EXT_CONTROL_ENABLE2: c_uint = 0x2;
pub const AS3722_EXT_CONTROL_ENABLE3: c_uint = 0x3;

// Interrupt IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum as3722_irq {
    AS3722_IRQ_LID,
    AS3722_IRQ_ACOK,
    AS3722_IRQ_ENABLE1,
    AS3722_IRQ_OCCUR_ALARM_SD0,
    AS3722_IRQ_ONKEY_LONG_PRESS,
    AS3722_IRQ_ONKEY,
    AS3722_IRQ_OVTMP,
    AS3722_IRQ_LOWBAT,
    AS3722_IRQ_SD0_LV,
    AS3722_IRQ_SD1_LV,
    AS3722_IRQ_SD2_LV,
    AS3722_IRQ_PWM1_OV_PROT,
    AS3722_IRQ_PWM2_OV_PROT,
    AS3722_IRQ_ENABLE2,
    AS3722_IRQ_SD6_LV,
    AS3722_IRQ_RTC_REP,
    AS3722_IRQ_RTC_ALARM,
    AS3722_IRQ_GPIO1,
    AS3722_IRQ_GPIO2,
    AS3722_IRQ_GPIO3,
    AS3722_IRQ_GPIO4,
    AS3722_IRQ_GPIO5,
    AS3722_IRQ_WATCHDOG,
    AS3722_IRQ_ENABLE3,
    AS3722_IRQ_TEMP_SD0_SHUTDOWN,
    AS3722_IRQ_TEMP_SD1_SHUTDOWN,
    AS3722_IRQ_TEMP_SD2_SHUTDOWN,
    AS3722_IRQ_TEMP_SD0_ALARM,
    AS3722_IRQ_TEMP_SD1_ALARM,
    AS3722_IRQ_TEMP_SD6_ALARM,
    AS3722_IRQ_OCCUR_ALARM_SD6,
    AS3722_IRQ_ADC,
    AS3722_IRQ_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3722 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub chip_irq: c_int,
    pub irq_flags: c_ulong,
    pub en_intern_int_pullup: bool,
    pub en_intern_i2c_pullup: bool,
    pub en_ac_ok_pwr_on: bool,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn regmap_read(_arg: as3722->regmap, _arg: reg, _arg: dest) -> return;
}
extern "C" {
    pub fn regmap_write(_arg: as3722->regmap, _arg: reg, _arg: value) -> return;
}
extern "C" {
    pub fn regmap_bulk_read(_arg: as3722->regmap, _arg: reg, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn regmap_bulk_write(_arg: as3722->regmap, _arg: reg, _arg: data, _arg: count) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: as3722->regmap, _arg: reg, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn regmap_irq_get_virq(_arg: as3722->irq_data, _arg: irq) -> return;
}
