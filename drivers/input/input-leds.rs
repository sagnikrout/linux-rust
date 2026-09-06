//! Automatically rewritten from C to Rust
//! Source: drivers/input/input-leds.c
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
// LED support for the input layer
//
// Copyright 2010-2015 Samuel Thibault <samuel.thibault@ens-lyon.org>
//

    static const struct {
    const char *name;
    const char *trigger;
    } input_led_info[LED_CNT] = {
    [LED_NUML]	= { "numlock", VT_TRIGGER("kbd-numlock") },
    [LED_CAPSL]	= { "capslock", VT_TRIGGER("kbd-capslock") },
    [LED_SCROLLL]	= { "scrolllock", VT_TRIGGER("kbd-scrolllock") },
    [LED_COMPOSE]	= { "compose" },
    [LED_KANA]	= { "kana", VT_TRIGGER("kbd-kanalock") },
    [LED_SLEEP]	= { "sleep" } ,
    [LED_SUSPEND]	= { "suspend" },
    [LED_MUTE]	= { "mute", AUDIO_TRIGGER("audio-mute") },
    [LED_MISC]	= { "misc" },
    [LED_MAIL]	= { "mail" },
    [LED_CHARGING]	= { "charging" },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_led {
    pub cdev: led_classdev,
    pub handle: *mut input_handle,
    pub /: *mut *mut *mut unsigned int code; / One of LED_ constants,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_leds {
    pub handle: input_handle,
    pub num_leds: c_uint,
    pub __counted_by(num_leds): input_led leds[],
}

#[no_mangle]
unsafe extern "C" fn input_leds_brightness_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness input_leds_brightness_get(struct led_classdev *cdev)
    {
    struct input_led *led = container_of(cdev, struct input_led, cdev);
    struct input_dev *input = led.handle.dev;
    return test_bit(led.code, input.led) ? cdev.max_brightness : 0;
    }
    static void input_leds_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct input_led *led = container_of(cdev, struct input_led, cdev);
    input_inject_event(led.handle, EV_LED, led.code, !!brightness);
    }
    static void input_leds_event(struct input_handle *handle, unsigned int type,
    unsigned int code, int value)
    {
    }
#[no_mangle]
unsafe extern "C" fn input_leds_get_count(dev: *mut input_dev) -> c_int {
    static int input_leds_get_count(struct input_dev *dev)
    {
    unsigned int led_code;
    let mut count: c_int = 0;
    for_each_set_bit(led_code, dev.ledbit, LED_CNT)
    if (input_led_info[led_code].name)
    count++;
    return count;
    }
    static int input_leds_connect(struct input_handler *handler,
    struct input_dev *dev,
    const struct input_device_id *id)
    {
    struct input_leds *leds;
    struct input_led *led;
    unsigned int num_leds;
    unsigned int led_code;
    int led_no;
    int error;
    num_leds = input_leds_get_count(dev);
    if (!num_leds)
    return -ENXIO;
    leds = kzalloc_flex(*leds, leds, num_leds);
    if (!leds)
    return -ENOMEM;
    leds.num_leds = num_leds;
    leds.handle.dev = dev;
    leds.handle.handler = handler;
    leds.handle.name = "leds";
    leds.handle.private = leds;
    error = input_register_handle(&leds.handle);
    if (error)
    goto err_free_mem;
    error = input_open_device(&leds.handle);
    if (error)
    goto err_unregister_handle;
    led_no = 0;
    for_each_set_bit(led_code, dev.ledbit, LED_CNT) {
    if (!input_led_info[led_code].name)
    continue;
    led = &leds.leds[led_no];
    led.handle = &leds.handle;
    led.code = led_code;
    led.cdev.name = kasprintf(GFP_KERNEL, "%s::%s",
    dev_name(&dev.dev),
    input_led_info[led_code].name);
    if (!led.cdev.name) {
    error = -ENOMEM;
    goto err_unregister_leds;
    }
    led.cdev.max_brightness = 1;
    led.cdev.brightness_get = input_leds_brightness_get;
    led.cdev.brightness_set = input_leds_brightness_set;
    led.cdev.default_trigger = input_led_info[led_code].trigger;
    error = led_classdev_register(&dev.dev, &led.cdev);
    if (error) {
    dev_err(&dev.dev, "failed to register LED %s: %d\n",
    led.cdev.name, error);
    kfree(led.cdev.name);
    goto err_unregister_leds;
    }
    led_no++;
    }
    return 0;
    err_unregister_leds:
    while (--led_no >= 0) {
    struct input_led *led = &leds.leds[led_no];
    led_classdev_unregister(&led.cdev);
    kfree(led.cdev.name);
    }
    input_close_device(&leds.handle);
    err_unregister_handle:
    input_unregister_handle(&leds.handle);
    err_free_mem:
    kfree(leds);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn input_leds_disconnect(handle: *mut input_handle) {
    static void input_leds_disconnect(struct input_handle *handle)
    {
    struct input_leds *leds = handle.private;
    int i;
    for (i = 0; i < leds.num_leds; i++) {
    struct input_led *led = &leds.leds[i];
    led_classdev_unregister(&led.cdev);
    kfree(led.cdev.name);
    }
    input_close_device(handle);
    input_unregister_handle(handle);
    kfree(leds);
    }
    static const struct input_device_id input_leds_ids[] = {
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT,
    .evbit = { BIT_MASK(EV_LED) },
    },
    { },
    };
    MODULE_DEVICE_TABLE(input, input_leds_ids);
    static struct input_handler input_leds_handler = {
    .event =	input_leds_event,
    .connect =	input_leds_connect,
    .disconnect =	input_leds_disconnect,
    .name =		"leds",
    .id_table =	input_leds_ids,
    };
#[no_mangle]
unsafe extern "C" fn input_leds_init() -> int __init {
    static int __init input_leds_init(void)
    {
    return input_register_handler(&input_leds_handler);
    }
    module_init(input_leds_init);
#[no_mangle]
unsafe extern "C" fn input_leds_exit() -> void __exit {
    static void __exit input_leds_exit(void)
    {
    input_unregister_handler(&input_leds_handler);
    }
    module_exit(input_leds_exit);
    MODULE_AUTHOR("Samuel Thibault <samuel.thibault@ens-lyon.org>");
    MODULE_AUTHOR("Dmitry Torokhov <dmitry.torokhov@gmail.com>");
    MODULE_DESCRIPTION("Input . LEDs Bridge");
    MODULE_LICENSE("GPL v2");
