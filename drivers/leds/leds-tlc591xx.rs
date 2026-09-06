//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-tlc591xx.c
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
// Copyright 2014 Belkin Inc.
// Copyright 2015 Andrew Lunn <andrew@lunn.ch>
//

pub const TLC591XX_MAX_LEDS: c_int = 16;
pub const TLC591XX_MAX_BRIGHTNESS: c_int = 256;
pub const TLC591XX_REG_MODE1: c_uint = 0x00;
pub const MODE1_RESPON_ADDR_MASK: c_uint = 0xF0;

pub const TLC591XX_REG_MODE2: c_uint = 0x01;

pub const TLC591XX_REG_GRPPWM: c_uint = 0x12;
pub const TLC591XX_REG_GRPFREQ: c_uint = 0x13;
// LED Driver Output State, determine the source that drives LED outputs
pub const LEDOUT_OFF: c_uint = 0x0	/* Output LOW */;
pub const LEDOUT_ON: c_uint = 0x1	/* Output HI-Z */;
pub const LEDOUT_DIM: c_uint = 0x2	/* Dimming */;
pub const LEDOUT_BLINK: c_uint = 0x3	/* Blinking */;
pub const LEDOUT_MASK: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlc591xx_led {
    pub active: bool,
    pub led_no: c_uint,
    pub ldev: led_classdev,
    pub priv: *mut tlc591xx_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlc591xx_priv {
    pub leds: [tlc591xx_led; TLC591XX_MAX_LEDS],
    pub regmap: *mut regmap,
    pub reg_ledout_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlc591xx {
    pub max_leds: c_uint,
    pub reg_ledout_offset: c_uint,
}

    static const struct tlc591xx tlc59116 = {
    .max_leds = 16,
    .reg_ledout_offset = 0x14,
    };
    static const struct tlc591xx tlc59108 = {
    .max_leds = 8,
    .reg_ledout_offset = 0x0c,
    };
    static int
    tlc591xx_set_mode(struct regmap *regmap, u8 mode)
    {
    int err;
    u8 val;
    err = regmap_write(regmap, TLC591XX_REG_MODE1, MODE1_NORMAL_MODE);
    if (err)
    return err;
    val = MODE2_OCH_STOP | mode;
    return regmap_write(regmap, TLC591XX_REG_MODE2, val);
    }
    static int
    tlc591xx_set_ledout(struct tlc591xx_priv *priv, struct tlc591xx_led *led,
    u8 val)
    {
    let mut i: c_uint = (led.led_no % 4) * 2;
    let mut mask: c_uint = LEDOUT_MASK << i;
    let mut addr: c_uint = priv.reg_ledout_offset + (led.led_no >> 2);
    val = val << i;
    return regmap_update_bits(priv.regmap, addr, mask, val);
    }
    static int
    tlc591xx_set_pwm(struct tlc591xx_priv *priv, struct tlc591xx_led *led,
    u8 brightness)
    {
    let mut pwm: u8 = TLC591XX_REG_PWM(led.led_no);
    return regmap_write(priv.regmap, pwm, brightness);
    }
    static int
    tlc591xx_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct tlc591xx_led *led = ldev_to_led(led_cdev);
    struct tlc591xx_priv *priv = led.priv;
    int err;
    switch ((int)brightness) {
    case 0:
    err = tlc591xx_set_ledout(priv, led, LEDOUT_OFF);
    break;
    case TLC591XX_MAX_BRIGHTNESS:
    err = tlc591xx_set_ledout(priv, led, LEDOUT_ON);
    break;
    default:
    err = tlc591xx_set_ledout(priv, led, LEDOUT_DIM);
    if (!err)
    err = tlc591xx_set_pwm(priv, led, brightness);
    }
    return err;
    }
    static const struct regmap_config tlc591xx_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x1e,
    };
    static const struct of_device_id of_tlc591xx_leds_match[] __maybe_unused = {
    { .compatible = "ti,tlc59116",
    .data = &tlc59116 },
    { .compatible = "ti,tlc59108",
    .data = &tlc59108 },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_tlc591xx_leds_match);
    static int
    tlc591xx_probe(struct i2c_client *client)
    {
    struct device_node *np;
    struct device *dev = &client.dev;
    const struct tlc591xx *tlc591xx;
    struct tlc591xx_priv *priv;
    int err, count, reg;
    np = dev_of_node(dev);
    if (!np)
    return -ENODEV;
    tlc591xx = device_get_match_data(dev);
    if (!tlc591xx)
    return -ENODEV;
    count = of_get_available_child_count(np);
    if (!count || count > tlc591xx.max_leds)
    return -EINVAL;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init_i2c(client, &tlc591xx_regmap);
    if (IS_ERR(priv.regmap)) {
    err = PTR_ERR(priv.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", err);
    return err;
    }
    priv.reg_ledout_offset = tlc591xx.reg_ledout_offset;
    i2c_set_clientdata(client, priv);
    err = tlc591xx_set_mode(priv.regmap, MODE2_DIM);
    if (err < 0)
    return err;
    for_each_available_child_of_node_scoped(np, child) {
    struct tlc591xx_led *led;
    let mut init_data: led_init_data = {};
    init_data.fwnode = of_fwnode_handle(child);
    err = of_property_read_u32(child, "reg", &reg);
    if (err)
    return err;
    if (reg < 0 || reg >= tlc591xx.max_leds ||
    priv.leds[reg].active)
    return -EINVAL;
    led = &priv.leds[reg];
    led.active = true;
    led.priv = priv;
    led.led_no = reg;
    led.ldev.brightness_set_blocking = tlc591xx_brightness_set;
    led.ldev.max_brightness = TLC591XX_MAX_BRIGHTNESS;
    err = devm_led_classdev_register_ext(dev, &led.ldev,
    &init_data);
    if (err < 0)
    return dev_err_probe(dev, err,
    "couldn't register LED %s\n",
    led.ldev.name);
    }
    return 0;
    }
    static const struct i2c_device_id tlc591xx_id[] = {
    { .name = "tlc59116" },
    { .name = "tlc59108" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tlc591xx_id);
    static struct i2c_driver tlc591xx_driver = {
    .driver = {
    .name = "tlc591xx",
    .of_match_table = of_match_ptr(of_tlc591xx_leds_match),
    },
    .probe = tlc591xx_probe,
    .id_table = tlc591xx_id,
    };
    module_i2c_driver(tlc591xx_driver);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TLC591XX LED driver");
