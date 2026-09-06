//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/dmx3191d.c
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
    dmx3191d.c - driver for the Domex DMX3191D SCSI card.
    Copyright (C) 2000 by Massimo Piccioni <dafastidio@libero.it>
    Portions Copyright (C) 2004 by Christoph Hellwig <hch@lst.de>
    Based on the generic NCR5380 driver by Drew Eckhardt et al.
//

//
// Definitions for the generic 5380 driver.
//

pub const DMX3191D_REGION_LEN: c_int = 8;
    static const struct scsi_host_template dmx3191d_driver_template = {
    .module			= THIS_MODULE,
    .proc_name		= DMX3191D_DRIVER_NAME,
    .name			= "Domex DMX3191D",
    .info			= NCR5380_info,
    .queuecommand		= NCR5380_queue_command,
    .eh_abort_handler	= NCR5380_abort,
    .eh_host_reset_handler	= NCR5380_host_reset,
    .can_queue		= 32,
    .this_id		= 7,
    .sg_tablesize		= SG_ALL,
    .cmd_per_lun		= 2,
    .dma_boundary		= PAGE_SIZE - 1,
    .cmd_size		= sizeof(struct NCR5380_cmd),
    };
    static int dmx3191d_probe_one(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct Scsi_Host *shost;
    struct NCR5380_hostdata *hostdata;
    unsigned long io;
    let mut error: c_int = -ENODEV;
    if (pci_enable_device(pdev))
    goto out;
    io = pci_resource_start(pdev, 0);
    if (!request_region(io, DMX3191D_REGION_LEN, DMX3191D_DRIVER_NAME)) {
    printk(KERN_ERR "dmx3191: region 0x%lx-0x%lx already reserved\n",
    io, io + DMX3191D_REGION_LEN);
    goto out_disable_device;
    }
    shost = scsi_host_alloc(&dmx3191d_driver_template,
    sizeof(struct NCR5380_hostdata));
    if (!shost)
    goto out_release_region;
    hostdata = shost_priv(shost);
    hostdata.base = io;
// This card does not seem to raise an interrupt on pdev->irq.
// Steam-powered SCSI controllers run without an IRQ anyway.
//
    shost.irq = NO_IRQ;
    error = NCR5380_init(shost, 0);
    if (error)
    goto out_host_put;
    NCR5380_maybe_reset_bus(shost);
    pci_set_drvdata(pdev, shost);
    error = scsi_add_host(shost, &pdev.dev);
    if (error)
    goto out_exit;
    scsi_scan_host(shost);
    return 0;
    out_exit:
    NCR5380_exit(shost);
    out_host_put:
    scsi_host_put(shost);
    out_release_region:
    release_region(io, DMX3191D_REGION_LEN);
    out_disable_device:
    pci_disable_device(pdev);
    out:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn dmx3191d_remove_one(pdev: *mut pci_dev) {
    static void dmx3191d_remove_one(struct pci_dev *pdev)
    {
    struct Scsi_Host *shost = pci_get_drvdata(pdev);
    struct NCR5380_hostdata *hostdata = shost_priv(shost);
    let mut io: c_ulong = hostdata.base;
    scsi_remove_host(shost);
    NCR5380_exit(shost);
    scsi_host_put(shost);
    release_region(io, DMX3191D_REGION_LEN);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id dmx3191d_pci_tbl[] = {
    {PCI_VENDOR_ID_DOMEX, PCI_DEVICE_ID_DOMEX_DMX3191D,
    PCI_ANY_ID, PCI_ANY_ID, 0, 0, 4},
    { }
    };
    MODULE_DEVICE_TABLE(pci, dmx3191d_pci_tbl);
    static struct pci_driver dmx3191d_pci_driver = {
    .name		= DMX3191D_DRIVER_NAME,
    .id_table	= dmx3191d_pci_tbl,
    .probe		= dmx3191d_probe_one,
    .remove		= dmx3191d_remove_one,
    };
    module_pci_driver(dmx3191d_pci_driver);
    MODULE_AUTHOR("Massimo Piccioni <dafastidio@libero.it>");
    MODULE_DESCRIPTION("Domex DMX3191D SCSI driver");
    MODULE_LICENSE("GPL");
