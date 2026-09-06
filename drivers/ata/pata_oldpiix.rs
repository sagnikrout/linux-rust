//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_oldpiix.c
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
// pata_oldpiix.c - Intel PATA/SATA controllers
//
// (C) 2005 Red Hat
//
// Some parts based on ata_piix.c by Jeff Garzik and others.
//
// Early PIIX differs significantly from the later PIIX as it lacks
// SITRE and the slave timing registers. This means that you have to
// set timing per channel, or be clever. Libata tells us whenever it
// does drive selection and we use this to reload the timings.
//
// Because of these behaviour differences PIIX gets its own driver module.
//

//
// oldpiix_pre_reset		-	probe begin
// @link: ATA link
// @deadline: deadline jiffies for the operation
//
// Set up cable type and use generic probe init
//
#[no_mangle]
unsafe extern "C" fn oldpiix_pre_reset(link: *mut ata_link, deadline: c_ulong) -> c_int {
    static int oldpiix_pre_reset(struct ata_link *link, unsigned long deadline)
    {
    struct ata_port *ap = link.ap;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    static const struct pci_bits oldpiix_enable_bits[] = {
    { 0x41U, 1U, 0x80UL, 0x80UL },	/* port 0 */
    { 0x43U, 1U, 0x80UL, 0x80UL },	/* port 1 */
    };
    if (!pci_test_config_bits(pdev, &oldpiix_enable_bits[ap.port_no]))
    return -ENOENT;
    return ata_sff_prereset(link, deadline);
    }
//
// oldpiix_set_piomode - Initialize host controller PATA PIO timings
// @ap: Port whose timings we are configuring
// @adev: Device whose timings we are configuring
//
// Set PIO mode for device, in host controller PCI config space.
//
// LOCKING:
// None (inherited from caller).
//
#[no_mangle]
unsafe extern "C" fn oldpiix_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void oldpiix_set_piomode (struct ata_port *ap, struct ata_device *adev)
    {
    let mut pio: c_uint = adev.pio_mode - XFER_PIO_0;
    struct pci_dev *dev	= to_pci_dev(ap.host.dev);
    let mut idetm_port: c_uint = ap.port_no ? 0x42 : 0x40;
    u16 idetm_data;
    let mut control: c_int = 0;
//
// See Intel Document 298600-004 for the timing programming rules
// for PIIX/ICH. Note that the early PIIX does not have the slave
// timing port at 0x44.
//
    static const	 /* ISP  RTC */
    u8 timings[][2]	= { { 0, 0 },
    { 0, 0 },
    { 1, 0 },
    { 2, 1 },
    { 2, 3 }, };
    if (pio > 1)
    control |= 1;	/* TIME */
    if (ata_pio_need_iordy(adev))
    control |= 2;	/* IE */
// Intel specifies that the prefetch/posting is for disk only
    if (adev.class == ATA_DEV_ATA)
    control |= 4;	/* PPE */
    pci_read_config_word(dev, idetm_port, &idetm_data);
//
// Set PPE, IE and TIME as appropriate.
// Clear the other drive's timing bits.
//
    if (adev.devno == 0) {
    idetm_data &= 0xCCE0;
    idetm_data |= control;
    } else {
    idetm_data &= 0xCC0E;
    idetm_data |= (control << 4);
    }
    idetm_data |= (timings[pio][0] << 12) |
    (timings[pio][1] << 8);
    pci_write_config_word(dev, idetm_port, idetm_data);
// Track which port is configured
    ap.private_data = adev;
    }
//
// oldpiix_set_dmamode - Initialize host controller PATA DMA timings
// @ap: Port whose timings we are configuring
// @adev: Device to program
//
// Set MWDMA mode for device, in host controller PCI config space.
//
// LOCKING:
// None (inherited from caller).
//
#[no_mangle]
unsafe extern "C" fn oldpiix_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void oldpiix_set_dmamode (struct ata_port *ap, struct ata_device *adev)
    {
    struct pci_dev *dev	= to_pci_dev(ap.host.dev);
    let mut idetm_port: u8 = ap.port_no ? 0x42 : 0x40;
    u16 idetm_data;
    static const	 /* ISP  RTC */
    u8 timings[][2]	= { { 0, 0 },
    { 0, 0 },
    { 1, 0 },
    { 2, 1 },
    { 2, 3 }, };
//
// MWDMA is driven by the PIO timings. We must also enable
// IORDY unconditionally along with TIME1. PPE has already
// been set when the PIO timing was set.
//
    let mut mwdma: c_uint = adev.dma_mode - XFER_MW_DMA_0;
    unsigned int control;
    const unsigned int needed_pio[3] = {
    XFER_PIO_0, XFER_PIO_3, XFER_PIO_4
    };
    let mut pio: c_int = needed_pio[mwdma] - XFER_PIO_0;
    pci_read_config_word(dev, idetm_port, &idetm_data);
    control = 3;	/* IORDY|TIME0 */
// Intel specifies that the PPE functionality is for disk only
    if (adev.class == ATA_DEV_ATA)
    control |= 4;	/* PPE enable */
// If the drive MWDMA is faster than it can do PIO then
    we must force PIO into PIO0 */
    if (adev.pio_mode < needed_pio[mwdma])
// Enable DMA timing only
    control |= 8;	/* PIO cycles in PIO0 */
// Mask out the relevant control and timing bits we will load. Also
    clear the other drive TIME register as a precaution */
    if (adev.devno == 0) {
    idetm_data &= 0xCCE0;
    idetm_data |= control;
    } else {
    idetm_data &= 0xCC0E;
    idetm_data |= (control << 4);
    }
    idetm_data |= (timings[pio][0] << 12) | (timings[pio][1] << 8);
    pci_write_config_word(dev, idetm_port, idetm_data);
// Track which port is configured
    ap.private_data = adev;
    }
//
// oldpiix_qc_issue	-	command issue
// @qc: command pending
//
// Called when the libata layer is about to issue a command. We wrap
// this interface so that we can load the correct ATA timings if
// necessary. Our logic also clears TIME0/TIME1 for the other device so
// that, even if we get this wrong, cycles to the other device will
// be made PIO0.
//
#[no_mangle]
unsafe extern "C" fn oldpiix_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    static unsigned int oldpiix_qc_issue(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_device *adev = qc.dev;
    if (adev != ap.private_data) {
    oldpiix_set_piomode(ap, adev);
    if (ata_dma_enabled(adev))
    oldpiix_set_dmamode(ap, adev);
    }
    return ata_bmdma_qc_issue(qc);
    }
    static const struct scsi_host_template oldpiix_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations oldpiix_pata_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .qc_issue		= oldpiix_qc_issue,
    .cable_detect		= ata_cable_40wire,
    .set_piomode		= oldpiix_set_piomode,
    .set_dmamode		= oldpiix_set_dmamode,
    .reset.prereset		= oldpiix_pre_reset,
    };
