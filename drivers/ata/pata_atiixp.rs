//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_atiixp.c
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
// pata_atiixp.c 	- ATI PATA for new ATA layer
// (C) 2005 Red Hat Inc
// (C) 2009-2010 Bartlomiej Zolnierkiewicz
//
// Based on
//
// linux/drivers/ide/pci/atiixp.c	Version 0.01-bart2	Feb. 26, 2004
//
// Copyright (C) 2003 ATI Inc. <hyu@ati.com>
// Copyright (C) 2004 Bartlomiej Zolnierkiewicz
//

    enum {
    ATIIXP_IDE_PIO_TIMING	= 0x40,
    ATIIXP_IDE_MWDMA_TIMING	= 0x44,
    ATIIXP_IDE_PIO_CONTROL	= 0x48,
    ATIIXP_IDE_PIO_MODE	= 0x4a,
    ATIIXP_IDE_UDMA_CONTROL	= 0x54,
    ATIIXP_IDE_UDMA_MODE 	= 0x56
    };
    static const struct dmi_system_id attixp_cable_override_dmi_table[] = {
    {
// Board has onboard PATA<->SATA converters
    .ident = "MSI E350DM-E33",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "MSI"),
    DMI_MATCH(DMI_BOARD_NAME, "E350DM-E33(MS-7720)"),
    },
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn atiixp_cable_detect(ap: *mut ata_port) -> c_int {
    static int atiixp_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u8 udma;
    if (dmi_check_system(attixp_cable_override_dmi_table))
    return ATA_CBL_PATA40_SHORT;
// Hack from drivers/ide/pci. Really we want to know how to do the
    raw detection not play follow the bios mode guess */
    pci_read_config_byte(pdev, ATIIXP_IDE_UDMA_MODE + ap.port_no, &udma);
    if ((udma & 0x07) >= 0x04 || (udma & 0x70) >= 0x40)
    return  ATA_CBL_PATA80;
    return ATA_CBL_PATA40;
    }
    static DEFINE_SPINLOCK(atiixp_lock);
//
// atiixp_prereset	-	perform reset handling
// @link: ATA link
// @deadline: deadline jiffies for the operation
//
// Reset sequence checking enable bits to see which ports are
// active.
//
#[no_mangle]
unsafe extern "C" fn atiixp_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static int atiixp_prereset(struct ata_link *link, unsigned long deadline)
    {
    static const struct pci_bits atiixp_enable_bits[] = {
    { 0x48, 1, 0x01, 0x00 },
    { 0x48, 1, 0x08, 0x00 }
    };
    struct ata_port *ap = link.ap;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    if (!pci_test_config_bits(pdev, &atiixp_enable_bits[ap.port_no]))
    return -ENOENT;
    return ata_sff_prereset(link, deadline);
    }
//
// atiixp_set_pio_timing	-	set initial PIO mode data
// @ap: ATA interface
// @adev: ATA device
// @pio: Requested PIO
//
// Called by both the pio and dma setup functions to set the controller
// timings for PIO transfers. We must load both the mode number and
// timing values into the controller.
//
#[no_mangle]
unsafe extern "C" fn atiixp_set_pio_timing(ap: *mut ata_port, adev: *mut ata_device, pio: c_int) {
    static void atiixp_set_pio_timing(struct ata_port *ap, struct ata_device *adev, int pio)
    {
    static const u8 pio_timings[5] = { 0x5D, 0x47, 0x34, 0x22, 0x20 };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut dn: c_int = 2 * ap.port_no + adev.devno;
    let mut timing_shift: c_int = (16 * ap.port_no) + 8 * (adev.devno ^ 1);
    u32 pio_timing_data;
    u16 pio_mode_data;
    pci_read_config_word(pdev, ATIIXP_IDE_PIO_MODE, &pio_mode_data);
    pio_mode_data &= ~(0x7 << (4 * dn));
    pio_mode_data |= pio << (4 * dn);
    pci_write_config_word(pdev, ATIIXP_IDE_PIO_MODE, pio_mode_data);
    pci_read_config_dword(pdev, ATIIXP_IDE_PIO_TIMING, &pio_timing_data);
    pio_timing_data &= ~(0xFF << timing_shift);
    pio_timing_data |= (pio_timings[pio] << timing_shift);
    pci_write_config_dword(pdev, ATIIXP_IDE_PIO_TIMING, pio_timing_data);
    }
//
// atiixp_set_piomode	-	set initial PIO mode data
// @ap: ATA interface
// @adev: ATA device
//
// Called to do the PIO mode setup. We use a shared helper for this
// as the DMA setup must also adjust the PIO timing information.
//
#[no_mangle]
unsafe extern "C" fn atiixp_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void atiixp_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    unsigned long flags;
    spin_lock_irqsave(&atiixp_lock, flags);
    atiixp_set_pio_timing(ap, adev, adev.pio_mode - XFER_PIO_0);
    spin_unlock_irqrestore(&atiixp_lock, flags);
    }
