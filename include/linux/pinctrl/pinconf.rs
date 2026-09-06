//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/pinconf.h
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
// Interface the pinconfig portions of the pinctrl subsystem
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// This interface is used in the core to keep track of pins.
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// struct pinconf_ops - pin config operations, to be implemented by
// pin configuration capable drivers.
// @is_generic: for pin controllers that want to use the generic interface,
// this flag tells the framework that it's generic.
// @pin_config_get: get the config of a certain pin, if the requested config
// is not available on this controller this should return -ENOTSUPP
// and if it is available but disabled it should return -EINVAL
// @pin_config_set: configure an individual pin
// @pin_config_group_get: get configurations for an entire pin group; should
// return -ENOTSUPP and -EINVAL using the same rules as pin_config_get.
// @pin_config_group_set: configure all pins in a group
// @pin_config_dbg_show: optional debugfs display hook that will provide
// per-device info for a certain pin in debugfs
// @pin_config_group_dbg_show: optional debugfs display hook that will provide
// per-device info for a certain group in debugfs
// @pin_config_config_dbg_show: optional debugfs display hook that will decode
// and display a driver's pin configuration parameter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinconf_ops {

    pub is_generic: bool,

    pub config): *mut c_ulong,
    pub num_configs): c_uint,
    pub config): *mut c_ulong,
    pub num_configs): c_uint,
    pub offset): c_uint,
    pub selector): c_uint,
    pub config): c_ulong,
}
