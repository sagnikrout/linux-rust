//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/raspberrypi-cpufreq.c
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
// Raspberry Pi cpufreq driver
//
// Copyright (C) 2019, Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
//

pub const RASPBERRYPI_FREQ_INTERVAL: c_int = 100000000;
    static struct platform_device *cpufreq_dt;
#[no_mangle]
unsafe extern "C" fn raspberrypi_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int raspberrypi_cpufreq_probe(struct platform_device *pdev)
    {
    struct device *cpu_dev;
    unsigned long min, max;
    unsigned long rate;
    struct clk *clk;
    int ret;
    cpu_dev = get_cpu_device(0);
    if (!cpu_dev) {
    pr_err("Cannot get CPU for cpufreq driver\n");
    return -ENODEV;
    }
    clk = clk_get(cpu_dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(cpu_dev, "Cannot get clock for CPU0\n");
    return PTR_ERR(clk);
    }
//
// The max and min frequencies are configurable in the Raspberry Pi
// firmware, so we query them at runtime.
//
    min = roundup(clk_round_rate(clk, 0), RASPBERRYPI_FREQ_INTERVAL);
    max = roundup(clk_round_rate(clk, ULONG_MAX), RASPBERRYPI_FREQ_INTERVAL);
    clk_put(clk);
    for (rate = min; rate <= max; rate += RASPBERRYPI_FREQ_INTERVAL) {
    ret = dev_pm_opp_add(cpu_dev, rate, 0);
    if (ret)
    goto remove_opp;
    }
    cpufreq_dt = platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    ret = PTR_ERR_OR_ZERO(cpufreq_dt);
    if (ret) {
    dev_err(cpu_dev, "Failed to create platform device, %d\n", ret);
    goto remove_opp;
    }
    return 0;
    remove_opp:
    dev_pm_opp_remove_all_dynamic(cpu_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn raspberrypi_cpufreq_remove(pdev: *mut platform_device) {
    static void raspberrypi_cpufreq_remove(struct platform_device *pdev)
    {
    struct device *cpu_dev;
    cpu_dev = get_cpu_device(0);
    if (cpu_dev)
    dev_pm_opp_remove_all_dynamic(cpu_dev);
    platform_device_unregister(cpufreq_dt);
    }
//
// Since the driver depends on clk-raspberrypi, which may return EPROBE_DEFER,
// all the activity is performed in the probe, which may be defered as well.
//
    static struct platform_driver raspberrypi_cpufreq_driver = {
    .driver = {
    .name = "raspberrypi-cpufreq",
    },
    .probe          = raspberrypi_cpufreq_probe,
    .remove		= raspberrypi_cpufreq_remove,
    };
    module_platform_driver(raspberrypi_cpufreq_driver);
    MODULE_AUTHOR("Nicolas Saenz Julienne <nsaenzjulienne@suse.de");
    MODULE_DESCRIPTION("Raspberry Pi cpufreq driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:raspberrypi-cpufreq");
