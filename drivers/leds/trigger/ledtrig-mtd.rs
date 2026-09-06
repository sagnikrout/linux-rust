//! Automatically rewritten from C to Rust
//! Source: drivers/leds/trigger/ledtrig-mtd.c
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
// LED MTD trigger
//
// Copyright 2016 Ezequiel Garcia <ezequiel@vanguardiasur.com.ar>
//
// Based on LED IDE-Disk Activity Trigger
//
// Copyright 2006 Openedhand Ltd.
//
// Author: Richard Purdie <rpurdie@openedhand.com>
//

pub const BLINK_DELAY: c_int = 30;
    DEFINE_LED_TRIGGER(ledtrig_mtd);
    DEFINE_LED_TRIGGER(ledtrig_nand);
#[no_mangle]
pub unsafe extern "C" fn ledtrig_mtd_activity() {
    void ledtrig_mtd_activity(void)
    {
    led_trigger_blink_oneshot(ledtrig_mtd, BLINK_DELAY, BLINK_DELAY, 0);
    led_trigger_blink_oneshot(ledtrig_nand, BLINK_DELAY, BLINK_DELAY, 0);
    }
    EXPORT_SYMBOL(ledtrig_mtd_activity);
#[no_mangle]
unsafe extern "C" fn ledtrig_mtd_init() -> int __init {
    static int __init ledtrig_mtd_init(void)
    {
    led_trigger_register_simple("mtd", &ledtrig_mtd);
    led_trigger_register_simple("nand-disk", &ledtrig_nand);
    return 0;
    }
    device_initcall(ledtrig_mtd_init);
