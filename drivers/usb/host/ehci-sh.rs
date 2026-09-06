//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-sh.c
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
// SuperH EHCI host controller driver
//
// Copyright (C) 2010  Paul Mundt
//
// Based on ohci-sh.c and ehci-atmel.c.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_sh_priv {
    pub fclk: *mut *mut clk iclk,,
    pub hcd: *mut usb_hcd,
}

#[no_mangle]
unsafe extern "C" fn ehci_sh_reset(hcd: *mut usb_hcd) -> c_int {
    static int ehci_sh_reset(struct usb_hcd *hcd)
    {
    struct ehci_hcd	*ehci = hcd_to_ehci(hcd);
    ehci.caps = hcd.regs;
    return ehci_setup(hcd);
    }
    static const struct hc_driver ehci_sh_hc_driver = {
    .description			= hcd_name,
    .product_desc			= "SuperH EHCI",
    .hcd_priv_size			= sizeof(struct ehci_hcd),
//
// generic hardware linkage
//
    .irq				= ehci_irq,
    .flags				= HCD_USB2 | HCD_DMA | HCD_MEMORY | HCD_BH,
//
// basic lifecycle operations
//
    .reset				= ehci_sh_reset,
    .start				= ehci_run,
    .stop				= ehci_stop,
    .shutdown			= ehci_shutdown,
//
// managing i/o requests and associated device resources
//
    .urb_enqueue			= ehci_urb_enqueue,
    .urb_dequeue			= ehci_urb_dequeue,
    .endpoint_disable		= ehci_endpoint_disable,
    .endpoint_reset			= ehci_endpoint_reset,
//
// scheduling support
//
    .get_frame_number		= ehci_get_frame,
//
// root hub support
//
    .hub_status_data		= ehci_hub_status_data,
    .hub_control			= ehci_hub_control,

    .bus_suspend			= ehci_bus_suspend,
    .bus_resume			= ehci_bus_resume,

    .relinquish_port		= ehci_relinquish_port,
    .port_handed_over		= ehci_port_handed_over,
    .clear_tt_buffer_complete	= ehci_clear_tt_buffer_complete,
    };
#[no_mangle]
unsafe extern "C" fn ehci_hcd_sh_probe(pdev: *mut platform_device) -> c_int {
    static int ehci_hcd_sh_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct ehci_sh_priv *priv;
    struct usb_hcd *hcd;
    int irq, ret;
    if (usb_disabled())
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto fail_create_hcd;
    }
// initialize hcd
    hcd = usb_create_hcd(&ehci_sh_hc_driver, &pdev.dev,
    dev_name(&pdev.dev));
    if (!hcd) {
    ret = -ENOMEM;
    goto fail_create_hcd;
    }
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    ret = PTR_ERR(hcd.regs);
    goto fail_request_resource;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    priv = devm_kzalloc(&pdev.dev, sizeof(struct ehci_sh_priv),
    GFP_KERNEL);
    if (!priv) {
    ret = -ENOMEM;
    goto fail_request_resource;
    }
// These are optional, we don't care if they fail
    priv.fclk = devm_clk_get(&pdev.dev, "usb_fck");
    if (IS_ERR(priv.fclk))
    priv.fclk = core::ptr::null_mut();
    priv.iclk = devm_clk_get(&pdev.dev, "usb_ick");
    if (IS_ERR(priv.iclk))
    priv.iclk = core::ptr::null_mut();
    ret = clk_enable(priv.fclk);
    if (ret)
    goto fail_request_resource;
    ret = clk_enable(priv.iclk);
    if (ret)
    goto fail_iclk;
    ret = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (ret != 0) {
    dev_err(&pdev.dev, "Failed to add hcd");
    goto fail_add_hcd;
    }
    device_wakeup_enable(hcd.self.controller);
    priv.hcd = hcd;
    platform_set_drvdata(pdev, priv);
    return ret;
    fail_add_hcd:
    clk_disable(priv.iclk);
    fail_iclk:
    clk_disable(priv.fclk);
    fail_request_resource:
    usb_put_hcd(hcd);
    fail_create_hcd:
    dev_err(&pdev.dev, "init %s fail, %d\n", dev_name(&pdev.dev), ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ehci_hcd_sh_remove(pdev: *mut platform_device) {
    static void ehci_hcd_sh_remove(struct platform_device *pdev)
    {
    struct ehci_sh_priv *priv = platform_get_drvdata(pdev);
    struct usb_hcd *hcd = priv.hcd;
    usb_remove_hcd(hcd);
    usb_put_hcd(hcd);
    clk_disable(priv.fclk);
    clk_disable(priv.iclk);
    }
#[no_mangle]
unsafe extern "C" fn ehci_hcd_sh_shutdown(pdev: *mut platform_device) {
    static void ehci_hcd_sh_shutdown(struct platform_device *pdev)
    {
    struct ehci_sh_priv *priv = platform_get_drvdata(pdev);
    struct usb_hcd *hcd = priv.hcd;
    if (hcd.driver.shutdown)
    hcd.driver.shutdown(hcd);
    }
    static struct platform_driver ehci_hcd_sh_driver = {
    .probe		= ehci_hcd_sh_probe,
    .remove		= ehci_hcd_sh_remove,
    .shutdown	= ehci_hcd_sh_shutdown,
    .driver		= {
    .name	= "sh_ehci",
    },
    };
    MODULE_ALIAS("platform:sh_ehci");
