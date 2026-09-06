//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/max7360-rotary.c
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
// Copyright 2025 Bootlin
//
// Author: Mathieu Dubois-Briand <mathieu.dubois-briand@bootlin.com>
//

pub const MAX7360_ROTARY_DEFAULT_STEPS: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max7360_rotary {
    pub input: *mut input_dev,
    pub regmap: *mut regmap,
    pub debounce_ms: c_uint,
    pub pos: c_uint,
    pub steps: u32,
    pub axis: u32,
    pub relative_axis: bool,
    pub rollover: bool,
}

#[no_mangle]
unsafe extern "C" fn max7360_rotary_report_event(max7360_rotary: *mut max7360_rotary, steps: c_int) {
    static void max7360_rotary_report_event(struct max7360_rotary *max7360_rotary, int steps)
    {
    if (max7360_rotary.relative_axis) {
    input_report_rel(max7360_rotary.input, max7360_rotary.axis, steps);
    } else {
    let mut pos: c_int = max7360_rotary.pos;
    let mut maxval: c_int = max7360_rotary.steps;
//
// Add steps to the position.
// Make sure added steps are always in ]-maxval; maxval[
// interval, so (pos + maxval) is always >= 0.
// Then set back pos to the [0; maxval[ interval.
//
    pos += steps % maxval;
    if (max7360_rotary.rollover)
    pos = (pos + maxval) % maxval;
    else
    pos = clamp(pos, 0, maxval - 1);
    max7360_rotary.pos = pos;
    input_report_abs(max7360_rotary.input, max7360_rotary.axis, max7360_rotary.pos);
    }
    input_sync(max7360_rotary.input);
    }
#[no_mangle]
unsafe extern "C" fn max7360_rotary_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max7360_rotary_irq(int irq, void *data)
    {
    struct max7360_rotary *max7360_rotary = data;
    struct device *dev = max7360_rotary.input.dev.parent;
    unsigned int val;
    int error;
    error = regmap_read(max7360_rotary.regmap, MAX7360_REG_RTR_CNT, &val);
    if (error < 0) {
    dev_err(dev, "Failed to read rotary counter\n");
    return IRQ_NONE;
    }
    if (val == 0)
    return IRQ_NONE;
    max7360_rotary_report_event(max7360_rotary, sign_extend32(val, 7));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max7360_rotary_hw_init(max7360_rotary: *mut max7360_rotary) -> c_int {
    static int max7360_rotary_hw_init(struct max7360_rotary *max7360_rotary)
    {
    struct device *dev = max7360_rotary.input.dev.parent;
    int val;
    int error;
    val = FIELD_PREP(MAX7360_ROT_DEBOUNCE, max7360_rotary.debounce_ms) |
    FIELD_PREP(MAX7360_ROT_INTCNT, 1) | MAX7360_ROT_INTCNT_DLY;
    error = regmap_write(max7360_rotary.regmap, MAX7360_REG_RTRCFG, val);
    if (error)
    dev_err(dev, "Failed to set max7360 rotary encoder configuration\n");
    return error;
    }
#[no_mangle]
unsafe extern "C" fn max7360_rotary_probe(pdev: *mut platform_device) -> c_int {
    static int max7360_rotary_probe(struct platform_device *pdev)
    {
    struct max7360_rotary *max7360_rotary;
    struct device *dev = &pdev.dev;
    struct input_dev *input;
    struct regmap *regmap;
    int irq;
    int error;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return dev_err_probe(dev, -ENODEV, "Could not get parent regmap\n");
    irq = fwnode_irq_get_byname(dev_fwnode(dev.parent), "inti");
    if (irq < 0)
    return dev_err_probe(dev, irq, "Failed to get IRQ\n");
    max7360_rotary = devm_kzalloc(dev, sizeof(*max7360_rotary), GFP_KERNEL);
    if (!max7360_rotary)
    return -ENOMEM;
    max7360_rotary.regmap = regmap;
    device_property_read_u32(dev.parent, "linux,axis", &max7360_rotary.axis);
    max7360_rotary.rollover = device_property_read_bool(dev.parent,
    "rotary-encoder,rollover");
    max7360_rotary.relative_axis =
    device_property_read_bool(dev.parent, "rotary-encoder,relative-axis");
    error = device_property_read_u32(dev.parent, "rotary-encoder,steps",
    &max7360_rotary.steps);
    if (error)
    max7360_rotary.steps = MAX7360_ROTARY_DEFAULT_STEPS;
    device_property_read_u32(dev.parent, "rotary-debounce-delay-ms",
    &max7360_rotary.debounce_ms);
    if (max7360_rotary.debounce_ms > MAX7360_ROT_DEBOUNCE_MAX)
    return dev_err_probe(dev, -EINVAL, "Invalid debounce timing: %u\n",
    max7360_rotary.debounce_ms);
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    max7360_rotary.input = input;
    input.id.bustype = BUS_I2C;
    input.name = pdev.name;
    if (max7360_rotary.relative_axis)
    input_set_capability(input, EV_REL, max7360_rotary.axis);
    else
    input_set_abs_params(input, max7360_rotary.axis, 0, max7360_rotary.steps, 0, 1);
    error = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), max7360_rotary_irq,
    IRQF_ONESHOT | IRQF_SHARED,
    "max7360-rotary", max7360_rotary);
    if (error)
    return dev_err_probe(dev, error, "Failed to register interrupt\n");
    error = input_register_device(input);
    if (error)
    return dev_err_probe(dev, error, "Could not register input device\n");
    error = max7360_rotary_hw_init(max7360_rotary);
    if (error)
    return dev_err_probe(dev, error, "Failed to initialize max7360 rotary\n");
    device_init_wakeup(dev, true);
    error = dev_pm_set_wake_irq(dev, irq);
    if (error)
    dev_warn(dev, "Failed to set up wakeup irq: %d\n", error);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max7360_rotary_remove(pdev: *mut platform_device) {
    static void max7360_rotary_remove(struct platform_device *pdev)
    {
    dev_pm_clear_wake_irq(&pdev.dev);
    device_init_wakeup(&pdev.dev, false);
    }
    static struct platform_driver max7360_rotary_driver = {
    .driver = {
    .name	= "max7360-rotary",
    },
    .probe		= max7360_rotary_probe,
    .remove		= max7360_rotary_remove,
    };
    module_platform_driver(max7360_rotary_driver);
    MODULE_DESCRIPTION("MAX7360 Rotary driver");
    MODULE_AUTHOR("Mathieu Dubois-Briand <mathieu.dubois-briand@bootlin.com>");
    MODULE_LICENSE("GPL");
