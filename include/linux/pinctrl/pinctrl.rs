//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/pinctrl.h
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
// Interface the pinctrl subsystem
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// This interface is used in the core to keep track of pins.
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// struct pingroup - provides information on pingroup
// @name: a name for pingroup
// @pins: an array of pins in the pingroup
// @npins: number of pins in the pingroup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pingroup {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub npins: usize,
}

// Convenience macro to define a single named or anonymous pingroup

//
// struct pinctrl_pin_desc - boards/machines provide information on their
// pins, pads or other muxable units in this struct
// @number: unique pin number from the global pin number space
// @name: a name for this pin
// @drv_data: driver-defined per-pin data. pinctrl core does not touch this
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_pin_desc {
    pub number: c_uint,
    pub name: *const c_char,
    pub drv_data: *mut c_void,
}

// Convenience macro to define a single named or anonymous pin descriptor

//
// struct pinctrl_gpio_range - each pin controller can provide subranges of
// the GPIO number space to be handled by the controller
// @node: list node for internal use
// @name: a name for the chip in this range
// @id: an ID number for the chip in this range
// @base: base offset of the GPIO range
// @pin_base: base pin number of the GPIO range if pins == NULL
// @npins: number of pins in the GPIO range, including the base number
// @pins: enumeration of pins in GPIO range or NULL
// @gc: an optional pointer to a gpio_chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_gpio_range {
    pub node: list_head,
    pub name: *const c_char,
    pub id: c_uint,
    pub base: c_uint,
    pub pin_base: c_uint,
    pub npins: c_uint,
    pub pins: *const c_uint,
    pub gc: *mut gpio_chip,
}

//
// struct pinctrl_ops - global pin control operations, to be implemented by
// pin controller drivers.
// @get_groups_count: Returns the count of total number of groups registered.
// @get_group_name: return the group name of the pin group
// @get_group_pins: return an array of pins corresponding to a certain
// group selector @pins, and the size of the array in @num_pins
// @pin_dbg_show: optional debugfs display hook that will provide per-device
// info for a certain pin in debugfs
// @dt_node_to_map: parse a device tree "pin configuration node", and create
// mapping table entries for it. These are returned through the @map and
// @num_maps output parameters. This function is optional, and may be
// omitted for pinctrl drivers that do not support device tree.
// @dt_free_map: free mapping table entries created via @dt_node_to_map. The
// top-level @map pointer must be freed, along with any dynamically
// allocated members of the mapping table entries themselves. This
// function is optional, and may be omitted for pinctrl drivers that do
// not support device tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_ops {
    pub pctldev): *mut *mut int (get_groups_count) (struct pinctrl_dev,
    pub selector): c_uint,
    pub num_pins): *mut c_uint,
    pub offset): c_uint,
    pub num_maps): *mut *mut *mut pinctrl_map map, unsigned int,
    pub num_maps): *mut *mut pinctrl_map map, unsigned int,
}

//
// struct pinctrl_desc - pin controller descriptor, register this to pin
// control subsystem
// @name: name for the pin controller
// @pins: an array of pin descriptors describing all the pins handled by
// this pin controller
// @npins: number of descriptors in the array, usually just ARRAY_SIZE()
// of the pins field above
// @pctlops: pin control operation vtable, to support global concepts like
// grouping of pins, this is optional.
// @pmxops: pinmux operations vtable, if you support pinmuxing in your driver
// @confops: pin config operations vtable, if you support pin configuration in
// your driver
// @owner: module providing the pin controller, used for refcounting
// @num_custom_params: Number of driver-specific custom parameters to be parsed
// from the hardware description
// @custom_params: List of driver_specific custom parameters to be parsed from
// the hardware description
// @custom_conf_items: Information how to print @params in debugfs, must be
// the same size as the @custom_params, i.e. @num_custom_params
// @link_consumers: If true create a device link between pinctrl and its
// consumers (i.e. the devices requesting pin control states). This is
// sometimes necessary to ascertain the right suspend/resume order for
// example.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinctrl_desc {
    pub name: *const c_char,
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub pctlops: *const pinctrl_ops,
    pub pmxops: *const pinmux_ops,
    pub confops: *const pinconf_ops,
    pub owner: *mut module,

    pub num_custom_params: c_uint,
    pub custom_params: *const pinconf_generic_params,
    pub custom_conf_items: *const pin_config_item,

    pub link_consumers: bool,
}

// External interface to pin controller
extern "C" {
    pub fn pinctrl_enable(pctldev: *mut pinctrl_dev) -> c_int;
}
// Please use pinctrl_register_and_init() and pinctrl_enable() instead
extern "C" {
    pub fn pinctrl_unregister(pctldev: *mut pinctrl_dev);
}
// Please use devm_pinctrl_register_and_init() instead

//
// struct pinfunction - Description about a function
// @name: Name of the function
// @groups: An array of groups for this function
// @ngroups: Number of groups in @groups
// @flags: Additional pin function flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinfunction {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: usize,
    pub flags: c_ulong,
}

// Convenience macro to define a single named pinfunction

// Same as PINCTRL_PINFUNCTION() but for the GPIO category of functions