//
// atiixp_set_dmamode	-	set initial DMA mode data
// @ap: ATA interface
// @adev: ATA device
//
// Called to do the DMA mode setup. We use timing tables for most
// modes but must tune an appropriate PIO mode to match.
//
#[no_mangle]
unsafe extern "C" fn atiixp_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void atiixp_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u8 mwdma_timings[5] = { 0x77, 0x21, 0x20 };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut dma: c_int = adev.dma_mode;
    let mut dn: c_int = 2 * ap.port_no + adev.devno;
    int wanted_pio;
    unsigned long flags;
    spin_lock_irqsave(&atiixp_lock, flags);
    if (adev.dma_mode >= XFER_UDMA_0) {
    u16 udma_mode_data;
    dma -= XFER_UDMA_0;
    pci_read_config_word(pdev, ATIIXP_IDE_UDMA_MODE, &udma_mode_data);
    udma_mode_data &= ~(0x7 << (4 * dn));
    udma_mode_data |= dma << (4 * dn);
    pci_write_config_word(pdev, ATIIXP_IDE_UDMA_MODE, udma_mode_data);
    } else {
    let mut timing_shift: c_int = (16 * ap.port_no) + 8 * (adev.devno ^ 1);
    u32 mwdma_timing_data;
    dma -= XFER_MW_DMA_0;
    pci_read_config_dword(pdev, ATIIXP_IDE_MWDMA_TIMING,
    &mwdma_timing_data);
    mwdma_timing_data &= ~(0xFF << timing_shift);
    mwdma_timing_data |= (mwdma_timings[dma] << timing_shift);
    pci_write_config_dword(pdev, ATIIXP_IDE_MWDMA_TIMING,
    mwdma_timing_data);
    }
//
// We must now look at the PIO mode situation. We may need to
// adjust the PIO mode to keep the timings acceptable
//
    if (adev.dma_mode >= XFER_MW_DMA_2)
    wanted_pio = 4;
