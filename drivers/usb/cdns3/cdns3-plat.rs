//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdns3-plat.c
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
// Cadence USBSS DRD Driver.
//
// Copyright (C) 2018-2020 Cadence.
// Copyright (C) 2017-2018 NXP
// Copyright (C) 2019 Texas Instruments
//
// Author: Peter Chen <peter.chen@nxp.com>
// Pawel Laszczak <pawell@cadence.com>
// Roger Quadros <rogerq@ti.com>
//

#[no_mangle]
unsafe extern "C" fn set_phy_power_on(cdns: *mut cdns) -> c_int {
    static int set_phy_power_on(struct cdns *cdns)
    {
    int ret;
    ret = phy_power_on(cdns.usb2_phy);
    if (ret)
    return ret;
    ret = phy_power_on(cdns.usb3_phy);
    if (ret)
    phy_power_off(cdns.usb2_phy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_phy_power_off(cdns: *mut cdns) {
    static void set_phy_power_off(struct cdns *cdns)
    {
    phy_power_off(cdns.usb3_phy);
    phy_power_off(cdns.usb2_phy);
    }
#[no_mangle]
unsafe extern "C" fn cdns3_plat_gadget_init(cdns: *mut cdns) -> c_int {
    static int cdns3_plat_gadget_init(struct cdns *cdns)
    {
    if (cdns.version < CDNSP_CONTROLLER_V2)
    return cdns3_gadget_init(cdns);
    else
    return cdnsp_gadget_init(cdns);
    }
#[no_mangle]
unsafe extern "C" fn cdns3_plat_host_init(cdns: *mut cdns) -> c_int {
    static int cdns3_plat_host_init(struct cdns *cdns)
    {
    return cdns_host_init(cdns);
    }
//
// cdns3_plat_probe - probe for cdns3 core device
// @pdev: Pointer to cdns3 core platform device
//
// Returns 0 on success otherwise negative errno
//
#[no_mangle]
unsafe extern "C" fn cdns3_plat_probe(pdev: *mut platform_device) -> c_int {
    static int cdns3_plat_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource	*res;
    struct cdns *cdns;
    void __iomem *regs;
    int ret;
    cdns = devm_kzalloc(dev, sizeof(*cdns), GFP_KERNEL);
    if (!cdns)
    return -ENOMEM;
    cdns.dev = dev;
    cdns.pdata = dev_get_platdata(dev);
    if (cdns.pdata && cdns.pdata.override_apb_timeout)
    cdns.override_apb_timeout = cdns.pdata.override_apb_timeout;
    if (device_is_compatible(dev, "cdns,cdnsp")) {
    cdns.no_drd = true;
    cdns.version = CDNSP_CONTROLLER_V2;
    dev_dbg(dev, "No DRD support\n");
    }
    platform_set_drvdata(pdev, cdns);
    ret = platform_get_irq_byname(pdev, "host");
    if (ret < 0)
    return ret;
    cdns.xhci_res[0].start = ret;
    cdns.xhci_res[0].end = ret;
    cdns.xhci_res[0].flags = IORESOURCE_IRQ | irq_get_trigger_type(ret);
    cdns.xhci_res[0].name = "host";
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "xhci");
    if (!res) {
    dev_err(dev, "couldn't get xhci resource\n");
    return -ENXIO;
    }
    cdns.xhci_res[1] = *res;
    cdns.dev_irq = platform_get_irq_byname(pdev, "peripheral");
    if (cdns.dev_irq < 0)
    return dev_err_probe(dev, cdns.dev_irq,
    "Failed to get peripheral IRQ\n");
    regs = devm_platform_ioremap_resource_byname(pdev, "dev");
    if (IS_ERR(regs))
    return dev_err_probe(dev, PTR_ERR(regs),
    "Failed to get dev base\n");
    cdns.dev_regs	= regs;
    if (!cdns.no_drd) {
    cdns.otg_irq = platform_get_irq_byname(pdev, "otg");
    if (cdns.otg_irq < 0)
    return dev_err_probe(dev, cdns.otg_irq,
    "Failed to get otg IRQ\n");
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "otg");
    if (!res) {
    dev_err(dev, "couldn't get otg resource\n");
    return -ENXIO;
    }
    cdns.otg_res = *res;
    }
    cdns.phyrst_a_enable = device_property_read_bool(dev, "cdns,phyrst-a-enable");
    cdns.wakeup_irq = platform_get_irq_byname_optional(pdev, "wakeup");
    if (cdns.wakeup_irq == -EPROBE_DEFER)
    return cdns.wakeup_irq;
    if (cdns.wakeup_irq < 0) {
    dev_dbg(dev, "couldn't get wakeup irq\n");
    cdns.wakeup_irq = 0x0;
    }
    cdns.usb2_phy = devm_phy_optional_get(dev, "cdns3,usb2-phy");
    if (IS_ERR(cdns.usb2_phy))
    return dev_err_probe(dev, PTR_ERR(cdns.usb2_phy),
    "Failed to get cdns3,usb2-phy\n");
    cdns.usb3_phy = devm_phy_optional_get(dev, "cdns3,usb3-phy");
    if (IS_ERR(cdns.usb3_phy))
    return dev_err_probe(dev, PTR_ERR(cdns.usb3_phy),
    "Failed to get cdns3,usb3-phy\n");
    ret = phy_init(cdns.usb2_phy);
    if (ret)
    return ret;
    ret = phy_init(cdns.usb3_phy);
    if (ret)
    goto err_phy3_init;
    ret = set_phy_power_on(cdns);
    if (ret)
    goto err_phy_power_on;
    ret = cdns_init(cdns);
    if (ret)
    goto err_cdns_init;
    cdns.gadget_init = cdns3_plat_gadget_init;
    cdns.host_init = cdns3_plat_host_init;
    ret = cdns_core_init_role(cdns);
    if (ret)
    goto err_cdns_init_role;
    device_set_wakeup_capable(dev, true);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    if (!(cdns.pdata && (cdns.pdata.quirks & CDNS3_DEFAULT_PM_RUNTIME_ALLOW)))
    pm_runtime_forbid(dev);
//
// The controller needs less time between bus and controller suspend,
// and we also needs a small delay to avoid frequently entering low
// power mode.
//
    pm_runtime_set_autosuspend_delay(dev, 20);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_use_autosuspend(dev);
    return 0;
    err_cdns_init_role:
    if (cdns.role_sw)
    usb_role_switch_unregister(cdns.role_sw);
    err_cdns_init:
    set_phy_power_off(cdns);
    err_phy_power_on:
    phy_exit(cdns.usb3_phy);
    err_phy3_init:
    phy_exit(cdns.usb2_phy);
    return ret;
    }
//
// cdns3_plat_remove() - unbind drd driver and clean up
// @pdev: Pointer to Linux platform device
//
#[no_mangle]
unsafe extern "C" fn cdns3_plat_remove(pdev: *mut platform_device) {
    static void cdns3_plat_remove(struct platform_device *pdev)
    {
    struct cdns *cdns = platform_get_drvdata(pdev);
    struct device *dev = cdns.dev;
    pm_runtime_get_sync(dev);
    if (!(cdns.pdata && (cdns.pdata.quirks & CDNS3_DEFAULT_PM_RUNTIME_ALLOW)))
    pm_runtime_allow(dev);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    cdns_remove(cdns);
    set_phy_power_off(cdns);
    phy_exit(cdns.usb2_phy);
    phy_exit(cdns.usb3_phy);
    }

    static int cdns3_set_platform_suspend(struct device *dev,
    bool suspend, bool wakeup)
    {
    struct cdns *cdns = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (cdns.pdata && cdns.pdata.platform_suspend)
    ret = cdns.pdata.platform_suspend(dev, suspend, wakeup);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns3_controller_suspend(dev: *mut device, msg: pm_message_t) -> c_int {
    static int cdns3_controller_suspend(struct device *dev, pm_message_t msg)
    {
    struct cdns *cdns = dev_get_drvdata(dev);
    bool wakeup;
    unsigned long flags;
    if (cdns.in_lpm)
    return 0;
    if (PMSG_IS_AUTO(msg))
    wakeup = true;
    else
    wakeup = device_may_wakeup(dev);
    cdns3_set_platform_suspend(cdns.dev, true, wakeup);
    set_phy_power_off(cdns);
    spin_lock_irqsave(&cdns.lock, flags);
    cdns.in_lpm = true;
    spin_unlock_irqrestore(&cdns.lock, flags);
    dev_dbg(cdns.dev, "%s ends\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns3_controller_resume(dev: *mut device, msg: pm_message_t) -> c_int {
    static int cdns3_controller_resume(struct device *dev, pm_message_t msg)
    {
    struct cdns *cdns = dev_get_drvdata(dev);
    int ret;
    unsigned long flags;
    if (!cdns.in_lpm)
    return 0;
    if (cdns_power_is_lost(cdns)) {
    phy_exit(cdns.usb2_phy);
    ret = phy_init(cdns.usb2_phy);
    if (ret)
    return ret;
    phy_exit(cdns.usb3_phy);
    ret = phy_init(cdns.usb3_phy);
    if (ret)
    return ret;
    }
    ret = set_phy_power_on(cdns);
    if (ret)
    return ret;
    cdns3_set_platform_suspend(cdns.dev, false, false);
    spin_lock_irqsave(&cdns.lock, flags);
    cdns_resume(cdns);
    cdns.in_lpm = false;
    spin_unlock_irqrestore(&cdns.lock, flags);
    cdns_set_active(cdns, !PMSG_IS_AUTO(msg));
    if (cdns.wakeup_pending) {
    cdns.wakeup_pending = false;
    enable_irq(cdns.wakeup_irq);
    }
    dev_dbg(cdns.dev, "%s ends\n", __func__);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns3_plat_runtime_suspend(dev: *mut device) -> c_int {
    static int cdns3_plat_runtime_suspend(struct device *dev)
    {
    return cdns3_controller_suspend(dev, PMSG_AUTO_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn cdns3_plat_runtime_resume(dev: *mut device) -> c_int {
    static int cdns3_plat_runtime_resume(struct device *dev)
    {
    return cdns3_controller_resume(dev, PMSG_AUTO_RESUME);
    }

#[no_mangle]
unsafe extern "C" fn cdns3_plat_suspend(dev: *mut device) -> c_int {
    static int cdns3_plat_suspend(struct device *dev)
    {
    struct cdns *cdns = dev_get_drvdata(dev);
    int ret;
    cdns_suspend(cdns);
    ret = cdns3_controller_suspend(dev, PMSG_SUSPEND);
    if (ret)
    return ret;
    if (device_may_wakeup(dev) && cdns.wakeup_irq)
    enable_irq_wake(cdns.wakeup_irq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns3_plat_resume(dev: *mut device) -> c_int {
    static int cdns3_plat_resume(struct device *dev)
    {
    return cdns3_controller_resume(dev, PMSG_RESUME);
    }

    static const struct dev_pm_ops cdns3_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(cdns3_plat_suspend, cdns3_plat_resume)
    SET_RUNTIME_PM_OPS(cdns3_plat_runtime_suspend,
    cdns3_plat_runtime_resume, core::ptr::null_mut())
    };

    static const struct of_device_id of_cdns3_match[] = {
    { .compatible = "cdns,usb3" },
    { .compatible = "cdns,cdnsp" },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_cdns3_match);

    static struct platform_driver cdns3_driver = {
    .probe		= cdns3_plat_probe,
    .remove		= cdns3_plat_remove,
    .driver		= {
    .name	= "cdns-usb3",
    .of_match_table	= of_match_ptr(of_cdns3_match),
    .pm	= &cdns3_pm_ops,
    },
    };
    module_platform_driver(cdns3_driver);
    MODULE_ALIAS("platform:cdns3");
