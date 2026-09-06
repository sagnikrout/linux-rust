//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ssb-hcd.c
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
// Sonics Silicon Backplane
// Broadcom USB-core driver  (SSB bus glue)
//
// Copyright 2011-2012 Hauke Mehrtens <hauke@hauke-m.de>
//
// Based on ssb-ohci driver
// Copyright 2007 Michael Buesch <m@bues.ch>
//
// Derived from the OHCI-PCI driver
// Copyright 1999 Roman Weissgaerber
// Copyright 2000-2002 David Brownell
// Copyright 1999 Linus Torvalds
// Copyright 1999 Gregory P. Smith
//
// Derived from the USBcore related parts of Broadcom-SB
// Copyright 2005-2011 Broadcom Corporation
//

    MODULE_AUTHOR("Hauke Mehrtens");
    MODULE_DESCRIPTION("Common USB driver for SSB Bus");
    MODULE_LICENSE("GPL");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_hcd_device {
    pub ehci_dev: *mut platform_device,
    pub ohci_dev: *mut platform_device,
    pub enable_flags: u32,
}

#[no_mangle]
unsafe extern "C" fn ssb_hcd_5354wa(dev: *mut ssb_device) {
    static void ssb_hcd_5354wa(struct ssb_device *dev)
    {

// Work around for 5354 failures
    if (dev.id.revision == 2 && dev.bus.chip_id == 0x5354) {
// Change syn01 reg
    ssb_write32(dev, 0x894, 0x00fe00fe);
// Change syn03 reg
    ssb_write32(dev, 0x89c, ssb_read32(dev, 0x89c) | 0x1);
    }

    }
#[no_mangle]
unsafe extern "C" fn ssb_hcd_usb20wa(dev: *mut ssb_device) {
    static void ssb_hcd_usb20wa(struct ssb_device *dev)
    {
    if (dev.id.coreid == SSB_DEV_USB20_HOST) {
//
// USB 2.0 special considerations:
//
// In addition to the standard SSB reset sequence, the Host
// Control Register must be programmed to bring the USB core
// and various phy components out of reset.
//
    ssb_write32(dev, 0x200, 0x7ff);
// Change Flush control reg
    ssb_write32(dev, 0x400, ssb_read32(dev, 0x400) & ~8);
    ssb_read32(dev, 0x400);
// Change Shim control reg
    ssb_write32(dev, 0x304, ssb_read32(dev, 0x304) & ~0x100);
    ssb_read32(dev, 0x304);
    udelay(1);
    ssb_hcd_5354wa(dev);
    }
    }
// based on arch/mips/brcm-boards/bcm947xx/pcibios.c
#[no_mangle]
unsafe extern "C" fn ssb_hcd_init_chip(dev: *mut ssb_device) -> u32 {
    static u32 ssb_hcd_init_chip(struct ssb_device *dev)
    {
    let mut flags: u32 = 0;
    if (dev.id.coreid == SSB_DEV_USB11_HOSTDEV)
// Put the device into host-mode.
    flags |= SSB_HCD_TMSLOW_HOSTMODE;
    ssb_device_enable(dev, flags);
    ssb_hcd_usb20wa(dev);
    return flags;
    }
    static const struct usb_ehci_pdata ehci_pdata = {
    };
    static const struct usb_ohci_pdata ohci_pdata = {
    };
    static struct platform_device *ssb_hcd_create_pdev(struct ssb_device *dev, bool ohci, u32 addr, u32 len)
    {
    struct platform_device *hci_dev;
    struct resource hci_res[2];
    int ret;
    memset(hci_res, 0, sizeof(hci_res));
    hci_res[0].start = addr;
    hci_res[0].end = hci_res[0].start + len - 1;
    hci_res[0].flags = IORESOURCE_MEM;
    hci_res[1].start = dev.irq;
    hci_res[1].flags = IORESOURCE_IRQ;
    hci_dev = platform_device_alloc(ohci ? "ohci-platform" :
    "ehci-platform" , 0);
    if (!hci_dev)
    return ERR_PTR(-ENOMEM);
    hci_dev.dev.parent = dev.dev;
    hci_dev.dev.dma_mask = &hci_dev.dev.coherent_dma_mask;
    ret = platform_device_add_resources(hci_dev, hci_res,
    ARRAY_SIZE(hci_res));
    if (ret)
    goto err_alloc;
    if (ohci)
    ret = platform_device_add_data(hci_dev, &ohci_pdata,
    sizeof(ohci_pdata));
    else
    ret = platform_device_add_data(hci_dev, &ehci_pdata,
    sizeof(ehci_pdata));
    if (ret)
    goto err_alloc;
    ret = platform_device_add(hci_dev);
    if (ret)
    goto err_alloc;
    return hci_dev;
    err_alloc:
    platform_device_put(hci_dev);
    return ERR_PTR(ret);
    }
    static int ssb_hcd_probe(struct ssb_device *dev,
    const struct ssb_device_id *id)
    {
    int err, tmp;
    int start, len;
    u16 chipid_top;
    let mut coreid: u16 = dev.id.coreid;
    struct ssb_hcd_device *usb_dev;
// USBcores are only connected on embedded devices.
    chipid_top = (dev.bus.chip_id & 0xFF00);
    if (chipid_top != 0x4700 && chipid_top != 0x5300)
    return -ENODEV;
// TODO: Probably need checks here; is the core connected?
    if (dma_set_mask_and_coherent(dev.dma_dev, DMA_BIT_MASK(32)))
    return -EOPNOTSUPP;
    usb_dev = devm_kzalloc(dev.dev, sizeof(struct ssb_hcd_device),
    GFP_KERNEL);
    if (!usb_dev)
    return -ENOMEM;
// We currently always attach SSB_DEV_USB11_HOSTDEV
// as HOST OHCI. If we want to attach it as Client device,
// we must branch here and call into the (yet to
// be written) Client mode driver. Same for remove().
    usb_dev.enable_flags = ssb_hcd_init_chip(dev);
    tmp = ssb_read32(dev, SSB_ADMATCH0);
    start = ssb_admatch_base(tmp);
    len = (coreid == SSB_DEV_USB20_HOST) ? 0x800 : ssb_admatch_size(tmp);
    usb_dev.ohci_dev = ssb_hcd_create_pdev(dev, true, start, len);
    if (IS_ERR(usb_dev.ohci_dev))
    return PTR_ERR(usb_dev.ohci_dev);
    if (coreid == SSB_DEV_USB20_HOST) {
    start = ssb_admatch_base(tmp) + 0x800; /* ehci core offset */
    usb_dev.ehci_dev = ssb_hcd_create_pdev(dev, false, start, len);
    if (IS_ERR(usb_dev.ehci_dev)) {
    err = PTR_ERR(usb_dev.ehci_dev);
    goto err_unregister_ohci_dev;
    }
    }
    ssb_set_drvdata(dev, usb_dev);
    return 0;
    err_unregister_ohci_dev:
    platform_device_unregister(usb_dev.ohci_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssb_hcd_remove(dev: *mut ssb_device) {
    static void ssb_hcd_remove(struct ssb_device *dev)
    {
    struct ssb_hcd_device *usb_dev = ssb_get_drvdata(dev);
    struct platform_device *ohci_dev = usb_dev.ohci_dev;
    struct platform_device *ehci_dev = usb_dev.ehci_dev;
    if (ohci_dev)
    platform_device_unregister(ohci_dev);
    if (ehci_dev)
    platform_device_unregister(ehci_dev);
    ssb_device_disable(dev, 0);
    }
#[no_mangle]
unsafe extern "C" fn ssb_hcd_shutdown(dev: *mut ssb_device) {
    static void ssb_hcd_shutdown(struct ssb_device *dev)
    {
    ssb_device_disable(dev, 0);
    }

#[no_mangle]
unsafe extern "C" fn ssb_hcd_suspend(dev: *mut ssb_device, state: pm_message_t) -> c_int {
    static int ssb_hcd_suspend(struct ssb_device *dev, pm_message_t state)
    {
    ssb_device_disable(dev, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_hcd_resume(dev: *mut ssb_device) -> c_int {
    static int ssb_hcd_resume(struct ssb_device *dev)
    {
    struct ssb_hcd_device *usb_dev = ssb_get_drvdata(dev);
    ssb_device_enable(dev, usb_dev.enable_flags);
    return 0;
    }

    static const struct ssb_device_id ssb_hcd_table[] = {
    SSB_DEVICE(SSB_VENDOR_BROADCOM, SSB_DEV_USB11_HOSTDEV, SSB_ANY_REV),
    SSB_DEVICE(SSB_VENDOR_BROADCOM, SSB_DEV_USB11_HOST, SSB_ANY_REV),
    SSB_DEVICE(SSB_VENDOR_BROADCOM, SSB_DEV_USB20_HOST, SSB_ANY_REV),
    {},
    };
    MODULE_DEVICE_TABLE(ssb, ssb_hcd_table);
    static struct ssb_driver ssb_hcd_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= ssb_hcd_table,
    .probe		= ssb_hcd_probe,
    .remove		= ssb_hcd_remove,
    .shutdown	= ssb_hcd_shutdown,
    .suspend	= ssb_hcd_suspend,
    .resume		= ssb_hcd_resume,
    };
#[no_mangle]
unsafe extern "C" fn ssb_hcd_init() -> int __init {
    static int __init ssb_hcd_init(void)
    {
    return ssb_driver_register(&ssb_hcd_driver);
    }
    module_init(ssb_hcd_init);
#[no_mangle]
unsafe extern "C" fn ssb_hcd_exit() -> void __exit {
    static void __exit ssb_hcd_exit(void)
    {
    ssb_driver_unregister(&ssb_hcd_driver);
    }
    module_exit(ssb_hcd_exit);
