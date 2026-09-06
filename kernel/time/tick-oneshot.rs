//! Automatically rewritten from C to Rust
//! Source: kernel/time/tick-oneshot.c
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
// This file contains functions which manage high resolution tick
// related events.
//
// Copyright(C) 2005-2006, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005-2007, Red Hat, Inc., Ingo Molnar
// Copyright(C) 2006-2007, Timesys Corp., Thomas Gleixner
//

//
// tick_program_event - program the CPU local timer device for the next event
// @expires: the time at which the next timer event should occur
// @force: flag to force reprograming even if the event time hasn't changed
//
// Return: 0 on success, negative error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn tick_program_event(expires: ktime_t, force: c_int) -> c_int {
    int tick_program_event(ktime_t expires, int force)
    {
    struct clock_event_device *dev = __this_cpu_read(tick_cpu_device.evtdev);
    if (unlikely(expires == KTIME_MAX)) {
//
// We don't need the clock event device any more, stop it.
//
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT_STOPPED);
    dev.next_event = KTIME_MAX;
    return 0;
    }
    if (unlikely(clockevent_state_oneshot_stopped(dev))) {
//
// We need the clock event again, configure it in ONESHOT mode
// before using it.
//
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    }
    return clockevents_program_event(dev, expires, force);
    }
//
// tick_resume_oneshot - resume oneshot mode
//
#[no_mangle]
pub unsafe extern "C" fn tick_resume_oneshot() {
    void tick_resume_oneshot(void)
    {
    struct clock_event_device *dev = __this_cpu_read(tick_cpu_device.evtdev);
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    clockevents_program_event(dev, ktime_get(), true);
    }
//
// tick_setup_oneshot - setup the event device for oneshot mode (hres or nohz)
// @newdev: Pointer to the clock event device to configure
// @handler: Function to be called when the event device triggers an interrupt
// @next_event: Initial expiry time for the next event (in ktime)
//
// Configures the specified clock event device for onshot mode,
// assigns the given handler as its event callback, and programs
// the device to trigger at the specified next event time.
//
    void tick_setup_oneshot(struct clock_event_device *newdev,
    void (*handler)(struct clock_event_device *),
    ktime_t next_event)
    {
    newdev.event_handler = handler;
    clockevents_switch_state(newdev, CLOCK_EVT_STATE_ONESHOT);
    clockevents_program_event(newdev, next_event, true);
    }
//
// tick_switch_to_oneshot - switch to oneshot mode
// @handler: function to call when an event occurs on the tick device
//
// Return: 0 on success, -EINVAL if the tick device is not present,
// not functional, or does not support oneshot mode.
//
#[no_mangle]
pub unsafe extern "C" fn tick_switch_to_oneshot(): *mut *mut void (handler)(struct clock_event_device) -> c_int {
    int tick_switch_to_oneshot(void (*handler)(struct clock_event_device *))
    {
    struct tick_device *td = this_cpu_ptr(&tick_cpu_device);
    struct clock_event_device *dev = td.evtdev;
    if (!dev || !(dev.features & CLOCK_EVT_FEAT_ONESHOT) ||
    !tick_device_is_functional(dev)) {
    pr_info("Clockevents: could not switch to one-shot mode:");
    if (!dev) {
    pr_cont(" no tick device\n");
    } else {
    if (!tick_device_is_functional(dev))
    pr_cont(" %s is not functional.\n", dev.name);
    else
    pr_cont(" %s does not support one-shot mode.\n",
    dev.name);
    }
    return -EINVAL;
    }
    td.mode = TICKDEV_MODE_ONESHOT;
    dev.event_handler = handler;
    clockevents_switch_state(dev, CLOCK_EVT_STATE_ONESHOT);
    tick_broadcast_switch_to_oneshot();
    return 0;
    }
//
// tick_oneshot_mode_active - check whether the system is in oneshot mode
//
// Return: 1 when either nohz or highres are enabled, otherwise 0.
//
#[no_mangle]
pub unsafe extern "C" fn tick_oneshot_mode_active() -> c_int {
    int tick_oneshot_mode_active(void)
    {
    unsigned long flags;
    int ret;
    local_irq_save(flags);
    ret = __this_cpu_read(tick_cpu_device.mode) == TICKDEV_MODE_ONESHOT;
    local_irq_restore(flags);
    return ret;
    }

//
// tick_init_highres - switch to high resolution mode
//
// Called with interrupts disabled.
//
// Return: 0 on success, -EINVAL if the tick device cannot switch
// to oneshot/high-resolution mode.
//
#[no_mangle]
pub unsafe extern "C" fn tick_init_highres() -> c_int {
    int tick_init_highres(void)
    {
    return tick_switch_to_oneshot(hrtimer_interrupt);
    }
