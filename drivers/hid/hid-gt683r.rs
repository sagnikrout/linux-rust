//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-gt683r.c
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
// MSI GT683R led driver
//
// Copyright (c) 2014 Janne Kanniainen <janne.kanniainen@gmail.com>
//

pub const GT683R_BUFFER_SIZE: c_int = 8;
//
// GT683R_LED_OFF: all LEDs are off
// GT683R_LED_AUDIO: LEDs brightness depends on sound level
// GT683R_LED_BREATHING: LEDs brightness varies at human breathing rate
// GT683R_LED_NORMAL: LEDs are fully on when enabled
//
    enum gt683r_led_mode {
    GT683R_LED_OFF = 0,
    GT683R_LED_AUDIO = 2,
    GT683R_LED_BREATHING = 3,
    GT683R_LED_NORMAL = 5
    };
    enum gt683r_panels {
    GT683R_LED_BACK = 0,
    GT683R_LED_SIDE = 1,
    GT683R_LED_FRONT = 2,
    GT683R_LED_COUNT,
    };
    static const char * const gt683r_panel_names[] = {
    "back",
    "side",
    "front",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gt683r_led {
    pub hdev: *mut hid_device,
    pub led_devs: [led_classdev; GT683R_LED_COUNT],
    pub lock: mutex,
    pub work: work_struct,
    pub brightnesses: [enum led_brightness; GT683R_LED_COUNT],
    pub mode: enum gt683r_led_mode,
}

    static const struct hid_device_id gt683r_led_id[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_MSI, USB_DEVICE_ID_MSI_GT683R_LED_PANEL) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, gt683r_led_id);
    static void gt683r_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    int i;
    struct device *dev = led_cdev.dev.parent;
    struct hid_device *hdev = to_hid_device(dev);
    struct gt683r_led *led = hid_get_drvdata(hdev);
    for (i = 0; i < GT683R_LED_COUNT; i++) {
    if (led_cdev == &led.led_devs[i])
    break;
    }
    if (i < GT683R_LED_COUNT) {
    led.brightnesses[i] = brightness;
    schedule_work(&led.work);
    }
    }
    static ssize_t mode_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u8 sysfs_mode;
    struct hid_device *hdev = to_hid_device(dev.parent);
    struct gt683r_led *led = hid_get_drvdata(hdev);
    if (led.mode == GT683R_LED_NORMAL)
    sysfs_mode = 0;
#[no_mangle]
pub unsafe extern "C" fn if(GT683R_LED_AUDIO: led->mode ==) -> else {
    else if (led.mode == GT683R_LED_AUDIO)
    sysfs_mode = 1;
    else
    sysfs_mode = 2;
    return scnprintf(buf, PAGE_SIZE, "%u\n", sysfs_mode);
    }
    static ssize_t mode_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u8 sysfs_mode;
    struct hid_device *hdev = to_hid_device(dev.parent);
    struct gt683r_led *led = hid_get_drvdata(hdev);
    if (kstrtou8(buf, 10, &sysfs_mode) || sysfs_mode > 2)
    return -EINVAL;
    mutex_lock(&led.lock);
    if (sysfs_mode == 0)
    led.mode = GT683R_LED_NORMAL;
