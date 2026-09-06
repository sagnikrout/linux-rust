//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-cros_ec.c
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
// ChromeOS EC LED Driver
//
// Copyright (C) 2024 Thomas Weißschuh <linux@weissschuh.net>
//

    static const char * const cros_ec_led_functions[] = {
    [EC_LED_ID_BATTERY_LED]            = LED_FUNCTION_CHARGING,
    [EC_LED_ID_POWER_LED]              = LED_FUNCTION_POWER,
    [EC_LED_ID_ADAPTER_LED]            = "adapter",
    [EC_LED_ID_LEFT_LED]               = "left",
    [EC_LED_ID_RIGHT_LED]              = "right",
    [EC_LED_ID_RECOVERY_HW_REINIT_LED] = "recovery-hw-reinit",
    [EC_LED_ID_SYSRQ_DEBUG_LED]        = "sysrq-debug",
    };
    static_assert(ARRAY_SIZE(cros_ec_led_functions) == EC_LED_ID_COUNT);
    static const int cros_ec_led_to_linux_id[] = {
    [EC_LED_COLOR_RED]    = LED_COLOR_ID_RED,
    [EC_LED_COLOR_GREEN]  = LED_COLOR_ID_GREEN,
    [EC_LED_COLOR_BLUE]   = LED_COLOR_ID_BLUE,
    [EC_LED_COLOR_YELLOW] = LED_COLOR_ID_YELLOW,
    [EC_LED_COLOR_WHITE]  = LED_COLOR_ID_WHITE,
    [EC_LED_COLOR_AMBER]  = LED_COLOR_ID_AMBER,
    };
    static_assert(ARRAY_SIZE(cros_ec_led_to_linux_id) == EC_LED_COLOR_COUNT);
    static const int cros_ec_linux_to_ec_id[] = {
    [LED_COLOR_ID_RED]    = EC_LED_COLOR_RED,
    [LED_COLOR_ID_GREEN]  = EC_LED_COLOR_GREEN,
    [LED_COLOR_ID_BLUE]   = EC_LED_COLOR_BLUE,
    [LED_COLOR_ID_YELLOW] = EC_LED_COLOR_YELLOW,
    [LED_COLOR_ID_WHITE]  = EC_LED_COLOR_WHITE,
    [LED_COLOR_ID_AMBER]  = EC_LED_COLOR_AMBER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_led_priv {
    pub led_mc_cdev: led_classdev_mc,
    pub cros_ec: *mut cros_ec_device,
    pub led_id: enum ec_led_id,
}

    static inline struct cros_ec_led_priv *cros_ec_led_cdev_to_priv(struct led_classdev *led_cdev)
    {
    return container_of(lcdev_to_mccdev(led_cdev), struct cros_ec_led_priv, led_mc_cdev);
    }
    union cros_ec_led_cmd_data {
    struct ec_params_led_control req;
    struct ec_response_led_control resp;
    };
    static int cros_ec_led_send_cmd(struct cros_ec_device *cros_ec,
    union cros_ec_led_cmd_data *arg)
    {
    int ret;
    ret = cros_ec_cmd(cros_ec, 1, EC_CMD_LED_CONTROL, &arg.req,
    sizeof(arg.req), &arg.resp, sizeof(arg.resp));
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_led_trigger_activate(led_cdev: *mut led_classdev) -> c_int {
    static int cros_ec_led_trigger_activate(struct led_classdev *led_cdev)
    {
    struct cros_ec_led_priv *priv = cros_ec_led_cdev_to_priv(led_cdev);
    let mut arg: union cros_ec_led_cmd_data = {};
    arg.req.led_id = priv.led_id;
    arg.req.flags = EC_LED_FLAGS_AUTO;
    return cros_ec_led_send_cmd(priv.cros_ec, &arg);
    }
    static struct led_hw_trigger_type cros_ec_led_trigger_type;
    static struct led_trigger cros_ec_led_trigger = {
    .name = "chromeos-auto",
    .trigger_type = &cros_ec_led_trigger_type,
    .activate = cros_ec_led_trigger_activate,
    };
    static int cros_ec_led_brightness_set_blocking(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct cros_ec_led_priv *priv = cros_ec_led_cdev_to_priv(led_cdev);
    let mut arg: union cros_ec_led_cmd_data = {};
    enum ec_led_colors led_color;
    struct mc_subled *subled;
    size_t i;
    led_mc_calc_color_components(&priv.led_mc_cdev, brightness);
    arg.req.led_id = priv.led_id;
    for (i = 0; i < priv.led_mc_cdev.num_colors; i++) {
    subled = &priv.led_mc_cdev.subled_info[i];
    led_color = cros_ec_linux_to_ec_id[subled.color_index];
    arg.req.brightness[led_color] = subled.brightness;
    }
    return cros_ec_led_send_cmd(priv.cros_ec, &arg);
    }
    static int cros_ec_led_count_subleds(struct device *dev,
    struct ec_response_led_control *resp,
    unsigned int *max_brightness)
    {
    unsigned int range, common_range = 0;
    let mut num_subleds: c_int = 0;
    size_t i;
    for (i = 0; i < EC_LED_COLOR_COUNT; i++) {
    range = resp.brightness_range[i];
    if (!range)
    continue;
    num_subleds++;
    if (!common_range)
    common_range = range;
    if (common_range != range) {
// The multicolor LED API expects a uniform max_brightness
    dev_err(dev, "Inconsistent LED brightness values\n");
    return -EINVAL;
    }
    }
// max_brightness = common_range;
    return num_subleds;
    }
    static const char *cros_ec_led_get_color_name(struct led_classdev_mc *led_mc_cdev)
    {
    int color;
    if (led_mc_cdev.num_colors == 1)
    color = led_mc_cdev.subled_info[0].color_index;
    else
    color = LED_COLOR_ID_MULTI;
    return led_get_color_name(color);
    }
    static int cros_ec_led_probe_one(struct device *dev, struct cros_ec_device *cros_ec,
    enum ec_led_id id)
    {
    let mut arg: union cros_ec_led_cmd_data = {};
    struct cros_ec_led_priv *priv;
    struct led_classdev *led_cdev;
    struct mc_subled *subleds;
    int i, ret, num_subleds;
    size_t subled;
    arg.req.led_id = id;
    arg.req.flags = EC_LED_FLAGS_QUERY;
    ret = cros_ec_led_send_cmd(cros_ec, &arg);
    if (ret == -EINVAL)
    return 0; /* Unknown LED, skip */
    if (ret == -EOPNOTSUPP)
    return -ENODEV;
    if (ret < 0)
    return ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    num_subleds = cros_ec_led_count_subleds(dev, &arg.resp,
    &priv.led_mc_cdev.led_cdev.max_brightness);
    if (num_subleds < 0)
    return num_subleds;
    if (num_subleds == 0)
    return 0; /* LED without any colors, skip */
    priv.cros_ec = cros_ec;
    priv.led_id = id;
    subleds = devm_kcalloc(dev, num_subleds, sizeof(*subleds), GFP_KERNEL);
    if (!subleds)
    return -ENOMEM;
    subled = 0;
    for (i = 0; i < EC_LED_COLOR_COUNT; i++) {
    if (!arg.resp.brightness_range[i])
    continue;
    subleds[subled].color_index = cros_ec_led_to_linux_id[i];
    if (subled == 0)
    subleds[subled].intensity = 100;
    subled++;
    }
    priv.led_mc_cdev.subled_info = subleds;
    priv.led_mc_cdev.num_colors = num_subleds;
    led_cdev = &priv.led_mc_cdev.led_cdev;
    led_cdev.brightness_set_blocking = cros_ec_led_brightness_set_blocking;
    led_cdev.trigger_type = &cros_ec_led_trigger_type;
    led_cdev.default_trigger = cros_ec_led_trigger.name;
    led_cdev.hw_control_trigger = cros_ec_led_trigger.name;
    led_cdev.name = devm_kasprintf(dev, GFP_KERNEL, "chromeos:%s:%s",
    cros_ec_led_get_color_name(&priv.led_mc_cdev),
    cros_ec_led_functions[id]);
    if (!led_cdev.name)
    return -ENOMEM;
    return devm_led_classdev_multicolor_register(dev, &priv.led_mc_cdev);
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_led_probe(pdev: *mut platform_device) -> c_int {
    static int cros_ec_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cros_ec_dev *ec_dev = dev_get_drvdata(dev.parent);
    struct cros_ec_device *cros_ec = ec_dev.ec_dev;
    int i, ret = 0;
    ret = devm_led_trigger_register(dev, &cros_ec_led_trigger);
    if (ret)
    return ret;
    for (i = 0; i < EC_LED_ID_COUNT; i++) {
    ret = cros_ec_led_probe_one(dev, cros_ec, i);
    if (ret)
    break;
    }
    return ret;
    }
    static const struct platform_device_id cros_ec_led_id[] = {
    { "cros-ec-led", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(platform, cros_ec_led_id);
    static struct platform_driver cros_ec_led_driver = {
    .driver.name	= "cros-ec-led",
    .probe		= cros_ec_led_probe,
    .id_table	= cros_ec_led_id,
    };
    module_platform_driver(cros_ec_led_driver);
    MODULE_DESCRIPTION("ChromeOS EC LED Driver");
    MODULE_AUTHOR("Thomas Weißschuh <linux@weissschuh.net");
    MODULE_LICENSE("GPL");
