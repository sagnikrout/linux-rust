//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/highbank-cpufreq.c
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
// Copyright (C) 2012 Calxeda, Inc.
//
// This driver provides the clk notifier callbacks that are used when
// the cpufreq-dt driver changes to frequency to alert the highbank
// EnergyCore Management Engine (ECME) about the need to change
// voltage. The ECME interfaces with the actual voltage regulators.
//

pub const HB_CPUFREQ_CHANGE_NOTE: c_uint = 0x80000001;
pub const HB_CPUFREQ_IPC_LEN: c_int = 7;
pub const HB_CPUFREQ_VOLT_RETRIES: c_int = 15;
#[no_mangle]
unsafe extern "C" fn hb_voltage_change(freq: c_uint) -> c_int {
    static int hb_voltage_change(unsigned int freq)
    {
    u32 msg[HB_CPUFREQ_IPC_LEN] = {HB_CPUFREQ_CHANGE_NOTE, freq / 1000000};
    return pl320_ipc_transmit(msg);
    }
    static int hb_cpufreq_clk_notify(struct notifier_block *nb,
    unsigned long action, void *hclk)
    {
    struct clk_notifier_data *clk_data = hclk;
    let mut i: c_int = 0;
    if (action == PRE_RATE_CHANGE) {
    if (clk_data.new_rate > clk_data.old_rate)
    while (hb_voltage_change(clk_data.new_rate))
    if (i++ > HB_CPUFREQ_VOLT_RETRIES)
    return NOTIFY_BAD;
    } else if (action == POST_RATE_CHANGE) {
    if (clk_data.new_rate < clk_data.old_rate)
    while (hb_voltage_change(clk_data.new_rate))
    if (i++ > HB_CPUFREQ_VOLT_RETRIES)
    return NOTIFY_BAD;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block hb_cpufreq_clk_nb = {
    .notifier_call = hb_cpufreq_clk_notify,
    };
#[no_mangle]
unsafe extern "C" fn hb_cpufreq_driver_init() -> int __init {
    static int __init hb_cpufreq_driver_init(void)
    {
    let mut devinfo: platform_device_info = { .name = "cpufreq-dt", };
    struct device *cpu_dev;
    struct clk *cpu_clk;
    struct device_node *np;
    int ret;
    if ((!of_machine_is_compatible("calxeda,highbank")) &&
    (!of_machine_is_compatible("calxeda,ecx-2000")))
    return -ENODEV;
    cpu_dev = get_cpu_device(0);
    if (!cpu_dev) {
    pr_err("failed to get highbank cpufreq device\n");
    return -ENODEV;
    }
    np = of_node_get(cpu_dev.of_node);
    if (!np) {
    pr_err("failed to find highbank cpufreq node\n");
    return -ENOENT;
    }
    cpu_clk = clk_get(cpu_dev, core::ptr::null_mut());
    if (IS_ERR(cpu_clk)) {
    ret = PTR_ERR(cpu_clk);
    pr_err("failed to get cpu0 clock: %d\n", ret);
    goto out_put_node;
    }
    ret = clk_notifier_register(cpu_clk, &hb_cpufreq_clk_nb);
    if (ret) {
    pr_err("failed to register clk notifier: %d\n", ret);
    goto out_put_node;
    }
// Instantiate cpufreq-dt
    platform_device_register_full(&devinfo);
    out_put_node:
    of_node_put(np);
    return ret;
    }
    module_init(hb_cpufreq_driver_init);
    static const struct of_device_id __maybe_unused hb_cpufreq_of_match[] = {
    { .compatible = "calxeda,highbank" },
    { .compatible = "calxeda,ecx-2000" },
    { },
    };
    MODULE_DEVICE_TABLE(of, hb_cpufreq_of_match);
    MODULE_AUTHOR("Mark Langsdorf <mark.langsdorf@calxeda.com>");
    MODULE_DESCRIPTION("Calxeda Highbank cpufreq driver");
    MODULE_LICENSE("GPL");
