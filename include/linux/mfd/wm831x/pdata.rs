//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/pdata.h
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
// include/linux/mfd/wm831x/pdata.h -- Platform data for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_backlight_pdata {
    pub /: *mut *mut *mut int isink; / ISINK to use, 1 or 2,
    pub /: *mut *mut *mut int max_uA; / Maximum current to allow,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_backup_pdata {
    pub charger_enable: c_int,
    pub /: *mut *mut *mut int no_constant_voltage; / Disable constant voltage charging,
    pub /: *mut *mut *mut int vlim; / Voltage limit in millivolts,
    pub /: *mut *mut *mut int ilim; / Current limit in microamps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_battery_pdata {
    pub /: *mut *mut *mut int enable; / Enable charging,
    pub /: *mut *mut *mut int fast_enable; / Enable fast charging,
    pub /: *mut *mut *mut int off_mask; / Mask OFF while charging,
    pub /: *mut *mut *mut int trickle_ilim; / Trickle charge current limit, in mA,
    pub /: *mut *mut *mut int vsel; / Target voltage, in mV,
    pub /: *mut *mut *mut int eoc_iterm; / End of trickle charge current, in mA,
    pub /: *mut *mut *mut int fast_ilim; / Fast charge current limit, in mA,
    pub /: *mut *mut *mut int timeout; / Charge cycle timeout, in minutes,
}

//
// Configuration for the WM831x DC-DC BuckWise convertors.  This
// should be passed as driver_data in the regulator_init_data.
//
// Currently all the configuration is for the fast DVS switching
// support of the devices.  This allows MFPs on the device to be
// configured as an input to switch between two output voltages,
// allowing voltage transitions without the expense of an access over
// I2C or SPI buses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_buckv_pdata {
    pub /: *mut *mut *mut int dvs_control_src; / Hardware DVS source to use (1 or 2),
    pub /: *mut *mut *mut int dvs_init_state; / DVS state to expect on startup,
    pub /: *mut *mut *mut int dvs_state_gpio; / CPU GPIO to use for monitoring status,
}

// Sources for status LED configuration.  Values are register values
// plus 1 to allow for a zero default for preserve.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm831x_status_src {
    WM831X_STATUS_PRESERVE = 0,  /* Keep the current hardware setting */
    WM831X_STATUS_OTP = 1,
    WM831X_STATUS_POWER = 2,
    WM831X_STATUS_CHARGER = 3,
    WM831X_STATUS_MANUAL = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_status_pdata {
    pub default_src: wm831x_status_src,
    pub name: *const c_char,
    pub default_trigger: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_touch_pdata {
    pub /: *mut *mut *mut int fivewire; / 1 for five wire mode, 0 for 4 wire,
    pub /: *mut *mut *mut int isel; / Current for pen down (uA),
    pub /: *mut *mut *mut int rpu; / Pen down sensitivity resistor divider,
    pub /: *mut *mut *mut int pressure; / Report pressure (boolean),
    pub /: *mut *mut *mut unsigned int data_irq; / Touch data ready IRQ,
    pub /: *mut *mut *mut int data_irqf; / IRQ flags for data ready IRQ,
    pub /: *mut *mut *mut unsigned int pd_irq; / Touch pendown detect IRQ,
    pub /: *mut *mut *mut int pd_irqf; / IRQ flags for pen down IRQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm831x_watchdog_action {
    WM831X_WDOG_NONE = 0,
    WM831X_WDOG_INTERRUPT = 1,
    WM831X_WDOG_RESET = 2,
    WM831X_WDOG_WAKE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_watchdog_pdata {
    pub secondary: wm831x_watchdog_action primary,,
    pub software:1: c_uint,
}

pub const WM831X_MAX_STATUS: c_int = 2;
pub const WM831X_MAX_DCDC: c_int = 4;
pub const WM831X_MAX_EPE: c_int = 2;
pub const WM831X_MAX_LDO: c_int = 11;
pub const WM831X_MAX_ISINK: c_int = 2;
pub const WM831X_GPIO_CONFIGURE: c_uint = 0x10000;
pub const WM831X_GPIO_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_pdata {
// Used to distinguish multiple WM831x chips
    pub wm831x_num: c_int,
// Called before subdevices are set up
    pub wm831x): *mut *mut int (pre_init)(struct wm831x,
// Called after subdevices are set up
    pub wm831x): *mut *mut int (post_init)(struct wm831x,
// Put the /IRQ line into CMOS mode
    pub irq_cmos: bool,
// Disable the touchscreen
    pub disable_touch: bool,
// The driver should initiate a power off sequence during shutdown
    pub soft_shutdown: bool,
    pub irq_base: c_int,
    pub gpio_base: c_int,
    pub gpio_defaults: [c_int; WM831X_GPIO_NUM],
    pub backlight: *mut wm831x_backlight_pdata,
    pub backup: *mut wm831x_backup_pdata,
    pub battery: *mut wm831x_battery_pdata,
    pub touch: *mut wm831x_touch_pdata,
    pub watchdog: *mut wm831x_watchdog_pdata,
// LED1 = 0 and so on
    pub status: [*mut wm831x_status_pdata; WM831X_MAX_STATUS],
// DCDC1 = 0 and so on
    pub dcdc: [*mut regulator_init_data; WM831X_MAX_DCDC],
// EPE1 = 0 and so on
    pub epe: [*mut regulator_init_data; WM831X_MAX_EPE],
// LDO1 = 0 and so on
    pub ldo: [*mut regulator_init_data; WM831X_MAX_LDO],
// ISINK1 = 0 and so on
    pub isink: [*mut regulator_init_data; WM831X_MAX_ISINK],
}
