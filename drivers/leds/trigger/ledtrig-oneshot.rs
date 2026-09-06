//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-oneshot.c
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
// One-shot LED Trigger
//
// Copyright 2012, Fabio Baltieri <fabio.baltieri@gmail.com>
//
// Based on ledtrig-timer.c by Richard Purdie <rpurdie@openedhand.com>
//

pub const DEFAULT_DELAY: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oneshot_trig_data {
    pub invert: c_uint,
}

    static ssize_t led_shot(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    struct oneshot_trig_data *oneshot_data = led_trigger_get_drvdata(dev);
    led_blink_set_oneshot(led_cdev,
    &led_cdev.blink_delay_on, &led_cdev.blink_delay_off,
    oneshot_data.invert);
// content is ignored
    return size;
    }
    static ssize_t led_invert_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct oneshot_trig_data *oneshot_data = led_trigger_get_drvdata(dev);
    return sprintf(buf, "%u\n", oneshot_data.invert);
    }
    static ssize_t led_invert_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    struct oneshot_trig_data *oneshot_data = led_trigger_get_drvdata(dev);
    unsigned long state;
    int ret;
    ret = kstrtoul(buf, 0, &state);
    if (ret)
    return ret;
    oneshot_data.invert = !!state;
    if (oneshot_data.invert)
    led_set_brightness_nosleep(led_cdev, LED_FULL);
    else
    led_set_brightness_nosleep(led_cdev, LED_OFF);
    return size;
    }
    static ssize_t led_delay_on_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    return sprintf(buf, "%lu\n", led_cdev.blink_delay_on);
    }
    static ssize_t led_delay_on_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    unsigned long state;
    int ret;
    ret = kstrtoul(buf, 0, &state);
    if (ret)
    return ret;
    led_cdev.blink_delay_on = state;
    return size;
    }
    static ssize_t led_delay_off_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    return sprintf(buf, "%lu\n", led_cdev.blink_delay_off);
    }
    static ssize_t led_delay_off_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = led_trigger_get_led(dev);
    unsigned long state;
    int ret;
    ret = kstrtoul(buf, 0, &state);
    if (ret)
    return ret;
    led_cdev.blink_delay_off = state;
    return size;
    }
    static DEVICE_ATTR(delay_on, 0644, led_delay_on_show, led_delay_on_store);
    static DEVICE_ATTR(delay_off, 0644, led_delay_off_show, led_delay_off_store);
    static DEVICE_ATTR(invert, 0644, led_invert_show, led_invert_store);
    static DEVICE_ATTR(shot, 0200, core::ptr::null_mut(), led_shot);
    static struct attribute *oneshot_trig_attrs[] = {
    &dev_attr_delay_on.attr,
    &dev_attr_delay_off.attr,
    &dev_attr_invert.attr,
    &dev_attr_shot.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(oneshot_trig);
#[no_mangle]
unsafe extern "C" fn pattern_init(led_cdev: *mut led_classdev) {
    static void pattern_init(struct led_classdev *led_cdev)
    {
    u32 *pattern;
    let mut size: c_uint = 0;
    pattern = led_get_default_pattern(led_cdev, &size);
    if (!pattern)
    goto out_default;
    if (size != 2) {
    dev_warn(led_cdev.dev,
    "Expected 2 but got %u values for delays pattern\n",
    size);
    goto out_default;
    }
    led_cdev.blink_delay_on = pattern[0];
    led_cdev.blink_delay_off = pattern[1];
    kfree(pattern);
    return;
    out_default:
    kfree(pattern);
    led_cdev.blink_delay_on = DEFAULT_DELAY;
    led_cdev.blink_delay_off = DEFAULT_DELAY;
    }
#[no_mangle]
unsafe extern "C" fn oneshot_trig_activate(led_cdev: *mut led_classdev) -> c_int {
    static int oneshot_trig_activate(struct led_classdev *led_cdev)
    {
    struct oneshot_trig_data *oneshot_data;
    oneshot_data = kzalloc_obj(*oneshot_data);
    if (!oneshot_data)
    return -ENOMEM;
    led_set_trigger_data(led_cdev, oneshot_data);
    if (led_cdev.flags & LED_INIT_DEFAULT_TRIGGER) {
    pattern_init(led_cdev);
//
// Mark as initialized even on pattern_init() error because
// any consecutive call to it would produce the same error.
//
    led_cdev.flags &= ~LED_INIT_DEFAULT_TRIGGER;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn oneshot_trig_deactivate(led_cdev: *mut led_classdev) {
    static void oneshot_trig_deactivate(struct led_classdev *led_cdev)
    {
    struct oneshot_trig_data *oneshot_data = led_get_trigger_data(led_cdev);
    kfree(oneshot_data);
// Stop blinking
    led_set_brightness(led_cdev, LED_OFF);
    }
    static struct led_trigger oneshot_led_trigger = {
    .name     = "oneshot",
    .activate = oneshot_trig_activate,
    .deactivate = oneshot_trig_deactivate,
    .groups = oneshot_trig_groups,
    };
    module_led_trigger(oneshot_led_trigger);
    MODULE_AUTHOR("Fabio Baltieri <fabio.baltieri@gmail.com>");
    MODULE_DESCRIPTION("One-shot LED trigger");
    MODULE_LICENSE("GPL v2");
