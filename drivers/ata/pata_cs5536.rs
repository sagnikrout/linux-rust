//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_cs5536.c
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
// pata_cs5536.c	- CS5536 PATA for new ATA layer
// (C) 2007 Martin K. Petersen <mkp@mkp.net>
// (C) 2011 Bartlomiej Zolnierkiewicz
//
// Documentation:
// Available from AMD web site.
//
// The IDE timing registers for the CS5536 live in the Geode Machine
// Specific Register file and not PCI config space.  Most BIOSes
// virtualize the PCI registers so the chip looks like a standard IDE
// controller.	Unfortunately not all implementations get this right.
// In particular some have problems with unaligned accesses to the
// virtualized PCI registers.  This driver always does full dword
// writes to work around the issue.  Also, in case of a bad BIOS this
// driver can be loaded with the "msr=1" parameter which forces using
// the Machine Specific Registers to configure the device.
//

    static int use_msr;
    module_param_named(msr, use_msr, int, 0644);
    MODULE_PARM_DESC(msr, "Force using MSR to configure IDE function (Default: 0)");

pub const use_msr: c_int = 0;

    enum {
    MSR_IDE_CFG		= 0x51300010,
    PCI_IDE_CFG		= 0x40,
    CFG			= 0,
    DTC			= 2,
    CAST			= 3,
    ETC			= 4,
    IDE_CFG_CHANEN		= (1 << 1),
    IDE_CFG_CABLE		= (1 << 17) | (1 << 16),
    IDE_D0_SHIFT		= 24,
    IDE_D1_SHIFT		= 16,
    IDE_DRV_MASK		= 0xff,
    IDE_CAST_D0_SHIFT	= 6,
    IDE_CAST_D1_SHIFT	= 4,
    IDE_CAST_DRV_MASK	= 0x3,
    IDE_CAST_CMD_MASK	= 0xff,
    IDE_CAST_CMD_SHIFT	= 24,
    IDE_ETC_UDMA_MASK	= 0xc0,
    };
