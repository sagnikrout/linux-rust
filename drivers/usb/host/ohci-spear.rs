//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-spear.c
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
// OHCI HCD (Host Controller Driver) for USB.
//
// Copyright (C) 2010 ST Microelectronics.
// Deepak Sikri<deepak.sikri@st.com>
//
// Based on various ohci-*.c drivers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_ohci {
    pub clk: *mut clk,
}

    static struct hc_driver __read_mostly ohci_spear_hc_driver;
#[no_mangle]
unsafe extern "C" fn spear_ohci_hcd_drv_probe(pdev: *mut platform_device) -> c_int {
    static int spear_ohci_hcd_drv_probe(struct platform_device *pdev)
    {
    const struct hc_driver *driver = &ohci_spear_hc_driver;
    struct usb_hcd *hcd = core::ptr::null_mut();
    struct clk *usbh_clk;
    struct spear_ohci *sohci_p;
    struct resource *res;
    int retval, irq;
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
    sohci_p = to_spear_ohci(hcd);
    sohci_p.clk = usbh_clk;
    clk_prepare_enable(sohci_p.clk);
    retval = usb_add_hcd(hcd, irq, 0);
    if (retval == 0) {
    device_wakeup_enable(hcd.self.controller);
    return retval;
    }
    clk_disable_unprepare(sohci_p.clk);
    err_put_hcd:
    usb_put_hcd(hcd);
    fail:
    dev_err(&pdev.dev, "init fail, %d\n", retval);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn spear_ohci_hcd_drv_remove(pdev: *mut platform_device) {
    static void spear_ohci_hcd_drv_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct spear_ohci *sohci_p = to_spear_ohci(hcd);
    usb_remove_hcd(hcd);
    clk_disable_unprepare(sohci_p.clk);
    usb_put_hcd(hcd);
    }

    static int spear_ohci_hcd_drv_suspend(struct platform_device *pdev,
    pm_message_t message)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct ohci_hcd	*ohci = hcd_to_ohci(hcd);
    struct spear_ohci *sohci_p = to_spear_ohci(hcd);
    let mut do_wakeup: bool = device_may_wakeup(&pdev.dev);
    int ret;
    if (time_before(jiffies, ohci.next_statechange))
    msleep(5);
    ohci.next_statechange = jiffies;
    ret = ohci_suspend(hcd, do_wakeup);
    if (ret)
    return ret;
    clk_disable_unprepare(sohci_p.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spear_ohci_hcd_drv_resume(dev: *mut platform_device) -> c_int {
    static int spear_ohci_hcd_drv_resume(struct platform_device *dev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(dev);
    struct ohci_hcd	*ohci = hcd_to_ohci(hcd);
    struct spear_ohci *sohci_p = to_spear_ohci(hcd);
    if (time_before(jiffies, ohci.next_statechange))
    msleep(5);
    ohci.next_statechange = jiffies;
    clk_prepare_enable(sohci_p.clk);
    ohci_resume(hcd, false);
    return 0;
    }

    static const struct of_device_id spear_ohci_id_table[] = {
    { .compatible = "st,spear600-ohci", },
    { },
    };
    MODULE_DEVICE_TABLE(of, spear_ohci_id_table);
// Driver definition to register with the platform bus
    static struct platform_driver spear_ohci_hcd_driver = {
    .probe =	spear_ohci_hcd_drv_probe,
    .remove =	spear_ohci_hcd_drv_remove,

    .suspend =	spear_ohci_hcd_drv_suspend,
    .resume =	spear_ohci_hcd_drv_resume,

    .driver = {
    .name = "spear-ohci",
    .of_match_table = spear_ohci_id_table,
    },
    };
    static const struct ohci_driver_overrides spear_overrides __initconst = {
    .extra_priv_size = sizeof(struct spear_ohci),
    };
#[no_mangle]
unsafe extern "C" fn ohci_spear_init() -> int __init {
    static int __init ohci_spear_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ohci_init_driver(&ohci_spear_hc_driver, &spear_overrides);
    return platform_driver_register(&spear_ohci_hcd_driver);
    }
    module_init(ohci_spear_init);
#[no_mangle]
unsafe extern "C" fn ohci_spear_cleanup() -> void __exit {
    static void __exit ohci_spear_cleanup(void)
    {
    platform_driver_unregister(&spear_ohci_hcd_driver);
    }
    module_exit(ohci_spear_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Deepak Sikri");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:spear-ohci");
