//! Automatically rewritten from C to Rust
//! Source: drivers/leds/led-class-flash.c
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
// LED Flash class interface
//
// Copyright (C) 2015 Samsung Electronics Co., Ltd.
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//

    (fled_cdev && fled_cdev.ops.op)

    ((has_flash_op(fled_cdev, op)) ?			\
    (fled_cdev.ops.op(fled_cdev, args)) :	\
    -EINVAL)
    static const char * const led_flash_fault_names[] = {
    "led-over-voltage",
    "flash-timeout-exceeded",
    "controller-over-temperature",
    "controller-short-circuit",
    "led-power-supply-over-current",
    "indicator-led-fault",
    "led-under-voltage",
    "controller-under-voltage",
    "led-over-temperature",
    };
    static ssize_t flash_brightness_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    unsigned long state;
    ssize_t ret;
    mutex_lock(&led_cdev.led_access);
    if (led_sysfs_is_disabled(led_cdev)) {
    ret = -EBUSY;
    goto unlock;
    }
    ret = kstrtoul(buf, 10, &state);
    if (ret)
    goto unlock;
    ret = led_set_flash_brightness(fled_cdev, state);
    if (ret < 0)
    goto unlock;
    ret = size;
    unlock:
    mutex_unlock(&led_cdev.led_access);
    return ret;
    }
    static ssize_t flash_brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
// no lock needed for this
    led_update_flash_brightness(fled_cdev);
    return sprintf(buf, "%u\n", fled_cdev.brightness.val);
    }
    static DEVICE_ATTR_RW(flash_brightness);
    static ssize_t max_flash_brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    return sprintf(buf, "%u\n", fled_cdev.brightness.max);
    }
    static DEVICE_ATTR_RO(max_flash_brightness);
    static ssize_t flash_strobe_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    unsigned long state;
    let mut ret: isize = -EBUSY;
    mutex_lock(&led_cdev.led_access);
    if (led_sysfs_is_disabled(led_cdev))
    goto unlock;
    ret = kstrtoul(buf, 10, &state);
    if (ret)
    goto unlock;
    if (state > 1) {
    ret = -EINVAL;
    goto unlock;
    }
    ret = led_set_flash_strobe(fled_cdev, state);
    if (ret < 0)
    goto unlock;
    ret = size;
    unlock:
    mutex_unlock(&led_cdev.led_access);
    return ret;
    }
    static ssize_t flash_strobe_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    bool state;
    int ret;
// no lock needed for this
    ret = led_get_flash_strobe(fled_cdev, &state);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%u\n", state);
    }
    static DEVICE_ATTR_RW(flash_strobe);
    static ssize_t flash_timeout_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    unsigned long flash_timeout;
    ssize_t ret;
    mutex_lock(&led_cdev.led_access);
    if (led_sysfs_is_disabled(led_cdev)) {
    ret = -EBUSY;
    goto unlock;
    }
    ret = kstrtoul(buf, 10, &flash_timeout);
    if (ret)
    goto unlock;
    ret = led_set_flash_timeout(fled_cdev, flash_timeout);
    if (ret < 0)
    goto unlock;
    ret = size;
    unlock:
    mutex_unlock(&led_cdev.led_access);
    return ret;
    }
    static ssize_t flash_timeout_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    return sprintf(buf, "%u\n", fled_cdev.timeout.val);
    }
    static DEVICE_ATTR_RW(flash_timeout);
    static ssize_t max_flash_timeout_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    return sprintf(buf, "%u\n", fled_cdev.timeout.max);
    }
    static DEVICE_ATTR_RO(max_flash_timeout);
    static ssize_t flash_fault_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    u32 fault, mask = 0x1;
    char *pbuf = buf;
    int i, ret, buf_len;
    ret = led_get_flash_fault(fled_cdev, &fault);
    if (ret < 0)
    return -EINVAL;
