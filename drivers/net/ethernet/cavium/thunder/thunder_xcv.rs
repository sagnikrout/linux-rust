//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/cavium/thunder/thunder_xcv.c
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
// Copyright (C) 2016 Cavium, Inc.
//

// Register offsets
pub const XCV_RESET: c_uint = 0x00;

pub const XCV_DLL_CTL: c_uint = 0x10;

pub const XCV_COMP_CTL: c_uint = 0x20;

pub const XCV_CTL: c_uint = 0x30;
pub const XCV_INT: c_uint = 0x40;
pub const XCV_INT_W1S: c_uint = 0x48;
pub const XCV_INT_ENA_W1C: c_uint = 0x50;
pub const XCV_INT_ENA_W1S: c_uint = 0x58;
pub const XCV_INBND_STATUS: c_uint = 0x80;
pub const XCV_BATCH_CRD_RET: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcv {
    pub reg_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
}

    static struct xcv *xcv;
// Supported devices
    static const struct pci_device_id xcv_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xA056) },
    { 0, }  /* end of table */
    };
    MODULE_AUTHOR("Cavium Inc");
    MODULE_DESCRIPTION("Cavium Thunder RGX/XCV Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION(DRV_VERSION);
    MODULE_DEVICE_TABLE(pci, xcv_id_table);
#[no_mangle]
pub unsafe extern "C" fn xcv_init_hw() {
    void xcv_init_hw(void)
    {
    u64  cfg;
// Take DLL out of reset
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg &= ~DLL_RESET;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
// Take clock tree out of reset
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg &= ~CLK_RESET;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
// Wait for DLL to lock
    msleep(1);
// Configure DLL - enable or bypass
// TX no bypass, RX bypass
//
    cfg = readq_relaxed(xcv.reg_base + XCV_DLL_CTL);
    cfg &= ~0xFF03;
    cfg |= CLKRX_BYP;
    writeq_relaxed(cfg, xcv.reg_base + XCV_DLL_CTL);
// Enable compensation controller and force the
// write to be visible to HW by readig back.
//
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg |= COMP_EN;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
    readq_relaxed(xcv.reg_base + XCV_RESET);
// Wait for compensation state machine to lock
    msleep(10);
// enable the XCV block
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg |= PORT_EN;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg |= CLK_RESET;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
    }
    EXPORT_SYMBOL(xcv_init_hw);
#[no_mangle]
pub unsafe extern "C" fn xcv_setup_link(link_up: bool, link_speed: c_int) {
    void xcv_setup_link(bool link_up, int link_speed)
    {
    u64  cfg;
    let mut speed: c_int = 2;
    if (!xcv) {
    pr_err("XCV init not done, probe may have failed\n");
    return;
    }
    if (link_speed == 100)
    speed = 1;
#[no_mangle]
pub unsafe extern "C" fn if(10: link_speed ==) -> else {
    else if (link_speed == 10)
    speed = 0;
    if (link_up) {
// set operating speed
    cfg = readq_relaxed(xcv.reg_base + XCV_CTL);
    cfg &= ~0x03;
    cfg |= speed;
    writeq_relaxed(cfg, xcv.reg_base + XCV_CTL);
// Reset datapaths
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg |= TX_DATA_RESET | RX_DATA_RESET;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
// Enable the packet flow
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg |= TX_PKT_RESET | RX_PKT_RESET;
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
// Return credits to RGX
    writeq_relaxed(0x01, xcv.reg_base + XCV_BATCH_CRD_RET);
    } else {
// Disable packet flow
    cfg = readq_relaxed(xcv.reg_base + XCV_RESET);
    cfg &= ~(TX_PKT_RESET | RX_PKT_RESET);
    writeq_relaxed(cfg, xcv.reg_base + XCV_RESET);
    readq_relaxed(xcv.reg_base + XCV_RESET);
    }
    }
    EXPORT_SYMBOL(xcv_setup_link);
#[no_mangle]
unsafe extern "C" fn xcv_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int xcv_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    int err;
    struct device *dev = &pdev.dev;
    xcv = devm_kzalloc(dev, sizeof(struct xcv), GFP_KERNEL);
    if (!xcv)
    return -ENOMEM;
    xcv.pdev = pdev;
    pci_set_drvdata(pdev, xcv);
    err = pci_enable_device(pdev);
    if (err) {
    dev_err(dev, "Failed to enable PCI device\n");
    goto err_kfree;
    }
    err = pci_request_regions(pdev, DRV_NAME);
    if (err) {
    dev_err(dev, "PCI request regions failed 0x%x\n", err);
    goto err_disable_device;
    }
// MAP configuration registers
    xcv.reg_base = pcim_iomap(pdev, PCI_CFG_REG_BAR_NUM, 0);
    if (!xcv.reg_base) {
    dev_err(dev, "XCV: Cannot map CSR memory space, aborting\n");
    err = -ENOMEM;
    goto err_release_regions;
    }
    return 0;
    err_release_regions:
    pci_release_regions(pdev);
    err_disable_device:
    pci_disable_device(pdev);
    err_kfree:
    devm_kfree(dev, xcv);
    xcv = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xcv_remove(pdev: *mut pci_dev) {
    static void xcv_remove(struct pci_dev *pdev)
    {
    struct device *dev = &pdev.dev;
    if (xcv) {
    devm_kfree(dev, xcv);
    xcv = core::ptr::null_mut();
    }
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_driver xcv_driver = {
    .name = DRV_NAME,
    .id_table = xcv_id_table,
    .probe = xcv_probe,
    .remove = xcv_remove,
    };
#[no_mangle]
unsafe extern "C" fn xcv_init_module() -> int __init {
    static int __init xcv_init_module(void)
    {
    pr_info("%s, ver %s\n", DRV_NAME, DRV_VERSION);
    return pci_register_driver(&xcv_driver);
    }
#[no_mangle]
unsafe extern "C" fn xcv_cleanup_module() -> void __exit {
    static void __exit xcv_cleanup_module(void)
    {
    pci_unregister_driver(&xcv_driver);
    }
    module_init(xcv_init_module);
    module_exit(xcv_cleanup_module);