// Some Bachmann OT200 devices have a non working UDMA support due a
// missing resistor.
//
    static const struct dmi_system_id udma_quirk_dmi_table[] = {
    {
    .ident = "Bachmann electronic OT200",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Bachmann electronic"),
    DMI_MATCH(DMI_PRODUCT_NAME, "OT200"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "1")
    },
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn cs5536_read(pdev: *mut pci_dev, reg: c_int, val: *mut u32) -> c_int {
    static int cs5536_read(struct pci_dev *pdev, int reg, u32 *val)
    {
    if (unlikely(use_msr)) {
    u32 dummy __maybe_unused;
    rdmsr(MSR_IDE_CFG + reg, *val, dummy);
    return 0;
    }
    return pci_read_config_dword(pdev, PCI_IDE_CFG + reg * 4, val);
    }
#[no_mangle]
unsafe extern "C" fn cs5536_write(pdev: *mut pci_dev, reg: c_int, val: c_int) -> c_int {
    static int cs5536_write(struct pci_dev *pdev, int reg, int val)
    {
    if (unlikely(use_msr)) {
    wrmsr(MSR_IDE_CFG + reg, val, 0);
    return 0;
    }
    return pci_write_config_dword(pdev, PCI_IDE_CFG + reg * 4, val);
    }
#[no_mangle]
unsafe extern "C" fn cs5536_program_dtc(adev: *mut ata_device, tim: u8) {
    static void cs5536_program_dtc(struct ata_device *adev, u8 tim)
    {
    struct pci_dev *pdev = to_pci_dev(adev.link.ap.host.dev);
    let mut dshift: c_int = adev.devno ? IDE_D1_SHIFT : IDE_D0_SHIFT;
    u32 dtc;
    cs5536_read(pdev, DTC, &dtc);
    dtc &= ~(IDE_DRV_MASK << dshift);
    dtc |= tim << dshift;
    cs5536_write(pdev, DTC, dtc);
    }
//
// cs5536_cable_detect	-	detect cable type
// @ap: Port to detect on
//
// Perform cable detection for ATA66 capable cable.
//
// Returns a cable type.
//
#[no_mangle]
unsafe extern "C" fn cs5536_cable_detect(ap: *mut ata_port) -> c_int {
    static int cs5536_cable_detect(struct ata_port *ap)
    {
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u32 cfg;
    cs5536_read(pdev, CFG, &cfg);
    if (cfg & IDE_CFG_CABLE)
    return ATA_CBL_PATA80;
    else
    return ATA_CBL_PATA40;
    }
//
// cs5536_set_piomode		-	PIO setup
// @ap: ATA interface
// @adev: device on the interface
//
#[no_mangle]
unsafe extern "C" fn cs5536_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void cs5536_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u8 drv_timings[5] = {
    0x98, 0x55, 0x32, 0x21, 0x20,
    };
    static const u8 addr_timings[5] = {
    0x2, 0x1, 0x0, 0x0, 0x0,
    };
    static const u8 cmd_timings[5] = {
    0x99, 0x92, 0x90, 0x22, 0x20,
    };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    struct ata_device *pair = ata_dev_pair(adev);
    let mut mode: c_int = adev.pio_mode - XFER_PIO_0;
    let mut cmdmode: c_int = mode;
    let mut cshift: c_int = adev.devno ? IDE_CAST_D1_SHIFT : IDE_CAST_D0_SHIFT;
    u32 cast;
    if (pair)
    cmdmode = min(mode, pair.pio_mode - XFER_PIO_0);
    cs5536_program_dtc(adev, drv_timings[mode]);
    cs5536_read(pdev, CAST, &cast);
    cast &= ~(IDE_CAST_DRV_MASK << cshift);
    cast |= addr_timings[mode] << cshift;
    cast &= ~(IDE_CAST_CMD_MASK << IDE_CAST_CMD_SHIFT);
    cast |= cmd_timings[cmdmode] << IDE_CAST_CMD_SHIFT;
    cs5536_write(pdev, CAST, cast);
    }
//
// cs5536_set_dmamode		-	DMA timing setup
// @ap: ATA interface
// @adev: Device being configured
//
#[no_mangle]
unsafe extern "C" fn cs5536_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void cs5536_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    static const u8 udma_timings[6] = {
    0xc2, 0xc1, 0xc0, 0xc4, 0xc5, 0xc6,
    };
    static const u8 mwdma_timings[3] = {
    0x67, 0x21, 0x20,
    };
    struct pci_dev *pdev = to_pci_dev(ap.host.dev);
    u32 etc;
    let mut mode: c_int = adev.dma_mode;
    let mut dshift: c_int = adev.devno ? IDE_D1_SHIFT : IDE_D0_SHIFT;
    cs5536_read(pdev, ETC, &etc);
    if (mode >= XFER_UDMA_0) {
    etc &= ~(IDE_DRV_MASK << dshift);
    etc |= udma_timings[mode - XFER_UDMA_0] << dshift;
    } else { /* MWDMA */
    etc &= ~(IDE_ETC_UDMA_MASK << dshift);
    cs5536_program_dtc(adev, mwdma_timings[mode - XFER_MW_DMA_0]);
    }
    cs5536_write(pdev, ETC, etc);
    }
    static const struct scsi_host_template cs5536_sht = {
    ATA_BMDMA_SHT(DRV_NAME),
    };
    static struct ata_port_operations cs5536_port_ops = {
    .inherits		= &ata_bmdma32_port_ops,
    .cable_detect		= cs5536_cable_detect,
    .set_piomode		= cs5536_set_piomode,
    .set_dmamode		= cs5536_set_dmamode,
    };
//
// cs5536_init_one
// @dev: PCI device
// @id: Entry in match table
//
#[no_mangle]
unsafe extern "C" fn cs5536_init_one(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int cs5536_init_one(struct pci_dev *dev, const struct pci_device_id *id)
    {
    static const struct ata_port_info info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .mwdma_mask = ATA_MWDMA2,
    .udma_mask = ATA_UDMA5,
    .port_ops = &cs5536_port_ops,
    };
    static const struct ata_port_info no_udma_info = {
    .flags = ATA_FLAG_SLAVE_POSS,
    .pio_mask = ATA_PIO4,
    .port_ops = &cs5536_port_ops,
    };
    const struct ata_port_info *ppi[2];
    u32 cfg;
    if (dmi_check_system(udma_quirk_dmi_table))
    ppi[0] = &no_udma_info;
    else
    ppi[0] = &info;
    ppi[1] = &ata_dummy_port_info;
    if (use_msr)
    dev_err(&dev.dev, DRV_NAME ": Using MSR regs instead of PCI\n");
    cs5536_read(dev, CFG, &cfg);
    if ((cfg & IDE_CFG_CHANEN) == 0) {
    dev_err(&dev.dev, DRV_NAME ": disabled by BIOS\n");
    return -ENODEV;
    }
    return ata_pci_bmdma_init_one(dev, ppi, &cs5536_sht, core::ptr::null_mut(), 0);
    }
    static const struct pci_device_id cs5536[] = {
    { PCI_VDEVICE(AMD,	PCI_DEVICE_ID_AMD_CS5536_IDE), },
    { PCI_VDEVICE(AMD,	PCI_DEVICE_ID_AMD_CS5536_DEV_IDE), },
    { },
    };
    static struct pci_driver cs5536_pci_driver = {
    .name		= DRV_NAME,
    .id_table	= cs5536,
    .probe		= cs5536_init_one,
    .remove		= ata_pci_remove_one,

    .suspend	= ata_pci_device_suspend,
    .resume		= ata_pci_device_resume,

    };
    module_pci_driver(cs5536_pci_driver);
    MODULE_AUTHOR("Martin K. Petersen");
    MODULE_DESCRIPTION("low-level driver for the CS5536 IDE controller");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, cs5536);
    MODULE_VERSION(DRV_VERSION);
