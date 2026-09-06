//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-sa1111.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// OHCI HCD (Host Controller Driver) for USB.
//
// (C) Copyright 1999 Roman Weissgaerber <weissg@vienna.at>
// (C) Copyright 2000-2002 David Brownell <dbrownell@users.sourceforge.net>
// (C) Copyright 2002 Hewlett-Packard Company
//
// SA1111 Bus Glue
//
// Written by Christopher Hoover <ch@hpl.hp.com>
// Based on fragments of previous driver by Russell King et al.
//
// This file is licenced under the GPL.
//

pub const USB_STATUS: c_uint = 0x0118;
pub const USB_RESET: c_uint = 0x011c;
pub const USB_IRQTEST: c_uint = 0x0120;

#[no_mangle]
unsafe extern "C" fn dump_hci_status(hcd: *mut usb_hcd, label: *const c_char) {
    static void dump_hci_status(struct usb_hcd *hcd, const char *label)
    {
    let mut status: c_ulong = readl_relaxed(hcd.regs + USB_STATUS);
    printk(KERN_DEBUG "%s USB_STATUS = { %s%s%s%s%s}\n", label,
    ((status & USB_STATUS_IRQHCIRMTWKUP) ? "IRQHCIRMTWKUP " : ""),
    ((status & USB_STATUS_IRQHCIBUFFACC) ? "IRQHCIBUFFACC " : ""),
    ((status & USB_STATUS_NIRQHCIM) ? "" : "IRQHCIM "),
    ((status & USB_STATUS_NHCIMFCLR) ? "" : "HCIMFCLR "),
    ((status & USB_STATUS_USBPWRSENSE) ? "USBPWRSENSE " : ""));
    }

#[no_mangle]
unsafe extern "C" fn ohci_sa1111_reset(hcd: *mut usb_hcd) -> c_int {
    static int ohci_sa1111_reset(struct usb_hcd *hcd)
    {
    struct ohci_hcd *ohci = hcd_to_ohci(hcd);
    ohci_hcd_init(ohci);
    return ohci_init(ohci);
    }
#[no_mangle]
unsafe extern "C" fn ohci_sa1111_start(hcd: *mut usb_hcd) -> c_int {
    static int ohci_sa1111_start(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci(hcd);
    int ret;
    ret = ohci_run(ohci);
    if (ret < 0) {
    ohci_err(ohci, "can't start\n");
    ohci_stop(hcd);
    }
    return ret;
    }
    static const struct hc_driver ohci_sa1111_hc_driver = {
    .description =		hcd_name,
    .product_desc =		"SA-1111 OHCI",
    .hcd_priv_size =	sizeof(struct ohci_hcd),
//
// generic hardware linkage
//
    .irq =			ohci_irq,
    .flags =		HCD_USB11 | HCD_DMA | HCD_MEMORY,
//
// basic lifecycle operations
//
    .reset =		ohci_sa1111_reset,
    .start =		ohci_sa1111_start,
    .stop =			ohci_stop,
    .shutdown =		ohci_shutdown,
//
// managing i/o requests and associated device resources
//
    .urb_enqueue =		ohci_urb_enqueue,
    .urb_dequeue =		ohci_urb_dequeue,
    .endpoint_disable =	ohci_endpoint_disable,
//
// scheduling support
//
    .get_frame_number =	ohci_get_frame,
//
// root hub support
//
    .hub_status_data =	ohci_hub_status_data,
    .hub_control =		ohci_hub_control,

    .bus_suspend =		ohci_bus_suspend,
    .bus_resume =		ohci_bus_resume,

    .start_port_reset =	ohci_start_port_reset,
    };
#[no_mangle]
unsafe extern "C" fn sa1111_start_hc(dev: *mut sa1111_dev) -> c_int {
    static int sa1111_start_hc(struct sa1111_dev *dev)
    {
    let mut usb_rst: c_uint = 0;
    int ret;
    dev_dbg(&dev.dev, "starting SA-1111 OHCI USB Controller\n");
    if (machine_is_assabet())
    usb_rst = USB_RESET_PWRSENSELOW | USB_RESET_PWRCTRLLOW;
//
// Configure the power sense and control lines.  Place the USB
// host controller in reset.
//
    writel_relaxed(usb_rst | USB_RESET_FORCEIFRESET | USB_RESET_FORCEHCRESET,
    dev.mapbase + USB_RESET);
//
// Now, carefully enable the USB clock, and take
// the USB host controller out of reset.
//
    ret = sa1111_enable_device(dev);
    if (ret == 0) {
    udelay(11);
    writel_relaxed(usb_rst, dev.mapbase + USB_RESET);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sa1111_stop_hc(dev: *mut sa1111_dev) {
    static void sa1111_stop_hc(struct sa1111_dev *dev)
    {
    unsigned int usb_rst;
    dev_dbg(&dev.dev, "stopping SA-1111 OHCI USB Controller\n");
//
// Put the USB host controller into reset.
//
    usb_rst = readl_relaxed(dev.mapbase + USB_RESET);
    writel_relaxed(usb_rst | USB_RESET_FORCEIFRESET | USB_RESET_FORCEHCRESET,
    dev.mapbase + USB_RESET);
//
// Stop the USB clock.
//
    sa1111_disable_device(dev);
    }
//
// ohci_hcd_sa1111_probe - initialize SA-1111-based HCDs
//
// Allocates basic resources for this USB host controller, and
// then invokes the start() method for the HCD associated with it.
//
#[no_mangle]
unsafe extern "C" fn ohci_hcd_sa1111_probe(dev: *mut sa1111_dev) -> c_int {
    static int ohci_hcd_sa1111_probe(struct sa1111_dev *dev)
    {
    struct usb_hcd *hcd;
    int ret, irq;
    if (usb_disabled())
    return -ENODEV;
//
// We don't call dma_set_mask_and_coherent() here because the
// DMA mask has already been appropraitely setup by the core
// SA-1111 bus code (which includes bug workarounds.)
//
    hcd = usb_create_hcd(&ohci_sa1111_hc_driver, &dev.dev, "sa1111");
    if (!hcd)
    return -ENOMEM;
    hcd.rsrc_start = dev.res.start;
    hcd.rsrc_len = resource_size(&dev.res);
    irq = sa1111_get_irq(dev, 1);
    if (irq <= 0) {
    ret = irq ? : -ENXIO;
    goto err1;
    }
//
// According to the "Intel StrongARM SA-1111 Microprocessor Companion
// Chip Specification Update" (June 2000), erratum #7, there is a
// significant bug in the SA1111 SDRAM shared memory controller.  If
// an access to a region of memory above 1MB relative to the bank base,
// it is important that address bit 10 _NOT_ be asserted. Depending
// on the configuration of the RAM, bit 10 may correspond to one
// of several different (processor-relative) address bits.
//
// Section 4.6 of the "Intel StrongARM SA-1111 Development Module
// User's Guide" mentions that jumpers R51 and R52 control the
// target of SA-1111 DMA (either SDRAM bank 0 on Assabet, or
// SDRAM bank 1 on Neponset). The default configuration selects
// Assabet, so any address in bank 1 is necessarily invalid.
//
// As a workaround, use a bounce buffer in addressable memory
// as local_mem, relying on ZONE_DMA to provide an area that
// fits within the above constraints.
//
// SZ_64K is an estimate for what size this might need.
//
    ret = usb_hcd_setup_local_mem(hcd, 0, 0, SZ_64K);
    if (ret)
    goto err1;
    if (!request_mem_region(hcd.rsrc_start, hcd.rsrc_len, hcd_name)) {
    dev_dbg(&dev.dev, "request_mem_region failed\n");
    ret = -EBUSY;
    goto err1;
    }
    hcd.regs = dev.mapbase;
    ret = sa1111_start_hc(dev);
    if (ret)
    goto err2;
    ret = usb_add_hcd(hcd, irq, 0);
    if (ret == 0) {
    device_wakeup_enable(hcd.self.controller);
    return ret;
    }
    sa1111_stop_hc(dev);
    err2:
    release_mem_region(hcd.rsrc_start, hcd.rsrc_len);
    err1:
    usb_put_hcd(hcd);
    return ret;
    }
//
// ohci_hcd_sa1111_remove - shutdown processing for SA-1111-based HCDs
// @dev: USB Host Controller being removed
//
// Reverses the effect of ohci_hcd_sa1111_probe(), first invoking
// the HCD's stop() method.
//
#[no_mangle]
unsafe extern "C" fn ohci_hcd_sa1111_remove(dev: *mut sa1111_dev) {
    static void ohci_hcd_sa1111_remove(struct sa1111_dev *dev)
    {
    struct usb_hcd *hcd = sa1111_get_drvdata(dev);
    usb_remove_hcd(hcd);
    sa1111_stop_hc(dev);
    release_mem_region(hcd.rsrc_start, hcd.rsrc_len);
    usb_put_hcd(hcd);
    }
#[no_mangle]
unsafe extern "C" fn ohci_hcd_sa1111_shutdown(_dev: *mut device) {
    static void ohci_hcd_sa1111_shutdown(struct device *_dev)
    {
    struct sa1111_dev *dev = to_sa1111_device(_dev);
    struct usb_hcd *hcd = sa1111_get_drvdata(dev);
    if (test_bit(HCD_FLAG_HW_ACCESSIBLE, &hcd.flags)) {
    hcd.driver.shutdown(hcd);
    sa1111_stop_hc(dev);
    }
    }
    static struct sa1111_driver ohci_hcd_sa1111_driver = {
    .drv = {
    .name	= "sa1111-ohci",
    .owner	= THIS_MODULE,
    .shutdown = ohci_hcd_sa1111_shutdown,
    },
    .devid		= SA1111_DEVID_USB,
    .probe		= ohci_hcd_sa1111_probe,
    .remove		= ohci_hcd_sa1111_remove,
    };
