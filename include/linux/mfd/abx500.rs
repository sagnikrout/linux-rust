//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/abx500.h
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
// Copyright (C) 2007-2009 ST-Ericsson AB
//
// ABX500 core access functions.
// The abx500 interface is used for the Analog Baseband chips.
//
// Author: Mattias Wallin <mattias.wallin@stericsson.com>
// Author: Mattias Nilsson <mattias.i.nilsson@stericsson.com>
// Author: Bengt Jonsson <bengt.g.jonsson@stericsson.com>
// Author: Rickard Andersson <rickard.andersson@stericsson.com>
//

//
// struct abx500_init_setting
// Initial value of the registers for driver to use during setup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_init_settings {
    pub bank: u8,
    pub reg: u8,
    pub setting: u8,
}

//
// abx500_mask_and_set_register_inerruptible() - Modifies selected bits of a
// target register
//
// @dev: The AB sub device.
// @bank: The i2c bank number.
// @bitmask: The bit mask to use.
// @bitvalues: The new bit values.
//
// Updates the value of an AB register:
// value -> ((value & ~bitmask) | (bitvalues & bitmask))
//
extern "C" {
    pub fn abx500_get_chip_id(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn abx500_event_registers_startup_state_get(dev: *mut device, event: *mut u8) -> c_int;
}
extern "C" {
    pub fn abx500_startup_irq_enabled(dev: *mut device, irq: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_ops {
    pub ): *mut *mut int (get_chip_id) (struct device,
    pub ): *mut *mut *mut int (get_register) (struct device , u8, u8, u8,
    pub u8): *mut *mut *mut int (set_register) (struct device , u8, u8,,
    pub u8): *mut *mut *mut *mut int (get_register_page) (struct device , u8, u8, u8 ,,
    pub u8): *mut *mut *mut *mut int (set_register_page) (struct device , u8, u8, u8 ,,
    pub u8): *mut *mut *mut int (mask_and_set_register) (struct device , u8, u8, u8,,
    pub ): *mut *mut *mut int (event_registers_startup_state_get) (struct device , u8,
    pub int): *mut *mut *mut int (startup_irq_enabled) (struct device , unsigned,
    pub ): *mut *mut void (dump_all_banks) (struct device,
}

extern "C" {
    pub fn abx500_register_ops(core_dev: *mut device, ops: *mut abx500_ops) -> c_int;
}
extern "C" {
    pub fn abx500_remove_ops(dev: *mut device);
}
