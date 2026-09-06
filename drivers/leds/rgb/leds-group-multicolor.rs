//! Automatically rewritten from C to Rust
//! Source: drivers/leds/rgb/leds-group-multicolor.c
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
// Multi-color LED built with monochromatic LED devices
//
// This driver groups several monochromatic LED devices in a single multicolor LED device.
//
// Compared to handling this grouping in user-space, the benefits are:
// - The state of the monochromatic LED relative to each other is always consistent.
// - The sysfs interface of the LEDs can be used for the group as a whole.
//
// Copyright 2023 Jean-Jacques Hiblot <jjhiblot@traphandler.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct leds_multicolor {
    pub mc_cdev: led_classdev_mc,
    pub monochromatics: *mut led_classdev,
}

#[no_mangle]
unsafe extern "C" fn leds_gmc_set(cdev: *mut led_classdev, brightness: enum led_brightness) -> c_int {
    static int leds_gmc_set(struct led_classdev *cdev, enum led_brightness brightness)
    {
    struct led_classdev_mc *mc_cdev = lcdev_to_mccdev(cdev);
    struct leds_multicolor *priv = container_of(mc_cdev, struct leds_multicolor, mc_cdev);
    let mut group_max_brightness: c_uint = mc_cdev.led_cdev.max_brightness;
    int i;
    for (i = 0; i < mc_cdev.num_colors; i++) {
    struct led_classdev *mono = priv.monochromatics[i];
    let mut mono_max_brightness: c_uint = mono.max_brightness;
    let mut intensity: c_uint = mc_cdev.subled_info[i].intensity;
    int mono_brightness;
//
// Scale the brightness according to relative intensity of the
// color AND the max brightness of the monochromatic LED.
//
    mono_brightness = DIV_ROUND_CLOSEST(brightness * intensity * mono_max_brightness,
    group_max_brightness * group_max_brightness);
    led_set_brightness(mono, mono_brightness);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn restore_sysfs_write_access(data: *mut c_void) {
    static void restore_sysfs_write_access(void *data)
    {
    struct led_classdev *led_cdev = data;
// Restore the write access to the LED
    mutex_lock(&led_cdev.led_access);
    led_sysfs_enable(led_cdev);
    mutex_unlock(&led_cdev.led_access);
    }
#[no_mangle]
unsafe extern "C" fn leds_gmc_probe(pdev: *mut platform_device) -> c_int {
    static int leds_gmc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    let mut init_data: led_init_data = {};
    struct led_classdev *cdev;
    struct mc_subled *subled;
    struct leds_multicolor *priv;
    let mut max_brightness: c_uint = 0;
    int i, ret, count = 0, common_flags = 0;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    for (;;) {
    struct led_classdev *led_cdev;
    led_cdev = devm_of_led_get_optional(dev, count);
    if (IS_ERR(led_cdev))
    return dev_err_probe(dev, PTR_ERR(led_cdev), "Unable to get LED #%d",
    count);
    if (!led_cdev)
    break;
    priv.monochromatics = devm_krealloc_array(dev, priv.monochromatics,
    count + 1, sizeof(*priv.monochromatics),
    GFP_KERNEL);
    if (!priv.monochromatics)
    return -ENOMEM;
    common_flags |= led_cdev.flags;
    priv.monochromatics[count] = led_cdev;
    max_brightness = max(max_brightness, led_cdev.max_brightness);
    count++;
    }
    subled = devm_kcalloc(dev, count, sizeof(*subled), GFP_KERNEL);
    if (!subled)
    return -ENOMEM;
    priv.mc_cdev.subled_info = subled;
    for (i = 0; i < count; i++) {
    struct led_classdev *led_cdev = priv.monochromatics[i];
    subled[i].color_index = led_cdev.color;
// Configure the LED intensity to its maximum
    subled[i].intensity = max_brightness;
    }
// Initialise the multicolor's LED class device
    cdev = &priv.mc_cdev.led_cdev;
    cdev.brightness_set_blocking = leds_gmc_set;
    cdev.max_brightness = max_brightness;
    cdev.color = LED_COLOR_ID_MULTI;
    priv.mc_cdev.num_colors = count;
// we only need suspend/resume if a sub-led requests it
    if (common_flags & LED_CORE_SUSPENDRESUME)
    cdev.flags = LED_CORE_SUSPENDRESUME;
    init_data.fwnode = dev_fwnode(dev);
    ret = devm_led_classdev_multicolor_register_ext(dev, &priv.mc_cdev, &init_data);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register multicolor LED for %s.\n",
    cdev.name);
    ret = leds_gmc_set(cdev, cdev.brightness);
    if (ret)
    return dev_err_probe(dev, ret, "failed to set LED value for %s.", cdev.name);
    for (i = 0; i < count; i++) {
    struct led_classdev *led_cdev = priv.monochromatics[i];
//
// Make the individual LED sysfs interface read-only to prevent the user
// to change the brightness of the individual LEDs of the group.
//
    mutex_lock(&led_cdev.led_access);
    led_sysfs_disable(led_cdev);
    mutex_unlock(&led_cdev.led_access);
// Restore the write access to the LED sysfs when the group is destroyed
    devm_add_action_or_reset(dev, restore_sysfs_write_access, led_cdev);
    }
    return 0;
    }
    static const struct of_device_id of_leds_group_multicolor_match[] = {
    { .compatible = "leds-group-multicolor" },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_leds_group_multicolor_match);
    static struct platform_driver leds_group_multicolor_driver = {
    .probe		= leds_gmc_probe,
    .driver		= {
    .name	= "leds_group_multicolor",
    .of_match_table = of_leds_group_multicolor_match,
    }
    };
    module_platform_driver(leds_group_multicolor_driver);
    MODULE_AUTHOR("Jean-Jacques Hiblot <jjhiblot@traphandler.com>");
    MODULE_DESCRIPTION("LEDs group multicolor driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:leds-group-multicolor");
