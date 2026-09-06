//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-gpio.c
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
// ledtrig-gio.c - LED Trigger Based on GPIO events
//
// Copyright 2009 Felipe Balbi <me@felipebalbi.com>
// Copyright 2023 Linus Walleij <linus.walleij@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_trig_data {
    pub led: *mut led_classdev,
    pub /: *mut *mut unsigned desired_brightness; / desired brightness when led is on,
    pub /: *mut *mut *mut gpio_desc gpiod; / gpio that triggers the led,
}

#[no_mangle]
unsafe extern "C" fn gpio_trig_irq(irq: c_int, _led: *mut c_void) -> irqreturn_t {
    static irqreturn_t gpio_trig_irq(int irq, void *_led)
    {
    struct led_classdev *led = _led;
    struct gpio_trig_data *gpio_data = led_get_trigger_data(led);
    int tmp;
    tmp = gpiod_get_value_cansleep(gpio_data.gpiod);
    if (tmp) {
    if (gpio_data.desired_brightness)
    led_set_brightness_nosleep(gpio_data.led,
    gpio_data.desired_brightness);
    else
    led_set_brightness_nosleep(gpio_data.led, LED_FULL);
    } else {
    led_set_brightness_nosleep(gpio_data.led, LED_OFF);
    }
    return IRQ_HANDLED;
    }
    static ssize_t desired_brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct gpio_trig_data *gpio_data = led_trigger_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", gpio_data.desired_brightness);
    }
    static ssize_t desired_brightness_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t n)
    {
    struct gpio_trig_data *gpio_data = led_trigger_get_drvdata(dev);
    u8 desired_brightness;
    int ret;
    ret = kstrtou8(buf, 10, &desired_brightness);
    if (ret)
    return ret;
    gpio_data.desired_brightness = desired_brightness;
    return n;
    }
    static DEVICE_ATTR_RW(desired_brightness);
    static struct attribute *gpio_trig_attrs[] = {
    &dev_attr_desired_brightness.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(gpio_trig);
#[no_mangle]
unsafe extern "C" fn gpio_trig_activate(led: *mut led_classdev) -> c_int {
    static int gpio_trig_activate(struct led_classdev *led)
    {
    struct gpio_trig_data *gpio_data;
    struct device *dev = led.dev;
    int ret;
    gpio_data = kzalloc_obj(*gpio_data);
    if (!gpio_data)
    return -ENOMEM;
//
// The generic property "trigger-sources" is followed,
// and we hope that this is a GPIO.
//
    gpio_data.gpiod = gpiod_get_optional(dev, "trigger-sources",
    GPIOD_IN | GPIOD_FLAGS_BIT_NONEXCLUSIVE);
    if (IS_ERR(gpio_data.gpiod)) {
    ret = PTR_ERR(gpio_data.gpiod);
    kfree(gpio_data);
    return ret;
    }
    if (!gpio_data.gpiod) {
    dev_err(dev, "no valid GPIO for the trigger\n");
    kfree(gpio_data);
    return -EINVAL;
    }
    gpiod_set_consumer_name(gpio_data.gpiod, "led-trigger");
    gpio_data.led = led;
    led_set_trigger_data(led, gpio_data);
    ret = request_threaded_irq(gpiod_to_irq(gpio_data.gpiod), core::ptr::null_mut(), gpio_trig_irq,
    IRQF_ONESHOT | IRQF_SHARED | IRQF_TRIGGER_RISING
    | IRQF_TRIGGER_FALLING, "ledtrig-gpio", led);
    if (ret) {
    dev_err(dev, "request_irq failed with error %d\n", ret);
    gpiod_put(gpio_data.gpiod);
    kfree(gpio_data);
    return ret;
    }
// Finally update the LED to initial status
    gpio_trig_irq(0, led);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_trig_deactivate(led: *mut led_classdev) {
    static void gpio_trig_deactivate(struct led_classdev *led)
    {
    struct gpio_trig_data *gpio_data = led_get_trigger_data(led);
    free_irq(gpiod_to_irq(gpio_data.gpiod), led);
    gpiod_put(gpio_data.gpiod);
    kfree(gpio_data);
    }
    static struct led_trigger gpio_led_trigger = {
    .name		= "gpio",
    .activate	= gpio_trig_activate,
    .deactivate	= gpio_trig_deactivate,
    .groups		= gpio_trig_groups,
    };
    module_led_trigger(gpio_led_trigger);
    MODULE_AUTHOR("Felipe Balbi <me@felipebalbi.com>");
    MODULE_DESCRIPTION("GPIO LED trigger");
    MODULE_LICENSE("GPL v2");
