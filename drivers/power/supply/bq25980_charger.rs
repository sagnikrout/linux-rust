//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/bq25980_charger.h
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
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com/

pub const BQ25980_BATOVP: c_uint = 0x0;
pub const BQ25980_BATOVP_ALM: c_uint = 0x1;
pub const BQ25980_BATOCP: c_uint = 0x2;
pub const BQ25980_BATOCP_ALM: c_uint = 0x3;
pub const BQ25980_BATUCP_ALM: c_uint = 0x4;
pub const BQ25980_CHRGR_CTRL_1: c_uint = 0x5;
pub const BQ25980_BUSOVP: c_uint = 0x6;
pub const BQ25980_BUSOVP_ALM: c_uint = 0x7;
pub const BQ25980_BUSOCP: c_uint = 0x8;
pub const BQ25980_BUSOCP_ALM: c_uint = 0x9;
pub const BQ25980_TEMP_CONTROL: c_uint = 0xA;
pub const BQ25980_TDIE_ALM: c_uint = 0xB;
pub const BQ25980_TSBUS_FLT: c_uint = 0xC;
pub const BQ25980_TSBAT_FLG: c_uint = 0xD;
pub const BQ25980_VAC_CONTROL: c_uint = 0xE;
pub const BQ25980_CHRGR_CTRL_2: c_uint = 0xF;
pub const BQ25980_CHRGR_CTRL_3: c_uint = 0x10;
pub const BQ25980_CHRGR_CTRL_4: c_uint = 0x11;
pub const BQ25980_CHRGR_CTRL_5: c_uint = 0x12;
pub const BQ25980_STAT1: c_uint = 0x13;
pub const BQ25980_STAT2: c_uint = 0x14;
pub const BQ25980_STAT3: c_uint = 0x15;
pub const BQ25980_STAT4: c_uint = 0x16;
pub const BQ25980_STAT5: c_uint = 0x17;
pub const BQ25980_FLAG1: c_uint = 0x18;
pub const BQ25980_FLAG2: c_uint = 0x19;
pub const BQ25980_FLAG3: c_uint = 0x1A;
pub const BQ25980_FLAG4: c_uint = 0x1B;
pub const BQ25980_FLAG5: c_uint = 0x1C;
pub const BQ25980_MASK1: c_uint = 0x1D;
pub const BQ25980_MASK2: c_uint = 0x1E;
pub const BQ25980_MASK3: c_uint = 0x1F;
pub const BQ25980_MASK4: c_uint = 0x20;
pub const BQ25980_MASK5: c_uint = 0x21;
pub const BQ25980_DEVICE_INFO: c_uint = 0x22;
pub const BQ25980_ADC_CONTROL1: c_uint = 0x23;
pub const BQ25980_ADC_CONTROL2: c_uint = 0x24;
pub const BQ25980_IBUS_ADC_MSB: c_uint = 0x25;
pub const BQ25980_IBUS_ADC_LSB: c_uint = 0x26;
pub const BQ25980_VBUS_ADC_MSB: c_uint = 0x27;
pub const BQ25980_VBUS_ADC_LSB: c_uint = 0x28;
pub const BQ25980_VAC1_ADC_MSB: c_uint = 0x29;
pub const BQ25980_VAC1_ADC_LSB: c_uint = 0x2A;
pub const BQ25980_VAC2_ADC_MSB: c_uint = 0x2B;
pub const BQ25980_VAC2_ADC_LSB: c_uint = 0x2C;
pub const BQ25980_VOUT_ADC_MSB: c_uint = 0x2D;
pub const BQ25980_VOUT_ADC_LSB: c_uint = 0x2E;
pub const BQ25980_VBAT_ADC_MSB: c_uint = 0x2F;
pub const BQ25980_VBAT_ADC_LSB: c_uint = 0x30;
pub const BQ25980_IBAT_ADC_MSB: c_uint = 0x31;
pub const BQ25980_IBAT_ADC_LSB: c_uint = 0x32;
pub const BQ25980_TSBUS_ADC_MSB: c_uint = 0x33;
pub const BQ25980_TSBUS_ADC_LSB: c_uint = 0x34;
pub const BQ25980_TSBAT_ADC_MSB: c_uint = 0x35;
pub const BQ25980_TSBAT_ADC_LSB: c_uint = 0x36;
pub const BQ25980_TDIE_ADC_MSB: c_uint = 0x37;
pub const BQ25980_TDIE_ADC_LSB: c_uint = 0x38;
pub const BQ25980_DEGLITCH_TIME: c_uint = 0x39;
pub const BQ25980_CHRGR_CTRL_6: c_uint = 0x3A;
pub const BQ25980_BUSOCP_STEP_uA: c_int = 250000;
pub const BQ25980_BUSOCP_OFFSET_uA: c_int = 1000000;
pub const BQ25980_BUSOCP_DFLT_uA: c_int = 4250000;
pub const BQ25975_BUSOCP_DFLT_uA: c_int = 4250000;
pub const BQ25960_BUSOCP_DFLT_uA: c_int = 3250000;
pub const BQ25980_BUSOCP_MIN_uA: c_int = 1000000;
pub const BQ25980_BUSOCP_SC_MAX_uA: c_int = 5750000;
pub const BQ25975_BUSOCP_SC_MAX_uA: c_int = 5750000;
pub const BQ25960_BUSOCP_SC_MAX_uA: c_int = 3750000;
pub const BQ25980_BUSOCP_BYP_MAX_uA: c_int = 8500000;
pub const BQ25975_BUSOCP_BYP_MAX_uA: c_int = 8500000;
pub const BQ25960_BUSOCP_BYP_MAX_uA: c_int = 5750000;
pub const BQ25980_BUSOVP_SC_STEP_uV: c_int = 100000;
pub const BQ25975_BUSOVP_SC_STEP_uV: c_int = 50000;
pub const BQ25960_BUSOVP_SC_STEP_uV: c_int = 50000;
pub const BQ25980_BUSOVP_SC_OFFSET_uV: c_int = 14000000;
pub const BQ25975_BUSOVP_SC_OFFSET_uV: c_int = 7000000;
pub const BQ25960_BUSOVP_SC_OFFSET_uV: c_int = 7000000;
pub const BQ25980_BUSOVP_BYP_STEP_uV: c_int = 50000;
pub const BQ25975_BUSOVP_BYP_STEP_uV: c_int = 25000;
pub const BQ25960_BUSOVP_BYP_STEP_uV: c_int = 25000;
pub const BQ25980_BUSOVP_BYP_OFFSET_uV: c_int = 7000000;
pub const BQ25975_BUSOVP_BYP_OFFSET_uV: c_int = 3500000;
pub const BQ25960_BUSOVP_BYP_OFFSET_uV: c_int = 3500000;
pub const BQ25980_BUSOVP_DFLT_uV: c_int = 17800000;
pub const BQ25980_BUSOVP_BYPASS_DFLT_uV: c_int = 8900000;
pub const BQ25975_BUSOVP_DFLT_uV: c_int = 8900000;
pub const BQ25975_BUSOVP_BYPASS_DFLT_uV: c_int = 4450000;
pub const BQ25960_BUSOVP_DFLT_uV: c_int = 8900000;
pub const BQ25980_BUSOVP_SC_MIN_uV: c_int = 14000000;
pub const BQ25975_BUSOVP_SC_MIN_uV: c_int = 7000000;
pub const BQ25960_BUSOVP_SC_MIN_uV: c_int = 7000000;
pub const BQ25980_BUSOVP_BYP_MIN_uV: c_int = 7000000;
pub const BQ25975_BUSOVP_BYP_MIN_uV: c_int = 3500000;
pub const BQ25960_BUSOVP_BYP_MIN_uV: c_int = 3500000;
pub const BQ25980_BUSOVP_SC_MAX_uV: c_int = 22000000;
pub const BQ25975_BUSOVP_SC_MAX_uV: c_int = 12750000;
pub const BQ25960_BUSOVP_SC_MAX_uV: c_int = 12750000;
pub const BQ25980_BUSOVP_BYP_MAX_uV: c_int = 12750000;
pub const BQ25975_BUSOVP_BYP_MAX_uV: c_int = 6500000;
pub const BQ25960_BUSOVP_BYP_MAX_uV: c_int = 6500000;
pub const BQ25980_BATOVP_STEP_uV: c_int = 20000;
pub const BQ25975_BATOVP_STEP_uV: c_int = 10000;
pub const BQ25960_BATOVP_STEP_uV: c_int = 10000;
pub const BQ25980_BATOVP_OFFSET_uV: c_int = 7000000;
pub const BQ25975_BATOVP_OFFSET_uV: c_int = 3500000;
pub const BQ25960_BATOVP_OFFSET_uV: c_int = 3500000;
pub const BQ25980_BATOVP_DFLT_uV: c_int = 14000000;
pub const BQ25975_BATOVP_DFLT_uV: c_int = 8900000;
pub const BQ25960_BATOVP_DFLT_uV: c_int = 8900000;
pub const BQ25980_BATOVP_MIN_uV: c_int = 7000000;
pub const BQ25975_BATOVP_MIN_uV: c_int = 3500000;
pub const BQ25960_BATOVP_MIN_uV: c_int = 3500000;
pub const BQ25980_BATOVP_MAX_uV: c_int = 9540000;
pub const BQ25975_BATOVP_MAX_uV: c_int = 4770000;
pub const BQ25960_BATOVP_MAX_uV: c_int = 4770000;
pub const BQ25980_BATOCP_STEP_uA: c_int = 100000;

pub const BQ25980_BATOCP_DFLT_uA: c_int = 8100000;
pub const BQ25960_BATOCP_DFLT_uA: c_int = 6100000;
pub const BQ25980_BATOCP_MIN_uA: c_int = 2000000;
pub const BQ25980_BATOCP_MAX_uA: c_int = 11000000;
pub const BQ25975_BATOCP_MAX_uA: c_int = 11000000;
pub const BQ25960_BATOCP_MAX_uA: c_int = 7000000;
pub const BQ25980_ENABLE_HIZ: c_uint = 0xff;
pub const BQ25980_DISABLE_HIZ: c_uint = 0x0;

pub const BQ25980_ADC_VOLT_STEP_uV: c_int = 1000;
pub const BQ25980_ADC_CURR_STEP_uA: c_int = 1000;

pub const BQ25980_WATCHDOG_MAX: c_int = 300000;
pub const BQ25980_WATCHDOG_MIN: c_int = 0;
pub const BQ25980_NUM_WD_VAL: c_int = 4;
