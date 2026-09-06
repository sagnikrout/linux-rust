//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-menf21bmc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MEN 14F021P00 Board Management Controller (BMC) LEDs Driver.
//
// This is the core LED driver of the MEN 14F021P00 BMC.
// There are four LEDs available which can be switched on and off.
// STATUS LED, HOT SWAP LED, USER LED 1, USER LED 2
//
// Copyright (C) 2014 MEN Mikro Elektronik Nuernberg GmbH
//

pub const BMC_CMD_LED_GET_SET: c_uint = 0xA0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menf21bmc_led {
    pub cdev: led_classdev,
    pub led_bit: u8,
    pub name: *const c_char,
    pub i2c_client: *mut i2c_client,
}

    static struct menf21bmc_led leds[] = {
    {
    .name = "menf21bmc:led_status",
    .led_bit = BMC_BIT_LED_STATUS,
    },
    {
    .name = "menf21bmc:led_hotswap",
    .led_bit = BMC_BIT_LED_HOTSWAP,
    },
    {
    .name = "menf21bmc:led_user1",
    .led_bit = BMC_BIT_LED_USER1,
    },
    {
    .name = "menf21bmc:led_user2",
    .led_bit = BMC_BIT_LED_USER2,
    }
    };
    static DEFINE_MUTEX(led_lock);
    static void
    menf21bmc_led_set(struct led_classdev *led_cdev, enum led_brightness value)
    {
    int led_val;
    struct menf21bmc_led *led = container_of(led_cdev,
    struct menf21bmc_led, cdev);
    mutex_lock(&led_lock);
    led_val = i2c_smbus_read_byte_data(led.i2c_client,
    BMC_CMD_LED_GET_SET);
    if (led_val < 0)
    goto err_out;
    if (value == LED_OFF)
    led_val &= ~led.led_bit;
    else
    led_val |= led.led_bit;
    i2c_smbus_write_byte_data(led.i2c_client,
    BMC_CMD_LED_GET_SET, led_val);
    err_out:
    mutex_unlock(&led_lock);
    }
#[no_mangle]
unsafe extern "C" fn menf21bmc_led_probe(pdev: *mut platform_device) -> c_int {
    static int menf21bmc_led_probe(struct platform_device *pdev)
    {
    int i;
    int ret;
    struct i2c_client *i2c_client = to_i2c_client(pdev.dev.parent);
    for (i = 0; i < ARRAY_SIZE(leds); i++) {
    leds[i].cdev.name = leds[i].name;
    leds[i].cdev.brightness_set = menf21bmc_led_set;
    leds[i].i2c_client = i2c_client;
    ret = devm_led_classdev_register(&pdev.dev, &leds[i].cdev);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to register LED device\n");
    return ret;
    }
    }
    dev_info(&pdev.dev, "MEN 140F21P00 BMC LED device enabled\n");
    return 0;
    }
    static struct platform_driver menf21bmc_led = {
    .probe		= menf21bmc_led_probe,
    .driver		= {
    .name		= "menf21bmc_led",
    },
    };
    module_platform_driver(menf21bmc_led);
    MODULE_AUTHOR("Andreas Werner <andreas.werner@men.de>");
    MODULE_DESCRIPTION("MEN 14F021P00 BMC led driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:menf21bmc_led");
