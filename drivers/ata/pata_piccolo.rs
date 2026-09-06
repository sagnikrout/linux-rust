//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_piccolo.c
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


//
// pata_piccolo.c - Toshiba Piccolo PATA/SATA controller driver.
//
// This is basically an update to ata_generic.c to add Toshiba Piccolo support
// then split out to keep ata_generic "clean".
//
// Copyright 2005 Red Hat Inc, all rights reserved.
//
// Elements from ide/pci/generic.c
// Copyright (C) 2001-2002	Andre Hedrick <andre@linux-ide.org>
// Portions (C) Copyright 2002  Red Hat Inc <alan@redhat.com>
//
// May be copied or modified under the terms of the GNU General Public License
//
// The timing data tables/programming info are courtesy of the NetBSD driver
//

#[no_mangle]
unsafe extern "C" fn tosh_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void tosh_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u16 pio[6] = {	/* For reg 0x50 low word & E088 */
    0x0566, 0x0433, 0x0311, 0x0201, 0x0200, 0x0100
    };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u16 conf;
    pci_read_config_word(pdev, 0x50, &conf);
    conf &= 0xE088;
    conf |= pio[adev.pio_mode - XFER_PIO_0];
    pci_write_config_word(pdev, 0x50, conf);
    }
#[no_mangle]
unsafe extern "C" fn tosh_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void tosh_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u32 conf;
    pci_read_config_dword(pdev, 0x5C, &conf);
    conf &= 0x78FFE088;	/* Keep the other bits */
    if (adev.dma_mode >= XFER_UDMA_0) {
    let mut udma: c_int = adev.dma_mode - XFER_UDMA_0;
    conf |= 0x80000000;
    conf |= (udma + 2) << 28;
    conf |= (2 - udma) * 0x111;	/* spread into three nibbles */
    } else {
    static const u32 mwdma[4] = {
    0x0655, 0x0200, 0x0200, 0x0100
    };
    conf |= mwdma[adev.dma_mode - XFER_MW_DMA_0];
    }
    pci_write_config_dword(pdev, 0x5C, conf);
    }
    static const struct scsi_host_template tosh_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations tosh_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .cable_detect	= ata_cable_unknown,
    .set_piomode	= tosh_set_piomode,
    .set_dmamode	= tosh_set_dmamode
    };
//
// ata_tosh_init_one		-	attach generic IDE
// @dev: PCI device found
// @id: match entry
//
// Called each time a matching IDE interface is found. We check if the
// interface is one we wish to claim and if so we perform any chip
// specific hacks then let the ATA layer do the heavy lifting.
//
#[no_mangle]
unsafe extern "C" fn ata_tosh_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int ata_tosh_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO5,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA2,
    .port_ops = &tosh_port_ops
    };
    const struct ata_port_info *ppi[] = { &info, &ata_dummy_port_info };
// Just one port for the moment
    return ata_pci_bmdma_init_one(dev, ppi, &tosh_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id ata_tosh[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA,PCI_DEVICE_ID_TOSHIBA_PICCOLO_1), },
    { PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA,PCI_DEVICE_ID_TOSHIBA_PICCOLO_2),  },
    { PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA,PCI_DEVICE_ID_TOSHIBA_PICCOLO_3),  },
    { PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA,PCI_DEVICE_ID_TOSHIBA_PICCOLO_5),  },
    { 0, },
    };
    static struct pci_driver ata_tosh_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= ata_tosh,
    .probe 		= ata_tosh_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(ata_tosh_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("Low level driver for Toshiba Piccolo ATA");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, ata_tosh);
    MODULE_VERSION(DRV_VERSION);
