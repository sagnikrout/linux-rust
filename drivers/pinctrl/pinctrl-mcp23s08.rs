//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-mcp23s08.h
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
// MCP23S08 SPI/I2C GPIO driver

//
// MCP types supported by driver
//
pub const MCP_TYPE_S08: c_int = 1;
pub const MCP_TYPE_S17: c_int = 2;
pub const MCP_TYPE_008: c_int = 3;
pub const MCP_TYPE_017: c_int = 4;
pub const MCP_TYPE_S18: c_int = 5;
pub const MCP_TYPE_018: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp23s08_info {
    pub regmap: *const regmap_config,
    pub label: *const c_char,
    pub type: c_uint,
    pub ngpio: u16,
    pub reg_shift: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp23s08 {
    pub addr: u8,
    pub irq_active_high: bool,
    pub reg_shift: bool,
    pub irq_rise: u16,
    pub irq_fall: u16,
    pub irq: c_int,
    pub irq_controller: bool,
    pub cached_gpio: c_int,
// lock protects regmap access with bypass/cache flags
    pub lock: mutex,
    pub chip: gpio_chip,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub pctldev: *mut pinctrl_dev,
    pub pinctrl_desc: pinctrl_desc,
    pub reset_gpio: *mut gpio_desc,
}
