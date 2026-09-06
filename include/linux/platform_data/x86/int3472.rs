//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/int3472.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Intel INT3472 ACPI camera sensor power-management support
//
// Author: Dan Scally <djrscally@gmail.com>
//

// FIXME drop this once the I2C_DEV_NAME_FORMAT macro has been added to include/linux/i2c.h

// PMIC GPIO Types
pub const INT3472_GPIO_TYPE_RESET: c_uint = 0x00;
pub const INT3472_GPIO_TYPE_POWERDOWN: c_uint = 0x01;
pub const INT3472_GPIO_TYPE_STROBE: c_uint = 0x02;
pub const INT3472_GPIO_TYPE_POWER_ENABLE: c_uint = 0x0b;
pub const INT3472_GPIO_TYPE_CLK_ENABLE: c_uint = 0x0c;
pub const INT3472_GPIO_TYPE_PRIVACY_LED: c_uint = 0x0d;
pub const INT3472_GPIO_TYPE_DOVDD: c_uint = 0x10;
pub const INT3472_GPIO_TYPE_HANDSHAKE: c_uint = 0x12;
pub const INT3472_GPIO_TYPE_HOTPLUG_DETECT: c_uint = 0x13;
pub const INT3472_PDEV_MAX_NAME_LEN: c_int = 23;
pub const INT3472_MAX_SENSOR_GPIOS: c_int = 3;
pub const INT3472_MAX_LEDS: c_int = 2;
pub const INT3472_MAX_REGULATORS: c_int = 3;
// E.g. "dovdd\0"
pub const GPIO_SUPPLY_NAME_LENGTH: c_int = 6;
// 12 chars for acpi_dev_name() + "-", e.g. "ABCD1234:00-"

// lower- and upper-case mapping
pub const GPIO_REGULATOR_SUPPLY_MAP_COUNT: c_int = 2;
//
// Ensure the GPIO is driven low/high for at least 2 ms before changing.
//
// 2 ms has been chosen because it is the minimum time ovXXXX sensors need to
// have their reset line driven logical high to properly register a reset.
//

pub const INT3472_LED_MAX_NAME_LEN: c_int = 32;
pub const CIO2_SENSOR_SSDB_MCLKSPEED_OFFSET: c_int = 86;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_cldb {
    pub version: u8,
//
// control logic type
// 0: UNKNOWN
// 1: DISCRETE(CRD-D)
// 2: PMIC TPS68470
// 3: PMIC uP6641
//
    pub control_logic_type: u8,
    pub control_logic_id: u8,
    pub sensor_card_sku: u8,
    pub reserved: [u8; 10],
    pub clock_source: u8,
    pub reserved2: [u8; 17],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_discrete_quirks {
// For models where AVDD GPIO is shared between sensors
    pub avdd_second_sensor: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_gpio_regulator {
// SUPPLY_MAP_COUNT * 2 to make room for second sensor mappings
    pub 2]: *mut *mut regulator_consumer_supply supply_map[GPIO_REGULATOR_SUPPLY_MAP_COUNT,
    pub supply_name_upper: [c_char; GPIO_SUPPLY_NAME_LENGTH],
    pub regulator_name: [c_char; GPIO_REGULATOR_NAME_LENGTH],
    pub rdev: *mut regulator_dev,
    pub rdesc: regulator_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_discrete_device {
    pub adev: *mut acpi_device,
    pub dev: *mut device,
    pub sensor: *mut acpi_device,
    pub sensor_name: *const c_char,
    pub regulators: [int3472_gpio_regulator; INT3472_MAX_REGULATORS],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_clock {
    pub clk: *mut clk,
    pub clk_hw: clk_hw,
    pub cl: *mut clk_lookup,
    pub ena_gpio: *mut gpio_desc,
    pub frequency: u32,
    pub imgclk_index: u8,
    pub clock: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_led {
    pub classdev: led_classdev,
    pub lookup: led_lookup_data,
    pub name: [c_char; INT3472_LED_MAX_NAME_LEN],
    pub gpio: *mut gpio_desc,
    pub leds: [}; INT3472_MAX_LEDS],
    pub quirks: int3472_discrete_quirks,
    pub /: *mut *mut unsigned int ngpios; / how many GPIOs have we seen,
    pub /: *mut *mut unsigned int n_leds; / how many LEDs have we registered,
    pub /: *mut *mut unsigned int n_sensor_gpios; / how many have we mapped to sensor,
    pub /: *mut *mut unsigned int n_regulator_gpios; / how many have we mapped to a regulator,
    pub gpios: gpiod_lookup_table,
}

extern "C" {
    pub fn skl_int3472_fill_cldb(adev: *mut acpi_device, cldb: *mut int3472_cldb) -> c_int;
}
extern "C" {
    pub fn int3472_discrete_parse_crs(int3472: *mut int3472_discrete_device) -> c_int;
}
extern "C" {
    pub fn int3472_discrete_cleanup(int3472: *mut int3472_discrete_device);
}
extern "C" {
    pub fn skl_int3472_register_dsm_clock(int3472: *mut int3472_discrete_device) -> c_int;
}
extern "C" {
    pub fn skl_int3472_unregister_clock(int3472: *mut int3472_discrete_device);
}
extern "C" {
    pub fn skl_int3472_unregister_regulator(int3472: *mut int3472_discrete_device);
}
extern "C" {
    pub fn skl_int3472_unregister_leds(int3472: *mut int3472_discrete_device);
}
