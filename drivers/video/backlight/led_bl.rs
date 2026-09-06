//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/led_bl.c
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
// Copyright (C) 2015-2019 Texas Instruments Incorporated -  http://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//
// Based on pwm_bl.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_bl_data {
    pub dev: *mut device,
    pub bl_dev: *mut backlight_device,
    pub leds: *mut led_classdev,
    pub enabled: bool,
    pub nb_leds: c_int,
    pub levels: *mut c_uint,
    pub default_brightness: c_uint,
    pub max_brightness: c_uint,
}

#[no_mangle]
unsafe extern "C" fn led_bl_set_brightness(priv: *mut led_bl_data, level: c_int) {
    static void led_bl_set_brightness(struct led_bl_data *priv, int level)
    {
    int i;
    int bkl_brightness;
    if (priv.levels)
    bkl_brightness = priv.levels[level];
    else
    bkl_brightness = level;
    for (i = 0; i < priv.nb_leds; i++)
    led_set_brightness(priv.leds[i], bkl_brightness);
    priv.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn led_bl_power_off(priv: *mut led_bl_data) {
    static void led_bl_power_off(struct led_bl_data *priv)
    {
    int i;
    if (!priv.enabled)
    return;
    for (i = 0; i < priv.nb_leds; i++)
    led_set_brightness(priv.leds[i], LED_OFF);
    priv.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn led_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int led_bl_update_status(struct backlight_device *bl)
    {
    struct led_bl_data *priv = bl_get_data(bl);
    let mut brightness: c_int = backlight_get_brightness(bl);
    if (brightness > 0)
    led_bl_set_brightness(priv, brightness);
    else
    led_bl_power_off(priv);
    return 0;
    }
    static const struct backlight_ops led_bl_ops = {
    .update_status	= led_bl_update_status,
    };
    static int led_bl_get_leds(struct device *dev,
    struct led_bl_data *priv)
    {
    int i, nb_leds, ret;
    struct device_node *node = dev.of_node;
    struct led_classdev **leds;
    unsigned int max_brightness;
    unsigned int default_brightness;
    ret = of_count_phandle_with_args(node, "leds", core::ptr::null_mut());
    if (ret < 0) {
    dev_err(dev, "Unable to get led count\n");
    return -EINVAL;
    }
    nb_leds = ret;
    if (nb_leds < 1) {
    dev_err(dev, "At least one LED must be specified!\n");
    return -EINVAL;
    }
    leds = devm_kcalloc(dev, nb_leds, sizeof(struct led_classdev *),
    GFP_KERNEL);
    if (!leds)
    return -ENOMEM;
    for (i = 0; i < nb_leds; i++) {
    leds[i] = devm_of_led_get(dev, i);
    if (IS_ERR(leds[i]))
    return PTR_ERR(leds[i]);
    }
// check that the LEDs all have the same brightness range
    max_brightness = leds[0].max_brightness;
    for (i = 1; i < nb_leds; i++) {
    if (max_brightness != leds[i].max_brightness) {
    dev_err(dev, "LEDs must have identical ranges\n");
    return -EINVAL;
    }
    }
// get the default brightness from the first LED from the list
    default_brightness = leds[0].brightness;
    priv.nb_leds = nb_leds;
    priv.leds = leds;
    priv.max_brightness = max_brightness;
    priv.default_brightness = default_brightness;
    return 0;
    }
    static int led_bl_parse_levels(struct device *dev,
    struct led_bl_data *priv)
    {
    struct device_node *node = dev.of_node;
    int num_levels;
    u32 value;
    int ret;
    if (!node)
    return -ENODEV;
    num_levels = of_property_count_u32_elems(node, "brightness-levels");
    if (num_levels > 1) {
    int i;
    unsigned int db;
    u32 *levels = core::ptr::null_mut();
    levels = devm_kcalloc(dev, num_levels, sizeof(u32),
    GFP_KERNEL);
    if (!levels)
    return -ENOMEM;
    ret = of_property_read_u32_array(node, "brightness-levels",
    levels,
    num_levels);
    if (ret < 0)
    return ret;
//
// Try to map actual LED brightness to backlight brightness
// level
//
    db = priv.default_brightness;
    for (i = 0 ; i < num_levels; i++) {
    if ((i && db > levels[i-1]) && db <= levels[i])
    break;
    }
    priv.default_brightness = i;
    priv.max_brightness = num_levels - 1;
    priv.levels = levels;
    } else if (num_levels >= 0)
    dev_warn(dev, "Not enough levels defined\n");
    ret = of_property_read_u32(node, "default-brightness-level", &value);
    if (!ret && value <= priv.max_brightness)
    priv.default_brightness = value;
#[no_mangle]
pub unsafe extern "C" fn if(priv->max_brightness: !ret && value >) -> else {
    else if (!ret  && value > priv.max_brightness)
    dev_warn(dev, "Invalid default brightness. Ignoring it\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn led_bl_probe(pdev: *mut platform_device) -> c_int {
    static int led_bl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct led_bl_data *priv;
    int ret, i;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.dev = &pdev.dev;
    ret = led_bl_get_leds(&pdev.dev, priv);
    if (ret)
    return ret;
    ret = led_bl_parse_levels(&pdev.dev, priv);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to parse DT data\n");
    return ret;
    }
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = priv.max_brightness;
    props.brightness = priv.default_brightness;
    props.power = (priv.default_brightness > 0) ? BACKLIGHT_POWER_OFF :
    BACKLIGHT_POWER_ON;
    priv.bl_dev = backlight_device_register(dev_name(&pdev.dev),
    &pdev.dev, priv, &led_bl_ops, &props);
    if (IS_ERR(priv.bl_dev)) {
    dev_err(&pdev.dev, "Failed to register backlight\n");
    return PTR_ERR(priv.bl_dev);
    }
    for (i = 0; i < priv.nb_leds; i++) {
    struct device_link *link;
    link = device_link_add(&pdev.dev, priv.leds[i].dev.parent,
    DL_FLAG_AUTOREMOVE_CONSUMER);
    if (!link) {
    dev_err(&pdev.dev, "Failed to add devlink (consumer %s, supplier %s)\n",
    dev_name(&pdev.dev), dev_name(priv.leds[i].dev.parent));
    backlight_device_unregister(priv.bl_dev);
    return -EINVAL;
    }
    }
    for (i = 0; i < priv.nb_leds; i++) {
    mutex_lock(&priv.leds[i].led_access);
    led_sysfs_disable(priv.leds[i]);
    mutex_unlock(&priv.leds[i].led_access);
    }
    backlight_update_status(priv.bl_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn led_bl_remove(pdev: *mut platform_device) {
    static void led_bl_remove(struct platform_device *pdev)
    {
    struct led_bl_data *priv = platform_get_drvdata(pdev);
    struct backlight_device *bl = priv.bl_dev;
    int i;
    backlight_device_unregister(bl);
    led_bl_power_off(priv);
    for (i = 0; i < priv.nb_leds; i++) {
    mutex_lock(&priv.leds[i].led_access);
    led_sysfs_enable(priv.leds[i]);
    mutex_unlock(&priv.leds[i].led_access);
    }
    }
    static const struct of_device_id led_bl_of_match[] = {
    { .compatible = "led-backlight" },
    { }
    };
    MODULE_DEVICE_TABLE(of, led_bl_of_match);
    static struct platform_driver led_bl_driver = {
    .driver		= {
    .name		= "led-backlight",
    .of_match_table	= led_bl_of_match,
    },
    .probe		= led_bl_probe,
    .remove		= led_bl_remove,
    };
    module_platform_driver(led_bl_driver);
    MODULE_DESCRIPTION("LED based Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:led-backlight");
