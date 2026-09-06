//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/bcm2835-pm.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// PM MFD driver for Broadcom BCM2835
//
// This driver binds to the PM block and creates the MFD device for
// the WDT and power drivers.
//

    static const struct mfd_cell bcm2835_pm_devs[] = {
    { .name = "bcm2835-wdt" },
    };
    static const struct mfd_cell bcm2835_power_devs[] = {
    { .name = "bcm2835-power" },
    };
    static int bcm2835_pm_get_pdata(struct platform_device *pdev,
    struct bcm2835_pm *pm)
    {
    if (of_property_present(pm.dev.of_node, "reg-names")) {
    struct resource *res;
    pm.base = devm_platform_ioremap_resource_byname(pdev, "pm");
    if (IS_ERR(pm.base))
    return PTR_ERR(pm.base);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "asb");
    if (res) {
    pm.asb = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(pm.asb))
    pm.asb = core::ptr::null_mut();
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    "rpivid_asb");
    if (res) {
    pm.rpivid_asb = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(pm.rpivid_asb))
    pm.rpivid_asb = core::ptr::null_mut();
    }
    return 0;
    }
// If no 'reg-names' property is found we can assume we're using old DTB.
    pm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pm.base))
    return PTR_ERR(pm.base);
    pm.asb = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(pm.asb))
    pm.asb = core::ptr::null_mut();
    pm.rpivid_asb = devm_platform_ioremap_resource(pdev, 2);
    if (IS_ERR(pm.rpivid_asb))
    pm.rpivid_asb = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pm_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_pm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm2835_pm *pm;
    int ret;
    pm = devm_kzalloc(dev, sizeof(*pm), GFP_KERNEL);
    if (!pm)
    return -ENOMEM;
    platform_set_drvdata(pdev, pm);
    pm.dev = dev;
    pm.soc = (uintptr_t)device_get_match_data(dev);
    ret = bcm2835_pm_get_pdata(pdev, pm);
    if (ret)
    return ret;
    ret = devm_mfd_add_devices(dev, -1,
    bcm2835_pm_devs, ARRAY_SIZE(bcm2835_pm_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return ret;
//
// We'll use the presence of the AXI ASB regs in the
// bcm2835-pm binding as the key for whether we can reference
// the full PM register range and support power domains.
//
    if (pm.asb || pm.soc == BCM2835_PM_SOC_BCM2712)
    return devm_mfd_add_devices(dev, -1, bcm2835_power_devs,
    ARRAY_SIZE(bcm2835_power_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    return 0;
    }
    static const struct of_device_id bcm2835_pm_of_match[] = {
    { .compatible = "brcm,bcm2835-pm-wdt", },
    { .compatible = "brcm,bcm2835-pm", .data = (void *)BCM2835_PM_SOC_BCM2835 },
    { .compatible = "brcm,bcm2711-pm", .data = (void *)BCM2835_PM_SOC_BCM2711 },
    { .compatible = "brcm,bcm2712-pm", .data = (void *)BCM2835_PM_SOC_BCM2712 },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_pm_of_match);
    static struct platform_driver bcm2835_pm_driver = {
    .probe		= bcm2835_pm_probe,
    .driver = {
    .name =	"bcm2835-pm",
    .of_match_table = bcm2835_pm_of_match,
    },
    };
    module_platform_driver(bcm2835_pm_driver);
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("Driver for Broadcom BCM2835 PM MFD");
