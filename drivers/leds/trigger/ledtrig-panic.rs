//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-panic.c
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
// Kernel Panic LED Trigger
//
// Copyright 2016 Ezequiel Garcia <ezequiel@vanguardiasur.com.ar>
//

    static struct led_trigger *trigger;
//
// This is called in a special context by the atomic panic
// notifier. This means the trigger can be changed without
// worrying about locking.
//
#[no_mangle]
unsafe extern "C" fn led_trigger_set_panic(led_cdev: *mut led_classdev) {
    static void led_trigger_set_panic(struct led_classdev *led_cdev)
    {
    if (led_cdev.trigger)
    list_del(&led_cdev.trig_list);
    list_add_tail(&led_cdev.trig_list, &trigger.led_cdevs);
// Avoid the delayed blink path
    led_cdev.blink_delay_on = 0;
    led_cdev.blink_delay_off = 0;
    led_cdev.trigger = trigger;
    }
    static int led_trigger_panic_notifier(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    struct led_classdev *led_cdev;
    list_for_each_entry(led_cdev, &leds_list, node)
    if (led_cdev.flags & LED_PANIC_INDICATOR)
    led_trigger_set_panic(led_cdev);
    return NOTIFY_DONE;
    }
    static struct notifier_block led_trigger_panic_nb = {
    .notifier_call = led_trigger_panic_notifier,
    };
#[no_mangle]
unsafe extern "C" fn led_panic_blink(state: c_int) -> c_long {
    static long led_panic_blink(int state)
    {
    led_trigger_event(trigger, state ? LED_FULL : LED_OFF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ledtrig_panic_init() -> int __init {
    static int __init ledtrig_panic_init(void)
    {
    led_trigger_register_simple("panic", &trigger);
    if (!trigger)
    return -ENOMEM;
    atomic_notifier_chain_register(&panic_notifier_list,
    &led_trigger_panic_nb);
    panic_blink = led_panic_blink;
    return 0;
    }
    device_initcall(ledtrig_panic_init);
