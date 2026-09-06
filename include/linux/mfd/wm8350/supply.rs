//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/supply.h
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
// supply.h  --  Power Supply Driver for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//

//
// Charger registers
//
pub const WM8350_BATTERY_CHARGER_CONTROL_1: c_uint = 0xA8;
pub const WM8350_BATTERY_CHARGER_CONTROL_2: c_uint = 0xA9;
pub const WM8350_BATTERY_CHARGER_CONTROL_3: c_uint = 0xAA;
//
// R168 (0xA8) - Battery Charger Control 1
//
pub const WM8350_CHG_ENA_R168: c_uint = 0x8000;
pub const WM8350_CHG_THR: c_uint = 0x2000;
pub const WM8350_CHG_EOC_SEL_MASK: c_uint = 0x1C00;
pub const WM8350_CHG_TRICKLE_TEMP_CHOKE: c_uint = 0x0200;
pub const WM8350_CHG_TRICKLE_USB_CHOKE: c_uint = 0x0100;
pub const WM8350_CHG_RECOVER_T: c_uint = 0x0080;
pub const WM8350_CHG_END_ACT: c_uint = 0x0040;
pub const WM8350_CHG_FAST: c_uint = 0x0020;
pub const WM8350_CHG_FAST_USB_THROTTLE: c_uint = 0x0010;
pub const WM8350_CHG_NTC_MON: c_uint = 0x0008;
pub const WM8350_CHG_BATT_HOT_MON: c_uint = 0x0004;
pub const WM8350_CHG_BATT_COLD_MON: c_uint = 0x0002;
pub const WM8350_CHG_CHIP_TEMP_MON: c_uint = 0x0001;
//
// R169 (0xA9) - Battery Charger Control 2
//
pub const WM8350_CHG_ACTIVE: c_uint = 0x8000;
pub const WM8350_CHG_PAUSE: c_uint = 0x4000;
pub const WM8350_CHG_STS_MASK: c_uint = 0x3000;
pub const WM8350_CHG_TIME_MASK: c_uint = 0x0F00;
pub const WM8350_CHG_MASK_WALL_FB: c_uint = 0x0080;
pub const WM8350_CHG_TRICKLE_SEL: c_uint = 0x0040;
pub const WM8350_CHG_VSEL_MASK: c_uint = 0x0030;
pub const WM8350_CHG_ISEL_MASK: c_uint = 0x000F;
pub const WM8350_CHG_STS_OFF: c_uint = 0x0000;
pub const WM8350_CHG_STS_TRICKLE: c_uint = 0x1000;
pub const WM8350_CHG_STS_FAST: c_uint = 0x2000;
//
// R170 (0xAA) - Battery Charger Control 3
//
pub const WM8350_CHG_THROTTLE_T_MASK: c_uint = 0x0060;
pub const WM8350_CHG_SMART: c_uint = 0x0010;
pub const WM8350_CHG_TIMER_ADJT_MASK: c_uint = 0x000F;
//
// Charger Interrupts
//
pub const WM8350_IRQ_CHG_BAT_HOT: c_int = 0;
pub const WM8350_IRQ_CHG_BAT_COLD: c_int = 1;
pub const WM8350_IRQ_CHG_BAT_FAIL: c_int = 2;
pub const WM8350_IRQ_CHG_TO: c_int = 3;
pub const WM8350_IRQ_CHG_END: c_int = 4;
pub const WM8350_IRQ_CHG_START: c_int = 5;
pub const WM8350_IRQ_CHG_FAST_RDY: c_int = 6;
pub const WM8350_IRQ_CHG_VBATT_LT_3P9: c_int = 10;
pub const WM8350_IRQ_CHG_VBATT_LT_3P1: c_int = 11;
pub const WM8350_IRQ_CHG_VBATT_LT_2P85: c_int = 12;
//
// Charger Policy
//

//
// Supply Registers.
//
pub const WM8350_USB_VOLTAGE_READBACK: c_uint = 0x9C;
pub const WM8350_LINE_VOLTAGE_READBACK: c_uint = 0x9D;
pub const WM8350_BATT_VOLTAGE_READBACK: c_uint = 0x9E;
//
// Supply Interrupts.
//
pub const WM8350_IRQ_USB_LIMIT: c_int = 15;
pub const WM8350_IRQ_EXT_USB_FB: c_int = 36;
pub const WM8350_IRQ_EXT_WALL_FB: c_int = 37;
pub const WM8350_IRQ_EXT_BAT_FB: c_int = 38;
//
// Policy to control charger state machine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_charger_policy {
// charger state machine policy  - set in machine driver
    pub /: *mut *mut int eoc_mA; / end of charge current (mA),
    pub /: *mut *mut int charge_mV; / charge voltage,
    pub /: *mut *mut int fast_limit_mA; / fast charge current limit,
    pub /: *mut *mut int fast_limit_USB_mA; / USB fast charge current limit,
    pub /: *mut *mut int charge_timeout; / charge timeout (mins),
    pub /: *mut *mut int trickle_start_mV; / trickle charge starts at mV,
    pub /: *mut *mut int trickle_charge_mA; / trickle charge current,
    pub /: *mut *mut int trickle_charge_USB_mA; / USB trickle charge current,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_power {
    pub pdev: *mut platform_device,
    pub battery: *mut power_supply,
    pub usb: *mut power_supply,
    pub ac: *mut power_supply,
    pub policy: *mut wm8350_charger_policy,
    pub rev_g_coeff: c_int,
}
