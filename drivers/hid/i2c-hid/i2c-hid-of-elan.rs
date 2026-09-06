//! Automatically rewritten from C to Rust
//! Source: drivers/hid/i2c-hid/i2c-hid-of-elan.c
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
// Driver for Elan touchscreens that use the i2c-hid protocol.
//
// Copyright 2020 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elan_i2c_hid_chip_data {
    pub post_gpio_reset_on_delay_ms: c_uint,
    pub post_gpio_reset_off_delay_ms: c_uint,
    pub post_power_delay_ms: c_uint,
    pub hid_descriptor_address: u16,
    pub main_supply_name: *const c_char,
    pub power_after_backlight: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_hid_of_elan {
    pub ops: i2chid_ops,
    pub vcc33: *mut regulator,
    pub vccio: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub no_reset_on_power_off: bool,
    pub chip_data: *const elan_i2c_hid_chip_data,
}

#[no_mangle]
unsafe extern "C" fn elan_i2c_hid_power_up(ops: *mut i2chid_ops) -> c_int {
    static int elan_i2c_hid_power_up(struct i2chid_ops *ops)
    {
    struct i2c_hid_of_elan *ihid_elan =
    container_of(ops, struct i2c_hid_of_elan, ops);
    int ret;
    gpiod_set_value_cansleep(ihid_elan.reset_gpio, 1);
    if (ihid_elan.vcc33) {
    ret = regulator_enable(ihid_elan.vcc33);
    if (ret)
    goto err_deassert_reset;
    }
    ret = regulator_enable(ihid_elan.vccio);
    if (ret)
    goto err_disable_vcc33;
    if (ihid_elan.chip_data.post_power_delay_ms)
    msleep(ihid_elan.chip_data.post_power_delay_ms);
    gpiod_set_value_cansleep(ihid_elan.reset_gpio, 0);
    if (ihid_elan.chip_data.post_gpio_reset_on_delay_ms)
    msleep(ihid_elan.chip_data.post_gpio_reset_on_delay_ms);
    return 0;
    err_disable_vcc33:
    if (ihid_elan.vcc33)
    regulator_disable(ihid_elan.vcc33);
    err_deassert_reset:
    if (ihid_elan.no_reset_on_power_off)
    gpiod_set_value_cansleep(ihid_elan.reset_gpio, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn elan_i2c_hid_power_down(ops: *mut i2chid_ops) {
    static void elan_i2c_hid_power_down(struct i2chid_ops *ops)
    {
    struct i2c_hid_of_elan *ihid_elan =
    container_of(ops, struct i2c_hid_of_elan, ops);
//
// Do not assert reset when the hardware allows for it to remain
// deasserted regardless of the state of the (shared) power supply to
// avoid wasting power when the supply is left on.
//
    if (!ihid_elan.no_reset_on_power_off)
    gpiod_set_value_cansleep(ihid_elan.reset_gpio, 1);
    if (ihid_elan.chip_data.post_gpio_reset_off_delay_ms)
    msleep(ihid_elan.chip_data.post_gpio_reset_off_delay_ms);
    regulator_disable(ihid_elan.vccio);
    if (ihid_elan.vcc33)
    regulator_disable(ihid_elan.vcc33);
    }
#[no_mangle]
unsafe extern "C" fn i2c_hid_of_elan_probe(client: *mut i2c_client) -> c_int {
    static int i2c_hid_of_elan_probe(struct i2c_client *client)
    {
    struct i2c_hid_of_elan *ihid_elan;
    int ret;
    let mut quirks: u32 = 0;
    ihid_elan = devm_kzalloc(&client.dev, sizeof(*ihid_elan), GFP_KERNEL);
    if (!ihid_elan)
    return -ENOMEM;
    ihid_elan.ops.power_up = elan_i2c_hid_power_up;
    ihid_elan.ops.power_down = elan_i2c_hid_power_down;
// Start out with reset asserted
    ihid_elan.reset_gpio =
    devm_gpiod_get_optional(&client.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ihid_elan.reset_gpio))
    return PTR_ERR(ihid_elan.reset_gpio);
    ihid_elan.no_reset_on_power_off = of_property_read_bool(client.dev.of_node,
    "no-reset-on-power-off");
    ihid_elan.vccio = devm_regulator_get(&client.dev, "vccio");
    if (IS_ERR(ihid_elan.vccio)) {
    ret = PTR_ERR(ihid_elan.vccio);
    goto err_deassert_reset;
    }
    ihid_elan.chip_data = device_get_match_data(&client.dev);
    if (ihid_elan.chip_data.main_supply_name) {
    ihid_elan.vcc33 = devm_regulator_get(&client.dev,
    ihid_elan.chip_data.main_supply_name);
    if (IS_ERR(ihid_elan.vcc33)) {
    ret = PTR_ERR(ihid_elan.vcc33);
    goto err_deassert_reset;
    }
    }
    if (ihid_elan.chip_data.power_after_backlight)
    quirks = HID_QUIRK_POWER_ON_AFTER_BACKLIGHT;
    ret = i2c_hid_core_probe(client, &ihid_elan.ops,
    ihid_elan.chip_data.hid_descriptor_address,
    quirks);
    if (ret)
    goto err_deassert_reset;
    return 0;
    err_deassert_reset:
    if (ihid_elan.no_reset_on_power_off)
    gpiod_set_value_cansleep(ihid_elan.reset_gpio, 0);
    return ret;
    }
    static const struct elan_i2c_hid_chip_data elan_ekth6915_chip_data = {
    .post_power_delay_ms = 1,
    .post_gpio_reset_on_delay_ms = 300,
    .hid_descriptor_address = 0x0001,
    .main_supply_name = "vcc33",
    .power_after_backlight = true,
    };
    static const struct elan_i2c_hid_chip_data elan_ekth6a12nay_chip_data = {
    .post_power_delay_ms = 10,
    .post_gpio_reset_on_delay_ms = 300,
    .hid_descriptor_address = 0x0001,
    .main_supply_name = "vcc33",
    .power_after_backlight = true,
    };
    static const struct elan_i2c_hid_chip_data focaltech_ft8112_chip_data = {
    .post_power_delay_ms = 10,
    .post_gpio_reset_on_delay_ms = 150,
    .hid_descriptor_address = 0x0001,
    .main_supply_name = "vcc33",
    };
    static const struct elan_i2c_hid_chip_data ilitek_ili9882t_chip_data = {
    .post_power_delay_ms = 1,
    .post_gpio_reset_on_delay_ms = 200,
    .post_gpio_reset_off_delay_ms = 65,
    .hid_descriptor_address = 0x0001,
//
// this touchscreen is tightly integrated with the panel and assumes
// that the relevant power rails (other than the IO rail) have already
// been turned on by the panel driver because we're a panel follower.
//
    .main_supply_name = core::ptr::null_mut(),
    };
    static const struct elan_i2c_hid_chip_data ilitek_ili2901_chip_data = {
    .post_power_delay_ms = 10,
    .post_gpio_reset_on_delay_ms = 100,
    .hid_descriptor_address = 0x0001,
    .main_supply_name = "vcc33",
    };
    static const struct elan_i2c_hid_chip_data parade_tc3408_chip_data = {
    .post_power_delay_ms = 10,
    .post_gpio_reset_on_delay_ms = 300,
    .hid_descriptor_address = 0x0001,
    .main_supply_name = "vcc33",
    };
    static const struct of_device_id elan_i2c_hid_of_match[] = {
    { .compatible = "elan,ekth6915", .data = &elan_ekth6915_chip_data },
    { .compatible = "elan,ekth6a12nay", .data = &elan_ekth6a12nay_chip_data },
    { .compatible = "focaltech,ft8112", .data = &focaltech_ft8112_chip_data },
    { .compatible = "ilitek,ili9882t", .data = &ilitek_ili9882t_chip_data },
    { .compatible = "ilitek,ili2901", .data = &ilitek_ili2901_chip_data },
    { .compatible = "parade,tc3408", .data = &parade_tc3408_chip_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, elan_i2c_hid_of_match);
    static struct i2c_driver elan_i2c_hid_ts_driver = {
    .driver = {
    .name	= "i2c_hid_of_elan",
    .pm	= &i2c_hid_core_pm,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(elan_i2c_hid_of_match),
    },
    .probe		= i2c_hid_of_elan_probe,
    .remove		= i2c_hid_core_remove,
    .shutdown	= i2c_hid_core_shutdown,
    };
    module_i2c_driver(elan_i2c_hid_ts_driver);
    MODULE_AUTHOR("Douglas Anderson <dianders@chromium.org>");
    MODULE_DESCRIPTION("Elan i2c-hid touchscreen driver");
    MODULE_LICENSE("GPL");
