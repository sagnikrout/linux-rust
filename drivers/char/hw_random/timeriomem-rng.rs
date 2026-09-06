//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/timeriomem-rng.c
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
// drivers/char/hw_random/timeriomem-rng.c
//
// Copyright (C) 2009 Alexander Clouter <alex@digriz.org.uk>
//
// Derived from drivers/char/hw_random/omap-rng.c
// Copyright 2005 (c) MontaVista Software, Inc.
// Author: Deepak Saxena <dsaxena@plexity.net>
//
// Overview:
// This driver is useful for platforms that have an IO range that provides
// periodic random data from a single IO memory address.  All the platform
// has to do is provide the address and 'wait time' that new data becomes
// available.
//
// TODO: add support for reading sizes other than 32bits and masking
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeriomem_rng_private {
    pub io_base: *mut void __iomem,
    pub period: ktime_t,
    pub present:1: c_uint,
    pub timer: hrtimer,
    pub completion: completion,
    pub rng_ops: hwrng,
}

    static int timeriomem_rng_read(struct hwrng *hwrng, void *data,
    size_t max, bool wait)
    {
    struct timeriomem_rng_private *priv =
    container_of(hwrng, struct timeriomem_rng_private, rng_ops);
    let mut retval: c_int = 0;
    let mut period_us: c_int = ktime_to_us(priv.period);
//
// There may not have been enough time for new data to be generated
// since the last request.  If the caller doesn't want to wait, let them
// bail out.  Otherwise, wait for the completion.  If the new data has
// already been generated, the completion should already be available.
//
    if (!wait && !priv.present)
    return 0;
    wait_for_completion(&priv.completion);
    do {
//
// After the first read, all additional reads will need to wait
// for the RNG to generate new data.  Since the period can have
// a wide range of values (1us to 1s have been observed), allow
// for 1% tolerance in the sleep time rather than a fixed value.
//
    if (retval > 0)
    usleep_range(period_us,
    period_us + max(1, period_us / 100));
// (u32 *)data = readl(priv->io_base);
    retval += sizeof(u32);
    data += sizeof(u32);
    max -= sizeof(u32);
    } while (wait && max > sizeof(u32));
//
// Block any new callers until the RNG has had time to generate new
// data.
//
    priv.present = 0;
    reinit_completion(&priv.completion);
    hrtimer_forward_now(&priv.timer, priv.period);
    hrtimer_restart(&priv.timer);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn timeriomem_rng_trigger(timer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart timeriomem_rng_trigger(struct hrtimer *timer)
    {
    struct timeriomem_rng_private *priv
    = container_of(timer, struct timeriomem_rng_private, timer);
    priv.present = 1;
    complete(&priv.completion);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn timeriomem_rng_probe(pdev: *mut platform_device) -> c_int {
    static int timeriomem_rng_probe(struct platform_device *pdev)
    {
    struct timeriomem_rng_data *pdata = pdev.dev.platform_data;
    struct timeriomem_rng_private *priv;
    struct resource *res;
    let mut err: c_int = 0;
    int period;
    if (!pdev.dev.of_node && !pdata) {
    dev_err(&pdev.dev, "timeriomem_rng_data is missing\n");
    return -EINVAL;
    }
// Allocate memory for the device structure (and zero it)
    priv = devm_kzalloc(&pdev.dev,
    sizeof(struct timeriomem_rng_private), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.io_base))
    return PTR_ERR(priv.io_base);
    if (res.start % 4 != 0 || resource_size(res) < 4) {
    dev_err(&pdev.dev,
    "address must be at least four bytes wide and 32-bit aligned\n");
    return -EINVAL;
    }
    if (pdev.dev.of_node) {
    int i;
    if (!of_property_read_u32(pdev.dev.of_node,
    "period", &i))
    period = i;
    else {
    dev_err(&pdev.dev, "missing period\n");
    return -EINVAL;
    }
    if (!of_property_read_u32(pdev.dev.of_node,
    "quality", &i))
    priv.rng_ops.quality = i;
    } else {
    period = pdata.period;
    priv.rng_ops.quality = pdata.quality;
    }
    priv.period = us_to_ktime(period);
    init_completion(&priv.completion);
    hrtimer_setup(&priv.timer, timeriomem_rng_trigger, CLOCK_MONOTONIC, HRTIMER_MODE_ABS);
    priv.rng_ops.name = dev_name(&pdev.dev);
    priv.rng_ops.read = timeriomem_rng_read;
// Assume random data is already available.
    priv.present = 1;
    complete(&priv.completion);
    err = devm_hwrng_register(&pdev.dev, &priv.rng_ops);
    if (err) {
    dev_err(&pdev.dev, "problem registering\n");
    return err;
    }
    dev_info(&pdev.dev, "32bits from 0x%p @ %dus\n",
    priv.io_base, period);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timeriomem_rng_remove(pdev: *mut platform_device) {
    static void timeriomem_rng_remove(struct platform_device *pdev)
    {
    struct timeriomem_rng_private *priv = platform_get_drvdata(pdev);
    hrtimer_cancel(&priv.timer);
    }
    static const struct of_device_id timeriomem_rng_match[] = {
    { .compatible = "timeriomem_rng" },
    {},
    };
    MODULE_DEVICE_TABLE(of, timeriomem_rng_match);
    static struct platform_driver timeriomem_rng_driver = {
    .driver = {
    .name		= "timeriomem_rng",
    .of_match_table	= timeriomem_rng_match,
    },
    .probe		= timeriomem_rng_probe,
    .remove		= timeriomem_rng_remove,
    };
    module_platform_driver(timeriomem_rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Clouter <alex@digriz.org.uk>");
    MODULE_DESCRIPTION("Timer IOMEM H/W RNG driver");
