//! Automatically rewritten from C to Rust
//! Source: drivers/devfreq/governor_performance.c
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
// linux/drivers/devfreq/governor_performance.c
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

    static int devfreq_performance_func(struct devfreq *df,
    unsigned long *freq)
    {
//
// target callback should be able to get floor value as
// said in devfreq.h
//
// freq = DEVFREQ_MAX_FREQ;
    return 0;
    }
    static int devfreq_performance_handler(struct devfreq *devfreq,
    unsigned int event, void *data)
    {
    let mut ret: c_int = 0;
    if (event == DEVFREQ_GOV_START) {
    mutex_lock(&devfreq.lock);
    ret = update_devfreq(devfreq);
    mutex_unlock(&devfreq.lock);
    }
    return ret;
    }
    static struct devfreq_governor devfreq_performance = {
    .name = DEVFREQ_GOV_PERFORMANCE,
    .get_target_freq = devfreq_performance_func,
    .event_handler = devfreq_performance_handler,
    };
#[no_mangle]
unsafe extern "C" fn devfreq_performance_init() -> int __init {
    static int __init devfreq_performance_init(void)
    {
    return devfreq_add_governor(&devfreq_performance);
    }
    subsys_initcall(devfreq_performance_init);
#[no_mangle]
unsafe extern "C" fn devfreq_performance_exit() -> void __exit {
    static void __exit devfreq_performance_exit(void)
    {
    int ret;
    ret = devfreq_remove_governor(&devfreq_performance);
    if (ret)
    pr_err("%s: failed remove governor %d\n", __func__, ret);
    return;
    }
    module_exit(devfreq_performance_exit);
    MODULE_DESCRIPTION("DEVFREQ Performance governor");
    MODULE_LICENSE("GPL");
