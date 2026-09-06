//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/sophgo/pinctrl-sophgo.h
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
// Copyright (C) 2024 Inochi Amaoto <inochiama@outlook.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_pin {
    pub id: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_pin_mux_config {
    pub pin: *const sophgo_pin,
    pub config: u32,
}

//
// struct sophgo_cfg_ops - pin configuration operations
//
// @pctrl_init: soc specific init callback
// @verify_pinmux_config: verify the pinmux config for a pin
// @verify_pin_group: verify the whole pinmux group
// @dt_node_to_map_post: post init for the pinmux config map
// @compute_pinconf_config: compute pinconf config
// @set_pinconf_config: set pinconf config (the caller holds lock)
// @set_pinmux_config: set mux config (the caller holds lock)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_cfg_ops {
    pub pctrl): *mut sophgo_pinctrl,
    pub config): *const *const int (verify_pinmux_config)(struct sophgo_pin_mux_config,
    pub npins): c_uint,
    pub npins): c_uint,
    pub mask): *mut *mut u32 value, u32,
    pub mask): u32 value, u32,
    pub config): *const *const sophgo_pin sp, u32,
}

//
// struct sophgo_vddio_cfg_ops - pin vddio operations
//
// @get_pull_up: get resistor for pull up;
// @get_pull_down: get resistor for pull down.
// @get_oc_map: get mapping for typical low level output current value to
// register value map.
// @get_schmitt_map: get mapping for register value to typical schmitt
// threshold.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_vddio_cfg_ops {
    pub psmap): *const *const *const int (get_pull_up)(struct sophgo_pin pin, u32,
    pub psmap): *const *const *const int (get_pull_down)(struct sophgo_pin pin, u32,
    pub map): *const u32,
    pub map): *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_pinctrl_data {
    pub pins: *const pinctrl_pin_desc,
    pub pindata: *const c_void,
    pub pdnames: *const *const c_char,
    pub vddio_ops: *const sophgo_vddio_cfg_ops,
    pub cfg_ops: *const sophgo_cfg_ops,
    pub pctl_ops: *const pinctrl_ops,
    pub pmx_ops: *const pinmux_ops,
    pub pconf_ops: *const pinconf_ops,
    pub npins: u16,
    pub npds: u16,
    pub pinsize: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_pinctrl {
    pub dev: *mut device,
    pub pctrl_dev: *mut pinctrl_dev,
    pub data: *const sophgo_pinctrl_data,
    pub pdesc: pinctrl_desc,
    pub mutex: mutex,
    pub lock: raw_spinlock_t,
    pub priv_ctrl: *mut c_void,
}

extern "C" {
    pub fn sophgo_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
