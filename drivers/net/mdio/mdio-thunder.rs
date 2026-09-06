//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-thunder.c
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
// Copyright (C) 2009-2016 Cavium, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thunder_mdiobus_nexus {
    pub bar0: *mut void __iomem,
    pub buses: [*mut cavium_mdiobus; 4],
}

    static int thunder_mdiobus_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct device_node *node;
    struct thunder_mdiobus_nexus *nexus;
    int err;
    int i;
    nexus = devm_kzalloc(&pdev.dev, sizeof(*nexus), GFP_KERNEL);
    if (!nexus)
    return -ENOMEM;
    pci_set_drvdata(pdev, nexus);
    err = pcim_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "Failed to enable PCI device\n");
    pci_set_drvdata(pdev, core::ptr::null_mut());
    return err;
    }
    err = pcim_request_all_regions(pdev, KBUILD_MODNAME);
    if (err) {
    dev_err(&pdev.dev, "pcim_request_all_regions failed\n");
    goto err_disable_device;
    }
    nexus.bar0 = pcim_iomap(pdev, 0, pci_resource_len(pdev, 0));
    if (!nexus.bar0) {
    err = -ENOMEM;
    goto err_disable_device;
    }
    i = 0;
    device_for_each_child_node_scoped(&pdev.dev, fwn) {
    struct resource r;
    struct mii_bus *mii_bus;
    struct cavium_mdiobus *bus;
    union cvmx_smix_en smi_en;
// If it is not an OF node we cannot handle it yet, so
// exit the loop.
//
    node = to_of_node(fwn);
    if (!node)
    break;
    err = of_address_to_resource(node, 0, &r);
    if (err) {
    dev_err(&pdev.dev,
    "Couldn't translate address for \"%pOFn\"\n",
    node);
    break;
    }
    mii_bus = devm_mdiobus_alloc_size(&pdev.dev, sizeof(*bus));
    if (!mii_bus)
    break;
    bus = mii_bus.priv;
    bus.mii_bus = mii_bus;
    nexus.buses[i] = bus;
    i++;
    bus.register_base = nexus.bar0 +
    r.start - pci_resource_start(pdev, 0);
    smi_en.u64 = 0;
    smi_en.s.en = 1;
    oct_mdio_writeq(smi_en.u64, bus.register_base + SMI_EN);
    bus.mii_bus.name = KBUILD_MODNAME;
    snprintf(bus.mii_bus.id, MII_BUS_ID_SIZE, "%llx", r.start);
    bus.mii_bus.parent = &pdev.dev;
    bus.mii_bus.read = cavium_mdiobus_read_c22;
    bus.mii_bus.write = cavium_mdiobus_write_c22;
    bus.mii_bus.read_c45 = cavium_mdiobus_read_c45;
    bus.mii_bus.write_c45 = cavium_mdiobus_write_c45;
    err = of_mdiobus_register(bus.mii_bus, node);
    if (err)
    dev_err(&pdev.dev, "of_mdiobus_register failed\n");
    dev_info(&pdev.dev, "Added bus at %llx\n", r.start);
    if (i >= ARRAY_SIZE(nexus.buses))
    break;
    }
    return 0;
    err_disable_device:
    pci_set_drvdata(pdev, core::ptr::null_mut());
    return err;
    }
#[no_mangle]
unsafe extern "C" fn thunder_mdiobus_pci_remove(pdev: *mut pci_dev) {
    static void thunder_mdiobus_pci_remove(struct pci_dev *pdev)
    {
    int i;
    struct thunder_mdiobus_nexus *nexus = pci_get_drvdata(pdev);
    for (i = 0; i < ARRAY_SIZE(nexus.buses); i++) {
    struct cavium_mdiobus *bus = nexus.buses[i];
    if (!bus)
    continue;
    mdiobus_unregister(bus.mii_bus);
    oct_mdio_writeq(0, bus.register_base + SMI_EN);
    }
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }
    static const struct pci_device_id thunder_mdiobus_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, 0xa02b) },
    { 0, } /* End of table. */
    };
    MODULE_DEVICE_TABLE(pci, thunder_mdiobus_id_table);
    static struct pci_driver thunder_mdiobus_driver = {
    .name = KBUILD_MODNAME,
    .id_table = thunder_mdiobus_id_table,
    .probe = thunder_mdiobus_pci_probe,
    .remove = thunder_mdiobus_pci_remove,
    };
    module_pci_driver(thunder_mdiobus_driver);
    MODULE_DESCRIPTION("Cavium ThunderX MDIO bus driver");
    MODULE_LICENSE("GPL v2");
