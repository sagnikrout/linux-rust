//! Automatically rewritten from C to Rust
//! Source: drivers/devfreq/governor_simpleondemand.c
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
// linux/drivers/devfreq/governor_simpleondemand.c
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

// Default constants for DevFreq-Simple-Ondemand (DFSO)

    static int devfreq_simple_ondemand_func(struct devfreq *df,
    unsigned long *freq)
    {
    int err;
    struct devfreq_dev_status *stat;
    unsigned long long a, b;
    let mut dfso_upthreshold: c_uint = DFSO_UPTHRESHOLD;
    let mut dfso_downdifferential: c_uint = DFSO_DOWNDIFFERENTIAL;
    struct devfreq_simple_ondemand_data *data = df.data;
    err = devfreq_update_stats(df);
    if (err)
    return err;
    stat = &df.last_status;
    if (data) {
    if (data.upthreshold)
    dfso_upthreshold = data.upthreshold;
    if (data.downdifferential)
    dfso_downdifferential = data.downdifferential;
    }
    if (dfso_upthreshold > 100 ||
    dfso_upthreshold < dfso_downdifferential)
    return -EINVAL;
// Assume MAX if it is going to be divided by zero
    if (stat.total_time == 0) {
// freq = DEVFREQ_MAX_FREQ;
    return 0;
    }
// Prevent overflow
    if (stat.busy_time >= (1 << 24) || stat.total_time >= (1 << 24)) {
    stat.busy_time >>= 7;
    stat.total_time >>= 7;
    }
// Set MAX if it's busy enough
    if (stat.busy_time * 100 >
    stat.total_time * dfso_upthreshold) {
// freq = DEVFREQ_MAX_FREQ;
    return 0;
    }
// Set MAX if we do not know the initial frequency
    if (stat.current_frequency == 0) {
// freq = DEVFREQ_MAX_FREQ;
    return 0;
    }
// Keep the current frequency
    if (stat.busy_time * 100 >
    stat.total_time * (dfso_upthreshold - dfso_downdifferential)) {
// freq = stat->current_frequency;
    return 0;
    }
// Set the desired frequency based on the load
    a = stat.busy_time;
    a *= stat.current_frequency;
    b = div_u64(a, stat.total_time);
    b *= 100;
    b = div_u64(b, (dfso_upthreshold - dfso_downdifferential / 2));
// freq = (unsigned long) b;
    return 0;
    }
    static int devfreq_simple_ondemand_handler(struct devfreq *devfreq,
    unsigned int event, void *data)
    {
    switch (event) {
    case DEVFREQ_GOV_START:
    devfreq_monitor_start(devfreq);
    break;
    case DEVFREQ_GOV_STOP:
    devfreq_monitor_stop(devfreq);
    break;
    case DEVFREQ_GOV_UPDATE_INTERVAL:
    devfreq_update_interval(devfreq, (unsigned int *)data);
    break;
    case DEVFREQ_GOV_SUSPEND:
    devfreq_monitor_suspend(devfreq);
    break;
    case DEVFREQ_GOV_RESUME:
    devfreq_monitor_resume(devfreq);
    break;
    default:
    break;
    }
    return 0;
    }
    static struct devfreq_governor devfreq_simple_ondemand = {
    .name = DEVFREQ_GOV_SIMPLE_ONDEMAND,
    .attrs = DEVFREQ_GOV_ATTR_POLLING_INTERVAL
    | DEVFREQ_GOV_ATTR_TIMER,
    .get_target_freq = devfreq_simple_ondemand_func,
    .event_handler = devfreq_simple_ondemand_handler,
    };
#[no_mangle]
unsafe extern "C" fn devfreq_simple_ondemand_init() -> int __init {
    static int __init devfreq_simple_ondemand_init(void)
    {
    return devfreq_add_governor(&devfreq_simple_ondemand);
    }
    subsys_initcall(devfreq_simple_ondemand_init);
#[no_mangle]
unsafe extern "C" fn devfreq_simple_ondemand_exit() -> void __exit {
    static void __exit devfreq_simple_ondemand_exit(void)
    {
    int ret;
    ret = devfreq_remove_governor(&devfreq_simple_ondemand);
    if (ret)
    pr_err("%s: failed remove governor %d\n", __func__, ret);
    return;
    }
    module_exit(devfreq_simple_ondemand_exit);
    MODULE_DESCRIPTION("DEVFREQ Simple On-demand governor");
    MODULE_LICENSE("GPL");
