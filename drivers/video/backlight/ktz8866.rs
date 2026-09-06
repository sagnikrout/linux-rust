//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ktz8866.c
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
// Backlight driver for the Kinetic KTZ8866
//
// Copyright (C) 2022, 2023 Jianhua Lu <lujianhua000@gmail.com>
//

pub const DEFAULT_BRIGHTNESS: c_int = 1500;
pub const MAX_BRIGHTNESS: c_int = 2047;
pub const REG_MAX: c_uint = 0x15;
// reg
pub const DEVICE_ID: c_uint = 0x01;
pub const BL_CFG1: c_uint = 0x02;
pub const BL_CFG2: c_uint = 0x03;
pub const BL_BRT_LSB: c_uint = 0x04;
pub const BL_BRT_MSB: c_uint = 0x05;
pub const BL_EN: c_uint = 0x08;
pub const LCD_BIAS_CFG1: c_uint = 0x09;
pub const LCD_BIAS_CFG2: c_uint = 0x0A;
pub const LCD_BIAS_CFG3: c_uint = 0x0B;
pub const LCD_BOOST_CFG: c_uint = 0x0C;
pub const OUTP_CFG: c_uint = 0x0D;
pub const OUTN_CFG: c_uint = 0x0E;
pub const FLAG: c_uint = 0x0F;
pub const BL_OPTION1: c_uint = 0x10;
pub const BL_OPTION2: c_uint = 0x11;
pub const PWM2DIG_LSBs: c_uint = 0x12;
pub const PWM2DIG_MSBs: c_uint = 0x13;
pub const BL_DIMMING: c_uint = 0x14;
pub const PWM_RAMP_TIME: c_uint = 0x15;
// definition

