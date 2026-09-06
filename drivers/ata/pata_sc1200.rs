//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_sc1200.c
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
// New ATA layer SC1200 driver		Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// TODO: Mode selection filtering
// TODO: Needs custom DMA cleanup code
//
// Based very heavily on
//
// linux/drivers/ide/pci/sc1200.c		Version 0.91	28-Jan-2003
//
// Copyright (C) 2000-2002		Mark Lord <mlord@pobox.com>
// May be copied or modified under the terms of the GNU General Public License
//
// Development of this chipset driver was funded
// by the nice folks at National Semiconductor.
//

pub const SC1200_REV_A: c_uint = 0x00;
pub const SC1200_REV_B1: c_uint = 0x01;
pub const SC1200_REV_B3: c_uint = 0x02;
pub const SC1200_REV_C1: c_uint = 0x03;
pub const SC1200_REV_D1: c_uint = 0x04;
//
// sc1200_clock	-	PCI clock
//
// Return the PCI bus clocking for the SC1200 chipset configuration
// in use. We return 0 for 33MHz 1 for 48MHz and 2 for 66Mhz
//
#[no_mangle]
unsafe extern "C" fn sc1200_clock() -> c_int {
    static int sc1200_clock(void)
    {
// Magic registers that give us the chipset data
    let mut chip_id: u8 = inb(0x903C);
    let mut silicon_rev: u8 = inb(0x903D);
    u16 pci_clock;
    if (chip_id == 0x04 && silicon_rev < SC1200_REV_B1)
    return 0;	/* 33 MHz mode */
// Clock generator configuration 0x901E its 8/9 are the PCI clocking
    0/3 is 33Mhz 1 is 48 2 is 66 */
    pci_clock = inw(0x901E);
    pci_clock >>= 8;
    pci_clock &= 0x03;
    if (pci_clock == 3)
    pci_clock = 0;
    return pci_clock;
    }
//
// sc1200_set_piomode		-	PIO setup
// @ap: ATA interface
// @adev: device on the interface
//
// Set our PIO requirements. This is fairly simple on the SC1200
//
#[no_mangle]
unsafe extern "C" fn sc1200_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void sc1200_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u32 pio_timings[4][5] = {
// format0, 33Mhz
    { 0x00009172, 0x00012171, 0x00020080, 0x00032010, 0x00040010 },
// format1, 33Mhz
    { 0xd1329172, 0x71212171, 0x30200080, 0x20102010, 0x00100010 },
// format1, 48Mhz
    { 0xfaa3f4f3, 0xc23232b2, 0x513101c1, 0x31213121, 0x10211021 },
// format1, 66Mhz
    { 0xfff4fff4, 0xf35353d3, 0x814102f1, 0x42314231, 0x11311131 }
    };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u32 format;
    let mut reg: c_uint = 0x40 + 0x10 * ap.port_no;
    let mut mode: c_int = adev.pio_mode - XFER_PIO_0;
    pci_read_config_dword(pdev, reg + 4, &format);
    format >>= 31;
    format += sc1200_clock();
    pci_write_config_dword(pdev, reg + 8 * adev.devno,
    pio_timings[format][mode]);
    }
//
// sc1200_set_dmamode		-	DMA timing setup
// @ap: ATA interface
// @adev: Device being configured
//
// We cannot mix MWDMA and UDMA without reloading timings each switch
// master to slave.
//
#[no_mangle]
unsafe extern "C" fn sc1200_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void sc1200_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u32 udma_timing[3][3] = {
    { 0x00921250, 0x00911140, 0x00911030 },
    { 0x00932470, 0x00922260, 0x00922140 },
    { 0x009436A1, 0x00933481, 0x00923261 }
    };
    static const u32 mwdma_timing[3][3] = {
    { 0x00077771, 0x00012121, 0x00002020 },
    { 0x000BBBB2, 0x00024241, 0x00013131 },
    { 0x000FFFF3, 0x00035352, 0x00015151 }
    };
    let mut clock: c_int = sc1200_clock();
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut reg: c_uint = 0x40 + 0x10 * ap.port_no;
    let mut mode: c_int = adev.dma_mode;
    u32 format;
    if (mode >= XFER_UDMA_0)
    format = udma_timing[clock][mode - XFER_UDMA_0];
    else
    format = mwdma_timing[clock][mode - XFER_MW_DMA_0];
    if (adev.devno == 0) {
    u32 timings;
    pci_read_config_dword(pdev, reg + 4, &timings);
    timings &= 0x80000000UL;
    timings |= format;
    pci_write_config_dword(pdev, reg + 4, timings);
    } else
    pci_write_config_dword(pdev, reg + 12, format);
    }
