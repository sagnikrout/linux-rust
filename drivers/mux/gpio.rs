//! Automatically rewritten from C to Rust
//! Source: drivers/mux/gpio.c
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
// GPIO-controlled multiplexer driver
//
// Copyright (C) 2017 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_gpio {
    pub gpios: *mut gpio_descs,
}

#[no_mangle]
unsafe extern "C" fn mux_gpio_set(mux: *mut mux_control, state: c_int) -> c_int {
    static int mux_gpio_set(struct mux_control *mux, int state)
    {
    struct mux_gpio *mux_gpio = mux_chip_priv(mux.chip);
    DECLARE_BITMAP(values, BITS_PER_TYPE(state));
    let mut value: u32 = state;
    bitmap_from_arr32(values, &value, BITS_PER_TYPE(value));
    gpiod_multi_set_value_cansleep(mux_gpio.gpios, values);
    return 0;
    }
    static const struct mux_control_ops mux_gpio_ops = {
    .set = mux_gpio_set,
    };
    static const struct of_device_id mux_gpio_dt_ids[] = {
    { .compatible = "gpio-mux", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mux_gpio_dt_ids);
#[no_mangle]
unsafe extern "C" fn mux_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mux_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mux_chip *mux_chip;
    struct mux_gpio *mux_gpio;
    int pins;
    s32 idle_state;
    int ret;
    pins = gpiod_count(dev, "mux");
    if (pins < 0)
    return pins;
    mux_chip = devm_mux_chip_alloc(dev, 1, sizeof(*mux_gpio));
    if (IS_ERR(mux_chip))
    return PTR_ERR(mux_chip);
    mux_gpio = mux_chip_priv(mux_chip);
    mux_chip.ops = &mux_gpio_ops;
    mux_gpio.gpios = devm_gpiod_get_array(dev, "mux", GPIOD_OUT_LOW);
    if (IS_ERR(mux_gpio.gpios))
    return dev_err_probe(dev, PTR_ERR(mux_gpio.gpios),
    "failed to get gpios\n");
    WARN_ON(pins != mux_gpio.gpios.ndescs);
    mux_chip.mux.states = BIT(pins);
    ret = device_property_read_u32(dev, "idle-state", (u32 *)&idle_state);
    if (ret >= 0 && idle_state != MUX_IDLE_AS_IS) {
    if (idle_state < 0 || idle_state >= mux_chip.mux.states) {
    dev_err(dev, "invalid idle-state %u\n", idle_state);
    return -EINVAL;
    }
    mux_chip.mux.idle_state = idle_state;
    }
    ret = devm_regulator_get_enable_optional(dev, "mux");
    if (ret && ret != -ENODEV)
    return dev_err_probe(dev, ret, "failed to get/enable mux supply\n");
    ret = devm_mux_chip_register(dev, mux_chip);
    if (ret < 0)
    return ret;
    dev_info(dev, "%u-way mux-controller registered\n",
    mux_chip.mux.states);
    return 0;
    }
    static struct platform_driver mux_gpio_driver = {
    .driver = {
    .name = "gpio-mux",
    .of_match_table	= mux_gpio_dt_ids,
    },
    .probe = mux_gpio_probe,
    };
    module_platform_driver(mux_gpio_driver);
    MODULE_DESCRIPTION("GPIO-controlled multiplexer driver");
    MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
    MODULE_LICENSE("GPL v2");
