//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-bd2606mvv.c
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
// Copyright (C) 2023 Andreas Kemnade
//
// Datasheet:
// https://fscdn.rohm.com/en/products/databook/datasheet/ic/power/led_driver/bd2606mvv_1-e.pdf
//
// If LED brightness cannot be controlled independently due to shared
// brightness registers, max_brightness is set to 1 and only on/off
// is possible for the affected LED pair.
//

pub const BD2606_MAX_LEDS: c_int = 6;
pub const BD2606_MAX_BRIGHTNESS: c_int = 63;
pub const BD2606_REG_PWRCNT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd2606mvv_led {
    pub led_no: c_uint,
    pub ldev: led_classdev,
    pub priv: *mut bd2606mvv_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd2606mvv_priv {
    pub leds: [bd2606mvv_led; BD2606_MAX_LEDS],
    pub regmap: *mut regmap,
}

    static int
    bd2606mvv_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct bd2606mvv_led *led = ldev_to_led(led_cdev);
    struct bd2606mvv_priv *priv = led.priv;
    int err;
    if (brightness == 0)
    return regmap_update_bits(priv.regmap,
    BD2606_REG_PWRCNT,
    1 << led.led_no,
    0);
// shared brightness register
    err = regmap_write(priv.regmap, led.led_no / 2,
    led_cdev.max_brightness == 1 ?
    BD2606_MAX_BRIGHTNESS : brightness);
    if (err)
    return err;
    return regmap_update_bits(priv.regmap,
    BD2606_REG_PWRCNT,
    1 << led.led_no,
    1 << led.led_no);
    }
    static const struct regmap_config bd2606mvv_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x3,
    };
#[no_mangle]
unsafe extern "C" fn bd2606mvv_probe(client: *mut i2c_client) -> c_int {
    static int bd2606mvv_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct bd2606mvv_priv *priv;
    struct fwnode_handle *led_fwnodes[BD2606_MAX_LEDS] = { 0 };
    int active_pairs[BD2606_MAX_LEDS / 2] = { 0 };
    int err, reg;
    int i, j;
    if (!dev_fwnode(dev))
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init_i2c(client, &bd2606mvv_regmap);
    if (IS_ERR(priv.regmap)) {
    err = PTR_ERR(priv.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", err);
    return err;
    }
    i2c_set_clientdata(client, priv);
    device_for_each_child_node_scoped(dev, child) {
    struct bd2606mvv_led *led;
    err = fwnode_property_read_u32(child, "reg", &reg);
    if (err)
    return err;
    if (reg < 0 || reg >= BD2606_MAX_LEDS || led_fwnodes[reg])
    return -EINVAL;
    led = &priv.leds[reg];
    led_fwnodes[reg] = fwnode_handle_get(child);
    active_pairs[reg / 2]++;
    led.priv = priv;
    led.led_no = reg;
    led.ldev.brightness_set_blocking = bd2606mvv_brightness_set;
    led.ldev.max_brightness = BD2606_MAX_BRIGHTNESS;
    }
    for (i = 0; i < BD2606_MAX_LEDS; i++) {
    let mut init_data: led_init_data = {};
    if (!led_fwnodes[i])
    continue;
    init_data.fwnode = led_fwnodes[i];
// Check whether brightness can be independently adjusted.
    if (active_pairs[i / 2] == 2)
    priv.leds[i].ldev.max_brightness = 1;
    err = devm_led_classdev_register_ext(dev,
    &priv.leds[i].ldev,
    &init_data);
    if (err < 0) {
    for (j = i; j < BD2606_MAX_LEDS; j++)
    fwnode_handle_put(led_fwnodes[j]);
    return dev_err_probe(dev, err,
    "couldn't register LED %s\n",
    priv.leds[i].ldev.name);
    }
    }
    return 0;
    }
    static const struct of_device_id __maybe_unused of_bd2606mvv_leds_match[] = {
    { .compatible = "rohm,bd2606mvv", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_bd2606mvv_leds_match);
    static struct i2c_driver bd2606mvv_driver = {
    .driver   = {
    .name    = "leds-bd2606mvv",
    .of_match_table = of_match_ptr(of_bd2606mvv_leds_match),
    },
    .probe = bd2606mvv_probe,
    };
    module_i2c_driver(bd2606mvv_driver);
    MODULE_AUTHOR("Andreas Kemnade <andreas@kemnade.info>");
    MODULE_DESCRIPTION("BD2606 LED driver");
    MODULE_LICENSE("GPL");