//
// sc1200_qc_issue		-	command issue
// @qc: command pending
//
// Called when the libata layer is about to issue a command. We wrap
// this interface so that we can load the correct ATA timings if
// necessary.  Specifically we have a problem that there is only
// one MWDMA/UDMA bit.
//
#[no_mangle]
unsafe extern "C" fn sc1200_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    static unsigned int sc1200_qc_issue(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_device *adev = qc.dev;
    struct ata_device *prev = ap.private_data;
// See if the DMA settings could be wrong
    if (ata_dma_enabled(adev) && adev != prev && prev != core::ptr::null_mut()) {
// Maybe, but do the channels match MWDMA/UDMA ?
    if ((ata_using_udma(adev) && !ata_using_udma(prev)) ||
    (ata_using_udma(prev) && !ata_using_udma(adev)))
// Switch the mode bits
    sc1200_set_dmamode(ap, adev);
    }
    return ata_bmdma_qc_issue(qc);
    }
//
// sc1200_qc_defer	-	implement serialization
// @qc: command
//
// Serialize command issue on this controller.
//
#[no_mangle]
unsafe extern "C" fn sc1200_qc_defer(qc: *mut ata_queued_cmd) -> c_int {
    static int sc1200_qc_defer(struct ata_queued_cmd *qc)
    {
    struct ata_host *host = qc.ap.host;
    struct ata_port *alt = host.ports[1 ^ qc.ap.port_no];
    int rc;
// First apply the usual rules
    rc = ata_std_qc_defer(qc);
    if (rc != 0)
    return rc;
// Now apply serialization rules. Only allow a command if the
    other channel state machine is idle */
    if (alt && alt.qc_active)
    return	ATA_DEFER_PORT;
    return 0;
    }
    static const struct scsi_host_template sc1200_sht = {
    ATA_BASE_SHT(DRV_NAME),
    .sg_tablesize	= LIBATA_DUMB_MAX_PRD,
    .dma_boundary	= ATA_DMA_BOUNDARY,
    };
    static struct ata_port_operations sc1200_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .qc_prep 	= ata_bmdma_dumb_qc_prep,
    .qc_issue	= sc1200_qc_issue,
    .qc_defer	= sc1200_qc_defer,
    .cable_detect	= ata_cable_40wire,
    .set_piomode	= sc1200_set_piomode,
    .set_dmamode	= sc1200_set_dmamode,
    };
//
// sc1200_init_one		-	Initialise an SC1200
// @dev: PCI device
// @id: Entry in match table
//
// Just throw the needed data at the libata helper and it does all
// our work.
//
#[no_mangle]
unsafe extern "C" fn sc1200_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int sc1200_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA2,
    .port_ops = &sc1200_port_ops
    };
    const struct ata_port_info *ppi[] = { &info, core::ptr::null_mut() };
    return ata_pci_bmdma_init_one(dev, ppi, &sc1200_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id sc1200[] = {
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_SCx200_IDE), },
    { },
    };
    static struct pci_driver sc1200_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= sc1200,
    .probe 		= sc1200_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(sc1200_pci_driver);
    MODULE_AUTHOR("Alan Cox, Mark Lord");
    MODULE_DESCRIPTION("low-level driver for the NS/AMD SC1200");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, sc1200);
    MODULE_VERSION(DRV_VERSION);
