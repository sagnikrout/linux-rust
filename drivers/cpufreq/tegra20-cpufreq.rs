//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/tegra20-cpufreq.c
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
// Copyright (C) 2010 Google, Inc.
//
// Author:
// Colin Cross <ccross@google.com>
// Based on arch/arm/plat-omap/cpu-omap.c, (C) 2005 Nokia Corporation
//

#[no_mangle]
unsafe extern "C" fn cpu0_node_has_opp_v2_prop() -> bool {
    static bool cpu0_node_has_opp_v2_prop(void)
    {
    struct device_node *np = of_cpu_device_node_get(0);
    let mut ret: bool = false;
    if (of_property_present(np, "operating-points-v2"))
    ret = true;
    of_node_put(np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpufreq_put_supported_hw(opp_token: *mut c_void) {
    static void tegra20_cpufreq_put_supported_hw(void *opp_token)
    {
    dev_pm_opp_put_supported_hw((unsigned long) opp_token);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpufreq_dt_unregister(cpufreq_dt: *mut c_void) {
    static void tegra20_cpufreq_dt_unregister(void *cpufreq_dt)
    {
    platform_device_unregister(cpufreq_dt);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int tegra20_cpufreq_probe(struct platform_device *pdev)
    {
    struct platform_device *cpufreq_dt;
    struct device *cpu_dev;
    u32 versions[2];
    int err;
    if (!cpu0_node_has_opp_v2_prop()) {
    dev_err(&pdev.dev, "operating points not found\n");
    dev_err(&pdev.dev, "please update your device tree\n");
    return -ENODEV;
    }
    if (of_machine_is_compatible("nvidia,tegra20")) {
    versions[0] = BIT(tegra_sku_info.cpu_process_id);
    versions[1] = BIT(tegra_sku_info.soc_speedo_id);
    } else {
    versions[0] = BIT(tegra_sku_info.cpu_process_id);
    versions[1] = BIT(tegra_sku_info.cpu_speedo_id);
    }
    dev_info(&pdev.dev, "hardware version 0x%x 0x%x\n",
    versions[0], versions[1]);
    cpu_dev = get_cpu_device(0);
    if (WARN_ON(!cpu_dev))
    return -ENODEV;
    err = dev_pm_opp_set_supported_hw(cpu_dev, versions, 2);
    if (err < 0) {
    dev_err(&pdev.dev, "failed to set supported hw: %d\n", err);
    return err;
    }
    err = devm_add_action_or_reset(&pdev.dev,
    tegra20_cpufreq_put_supported_hw,
    (void *)((unsigned long) err));
    if (err)
    return err;
    cpufreq_dt = platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    err = PTR_ERR_OR_ZERO(cpufreq_dt);
    if (err) {
    dev_err(&pdev.dev,
    "failed to create cpufreq-dt device: %d\n", err);
    return err;
    }
    err = devm_add_action_or_reset(&pdev.dev,
    tegra20_cpufreq_dt_unregister,
    cpufreq_dt);
    if (err)
    return err;
    return 0;
    }
    static struct platform_driver tegra20_cpufreq_driver = {
    .probe		= tegra20_cpufreq_probe,
    .driver		= {
    .name	= "tegra20-cpufreq",
    },
    };
    module_platform_driver(tegra20_cpufreq_driver);
    MODULE_ALIAS("platform:tegra20-cpufreq");
    MODULE_AUTHOR("Colin Cross <ccross@android.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra20 cpufreq driver");
    MODULE_LICENSE("GPL");
