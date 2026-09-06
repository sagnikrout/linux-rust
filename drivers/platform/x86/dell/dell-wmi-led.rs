//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-led.c
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


//
// Copyright (C) 2010 Dell Inc.
// Louis Davis <louis_davis@dell.com>
// Jim Dailey <jim_dailey@dell.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as
// published by the Free Software Foundation.
//

    MODULE_AUTHOR("Louis Davis/Jim Dailey");
    MODULE_DESCRIPTION("Dell LED Control Driver");
    MODULE_LICENSE("GPL");

    MODULE_ALIAS("wmi:" DELL_LED_BIOS_GUID);
// Error Result Codes:
pub const INVALID_DEVICE_ID: c_int = 250;
pub const INVALID_PARAMETER: c_int = 251;
pub const INVALID_BUFFER: c_int = 252;
pub const INTERFACE_ERROR: c_int = 253;
pub const UNSUPPORTED_COMMAND: c_int = 254;
pub const UNSPECIFIED_ERROR: c_int = 255;
// Device ID
pub const DEVICE_ID_PANEL_BACK: c_int = 1;
// LED Commands
pub const CMD_LED_ON: c_int = 16;
pub const CMD_LED_OFF: c_int = 17;
pub const CMD_LED_BLINK: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_args {
    pub length: c_uchar,
    pub result_code: c_uchar,
    pub device_id: c_uchar,
    pub command: c_uchar,
    pub on_time: c_uchar,
    pub off_time: c_uchar,
}

    static int dell_led_perform_fn(u8 length, u8 result_code, u8 device_id,
    u8 command, u8 on_time, u8 off_time)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    struct bios_args *bios_return;
    struct acpi_buffer input;
    union acpi_object *obj;
    acpi_status status;
    u8 return_code;
    struct bios_args args = {
    .length = length,
    .result_code = result_code,
    .device_id = device_id,
    .command = command,
    .on_time = on_time,
    .off_time = off_time
    };
    input.length = sizeof(struct bios_args);
    input.pointer = &args;
    status = wmi_evaluate_method(DELL_LED_BIOS_GUID, 0, 1, &input, &output);
    if (ACPI_FAILURE(status))
    return status;
    obj = output.pointer;
    if (!obj)
    return -EINVAL;
    if (obj.type != ACPI_TYPE_BUFFER) {
    kfree(obj);
    return -EINVAL;
    }
    bios_return = ((struct bios_args *)obj.buffer.pointer);
    return_code = bios_return.result_code;
    kfree(obj);
    return return_code;
    }
#[no_mangle]
unsafe extern "C" fn led_on() -> c_int {
    static int led_on(void)
    {
    return dell_led_perform_fn(3,	/* Length of command */
    INTERFACE_ERROR,	/* Init to  INTERFACE_ERROR */
    DEVICE_ID_PANEL_BACK,	/* Device ID */
    CMD_LED_ON,		/* Command */
    0,			/* not used */
    0);			/* not used */
    }
#[no_mangle]
unsafe extern "C" fn led_off() -> c_int {
    static int led_off(void)
    {
    return dell_led_perform_fn(3,	/* Length of command */
    INTERFACE_ERROR,	/* Init to  INTERFACE_ERROR */
    DEVICE_ID_PANEL_BACK,	/* Device ID */
    CMD_LED_OFF,		/* Command */
    0,			/* not used */
    0);			/* not used */
    }
#[no_mangle]
unsafe extern "C" fn led_blink(on_eighths: c_uchar, off_eighths: c_uchar) -> c_int {
    static int led_blink(unsigned char on_eighths, unsigned char off_eighths)
    {
    return dell_led_perform_fn(5,	/* Length of command */
    INTERFACE_ERROR,	/* Init to  INTERFACE_ERROR */
    DEVICE_ID_PANEL_BACK,	/* Device ID */
    CMD_LED_BLINK,		/* Command */
    on_eighths,		/* blink on in eigths of a second */
    off_eighths);		/* blink off in eights of a second */
    }
    static void dell_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    if (value == LED_OFF)
    led_off();
    else
    led_on();
    }
    static int dell_led_blink(struct led_classdev *led_cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    unsigned long on_eighths;
    unsigned long off_eighths;
//
// The Dell LED delay is based on 125ms intervals.
// Need to round up to next interval.
//
    on_eighths = DIV_ROUND_UP(*delay_on, 125);
    on_eighths = clamp_t(unsigned long, on_eighths, 1, 255);
// delay_on = on_eighths * 125;
    off_eighths = DIV_ROUND_UP(*delay_off, 125);
    off_eighths = clamp_t(unsigned long, off_eighths, 1, 255);
// delay_off = off_eighths * 125;
    led_blink(on_eighths, off_eighths);
    return 0;
    }
    static struct led_classdev dell_led = {
    .name		= "dell::lid",
    .brightness	= LED_OFF,
    .max_brightness = 1,
    .brightness_set = dell_led_set,
    .blink_set	= dell_led_blink,
    .flags		= LED_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn dell_led_init() -> int __init {
    static int __init dell_led_init(void)
    {
    let mut error: c_int = 0;
    if (!wmi_has_guid(DELL_LED_BIOS_GUID))
    return -ENODEV;
    error = led_off();
    if (error != 0)
    return -ENODEV;
    return led_classdev_register(core::ptr::null_mut(), &dell_led);
    }
#[no_mangle]
unsafe extern "C" fn dell_led_exit() -> void __exit {
    static void __exit dell_led_exit(void)
    {
    led_classdev_unregister(&dell_led);
    led_off();
    }
    module_init(dell_led_init);
    module_exit(dell_led_exit);
