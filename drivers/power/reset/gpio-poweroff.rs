//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/gpio-poweroff.c
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
// Toggles a GPIO pin to power down a device
//
// Jamie Lentin <jm@lentin.co.uk>
// Andrew Lunn <andrew@lunn.ch>
//
// Copyright (C) 2012 Jamie Lentin
//

pub const DEFAULT_TIMEOUT_MS: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_poweroff {
    pub reset_gpio: *mut gpio_desc,
    pub timeout_ms: u32,
    pub active_delay_ms: u32,
    pub inactive_delay_ms: u32,
}

#[no_mangle]
unsafe extern "C" fn gpio_poweroff_do_poweroff(data: *mut sys_off_data) -> c_int {
    static int gpio_poweroff_do_poweroff(struct sys_off_data *data)
    {
    struct gpio_poweroff *gpio_poweroff = data.cb_data;
// drive it active, also inactive->active edge
    gpiod_direction_output(gpio_poweroff.reset_gpio, 1);
    mdelay(gpio_poweroff.active_delay_ms);
// drive inactive, also active->inactive edge
    gpiod_set_value_cansleep(gpio_poweroff.reset_gpio, 0);
    mdelay(gpio_poweroff.inactive_delay_ms);
// drive it active, also inactive->active edge
    gpiod_set_value_cansleep(gpio_poweroff.reset_gpio, 1);
// give it some time
    mdelay(gpio_poweroff.timeout_ms);
//
// If code reaches this point, it means that gpio-poweroff has failed
// to actually power off the system.
// Warn the user that the attempt to poweroff via gpio-poweroff
// has gone wrong.
//
    WARN(1, "Failed to poweroff via gpio-poweroff mechanism\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn gpio_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_poweroff_probe(struct platform_device *pdev)
    {
    struct gpio_poweroff *gpio_poweroff;
    let mut input: bool = false;
    enum gpiod_flags flags;
    let mut priority: c_int = SYS_OFF_PRIO_DEFAULT;
    int ret;
    gpio_poweroff = devm_kzalloc(&pdev.dev, sizeof(*gpio_poweroff), GFP_KERNEL);
    if (!gpio_poweroff)
    return -ENOMEM;
    input = device_property_read_bool(&pdev.dev, "input");
    if (input)
    flags = GPIOD_IN;
    else
    flags = GPIOD_OUT_LOW;
    gpio_poweroff.active_delay_ms = 100;
    gpio_poweroff.inactive_delay_ms = 100;
    gpio_poweroff.timeout_ms = DEFAULT_TIMEOUT_MS;
    device_property_read_u32(&pdev.dev, "active-delay-ms", &gpio_poweroff.active_delay_ms);
    device_property_read_u32(&pdev.dev, "inactive-delay-ms",
    &gpio_poweroff.inactive_delay_ms);
    device_property_read_u32(&pdev.dev, "timeout-ms", &gpio_poweroff.timeout_ms);
    device_property_read_u32(&pdev.dev, "priority", &priority);
    if (priority > 255) {
    dev_err(&pdev.dev, "Invalid priority property: %u\n", priority);
    return -EINVAL;
    }
    gpio_poweroff.reset_gpio = devm_gpiod_get(&pdev.dev, core::ptr::null_mut(), flags);
    if (IS_ERR(gpio_poweroff.reset_gpio))
    return PTR_ERR(gpio_poweroff.reset_gpio);
    ret = devm_register_sys_off_handler(&pdev.dev, SYS_OFF_MODE_POWER_OFF,
    priority, gpio_poweroff_do_poweroff, gpio_poweroff);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Cannot register poweroff handler\n");
    return 0;
    }
    static const struct of_device_id of_gpio_poweroff_match[] = {
    { .compatible = "gpio-poweroff", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_gpio_poweroff_match);
    static struct platform_driver gpio_poweroff_driver = {
    .probe = gpio_poweroff_probe,
    .driver = {
    .name = "poweroff-gpio",
    .of_match_table = of_gpio_poweroff_match,
    },
    };
    module_platform_driver(gpio_poweroff_driver);
    MODULE_AUTHOR("Jamie Lentin <jm@lentin.co.uk>");
    MODULE_DESCRIPTION("GPIO poweroff driver");
    MODULE_ALIAS("platform:poweroff-gpio");
