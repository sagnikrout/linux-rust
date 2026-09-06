//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-input-events.c
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
// Input Events LED trigger
//
// Copyright (C) 2024 Hans de Goede <hansg@kernel.org>
//

    let mut led_off_delay_ms: static unsigned long = 5000;
    module_param(led_off_delay_ms, ulong, 0644);
    MODULE_PARM_DESC(led_off_delay_ms,
    "Specify delay in ms for turning LEDs off after last input event");
    static struct input_events_data {
    struct delayed_work work;
    spinlock_t lock;
// To avoid repeatedly setting the brightness while there are events
    bool led_on;
    unsigned long led_off_time;
    } input_events_data;
    static struct led_trigger *input_events_led_trigger;
#[no_mangle]
unsafe extern "C" fn led_input_events_work(work: *mut work_struct) {
    static void led_input_events_work(struct work_struct *work)
    {
    struct input_events_data *data =
    container_of(work, struct input_events_data, work.work);
    spin_lock_irq(&data.lock);
//
// This time_after_eq() check avoids a race where this work starts
// running before a new event pushed led_off_time back.
//
    if (time_after_eq(jiffies, data.led_off_time)) {
    led_trigger_event(input_events_led_trigger, LED_OFF);
    data.led_on = false;
    }
    spin_unlock_irq(&data.lock);
    }
    static void input_events_event(struct input_handle *handle, unsigned int type,
    unsigned int code, int val)
    {
    struct input_events_data *data = &input_events_data;
    let mut led_off_delay: c_ulong = msecs_to_jiffies(led_off_delay_ms);
    unsigned long flags;
    spin_lock_irqsave(&data.lock, flags);
    if (!data.led_on) {
    led_trigger_event(input_events_led_trigger, LED_FULL);
    data.led_on = true;
    }
    data.led_off_time = jiffies + led_off_delay;
    spin_unlock_irqrestore(&data.lock, flags);
    mod_delayed_work(system_percpu_wq, &data.work, led_off_delay);
    }
    static int input_events_connect(struct input_handler *handler, struct input_dev *dev,
    const struct input_device_id *id)
    {
    struct input_handle *handle;
    int ret;
    handle = kzalloc_obj(*handle);
    if (!handle)
    return -ENOMEM;
    handle.dev = dev;
    handle.handler = handler;
    handle.name = KBUILD_MODNAME;
    ret = input_register_handle(handle);
    if (ret)
    goto err_free_handle;
    ret = input_open_device(handle);
    if (ret)
    goto err_unregister_handle;
    return 0;
    err_unregister_handle:
    input_unregister_handle(handle);
    err_free_handle:
    kfree(handle);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn input_events_disconnect(handle: *mut input_handle) {
    static void input_events_disconnect(struct input_handle *handle)
    {
    input_close_device(handle);
    input_unregister_handle(handle);
    kfree(handle);
    }
    static const struct input_device_id input_events_ids[] = {
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT,
    .evbit = { BIT_MASK(EV_KEY) },
    },
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT,
    .evbit = { BIT_MASK(EV_REL) },
    },
    {
    .flags = INPUT_DEVICE_ID_MATCH_EVBIT,
    .evbit = { BIT_MASK(EV_ABS) },
    },
    { }
    };
    static struct input_handler input_events_handler = {
    .name = KBUILD_MODNAME,
    .event = input_events_event,
    .connect = input_events_connect,
    .disconnect = input_events_disconnect,
    .id_table = input_events_ids,
    };
#[no_mangle]
unsafe extern "C" fn input_events_init() -> int __init {
    static int __init input_events_init(void)
    {
    int ret;
    INIT_DELAYED_WORK(&input_events_data.work, led_input_events_work);
    spin_lock_init(&input_events_data.lock);
    led_trigger_register_simple("input-events", &input_events_led_trigger);
    ret = input_register_handler(&input_events_handler);
    if (ret) {
    led_trigger_unregister_simple(input_events_led_trigger);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn input_events_exit() -> void __exit {
    static void __exit input_events_exit(void)
    {
    input_unregister_handler(&input_events_handler);
    cancel_delayed_work_sync(&input_events_data.work);
    led_trigger_unregister_simple(input_events_led_trigger);
    }
    module_init(input_events_init);
    module_exit(input_events_exit);
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_DESCRIPTION("Input Events LED trigger");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("ledtrig:input-events");
