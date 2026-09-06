//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/core.h
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
// Core private header for the pin control subsystem
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// struct pinctrl_dev - pin control class device
// @node: node to include this pin controller in the global pin controller list
// @desc: the pin controller descriptor supplied when initializing this pin
// controller
// @pin_desc_tree: each pin descriptor for this pin controller is stored in
// this radix tree
// @pin_group_tree: optionally each pin group can be stored in this radix tree
// @num_groups: optionally number of groups can be kept here
// @pin_function_tree: optionally each function can be stored in this radix tree
// @num_functions: optionally number of functions can be kept here
// @gpio_ranges: a list of GPIO ranges that is handled by this pin controller,
// ranges are added to this list at runtime
// @dev: the device entry for this pin controller
// @owner: module providing the pin controller, used for refcounting
// @driver_data: driver data for drivers registering to the pin controller
// subsystem
// @p: result of pinctrl_get() for this device
// @hog_default: default state for pins hogged by this device
// @hog_sleep: sleep state for pins hogged by this device
// @mutex: mutex taken on each pin controller specific action
// @device_root: debugfs root for this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_dev {
    pub node: list_head,
    pub desc: *const pinctrl_desc,
    pub pin_desc_tree: radix_tree_root,

    pub pin_group_tree: radix_tree_root,
    pub num_groups: c_uint,

    pub pin_function_tree: radix_tree_root,
    pub num_functions: c_uint,

    pub gpio_ranges: list_head,
    pub dev: *mut device,
    pub owner: *mut module,
    pub driver_data: *mut c_void,
    pub p: *mut pinctrl,
    pub hog_default: *mut pinctrl_state,
    pub hog_sleep: *mut pinctrl_state,
    pub mutex: mutex,

    pub device_root: *mut dentry,

}

//
// struct pinctrl - per-device pin control state holder
// @node: global list node
// @dev: the device using this pin control handle
// @states: a list of states for this device
// @state: the current state
// @dt_maps: the mapping table chunks dynamically parsed from device tree for
// this device, if any
// @users: reference count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl {
    pub node: list_head,
    pub dev: *mut device,
    pub states: list_head,
    pub state: *mut pinctrl_state,
    pub dt_maps: list_head,
    pub users: kref,
}

//
// struct pinctrl_state - a pinctrl state for a device
// @node: list node for struct pinctrl's @states field
// @name: the name of this state
// @settings: a list of settings for this state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_state {
    pub node: list_head,
    pub name: *const c_char,
    pub settings: list_head,
}

//
// struct pinctrl_setting_mux - setting data for MAP_TYPE_MUX_GROUP
// @group: the group selector to program
// @func: the function selector to program
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_setting_mux {
    pub group: c_uint,
    pub func: c_uint,
}

//
// struct pinctrl_setting_configs - setting data for MAP_TYPE_CONFIGS_
// @group_or_pin: the group selector or pin ID to program
// @configs: a pointer to an array of config parameters/values to program into
// hardware. Each individual pin controller defines the format and meaning
// of config parameters.
// @num_configs: the number of entries in array @configs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_setting_configs {
    pub group_or_pin: c_uint,
    pub configs: *mut c_ulong,
    pub num_configs: c_uint,
}

//
// struct pinctrl_setting - an individual mux or config setting
// @node: list node for struct pinctrl_settings's @settings field
// @type: the type of setting
// @pctldev: pin control device handling to be programmed. Not used for
// PIN_MAP_TYPE_DUMMY_STATE.
// @dev_name: the name of the device using this state
// @data: Data specific to the setting type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_setting {
    pub node: list_head,
    pub type: pinctrl_map_type,
    pub pctldev: *mut pinctrl_dev,
    pub dev_name: *const c_char,
    pub mux: pinctrl_setting_mux,
    pub configs: pinctrl_setting_configs,
    pub data: },
}

//
// struct pin_desc - pin descriptor for each physical pin in the arch
// @pctldev: corresponding pin control device
// @name: a name for the pin, e.g. the name of the pin/pad/finger on a
// datasheet or such
// @dynamic_name: if the name of this pin was dynamically allocated
// @drv_data: driver-defined per-pin data. pinctrl core does not touch this
// @mux_usecount: If zero, the pin is not claimed, and @owner should be NULL.
// If non-zero, this pin is claimed by @owner. This field is an integer
// rather than a boolean, since pinctrl_get() might process multiple
// mapping table entries that refer to, and hence claim, the same group
// or pin, and each of these will increment the @usecount.
// @mux_owner: The name of device that called pinctrl_get().
// @mux_setting: The most recent selected mux setting for this pin, if any.
// @gpio_owner: If pinctrl_gpio_request() was called for this pin, this is
// the name of the GPIO that "owns" this pin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pin_desc {
    pub pctldev: *mut pinctrl_dev,
    pub name: *const c_char,
    pub dynamic_name: bool,
    pub drv_data: *mut c_void,
// These fields only added when supporting pinmux drivers

    pub mux_usecount: c_uint,
    pub mux_owner: *const c_char,
    pub mux_setting: *const pinctrl_setting_mux,
    pub gpio_owner: *const c_char,
    pub mux_lock: mutex,

}

//
// struct pinctrl_maps - a list item containing part of the mapping table
// @node: mapping table list node
// @maps: array of mapping table entries
// @num_maps: the number of entries in @maps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_maps {
    pub node: list_head,
    pub maps: *const pinctrl_map,
    pub num_maps: c_uint,
}

//
// struct group_desc - generic pin group descriptor
// @grp: generic data of the pin group (name and pins)
// @data: pin controller driver specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_desc {
    pub grp: pingroup,
    pub data: *mut c_void,
}

// Convenient macro to define a generic pin group descriptor

extern "C" {
    pub fn pinctrl_generic_get_group_count(pctldev: *mut pinctrl_dev) -> c_int;
}

extern "C" {
    pub fn pin_get_from_name(pctldev: *mut pinctrl_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn radix_tree_lookup(_arg: &pctldev->pin_desc_tree, _arg: pin) -> return;
}
extern "C" {
    pub fn pinctrl_force_sleep(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn pinctrl_force_default(pctldev: *mut pinctrl_dev) -> c_int;
}
