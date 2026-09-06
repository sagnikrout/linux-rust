//! Automatically rewritten from C to Rust
//! Source: drivers/base/pinctrl.c
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
// Driver core interface to the pinctrl subsystem.
//
// Copyright (C) 2012 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
// Based on bits of regulator core, gpio core and clk core
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

//
// pinctrl_bind_pins() - called by the device core before probe
// @dev: the device that is just about to probe
//
#[no_mangle]
pub unsafe extern "C" fn pinctrl_bind_pins(dev: *mut device) -> c_int {
    int pinctrl_bind_pins(struct device *dev)
    {
    int ret;
    if (dev_of_node_reused(dev))
    return 0;
    dev.pins = devm_kzalloc(dev, sizeof(*(dev.pins)), GFP_KERNEL);
    if (!dev.pins)
    return -ENOMEM;
    dev.pins.p = devm_pinctrl_get(dev);
    if (IS_ERR(dev.pins.p)) {
    dev_dbg(dev, "no pinctrl handle\n");
    ret = PTR_ERR(dev.pins.p);
    goto cleanup_alloc;
    }
    dev.pins.default_state = pinctrl_lookup_state(dev.pins.p,
    PINCTRL_STATE_DEFAULT);
    if (IS_ERR(dev.pins.default_state)) {
    dev_dbg(dev, "no default pinctrl state\n");
    ret = 0;
    goto cleanup_get;
    }
    dev.pins.init_state = pinctrl_lookup_state(dev.pins.p,
    PINCTRL_STATE_INIT);
    if (IS_ERR(dev.pins.init_state)) {
// Not supplying this state is perfectly legal
    dev_dbg(dev, "no init pinctrl state\n");
    ret = pinctrl_select_state(dev.pins.p,
    dev.pins.default_state);
    } else {
    ret = pinctrl_select_state(dev.pins.p, dev.pins.init_state);
    }
    if (ret) {
    dev_dbg(dev, "failed to activate initial pinctrl state\n");
    goto cleanup_get;
    }

//
// If power management is enabled, we also look for the optional
// sleep and idle pin states, with semantics as defined in
// <linux/pinctrl/pinctrl-state.h>
//
    dev.pins.sleep_state = pinctrl_lookup_state(dev.pins.p,
    PINCTRL_STATE_SLEEP);
    if (IS_ERR(dev.pins.sleep_state))
// Not supplying this state is perfectly legal
    dev_dbg(dev, "no sleep pinctrl state\n");
    dev.pins.idle_state = pinctrl_lookup_state(dev.pins.p,
    PINCTRL_STATE_IDLE);
    if (IS_ERR(dev.pins.idle_state))
// Not supplying this state is perfectly legal
    dev_dbg(dev, "no idle pinctrl state\n");

    return 0;
//
// If no pinctrl handle or default state was found for this device,
// let's explicitly free the pin container in the device, there is
// no point in keeping it around.
//
    cleanup_get:
    devm_pinctrl_put(dev.pins.p);
    cleanup_alloc:
    devm_kfree(dev, dev.pins);
    dev.pins = core::ptr::null_mut();
// Return deferrals
    if (ret == -EPROBE_DEFER)
    return ret;
// Return serious errors
    if (ret == -EINVAL)
    return ret;
// We ignore errors like -ENOENT meaning no pinctrl state
    return 0;
    }
