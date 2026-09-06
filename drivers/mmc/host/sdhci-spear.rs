//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-spear.c
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


//
// drivers/mmc/host/sdhci-spear.c
//
// Support of SDHCI platform devices for spear soc family
//
// Copyright (C) 2010 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// Inspired by sdhci-pltfm.c
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_sdhci {
    pub clk: *mut clk,
}

// sdhci ops
    static const struct sdhci_ops sdhci_pltfm_ops = {
    .set_clock = sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
#[no_mangle]
unsafe extern "C" fn sdhci_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_probe(struct platform_device *pdev)
    {
    struct sdhci_host *host;
    struct spear_sdhci *sdhci;
    struct device *dev;
    int ret;
    dev = pdev.dev.parent ? pdev.dev.parent : &pdev.dev;
    host = sdhci_alloc_host(dev, sizeof(*sdhci));
    if (IS_ERR(host)) {
    ret = PTR_ERR(host);
    dev_dbg(&pdev.dev, "cannot allocate memory for sdhci\n");
    goto err;
    }
    host.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(host.ioaddr)) {
    ret = PTR_ERR(host.ioaddr);
    dev_dbg(&pdev.dev, "unable to map iomem: %d\n", ret);
    goto err;
    }
    host.hw_name = "sdhci";
    host.ops = &sdhci_pltfm_ops;
    host.irq = platform_get_irq(pdev, 0);
    if (host.irq < 0) {
    ret = host.irq;
    goto err;
    }
    host.quirks = SDHCI_QUIRK_BROKEN_ADMA;
    sdhci = sdhci_priv(host);
// clk enable
    sdhci.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sdhci.clk)) {
    ret = PTR_ERR(sdhci.clk);
    dev_dbg(&pdev.dev, "Error getting clock\n");
    goto err;
    }
    ret = clk_prepare_enable(sdhci.clk);
    if (ret) {
    dev_dbg(&pdev.dev, "Error enabling clock\n");
    goto err;
    }
    ret = clk_set_rate(sdhci.clk, 50000000);
    if (ret)
    dev_dbg(&pdev.dev, "Error setting desired clk, clk=%lu\n",
    clk_get_rate(sdhci.clk));
//
// It is optional to use GPIOs for sdhci card detection. If we
// find a descriptor using slot GPIO, we use it.
//
    ret = mmc_gpiod_request_cd(host.mmc, "cd", 0, false, 0);
    if (ret == -EPROBE_DEFER)
    goto disable_clk;
    ret = sdhci_add_host(host);
    if (ret)
    goto disable_clk;
    platform_set_drvdata(pdev, host);
    return 0;
    disable_clk:
    clk_disable_unprepare(sdhci.clk);
    err:
    dev_err(&pdev.dev, "spear-sdhci probe failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_remove(pdev: *mut platform_device) {
    static void sdhci_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    struct spear_sdhci *sdhci = sdhci_priv(host);
    let mut dead: c_int = 0;
    u32 scratch;
    scratch = readl(host.ioaddr + SDHCI_INT_STATUS);
    if (scratch == (u32)-1)
    dead = 1;
    sdhci_remove_host(host, dead);
    clk_disable_unprepare(sdhci.clk);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_suspend(dev: *mut device) -> c_int {
    static int sdhci_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct spear_sdhci *sdhci = sdhci_priv(host);
    int ret;
    if (host.tuning_mode != SDHCI_TUNING_MODE_3)
    mmc_retune_needed(host.mmc);
    ret = sdhci_suspend_host(host);
    if (!ret)
    clk_disable(sdhci.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_resume(dev: *mut device) -> c_int {
    static int sdhci_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct spear_sdhci *sdhci = sdhci_priv(host);
    int ret;
    ret = clk_enable(sdhci.clk);
    if (ret) {
    dev_dbg(dev, "Resume: Error enabling clock\n");
    return ret;
    }
    return sdhci_resume_host(host);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sdhci_pm_ops, sdhci_suspend, sdhci_resume);
    static const struct of_device_id sdhci_spear_id_table[] = {
    { .compatible = "st,spear300-sdhci" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sdhci_spear_id_table);
    static struct platform_driver sdhci_driver = {
    .driver = {
    .name	= "sdhci",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm	= pm_sleep_ptr(&sdhci_pm_ops),
    .of_match_table = sdhci_spear_id_table,
    },
    .probe		= sdhci_probe,
    .remove		= sdhci_remove,
    };
    module_platform_driver(sdhci_driver);
    MODULE_DESCRIPTION("SPEAr Secure Digital Host Controller Interface driver");
    MODULE_AUTHOR("Viresh Kumar <vireshk@kernel.org>");
    MODULE_LICENSE("GPL v2");
