//! Automatically rewritten from C to Rust
//! Source: drivers/usb/common/led.c
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


// SPDX-License-Identifier: GPL-2.0
//
// LED Triggers for USB Activity
//
// Copyright 2014 Michal Sojka <sojka@merica.cz>
//

pub const BLINK_DELAY: c_int = 30;
    DEFINE_LED_TRIGGER(ledtrig_usb_gadget);
    DEFINE_LED_TRIGGER(ledtrig_usb_host);
#[no_mangle]
pub unsafe extern "C" fn usb_led_activity(ev: enum usb_led_event) {
    void usb_led_activity(enum usb_led_event ev)
    {
    struct led_trigger *trig = core::ptr::null_mut();
    switch (ev) {
    case USB_LED_EVENT_GADGET:
    trig = ledtrig_usb_gadget;
    break;
    case USB_LED_EVENT_HOST:
    trig = ledtrig_usb_host;
    break;
    }
// led_trigger_blink_oneshot() handles trig == NULL gracefully
    led_trigger_blink_oneshot(trig, BLINK_DELAY, BLINK_DELAY, 0);
    }
    EXPORT_SYMBOL_GPL(usb_led_activity);
#[no_mangle]
pub unsafe extern "C" fn ledtrig_usb_init() -> void __init {
    void __init ledtrig_usb_init(void)
    {
    led_trigger_register_simple("usb-gadget", &ledtrig_usb_gadget);
    led_trigger_register_simple("usb-host", &ledtrig_usb_host);
    }
#[no_mangle]
pub unsafe extern "C" fn ledtrig_usb_exit() -> void __exit {
    void __exit ledtrig_usb_exit(void)
    {
    led_trigger_unregister_simple(ledtrig_usb_gadget);
    led_trigger_unregister_simple(ledtrig_usb_host);
    }