pub const LCD_BIAS_EN: c_uint = 0x9F;
pub const PWM_HYST: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktz8866 {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub led_on: bool,
    pub enable_gpio: *mut gpio_desc,
}

    static const struct regmap_config ktz8866_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    };
    static int ktz8866_write(struct ktz8866 *ktz, unsigned int reg,
    unsigned int val)
    {
    return regmap_write(ktz.regmap, reg, val);
    }
    static int ktz8866_update_bits(struct ktz8866 *ktz, unsigned int reg,
    unsigned int mask, unsigned int val)
    {
    return regmap_update_bits(ktz.regmap, reg, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn ktz8866_backlight_update_status(backlight_dev: *mut backlight_device) -> c_int {
    static int ktz8866_backlight_update_status(struct backlight_device *backlight_dev)
    {
    struct ktz8866 *ktz = bl_get_data(backlight_dev);
    let mut brightness: c_uint = backlight_get_brightness(backlight_dev);
    if (!ktz.led_on && brightness > 0) {
    ktz8866_update_bits(ktz, BL_EN, BL_EN_BIT, BL_EN_BIT);
    ktz.led_on = true;
    } else if (brightness == 0) {
    ktz8866_update_bits(ktz, BL_EN, BL_EN_BIT, 0);
    ktz.led_on = false;
    }
// Set brightness
    ktz8866_write(ktz, BL_BRT_LSB, brightness & 0x7);
    ktz8866_write(ktz, BL_BRT_MSB, (brightness >> 3) & 0xFF);
    return 0;
    }
    static const struct backlight_ops ktz8866_backlight_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = ktz8866_backlight_update_status,
    };
#[no_mangle]
unsafe extern "C" fn ktz8866_init(ktz: *mut ktz8866) {
    static void ktz8866_init(struct ktz8866 *ktz)
    {
    let mut val: c_uint = 0;
    if (!of_property_read_u32(ktz.client.dev.of_node, "current-num-sinks", &val))
    ktz8866_write(ktz, BL_EN, BIT(val) - 1);
    else
// Enable all 6 current sinks if the number of current sinks isn't specified.
    ktz8866_write(ktz, BL_EN, BIT(6) - 1);
    if (!of_property_read_u32(ktz.client.dev.of_node, "kinetic,current-ramp-delay-ms", &val)) {
    if (val <= 128)
    ktz8866_write(ktz, BL_CFG2, BIT(7) | (ilog2(val) << 3) | PWM_HYST);
    else
    ktz8866_write(ktz, BL_CFG2, BIT(7) | ((5 + val / 64) << 3) | PWM_HYST);
    }
    if (!of_property_read_u32(ktz.client.dev.of_node, "kinetic,led-enable-ramp-delay-ms", &val)) {
    if (val == 0)
    ktz8866_write(ktz, BL_DIMMING, 0);
    else {
    let mut ramp_off_time: c_uint = ilog2(val) + 1;
    let mut ramp_on_time: c_uint = ramp_off_time << 4;
    ktz8866_write(ktz, BL_DIMMING, ramp_on_time | ramp_off_time);
    }
    }
    if (of_property_read_bool(ktz.client.dev.of_node, "kinetic,enable-lcd-bias"))
    ktz8866_write(ktz, LCD_BIAS_CFG1, LCD_BIAS_EN);
    }
#[no_mangle]
unsafe extern "C" fn ktz8866_probe(client: *mut i2c_client) -> c_int {
    static int ktz8866_probe(struct i2c_client *client)
    {
    struct backlight_device *backlight_dev;
    struct backlight_properties props;
    struct ktz8866 *ktz;
    let mut ret: c_int = 0;
    ktz = devm_kzalloc(&client.dev, sizeof(*ktz), GFP_KERNEL);
    if (!ktz)
    return -ENOMEM;
    ktz.client = client;
    ktz.regmap = devm_regmap_init_i2c(client, &ktz8866_regmap_config);
    if (IS_ERR(ktz.regmap))
    return dev_err_probe(&client.dev, PTR_ERR(ktz.regmap), "failed to init regmap\n");
    ret = devm_regulator_get_enable(&client.dev, "vddpos");
    if (ret)
    return dev_err_probe(&client.dev, ret, "get regulator vddpos failed\n");
    ret = devm_regulator_get_enable(&client.dev, "vddneg");
    if (ret)
    return dev_err_probe(&client.dev, ret, "get regulator vddneg failed\n");
    ktz.enable_gpio = devm_gpiod_get_optional(&client.dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(ktz.enable_gpio))
    return PTR_ERR(ktz.enable_gpio);
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = MAX_BRIGHTNESS;
    props.brightness = DEFAULT_BRIGHTNESS;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    backlight_dev = devm_backlight_device_register(&client.dev, "ktz8866-backlight",
    &client.dev, ktz, &ktz8866_backlight_ops, &props);
    if (IS_ERR(backlight_dev))
    return dev_err_probe(&client.dev, PTR_ERR(backlight_dev),
    "failed to register backlight device\n");
    ktz8866_init(ktz);
    i2c_set_clientdata(client, backlight_dev);
    backlight_update_status(backlight_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktz8866_remove(client: *mut i2c_client) {
    static void ktz8866_remove(struct i2c_client *client)
    {
    struct backlight_device *backlight_dev = i2c_get_clientdata(client);
    backlight_dev.props.brightness = 0;
    backlight_update_status(backlight_dev);
    }
    static const struct i2c_device_id ktz8866_ids[] = {
    { .name = "ktz8866" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ktz8866_ids);
    static const struct of_device_id ktz8866_match_table[] = {
    {
    .compatible = "kinetic,ktz8866",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ktz8866_match_table);
    static struct i2c_driver ktz8866_driver = {
    .driver = {
    .name = "ktz8866",
    .of_match_table = ktz8866_match_table,
    },
    .probe = ktz8866_probe,
    .remove = ktz8866_remove,
    .id_table = ktz8866_ids,
    };
    module_i2c_driver(ktz8866_driver);
    MODULE_DESCRIPTION("Kinetic KTZ8866 Backlight Driver");
    MODULE_AUTHOR("Jianhua Lu <lujianhua000@gmail.com>");
    MODULE_LICENSE("GPL");
