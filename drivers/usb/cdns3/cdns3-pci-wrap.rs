//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdns3-pci-wrap.c
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
// Cadence USBSS PCI Glue driver
//
// Copyright (C) 2018-2019 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_wrap {
    pub plat_dev: *mut platform_device,
    pub dev_res: [resource; 6],
    pub devfn: c_int,
}

pub const RES_IRQ_HOST_ID: c_int = 0;
pub const RES_IRQ_PERIPHERAL_ID: c_int = 1;
pub const RES_IRQ_OTG_ID: c_int = 2;
pub const RES_HOST_ID: c_int = 3;
pub const RES_DEV_ID: c_int = 4;
pub const RES_DRD_ID: c_int = 5;
pub const PCI_BAR_HOST: c_int = 0;
pub const PCI_BAR_DEV: c_int = 2;
pub const PCI_BAR_OTG: c_int = 0;
pub const PCI_DEV_FN_HOST_DEVICE: c_int = 0;
pub const PCI_DEV_FN_OTG: c_int = 1;

    static struct pci_dev *cdns3_get_second_fun(struct pci_dev *pdev)
    {
    struct pci_dev *func;
//
// Gets the second function.
// It's little tricky, but this platform has two function.
// The fist keeps resources for Host/Device while the second
// keeps resources for DRD/OTG.
//
    func = pci_get_device(pdev.vendor, pdev.device, core::ptr::null_mut());
    if (unlikely(!func))
    return core::ptr::null_mut();
    if (func.devfn == pdev.devfn) {
    func = pci_get_device(pdev.vendor, pdev.device, func);
    if (unlikely(!func))
    return core::ptr::null_mut();
    }
    if (func.devfn != PCI_DEV_FN_HOST_DEVICE &&
    func.devfn != PCI_DEV_FN_OTG) {
    return core::ptr::null_mut();
    }
    return func;
    }
    static int cdns3_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct platform_device_info plat_info;
    struct cdns3_wrap *wrap;
    struct resource *res;
    struct pci_dev *func;
    int err;
//
// for GADGET/HOST PCI (devfn) function number is 0,
// for OTG PCI (devfn) function number is 1
//
    if (!id || (pdev.devfn != PCI_DEV_FN_HOST_DEVICE &&
    pdev.devfn != PCI_DEV_FN_OTG))
    return -EINVAL;
    func = cdns3_get_second_fun(pdev);
    if (unlikely(!func))
    return -EINVAL;
    err = pcim_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "Enabling PCI device has failed %d\n", err);
    return err;
    }
    pci_set_master(pdev);
    if (pci_is_enabled(func)) {
    wrap = pci_get_drvdata(func);
    } else {
    wrap = kzalloc_obj(*wrap);
    if (!wrap)
    return -ENOMEM;
    }
    res = wrap.dev_res;
    if (pdev.devfn == PCI_DEV_FN_HOST_DEVICE) {
// function 0: host(BAR_0) + device(BAR_1).
    dev_dbg(&pdev.dev, "Initialize Device resources\n");
    res[RES_DEV_ID].start = pci_resource_start(pdev, PCI_BAR_DEV);
    res[RES_DEV_ID].end =   pci_resource_end(pdev, PCI_BAR_DEV);
    res[RES_DEV_ID].name = "dev";
    res[RES_DEV_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "USBSS-DEV physical base addr: %pa\n",
    &res[RES_DEV_ID].start);
    res[RES_HOST_ID].start = pci_resource_start(pdev, PCI_BAR_HOST);
    res[RES_HOST_ID].end = pci_resource_end(pdev, PCI_BAR_HOST);
    res[RES_HOST_ID].name = "xhci";
    res[RES_HOST_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "USBSS-XHCI physical base addr: %pa\n",
    &res[RES_HOST_ID].start);
// Interrupt for XHCI
    wrap.dev_res[RES_IRQ_HOST_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_HOST_ID].name = "host";
    wrap.dev_res[RES_IRQ_HOST_ID].flags = IORESOURCE_IRQ;
// Interrupt device. It's the same as for HOST.
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].name = "peripheral";
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].flags = IORESOURCE_IRQ;
    } else {
    res[RES_DRD_ID].start = pci_resource_start(pdev, PCI_BAR_OTG);
    res[RES_DRD_ID].end =   pci_resource_end(pdev, PCI_BAR_OTG);
    res[RES_DRD_ID].name = "otg";
    res[RES_DRD_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "USBSS-DRD physical base addr: %pa\n",
    &res[RES_DRD_ID].start);
// Interrupt for OTG/DRD.
    wrap.dev_res[RES_IRQ_OTG_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_OTG_ID].name = "otg";
    wrap.dev_res[RES_IRQ_OTG_ID].flags = IORESOURCE_IRQ;
    }
    if (pci_is_enabled(func)) {
// set up platform device info
    memset(&plat_info, 0, sizeof(plat_info));
    plat_info.parent = &pdev.dev;
    plat_info.fwnode = pdev.dev.fwnode;
    plat_info.name = PLAT_DRIVER_NAME;
    plat_info.id = pdev.devfn;
    wrap.devfn  = pdev.devfn;
    plat_info.res = wrap.dev_res;
    plat_info.num_res = ARRAY_SIZE(wrap.dev_res);
    plat_info.dma_mask = pdev.dma_mask;
// register platform device
    wrap.plat_dev = platform_device_register_full(&plat_info);
    if (IS_ERR(wrap.plat_dev)) {
    err = PTR_ERR(wrap.plat_dev);
    kfree(wrap);
    return err;
    }
    }
    pci_set_drvdata(pdev, wrap);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cdns3_pci_remove(pdev: *mut pci_dev) {
    static void cdns3_pci_remove(struct pci_dev *pdev)
    {
    struct cdns3_wrap *wrap;
    struct pci_dev *func;
    func = cdns3_get_second_fun(pdev);
    wrap = (struct cdns3_wrap *)pci_get_drvdata(pdev);
    if (wrap.devfn == pdev.devfn)
    platform_device_unregister(wrap.plat_dev);
    if (!pci_is_enabled(func))
    kfree(wrap);
    }
    static const struct pci_device_id cdns3_pci_ids[] = {
    { PCI_VDEVICE(CDNS, PCI_DEVICE_ID_CDNS_USBSS) },
    { 0, }
    };
    static struct pci_driver cdns3_pci_driver = {
    .name = PCI_DRIVER_NAME,
    .id_table = cdns3_pci_ids,
    .probe = cdns3_pci_probe,
    .remove = cdns3_pci_remove,
    };
    module_pci_driver(cdns3_pci_driver);
    MODULE_DEVICE_TABLE(pci, cdns3_pci_ids);
    MODULE_AUTHOR("Pawel Laszczak <pawell@cadence.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence USBSS PCI wrapper");
