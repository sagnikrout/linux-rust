//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/da9121-regulator.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// DA9121 Single-channel dual-phase 10A buck converter
// DA9130 Single-channel dual-phase 10A buck converter (Automotive)
// DA9217 Single-channel dual-phase  6A buck converter
// DA9122 Dual-channel single-phase  5A buck converter
// DA9131 Dual-channel single-phase  5A buck converter (Automotive)
// DA9220 Dual-channel single-phase  3A buck converter
// DA9132 Dual-channel single-phase  3A buck converter (Automotive)
//
// Copyright (C) 2020  Dialog Semiconductor
//
// Authors: Steve Twiss, Dialog Semiconductor
// Adam Ward, Dialog Semiconductor
//
// Values for: DA9121_REG_BUCK_BUCKx_4 registers, fields CHx_y_MODE
// DA9121_REG_BUCK_BUCKx_7 registers, fields CHx_RIPPLE_CANCEL
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9121_variant {
    DA9121_TYPE_DA9121_DA9130,
    DA9121_TYPE_DA9220_DA9132,
    DA9121_TYPE_DA9122_DA9131,
    DA9121_TYPE_DA9217,
    DA9121_TYPE_DA9141,
    DA9121_TYPE_DA9142
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9121_subvariant {
    DA9121_SUBTYPE_DA9121,
    DA9121_SUBTYPE_DA9130,
    DA9121_SUBTYPE_DA9220,
    DA9121_SUBTYPE_DA9132,
    DA9121_SUBTYPE_DA9122,
    DA9121_SUBTYPE_DA9131,
    DA9121_SUBTYPE_DA9217,
    DA9121_SUBTYPE_DA9141,
    DA9121_SUBTYPE_DA9142
}

// Minimum, maximum and default polling millisecond periods are provided
// here as an example. It is expected that any final implementation will
// include a modification of these settings to match the required
// application.
//
pub const DA9121_DEFAULT_POLLING_PERIOD_MS: c_int = 3000;
pub const DA9121_MAX_POLLING_PERIOD_MS: c_int = 10000;
pub const DA9121_MIN_POLLING_PERIOD_MS: c_int = 1000;
// Registers
pub const DA9121_REG_SYS_STATUS_0: c_uint = 0x01;
pub const DA9121_REG_SYS_STATUS_1: c_uint = 0x02;
pub const DA9121_REG_SYS_STATUS_2: c_uint = 0x03;
pub const DA9121_REG_SYS_EVENT_0: c_uint = 0x04;
pub const DA9121_REG_SYS_EVENT_1: c_uint = 0x05;
pub const DA9121_REG_SYS_EVENT_2: c_uint = 0x06;
pub const DA9121_REG_SYS_MASK_0: c_uint = 0x07;
pub const DA9121_REG_SYS_MASK_1: c_uint = 0x08;
pub const DA9121_REG_SYS_MASK_2: c_uint = 0x09;
pub const DA9121_REG_SYS_MASK_3: c_uint = 0x0A;
pub const DA9121_REG_SYS_CONFIG_0: c_uint = 0x0B;
pub const DA9121_REG_SYS_CONFIG_1: c_uint = 0x0C;
pub const DA9121_REG_SYS_CONFIG_2: c_uint = 0x0D;
pub const DA9121_REG_SYS_CONFIG_3: c_uint = 0x0E;
pub const DA9121_REG_SYS_GPIO0_0: c_uint = 0x10;
pub const DA9121_REG_SYS_GPIO0_1: c_uint = 0x11;
pub const DA9121_REG_SYS_GPIO1_0: c_uint = 0x12;
pub const DA9121_REG_SYS_GPIO1_1: c_uint = 0x13;
pub const DA9121_REG_SYS_GPIO2_0: c_uint = 0x14;
pub const DA9121_REG_SYS_GPIO2_1: c_uint = 0x15;
pub const DA914x_REG_SYS_GPIO3_0: c_uint = 0x16;
pub const DA914x_REG_SYS_GPIO3_1: c_uint = 0x17;
pub const DA914x_REG_SYS_GPIO4_0: c_uint = 0x18;
pub const DA914x_REG_SYS_GPIO4_1: c_uint = 0x19;
pub const DA914x_REG_SYS_ADMUX1_0: c_uint = 0x1A;
pub const DA914x_REG_SYS_ADMUX1_1: c_uint = 0x1B;
pub const DA914x_REG_SYS_ADMUX2_0: c_uint = 0x1C;
pub const DA914x_REG_SYS_ADMUX2_1: c_uint = 0x1D;
pub const DA9121_REG_BUCK_BUCK1_0: c_uint = 0x20;
pub const DA9121_REG_BUCK_BUCK1_1: c_uint = 0x21;
pub const DA9121_REG_BUCK_BUCK1_2: c_uint = 0x22;
pub const DA9121_REG_BUCK_BUCK1_3: c_uint = 0x23;
pub const DA9121_REG_BUCK_BUCK1_4: c_uint = 0x24;
pub const DA9121_REG_BUCK_BUCK1_5: c_uint = 0x25;
pub const DA9121_REG_BUCK_BUCK1_6: c_uint = 0x26;
pub const DA9121_REG_BUCK_BUCK1_7: c_uint = 0x27;
pub const DA9xxx_REG_BUCK_BUCK2_0: c_uint = 0x28;
pub const DA9xxx_REG_BUCK_BUCK2_1: c_uint = 0x29;
pub const DA9xxx_REG_BUCK_BUCK2_2: c_uint = 0x2A;
pub const DA9xxx_REG_BUCK_BUCK2_3: c_uint = 0x2B;
pub const DA9xxx_REG_BUCK_BUCK2_4: c_uint = 0x2C;
pub const DA9xxx_REG_BUCK_BUCK2_5: c_uint = 0x2D;
pub const DA9xxx_REG_BUCK_BUCK2_6: c_uint = 0x2E;
pub const DA9xxx_REG_BUCK_BUCK2_7: c_uint = 0x2F;
pub const DA9121_REG_OTP_DEVICE_ID: c_uint = 0x48;
pub const DA9121_REG_OTP_VARIANT_ID: c_uint = 0x49;
pub const DA9121_REG_OTP_CUSTOMER_ID: c_uint = 0x4A;
pub const DA9121_REG_OTP_CONFIG_ID: c_uint = 0x4B;
// Register bits
// DA9121_REG_SYS_STATUS_0

// DA9121_REG_SYS_STATUS_1

// DA9121_REG_SYS_STATUS_2

// DA9121_REG_SYS_EVENT_0

// DA9121_REG_SYS_EVENT_1

// DA9121_REG_SYS_EVENT_2

// DA9121_REG_SYS_MASK_0

// DA9121_REG_SYS_MASK_1

// DA9121_REG_SYS_MASK_2

// DA9122_REG_SYS_MASK_3

// DA9121_REG_SYS_CONFIG_0
pub const DA9121_MASK_SYS_CONFIG_0_CH1_DIS_DLY: c_uint = 0xF0;
pub const DA9121_MASK_SYS_CONFIG_0_CH1_EN_DLY: c_uint = 0x0F;
// DA9xxx_REG_SYS_CONFIG_1
pub const DA9xxx_MASK_SYS_CONFIG_1_CH2_DIS_DLY: c_uint = 0xF0;
pub const DA9xxx_MASK_SYS_CONFIG_1_CH2_EN_DLY: c_uint = 0x0F;
// DA9121_REG_SYS_CONFIG_2
pub const DA9121_MASK_SYS_CONFIG_2_OC_LATCHOFF: c_uint = 0x60;

pub const DA9121_MASK_SYS_CONFIG_2_PG_DVC_MASK: c_uint = 0x0C;
// DA9121_REG_SYS_CONFIG_3

// DA9121_REG_SYS_GPIO0_0

// DA9121_REG_SYS_GPIO0_1

pub const DA9121_MASK_SYS_GPIO0_1_GPIO0_DEB: c_uint = 0x30;

pub const DA9121_MASK_SYS_GPIO0_1_GPIO0_TRIG: c_uint = 0x03;
// DA9121_REG_SYS_GPIO1_0
pub const DA9121_MASK_SYS_GPIO1_0_GPIO1_MODE: c_uint = 0x1E;

// DA9121_REG_SYS_GPIO1_1

pub const DA9121_MASK_SYS_GPIO1_1_GPIO1_DEB: c_uint = 0x30;

pub const DA9121_MASK_SYS_GPIO1_1_GPIO1_TRIG: c_uint = 0x03;
// DA9121_REG_SYS_GPIO2_0
pub const DA9121_MASK_SYS_GPIO2_0_GPIO2_MODE: c_uint = 0x1E;

// DA9121_REG_SYS_GPIO2_1

pub const DA9121_MASK_SYS_GPIO2_1_GPIO2_DEB: c_uint = 0x30;

pub const DA9121_MASK_SYS_GPIO2_1_GPIO2_TRIG: c_uint = 0x03;
// DA9121_REG_BUCK_BUCK1_0 / DA9xxx_REG_BUCK_BUCK2_0
pub const DA9121_MASK_BUCK_BUCKx_0_CHx_SR_DVC_DWN: c_uint = 0x70;
pub const DA9121_MASK_BUCK_BUCKx_0_CHx_SR_DVC_UP: c_uint = 0x0E;

// DA9121_REG_BUCK_BUCK1_1 / DA9xxx_REG_BUCK_BUCK2_1
pub const DA9121_MASK_BUCK_BUCKx_1_CHx_SR_SHDN: c_uint = 0x70;
pub const DA9121_MASK_BUCK_BUCKx_1_CHx_SR_STARTUP: c_uint = 0x0E;

// DA9121_REG_BUCK_BUCK1_2 / DA9xxx_REG_BUCK_BUCK2_2
pub const DA9121_MASK_BUCK_BUCKx_2_CHx_ILIM: c_uint = 0x0F;
// DA9121_REG_BUCK_BUCK1_3 / DA9xxx_REG_BUCK_BUCK2_3
pub const DA9121_MASK_BUCK_BUCKx_3_CHx_VMAX: c_uint = 0xFF;
// DA9121_REG_BUCK_BUCK1_4 / DA9xxx_REG_BUCK_BUCK2_4

pub const DA9121_MASK_BUCK_BUCKx_4_CHx_B_MODE: c_uint = 0x0C;
pub const DA9121_MASK_BUCK_BUCKx_4_CHx_A_MODE: c_uint = 0x03;
// DA9121_REG_BUCK_BUCK1_5 / DA9xxx_REG_BUCK_BUCK2_5
pub const DA9121_MASK_BUCK_BUCKx_5_CHx_A_VOUT: c_uint = 0xFF;
// DA9121_REG_BUCK_BUCK1_6 / DA9xxx_REG_BUCK_BUCK2_6
pub const DA9121_MASK_BUCK_BUCKx_6_CHx_B_VOUT: c_uint = 0xFF;
// DA9121_REG_BUCK_BUCK1_7 / DA9xxx_REG_BUCK_BUCK2_7
pub const DA9xxx_MASK_BUCK_BUCKx_7_CHx_RIPPLE_CANCEL: c_uint = 0x03;
// DA9121_REG_OTP_DEVICE_ID
pub const DA9121_MASK_OTP_DEVICE_ID_DEV_ID: c_uint = 0xFF;
pub const DA9121_DEVICE_ID: c_uint = 0x05;
pub const DA914x_DEVICE_ID: c_uint = 0x26;
// DA9121_REG_OTP_VARIANT_ID
pub const DA9121_SHIFT_OTP_VARIANT_ID_MRC: c_int = 4;
pub const DA9121_MASK_OTP_VARIANT_ID_MRC: c_uint = 0xF0;
pub const DA9121_SHIFT_OTP_VARIANT_ID_VRC: c_int = 0;
pub const DA9121_MASK_OTP_VARIANT_ID_VRC: c_uint = 0x0F;
pub const DA9121_VARIANT_MRC_BASE: c_uint = 0x2;
pub const DA9121_VARIANT_VRC: c_uint = 0x1;
pub const DA9220_VARIANT_VRC: c_uint = 0x0;
pub const DA9122_VARIANT_VRC: c_uint = 0x2;
pub const DA9217_VARIANT_VRC: c_uint = 0x7;
pub const DA9130_VARIANT_VRC: c_uint = 0x0;
pub const DA9131_VARIANT_VRC: c_uint = 0x1;
pub const DA9132_VARIANT_VRC: c_uint = 0x2;
pub const DA914x_VARIANT_MRC_BASE: c_uint = 0x0;
pub const DA9141_VARIANT_VRC: c_uint = 0x1;
pub const DA9142_VARIANT_VRC: c_uint = 0x2;
// DA9121_REG_OTP_CUSTOMER_ID
pub const DA9121_MASK_OTP_CUSTOMER_ID_CUST_ID: c_uint = 0xFF;
// DA9121_REG_OTP_CONFIG_ID
pub const DA9121_MASK_OTP_CONFIG_ID_CONFIG_REV: c_uint = 0xFF;
