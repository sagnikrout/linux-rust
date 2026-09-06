//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-camera.c
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
// Camera Flash and Torch On/Off Trigger
//
// based on ledtrig-ide-disk.c
//
// Copyright 2013 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

    DEFINE_LED_TRIGGER(ledtrig_flash);
    DEFINE_LED_TRIGGER(ledtrig_torch);
#[no_mangle]
pub unsafe extern "C" fn ledtrig_flash_ctrl(on: bool) {
    void ledtrig_flash_ctrl(bool on)
    {
    let mut brt: enum led_brightness = on ? LED_FULL : LED_OFF;
    led_trigger_event(ledtrig_flash, brt);
    }
    EXPORT_SYMBOL_GPL(ledtrig_flash_ctrl);
#[no_mangle]
pub unsafe extern "C" fn ledtrig_torch_ctrl(on: bool) {
    void ledtrig_torch_ctrl(bool on)
    {
    let mut brt: enum led_brightness = on ? LED_FULL : LED_OFF;
    led_trigger_event(ledtrig_torch, brt);
    }
    EXPORT_SYMBOL_GPL(ledtrig_torch_ctrl);
#[no_mangle]
unsafe extern "C" fn ledtrig_camera_init() -> int __init {
    static int __init ledtrig_camera_init(void)
    {
    led_trigger_register_simple("flash", &ledtrig_flash);
    led_trigger_register_simple("torch", &ledtrig_torch);
    return 0;
    }
    module_init(ledtrig_camera_init);
#[no_mangle]
unsafe extern "C" fn ledtrig_camera_exit() -> void __exit {
    static void __exit ledtrig_camera_exit(void)
    {
    led_trigger_unregister_simple(ledtrig_torch);
    led_trigger_unregister_simple(ledtrig_flash);
    }
    module_exit(ledtrig_camera_exit);
    MODULE_DESCRIPTION("LED Trigger for Camera Flash/Torch Control");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL v2");
