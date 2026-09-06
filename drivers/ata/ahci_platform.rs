//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_platform.c
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
// AHCI SATA platform driver
//
// Copyright 2004-2005  Red Hat, Inc.
// Jeff Garzik <jgarzik@pobox.com>
// Copyright 2010  MontaVista Software, LLC.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

    static const struct ata_port_info ahci_port_info = {
    .flags		= AHCI_FLAG_COMMON,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_platform_ops,
    };
    static const struct ata_port_info ahci_port_info_nolpm = {
    .flags		= AHCI_FLAG_COMMON | ATA_FLAG_NO_LPM,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_platform_ops,
    };
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
#[no_mangle]
unsafe extern "C" fn ahci_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ahci_host_priv *hpriv;
    const struct ata_port_info *port;
    int rc;
    hpriv = ahci_platform_get_resources(pdev,
    AHCI_PLATFORM_GET_RESETS);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    rc = ahci_platform_enable_resources(hpriv);
    if (rc)
    return rc;
    if (device_is_compatible(dev, "hisilicon,hisi-ahci"))
    hpriv.flags |= AHCI_HFLAG_NO_FBS | AHCI_HFLAG_NO_NCQ;
    port = device_get_match_data(dev);
    if (!port)
    port = &ahci_port_info;
    rc = ahci_platform_init_host(pdev, hpriv, port,
    &ahci_platform_sht);
    if (rc)
    goto disable_resources;
    return 0;
    disable_resources:
    ahci_platform_disable_resources(hpriv);
    return rc;
    }
    static SIMPLE_DEV_PM_OPS(ahci_pm_ops, ahci_platform_suspend,
    ahci_platform_resume);
    static const struct of_device_id ahci_of_match[] = {
    { .compatible = "generic-ahci", },
// Keep the following compatibles for device tree compatibility
    { .compatible = "ibm,476gtr-ahci", },
    { .compatible = "hisilicon,hisi-ahci", },
    { .compatible = "cavium,octeon-7130-ahci", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_of_match);
    static const struct acpi_device_id ahci_acpi_match[] = {
    { .id = "APMC0D33", .driver_data = (unsigned long)&ahci_port_info_nolpm },
    { ACPI_DEVICE_CLASS(PCI_CLASS_STORAGE_SATA_AHCI, 0xffffff) },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ahci_acpi_match);
    static struct platform_driver ahci_driver = {
    .probe = ahci_probe,
    .remove = ata_platform_remove_one,
    .shutdown = ahci_platform_shutdown,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_of_match,
    .acpi_match_table = ahci_acpi_match,
    .pm = &ahci_pm_ops,
    },
    };
    module_platform_driver(ahci_driver);
    MODULE_DESCRIPTION("AHCI SATA platform driver");
    MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ahci");
