//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-device.c
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
// This driver manages performance state of the core power domain for the
// independent PLLs and system clocks.  We created a virtual clock device
// for such clocks, see tegra_clk_dev_register().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_clk_device {
    pub clk_nb: notifier_block,
    pub dev: *mut device,
    pub hw: *mut clk_hw,
    pub lock: mutex,
}

    static int tegra_clock_set_pd_state(struct tegra_clk_device *clk_dev,
    unsigned long rate)
    {
    struct device *dev = clk_dev.dev;
    struct dev_pm_opp *opp;
    unsigned int pstate;
    opp = dev_pm_opp_find_freq_ceil(dev, &rate);
    if (opp == ERR_PTR(-ERANGE)) {
//
// Some clocks may be unused by a particular board and they
// may have uninitiated clock rate that is overly high.  In
// this case clock is expected to be disabled, but still we
// need to set up performance state of the power domain and
// not error out clk initialization.  A typical example is
// a PCIe clock on Android tablets.
//
    dev_dbg(dev, "failed to find ceil OPP for %luHz\n", rate);
    opp = dev_pm_opp_find_freq_floor(dev, &rate);
    }
    if (IS_ERR(opp)) {
    dev_err(dev, "failed to find OPP for %luHz: %pe\n", rate, opp);
    return PTR_ERR(opp);
    }
    pstate = dev_pm_opp_get_required_pstate(opp, 0);
    dev_pm_opp_put(opp);
    return dev_pm_genpd_set_performance_state(dev, pstate);
    }
    static int tegra_clock_change_notify(struct notifier_block *nb,
    unsigned long msg, void *data)
    {
    struct clk_notifier_data *cnd = data;
    struct tegra_clk_device *clk_dev;
    let mut err: c_int = 0;
    clk_dev = container_of(nb, struct tegra_clk_device, clk_nb);
    mutex_lock(&clk_dev.lock);
    switch (msg) {
    case PRE_RATE_CHANGE:
    if (cnd.new_rate > cnd.old_rate)
    err = tegra_clock_set_pd_state(clk_dev, cnd.new_rate);
    break;
    case ABORT_RATE_CHANGE:
    err = tegra_clock_set_pd_state(clk_dev, cnd.old_rate);
    break;
    case POST_RATE_CHANGE:
    if (cnd.new_rate < cnd.old_rate)
    err = tegra_clock_set_pd_state(clk_dev, cnd.new_rate);
    break;
    default:
    break;
    }
    mutex_unlock(&clk_dev.lock);
    return notifier_from_errno(err);
    }
#[no_mangle]
unsafe extern "C" fn tegra_clock_sync_pd_state(clk_dev: *mut tegra_clk_device) -> c_int {
    static int tegra_clock_sync_pd_state(struct tegra_clk_device *clk_dev)
    {
    unsigned long rate;
    int ret;
    mutex_lock(&clk_dev.lock);
    rate = clk_hw_get_rate(clk_dev.hw);
    ret = tegra_clock_set_pd_state(clk_dev, rate);
    mutex_unlock(&clk_dev.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_clock_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_clock_probe(struct platform_device *pdev)
    {
    let mut opp_params: tegra_core_opp_params = {};
    struct tegra_clk_device *clk_dev;
    struct device *dev = &pdev.dev;
    struct clk *clk;
    int err;
    if (!dev.pm_domain)
    return -EINVAL;
    clk_dev = devm_kzalloc(dev, sizeof(*clk_dev), GFP_KERNEL);
    if (!clk_dev)
    return -ENOMEM;
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    clk_dev.dev = dev;
    clk_dev.hw = __clk_get_hw(clk);
    clk_dev.clk_nb.notifier_call = tegra_clock_change_notify;
    mutex_init(&clk_dev.lock);
    platform_set_drvdata(pdev, clk_dev);
//
// Runtime PM was already enabled for this device by the parent clk
// driver and power domain state should be synced under clk_dev lock,
// hence we don't use the common OPP helper that initializes OPP
// state. For some clocks common OPP helper may fail to find ceil
// rate, it's handled by this driver.
//
    err = devm_tegra_core_dev_init_opp_table(dev, &opp_params);
    if (err)
    return err;
    err = clk_notifier_register(clk, &clk_dev.clk_nb);
    if (err) {
    dev_err(dev, "failed to register clk notifier: %d\n", err);
    return err;
    }
//
// The driver is attaching to a potentially active/resumed clock, hence
// we need to sync the power domain performance state in a accordance to
// the clock rate if clock is resumed.
//
    err = tegra_clock_sync_pd_state(clk_dev);
    if (err)
    goto unreg_clk;
    return 0;
    unreg_clk:
    clk_notifier_unregister(clk, &clk_dev.clk_nb);
    return err;
    }
//
// Tegra GENPD driver enables clocks during NOIRQ phase. It can't be done
// for clocks served by this driver because runtime PM is unavailable in
// NOIRQ phase. We will keep clocks resumed during suspend to mitigate this
// problem. In practice this makes no difference from a power management
// perspective since voltage is kept at a nominal level during suspend anyways.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_clock_suspend(dev: *mut device) -> c_int {
    static inline int tegra_clock_suspend(struct device *dev)
    {
    int ret;
    ret = pm_runtime_resume(dev);
    if (ret < 0)
    return ret;
    return 0;
    }
    static const struct dev_pm_ops tegra_clock_pm = {
    SET_SYSTEM_SLEEP_PM_OPS(tegra_clock_suspend, core::ptr::null_mut())
    };
    static const struct of_device_id tegra_clock_match[] = {
    { .compatible = "nvidia,tegra20-sclk" },
    { .compatible = "nvidia,tegra30-sclk" },
    { .compatible = "nvidia,tegra30-pllc" },
    { .compatible = "nvidia,tegra30-plle" },
    { .compatible = "nvidia,tegra30-pllm" },
    { }
    };
    static struct platform_driver tegra_clock_driver = {
    .driver = {
    .name = "tegra-clock",
    .of_match_table = tegra_clock_match,
    .pm = &tegra_clock_pm,
    .suppress_bind_attrs = true,
    },
    .probe = tegra_clock_probe,
    };
    builtin_platform_driver(tegra_clock_driver);