// buf = '\0';
    for (i = 0; i < LED_NUM_FLASH_FAULTS; ++i) {
    if (fault & mask) {
    buf_len = sprintf(pbuf, "%s ",
    led_flash_fault_names[i]);
    pbuf += buf_len;
    }
    mask <<= 1;
    }
    return strlen(strcat(buf, "\n"));
    }
    static DEVICE_ATTR_RO(flash_fault);
    static struct attribute *led_flash_strobe_attrs[] = {
    &dev_attr_flash_strobe.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *led_flash_timeout_attrs[] = {
    &dev_attr_flash_timeout.attr,
    &dev_attr_max_flash_timeout.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *led_flash_brightness_attrs[] = {
    &dev_attr_flash_brightness.attr,
    &dev_attr_max_flash_brightness.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *led_flash_fault_attrs[] = {
    &dev_attr_flash_fault.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group led_flash_strobe_group = {
    .attrs = led_flash_strobe_attrs,
    };
    static const struct attribute_group led_flash_timeout_group = {
    .attrs = led_flash_timeout_attrs,
    };
    static const struct attribute_group led_flash_brightness_group = {
    .attrs = led_flash_brightness_attrs,
    };
    static const struct attribute_group led_flash_fault_group = {
    .attrs = led_flash_fault_attrs,
    };
#[no_mangle]
unsafe extern "C" fn led_flash_resume(led_cdev: *mut led_classdev) {
    static void led_flash_resume(struct led_classdev *led_cdev)
    {
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    call_flash_op(fled_cdev, flash_brightness_set,
    fled_cdev.brightness.val);
    call_flash_op(fled_cdev, timeout_set, fled_cdev.timeout.val);
    }
#[no_mangle]
unsafe extern "C" fn led_flash_init_sysfs_groups(fled_cdev: *mut led_classdev_flash) {
    static void led_flash_init_sysfs_groups(struct led_classdev_flash *fled_cdev)
    {
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    const struct led_flash_ops *ops = fled_cdev.ops;
    const struct attribute_group **flash_groups = fled_cdev.sysfs_groups;
    let mut num_sysfs_groups: c_int = 0;
    flash_groups[num_sysfs_groups++] = &led_flash_strobe_group;
    if (ops.flash_brightness_set)
    flash_groups[num_sysfs_groups++] = &led_flash_brightness_group;
    if (ops.timeout_set)
    flash_groups[num_sysfs_groups++] = &led_flash_timeout_group;
    if (ops.fault_get)
    flash_groups[num_sysfs_groups++] = &led_flash_fault_group;
    led_cdev.groups = flash_groups;
    }
    int led_classdev_flash_register_ext(struct device *parent,
    struct led_classdev_flash *fled_cdev,
    struct led_init_data *init_data)
    {
    struct led_classdev *led_cdev;
    const struct led_flash_ops *ops;
    int ret;
    if (!fled_cdev)
    return -EINVAL;
    led_cdev = &fled_cdev.led_cdev;
    if (led_cdev.flags & LED_DEV_CAP_FLASH) {
    if (!led_cdev.brightness_set_blocking)
    return -EINVAL;
    ops = fled_cdev.ops;
    if (!ops || !ops.strobe_set)
    return -EINVAL;
    led_cdev.flash_resume = led_flash_resume;
// Select the sysfs attributes to be created for the device
    led_flash_init_sysfs_groups(fled_cdev);
    }
// Register led class device
    ret = led_classdev_register_ext(parent, led_cdev, init_data);
    if (ret < 0)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL_GPL(led_classdev_flash_register_ext);
#[no_mangle]
pub unsafe extern "C" fn led_classdev_flash_unregister(fled_cdev: *mut led_classdev_flash) {
    void led_classdev_flash_unregister(struct led_classdev_flash *fled_cdev)
    {
    if (!fled_cdev)
    return;
    led_classdev_unregister(&fled_cdev.led_cdev);
    }
    EXPORT_SYMBOL_GPL(led_classdev_flash_unregister);
#[no_mangle]
unsafe extern "C" fn devm_led_classdev_flash_release(dev: *mut device, res: *mut c_void) {
    static void devm_led_classdev_flash_release(struct device *dev, void *res)
    {
    led_classdev_flash_unregister(*(struct led_classdev_flash **)res);
    }
    int devm_led_classdev_flash_register_ext(struct device *parent,
    struct led_classdev_flash *fled_cdev,
    struct led_init_data *init_data)
    {
    struct led_classdev_flash **dr;
    int ret;
    dr = devres_alloc(devm_led_classdev_flash_release, sizeof(*dr),
    GFP_KERNEL);
    if (!dr)
    return -ENOMEM;
    ret = led_classdev_flash_register_ext(parent, fled_cdev, init_data);
    if (ret) {
    devres_free(dr);
    return ret;
    }
// dr = fled_cdev;
    devres_add(parent, dr);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_led_classdev_flash_register_ext);
    static int devm_led_classdev_flash_match(struct device *dev,
    void *res, void *data)
    {
    struct led_classdev_flash **p = res;
    if (WARN_ON(!p || !*p))
    return 0;
    return *p == data;
    }
    void devm_led_classdev_flash_unregister(struct device *dev,
    struct led_classdev_flash *fled_cdev)
    {
    WARN_ON(devres_release(dev,
    devm_led_classdev_flash_release,
    devm_led_classdev_flash_match, fled_cdev));
    }
    EXPORT_SYMBOL_GPL(devm_led_classdev_flash_unregister);
#[no_mangle]
unsafe extern "C" fn led_clamp_align(s: *mut led_flash_setting) {
    static void led_clamp_align(struct led_flash_setting *s)
    {
    u32 v, offset;
    v = s.val + s.step / 2;
    v = clamp(v, s.min, s.max);
    offset = v - s.min;
    offset = s.step * (offset / s.step);
    s.val = s.min + offset;
    }
#[no_mangle]
pub unsafe extern "C" fn led_set_flash_timeout(fled_cdev: *mut led_classdev_flash, timeout: u32) -> c_int {
    int led_set_flash_timeout(struct led_classdev_flash *fled_cdev, u32 timeout)
    {
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    struct led_flash_setting *s = &fled_cdev.timeout;
    s.val = timeout;
    led_clamp_align(s);
    if (!(led_cdev.flags & LED_SUSPENDED))
    return call_flash_op(fled_cdev, timeout_set, s.val);
    return 0;
    }
    EXPORT_SYMBOL_GPL(led_set_flash_timeout);
#[no_mangle]
pub unsafe extern "C" fn led_get_flash_fault(fled_cdev: *mut led_classdev_flash, fault: *mut u32) -> c_int {
    int led_get_flash_fault(struct led_classdev_flash *fled_cdev, u32 *fault)
    {
    return call_flash_op(fled_cdev, fault_get, fault);
    }
    EXPORT_SYMBOL_GPL(led_get_flash_fault);
    int led_set_flash_brightness(struct led_classdev_flash *fled_cdev,
    u32 brightness)
    {
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    struct led_flash_setting *s = &fled_cdev.brightness;
    s.val = brightness;
    led_clamp_align(s);
    if (!(led_cdev.flags & LED_SUSPENDED))
    return call_flash_op(fled_cdev, flash_brightness_set, s.val);
    return 0;
    }
    EXPORT_SYMBOL_GPL(led_set_flash_brightness);
#[no_mangle]
pub unsafe extern "C" fn led_update_flash_brightness(fled_cdev: *mut led_classdev_flash) -> c_int {
    int led_update_flash_brightness(struct led_classdev_flash *fled_cdev)
    {
    struct led_flash_setting *s = &fled_cdev.brightness;
    u32 brightness;
    if (has_flash_op(fled_cdev, flash_brightness_get)) {
    int ret = call_flash_op(fled_cdev, flash_brightness_get,
    &brightness);
    if (ret < 0)
    return ret;
    s.val = brightness;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(led_update_flash_brightness);
#[no_mangle]
pub unsafe extern "C" fn led_set_flash_duration(fled_cdev: *mut led_classdev_flash, duration: u32) -> c_int {
    int led_set_flash_duration(struct led_classdev_flash *fled_cdev, u32 duration)
    {
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    struct led_flash_setting *s = &fled_cdev.duration;
    s.val = duration;
    led_clamp_align(s);
    if (!(led_cdev.flags & LED_SUSPENDED))
    return call_flash_op(fled_cdev, duration_set, s.val);
    return 0;
    }
    EXPORT_SYMBOL_GPL(led_set_flash_duration);
    MODULE_AUTHOR("Jacek Anaszewski <j.anaszewski@samsung.com>");
    MODULE_DESCRIPTION("LED Flash class interface");
    MODULE_LICENSE("GPL v2");
