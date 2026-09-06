//! Automatically rewritten from C to Rust
//! Source: drivers/iio/trigger/iio-trig-hrtimer.c
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
// The industrial I/O periodic hrtimer trigger driver
//
// Copyright (C) Intuitive Aerial AB
// Written by Marten Svanfeldt, marten@intuitiveaerial.com
// Copyright (C) 2012, Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
// Copyright (C) 2015, Intel Corporation
//

// Defined locally, not in time64.h yet.

// default sampling frequency - 100Hz
pub const HRTIMER_DEFAULT_SAMPLING_FREQUENCY: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_hrtimer_info {
    pub swt: iio_sw_trigger,
    pub timer: hrtimer,
    pub sampling_frequency: [c_int; 2],
    pub period: ktime_t,
}

    static const struct config_item_type iio_hrtimer_type = {
    .ct_owner = THIS_MODULE,
    };
    static
    ssize_t iio_hrtimer_show_sampling_frequency(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct iio_trigger *trig = to_iio_trigger(dev);
    struct iio_hrtimer_info *info = iio_trigger_get_drvdata(trig);
    return iio_format_value(buf, IIO_VAL_INT_PLUS_MICRO,
    ARRAY_SIZE(info.sampling_frequency),
    info.sampling_frequency);
    }
    static
    ssize_t iio_hrtimer_store_sampling_frequency(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct iio_trigger *trig = to_iio_trigger(dev);
    struct iio_hrtimer_info *info = iio_trigger_get_drvdata(trig);
    unsigned long long val;
    u64 period;
    int integer, fract, ret;
    ret = iio_str_to_fixpoint(buf, 100, &integer, &fract);
    if (ret)
    return ret;
    if (integer < 0 || fract < 0)
    return -ERANGE;
    val = fract + 1000ULL * integer;  /* mHz */
    if (!val || val > UINT_MAX)
    return -EINVAL;
    info.sampling_frequency[0] = integer;  /* Hz */
    info.sampling_frequency[1] = fract * 1000;  /* uHz */
    period = PSEC_PER_SEC;
    do_div(period, val);
    info.period = period;  /* nS */
    return len;
    }
    static DEVICE_ATTR(sampling_frequency, S_IRUGO | S_IWUSR,
    iio_hrtimer_show_sampling_frequency,
    iio_hrtimer_store_sampling_frequency);
    static struct attribute *iio_hrtimer_attrs[] = {
    &dev_attr_sampling_frequency.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group iio_hrtimer_attr_group = {
    .attrs = iio_hrtimer_attrs,
    };
    static const struct attribute_group *iio_hrtimer_attr_groups[] = {
    &iio_hrtimer_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn iio_hrtimer_trig_handler(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart iio_hrtimer_trig_handler(struct hrtimer *timer)
    {
    struct iio_hrtimer_info *info;
    info = container_of(timer, struct iio_hrtimer_info, timer);
    hrtimer_forward_now(timer, info.period);
    iio_trigger_poll(info.swt.trigger);
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn iio_trig_hrtimer_set_state(trig: *mut iio_trigger, state: bool) -> c_int {
    static int iio_trig_hrtimer_set_state(struct iio_trigger *trig, bool state)
    {
    struct iio_hrtimer_info *trig_info;
    trig_info = iio_trigger_get_drvdata(trig);
    if (state)
    hrtimer_start(&trig_info.timer, trig_info.period,
    HRTIMER_MODE_REL_HARD);
    else
    hrtimer_cancel(&trig_info.timer);
    return 0;
    }
    static const struct iio_trigger_ops iio_hrtimer_trigger_ops = {
    .set_trigger_state = iio_trig_hrtimer_set_state,
    };
    static struct iio_sw_trigger *iio_trig_hrtimer_probe(const char *name)
    {
    struct iio_hrtimer_info *trig_info;
    int ret;
    trig_info = kzalloc_obj(*trig_info);
    if (!trig_info)
    return ERR_PTR(-ENOMEM);
    trig_info.swt.trigger = iio_trigger_alloc(core::ptr::null_mut(), "%s", name);
    if (!trig_info.swt.trigger) {
    ret = -ENOMEM;
    goto err_free_trig_info;
    }
    iio_trigger_set_drvdata(trig_info.swt.trigger, trig_info);
    trig_info.swt.trigger.ops = &iio_hrtimer_trigger_ops;
    trig_info.swt.trigger.dev.groups = iio_hrtimer_attr_groups;
    hrtimer_setup(&trig_info.timer, iio_hrtimer_trig_handler, CLOCK_MONOTONIC,
    HRTIMER_MODE_REL_HARD);
    trig_info.sampling_frequency[0] = HRTIMER_DEFAULT_SAMPLING_FREQUENCY;
    trig_info.period = NSEC_PER_SEC / trig_info.sampling_frequency[0];
    ret = iio_trigger_register(trig_info.swt.trigger);
    if (ret)
    goto err_free_trigger;
    iio_swt_group_init_type_name(&trig_info.swt, name, &iio_hrtimer_type);
    return &trig_info.swt;
    err_free_trigger:
    iio_trigger_free(trig_info.swt.trigger);
    err_free_trig_info:
    kfree(trig_info);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn iio_trig_hrtimer_remove(swt: *mut iio_sw_trigger) -> c_int {
    static int iio_trig_hrtimer_remove(struct iio_sw_trigger *swt)
    {
    struct iio_hrtimer_info *trig_info;
    trig_info = iio_trigger_get_drvdata(swt.trigger);
    iio_trigger_unregister(swt.trigger);
// cancel the timer after unreg to make sure no one rearms it
    hrtimer_cancel(&trig_info.timer);
    iio_trigger_free(swt.trigger);
    kfree(trig_info);
    return 0;
    }
    static const struct iio_sw_trigger_ops iio_trig_hrtimer_ops = {
    .probe		= iio_trig_hrtimer_probe,
    .remove		= iio_trig_hrtimer_remove,
    };
    static struct iio_sw_trigger_type iio_trig_hrtimer = {
    .name = "hrtimer",
    .owner = THIS_MODULE,
    .ops = &iio_trig_hrtimer_ops,
    };
    module_iio_sw_trigger_driver(iio_trig_hrtimer);
    MODULE_AUTHOR("Marten Svanfeldt <marten@intuitiveaerial.com>");
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com>");
    MODULE_DESCRIPTION("Periodic hrtimer trigger for the IIO subsystem");
    MODULE_LICENSE("GPL v2");