#[no_mangle]
pub unsafe extern "C" fn if(XFER_MW_DMA_1: adev->dma_mode ==) -> else {
    else if (adev.dma_mode == XFER_MW_DMA_1)
    wanted_pio = 3;
#[no_mangle]
pub unsafe extern "C" fn if(XFER_MW_DMA_0: adev->dma_mode ==) -> else {
    else if (adev.dma_mode == XFER_MW_DMA_0)
    wanted_pio = 0;
    else BUG();
    if (adev.pio_mode != wanted_pio)
    atiixp_set_pio_timing(ap, adev, wanted_pio);
    spin_unlock_irqrestore(&atiixp_lock, flags);
    }
//
// atiixp_bmdma_start	-	DMA start callback
// @qc: Command in progress
//
// When DMA begins we need to ensure that the UDMA control
// register for the channel is correctly set.
//
// Note: The host lock held by the libata layer protects
// us from two channels both trying to set DMA bits at once
//
#[no_mangle]
unsafe extern "C" fn atiixp_bmdma_start(qc: *mut ata_queued_cmd) {
    static void atiixp_bmdma_start(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_device *adev = qc.dev;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut dn: c_int = (2 * ap.port_no) + adev.devno;
    u16 tmp16;
    pci_read_config_word(pdev, ATIIXP_IDE_UDMA_CONTROL, &tmp16);
    if (ata_using_udma(adev))
    tmp16 |= (1 << dn);
    else
    tmp16 &= ~(1 << dn);
    pci_write_config_word(pdev, ATIIXP_IDE_UDMA_CONTROL, tmp16);
    ata_bmdma_start(qc);
    }
//
// atiixp_bmdma_stop	-	DMA stop callback
// @qc: Command in progress
//
// DMA has completed. Clear the UDMA flag as the next operations will
// be PIO ones not UDMA data transfer.
//
// Note: The host lock held by the libata layer protects
// us from two channels both trying to set DMA bits at once
//
#[no_mangle]
unsafe extern "C" fn atiixp_bmdma_stop(qc: *mut ata_queued_cmd) {
    static void atiixp_bmdma_stop(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut dn: c_int = (2 * ap.port_no) + qc.dev.devno;
    u16 tmp16;
    pci_read_config_word(pdev, ATIIXP_IDE_UDMA_CONTROL, &tmp16);
    tmp16 &= ~(1 << dn);
    pci_write_config_word(pdev, ATIIXP_IDE_UDMA_CONTROL, tmp16);
    ata_bmdma_stop(qc);
    }
    static const struct scsi_host_template atiixp_sht = {
    ATA_BASE_SHT(DRV_NAME),
    .sg_tablesize		= LIBATA_DUMB_MAX_PRD,
    .dma_boundary		= ATA_DMA_BOUNDARY,
    };
    static struct ata_port_operations atiixp_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .qc_prep 	= ata_bmdma_dumb_qc_prep,
    .bmdma_start 	= atiixp_bmdma_start,
    .bmdma_stop	= atiixp_bmdma_stop,
    .reset.prereset	= atiixp_prereset,
    .cable_detect	= atiixp_cable_detect,
    .set_piomode	= atiixp_set_piomode,
    .set_dmamode	= atiixp_set_dmamode,
    };
#[no_mangle]
unsafe extern "C" fn atiixp_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int atiixp_init_one(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA12_ONLY,
    .udma_mask = ATA_UDMA5,
    .port_ops = &atiixp_port_ops
    };
    const struct ata_port_info *ppi[] = { &info, &info };
// SB600 doesn't have secondary port wired
    if (pdev.device == PCI_DEVICE_ID_ATI_IXP600_IDE)
    ppi[1] = &ata_dummy_port_info;
    return ata_pci_bmdma_init_one(pdev, ppi, &atiixp_sht, core::ptr::null_mut(),
    ATA_HOST_PARALLEL_SCAN);
    }
    static const struct pci_device_id atiixp[] = {
    { PCI_VDEVICE(ATI, PCI_DEVICE_ID_ATI_IXP200_IDE), },
    { PCI_VDEVICE(ATI, PCI_DEVICE_ID_ATI_IXP300_IDE), },
    { PCI_VDEVICE(ATI, PCI_DEVICE_ID_ATI_IXP400_IDE), },
    { PCI_VDEVICE(ATI, PCI_DEVICE_ID_ATI_IXP600_IDE), },
    { PCI_VDEVICE(ATI, PCI_DEVICE_ID_ATI_IXP700_IDE), },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_HUDSON2_IDE), },
    { },
    };
    static struct pci_driver atiixp_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= atiixp,
    .probe 		= atiixp_init_one,
    .remove		= ata_pci_remove_one,

    .resume		= ata_pci_device_resume,
    .suspend	= ata_pci_device_suspend,

    };
    module_pci_driver(atiixp_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("low-level driver for ATI IXP200/300/400");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, atiixp);
    MODULE_VERSION(DRV_VERSION);
