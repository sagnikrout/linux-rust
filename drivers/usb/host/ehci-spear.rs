//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-spear.c
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
// Driver for EHCI HCD on SPEAr SOC
//
// Copyright (C) 2010 ST Micro Electronics,
// Deepak Sikri <deepak.sikri@st.com>
//
// Based on various ehci-*.c drivers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_ehci {
    pub clk: *mut clk,
}

    static struct hc_driver __read_mostly ehci_spear_hc_driver;
#[no_mangle]
unsafe extern "C" fn ehci_spear_drv_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_spear_drv_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    let mut do_wakeup: bool = device_may_wakeup(dev);
    return ehci_suspend(hcd, do_wakeup);
    }
#[no_mangle]
unsafe extern "C" fn ehci_spear_drv_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ehci_spear_drv_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    ehci_resume(hcd, false);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(ehci_spear_pm_ops, ehci_spear_drv_suspend,
    ehci_spear_drv_resume);
#[no_mangle]
unsafe extern "C" fn spear_ehci_hcd_drv_probe(pdev: *mut platform_device) -> c_int {
    static int spear_ehci_hcd_drv_probe(struct platform_device *pdev)
    {
    struct usb_hcd *hcd ;
    struct spear_ehci *sehci;
    struct resource *res;
    struct clk *usbh_clk;
    const struct hc_driver *driver = &ehci_spear_hc_driver;
    int irq, retval;
    if (usb_disabled())
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    retval = irq;
    goto fail;
    }
//
// Right now device-tree probed devices don't get dma_mask set.
// Since shared usb code relies on it, set it here for now.
// Once we have dma capability bindings this can go away.
//
    retval = dma_coerce_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (retval)
    goto fail;
    usbh_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usbh_clk)) {
    dev_err(&pdev.dev, "Error getting interface clock\n");
    retval = PTR_ERR(usbh_clk);
    goto fail;
    }
    hcd = usb_create_hcd(driver, &pdev.dev, dev_name(&pdev.dev));
    if (!hcd) {
    retval = -ENOMEM;
    goto fail;
    }
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    retval = PTR_ERR(hcd.regs);
    goto err_put_hcd;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    sehci = to_spear_ehci(hcd);
    sehci.clk = usbh_clk;
// registers start at offset 0x0
    hcd_to_ehci(hcd).caps = hcd.regs;
    retval = clk_prepare_enable(sehci.clk);
    if (retval)
    goto err_put_hcd;
    retval = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (retval)
    goto err_stop_ehci;
    device_wakeup_enable(hcd.self.controller);
    return retval;
    err_stop_ehci:
    clk_disable_unprepare(sehci.clk);
    err_put_hcd:
    usb_put_hcd(hcd);
    fail:
    dev_err(&pdev.dev, "init fail, %d\n", retval);
    return retval ;
    }
#[no_mangle]
unsafe extern "C" fn spear_ehci_hcd_drv_remove(pdev: *mut platform_device) {
    static void spear_ehci_hcd_drv_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct spear_ehci *sehci = to_spear_ehci(hcd);
    usb_remove_hcd(hcd);
    clk_disable_unprepare(sehci.clk);
    usb_put_hcd(hcd);
    }
    static const struct of_device_id spear_ehci_id_table[] = {
    { .compatible = "st,spear600-ehci", },
    { },
    };
    MODULE_DEVICE_TABLE(of, spear_ehci_id_table);
    static struct platform_driver spear_ehci_hcd_driver = {
    .probe		= spear_ehci_hcd_drv_probe,
    .remove		= spear_ehci_hcd_drv_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver		= {
    .name = "spear-ehci",
    .bus = &platform_bus_type,
    .pm = pm_ptr(&ehci_spear_pm_ops),
    .of_match_table = spear_ehci_id_table,
    }
    };
    static const struct ehci_driver_overrides spear_overrides __initconst = {
    .extra_priv_size = sizeof(struct spear_ehci),
    };
#[no_mangle]
unsafe extern "C" fn ehci_spear_init() -> int __init {
    static int __init ehci_spear_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&ehci_spear_hc_driver, &spear_overrides);
    return platform_driver_register(&spear_ehci_hcd_driver);
    }
    module_init(ehci_spear_init);
#[no_mangle]
unsafe extern "C" fn ehci_spear_cleanup() -> void __exit {
    static void __exit ehci_spear_cleanup(void)
    {
    platform_driver_unregister(&spear_ehci_hcd_driver);
    }
    module_exit(ehci_spear_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_ALIAS("platform:spear-ehci");
    MODULE_AUTHOR("Deepak Sikri");
    MODULE_LICENSE("GPL");
