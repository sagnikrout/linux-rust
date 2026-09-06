//! Automatically rewritten from C to Rust
//! Source: drivers/input/apm-power.c
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
// Input Power Event -> APM Bridge
//
// Copyright (c) 2007 Richard Purdie
//

#[no_mangle]
unsafe extern "C" fn system_power_event(keycode: c_uint) {
    static void system_power_event(unsigned int keycode)
    {
    switch (keycode) {
    case KEY_SUSPEND:
    apm_queue_event(APM_USER_SUSPEND);
    pr_info("Requesting system suspend...\n");
    break;
    default:
    break;
    }
    }
    static void apmpower_event(struct input_handle *handle, unsigned int type,
    unsigned int code, int value)
    {
// only react on key down events
    if (value != 1)
    return;
    switch (type) {
    case EV_PWR:
    system_power_event(code);
    break;
    default:
    break;
    }
    }
    static int apmpower_connect(struct input_handler *handler,
    struct input_dev *dev,
    const struct input_device_id *id)
    {
    struct input_handle *handle;
    int error;
    handle = kzalloc_obj(struct input_handle);
    if (!handle)
    return -ENOMEM;
    handle.dev = dev;
    handle.handler = handler;
    handle.name = "apm-power";
    error = input_register_handle(handle);
    if (error) {
    pr_err("Failed to register input power handler, error %d\n",
    error);
    kfree(handle);
    return error;
    }
    error = input_open_device(handle);
    if (error) {
    pr_err("Failed to open input power device, error %d\n", error);
    input_unregister_handle(handle);
    kfree(handle);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apmpower_disconnect(handle: *mut input_handle) {
    static void apmpower_disconnect(struct input_handle *handle)
    {
    input_close_device(handle);
    input_unregister_handle(handle);
    kfree(handle);
    }
    static const struct input_device_id apmpower_ids[] = {
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT,
    .evbit = { BIT_MASK(EV_PWR) },
    },
    { },
    };
    MODULE_DEVICE_TABLE(input, apmpower_ids);
    static struct input_handler apmpower_handler = {
    .event =	apmpower_event,
    .connect =	apmpower_connect,
    .disconnect =	apmpower_disconnect,
    .name =		"apm-power",
    .id_table =	apmpower_ids,
    };
#[no_mangle]
unsafe extern "C" fn apmpower_init() -> int __init {
    static int __init apmpower_init(void)
    {
    return input_register_handler(&apmpower_handler);
    }
#[no_mangle]
unsafe extern "C" fn apmpower_exit() -> void __exit {
    static void __exit apmpower_exit(void)
    {
    input_unregister_handler(&apmpower_handler);
    }
    module_init(apmpower_init);
    module_exit(apmpower_exit);
    MODULE_AUTHOR("Richard Purdie <rpurdie@rpsys.net>");
    MODULE_DESCRIPTION("Input Power Event . APM Bridge");
    MODULE_LICENSE("GPL");
