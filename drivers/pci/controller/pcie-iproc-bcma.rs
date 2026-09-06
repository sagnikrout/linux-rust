//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-iproc-bcma.c
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
// Copyright (C) 2015 Broadcom Corporation
// Copyright (C) 2015 Hauke Mehrtens <hauke@hauke-m.de>
//

// NS: CLASS field is R/O, and set to wrong 0x200 value
#[no_mangle]
unsafe extern "C" fn bcma_pcie2_fixup_class(dev: *mut pci_dev) {
    static void bcma_pcie2_fixup_class(struct pci_dev *dev)
    {
    dev.class = PCI_CLASS_BRIDGE_PCI_NORMAL;
    }
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_BROADCOM, 0x8011, bcma_pcie2_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_BROADCOM, 0x8012, bcma_pcie2_fixup_class);
#[no_mangle]
unsafe extern "C" fn iproc_bcma_pcie_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int {
    static int iproc_bcma_pcie_map_irq(const struct pci_dev *dev, u8 slot, u8 pin)
    {
    struct iproc_pcie *pcie = dev.sysdata;
    struct bcma_device *bdev = container_of(pcie.dev, struct bcma_device, dev);
    return bcma_core_irq(bdev, 5);
    }
#[no_mangle]
unsafe extern "C" fn iproc_bcma_pcie_probe(bdev: *mut bcma_device) -> c_int {
    static int iproc_bcma_pcie_probe(struct bcma_device *bdev)
    {
    struct device *dev = &bdev.dev;
    struct iproc_pcie *pcie;
    struct pci_host_bridge *bridge;
    int ret;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    pcie.dev = dev;
    pcie.type = IPROC_PCIE_PAXB_BCMA;
    pcie.base = bdev.io_addr;
    if (!pcie.base) {
    dev_err(dev, "no controller registers\n");
    return -ENOMEM;
    }
    pcie.base_addr = bdev.addr;
    pcie.mem.start = bdev.addr_s[0];
    pcie.mem.end = bdev.addr_s[0] + SZ_128M - 1;
    pcie.mem.name = "PCIe MEM space";
    pcie.mem.flags = IORESOURCE_MEM;
    pci_add_resource(&bridge.windows, &pcie.mem);
    ret = devm_request_pci_bus_resources(dev, &bridge.windows);
    if (ret)
    return ret;
    bridge.map_irq = iproc_bcma_pcie_map_irq;
    bcma_set_drvdata(bdev, pcie);
    return iproc_pcie_setup(pcie, &bridge.windows);
    }
#[no_mangle]
unsafe extern "C" fn iproc_bcma_pcie_remove(bdev: *mut bcma_device) {
    static void iproc_bcma_pcie_remove(struct bcma_device *bdev)
    {
    struct iproc_pcie *pcie = bcma_get_drvdata(bdev);
    iproc_pcie_remove(pcie);
    }
    static const struct bcma_device_id iproc_bcma_pcie_table[] = {
    BCMA_CORE(BCMA_MANUF_BCM, BCMA_CORE_NS_PCIEG2, BCMA_ANY_REV, BCMA_ANY_CLASS),
    {},
    };
    MODULE_DEVICE_TABLE(bcma, iproc_bcma_pcie_table);
    static struct bcma_driver iproc_bcma_pcie_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= iproc_bcma_pcie_table,
    .probe		= iproc_bcma_pcie_probe,
    .remove		= iproc_bcma_pcie_remove,
    };
    module_bcma_driver(iproc_bcma_pcie_driver);
    MODULE_AUTHOR("Hauke Mehrtens");
    MODULE_DESCRIPTION("Broadcom iProc PCIe BCMA driver");
    MODULE_LICENSE("GPL v2");
