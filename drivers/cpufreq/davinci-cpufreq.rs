//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/davinci-cpufreq.c
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
// CPU frequency scaling for DaVinci
//
// Copyright (C) 2009 Texas Instruments Incorporated - https://www.ti.com
//
// Based on linux/arch/arm/plat-omap/cpu-omap.c. Original Copyright follows:
//
// Copyright (C) 2005 Nokia Corporation
// Written by Tony Lindgren <tony@atomide.com>
//
// Based on cpu-sa1110.c, Copyright (C) 2001 Russell King
//
// Copyright (C) 2007-2008 Texas Instruments, Inc.
// Updated to support OMAP3
// Rajendra Nayak <rnayak@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_cpufreq {
    pub dev: *mut device,
    pub armclk: *mut clk,
    pub asyncclk: *mut clk,
    pub asyncrate: c_ulong,
}

    static struct davinci_cpufreq cpufreq;
#[no_mangle]
unsafe extern "C" fn davinci_target(policy: *mut cpufreq_policy, idx: c_uint) -> c_int {
    static int davinci_target(struct cpufreq_policy *policy, unsigned int idx)
    {
    struct davinci_cpufreq_config *pdata = cpufreq.dev.platform_data;
    struct clk *armclk = cpufreq.armclk;
    unsigned int old_freq, new_freq;
    let mut ret: c_int = 0;
    old_freq = policy.cur;
    new_freq = pdata.freq_table[idx].frequency;
// if moving to higher frequency, up the voltage beforehand
    if (pdata.set_voltage && new_freq > old_freq) {
    ret = pdata.set_voltage(idx);
    if (ret)
    return ret;
    }
    ret = clk_set_rate(armclk, new_freq * 1000);
    if (ret)
    return ret;
    if (cpufreq.asyncclk) {
    ret = clk_set_rate(cpufreq.asyncclk, cpufreq.asyncrate);
    if (ret)
    return ret;
    }
// if moving to lower freq, lower the voltage after lowering freq
    if (pdata.set_voltage && new_freq < old_freq)
    pdata.set_voltage(idx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn davinci_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int davinci_cpu_init(struct cpufreq_policy *policy)
    {
    let mut result: c_int = 0;
    struct davinci_cpufreq_config *pdata = cpufreq.dev.platform_data;
    struct cpufreq_frequency_table *freq_table = pdata.freq_table;
    if (policy.cpu != 0)
    return -EINVAL;
// Finish platform specific initialization
    if (pdata.init) {
    result = pdata.init();
    if (result)
    return result;
    }
    policy.clk = cpufreq.armclk;
//
// Time measurement across the target() function yields ~1500-1800us
// time taken with no drivers on notification list.
// Setting the latency to 2000 us to accommodate addition of drivers
// to pre/post change notification list.
//
    cpufreq_generic_init(policy, freq_table, 2000 * 1000);
    return 0;
    }
    static struct cpufreq_driver davinci_driver = {
    .flags		= CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= davinci_target,
    .get		= cpufreq_generic_get,
    .init		= davinci_cpu_init,
    .name		= "davinci",
    };
#[no_mangle]
unsafe extern "C" fn davinci_cpufreq_probe(pdev: *mut platform_device) -> int __init {
    static int __init davinci_cpufreq_probe(struct platform_device *pdev)
    {
    struct davinci_cpufreq_config *pdata = pdev.dev.platform_data;
    struct clk *asyncclk;
    if (!pdata)
    return -EINVAL;
    if (!pdata.freq_table)
    return -EINVAL;
    cpufreq.dev = &pdev.dev;
    cpufreq.armclk = clk_get(core::ptr::null_mut(), "arm");
    if (IS_ERR(cpufreq.armclk)) {
    dev_err(cpufreq.dev, "Unable to get ARM clock\n");
    return PTR_ERR(cpufreq.armclk);
    }
    asyncclk = clk_get(cpufreq.dev, "async");
    if (!IS_ERR(asyncclk)) {
    cpufreq.asyncclk = asyncclk;
    cpufreq.asyncrate = clk_get_rate(asyncclk);
    }
    return cpufreq_register_driver(&davinci_driver);
    }
#[no_mangle]
unsafe extern "C" fn davinci_cpufreq_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit davinci_cpufreq_remove(struct platform_device *pdev)
    {
    cpufreq_unregister_driver(&davinci_driver);
    clk_put(cpufreq.armclk);
    if (cpufreq.asyncclk)
    clk_put(cpufreq.asyncclk);
    }
    static struct platform_driver davinci_cpufreq_driver = {
    .driver = {
    .name	 = "cpufreq-davinci",
    },
    .remove = __exit_p(davinci_cpufreq_remove),
    };
#[no_mangle]
pub unsafe extern "C" fn davinci_cpufreq_init() -> int __init {
    int __init davinci_cpufreq_init(void)
    {
    return platform_driver_probe(&davinci_cpufreq_driver,
    davinci_cpufreq_probe);
    }
