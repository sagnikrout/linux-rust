//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdnsp-pci.c
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
// Cadence USBSSP PCI Glue driver.
//
// Copyright (C) 2019 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_wrap {
    pub plat_dev: *mut platform_device,
    pub prop: [property_entry; 3],
    pub dev_res: [resource; 6],
    pub devfn: c_int,
}

pub const RES_IRQ_HOST_ID: c_int = 0;
pub const RES_IRQ_PERIPHERAL_ID: c_int = 1;
pub const RES_IRQ_OTG_ID: c_int = 2;
pub const RES_HOST_ID: c_int = 3;
pub const RES_DEV_ID: c_int = 4;
pub const RES_DRD_ID: c_int = 5;
// DRD PCI configuration - 64-bit addressing
// First PCI function
pub const PCI_BAR_HOST: c_int = 0;
pub const PCI_BAR_DEV: c_int = 2;
// Second PCI function
pub const PCI_BAR_OTG: c_int = 0;
// Device only PCI configuration - 32-bit addressing
// First PCI function
pub const PCI_BAR_ONLY_DEV: c_int = 1;
pub const PCI_DEV_FN_HOST_DEVICE: c_int = 0;
pub const PCI_DEV_FN_OTG: c_int = 1;

pub const PCI_DEVICE_ID_CDNS_UDC_USBSSP: c_uint = 0x0400;
pub const CHICKEN_APB_TIMEOUT_VALUE: c_uint = 0x1C20;
    static struct pci_dev *cdnsp_get_second_fun(struct pci_dev *pdev)
    {
//
// Gets the second function.
// Platform has two function. The first keeps resources for
// Host/Device while the second keeps resources for DRD/OTG.
//
    if (pdev.device == PCI_DEVICE_ID_CDNS_USBSSP)
    return pci_get_device(pdev.vendor, PCI_DEVICE_ID_CDNS_USBSS, core::ptr::null_mut());
    if (pdev.device == PCI_DEVICE_ID_CDNS_USBSS)
    return pci_get_device(pdev.vendor, PCI_DEVICE_ID_CDNS_USBSSP, core::ptr::null_mut());
    return core::ptr::null_mut();
    }
    static int cdnsp_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct platform_device_info plat_info;
    static struct cdns3_platform_data pdata;
    struct cdnsp_wrap *wrap;
    struct resource *res;
    struct pci_dev *func;
    let mut no_drd: bool = false;
    let mut ret: c_int = 0;
