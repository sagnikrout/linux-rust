//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_serverworks.c
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
// pata_serverworks.c 	- Serverworks PATA for new ATA layer
// (C) 2005 Red Hat Inc
// (C) 2010 Bartlomiej Zolnierkiewicz
//
// based upon
//
// serverworks.c
//
// Copyright (C) 1998-2000 Michel Aubry
// Copyright (C) 1998-2000 Andrzej Krzysztofowicz
// Copyright (C) 1998-2000 Andre Hedrick <andre@linux-ide.org>
// Portions copyright (c) 2001 Sun Microsystems
//
// RCC/ServerWorks IDE driver for Linux
//
// OSB4: `Open South Bridge' IDE Interface (fn 1)
// supports UDMA mode 2 (33 MB/s)
//
// CSB5: `Champion South Bridge' IDE Interface (fn 1)
// all revisions support UDMA mode 4 (66 MB/s)
// revision A2.0 and up support UDMA mode 5 (100 MB/s)
//
// *** The CSB5 does not provide ANY register
// *** to detect 80-conductor cable presence.
//
// CSB6: `Champion South Bridge' IDE Interface (optional: third channel)
//
// Documentation:
// Available under NDA only. Errata info very hard to get.
//

pub const SVWKS_CSB5_REVISION_NEW: c_uint = 0x92 /* min PCI_REVISION_ID for UDMA5 (A2.0) */;
pub const SVWKS_CSB6_REVISION: c_uint = 0xa0 /* min PCI_REVISION_ID for UDMA4 (A1.0) */;
//
// Seagate Barracuda ATA IV Family drives in UDMA mode 5
// can overrun their FIFOs when used with the CSB5.
//
    static const char * const csb_bad_ata100[] = {
    "ST320011A",
    "ST340016A",
    "ST360021A",
    "ST380021A",
    core::ptr::null_mut()
    };
