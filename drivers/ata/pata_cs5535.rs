//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_cs5535.c
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
// pata-cs5535.c 	- CS5535 PATA for new ATA layer
// (C) 2005-2006 Red Hat Inc
// Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// based upon cs5535.c from AMD <Jens.Altmann@amd.com> as cleaned up and
// made readable and Linux style by Wolfgang Zuleger <wolfgang.zuleger@gmx.de>
// and Alexander Kiausch <alex.kiausch@t-online.de>
//
// Loosely based on the piix & svwks drivers.
//
// Documentation:
// Available from AMD web site.
// TODO
// Review errata to see if serializing is necessary
//

//
// The Geode (Aka Athlon GX now) uses an internal MSR based
// bus system for control. Demented but there you go.
//
pub const MSR_ATAC_BASE: c_uint = 0x51300000;

pub const ATAC_BM0_CMD_PRIM: c_uint = 0x00;
pub const ATAC_BM0_STS_PRIM: c_uint = 0x02;
pub const ATAC_BM0_PRD: c_uint = 0x04;
pub const CS5535_CABLE_DETECT: c_uint = 0x48;
//
// cs5535_cable_detect	-	detect cable type
// @ap: Port to detect on
//
// Perform cable detection for ATA66 capable cable. Return a libata
// cable type.
//
#[no_mangle]
unsafe extern "C" fn cs5535_cable_detect(ap: *mut ata_port) -> c_int {
    static int cs5535_cable_detect(struct ata_port *ap)
    {
    u8 cable;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    pci_read_config_byte(pdev, CS5535_CABLE_DETECT, &cable);
    if (cable & 1)
    return ATA_CBL_PATA80;
    else
    return ATA_CBL_PATA40;
    }
//
// cs5535_set_piomode		-	PIO setup
// @ap: ATA interface
// @adev: device on the interface
//
// Set our PIO requirements. The CS5535 is pretty clean about all this
//
#[no_mangle]
unsafe extern "C" fn cs5535_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void cs5535_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u16 pio_timings[5] = {
    0xF7F4, 0xF173, 0x8141, 0x5131, 0x1131
    };
    static const u16 pio_cmd_timings[5] = {
    0xF7F4, 0x53F3, 0x13F1, 0x5131, 0x1131
    };
    u32 reg, __maybe_unused dummy;
    struct ata_device *pair = ata_dev_pair(adev);
    let mut mode: c_int = adev.pio_mode - XFER_PIO_0;
    let mut cmdmode: c_int = mode;
// Command timing has to be for the lowest of the pair of devices
    if (pair) {
    let mut pairmode: c_int = pair.pio_mode - XFER_PIO_0;
    cmdmode = min(mode, pairmode);
// Write the other drive timing register if it changed
    if (cmdmode < pairmode)
    wrmsr(ATAC_CH0D0_PIO + 2 * pair.devno,
    pio_cmd_timings[cmdmode] << 16 | pio_timings[pairmode], 0);
    }
// Write the drive timing register
    wrmsr(ATAC_CH0D0_PIO + 2 * adev.devno,
    pio_cmd_timings[cmdmode] << 16 | pio_timings[mode], 0);
// Set the PIO "format 1" bit in the DMA timing register
    rdmsr(ATAC_CH0D0_DMA + 2 * adev.devno, reg, dummy);
    wrmsr(ATAC_CH0D0_DMA + 2 * adev.devno, reg | 0x80000000UL, 0);
    }
//
// cs5535_set_dmamode		-	DMA timing setup
// @ap: ATA interface
// @adev: Device being configured
//
#[no_mangle]
unsafe extern "C" fn cs5535_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void cs5535_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u32 udma_timings[5] = {
    0x7F7436A1, 0x7F733481, 0x7F723261, 0x7F713161, 0x7F703061
    };
    static const u32 mwdma_timings[3] = {
    0x7F0FFFF3, 0x7F035352, 0x7F024241
    };
    u32 reg, __maybe_unused dummy;
    let mut mode: c_int = adev.dma_mode;
    rdmsr(ATAC_CH0D0_DMA + 2 * adev.devno, reg, dummy);
    reg &= 0x80000000UL;
    if (mode >= XFER_UDMA_0)
    reg |= udma_timings[mode - XFER_UDMA_0];
    else
    reg |= mwdma_timings[mode - XFER_MW_DMA_0];
    wrmsr(ATAC_CH0D0_DMA + 2 * adev.devno, reg, 0);
    }
    static const struct scsi_host_template cs5535_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations cs5535_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .cable_detect	= cs5535_cable_detect,
    .set_piomode	= cs5535_set_piomode,
    .set_dmamode	= cs5535_set_dmamode,
    };
//
// cs5535_init_one		-	Initialise a CS5530
// @dev: PCI device
// @id: Entry in match table
//
// Install a driver for the newly found CS5530 companion chip. Most of
// this is just housekeeping. We have to set the chip up correctly and
// turn off various bits of emulation magic.
//
#[no_mangle]
unsafe extern "C" fn cs5535_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int cs5535_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA4,
    .port_ops = &cs5535_port_ops
    };
    const struct ata_port_info *ppi[] = { &info, &ata_dummy_port_info };
    return ata_pci_bmdma_init_one(dev, ppi, &cs5535_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id cs5535[] = {
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_CS5535_IDE), },
    { PCI_VDEVICE(AMD, PCI_DEVICE_ID_AMD_CS5535_IDE), },
    { },
    };
    static struct pci_driver cs5535_pci_driver = {
    .name		= DRV_NAME,
    .id_table	= cs5535,
    .probe 		= cs5535_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(cs5535_pci_driver);
    MODULE_AUTHOR("Alan Cox, Jens Altmann, Wolfgan Zuleger, Alexander Kiausch");
    MODULE_DESCRIPTION("low-level driver for the NS/AMD 5535");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, cs5535);
    MODULE_VERSION(DRV_VERSION);
