//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/common.c
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
// Copyright (C) 2014 NVIDIA CORPORATION.  All rights reserved.
//

    static const struct of_device_id tegra_machine_match[] = {
    { .compatible = "nvidia,tegra20", },
    { .compatible = "nvidia,tegra30", },
    { .compatible = "nvidia,tegra114", },
    { .compatible = "nvidia,tegra124", },
    { .compatible = "nvidia,tegra132", },
    { .compatible = "nvidia,tegra210", },
    { }
    };
#[no_mangle]
pub unsafe extern "C" fn soc_is_tegra() -> bool {
    bool soc_is_tegra(void)
    {
    return of_machine_device_match(tegra_machine_match);
    }
#[no_mangle]
unsafe extern "C" fn tegra_core_dev_init_opp_state(dev: *mut device) -> c_int {
    static int tegra_core_dev_init_opp_state(struct device *dev)
    {
    unsigned long rate;
    struct clk *clk;
    bool rpm_enabled;
    int err;
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(dev, "failed to get clk: %pe\n", clk);
    return PTR_ERR(clk);
    }
    rate = clk_get_rate(clk);
    if (!rate) {
    dev_err(dev, "failed to get clk rate\n");
    return -EINVAL;
    }
//
// Runtime PM of the device must be enabled in order to set up
// GENPD's performance properly because GENPD core checks whether
// device is suspended and this check doesn't work while RPM is
// disabled. This makes sure the OPP vote below gets cached in
// GENPD for the device. Instead, the vote is done the next time
// the device gets runtime resumed.
//
    rpm_enabled = pm_runtime_enabled(dev);
    if (!rpm_enabled)
    pm_runtime_enable(dev);
// should never happen in practice
    if (!pm_runtime_enabled(dev)) {
    dev_WARN(dev, "failed to enable runtime PM\n");
    pm_runtime_disable(dev);
    return -EINVAL;
    }
// first dummy rate-setting initializes voltage vote
    err = dev_pm_opp_set_rate(dev, rate);
    if (!rpm_enabled)
    pm_runtime_disable(dev);
    if (err) {
    dev_err(dev, "failed to initialize OPP clock: %d\n", err);
    return err;
    }
    return 0;
    }
//
// devm_tegra_core_dev_init_opp_table() - initialize OPP table
// @dev: device for which OPP table is initialized
// @params: pointer to the OPP table configuration
//
// This function will initialize OPP table and sync OPP state of a Tegra SoC
// core device.
//
// Return: 0 on success or errorno.
//
    int devm_tegra_core_dev_init_opp_table(struct device *dev,
    struct tegra_core_opp_params *params)
    {
    u32 hw_version;
    int err;
//
// The clk's connection id to set is NULL and this is a NULL terminated
// array, hence two NULL entries.
//
    const char *clk_names[] = { core::ptr::null_mut(), core::ptr::null_mut() };
    struct dev_pm_opp_config config = {
//
// For some devices we don't have any OPP table in the DT, and
// in order to use the same code path for all the devices, we
// create a dummy OPP table for them via this. The dummy OPP
// table is only capable of doing clk_set_rate() on invocation
// of dev_pm_opp_set_rate() and doesn't provide any other
// functionality.
//
    .clk_names = clk_names,
    };
    if (of_machine_is_compatible("nvidia,tegra20")) {
    hw_version = BIT(tegra_sku_info.soc_process_id);
    config.supported_hw = &hw_version;
    config.supported_hw_count = 1;
    } else if (of_machine_is_compatible("nvidia,tegra30") ||
    of_machine_is_compatible("nvidia,tegra114")) {
    hw_version = BIT(tegra_sku_info.soc_speedo_id);
    config.supported_hw = &hw_version;
    config.supported_hw_count = 1;
    }
    err = devm_pm_opp_set_config(dev, &config);
    if (err) {
    dev_err(dev, "failed to set OPP config: %d\n", err);
    return err;
    }
//
// Tegra124+ doesn't support OPP yet, return early for pre-Tegra124
// case.
//
    if (!config.supported_hw)
    return -ENODEV;
//
// Older device-trees have an empty OPP table, we will get
// -ENODEV from devm_pm_opp_of_add_table() in this case.
//
    err = devm_pm_opp_of_add_table(dev);
    if (err) {
    if (err != -ENODEV)
    dev_err(dev, "failed to add OPP table: %d\n", err);
    return err;
    }
    if (params.init_state) {
    err = tegra_core_dev_init_opp_state(dev);
    if (err)
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_tegra_core_dev_init_opp_table);
