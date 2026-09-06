//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/enetc/enetc_pci_mdio.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2019 NXP

pub const NETC_EMDIO_VEN_ID: c_uint = 0x1131;
pub const NETC_EMDIO_DEV_ID: c_uint = 0xee00;
pub const ENETC_MDIO_DEV_ID: c_uint = 0xee01;

    DEFINE_STATIC_KEY_FALSE(enetc_has_err050089);
    EXPORT_SYMBOL_GPL(enetc_has_err050089);
#[no_mangle]
unsafe extern "C" fn enetc_emdio_enable_err050089(pdev: *mut pci_dev) {
    static void enetc_emdio_enable_err050089(struct pci_dev *pdev)
    {
    if (pdev.vendor == PCI_VENDOR_ID_FREESCALE &&
    pdev.device == ENETC_MDIO_DEV_ID) {
    static_branch_inc(&enetc_has_err050089);
    dev_info(&pdev.dev, "Enabled ERR050089 workaround\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn enetc_emdio_disable_err050089(pdev: *mut pci_dev) {
    static void enetc_emdio_disable_err050089(struct pci_dev *pdev)
    {
    if (pdev.vendor == PCI_VENDOR_ID_FREESCALE &&
    pdev.device == ENETC_MDIO_DEV_ID) {
    static_branch_dec(&enetc_has_err050089);
    if (!static_key_enabled(&enetc_has_err050089.key))
    dev_info(&pdev.dev, "Disabled ERR050089 workaround\n");
    }
    }
    static int enetc_pci_mdio_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct enetc_mdio_priv *mdio_priv;
    struct device *dev = &pdev.dev;
    void __iomem *port_regs;
    struct enetc_hw *hw;
    struct mii_bus *bus;
    int err;
    port_regs = pci_iomap(pdev, 0, 0);
    if (!port_regs) {
    dev_err(dev, "iomap failed\n");
    err = -ENXIO;
    goto err_ioremap;
    }
    hw = enetc_hw_alloc(dev, port_regs);
    if (IS_ERR(hw)) {
    err = PTR_ERR(hw);
    goto err_hw_alloc;
    }
    bus = devm_mdiobus_alloc_size(dev, sizeof(*mdio_priv));
    if (!bus) {
    err = -ENOMEM;
    goto err_mdiobus_alloc;
    }
    bus.name = ENETC_MDIO_BUS_NAME;
    bus.read = enetc_mdio_read_c22;
    bus.write = enetc_mdio_write_c22;
    bus.read_c45 = enetc_mdio_read_c45;
    bus.write_c45 = enetc_mdio_write_c45;
    bus.parent = dev;
    mdio_priv = bus.priv;
    mdio_priv.hw = hw;
    mdio_priv.mdio_base = ENETC_EMDIO_BASE;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s", dev_name(dev));
    pcie_flr(pdev);
    err = pci_enable_device_mem(pdev);
    if (err) {
    dev_err(dev, "device enable failed\n");
    goto err_pci_enable;
    }
    err = pci_request_region(pdev, 0, KBUILD_MODNAME);
    if (err) {
    dev_err(dev, "pci_request_region failed\n");
    goto err_pci_mem_reg;
    }
    enetc_emdio_enable_err050089(pdev);
    err = of_mdiobus_register(bus, dev.of_node);
    if (err)
    goto err_mdiobus_reg;
    pci_set_drvdata(pdev, bus);
    return 0;
    err_mdiobus_reg:
    enetc_emdio_disable_err050089(pdev);
    pci_release_region(pdev, 0);
    err_pci_mem_reg:
    pci_disable_device(pdev);
    err_pci_enable:
    err_mdiobus_alloc:
    err_hw_alloc:
    iounmap(port_regs);
    err_ioremap:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn enetc_pci_mdio_remove(pdev: *mut pci_dev) {
    static void enetc_pci_mdio_remove(struct pci_dev *pdev)
    {
    struct mii_bus *bus = pci_get_drvdata(pdev);
    struct enetc_mdio_priv *mdio_priv;
    mdiobus_unregister(bus);
    enetc_emdio_disable_err050089(pdev);
    mdio_priv = bus.priv;
    iounmap(mdio_priv.hw.port);
    pci_release_region(pdev, 0);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id enetc_pci_mdio_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_FREESCALE, ENETC_MDIO_DEV_ID) },
    { PCI_DEVICE(NETC_EMDIO_VEN_ID, NETC_EMDIO_DEV_ID) },
    { 0, } /* End of table. */
    };
    MODULE_DEVICE_TABLE(pci, enetc_pci_mdio_id_table);
    static struct pci_driver enetc_pci_mdio_driver = {
    .name = KBUILD_MODNAME,
    .id_table = enetc_pci_mdio_id_table,
    .probe = enetc_pci_mdio_probe,
    .remove = enetc_pci_mdio_remove,
    };
    module_pci_driver(enetc_pci_mdio_driver);
    MODULE_DESCRIPTION(ENETC_MDIO_DRV_NAME);
    MODULE_LICENSE("Dual BSD/GPL");
