//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_efar.c
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
// pata_efar.c - EFAR PIIX clone controller driver
//
// (C) 2005 Red Hat
// (C) 2009-2010 Bartlomiej Zolnierkiewicz
//
// Some parts based on ata_piix.c by Jeff Garzik and others.
//
// The EFAR is a PIIX4 clone with UDMA66 support. Unlike the later
// Intel ICH controllers the EFAR widened the UDMA mode register bits
// and doesn't require the funky clock selection.
//

//
// efar_pre_reset	-	Enable bits
// @link: ATA link
// @deadline: deadline jiffies for the operation
//
// Perform cable detection for the EFAR ATA interface. This is
// different to the PIIX arrangement
//
#[no_mangle]
unsafe extern "C" fn efar_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static int efar_pre_reset(struct ata_link *link, unsigned long deadline)
    {
    static const struct pci_bits efar_enable_bits[] = {
    { 0x41U, 1U, 0x80UL, 0x80UL },	/* port 0 */
    { 0x43U, 1U, 0x80UL, 0x80UL },	/* port 1 */
    };
    struct ata_port *ap = link.ap;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    if (!pci_test_config_bits(pdev, &efar_enable_bits[ap.port_no]))
    return -ENOENT;
    return ata_sff_prereset(link, deadline);
    }
//
// efar_cable_detect	-	check for 40/80 pin
// @ap: Port
//
// Perform cable detection for the EFAR ATA interface. This is
// different to the PIIX arrangement
//
#[no_mangle]
unsafe extern "C" fn efar_cable_detect(ap: *mut ata_port) -> c_int {
    static int efar_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u8 tmp;
    pci_read_config_byte(pdev, 0x47, &tmp);
    if (tmp & (2 >> ap.port_no))
    return ATA_CBL_PATA40;
    return ATA_CBL_PATA80;
    }
    static DEFINE_SPINLOCK(efar_lock);
//
// efar_set_piomode - Initialize host controller PATA PIO timings
// @ap: Port whose timings we are configuring
// @adev: Device to program
//
// Set PIO mode for device, in host controller PCI config space.
//
// LOCKING:
// None (inherited from caller).
//
#[no_mangle]
unsafe extern "C" fn efar_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void efar_set_piomode (struct ata_port *ap, struct ata_device *adev)
    {
    let mut pio: c_uint = adev.pio_mode - XFER_PIO_0;
    struct pci_dev *dev	= to_pci_dev(ap.host.dev);
    let mut master_port: c_uint = ap.port_no ? 0x42 : 0x40;
    unsigned long flags;
    u16 master_data;
    u8 udma_enable;
    let mut control: c_int = 0;
//
// See Intel Document 298600-004 for the timing programing rules
// for PIIX/ICH. The EFAR is a clone so very similar
//
    static const	 /* ISP  RTC */
    u8 timings[][2]	= { { 0, 0 },
    { 0, 0 },
    { 1, 0 },
    { 2, 1 },
    { 2, 3 }, };
    if (pio > 1)
    control |= 1;	/* TIME */
    if (ata_pio_need_iordy(adev))	/* PIO 3/4 require IORDY */
    control |= 2;	/* IE */
// Intel specifies that the prefetch/posting is for disk only
    if (adev.class == ATA_DEV_ATA)
    control |= 4;	/* PPE */
    spin_lock_irqsave(&efar_lock, flags);
    pci_read_config_word(dev, master_port, &master_data);
// Set PPE, IE, and TIME as appropriate
    if (adev.devno == 0) {
    master_data &= 0xCCF0;
    master_data |= control;
    master_data |= (timings[pio][0] << 12) |
    (timings[pio][1] << 8);
    } else {
    let mut shift: c_int = 4 * ap.port_no;
    u8 slave_data;
    master_data &= 0xFF0F;
    master_data |= (control << 4);
// Slave timing in separate register
    pci_read_config_byte(dev, 0x44, &slave_data);
    slave_data &= ap.port_no ? 0x0F : 0xF0;
    slave_data |= ((timings[pio][0] << 2) | timings[pio][1]) << shift;
    pci_write_config_byte(dev, 0x44, slave_data);
    }
    master_data |= 0x4000;	/* Ensure SITRE is set */
    pci_write_config_word(dev, master_port, master_data);
    pci_read_config_byte(dev, 0x48, &udma_enable);
    udma_enable &= ~(1 << (2 * ap.port_no + adev.devno));
    pci_write_config_byte(dev, 0x48, udma_enable);
    spin_unlock_irqrestore(&efar_lock, flags);
    }
