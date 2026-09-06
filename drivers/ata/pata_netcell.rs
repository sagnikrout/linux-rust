//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_netcell.c
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
// pata_netcell.c - Netcell PATA driver
//
// (c) 2006 Red Hat
//

// No PIO or DMA methods needed for this device
    static unsigned int netcell_read_id(struct ata_device *adev,
    struct ata_taskfile *tf, __le16 *id)
    {
    let mut err_mask: c_uint = ata_do_dev_read_id(adev, tf, id);
// Firmware forgets to mark words 85-87 valid
    if (err_mask == 0)
    id[ATA_ID_CSF_DEFAULT] |= cpu_to_le16(0x4000);
    return err_mask;
    }
    static const struct scsi_host_template netcell_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations netcell_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .cable_detect	= ata_cable_80wire,
    .read_id	= netcell_read_id,
    };
//
// netcell_init_one - Register Netcell ATA PCI device with kernel services
// @pdev: PCI device to register
// @ent: Entry in netcell_pci_tbl matching with @pdev
//
// Called from kernel PCI layer.
//
// LOCKING:
// Inherited from PCI layer (may sleep).
//
// RETURNS:
// Zero on success, or -ERRNO value.
//
#[no_mangle]
unsafe extern "C" fn netcell_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int netcell_init_one (struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    static const struct ata_port_info info = {
    .flags		= ATA_FLAG_SLAVE_POSS,
// Actually we don't really care about these as the
    firmware deals with it */
    .pio_mask	= ATA_PIO4,
    .mwdma_mask	= ATA_MWDMA2,
    .udma_mask 	= ATA_UDMA5, /* UDMA 133 */
    .port_ops	= &netcell_ops,
    };
    const struct ata_port_info *port_info[] = { &info, core::ptr::null_mut() };
    int rc;
    ata_print_version_once(&pdev.dev, DRV_VERSION);
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
// Any chip specific setup/optimisation/messages here
    ata_pci_bmdma_clear_simplex(pdev);
// And let the library code do the work
    return ata_pci_bmdma_init_one(pdev, port_info, &netcell_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id netcell_pci_tbl[] = {
    { PCI_VDEVICE(NETCELL, PCI_DEVICE_ID_REVOLUTION), },
    { }	/* terminate list */
    };
    static struct pci_driver netcell_pci_driver = {
    .name			= DRV_NAME,
    .id_table		= netcell_pci_tbl,
    .probe			= netcell_init_one,
    .remove			= ata_pci_remove_one,

    .suspend		= ata_pci_device_suspend,
    .resume			= ata_pci_device_resume,

    };
    module_pci_driver(netcell_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("SCSI low-level driver for Netcell PATA RAID");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, netcell_pci_tbl);
    MODULE_VERSION(DRV_VERSION);
