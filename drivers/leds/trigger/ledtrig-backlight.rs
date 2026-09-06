//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-backlight.c
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
// Backlight emulation LED trigger
//
// Copyright 2008 (C) Rodolfo Giometti <giometti@linux.it>
// Copyright 2008 (C) Eurotech S.p.A. <info@eurotech.it>
//

pub const BLANK: c_int = 1;
pub const UNBLANK: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_trig_notifier {
    pub led: *mut led_classdev,
    pub brightness: c_int,
    pub old_status: c_int,
    pub invert: unsigned,
    pub entry: list_head,
}

    static DEFINE_MUTEX(ledtrig_backlight_list_mutex);
    static LIST_HEAD(ledtrig_backlight_list);
#[no_mangle]
unsafe extern "C" fn ledtrig_backlight_notify_blank(n: *mut bl_trig_notifier, new_status: c_int) {
    static void ledtrig_backlight_notify_blank(struct bl_trig_notifier *n, int new_status)
    {
    struct led_classdev *led = n.led;
    if (new_status == n.old_status)
    return;
    if ((n.old_status == UNBLANK) ^ n.invert) {
    n.brightness = led.brightness;
    led_set_brightness_nosleep(led, LED_OFF);
    } else {
    led_set_brightness_nosleep(led, n.brightness);
    }
    n.old_status = new_status;
    }
#[no_mangle]
pub unsafe extern "C" fn ledtrig_backlight_blank(blank: bool) {
    void ledtrig_backlight_blank(bool blank)
    {
    struct bl_trig_notifier *n;
    let mut new_status: c_int = blank ? BLANK : UNBLANK;
    guard(mutex)(&ledtrig_backlight_list_mutex);
    list_for_each_entry(n, &ledtrig_backlight_list, entry)
    ledtrig_backlight_notify_blank(n, new_status);
    }
    EXPORT_SYMBOL(ledtrig_backlight_blank);
    static ssize_t bl_trig_invert_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct bl_trig_notifier *n = led_trigger_get_drvdata(dev);
    return sprintf(buf, "%u\n", n.invert);
    }
    static ssize_t bl_trig_invert_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t num)
    {
    struct led_classdev *led = led_trigger_get_led(dev);
    struct bl_trig_notifier *n = led_trigger_get_drvdata(dev);
    unsigned long invert;
    int ret;
    ret = kstrtoul(buf, 10, &invert);
    if (ret < 0)
    return ret;
    if (invert > 1)
    return -EINVAL;
    n.invert = invert;
// After inverting, we need to update the LED.
    if ((n.old_status == BLANK) ^ n.invert)
    led_set_brightness_nosleep(led, LED_OFF);
    else
    led_set_brightness_nosleep(led, n.brightness);
    return num;
    }
    static DEVICE_ATTR(inverted, 0644, bl_trig_invert_show, bl_trig_invert_store);
    static struct attribute *bl_trig_attrs[] = {
    &dev_attr_inverted.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(bl_trig);
#[no_mangle]
unsafe extern "C" fn bl_trig_activate(led: *mut led_classdev) -> c_int {
    static int bl_trig_activate(struct led_classdev *led)
    {
    struct bl_trig_notifier *n;
    n = kzalloc_obj(struct bl_trig_notifier);
    if (!n)
    return -ENOMEM;
    led_set_trigger_data(led, n);
    n.led = led;
    n.brightness = led.brightness;
    n.old_status = UNBLANK;
    guard(mutex)(&ledtrig_backlight_list_mutex);
    list_add(&n.entry, &ledtrig_backlight_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bl_trig_deactivate(led: *mut led_classdev) {
    static void bl_trig_deactivate(struct led_classdev *led)
    {
    struct bl_trig_notifier *n = led_get_trigger_data(led);
    guard(mutex)(&ledtrig_backlight_list_mutex);
    list_del(&n.entry);
    kfree(n);
    }
    static struct led_trigger bl_led_trigger = {
    .name		= "backlight",
    .activate	= bl_trig_activate,
    .deactivate	= bl_trig_deactivate,
    .groups		= bl_trig_groups,
    };
    module_led_trigger(bl_led_trigger);
    MODULE_AUTHOR("Rodolfo Giometti <giometti@linux.it>");
    MODULE_DESCRIPTION("Backlight emulation LED trigger");
    MODULE_LICENSE("GPL v2");
