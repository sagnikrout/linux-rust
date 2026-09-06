//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/fakekey.c
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
// fakekey.c
// Functions for simulating key presses.
//
// Copyright (C) 2010 the Speakup Team
//

pub const PRESSED: c_int = 1;
pub const RELEASED: c_int = 0;
    static DEFINE_PER_CPU(int, reporting_keystroke);
    static struct input_dev *virt_keyboard;
#[no_mangle]
pub unsafe extern "C" fn speakup_add_virtual_keyboard() -> c_int {
    int speakup_add_virtual_keyboard(void)
    {
    int err;
    virt_keyboard = input_allocate_device();
    if (!virt_keyboard)
    return -ENOMEM;
    virt_keyboard.name = "Speakup";
    virt_keyboard.id.bustype = BUS_VIRTUAL;
    virt_keyboard.phys = "speakup/input0";
    virt_keyboard.dev.parent = core::ptr::null_mut();
    __set_bit(EV_KEY, virt_keyboard.evbit);
    __set_bit(KEY_DOWN, virt_keyboard.keybit);
    err = input_register_device(virt_keyboard);
    if (err) {
    input_free_device(virt_keyboard);
    virt_keyboard = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_remove_virtual_keyboard() {
    void speakup_remove_virtual_keyboard(void)
    {
    if (virt_keyboard) {
    input_unregister_device(virt_keyboard);
    virt_keyboard = core::ptr::null_mut();
    }
    }
//
// Send a simulated down-arrow to the application.
//
#[no_mangle]
pub unsafe extern "C" fn speakup_fake_down_arrow() {
    void speakup_fake_down_arrow(void)
    {
    unsigned long flags;
// disable keyboard interrupts
    local_irq_save(flags);
// don't change CPU
    preempt_disable();
    __this_cpu_write(reporting_keystroke, true);
    input_report_key(virt_keyboard, KEY_DOWN, PRESSED);
    input_report_key(virt_keyboard, KEY_DOWN, RELEASED);
    input_sync(virt_keyboard);
    __this_cpu_write(reporting_keystroke, false);
// re-enable preemption
    preempt_enable();
// re-enable keyboard interrupts
    local_irq_restore(flags);
    }
//
// Are we handling a simulated key press on the current CPU?
// Returns a boolean.
//
#[no_mangle]
pub unsafe extern "C" fn speakup_fake_key_pressed() -> bool {
    bool speakup_fake_key_pressed(void)
    {
    return this_cpu_read(reporting_keystroke);
    }
