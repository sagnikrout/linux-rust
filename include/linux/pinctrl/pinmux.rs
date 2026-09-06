//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/pinmux.h
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
// Interface the pinmux subsystem
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// struct pinmux_ops - pinmux operations, to be implemented by pin controller
// drivers that support pinmuxing
// @request: called by the core to see if a certain pin can be made
// available for muxing. This is called by the core to acquire the pins
// before selecting any actual mux setting across a function. The driver
// is allowed to answer "no" by returning a negative error code
// @free: the reverse function of the request() callback, frees a pin after
// being requested
// @get_functions_count: returns number of selectable named functions available
// in this pinmux driver
// @get_function_name: return the function name of the muxing selector,
// called by the core to figure out which mux setting it shall map a
// certain device to
// @get_function_groups: return an array of groups names (in turn
// referencing pins) connected to a certain function selector. The group
// name can be used with the generic @pinctrl_ops to retrieve the
// actual pins affected. The applicable groups will be returned in
// @groups and the number of groups in @num_groups
// @function_is_gpio: determine if the indicated function selector passed
// corresponds to the GPIO function which is used by the accelerated GPIO
// functions @gpio_request_enable, @gpio_disable_free and
// @gpio_set_direction. When the pin control core can properly determine
// if a function is a GPIO function, it is easier to use the @strict mode
// on the pin controller. Since a single function is passed, this is
// only useful on pin controllers that use a specific function for GPIO,
// and that usually presupposes that a one-group-per-pin approach is
// used, so that a single function can be set on a single pin to turn
// it to GPIO mode.
// @set_mux: enable a certain muxing function with a certain pin group. The
// driver does not need to figure out whether enabling this function
// conflicts some other use of the pins in that group, such collisions
// are handled by the pinmux subsystem. The @func_selector selects a
// certain function whereas @group_selector selects a certain set of pins
// to be used. On simple controllers the latter argument may be ignored
// @release_mux: Release software resources acquired by @set_mux. This callback
// must not change hardware state to avoid glitches when switching mux.
// @gpio_request_enable: requests and enables GPIO on a certain pin.
// Implement this only if you can mux every pin individually as GPIO. The
// affected GPIO range is passed along with an offset(pin number) into that
// specific GPIO range - function selectors and pin groups are orthogonal
// to this, the core will however make sure the pins do not collide.
// @gpio_disable_free: free up GPIO muxing on a certain pin, the reverse of
// @gpio_request_enable
// @gpio_set_direction: Since controllers may need different configurations
// depending on whether the GPIO is configured as input or output,
// a direction selector function may be implemented as a backing
// to the GPIO controllers that need pin muxing.
// @strict: do not allow simultaneous use of the same pin for GPIO and another
// function. Check both gpio_owner and mux_owner strictly before approving
// the pin request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinmux_ops {
    pub offset): *mut *mut *mut int (request) (struct pinctrl_dev pctldev, unsigned int,
    pub offset): *mut *mut *mut int (free) (struct pinctrl_dev pctldev, unsigned int,
    pub pctldev): *mut *mut int (get_functions_count) (struct pinctrl_dev,
    pub selector): c_uint,
    pub num_groups): *mut c_uint,
    pub selector): c_uint,
    pub group_selector): c_uint,
    pub group_selector): c_uint,
    pub offset): c_uint,
    pub offset): c_uint,
    pub input): bool,
    pub strict: bool,
}
