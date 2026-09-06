//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/gpio-restart.c
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
// Toggles a GPIO pin to restart a device
//
// Copyright (C) 2014 Google, Inc.
//
// Based on the gpio-poweroff driver.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_restart {
    pub reset_gpio: *mut gpio_desc,
    pub active_delay_ms: u32,
    pub inactive_delay_ms: u32,
    pub wait_delay_ms: u32,
}

#[no_mangle]
unsafe extern "C" fn gpio_restart_notify(data: *mut sys_off_data) -> c_int {
    static int gpio_restart_notify(struct sys_off_data *data)
    {
    struct gpio_restart *gpio_restart = data.cb_data;
// drive it active, also inactive->active edge
    gpiod_direction_output(gpio_restart.reset_gpio, 1);
    mdelay(gpio_restart.active_delay_ms);
// drive inactive, also active->inactive edge
    gpiod_set_value(gpio_restart.reset_gpio, 0);
    mdelay(gpio_restart.inactive_delay_ms);
// drive it active, also inactive->active edge
    gpiod_set_value(gpio_restart.reset_gpio, 1);
// give it some time
    mdelay(gpio_restart.wait_delay_ms);
    WARN_ON(1);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn gpio_restart_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_restart_probe(struct platform_device *pdev)
    {
    struct gpio_restart *gpio_restart;
    let mut open_source: bool = false;
    let mut priority: c_int = 129;
    u32 property;
    int ret;
    gpio_restart = devm_kzalloc(&pdev.dev, sizeof(*gpio_restart),
    GFP_KERNEL);
    if (!gpio_restart)
    return -ENOMEM;
    open_source = of_property_read_bool(pdev.dev.of_node, "open-source");
    gpio_restart.reset_gpio = devm_gpiod_get(&pdev.dev, core::ptr::null_mut(),
    open_source ? GPIOD_IN : GPIOD_OUT_LOW);
    ret = PTR_ERR_OR_ZERO(gpio_restart.reset_gpio);
    if (ret) {
    if (ret != -EPROBE_DEFER)
    dev_err(&pdev.dev, "Could not get reset GPIO\n");
    return ret;
    }
    gpio_restart.active_delay_ms = 100;
    gpio_restart.inactive_delay_ms = 100;
    gpio_restart.wait_delay_ms = 3000;
    ret = of_property_read_u32(pdev.dev.of_node, "priority", &property);
    if (!ret) {
    if (property > 255)
    dev_err(&pdev.dev, "Invalid priority property: %u\n",
    property);
    else
    priority = property;
    }
    of_property_read_u32(pdev.dev.of_node, "active-delay",
    &gpio_restart.active_delay_ms);
    of_property_read_u32(pdev.dev.of_node, "inactive-delay",
    &gpio_restart.inactive_delay_ms);
    of_property_read_u32(pdev.dev.of_node, "wait-delay",
    &gpio_restart.wait_delay_ms);
    ret = devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_RESTART,
    priority,
    gpio_restart_notify,
    gpio_restart);
    if (ret) {
    dev_err(&pdev.dev, "%s: cannot register restart handler, %d\n",
    __func__, ret);
    return -ENODEV;
    }
    return 0;
    }
    static const struct of_device_id of_gpio_restart_match[] = {
    { .compatible = "gpio-restart", },
    {},
    };
    static struct platform_driver gpio_restart_driver = {
    .probe = gpio_restart_probe,
    .driver = {
    .name = "restart-gpio",
    .of_match_table = of_gpio_restart_match,
    },
    };
    module_platform_driver(gpio_restart_driver);
    MODULE_AUTHOR("David Riley <davidriley@chromium.org>");
    MODULE_DESCRIPTION("GPIO restart driver");
