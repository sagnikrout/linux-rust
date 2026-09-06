//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-heartbeat.c
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
// LED Heartbeat Trigger
//
// Copyright (C) 2006 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
//
// Based on Richard Purdie's ledtrig-timer.c and some arch's
// CONFIG_HEARTBEAT code.
//

    static int panic_heartbeats;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct heartbeat_trig_data {
    pub led_cdev: *mut led_classdev,
    pub phase: c_uint,
    pub period: c_uint,
    pub timer: timer_list,
    pub invert: c_uint,
}

#[no_mangle]
unsafe extern "C" fn led_heartbeat_function(t: *mut timer_list) {
    static void led_heartbeat_function(struct timer_list *t)
    {
    struct heartbeat_trig_data *heartbeat_data =
    timer_container_of(heartbeat_data, t, timer);
    struct led_classdev *led_cdev;
    let mut brightness: c_ulong = LED_OFF;
    let mut delay: c_ulong = 0;
    led_cdev = heartbeat_data.led_cdev;
    if (unlikely(panic_heartbeats)) {
    led_set_brightness_nosleep(led_cdev, LED_OFF);
    return;
    }
    if (test_and_clear_bit(LED_BLINK_BRIGHTNESS_CHANGE, &led_cdev.work_flags))
    led_cdev.blink_brightness = led_cdev.new_blink_brightness;
// acts like an actual heart beat -- ie thump-thump-pause...
    switch (heartbeat_data.phase) {
    case 0:
//
// The hyperbolic function below modifies the
// heartbeat period length in dependency of the
// current (1min) load. It goes through the points
// f(0)=1260, f(1)=860, f(5)=510, f(inf)->300.
//
    heartbeat_data.period = 300 +
    (6720 << FSHIFT) / (5 * avenrun[0] + (7 << FSHIFT));
    heartbeat_data.period =
    msecs_to_jiffies(heartbeat_data.period);
    delay = msecs_to_jiffies(70);
    heartbeat_data.phase++;
    if (!heartbeat_data.invert)
    brightness = led_cdev.blink_brightness;
    break;
    case 1:
    delay = heartbeat_data.period / 4 - msecs_to_jiffies(70);
    heartbeat_data.phase++;
    if (heartbeat_data.invert)
    brightness = led_cdev.blink_brightness;
    break;
    case 2:
    delay = msecs_to_jiffies(70);
    heartbeat_data.phase++;
    if (!heartbeat_data.invert)
    brightness = led_cdev.blink_brightness;
    break;
    default:
    delay = heartbeat_data.period - heartbeat_data.period / 4 -
    msecs_to_jiffies(70);
    heartbeat_data.phase = 0;
    if (heartbeat_data.invert)
    brightness = led_cdev.blink_brightness;
    break;
    }
    led_set_brightness_nosleep(led_cdev, brightness);
    mod_timer(&heartbeat_data.timer, jiffies + delay);
    }
    static ssize_t led_invert_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct heartbeat_trig_data *heartbeat_data =
    led_trigger_get_drvdata(dev);
    return sprintf(buf, "%u\n", heartbeat_data.invert);
    }
    static ssize_t led_invert_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct heartbeat_trig_data *heartbeat_data =
    led_trigger_get_drvdata(dev);
    unsigned long state;
    int ret;
    ret = kstrtoul(buf, 0, &state);
    if (ret)
    return ret;
    heartbeat_data.invert = !!state;
    return size;
    }
    static DEVICE_ATTR(invert, 0644, led_invert_show, led_invert_store);
    static struct attribute *heartbeat_trig_attrs[] = {
    &dev_attr_invert.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(heartbeat_trig);
#[no_mangle]
unsafe extern "C" fn heartbeat_trig_activate(led_cdev: *mut led_classdev) -> c_int {
    static int heartbeat_trig_activate(struct led_classdev *led_cdev)
    {
    struct heartbeat_trig_data *heartbeat_data;
    heartbeat_data = kzalloc_obj(*heartbeat_data);
    if (!heartbeat_data)
    return -ENOMEM;
    led_set_trigger_data(led_cdev, heartbeat_data);
    heartbeat_data.led_cdev = led_cdev;
    timer_setup(&heartbeat_data.timer, led_heartbeat_function, 0);
    heartbeat_data.phase = 0;
    if (!led_cdev.blink_brightness)
    led_cdev.blink_brightness = led_cdev.max_brightness;
    led_heartbeat_function(&heartbeat_data.timer);
    set_bit(LED_BLINK_SW, &led_cdev.work_flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn heartbeat_trig_deactivate(led_cdev: *mut led_classdev) {
    static void heartbeat_trig_deactivate(struct led_classdev *led_cdev)
    {
    struct heartbeat_trig_data *heartbeat_data =
    led_get_trigger_data(led_cdev);
    timer_shutdown_sync(&heartbeat_data.timer);
    kfree(heartbeat_data);
    clear_bit(LED_BLINK_SW, &led_cdev.work_flags);
    }
    static struct led_trigger heartbeat_led_trigger = {
    .name     = "heartbeat",
    .activate = heartbeat_trig_activate,
    .deactivate = heartbeat_trig_deactivate,
    .groups = heartbeat_trig_groups,
    };
    static int heartbeat_reboot_notifier(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    led_trigger_unregister(&heartbeat_led_trigger);
    return NOTIFY_DONE;
    }
    static int heartbeat_panic_notifier(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    panic_heartbeats = 1;
    return NOTIFY_DONE;
    }
    static struct notifier_block heartbeat_reboot_nb = {
    .notifier_call = heartbeat_reboot_notifier,
    };
    static struct notifier_block heartbeat_panic_nb = {
    .notifier_call = heartbeat_panic_notifier,
    };
#[no_mangle]
unsafe extern "C" fn heartbeat_trig_init() -> int __init {
    static int __init heartbeat_trig_init(void)
    {
    let mut rc: c_int = led_trigger_register(&heartbeat_led_trigger);
    if (!rc) {
    atomic_notifier_chain_register(&panic_notifier_list,
    &heartbeat_panic_nb);
    register_reboot_notifier(&heartbeat_reboot_nb);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn heartbeat_trig_exit() -> void __exit {
    static void __exit heartbeat_trig_exit(void)
    {
    unregister_reboot_notifier(&heartbeat_reboot_nb);
    atomic_notifier_chain_unregister(&panic_notifier_list,
    &heartbeat_panic_nb);
    led_trigger_unregister(&heartbeat_led_trigger);
    }
    module_init(heartbeat_trig_init);
    module_exit(heartbeat_trig_exit);
    MODULE_AUTHOR("Atsushi Nemoto <anemo@mba.ocn.ne.jp>");
    MODULE_DESCRIPTION("Heartbeat LED trigger");
    MODULE_LICENSE("GPL v2");