//
// oem_cable	-	Dell/Sun serverworks cable detection
// @ap: ATA port to do cable detect
//
// Dell PowerEdge and Sun Cobalt 'Alpine' hide the 40/80 pin select
// for their interfaces in the top two bits of the subsystem ID.
//
#[no_mangle]
unsafe extern "C" fn oem_cable(ap: *mut ata_port) -> c_int {
    static int oem_cable(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    if (pdev.subsystem_device & (1 << (ap.port_no + 14)))
    return ATA_CBL_PATA80;
    return ATA_CBL_PATA40;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sv_cable_table {
    pub device: c_int,
    pub subvendor: c_int,
    pub ap): *mut *mut int (cable_detect)(struct ata_port,
}

    static struct sv_cable_table cable_detect[] = {
    { PCI_DEVICE_ID_SERVERWORKS_CSB5IDE,   PCI_VENDOR_ID_DELL, oem_cable },
    { PCI_DEVICE_ID_SERVERWORKS_CSB6IDE,   PCI_VENDOR_ID_DELL, oem_cable },
    { PCI_DEVICE_ID_SERVERWORKS_CSB5IDE,   PCI_VENDOR_ID_SUN,  oem_cable },
    { PCI_DEVICE_ID_SERVERWORKS_OSB4IDE,   PCI_ANY_ID, ata_cable_40wire  },
    { PCI_DEVICE_ID_SERVERWORKS_CSB5IDE,   PCI_ANY_ID, ata_cable_unknown },
    { PCI_DEVICE_ID_SERVERWORKS_CSB6IDE,   PCI_ANY_ID, ata_cable_unknown },
    { PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2,  PCI_ANY_ID, ata_cable_unknown },
    { PCI_DEVICE_ID_SERVERWORKS_HT1000IDE, PCI_ANY_ID, ata_cable_unknown },
    { }
    };
//
// serverworks_cable_detect	-	cable detection
// @ap: ATA port
//
// Perform cable detection according to the device and subvendor
// identifications
//
#[no_mangle]
unsafe extern "C" fn serverworks_cable_detect(ap: *mut ata_port) -> c_int {
    static int serverworks_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    struct sv_cable_table *cb = cable_detect;
    while(cb.device) {
    if (cb.device == pdev.device &&
    (cb.subvendor == pdev.subsystem_vendor ||
    cb.subvendor == PCI_ANY_ID)) {
    return cb.cable_detect(ap);
    }
    cb++;
    }
    BUG();
    return -1;	/* kill compiler warning */
    }
//
// serverworks_is_csb	-	Check for CSB or OSB
// @pdev: PCI device to check
//
// Returns true if the device being checked is known to be a CSB
// series device.
//
#[no_mangle]
unsafe extern "C" fn serverworks_is_csb(pdev: *mut pci_dev) -> u8 {
    static u8 serverworks_is_csb(struct pci_dev *pdev)
    {
    switch (pdev.device) {
    case PCI_DEVICE_ID_SERVERWORKS_CSB5IDE:
    case PCI_DEVICE_ID_SERVERWORKS_CSB6IDE:
    case PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2:
    case PCI_DEVICE_ID_SERVERWORKS_HT1000IDE:
    return 1;
    default:
    break;
    }
    return 0;
    }
//
// serverworks_osb4_filter	-	mode selection filter
// @adev: ATA device
// @mask: Mask of proposed modes
//
// Filter the offered modes for the device to apply controller
// specific rules. OSB4 requires no UDMA for disks due to a FIFO
// bug we hit.
//
#[no_mangle]
unsafe extern "C" fn serverworks_osb4_filter(adev: *mut ata_device, mask: c_uint) -> c_uint {
    static unsigned int serverworks_osb4_filter(struct ata_device *adev, unsigned int mask)
    {
    if (adev.class == ATA_DEV_ATA)
    mask &= ~ATA_MASK_UDMA;
    return mask;
    }
//
// serverworks_csb_filter	-	mode selection filter
// @adev: ATA device
// @mask: Mask of proposed modes
//
// Check the list of devices with broken UDMA5 and
// disable UDMA5 if matched.
//
    static unsigned int serverworks_csb_filter(struct ata_device *adev,
    unsigned int mask)
    {
    const char *p;
    char model_num[ATA_ID_PROD_LEN + 1];
    int i;
// Disk, UDMA
    if (adev.class != ATA_DEV_ATA)
    return mask;
// Actually do need to check
    ata_id_c_string(adev.id, model_num, ATA_ID_PROD, sizeof(model_num));
    for (i = 0; (p = csb_bad_ata100[i]) != core::ptr::null_mut(); i++) {
    if (!strcmp(p, model_num))
    mask &= ~(0xE0 << ATA_SHIFT_UDMA);
    }
    return mask;
    }
//
// serverworks_set_piomode	-	set initial PIO mode data
// @ap: ATA interface
// @adev: ATA device
//
// Program the OSB4/CSB5 timing registers for PIO. The PIO register
// load is done as a simple lookup.
//
#[no_mangle]
unsafe extern "C" fn serverworks_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void serverworks_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u8 pio_mode[] = { 0x5d, 0x47, 0x34, 0x22, 0x20 };
    let mut offset: c_int = 1 + 2 * ap.port_no - adev.devno;
    let mut devbits: c_int = (2 * ap.port_no + adev.devno) * 4;
    u16 csb5_pio;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut pio: c_int = adev.pio_mode - XFER_PIO_0;
    pci_write_config_byte(pdev, 0x40 + offset, pio_mode[pio]);
// The OSB4 just requires the timing but the CSB series want the
    mode number as well */
    if (serverworks_is_csb(pdev)) {
    pci_read_config_word(pdev, 0x4A, &csb5_pio);
    csb5_pio &= ~(0x0F << devbits);
    pci_write_config_word(pdev, 0x4A, csb5_pio | (pio << devbits));
    }
    }
//
// serverworks_set_dmamode	-	set initial DMA mode data
// @ap: ATA interface
// @adev: ATA device
//
// Program the MWDMA/UDMA modes for the serverworks OSB4/CSB5
// chipset. The MWDMA mode values are pulled from a lookup table
// while the chipset uses mode number for UDMA.
//
#[no_mangle]
unsafe extern "C" fn serverworks_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void serverworks_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u8 dma_mode[] = { 0x77, 0x21, 0x20 };
    let mut offset: c_int = 1 + 2 * ap.port_no - adev.devno;
    let mut devbits: c_int = 2 * ap.port_no + adev.devno;
    u8 ultra;
    u8 ultra_cfg;
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    pci_read_config_byte(pdev, 0x54, &ultra_cfg);
    pci_read_config_byte(pdev, 0x56 + ap.port_no, &ultra);
    ultra &= ~(0x0F << (adev.devno * 4));
    if (adev.dma_mode >= XFER_UDMA_0) {
    pci_write_config_byte(pdev, 0x44 + offset,  0x20);
    ultra |= (adev.dma_mode - XFER_UDMA_0)
    << (adev.devno * 4);
    ultra_cfg |=  (1 << devbits);
    } else {
    pci_write_config_byte(pdev, 0x44 + offset,
    dma_mode[adev.dma_mode - XFER_MW_DMA_0]);
    ultra_cfg &= ~(1 << devbits);
    }
    pci_write_config_byte(pdev, 0x56 + ap.port_no, ultra);
    pci_write_config_byte(pdev, 0x54, ultra_cfg);
    }
    static const struct scsi_host_template serverworks_osb4_sht = {
    ATA_BASE_SHT(DRV_NAME),
    .sg_tablesize	= LIBATA_DUMB_MAX_PRD,
    .dma_boundary	= ATA_DMA_BOUNDARY,
    };
    static const struct scsi_host_template serverworks_csb_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations serverworks_osb4_port_ops = {
    .inherits	= &ata_bmdma_port_ops,
    .qc_prep	= ata_bmdma_dumb_qc_prep,
    .cable_detect	= serverworks_cable_detect,
    .mode_filter	= serverworks_osb4_filter,
    .set_piomode	= serverworks_set_piomode,
    .set_dmamode	= serverworks_set_dmamode,
    };
    static struct ata_port_operations serverworks_csb_port_ops = {
    .inherits	= &serverworks_osb4_port_ops,
    .qc_prep	= ata_bmdma_qc_prep,
    .mode_filter	= serverworks_csb_filter,
    };
#[no_mangle]
unsafe extern "C" fn serverworks_fixup_osb4(pdev: *mut pci_dev) -> c_int {
    static int serverworks_fixup_osb4(struct pci_dev *pdev)
    {
    u32 reg;
    struct pci_dev *isa_dev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS,
    PCI_DEVICE_ID_SERVERWORKS_OSB4, core::ptr::null_mut());
    if (isa_dev) {
    pci_read_config_dword(isa_dev, 0x64, &reg);
    reg &= ~0x00002000; /* disable 600ns interrupt mask */
    if (!(reg & 0x00004000))
    dev_info(&pdev.dev, "UDMA not BIOS enabled.\n");
    reg |=  0x00004000; /* enable UDMA/33 support */
    pci_write_config_dword(isa_dev, 0x64, reg);
    pci_dev_put(isa_dev);
    return 0;
    }
    dev_warn(&pdev.dev, "Unable to find bridge.\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn serverworks_fixup_csb(pdev: *mut pci_dev) -> c_int {
    static int serverworks_fixup_csb(struct pci_dev *pdev)
    {
    u8 btr;
// Third Channel Test
    if (!(PCI_FUNC(pdev.devfn) & 1)) {
    let mut findev: *mut pci_dev = core::ptr::null_mut();
    let mut reg4c: u32 = 0;
    findev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS,
    PCI_DEVICE_ID_SERVERWORKS_CSB5, core::ptr::null_mut());
    if (findev) {
    pci_read_config_dword(findev, 0x4C, &reg4c);
    reg4c &= ~0x000007FF;
    reg4c |=  0x00000040;
    reg4c |=  0x00000020;
    pci_write_config_dword(findev, 0x4C, reg4c);
    pci_dev_put(findev);
    }
    } else {
    let mut findev: *mut pci_dev = core::ptr::null_mut();
    let mut reg41: u8 = 0;
    findev = pci_get_device(PCI_VENDOR_ID_SERVERWORKS,
    PCI_DEVICE_ID_SERVERWORKS_CSB6, core::ptr::null_mut());
    if (findev) {
    pci_read_config_byte(findev, 0x41, &reg41);
    reg41 &= ~0x40;
    pci_write_config_byte(findev, 0x41, reg41);
    pci_dev_put(findev);
    }
    }
// setup the UDMA Control register
//
// 1. clear bit 6 to enable DMA
// 2. enable DMA modes with bits 0-1
// 00 : legacy
// 01 : udma2
// 10 : udma2/udma4
// 11 : udma2/udma4/udma5
//
    pci_read_config_byte(pdev, 0x5A, &btr);
    btr &= ~0x40;
    if (!(PCI_FUNC(pdev.devfn) & 1))
    btr |= 0x2;
    else
    btr |= (pdev.revision >= SVWKS_CSB5_REVISION_NEW) ? 0x3 : 0x2;
    pci_write_config_byte(pdev, 0x5A, btr);
    return btr;
    }
#[no_mangle]
unsafe extern "C" fn serverworks_fixup_ht1000(pdev: *mut pci_dev) {
    static void serverworks_fixup_ht1000(struct pci_dev *pdev)
    {
    u8 btr;
// Setup HT1000 SouthBridge Controller - Single Channel Only
    pci_read_config_byte(pdev, 0x5A, &btr);
    btr &= ~0x40;
    btr |= 0x3;
    pci_write_config_byte(pdev, 0x5A, btr);
    }
#[no_mangle]
unsafe extern "C" fn serverworks_fixup(pdev: *mut pci_dev) -> c_int {
    static int serverworks_fixup(struct pci_dev *pdev)
    {
    let mut rc: c_int = 0;
// Force master latency timer to 64 PCI clocks
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 0x40);
    switch (pdev.device) {
    case PCI_DEVICE_ID_SERVERWORKS_OSB4IDE:
    rc = serverworks_fixup_osb4(pdev);
    break;
    case PCI_DEVICE_ID_SERVERWORKS_CSB5IDE:
    ata_pci_bmdma_clear_simplex(pdev);
    fallthrough;
    case PCI_DEVICE_ID_SERVERWORKS_CSB6IDE:
    case PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2:
    rc = serverworks_fixup_csb(pdev);
    break;
    case PCI_DEVICE_ID_SERVERWORKS_HT1000IDE:
    serverworks_fixup_ht1000(pdev);
    break;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn serverworks_init_one(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int serverworks_init_one(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info[4] = {
    { /* OSB4 */
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA2,
    .port_ops = &serverworks_osb4_port_ops
    }, { /* OSB4 no UDMA */
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
// No UDMA
    .port_ops = &serverworks_osb4_port_ops
    }, { /* CSB5 */
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA4,
    .port_ops = &serverworks_csb_port_ops
    }, { /* CSB5 - later revisions*/
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA5,
    .port_ops = &serverworks_csb_port_ops
    }
    };
    const struct ata_port_info *ppi[] = { &info[id.driver_data], core::ptr::null_mut() };
    const struct scsi_host_template *sht = &serverworks_csb_sht;
    int rc;
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
    rc = serverworks_fixup(pdev);
// OSB4 : South Bridge and IDE
    if (pdev.device == PCI_DEVICE_ID_SERVERWORKS_OSB4IDE) {
// Select non UDMA capable OSB4 if we can't do fixups
    if (rc < 0)
    ppi[0] = &info[1];
    sht = &serverworks_osb4_sht;
    }
// setup CSB5/CSB6 : South Bridge and IDE option RAID
    else if ((pdev.device == PCI_DEVICE_ID_SERVERWORKS_CSB5IDE) ||
    (pdev.device == PCI_DEVICE_ID_SERVERWORKS_CSB6IDE) ||
    (pdev.device == PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2)) {
// If the returned btr is the newer revision then
    select the right info block */
    if (rc == 3)
    ppi[0] = &info[3];
// Is this the 3rd channel CSB6 IDE ?
    if (pdev.device == PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2)
    ppi[1] = &ata_dummy_port_info;
    }
    return ata_pci_bmdma_init_one(pdev, ppi, sht, core::ptr::null_mut(), 0);
    }

#[no_mangle]
unsafe extern "C" fn serverworks_reinit_one(pdev: *mut pci_dev) -> c_int {
    static int serverworks_reinit_one(struct pci_dev *pdev)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    int rc;
    rc = ata_pci_device_do_resume(pdev);
    if (rc)
    return rc;
    (void)serverworks_fixup(pdev);
    ata_host_resume(host);
    return 0;
    }

    static const struct pci_device_id serverworks[] = {
    {
    PCI_VDEVICE(SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_OSB4IDE),
    .driver_data = 0,
    }, {
    PCI_VDEVICE(SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_CSB5IDE),
    .driver_data = 2,
    }, {
    PCI_VDEVICE(SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_CSB6IDE),
    .driver_data = 2,
    }, {
    PCI_VDEVICE(SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_CSB6IDE2),
    .driver_data = 2,
    }, {
    PCI_VDEVICE(SERVERWORKS, PCI_DEVICE_ID_SERVERWORKS_HT1000IDE),
    .driver_data = 2
    },
    { }
    };
    static struct pci_driver serverworks_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= serverworks,
    .probe 		= serverworks_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= serverworks_reinit_one,

    };
    module_pci_driver(serverworks_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("low-level driver for Serverworks OSB4/CSB5/CSB6");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, serverworks);
    MODULE_VERSION(DRV_VERSION);
