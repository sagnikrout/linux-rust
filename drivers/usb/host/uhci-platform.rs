//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/uhci-platform.c
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
// Generic UHCI HCD (Host Controller Driver) for Platform Devices
//
// Copyright (c) 2011 Tony Prisk <linux@prisktech.co.nz>
//
// This file is based on uhci-grlib.c
// (C) Copyright 2004-2007 Alan Stern, stern@rowland.harvard.edu
//

#[no_mangle]
unsafe extern "C" fn uhci_platform_init(hcd: *mut usb_hcd) -> c_int {
    static int uhci_platform_init(struct usb_hcd *hcd)
    {
    struct uhci_hcd *uhci = hcd_to_uhci(hcd);
// Probe number of ports if not already provided by DT
    if (!uhci.rh_numports)
    uhci.rh_numports = uhci_count_ports(hcd);
// Set up pointers to to generic functions
    uhci.reset_hc = uhci_generic_reset_hc;
    uhci.check_and_reset_hc = uhci_generic_check_and_reset_hc;
// No special actions need to be taken for the functions below
    uhci.configure_hc = core::ptr::null_mut();
    uhci.resume_detect_interrupts_are_broken = core::ptr::null_mut();
    uhci.global_suspend_mode_is_broken = core::ptr::null_mut();
// Reset if the controller isn't already safely quiescent.
    check_and_reset_hc(uhci);
    return 0;
    }
    static const struct hc_driver uhci_platform_hc_driver = {
    .description =		hcd_name,
    .product_desc =		"Generic UHCI Host Controller",
    .hcd_priv_size =	sizeof(struct uhci_hcd),
// Generic hardware linkage
    .irq =			uhci_irq,
    .flags =		HCD_MEMORY | HCD_DMA | HCD_USB11,
// Basic lifecycle operations
    .reset =		uhci_platform_init,
    .start =		uhci_start,

    .pci_suspend =		core::ptr::null_mut(),
    .pci_resume =		core::ptr::null_mut(),
    .bus_suspend =		uhci_rh_suspend,
    .bus_resume =		uhci_rh_resume,

    .stop =			uhci_stop,
    .urb_enqueue =		uhci_urb_enqueue,
    .urb_dequeue =		uhci_urb_dequeue,
    .endpoint_disable =	uhci_hcd_endpoint_disable,
    .get_frame_number =	uhci_hcd_get_frame_number,
    .hub_status_data =	uhci_hub_status_data,
    .hub_control =		uhci_hub_control,
    };
#[no_mangle]
unsafe extern "C" fn uhci_hcd_platform_probe(pdev: *mut platform_device) -> c_int {
    static int uhci_hcd_platform_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut dma_mask_64: bool = false;
    struct usb_hcd *hcd;
    struct uhci_hcd	*uhci;
    struct resource *res;
    int ret;
    if (usb_disabled())
    return -ENODEV;
//
// Right now device-tree probed devices don't get dma_mask set.
// Since shared usb code relies on it, set it here for now.
// Once we have dma capability bindings this can go away.
//
    if (of_device_get_match_data(&pdev.dev))
    dma_mask_64 = true;
    ret = dma_coerce_mask_and_coherent(&pdev.dev,
    dma_mask_64 ? DMA_BIT_MASK(64) : DMA_BIT_MASK(32));
    if (ret)
    return ret;
    hcd = usb_create_hcd(&uhci_platform_hc_driver, &pdev.dev,
    pdev.name);
    if (!hcd)
    return -ENOMEM;
    uhci = hcd_to_uhci(hcd);
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    ret = PTR_ERR(hcd.regs);
    goto err_rmr;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    uhci.regs = hcd.regs;
// Grab some things from the device-tree
    if (np) {
    u32 num_ports;
    if (of_property_read_u32(np, "#ports", &num_ports) == 0) {
    uhci.rh_numports = num_ports;
    dev_info(&pdev.dev,
    "Detected %d ports from device-tree\n",
    num_ports);
    }
    if (of_device_is_compatible(np, "aspeed,ast2400-uhci") ||
    of_device_is_compatible(np, "aspeed,ast2500-uhci") ||
    of_device_is_compatible(np, "aspeed,ast2600-uhci") ||
    of_device_is_compatible(np, "aspeed,ast2700-uhci")) {
    uhci.is_aspeed = 1;
    dev_info(&pdev.dev,
    "Enabled Aspeed implementation workarounds\n");
    }
    }
// Get and enable clock if any specified
    uhci.clk = devm_clk_get_optional(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(uhci.clk)) {
    ret = PTR_ERR(uhci.clk);
    goto err_rmr;
    }
    ret = clk_prepare_enable(uhci.clk);
    if (ret) {
    dev_err(&pdev.dev, "Error couldn't enable clock (%d)\n", ret);
    goto err_rmr;
    }
    uhci.rsts = devm_reset_control_array_get_optional_shared(&pdev.dev);
    if (IS_ERR(uhci.rsts)) {
    ret = PTR_ERR(uhci.rsts);
    goto err_clk;
    }
    ret = reset_control_deassert(uhci.rsts);
    if (ret)
    goto err_clk;
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto err_reset;
    ret = usb_add_hcd(hcd, ret, IRQF_SHARED);
    if (ret)
    goto err_reset;
    device_wakeup_enable(hcd.self.controller);
    return 0;
    err_reset:
    reset_control_assert(uhci.rsts);
    err_clk:
    clk_disable_unprepare(uhci.clk);
    err_rmr:
    usb_put_hcd(hcd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uhci_hcd_platform_remove(pdev: *mut platform_device) {
    static void uhci_hcd_platform_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct uhci_hcd *uhci = hcd_to_uhci(hcd);
    reset_control_assert(uhci.rsts);
    clk_disable_unprepare(uhci.clk);
    usb_remove_hcd(hcd);
    usb_put_hcd(hcd);
    }
// Make sure the controller is quiescent and that we're not using it
// any more.  This is mainly for the benefit of programs which, like kexec,
// expect the hardware to be idle: not doing DMA or generating IRQs.
//
// This routine may be called in a damaged or failing kernel.  Hence we
// do not acquire the spinlock before shutting down the controller.
//
#[no_mangle]
unsafe extern "C" fn uhci_hcd_platform_shutdown(op: *mut platform_device) {
    static void uhci_hcd_platform_shutdown(struct platform_device *op)
    {
    struct usb_hcd *hcd = platform_get_drvdata(op);
    uhci_hc_died(hcd_to_uhci(hcd));
    }
    static const struct of_device_id platform_uhci_ids[] = {
    { .compatible = "generic-uhci", },
    { .compatible = "platform-uhci", },
    { .compatible = "aspeed,ast2700-uhci", .data = (void *)1 },
    {}
    };
    MODULE_DEVICE_TABLE(of, platform_uhci_ids);
    static struct platform_driver uhci_platform_driver = {
    .probe		= uhci_hcd_platform_probe,
    .remove		= uhci_hcd_platform_remove,
    .shutdown	= uhci_hcd_platform_shutdown,
    .driver = {
    .name = "platform-uhci",
    .of_match_table = platform_uhci_ids,
    },
    };
    MODULE_SOFTDEP("pre: ehci_platform");
