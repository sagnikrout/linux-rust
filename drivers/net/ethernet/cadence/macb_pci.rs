//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/cadence/macb_pci.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// DOC: Cadence GEM PCI wrapper.
//
// Copyright (C) 2016 Cadence Design Systems - https://www.cadence.com
//
// Authors: Rafal Ozieblo <rafalo@cadence.com>
// Bartosz Folta <bfolta@cadence.com>
//

pub const PCI_DEVICE_ID_CDNS_MACB: c_uint = 0xe007;
pub const GEM_PCLK_RATE: c_int = 50000000;
pub const GEM_HCLK_RATE: c_int = 50000000;
#[no_mangle]
unsafe extern "C" fn macb_probe(pci: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int macb_probe(struct pci_dev *pci, const struct pci_device_id *id)
    {
    int err;
    struct platform_device *pdev;
    struct platform_device_info plat_info;
    struct macb_platform_data plat_data;
    struct resource res[2];
// enable pci device
    err = pcim_enable_device(pci);
    if (err < 0) {
    dev_err(&pci.dev, "Enabling PCI device has failed: %d", err);
    return err;
    }
    pci_set_master(pci);
// set up resources
    memset(res, 0x00, sizeof(struct resource) * ARRAY_SIZE(res));
    res[0].start = pci_resource_start(pci, 0);
    res[0].end = pci_resource_end(pci, 0);
    res[0].name = PCI_DRIVER_NAME;
    res[0].flags = IORESOURCE_MEM;
    res[1].start = pci_irq_vector(pci, 0);
    res[1].name = PCI_DRIVER_NAME;
    res[1].flags = IORESOURCE_IRQ;
    dev_info(&pci.dev, "EMAC physical base addr: %pa\n",
    &res[0].start);
// set up macb platform data
    memset(&plat_data, 0, sizeof(plat_data));
// initialize clocks
    plat_data.pclk = clk_register_fixed_rate(&pci.dev, "pclk", core::ptr::null_mut(), 0,
    GEM_PCLK_RATE);
    if (IS_ERR(plat_data.pclk)) {
    err = PTR_ERR(plat_data.pclk);
    goto err_pclk_register;
    }
    plat_data.hclk = clk_register_fixed_rate(&pci.dev, "hclk", core::ptr::null_mut(), 0,
    GEM_HCLK_RATE);
    if (IS_ERR(plat_data.hclk)) {
    err = PTR_ERR(plat_data.hclk);
    goto err_hclk_register;
    }
// set up platform device info
    memset(&plat_info, 0, sizeof(plat_info));
    plat_info.parent = &pci.dev;
    plat_info.fwnode = pci.dev.fwnode;
    plat_info.name = PLAT_DRIVER_NAME;
    plat_info.id = pci.devfn;
    plat_info.res = res;
    plat_info.num_res = ARRAY_SIZE(res);
    plat_info.data = &plat_data;
    plat_info.size_data = sizeof(plat_data);
    plat_info.dma_mask = pci.dma_mask;
// register platform device
    pdev = platform_device_register_full(&plat_info);
    if (IS_ERR(pdev)) {
    err = PTR_ERR(pdev);
    goto err_plat_dev_register;
    }
    pci_set_drvdata(pci, pdev);
    return 0;
    err_plat_dev_register:
    clk_unregister_fixed_rate(plat_data.hclk);
    err_hclk_register:
    clk_unregister_fixed_rate(plat_data.pclk);
    err_pclk_register:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn macb_remove(pci: *mut pci_dev) {
    static void macb_remove(struct pci_dev *pci)
    {
    struct platform_device *pdev = pci_get_drvdata(pci);
    struct macb_platform_data *plat_data = dev_get_platdata(&pdev.dev);
    struct clk *pclk = plat_data.pclk;
    struct clk *hclk = plat_data.hclk;
    platform_device_unregister(pdev);
    clk_unregister_fixed_rate(pclk);
    clk_unregister_fixed_rate(hclk);
    }
    static const struct pci_device_id dev_id_table[] = {
    { PCI_VDEVICE(CDNS, PCI_DEVICE_ID_CDNS_MACB) },
    { 0, }
    };
    static struct pci_driver macb_pci_driver = {
    .name     = PCI_DRIVER_NAME,
    .id_table = dev_id_table,
    .probe    = macb_probe,
    .remove	  = macb_remove,
    };
    module_pci_driver(macb_pci_driver);
    MODULE_DEVICE_TABLE(pci, dev_id_table);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Cadence NIC PCI wrapper");
