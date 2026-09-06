//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/cros_kbd_led_backlight.c
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


// SPDX-License-Identifier: GPL-2.0+
// Keyboard backlight LED driver for ChromeOS
//
// Copyright (C) 2012 Google, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyboard_led {
    pub cdev: led_classdev,
    pub ec: *mut cros_ec_device,
}

//
// struct keyboard_led_drvdata - keyboard LED driver data.
// @init:			Init function.
// @brightness_get:		Get LED brightness level.
// @brightness_set:		Set LED brightness level.  Must not sleep.
// @brightness_set_blocking:	Set LED brightness level.  It can block the
// caller for the time required for accessing a
// LED device register
//
// See struct led_classdev in include/linux/leds.h for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyboard_led_drvdata {
    pub keyboard_led): *mut *mut *mut int (init)(struct platform_device pdev, struct keyboard_led,
    pub led_cdev): *mut *mut enum led_brightness (brightness_get)(struct led_classdev,
    void (*brightness_set)(struct led_classdev *led_cdev,
    pub brightness): enum led_brightness,
    int (*brightness_set_blocking)(struct led_classdev *led_cdev,
    pub brightness): enum led_brightness,
}

// Keyboard LED ACPI Device must be defined in firmware

    static void keyboard_led_set_brightness_acpi(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    union acpi_object param;
    struct acpi_object_list input;
    acpi_status status;
    param.type = ACPI_TYPE_INTEGER;
    param.integer.value = brightness;
    input.count = 1;
    input.pointer = &param;
    status = acpi_evaluate_object(core::ptr::null_mut(), ACPI_KEYBOARD_BACKLIGHT_WRITE,
    &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    dev_err(cdev.dev, "Error setting keyboard LED value: %d\n",
    status);
    }
    static enum led_brightness
    keyboard_led_get_brightness_acpi(struct led_classdev *cdev)
    {
    unsigned long long brightness;
    acpi_status status;
    status = acpi_evaluate_integer(core::ptr::null_mut(), ACPI_KEYBOARD_BACKLIGHT_READ,
    core::ptr::null_mut(), &brightness);
    if (ACPI_FAILURE(status)) {
    dev_err(cdev.dev, "Error getting keyboard LED value: %d\n",
    status);
    return -EIO;
    }
    return brightness;
    }
    static int keyboard_led_init_acpi(struct platform_device *pdev,
    struct keyboard_led *keyboard_led)
    {
    acpi_handle handle;
    acpi_status status;
// Look for the keyboard LED ACPI Device
    status = acpi_get_handle(ACPI_ROOT_OBJECT,
    ACPI_KEYBOARD_BACKLIGHT_DEVICE,
    &handle);
    if (ACPI_FAILURE(status)) {
    dev_err(&pdev.dev, "Unable to find ACPI device %s: %d\n",
    ACPI_KEYBOARD_BACKLIGHT_DEVICE, status);
    return -ENXIO;
    }
    return 0;
    }
    static const struct keyboard_led_drvdata keyboard_led_drvdata_acpi = {
    .init = keyboard_led_init_acpi,
    .brightness_set = keyboard_led_set_brightness_acpi,
    .brightness_get = keyboard_led_get_brightness_acpi,
    };

    static int keyboard_led_init_ec_pwm_mfd(struct platform_device *pdev,
    struct keyboard_led *keyboard_led)
    {
    struct cros_ec_dev *ec_dev = dev_get_drvdata(pdev.dev.parent);
    struct cros_ec_device *cros_ec = ec_dev.ec_dev;
    keyboard_led.ec = cros_ec;
    return 0;
    }
    static int
    keyboard_led_set_brightness_ec_pwm(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    DEFINE_RAW_FLEX(struct cros_ec_command, msg, data,
    sizeof(struct ec_params_pwm_set_keyboard_backlight));
    struct ec_params_pwm_set_keyboard_backlight *params =
    (struct ec_params_pwm_set_keyboard_backlight *)msg.data;
    struct keyboard_led *keyboard_led = container_of(cdev, struct keyboard_led, cdev);
    msg.command = EC_CMD_PWM_SET_KEYBOARD_BACKLIGHT;
    msg.outsize = sizeof(*params);
    params.percent = brightness;
    return cros_ec_cmd_xfer_status(keyboard_led.ec, msg);
    }
    static enum led_brightness
    keyboard_led_get_brightness_ec_pwm(struct led_classdev *cdev)
    {
    DEFINE_RAW_FLEX(struct cros_ec_command, msg, data,
    sizeof(struct ec_response_pwm_get_keyboard_backlight));
    struct ec_response_pwm_get_keyboard_backlight *resp =
    (struct ec_response_pwm_get_keyboard_backlight *)msg.data;
    struct keyboard_led *keyboard_led = container_of(cdev, struct keyboard_led, cdev);
    int ret;
    msg.command = EC_CMD_PWM_GET_KEYBOARD_BACKLIGHT;
    msg.insize = sizeof(*resp);
    ret = cros_ec_cmd_xfer_status(keyboard_led.ec, msg);
    if (ret < 0)
    return ret;
    return resp.percent;
    }
    static const struct keyboard_led_drvdata keyboard_led_drvdata_ec_pwm_mfd = {
    .init = keyboard_led_init_ec_pwm_mfd,
    .brightness_set_blocking = keyboard_led_set_brightness_ec_pwm,
    .brightness_get = keyboard_led_get_brightness_ec_pwm,
    };
#[no_mangle]
unsafe extern "C" fn keyboard_led_is_mfd_device(pdev: *mut platform_device) -> c_int {
    static int keyboard_led_is_mfd_device(struct platform_device *pdev)
    {
    return IS_ENABLED(CONFIG_MFD_CROS_EC_DEV) && mfd_get_cell(pdev);
    }
#[no_mangle]
unsafe extern "C" fn keyboard_led_probe(pdev: *mut platform_device) -> c_int {
    static int keyboard_led_probe(struct platform_device *pdev)
    {
    const struct keyboard_led_drvdata *drvdata;
    struct keyboard_led *keyboard_led;
    int err;
    if (keyboard_led_is_mfd_device(pdev))
    drvdata = &keyboard_led_drvdata_ec_pwm_mfd;
    else
    drvdata = device_get_match_data(&pdev.dev);
    if (!drvdata)
    return -EINVAL;
    keyboard_led = devm_kzalloc(&pdev.dev, sizeof(*keyboard_led), GFP_KERNEL);
    if (!keyboard_led)
    return -ENOMEM;
    if (drvdata.init) {
    err = drvdata.init(pdev, keyboard_led);
    if (err)
    return err;
    }
    keyboard_led.cdev.name = "chromeos::kbd_backlight";
    keyboard_led.cdev.flags |= LED_CORE_SUSPENDRESUME | LED_REJECT_NAME_CONFLICT;
    keyboard_led.cdev.max_brightness = 100;
    keyboard_led.cdev.brightness_set = drvdata.brightness_set;
    keyboard_led.cdev.brightness_set_blocking = drvdata.brightness_set_blocking;
    keyboard_led.cdev.brightness_get = drvdata.brightness_get;
    err = devm_led_classdev_register(&pdev.dev, &keyboard_led.cdev);
    if (err == -EEXIST) /* Already bound via other mechanism */
    return -ENODEV;
    return err;
    }

    static const struct acpi_device_id keyboard_led_acpi_match[] = {
    { "GOOG0002", (kernel_ulong_t)&keyboard_led_drvdata_acpi },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, keyboard_led_acpi_match);

    static const struct platform_device_id keyboard_led_id[] = {
    { .name = "cros-keyboard-leds" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, keyboard_led_id);
    static struct platform_driver keyboard_led_driver = {
    .driver		= {
    .name	= "cros-keyboard-leds",
    .acpi_match_table = ACPI_PTR(keyboard_led_acpi_match),
    },
    .probe		= keyboard_led_probe,
    .id_table	= keyboard_led_id,
    };
    module_platform_driver(keyboard_led_driver);
    MODULE_AUTHOR("Simon Que <sque@chromium.org>");
    MODULE_DESCRIPTION("ChromeOS Keyboard backlight LED Driver");
    MODULE_LICENSE("GPL");