//
// oldpiix_init_one - Register PIIX ATA PCI device with kernel services
// @pdev: PCI device to register
// @ent: Entry in oldpiix_pci_tbl matching with @pdev
//
// Called from kernel PCI layer.  We probe for combined mode (sigh),
// and then hand over control to libata, for it to do the rest.
//
// LOCKING:
// Inherited from PCI layer (may sleep).
//
// RETURNS:
// Zero on success, or -ERRNO value.
//
#[no_mangle]
unsafe extern "C" fn oldpiix_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int oldpiix_init_one (struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    static const struct ata_port_info info = {
    .flags		= ATA_FLAG_SLAVE_POSS,
    .pio_mask	= ATA_PIO4,
    .mwdma_mask	= ATA_MWDMA12_ONLY,
    .port_ops	= &oldpiix_pata_ops,
    };
    const struct ata_port_info *ppi[] = { &info, core::ptr::null_mut() };
    ata_print_version_once(&pdev.dev, DRV_VERSION);
    return ata_pci_bmdma_init_one(pdev, ppi, &oldpiix_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id oldpiix_pci_tbl[] = {
    { PCI_VDEVICE(INTEL, 0x1230), },
    { }	/* terminate list */
    };
    static struct pci_driver oldpiix_pci_driver = {
    .name			= DRV_NAME,
    .id_table		= oldpiix_pci_tbl,
    .probe			= oldpiix_init_one,
    .remove			= ata_pci_remove_one,

    .suspend		= ata_pci_device_suspend,
    .resume			= ata_pci_device_resume,

    };
    module_pci_driver(oldpiix_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("SCSI low-level driver for early PIIX series controllers");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, oldpiix_pci_tbl);
    MODULE_VERSION(DRV_VERSION);
