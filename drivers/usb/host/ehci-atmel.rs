//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-atmel.c
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
// Driver for EHCI UHP on Atmel chips
//
// Copyright (C) 2009 Atmel Corporation,
// Nicolas Ferre <nicolas.ferre@atmel.com>
//
// Based on various ehci-*.c drivers
//

// interface and function clocks

    ((struct atmel_ehci_priv *)hcd_to_ehci(h).priv)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ehci_priv {
    pub iclk: *mut clk,
    pub uclk: *mut clk,
    pub clocked: bool,
}

    static struct hc_driver __read_mostly ehci_atmel_hc_driver;
    static const struct ehci_driver_overrides ehci_atmel_drv_overrides __initconst = {
    .extra_priv_size = sizeof(struct atmel_ehci_priv),
    };
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn atmel_start_clock(atmel_ehci: *mut atmel_ehci_priv) {
    static void atmel_start_clock(struct atmel_ehci_priv *atmel_ehci)
    {
    if (atmel_ehci.clocked)
    return;
    clk_prepare_enable(atmel_ehci.uclk);
    clk_prepare_enable(atmel_ehci.iclk);
    atmel_ehci.clocked = true;
    }
#[no_mangle]
unsafe extern "C" fn atmel_stop_clock(atmel_ehci: *mut atmel_ehci_priv) {
    static void atmel_stop_clock(struct atmel_ehci_priv *atmel_ehci)
    {
    if (!atmel_ehci.clocked)
    return;
    clk_disable_unprepare(atmel_ehci.iclk);
    clk_disable_unprepare(atmel_ehci.uclk);
    atmel_ehci.clocked = false;
    }
#[no_mangle]
unsafe extern "C" fn atmel_start_ehci(pdev: *mut platform_device) {
    static void atmel_start_ehci(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct atmel_ehci_priv *atmel_ehci = hcd_to_atmel_ehci_priv(hcd);
    dev_dbg(&pdev.dev, "start\n");
    atmel_start_clock(atmel_ehci);
    }
#[no_mangle]
unsafe extern "C" fn atmel_stop_ehci(pdev: *mut platform_device) {
    static void atmel_stop_ehci(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct atmel_ehci_priv *atmel_ehci = hcd_to_atmel_ehci_priv(hcd);
    dev_dbg(&pdev.dev, "stop\n");
    atmel_stop_clock(atmel_ehci);
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn ehci_atmel_drv_probe(pdev: *mut platform_device) -> c_int {
    static int ehci_atmel_drv_probe(struct platform_device *pdev)
    {
    struct usb_hcd *hcd;
    const struct hc_driver *driver = &ehci_atmel_hc_driver;
    struct resource *res;
    struct ehci_hcd *ehci;
    struct atmel_ehci_priv *atmel_ehci;
    int irq;
    int retval;
    if (usb_disabled())
    return -ENODEV;
    pr_debug("Initializing Atmel-SoC USB Host Controller\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    retval = irq;
    goto fail_create_hcd;
    }
// Right now device-tree probed devices don't get dma_mask set.
// Since shared usb code relies on it, set it here for now.
// Once we have dma capability bindings this can go away.
//
    retval = dma_coerce_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (retval)
    goto fail_create_hcd;
    hcd = usb_create_hcd(driver, &pdev.dev, dev_name(&pdev.dev));
    if (!hcd) {
    retval = -ENOMEM;
    goto fail_create_hcd;
    }
    atmel_ehci = hcd_to_atmel_ehci_priv(hcd);
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    retval = PTR_ERR(hcd.regs);
    goto fail_request_resource;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    atmel_ehci.iclk = devm_clk_get(&pdev.dev, "ehci_clk");
    if (IS_ERR(atmel_ehci.iclk)) {
    dev_err(&pdev.dev, "Error getting interface clock\n");
    retval = -ENOENT;
    goto fail_request_resource;
    }
    atmel_ehci.uclk = devm_clk_get(&pdev.dev, "usb_clk");
    if (IS_ERR(atmel_ehci.uclk)) {
    dev_err(&pdev.dev, "failed to get uclk\n");
    retval = PTR_ERR(atmel_ehci.uclk);
    goto fail_request_resource;
    }
    ehci = hcd_to_ehci(hcd);
// registers start at offset 0x0
    ehci.caps = hcd.regs;
    atmel_start_ehci(pdev);
    retval = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (retval)
    goto fail_add_hcd;
    device_wakeup_enable(hcd.self.controller);
    if (of_usb_get_phy_mode(pdev.dev.of_node) == USBPHY_INTERFACE_MODE_HSIC)
    writel(EHCI_INSNREG08_HSIC_EN, hcd.regs + EHCI_INSNREG(8));
    return retval;
    fail_add_hcd:
    atmel_stop_ehci(pdev);
    fail_request_resource:
    usb_put_hcd(hcd);
    fail_create_hcd:
    dev_err(&pdev.dev, "init %s fail, %d\n",
    dev_name(&pdev.dev), retval);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ehci_atmel_drv_remove(pdev: *mut platform_device) {
    static void ehci_atmel_drv_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    usb_remove_hcd(hcd);
    usb_put_hcd(hcd);
    atmel_stop_ehci(pdev);
    }
#[no_mangle]
unsafe extern "C" fn ehci_atmel_drv_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_atmel_drv_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct atmel_ehci_priv *atmel_ehci = hcd_to_atmel_ehci_priv(hcd);
    int ret;
    ret = ehci_suspend(hcd, false);
    if (ret)
    return ret;
    atmel_stop_clock(atmel_ehci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ehci_atmel_drv_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_atmel_drv_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct atmel_ehci_priv *atmel_ehci = hcd_to_atmel_ehci_priv(hcd);
    atmel_start_clock(atmel_ehci);
    ehci_resume(hcd, false);
    return 0;
    }

    static const struct of_device_id atmel_ehci_dt_ids[] = {
    { .compatible = "atmel,at91sam9g45-ehci" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_ehci_dt_ids);

    static SIMPLE_DEV_PM_OPS(ehci_atmel_pm_ops, ehci_atmel_drv_suspend,
    ehci_atmel_drv_resume);
    static struct platform_driver ehci_atmel_driver = {
    .probe		= ehci_atmel_drv_probe,
    .remove		= ehci_atmel_drv_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver		= {
    .name	= "atmel-ehci",
    .pm	= &ehci_atmel_pm_ops,
    .of_match_table	= of_match_ptr(atmel_ehci_dt_ids),
    },
    };
#[no_mangle]
unsafe extern "C" fn ehci_atmel_init() -> int __init {
    static int __init ehci_atmel_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&ehci_atmel_hc_driver, &ehci_atmel_drv_overrides);
    return platform_driver_register(&ehci_atmel_driver);
    }
    module_init(ehci_atmel_init);
#[no_mangle]
unsafe extern "C" fn ehci_atmel_cleanup() -> void __exit {
    static void __exit ehci_atmel_cleanup(void)
    {
    platform_driver_unregister(&ehci_atmel_driver);
    }
    module_exit(ehci_atmel_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_ALIAS("platform:atmel-ehci");
    MODULE_AUTHOR("Nicolas Ferre");
    MODULE_LICENSE("GPL");