//
// For GADGET/HOST PCI (devfn) function number is 0,
// for OTG PCI (devfn) function number is 1.
//
    if (!id || (pdev.devfn != PCI_DEV_FN_HOST_DEVICE &&
    pdev.devfn != PCI_DEV_FN_OTG))
    return -EINVAL;
    if (pdev.device == PCI_DEVICE_ID_CDNS_UDC_USBSSP)
    no_drd = true;
    func = cdnsp_get_second_fun(pdev);
    if (!func && !no_drd)
    return -EINVAL;
    if ((func && func.class == PCI_CLASS_SERIAL_USB_XHCI) ||
    pdev.class == PCI_CLASS_SERIAL_USB_XHCI) {
    ret = -EINVAL;
    goto put_pci;
    }
    ret = pcim_enable_device(pdev);
    if (ret) {
    dev_err(&pdev.dev, "Enabling PCI device has failed %d\n", ret);
    goto put_pci;
    }
    pci_set_master(pdev);
    if (func && pci_is_enabled(func)) {
    wrap = pci_get_drvdata(func);
    } else {
    wrap = kzalloc_obj(*wrap);
    if (!wrap) {
    ret = -ENOMEM;
    goto put_pci;
    }
    }
    res = wrap.dev_res;
    if (pdev.devfn == PCI_DEV_FN_HOST_DEVICE) {
    let mut bar_dev: c_int = no_drd ? PCI_BAR_ONLY_DEV : PCI_BAR_DEV;
// Function 0: host(BAR_0) + device(BAR_2).
    dev_dbg(&pdev.dev, "Initialize Device resources\n");
    res[RES_DEV_ID].start = pci_resource_start(pdev, bar_dev);
    res[RES_DEV_ID].end = pci_resource_end(pdev, bar_dev);
    res[RES_DEV_ID].name = "dev";
    res[RES_DEV_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "USBSSP-DEV physical base addr: %pa\n",
    &res[RES_DEV_ID].start);
    res[RES_HOST_ID].start = pci_resource_start(pdev, PCI_BAR_HOST);
    res[RES_HOST_ID].end = pci_resource_end(pdev, PCI_BAR_HOST);
    res[RES_HOST_ID].name = "xhci";
    res[RES_HOST_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "USBSSP-XHCI physical base addr: %pa\n",
    &res[RES_HOST_ID].start);
// Interrupt for XHCI
    wrap.dev_res[RES_IRQ_HOST_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_HOST_ID].name = "host";
    wrap.dev_res[RES_IRQ_HOST_ID].flags = IORESOURCE_IRQ;
// Interrupt for device. It's the same as for HOST.
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].name = "peripheral";
    wrap.dev_res[RES_IRQ_PERIPHERAL_ID].flags = IORESOURCE_IRQ;
    } else {
    res[RES_DRD_ID].start = pci_resource_start(pdev, PCI_BAR_OTG);
    res[RES_DRD_ID].end = pci_resource_end(pdev, PCI_BAR_OTG);
    res[RES_DRD_ID].name = "otg";
    res[RES_DRD_ID].flags = IORESOURCE_MEM;
    dev_dbg(&pdev.dev, "CDNSP-DRD physical base addr: %pa\n",
    &res[RES_DRD_ID].start);
// Interrupt for OTG/DRD.
    wrap.dev_res[RES_IRQ_OTG_ID].start = pdev.irq;
    wrap.dev_res[RES_IRQ_OTG_ID].name = "otg";
    wrap.dev_res[RES_IRQ_OTG_ID].flags = IORESOURCE_IRQ;
    }
    if (no_drd || pci_is_enabled(func)) {
    let mut idx: u8 = 0;
// set up platform device info
    pdata.override_apb_timeout = CHICKEN_APB_TIMEOUT_VALUE;
    if (no_drd) {
    wrap.prop[idx++] = PROPERTY_ENTRY_STRING("compatible",
    "cdns,cdnsp");
    wrap.prop[idx++] = PROPERTY_ENTRY_STRING("dr_mode", "peripheral");
    } else {
    wrap.prop[idx++] = PROPERTY_ENTRY_STRING("dr_mode", "otg");
    wrap.prop[idx++] = PROPERTY_ENTRY_BOOL("usb-role-switch");
    }
    memset(&plat_info, 0, sizeof(plat_info));
    plat_info.parent = &pdev.dev;
    plat_info.fwnode = pdev.dev.fwnode;
    plat_info.name = PLAT_DRIVER_NAME;
    plat_info.id = pdev.devfn;
    plat_info.res = wrap.dev_res;
    plat_info.num_res = ARRAY_SIZE(wrap.dev_res);
    plat_info.dma_mask = pdev.dma_mask;
    plat_info.data = &pdata;
    plat_info.size_data = sizeof(pdata);
    plat_info.properties = wrap.prop;
    wrap.devfn = pdev.devfn;
// register platform device
    wrap.plat_dev = platform_device_register_full(&plat_info);
    if (IS_ERR(wrap.plat_dev)) {
    ret = PTR_ERR(wrap.plat_dev);
    kfree(wrap);
    goto put_pci;
    }
    }
    pci_set_drvdata(pdev, wrap);
    put_pci:
    pci_dev_put(func);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdnsp_pci_remove(pdev: *mut pci_dev) {
    static void cdnsp_pci_remove(struct pci_dev *pdev)
    {
    struct cdnsp_wrap *wrap;
    struct pci_dev *func;
    func = cdnsp_get_second_fun(pdev);
    wrap = pci_get_drvdata(pdev);
    if (wrap.devfn == pdev.devfn)
    platform_device_unregister(wrap.plat_dev);
    if (!func || !pci_is_enabled(func))
    kfree(wrap);
    pci_dev_put(func);
    }
    static const struct pci_device_id cdnsp_pci_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CDNS, PCI_DEVICE_ID_CDNS_UDC_USBSSP),
    .class = PCI_CLASS_SERIAL_USB_DEVICE },
    { PCI_DEVICE(PCI_VENDOR_ID_CDNS, PCI_DEVICE_ID_CDNS_UDC_USBSSP),
    .class = PCI_CLASS_SERIAL_USB_CDNS },
    { PCI_DEVICE(PCI_VENDOR_ID_CDNS, PCI_DEVICE_ID_CDNS_USBSSP),
    .class = PCI_CLASS_SERIAL_USB_DEVICE },
    { PCI_DEVICE(PCI_VENDOR_ID_CDNS, PCI_DEVICE_ID_CDNS_USBSSP),
    .class = PCI_CLASS_SERIAL_USB_CDNS },
    { PCI_DEVICE(PCI_VENDOR_ID_CDNS, PCI_DEVICE_ID_CDNS_USBSS),
    .class = PCI_CLASS_SERIAL_USB_CDNS },
    { 0, }
    };
    static struct pci_driver cdnsp_pci_driver = {
    .name = PCI_DRIVER_NAME,
    .id_table = cdnsp_pci_ids,
    .probe = cdnsp_pci_probe,
    .remove = cdnsp_pci_remove,
    };
    module_pci_driver(cdnsp_pci_driver);
    MODULE_DEVICE_TABLE(pci, cdnsp_pci_ids);
    MODULE_ALIAS("pci:cdnsp");
    MODULE_AUTHOR("Pawel Laszczak <pawell@cadence.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence CDNSP PCI wrapper");
