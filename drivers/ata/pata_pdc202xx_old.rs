//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_pdc202xx_old.c
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
// pata_pdc202xx_old.c 	- Promise PDC202xx PATA for new ATA layer
// (C) 2005 Red Hat Inc
// Alan Cox <alan@lxorguk.ukuu.org.uk>
// (C) 2007,2009,2010 Bartlomiej Zolnierkiewicz
//
// Based in part on linux/drivers/ide/pci/pdc202xx_old.c
//
// First cut with LBA48/ATAPI
//
// TODO:
// Channel interlock/reset on both required ?
//

#[no_mangle]
unsafe extern "C" fn pdc2026x_cable_detect(ap: *mut ata_port) -> c_int {
    static int pdc2026x_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u16 cis;
    pci_read_config_word(pdev, 0x50, &cis);
    if (cis & (1 << (10 + ap.port_no)))
    return ATA_CBL_PATA40;
    return ATA_CBL_PATA80;
    }
    static void pdc202xx_exec_command(struct ata_port *ap,
    const struct ata_taskfile *tf)
    {
    iowrite8(tf.command, ap.ioaddr.command_addr);
    ndelay(400);
    }
#[no_mangle]
unsafe extern "C" fn pdc202xx_irq_check(ap: *mut ata_port) -> bool {
    static bool pdc202xx_irq_check(struct ata_port *ap)
    {
    struct pci_dev *pdev	= to_pci_dev(ap.host.dev);
    let mut master: c_ulong = pci_resource_start(pdev, 4);
    let mut sc1d: u8 = inb(master + 0x1d);
    if (ap.port_no) {
//
// bit 7: error, bit 6: interrupting,
// bit 5: FIFO full, bit 4: FIFO empty
//
    return sc1d & 0x40;
    } else	{
//
// bit 3: error, bit 2: interrupting,
// bit 1: FIFO full, bit 0: FIFO empty
//
    return sc1d & 0x04;
    }
    }
//
// pdc202xx_configure_piomode	-	set chip PIO timing
// @ap: ATA interface
// @adev: ATA device
// @pio: PIO mode
//
// Called to do the PIO mode setup. Our timing registers are shared
// so a configure_dmamode call will undo any work we do here and vice
// versa
//
#[no_mangle]
unsafe extern "C" fn pdc202xx_configure_piomode(ap: *mut ata_port, adev: *mut ata_device, pio: c_int) {
    static void pdc202xx_configure_piomode(struct ata_port *ap, struct ata_device *adev, int pio)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut port: c_int = 0x60 + 8 * ap.port_no + 4 * adev.devno;
    static const u16 pio_timing[5] = {
    0x0913, 0x050C , 0x0308, 0x0206, 0x0104
    };
    u8 r_ap, r_bp;
    pci_read_config_byte(pdev, port, &r_ap);
    pci_read_config_byte(pdev, port + 1, &r_bp);
    r_ap &= ~0x3F;	/* Preserve ERRDY_EN, SYNC_IN */
    r_bp &= ~0x1F;
    r_ap |= (pio_timing[pio] >> 8);
    r_bp |= (pio_timing[pio] & 0xFF);
    if (ata_pio_need_iordy(adev))
    r_ap |= 0x20;	/* IORDY enable */
    if (adev.class == ATA_DEV_ATA)
    r_ap |= 0x10;	/* FIFO enable */
    pci_write_config_byte(pdev, port, r_ap);
    pci_write_config_byte(pdev, port + 1, r_bp);
    }
//
// pdc202xx_set_piomode	-	set initial PIO mode data
// @ap: ATA interface
// @adev: ATA device
//
// Called to do the PIO mode setup. Our timing registers are shared
// but we want to set the PIO timing by default.
//
#[no_mangle]
unsafe extern "C" fn pdc202xx_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void pdc202xx_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    pdc202xx_configure_piomode(ap, adev, adev.pio_mode - XFER_PIO_0);
    }
