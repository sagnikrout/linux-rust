//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-max77705.c
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
// Based on leds-max77650 driver
//
// LED driver for MAXIM 77705 PMIC.
// Copyright (C) 2025 Dzmitry Sankouski <dsankouski@gmail.org>
//

pub const MAX77705_LED_NUM_LEDS: c_int = 4;

pub const MAX77705_LED_MAX_BRIGHTNESS: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77705_led {
    pub cdev: led_classdev,
    pub mcdev: led_classdev_mc,
    pub regmap: *mut regmap,
    pub subled_info: *mut mc_subled,
}

    static const struct regmap_config max77705_leds_regmap_config = {
    .reg_base = MAX77705_RGBLED_REG_BASE,
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX77705_LED_REG_END,
    };
    static int max77705_rgb_blink(struct led_classdev *cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct max77705_led *led = container_of(cdev, struct max77705_led, cdev);
    int value, on_value, off_value;
    if (*delay_on < MAX77705_RGB_DELAY_100_STEP)
    on_value = 0;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_100_STEP_LIM: *mut *mut delay_on <) -> else {
    else if (*delay_on < MAX77705_RGB_DELAY_100_STEP_LIM)
    on_value = *delay_on / MAX77705_RGB_DELAY_100_STEP - 1;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_250_STEP_LIM: *mut *mut delay_on <) -> else {
    else if (*delay_on < MAX77705_RGB_DELAY_250_STEP_LIM)
    on_value = (*delay_on - MAX77705_RGB_DELAY_100_STEP_LIM) /
    MAX77705_RGB_DELAY_250_STEP +
    MAX77705_RGB_DELAY_100_STEP_COUNT;
    else
    on_value = 15;
    on_value <<= 4;
    if (*delay_off < 1)
    off_value = 0;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_500_STEP: *mut *mut delay_off <) -> else {
    else if (*delay_off < MAX77705_RGB_DELAY_500_STEP)
    off_value = 1;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_500_STEP_LIM: *mut *mut delay_off <) -> else {
    else if (*delay_off < MAX77705_RGB_DELAY_500_STEP_LIM)
    off_value = *delay_off / MAX77705_RGB_DELAY_500_STEP;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_1000_STEP_LIM: *mut *mut delay_off <) -> else {
    else if (*delay_off < MAX77705_RGB_DELAY_1000_STEP_LIM)
    off_value = (*delay_off - MAX77705_RGB_DELAY_1000_STEP_LIM) /
    MAX77705_RGB_DELAY_1000_STEP +
    MAX77705_RGB_DELAY_500_STEP_COUNT;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77705_RGB_DELAY_2000_STEP_LIM: *mut *mut delay_off <) -> else {
    else if (*delay_off < MAX77705_RGB_DELAY_2000_STEP_LIM)
    off_value = (*delay_off - MAX77705_RGB_DELAY_2000_STEP_LIM) /
    MAX77705_RGB_DELAY_2000_STEP +
    MAX77705_RGB_DELAY_1000_STEP_COUNT;
    else
    off_value = 15;
    value = on_value | off_value;
    return regmap_write(led.regmap, MAX77705_RGBLED_REG_LEDBLNK, value);
    }
    static int max77705_led_brightness_set(struct regmap *regmap, struct mc_subled *subled,
    int num_colors)
    {
    int ret;
    for (int i = 0; i < num_colors; i++) {
    unsigned int channel, brightness;
    channel = subled[i].channel;
    brightness = subled[i].brightness;
    if (brightness == LED_OFF) {
// Flash OFF
    ret = regmap_update_bits(regmap,
    MAX77705_RGBLED_REG_LEDEN,
    MAX77705_LED_EN_MASK << MAX77705_LED_EN_SHIFT(channel), 0);
    } else {
// Set current
    ret = regmap_write(regmap, MAX77705_LED_REG_BRIGHTNESS(channel),
    brightness);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(regmap,
    MAX77705_RGBLED_REG_LEDEN,
    LED_ON << MAX77705_LED_EN_SHIFT(channel),
    MAX77705_LED_EN_MASK << MAX77705_LED_EN_SHIFT(channel));
    }
    }
    return ret;
    }
    static int max77705_led_brightness_set_single(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct max77705_led *led = container_of(cdev, struct max77705_led, cdev);
    led.subled_info.brightness = brightness;
    return max77705_led_brightness_set(led.regmap, led.subled_info, 1);
    }
    static int max77705_led_brightness_set_multi(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct led_classdev_mc *mcdev = lcdev_to_mccdev(cdev);
    struct max77705_led *led = container_of(mcdev, struct max77705_led, mcdev);
    led_mc_calc_color_components(mcdev, brightness);
    return max77705_led_brightness_set(led.regmap, led.mcdev.subled_info, mcdev.num_colors);
    }
    static int max77705_parse_subled(struct device *dev, struct fwnode_handle *np,
    struct mc_subled *info)
    {
    let mut color: u32 = LED_COLOR_ID_GREEN;
    u32 reg;
    int ret;
    ret = fwnode_property_read_u32(np, "reg", &reg);
    if (ret || !reg || reg >= MAX77705_LED_NUM_LEDS)
    return dev_err_probe(dev, -EINVAL, "invalid \"reg\" of %pOFn\n", np);
    info.channel = reg;
    ret = fwnode_property_read_u32(np, "color", &color);
    if (ret < 0 && ret != -EINVAL)
    return dev_err_probe(dev, ret,
    "failed to parse \"color\" of %pOF\n", np);
    info.color_index = color;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max77705_add_led(dev: *mut device, regmap: *mut regmap, np: *mut fwnode_handle) -> c_int {
    static int max77705_add_led(struct device *dev, struct regmap *regmap, struct fwnode_handle *np)
    {
    int ret, i = 0;
    unsigned int color, reg;
    struct max77705_led *led;
    struct led_classdev *cdev;
    struct mc_subled *info;
    struct fwnode_handle *child;
    let mut init_data: led_init_data = {};
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    ret = fwnode_property_read_u32(np, "color", &color);
    if (ret < 0 && ret != -EINVAL)
    return dev_err_probe(dev, ret,
    "failed to parse \"color\" of %pOF\n", np);
    led.regmap = regmap;
    init_data.fwnode = np;
    if (color == LED_COLOR_ID_RGB) {
    let mut num_channels: c_int = of_get_available_child_count(to_of_node(np));
    ret = fwnode_property_read_u32(np, "reg", &reg);
    if (ret || reg >= MAX77705_LED_NUM_LEDS)
    return -EINVAL;
    info = devm_kcalloc(dev, num_channels, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    cdev = &led.mcdev.led_cdev;
    cdev.max_brightness = MAX77705_LED_MAX_BRIGHTNESS;
    cdev.brightness_set_blocking = max77705_led_brightness_set_multi;
    cdev.blink_set = max77705_rgb_blink;
    fwnode_for_each_child_node(np, child) {
    ret = max77705_parse_subled(dev, child, &info[i]);
    if (ret < 0)
    return ret;
    info[i].intensity = 0;
    i++;
    }
    led.mcdev.subled_info = info;
    led.mcdev.num_colors = num_channels;
    led.cdev = *cdev;
    ret = devm_led_classdev_multicolor_register_ext(dev, &led.mcdev, &init_data);
    if (ret)
    return ret;
    ret = max77705_led_brightness_set_multi(&led.cdev, LED_OFF);
    if (ret)
    return ret;
    } else {
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    max77705_parse_subled(dev, np, info);
    led.subled_info = info;
    led.cdev.brightness_set_blocking = max77705_led_brightness_set_single;
    led.cdev.blink_set = max77705_rgb_blink;
    led.cdev.max_brightness = MAX77705_LED_MAX_BRIGHTNESS;
    ret = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (ret)
    return ret;
    ret = max77705_led_brightness_set_single(&led.cdev, LED_OFF);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max77705_led_probe(pdev: *mut platform_device) -> c_int {
    static int max77705_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct i2c_client *i2c = to_i2c_client(pdev.dev.parent);
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_i2c(i2c, &max77705_leds_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Failed to register LEDs regmap\n");
    device_for_each_child_node_scoped(dev, child) {
    ret = max77705_add_led(dev, regmap, child);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct of_device_id max77705_led_of_match[] = {
    { .compatible = "maxim,max77705-rgb" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max77705_led_of_match);
    static struct platform_driver max77705_led_driver = {
    .driver = {
    .name = "max77705-led",
    .of_match_table = max77705_led_of_match,
    },
    .probe = max77705_led_probe,
    };
    module_platform_driver(max77705_led_driver);
    MODULE_DESCRIPTION("Maxim MAX77705 LED driver");
    MODULE_AUTHOR("Dzmitry Sankouski <dsankouski@gmail.com>");
    MODULE_LICENSE("GPL");