//
// efar_set_dmamode - Initialize host controller PATA DMA timings
// @ap: Port whose timings we are configuring
// @adev: Device to program
//
// Set UDMA/MWDMA mode for device, in host controller PCI config space.
//
// LOCKING:
// None (inherited from caller).
//
#[no_mangle]
unsafe extern "C" fn efar_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void efar_set_dmamode (struct ata_port *ap, struct ata_device *adev)
    {
    struct pci_dev *dev	= to_pci_dev(ap.host.dev);
    let mut master_port: u8 = ap.port_no ? 0x42 : 0x40;
    u16 master_data;
    let mut speed: u8 = adev.dma_mode;
    let mut devid: c_int = adev.devno + 2 * ap.port_no;
    unsigned long flags;
    u8 udma_enable;
    static const	 /* ISP  RTC */
    u8 timings[][2]	= { { 0, 0 },
    { 0, 0 },
    { 1, 0 },
    { 2, 1 },
    { 2, 3 }, };
    spin_lock_irqsave(&efar_lock, flags);
    pci_read_config_word(dev, master_port, &master_data);
    pci_read_config_byte(dev, 0x48, &udma_enable);
    if (speed >= XFER_UDMA_0) {
    let mut udma: c_uint = adev.dma_mode - XFER_UDMA_0;
    u16 udma_timing;
    udma_enable |= (1 << devid);
// Load the UDMA mode number
    pci_read_config_word(dev, 0x4A, &udma_timing);
    udma_timing &= ~(7 << (4 * devid));
    udma_timing |= udma << (4 * devid);
    pci_write_config_word(dev, 0x4A, udma_timing);
    } else {
//
// MWDMA is driven by the PIO timings. We must also enable
// IORDY unconditionally along with TIME1. PPE has already
// been set when the PIO timing was set.
//
    let mut mwdma: c_uint = adev.dma_mode - XFER_MW_DMA_0;
    unsigned int control;
    u8 slave_data;
    const unsigned int needed_pio[3] = {
    XFER_PIO_0, XFER_PIO_3, XFER_PIO_4
    };
    let mut pio: c_int = needed_pio[mwdma] - XFER_PIO_0;
    control = 3;	/* IORDY|TIME1 */
// If the drive MWDMA is faster than it can do PIO then
    we must force PIO into PIO0 */
    if (adev.pio_mode < needed_pio[mwdma])
// Enable DMA timing only
    control |= 8;	/* PIO cycles in PIO0 */
    if (adev.devno) {	/* Slave */
    master_data &= 0xFF4F;  /* Mask out IORDY|TIME1|DMAONLY */
    master_data |= control << 4;
    pci_read_config_byte(dev, 0x44, &slave_data);
    slave_data &= ap.port_no ? 0x0F : 0xF0;
// Load the matching timing
    slave_data |= ((timings[pio][0] << 2) | timings[pio][1]) << (ap.port_no ? 4 : 0);
    pci_write_config_byte(dev, 0x44, slave_data);
    } else { 	/* Master */
    master_data &= 0xCCF4;	/* Mask out IORDY|TIME1|DMAONLY
    and master timing bits */
    master_data |= control;
    master_data |=
    (timings[pio][0] << 12) |
    (timings[pio][1] << 8);
    }
    udma_enable &= ~(1 << devid);
    pci_write_config_word(dev, master_port, master_data);
    }
    pci_write_config_byte(dev, 0x48, udma_enable);
    spin_unlock_irqrestore(&efar_lock, flags);
    }
    static const struct scsi_host_template efar_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations efar_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .cable_detect		= efar_cable_detect,
    .set_piomode		= efar_set_piomode,
    .set_dmamode		= efar_set_dmamode,
    .reset.prereset		= efar_pre_reset,
    };
//
// efar_init_one - Register EFAR ATA PCI device with kernel services
// @pdev: PCI device to register
// @ent: Entry in efar_pci_tbl matching with @pdev
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
unsafe extern "C" fn efar_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int efar_init_one (struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    static const struct ata_port_info info = {
    .flags		= ATA_FLAG_SLAVE_POSS,
    .pio_mask	= ATA_PIO4,
    .mwdma_mask	= ATA_MWDMA12_ONLY,
    .udma_mask 	= ATA_UDMA4,
    .port_ops	= &efar_ops,
    };
    const struct ata_port_info *ppi[] = { &info, &info };
    ata_print_version_once(&pdev.dev, DRV_VERSION);
    return ata_pci_bmdma_init_one(pdev, ppi, &efar_sht, core::ptr::null_mut(),
    ATA_HOST_PARALLEL_SCAN);
    }
    static const struct pci_device_id efar_pci_tbl[] = {
    { PCI_VDEVICE(EFAR, 0x9130), },
    { }	/* terminate list */
    };
    static struct pci_driver efar_pci_driver = {
    .name			= DRV_NAME,
    .id_table		= efar_pci_tbl,
    .probe			= efar_init_one,
    .remove			= ata_pci_remove_one,

    .suspend		= ata_pci_device_suspend,
    .resume			= ata_pci_device_resume,

    };
    module_pci_driver(efar_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("SCSI low-level driver for EFAR PIIX clones");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, efar_pci_tbl);
    MODULE_VERSION(DRV_VERSION);
