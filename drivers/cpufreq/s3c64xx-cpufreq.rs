//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/s3c64xx-cpufreq.c
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
// Copyright 2009 Wolfson Microelectronics plc
//
// S3C64xx CPUfreq Support
//

    static struct regulator *vddarm;
    static unsigned long regulator_latency;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c64xx_dvfs {
    pub vddarm_min: c_uint,
    pub vddarm_max: c_uint,
}

    static struct s3c64xx_dvfs s3c64xx_dvfs_table[] = {
    [0] = { 1000000, 1150000 },
    [1] = { 1050000, 1150000 },
    [2] = { 1100000, 1150000 },
    [3] = { 1200000, 1350000 },
    [4] = { 1300000, 1350000 },
    };

    static struct cpufreq_frequency_table s3c64xx_freq_table[] = {
    { 0, 0,  66000 },
    { 0, 0, 100000 },
    { 0, 0, 133000 },
    { 0, 1, 200000 },
    { 0, 1, 222000 },
    { 0, 1, 266000 },
    { 0, 2, 333000 },
    { 0, 2, 400000 },
    { 0, 2, 532000 },
    { 0, 2, 533000 },
    { 0, 3, 667000 },
    { 0, 4, 800000 },
    { 0, 0, CPUFREQ_TABLE_END },
    };
    static int s3c64xx_cpufreq_set_target(struct cpufreq_policy *policy,
    unsigned int index)
    {
    let mut new_freq: c_uint = s3c64xx_freq_table[index].frequency;
    int ret;

    struct s3c64xx_dvfs *dvfs;
    unsigned int old_freq;
    old_freq = clk_get_rate(policy.clk) / 1000;
    dvfs = &s3c64xx_dvfs_table[s3c64xx_freq_table[index].driver_data];
    if (vddarm && new_freq > old_freq) {
    ret = regulator_set_voltage(vddarm,
    dvfs.vddarm_min,
    dvfs.vddarm_max);
    if (ret != 0) {
    pr_err("Failed to set VDDARM for %dkHz: %d\n",
    new_freq, ret);
    return ret;
    }
    }

    ret = clk_set_rate(policy.clk, new_freq * 1000);
    if (ret < 0) {
    pr_err("Failed to set rate %dkHz: %d\n",
    new_freq, ret);
    return ret;
    }

    if (vddarm && new_freq < old_freq) {
    ret = regulator_set_voltage(vddarm,
    dvfs.vddarm_min,
    dvfs.vddarm_max);
    if (ret != 0) {
    pr_err("Failed to set VDDARM for %dkHz: %d\n",
    new_freq, ret);
    if (clk_set_rate(policy.clk, old_freq * 1000) < 0)
    pr_err("Failed to restore original clock rate\n");
    return ret;
    }
    }

    pr_debug("Set actual frequency %lukHz\n",
    clk_get_rate(policy.clk) / 1000);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn s3c64xx_cpufreq_config_regulator() {
    static void s3c64xx_cpufreq_config_regulator(void)
    {
    int count, v, i, found;
    struct cpufreq_frequency_table *freq;
    struct s3c64xx_dvfs *dvfs;
    count = regulator_count_voltages(vddarm);
    if (count < 0) {
    pr_err("Unable to check supported voltages\n");
    }
    if (!count)
    goto out;
    cpufreq_for_each_valid_entry(freq, s3c64xx_freq_table) {
    dvfs = &s3c64xx_dvfs_table[freq.driver_data];
    found = 0;
    for (i = 0; i < count; i++) {
    v = regulator_list_voltage(vddarm, i);
    if (v >= dvfs.vddarm_min && v <= dvfs.vddarm_max)
    found = 1;
    }
    if (!found) {
    pr_debug("%dkHz unsupported by regulator\n",
    freq.frequency);
    freq.frequency = CPUFREQ_ENTRY_INVALID;
    }
    }
    out:
// Guess based on having to do an I2C/SPI write; in future we
// will be able to query the regulator performance here.
    regulator_latency = 1 * 1000 * 1000;
    }

#[no_mangle]
unsafe extern "C" fn s3c64xx_cpufreq_driver_init(policy: *mut cpufreq_policy) -> c_int {
    static int s3c64xx_cpufreq_driver_init(struct cpufreq_policy *policy)
    {
    struct cpufreq_frequency_table *freq;
    if (policy.cpu != 0)
    return -EINVAL;
    policy.clk = clk_get(core::ptr::null_mut(), "armclk");
    if (IS_ERR(policy.clk)) {
    pr_err("Unable to obtain ARMCLK: %ld\n",
    PTR_ERR(policy.clk));
    return PTR_ERR(policy.clk);
    }

    vddarm = regulator_get(core::ptr::null_mut(), "vddarm");
    if (IS_ERR(vddarm)) {
    pr_err("Failed to obtain VDDARM: %ld\n", PTR_ERR(vddarm));
    pr_err("Only frequency scaling available\n");
    vddarm = core::ptr::null_mut();
    } else {
    s3c64xx_cpufreq_config_regulator();
    }

    cpufreq_for_each_entry(freq, s3c64xx_freq_table) {
    unsigned long r;
// Check for frequencies we can generate
    r = clk_round_rate(policy.clk, freq.frequency * 1000);
    r /= 1000;
    if (r != freq.frequency) {
    pr_debug("%dkHz unsupported by clock\n",
    freq.frequency);
    freq.frequency = CPUFREQ_ENTRY_INVALID;
    }
// If we have no regulator then assume startup
// frequency is the maximum we can support.
    if (!vddarm && freq.frequency > clk_get_rate(policy.clk) / 1000)
    freq.frequency = CPUFREQ_ENTRY_INVALID;
    }
// Datasheet says PLL stabalisation time (if we were to use
// the PLLs, which we don't currently) is ~300us worst case,
// but add some fudge.
//
    cpufreq_generic_init(policy, s3c64xx_freq_table,
    (500 * 1000) + regulator_latency);
    return 0;
    }
    static struct cpufreq_driver s3c64xx_cpufreq_driver = {
    .flags		= CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= s3c64xx_cpufreq_set_target,
    .get		= cpufreq_generic_get,
    .init		= s3c64xx_cpufreq_driver_init,
    .name		= "s3c",
    };
#[no_mangle]
unsafe extern "C" fn s3c64xx_cpufreq_init() -> int __init {
    static int __init s3c64xx_cpufreq_init(void)
    {
    return cpufreq_register_driver(&s3c64xx_cpufreq_driver);
    }
    module_init(s3c64xx_cpufreq_init);