//
// pdc202xx_set_dmamode	-	set DMA mode in chip
// @ap: ATA interface
// @adev: ATA device
//
// Load DMA cycle times into the chip ready for a DMA transfer
// to occur.
//
#[no_mangle]
unsafe extern "C" fn pdc202xx_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void pdc202xx_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    let mut port: c_int = 0x60 + 8 * ap.port_no + 4 * adev.devno;
    static u8 udma_timing[6][2] = {
    { 0x60, 0x03 },	/* 33 Mhz Clock */
    { 0x40, 0x02 },
    { 0x20, 0x01 },
    { 0x40, 0x02 },	/* 66 Mhz Clock */
    { 0x20, 0x01 },
    { 0x20, 0x01 }
    };
    static u8 mdma_timing[3][2] = {
    { 0xe0, 0x0f },
    { 0x60, 0x04 },
    { 0x60, 0x03 },
    };
    u8 r_bp, r_cp;
    pci_read_config_byte(pdev, port + 1, &r_bp);
    pci_read_config_byte(pdev, port + 2, &r_cp);
    r_bp &= ~0xE0;
    r_cp &= ~0x0F;
    if (adev.dma_mode >= XFER_UDMA_0) {
    let mut speed: c_int = adev.dma_mode - XFER_UDMA_0;
    r_bp |= udma_timing[speed][0];
    r_cp |= udma_timing[speed][1];
    } else {
    let mut speed: c_int = adev.dma_mode - XFER_MW_DMA_0;
    r_bp |= mdma_timing[speed][0];
    r_cp |= mdma_timing[speed][1];
    }
    pci_write_config_byte(pdev, port + 1, r_bp);
    pci_write_config_byte(pdev, port + 2, r_cp);
    }
