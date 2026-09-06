//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_triflex.c
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
// pata_triflex.c 	- Compaq PATA for new ATA layer
// (C) 2005 Red Hat Inc
// Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// based upon
//
// triflex.c
//
// IDE Chipset driver for the Compaq TriFlex IDE controller.
//
// Known to work with the Compaq Workstation 5x00 series.
//
// Copyright (C) 2002 Hewlett-Packard Development Group, L.P.
// Author: Torben Mathiasen <torben.mathiasen@hp.com>
//
// Loosely based on the piix & svwks drivers.
//
// Documentation:
// Not publicly available.
//

//
// triflex_prereset		-	probe begin
// @link: ATA link
// @deadline: deadline jiffies for the operation
//
// Set up cable type and use generic probe init
//
#[no_mangle]
unsafe extern "C" fn triflex_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static int triflex_prereset(struct ata_link *link, unsigned long deadline)
    {
    static const struct pci_bits triflex_enable_bits[] = {
    { 0x80, 1, 0x01, 0x01 },
    { 0x80, 1, 0x02, 0x02 }
    };
    struct ata_port *ap = link.ap;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    if (!pci_test_config_bits(pdev, &triflex_enable_bits[ap.port_no]))
    return -ENOENT;
    return ata_sff_prereset(link, deadline);
    }
//
// triflex_load_timing		-	timing configuration
// @ap: ATA interface
// @adev: Device on the bus
// @speed: speed to configure
//
// The Triflex has one set of timings per device per channel. This
// means we must do some switching. As the PIO and DMA timings don't
// match we have to do some reloading unlike PIIX devices where tuning
// tricks can avoid it.
//
#[no_mangle]
unsafe extern "C" fn triflex_load_timing(ap: *mut ata_port, adev: *mut ata_device, speed: c_int) {
    static void triflex_load_timing(struct ata_port *ap, struct ata_device *adev, int speed)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut timing: u32 = 0;
    u32 triflex_timing, old_triflex_timing;
    let mut channel_offset: c_int = ap.port_no ? 0x74: 0x70;
    let mut is_slave: c_uint = (adev.devno != 0);
    pci_read_config_dword(pdev, channel_offset, &old_triflex_timing);
    triflex_timing = old_triflex_timing;
    switch(speed)
    {
    case XFER_MW_DMA_2:
    timing = 0x0103;break;
    case XFER_MW_DMA_1:
    timing = 0x0203;break;
    case XFER_MW_DMA_0:
    timing = 0x0808;break;
    case XFER_SW_DMA_2:
    case XFER_SW_DMA_1:
    case XFER_SW_DMA_0:
    timing = 0x0F0F;break;
    case XFER_PIO_4:
    timing = 0x0202;break;
    case XFER_PIO_3:
    timing = 0x0204;break;
    case XFER_PIO_2:
    timing = 0x0404;break;
    case XFER_PIO_1:
    timing = 0x0508;break;
    case XFER_PIO_0:
    timing = 0x0808;break;
    default:
    BUG();
    }
    triflex_timing &= ~ (0xFFFF << (16 * is_slave));
    triflex_timing |= (timing << (16 * is_slave));
    if (triflex_timing != old_triflex_timing)
    pci_write_config_dword(pdev, channel_offset, triflex_timing);
    }
//
// triflex_set_piomode	-	set initial PIO mode data
// @ap: ATA interface
// @adev: ATA device
//
// Use the timing loader to set up the PIO mode. We have to do this
// because DMA start/stop will only be called once DMA occurs. If there
// has been no DMA then the PIO timings are still needed.
//
#[no_mangle]
unsafe extern "C" fn triflex_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void triflex_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    triflex_load_timing(ap, adev, adev.pio_mode);
    }
//
// triflex_bmdma_start	-	DMA start callback
// @qc: Command in progress
//
// Usually drivers set the DMA timing at the point the set_dmamode call
// is made. Triflex however requires we load new timings on the
// transition or keep matching PIO/DMA pairs (ie MWDMA2/PIO4 etc).
// We load the DMA timings just before starting DMA and then restore
// the PIO timing when the DMA is finished.
//
#[no_mangle]
unsafe extern "C" fn triflex_bmdma_start(qc: *mut ata_queued_cmd) {
    static void triflex_bmdma_start(struct ata_queued_cmd *qc)
    {
    triflex_load_timing(qc.ap, qc.dev, qc.dev.dma_mode);
    ata_bmdma_start(qc);
    }
//
// triflex_bmdma_stop	-	DMA stop callback
// @qc: ATA command
//
// We loaded new timings in dma_start, as a result we need to restore
// the PIO timings in dma_stop so that the next command issue gets the
// right clock values.
//
#[no_mangle]
unsafe extern "C" fn triflex_bmdma_stop(qc: *mut ata_queued_cmd) {
    static void triflex_bmdma_stop(struct ata_queued_cmd *qc)
    {
    ata_bmdma_stop(qc);
    triflex_load_timing(qc.ap, qc.dev, qc.dev.pio_mode);
    }
    static const struct scsi_host_template triflex_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations triflex_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .bmdma_start 	= triflex_bmdma_start,
    .bmdma_stop	= triflex_bmdma_stop,
    .cable_detect	= ata_cable_40wire,
    .set_piomode	= triflex_set_piomode,
    .reset.prereset	= triflex_prereset,
    };
#[no_mangle]
unsafe extern "C" fn triflex_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int triflex_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .port_ops = &triflex_port_ops
    };
    const struct ata_port_info *ppi[] = { &info, core::ptr::null_mut() };
    ata_print_version_once(&dev.dev, DRV_VERSION);
    return ata_pci_bmdma_init_one(dev, ppi, &triflex_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id triflex[] = {
    { PCI_VDEVICE(COMPAQ, PCI_DEVICE_ID_COMPAQ_TRIFLEX_IDE), },
    { },
    };

#[no_mangle]
unsafe extern "C" fn triflex_ata_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> c_int {
    static int triflex_ata_pci_device_suspend(struct pci_dev *pdev, pm_message_t mesg)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    ata_host_suspend(host, mesg);
//
// We must not disable or powerdown the device.
// APM bios refuses to suspend if IDE is not accessible.
//
    pci_save_state(pdev);
    return 0;
    }

    static struct pci_driver triflex_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= triflex,
    .probe 		= triflex_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= triflex_ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(triflex_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("low-level driver for Compaq Triflex");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, triflex);
    MODULE_VERSION(DRV_VERSION);
