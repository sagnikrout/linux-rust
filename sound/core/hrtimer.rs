//! Automatically rewritten from C to Rust
//! Source: sound/core/hrtimer.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA timer back-end using hrtimer
// Copyright (C) 2008 Takashi Iwai
//

    MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
    MODULE_DESCRIPTION("ALSA hrtimer backend");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("snd-timer-" __stringify(SNDRV_TIMER_GLOBAL_HRTIMER));

    static unsigned int resolution __ro_after_init;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hrtimer {
    pub timer: *mut snd_timer,
    pub hrt: hrtimer,
    pub in_callback: bool,
}

#[no_mangle]
unsafe extern "C" fn snd_hrtimer_callback(hrt: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart snd_hrtimer_callback(struct hrtimer *hrt)
    {
    struct snd_hrtimer *stime = container_of(hrt, struct snd_hrtimer, hrt);
    struct snd_timer *t = stime.timer;
    ktime_t delta;
    unsigned long ticks;
    let mut ret: enum hrtimer_restart = HRTIMER_NORESTART;
    scoped_guard(spinlock, &t.lock) {
    if (!t.running)
    return HRTIMER_NORESTART; /* fast path */
    stime.in_callback = true;
    ticks = t.sticks;
    }
// calculate the drift
    delta = ktime_sub(hrtimer_cb_get_time(hrt), hrtimer_get_expires(hrt));
    if (delta > 0)
    ticks += ktime_divns(delta, ticks * resolution);
    snd_timer_interrupt(stime.timer, ticks);
    guard(spinlock)(&t.lock);
    if (t.running) {
    hrtimer_add_expires_ns(hrt, t.sticks * resolution);
    ret = HRTIMER_RESTART;
    }
    stime.in_callback = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_open(t: *mut snd_timer) -> c_int {
    static int snd_hrtimer_open(struct snd_timer *t)
    {
    struct snd_hrtimer *stime;
    stime = kzalloc_obj(*stime);
    if (!stime)
    return -ENOMEM;
    stime.timer = t;
    hrtimer_setup(&stime.hrt, snd_hrtimer_callback, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    t.private_data = stime;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_close(t: *mut snd_timer) -> c_int {
    static int snd_hrtimer_close(struct snd_timer *t)
    {
    struct snd_hrtimer *stime = t.private_data;
    if (stime) {
    scoped_guard(spinlock_irq, &t.lock) {
    t.running = 0; /* just to be sure */
    stime.in_callback = 1; /* skip start/stop */
    }
    hrtimer_cancel(&stime.hrt);
    kfree(stime);
    t.private_data = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_start(t: *mut snd_timer) -> c_int {
    static int snd_hrtimer_start(struct snd_timer *t)
    {
    struct snd_hrtimer *stime = t.private_data;
    if (stime.in_callback)
    return 0;
    hrtimer_start(&stime.hrt, ns_to_ktime(t.sticks * resolution),
    HRTIMER_MODE_REL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_stop(t: *mut snd_timer) -> c_int {
    static int snd_hrtimer_stop(struct snd_timer *t)
    {
    struct snd_hrtimer *stime = t.private_data;
    if (stime.in_callback)
    return 0;
    hrtimer_try_to_cancel(&stime.hrt);
    return 0;
    }
    static const struct snd_timer_hardware hrtimer_hw __initconst = {
    .flags =	SNDRV_TIMER_HW_AUTO | SNDRV_TIMER_HW_WORK,
    .open =		snd_hrtimer_open,
    .close =	snd_hrtimer_close,
    .start =	snd_hrtimer_start,
    .stop =		snd_hrtimer_stop,
    };
//
// entry functions
//
    static struct snd_timer *mytimer;
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_init() -> int __init {
    static int __init snd_hrtimer_init(void)
    {
    struct snd_timer *timer;
    int err;
    resolution = hrtimer_resolution;
// Create a new timer and set up the fields
    err = snd_timer_global_new("hrtimer", SNDRV_TIMER_GLOBAL_HRTIMER,
    &timer);
    if (err < 0)
    return err;
    timer.module = THIS_MODULE;
    strscpy(timer.name, "HR timer");
    timer.hw = hrtimer_hw;
    timer.hw.resolution = resolution;
    timer.hw.ticks = NANO_SEC / resolution;
    timer.max_instances = 100; /* lower the limit */
    err = snd_timer_global_register(timer);
    if (err < 0) {
    snd_timer_global_free(timer);
    return err;
    }
    mytimer = timer; /* remember this */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_hrtimer_exit() -> void __exit {
    static void __exit snd_hrtimer_exit(void)
    {
    if (mytimer) {
    snd_timer_global_free(mytimer);
    mytimer = core::ptr::null_mut();
    }
    }
    module_init(snd_hrtimer_init);
    module_exit(snd_hrtimer_exit);