//
// pdc2026x_bmdma_start		-	DMA engine begin
// @qc: ATA command
//
// In UDMA3 or higher we have to clock switch for the duration of the
// DMA transfer sequence.
//
// Note: The host lock held by the libata layer protects
// us from two channels both trying to set DMA bits at once
//
#[no_mangle]
unsafe extern "C" fn pdc2026x_bmdma_start(qc: *mut ata_queued_cmd) {
    static void pdc2026x_bmdma_start(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_device *adev = qc.dev;
    struct ata_taskfile *tf = &qc.tf;
    let mut sel66: c_int = ap.port_no ? 0x08: 0x02;
    void __iomem *master = ap.host.ports[0].ioaddr.bmdma_addr;
    void __iomem *clock = master + 0x11;
    void __iomem *atapi_reg = master + 0x20 + (4 * ap.port_no);
    u32 len;
// Check we keep host level locking here
    if (adev.dma_mode > XFER_UDMA_2)
    iowrite8(ioread8(clock) | sel66, clock);
    else
    iowrite8(ioread8(clock) & ~sel66, clock);
// The DMA clocks may have been trashed by a reset. FIXME: make conditional
    and move to qc_issue ? */
    pdc202xx_set_dmamode(ap, qc.dev);
// Cases the state machine will not complete correctly without help
    if ((tf.flags & ATA_TFLAG_LBA48) ||  tf.protocol == ATAPI_PROT_DMA) {
    len = qc.nbytes / 2;
    if (tf.flags & ATA_TFLAG_WRITE)
    len |= 0x06000000;
    else
    len |= 0x05000000;
    iowrite32(len, atapi_reg);
    }
// Activate DMA
    ata_bmdma_start(qc);
    }
//
// pdc2026x_bmdma_stop		-	DMA engine stop
// @qc: ATA command
//
// After a DMA completes we need to put the clock back to 33MHz for
// PIO timings.
//
// Note: The host lock held by the libata layer protects
// us from two channels both trying to set DMA bits at once
//
#[no_mangle]
unsafe extern "C" fn pdc2026x_bmdma_stop(qc: *mut ata_queued_cmd) {
    static void pdc2026x_bmdma_stop(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_device *adev = qc.dev;
    struct ata_taskfile *tf = &qc.tf;
    let mut sel66: c_int = ap.port_no ? 0x08: 0x02;
// The clock bits are in the same register for both channels
    void __iomem *master = ap.host.ports[0].ioaddr.bmdma_addr;
    void __iomem *clock = master + 0x11;
    void __iomem *atapi_reg = master + 0x20 + (4 * ap.port_no);
// Cases the state machine will not complete correctly
    if (tf.protocol == ATAPI_PROT_DMA || (tf.flags & ATA_TFLAG_LBA48)) {
    iowrite32(0, atapi_reg);
    iowrite8(ioread8(clock) & ~sel66, clock);
    }
// Flip back to 33Mhz for PIO
    if (adev.dma_mode > XFER_UDMA_2)
    iowrite8(ioread8(clock) & ~sel66, clock);
    ata_bmdma_stop(qc);
    pdc202xx_set_piomode(ap, adev);
    }
//
// pdc2026x_dev_config	-	device setup hook
// @adev: newly found device
//
// Perform chip specific early setup. We need to lock the transfer
// sizes to 8bit to avoid making the state engine on the 2026x cards
// barf.
//
#[no_mangle]
unsafe extern "C" fn pdc2026x_dev_config(adev: *mut ata_device) {
    static void pdc2026x_dev_config(struct ata_device *adev)
    {
    adev.max_sectors = 256;
    }
#[no_mangle]
unsafe extern "C" fn pdc2026x_port_start(ap: *mut ata_port) -> c_int {
    static int pdc2026x_port_start(struct ata_port *ap)
    {
    void __iomem *bmdma = ap.ioaddr.bmdma_addr;
    if (bmdma) {
// Enable burst mode
    let mut burst: u8 = ioread8(bmdma + 0x1f);
    iowrite8(burst | 0x01, bmdma + 0x1f);
    }
    return ata_bmdma_port_start(ap);
    }
//
// pdc2026x_check_atapi_dma - Check whether ATAPI DMA can be supported for this command
// @qc: Metadata associated with taskfile to check
//
// Just say no - not supported on older Promise.
//
// LOCKING:
// None (inherited from caller).
//
// RETURNS: 0 when ATAPI DMA can be used
// 1 otherwise
//
#[no_mangle]
unsafe extern "C" fn pdc2026x_check_atapi_dma(qc: *mut ata_queued_cmd) -> c_int {
    static int pdc2026x_check_atapi_dma(struct ata_queued_cmd *qc)
    {
    return 1;
    }
    static const struct scsi_host_template pdc202xx_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations pdc2024x_port_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .cable_detect		= ata_cable_40wire,
    .set_piomode		= pdc202xx_set_piomode,
    .set_dmamode		= pdc202xx_set_dmamode,
    .sff_exec_command	= pdc202xx_exec_command,
    .sff_irq_check		= pdc202xx_irq_check,
    };
    static struct ata_port_operations pdc2026x_port_ops = {
    .inherits		= &pdc2024x_port_ops,
    .check_atapi_dma	= pdc2026x_check_atapi_dma,
    .bmdma_start		= pdc2026x_bmdma_start,
    .bmdma_stop		= pdc2026x_bmdma_stop,
    .cable_detect		= pdc2026x_cable_detect,
    .dev_config		= pdc2026x_dev_config,
    .port_start		= pdc2026x_port_start,
    .sff_exec_command	= pdc202xx_exec_command,
    .sff_irq_check		= pdc202xx_irq_check,
    };
#[no_mangle]
unsafe extern "C" fn pdc202xx_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int pdc202xx_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info[3] = {
    {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA2,
    .port_ops = &pdc2024x_port_ops
    },
    {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA4,
    .port_ops = &pdc2026x_port_ops
    },
    {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA5,
    .port_ops = &pdc2026x_port_ops
    }
    };
    const struct ata_port_info *ppi[] = { &info[id.driver_data], core::ptr::null_mut() };
    if (dev.device == PCI_DEVICE_ID_PROMISE_20265) {
    struct pci_dev *bridge = dev.bus.self;
// Don't grab anything behind a Promise I2O RAID
    if (bridge && bridge.vendor == PCI_VENDOR_ID_INTEL) {
    if (bridge.device == PCI_DEVICE_ID_INTEL_I960)
    return -ENODEV;
    if (bridge.device == PCI_DEVICE_ID_INTEL_I960RM)
    return -ENODEV;
    }
    }
    return ata_pci_bmdma_init_one(dev, ppi, &pdc202xx_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id pdc202xx[] = {
    { PCI_VDEVICE(PROMISE, PCI_DEVICE_ID_PROMISE_20246), .driver_data = 0 },
    { PCI_VDEVICE(PROMISE, PCI_DEVICE_ID_PROMISE_20262), .driver_data = 1 },
    { PCI_VDEVICE(PROMISE, PCI_DEVICE_ID_PROMISE_20263), .driver_data = 1 },
    { PCI_VDEVICE(PROMISE, PCI_DEVICE_ID_PROMISE_20265), .driver_data = 2 },
    { PCI_VDEVICE(PROMISE, PCI_DEVICE_ID_PROMISE_20267), .driver_data = 2 },
    { }
    };
    static struct pci_driver pdc202xx_pci_driver = {
    .name 		= DRV_NAME,
    .id_table	= pdc202xx,
    .probe 		= pdc202xx_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(pdc202xx_pci_driver);
    MODULE_AUTHOR("Alan Cox");
    MODULE_DESCRIPTION("low-level driver for Promise 2024x and 20262-20267");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, pdc202xx);
    MODULE_VERSION(DRV_VERSION);