#[no_mangle]
pub unsafe extern "C" fn if(1: sysfs_mode ==) -> else {
    else if (sysfs_mode == 1)
    led.mode = GT683R_LED_AUDIO;
    else
    led.mode = GT683R_LED_BREATHING;
    mutex_unlock(&led.lock);
    schedule_work(&led.work);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn gt683r_led_snd_msg(led: *mut gt683r_led, msg: *mut u8) -> c_int {
    static int gt683r_led_snd_msg(struct gt683r_led *led, u8 *msg)
    {
    int ret;
    ret = hid_hw_raw_request(led.hdev, msg[0], msg, GT683R_BUFFER_SIZE,
    HID_FEATURE_REPORT, HID_REQ_SET_REPORT);
    if (ret != GT683R_BUFFER_SIZE) {
    hid_err(led.hdev,
    "failed to send set report request: %i\n", ret);
    if (ret < 0)
    return ret;
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gt683r_leds_set(led: *mut gt683r_led, leds: u8) -> c_int {
    static int gt683r_leds_set(struct gt683r_led *led, u8 leds)
    {
    int ret;
    u8 *buffer;
    buffer = kzalloc(GT683R_BUFFER_SIZE, GFP_KERNEL);
    if (!buffer)
    return -ENOMEM;
    buffer[0] = 0x01;
    buffer[1] = 0x02;
    buffer[2] = 0x30;
    buffer[3] = leds;
    ret = gt683r_led_snd_msg(led, buffer);
    kfree(buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gt683r_mode_set(led: *mut gt683r_led, mode: u8) -> c_int {
    static int gt683r_mode_set(struct gt683r_led *led, u8 mode)
    {
    int ret;
    u8 *buffer;
    buffer = kzalloc(GT683R_BUFFER_SIZE, GFP_KERNEL);
    if (!buffer)
    return -ENOMEM;
    buffer[0] = 0x01;
    buffer[1] = 0x02;
    buffer[2] = 0x20;
    buffer[3] = mode;
    buffer[4] = 0x01;
    ret = gt683r_led_snd_msg(led, buffer);
    kfree(buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gt683r_led_work(work: *mut work_struct) {
    static void gt683r_led_work(struct work_struct *work)
    {
    int i;
    let mut leds: u8 = 0;
    u8 mode;
    struct gt683r_led *led = container_of(work, struct gt683r_led, work);
    mutex_lock(&led.lock);
    for (i = 0; i < GT683R_LED_COUNT; i++) {
    if (led.brightnesses[i])
    leds |= BIT(i);
    }
    if (gt683r_leds_set(led, leds))
    goto fail;
    if (leds)
    mode = led.mode;
    else
    mode = GT683R_LED_OFF;
    gt683r_mode_set(led, mode);
    fail:
    mutex_unlock(&led.lock);
    }
    static DEVICE_ATTR_RW(mode);
    static struct attribute *gt683r_led_attrs[] = {
    &dev_attr_mode.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group gt683r_led_group = {
    .name = "gt683r",
    .attrs = gt683r_led_attrs,
    };
    static const struct attribute_group *gt683r_led_groups[] = {
    &gt683r_led_group,
    core::ptr::null_mut()
    };
    static int gt683r_led_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    int i;
    int ret;
    int name_sz;
    char *name;
    struct gt683r_led *led;
    led = devm_kzalloc(&hdev.dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    mutex_init(&led.lock);
    INIT_WORK(&led.work, gt683r_led_work);
    led.mode = GT683R_LED_NORMAL;
    led.hdev = hdev;
    hid_set_drvdata(hdev, led);
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "hid parsing failed\n");
    return ret;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_HIDRAW);
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    return ret;
    }
    for (i = 0; i < GT683R_LED_COUNT; i++) {
    name_sz = strlen(dev_name(&hdev.dev)) +
    strlen(gt683r_panel_names[i]) + 3;
    name = devm_kzalloc(&hdev.dev, name_sz, GFP_KERNEL);
    if (!name) {
    ret = -ENOMEM;
    goto fail;
    }
    snprintf(name, name_sz, "%s::%s",
    dev_name(&hdev.dev), gt683r_panel_names[i]);
    led.led_devs[i].name = name;
    led.led_devs[i].max_brightness = 1;
    led.led_devs[i].brightness_set = gt683r_brightness_set;
    led.led_devs[i].groups = gt683r_led_groups;
    ret = led_classdev_register(&hdev.dev, &led.led_devs[i]);
    if (ret) {
    hid_err(hdev, "could not register led device\n");
    goto fail;
    }
    }
    return 0;
    fail:
    for (i = i - 1; i >= 0; i--)
    led_classdev_unregister(&led.led_devs[i]);
    hid_hw_stop(hdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gt683r_led_remove(hdev: *mut hid_device) {
    static void gt683r_led_remove(struct hid_device *hdev)
    {
    int i;
    struct gt683r_led *led = hid_get_drvdata(hdev);
    for (i = 0; i < GT683R_LED_COUNT; i++)
    led_classdev_unregister(&led.led_devs[i]);
    flush_work(&led.work);
    hid_hw_stop(hdev);
    }
    static struct hid_driver gt683r_led_driver = {
    .probe = gt683r_led_probe,
    .remove = gt683r_led_remove,
    .name = "gt683r_led",
    .id_table = gt683r_led_id,
    };
    module_hid_driver(gt683r_led_driver);
    MODULE_AUTHOR("Janne Kanniainen");
    MODULE_DESCRIPTION("MSI GT683R led driver");
    MODULE_LICENSE("GPL");
