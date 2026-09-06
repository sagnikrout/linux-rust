//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/machine.h
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
// Machine interface for the pinctrl subsystem.
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pinctrl_map_type {
    PIN_MAP_TYPE_INVALID,
    PIN_MAP_TYPE_DUMMY_STATE,
    PIN_MAP_TYPE_MUX_GROUP,
    PIN_MAP_TYPE_CONFIGS_PIN,
    PIN_MAP_TYPE_CONFIGS_GROUP,
}

//
// struct pinctrl_map_mux - mapping table content for MAP_TYPE_MUX_GROUP
// @group: the name of the group whose mux function is to be configured. This
// field may be left NULL, and the first applicable group for the function
// will be used.
// @function: the mux function to select for the group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_map_mux {
    pub group: *const c_char,
    pub function: *const c_char,
}

//
// struct pinctrl_map_configs - mapping table content for MAP_TYPE_CONFIGS_
// @group_or_pin: the name of the pin or group whose configuration parameters
// are to be configured.
// @configs: a pointer to an array of config parameters/values to program into
// hardware. Each individual pin controller defines the format and meaning
// of config parameters.
// @num_configs: the number of entries in array @configs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_map_configs {
    pub group_or_pin: *const c_char,
    pub configs: *mut c_ulong,
    pub num_configs: c_uint,
}

//
// struct pinctrl_map - boards/machines shall provide this map for devices
// @dev_name: the name of the device using this specific mapping, the name
// must be the same as in your struct device*. If this name is set to the
// same name as the pin controllers own dev_name(), the map entry will be
// hogged by the driver itself upon registration
// @name: the name of this specific map entry for the particular machine.
// This is the parameter passed to pinmux_lookup_state()
// @type: the type of mapping table entry
// @ctrl_dev_name: the name of the device controlling this specific mapping,
// the name must be the same as in your struct device*. This field is not
// used for PIN_MAP_TYPE_DUMMY_STATE
// @data: Data specific to the mapping type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_map {
    pub dev_name: *const c_char,
    pub name: *const c_char,
    pub type: pinctrl_map_type,
    pub ctrl_dev_name: *const c_char,
    pub mux: pinctrl_map_mux,
    pub configs: pinctrl_map_configs,
    pub data: },
}

// Convenience macros to create mapping table entries

extern "C" {
    pub fn pinctrl_unregister_mappings(map: *const pinctrl_map);
}
extern "C" {
    pub fn pinctrl_provide_dummies();
}

