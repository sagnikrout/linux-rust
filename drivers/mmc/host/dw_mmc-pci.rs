//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/dw_mmc-pci.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Synopsys DesignWare Multimedia Card PCI Interface driver
//
// Copyright (C) 2012 Vayavya Labs Pvt. Ltd.
//

pub const SYNOPSYS_DW_MCI_VENDOR_ID: c_uint = 0x700;
pub const SYNOPSYS_DW_MCI_DEVICE_ID: c_uint = 0x1107;
// Defining the Capabilities

    MMC_CAP_SD_HIGHSPEED | MMC_CAP_8_BIT_DATA |\
    MMC_CAP_SDIO_IRQ)
    static const struct dw_mci_drv_data pci_drv_data = {
    .common_caps = DW_MCI_CAPABILITIES,
    };
    static int dw_mci_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *entries)
    {
    struct dw_mci *host;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    host = dw_mci_alloc_host(&pdev.dev);
    if (IS_ERR(host))
    return PTR_ERR(host);
    host.irq = pdev.irq;
    host.irq_flags = IRQF_SHARED;
    host.fifo_depth = 32;
    host.detect_delay_ms = 200;
    host.bus_hz = 33 * 1000 * 1000;
    host.drv_data = &pci_drv_data;
    host.regs = pcim_iomap_region(pdev, BAR_2, pci_name(pdev));
    if (IS_ERR(host.regs))
    return PTR_ERR(host.regs);
    pci_set_master(pdev);
    ret = dw_mci_probe(host);
    if (ret)
    return ret;
    pci_set_drvdata(pdev, host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mci_pci_remove(pdev: *mut pci_dev) {
    static void dw_mci_pci_remove(struct pci_dev *pdev)
    {
    struct dw_mci *host = pci_get_drvdata(pdev);
    dw_mci_remove(host);
    }
    static const struct pci_device_id dw_mci_pci_id[] = {
    { PCI_DEVICE(SYNOPSYS_DW_MCI_VENDOR_ID, SYNOPSYS_DW_MCI_DEVICE_ID) },
    {}
    };
    MODULE_DEVICE_TABLE(pci, dw_mci_pci_id);
    static struct pci_driver dw_mci_pci_driver = {
    .name		= "dw_mmc_pci",
    .id_table	= dw_mci_pci_id,
    .probe		= dw_mci_pci_probe,
    .remove		= dw_mci_pci_remove,
    .driver		=	{
    .pm =   pm_ptr(&dw_mci_pmops),
    },
    };
    module_pci_driver(dw_mci_pci_driver);
    MODULE_DESCRIPTION("DW Multimedia Card PCI Interface driver");
    MODULE_AUTHOR("Shashidhar Hiremath <shashidharh@vayavyalabs.com>");
    MODULE_LICENSE("GPL v2");
