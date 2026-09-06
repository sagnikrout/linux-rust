//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_macio.c
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
// Libata based driver for Apple "macio" family of PATA controllers
//
// Copyright 2008/2009 Benjamin Herrenschmidt, IBM Corp
// <benh@kernel.crashing.org>
//
// Some bits and pieces from drivers/ide/ppc/pmac.c
//

    dev_printk(KERN_DEBUG , dev , format , ## arg)

    ({ if (0) dev_printk(KERN_DEBUG, dev, format, ##arg); 0; })

// Models of macio ATA controller
    enum {
    controller_ohare,	/* OHare based */
    controller_heathrow,	/* Heathrow/Paddington */
    controller_kl_ata3,	/* KeyLargo ATA-3 */
    controller_kl_ata4,	/* KeyLargo ATA-4 */
    controller_un_ata6,	/* UniNorth2 ATA-6 */
    controller_k2_ata6,	/* K2 ATA-6 */
    controller_sh_ata6,	/* Shasta ATA-6 */
    };
    static const char* macio_ata_names[] = {
    "OHare ATA",		/* OHare based */
    "Heathrow ATA",		/* Heathrow/Paddington */
    "KeyLargo ATA-3",	/* KeyLargo ATA-3 (MDMA only) */
    "KeyLargo ATA-4",	/* KeyLargo ATA-4 (UDMA/66) */
    "UniNorth ATA-6",	/* UniNorth2 ATA-6 (UDMA/100) */
    "K2 ATA-6",		/* K2 ATA-6 (UDMA/100) */
    "Shasta ATA-6",		/* Shasta ATA-6 (UDMA/133) */
    };
//
// Extra registers, both 32-bit little-endian
//
pub const IDE_TIMING_CONFIG: c_uint = 0x200;
pub const IDE_INTERRUPT: c_uint = 0x300;
// Kauai (U2) ATA has different register setup
pub const IDE_KAUAI_PIO_CONFIG: c_uint = 0x200;
pub const IDE_KAUAI_ULTRA_CONFIG: c_uint = 0x210;
pub const IDE_KAUAI_POLL_CONFIG: c_uint = 0x220;
//
// Timing configuration register definitions
//
// Number of IDE_SYSCLK_NS ticks, argument is in nanoseconds

// 133Mhz cell, found in shasta.
// See comments about 100 Mhz Uninorth 2...
// Note that PIO_MASK and MDMA_MASK seem to overlap, that's just
// weird and I don't now why .. at this stage
//
pub const TR_133_PIOREG_PIO_MASK: c_uint = 0xff000fff;
pub const TR_133_PIOREG_MDMA_MASK: c_uint = 0x00fff800;
pub const TR_133_UDMAREG_UDMA_MASK: c_uint = 0x0003ffff;
pub const TR_133_UDMAREG_UDMA_EN: c_uint = 0x00000001;
// 100Mhz cell, found in Uninorth 2 and K2. It appears as a pci device
// (106b/0033) on uninorth or K2 internal PCI bus and it's clock is
// controlled like gem or fw. It appears to be an evolution of keylargo
// ATA4 with a timing register extended to 2x32bits registers (one
// for PIO & MWDMA and one for UDMA, and a similar DBDMA channel.
// It has it's own local feature control register as well.
//
// After scratching my mind over the timing values, at least for PIO
// and MDMA, I think I've figured the format of the timing register,
// though I use pre-calculated tables for UDMA as usual...
//
pub const TR_100_PIO_ADDRSETUP_MASK: c_uint = 0xff000000 /* Size of field unknown */;
pub const TR_100_PIO_ADDRSETUP_SHIFT: c_int = 24;
pub const TR_100_MDMA_MASK: c_uint = 0x00fff000;
pub const TR_100_MDMA_RECOVERY_MASK: c_uint = 0x00fc0000;
pub const TR_100_MDMA_RECOVERY_SHIFT: c_int = 18;
pub const TR_100_MDMA_ACCESS_MASK: c_uint = 0x0003f000;
pub const TR_100_MDMA_ACCESS_SHIFT: c_int = 12;
pub const TR_100_PIO_MASK: c_uint = 0xff000fff;
pub const TR_100_PIO_RECOVERY_MASK: c_uint = 0x00000fc0;
pub const TR_100_PIO_RECOVERY_SHIFT: c_int = 6;
pub const TR_100_PIO_ACCESS_MASK: c_uint = 0x0000003f;
pub const TR_100_PIO_ACCESS_SHIFT: c_int = 0;
pub const TR_100_UDMAREG_UDMA_MASK: c_uint = 0x0000ffff;
pub const TR_100_UDMAREG_UDMA_EN: c_uint = 0x00000001;
// 66Mhz cell, found in KeyLargo. Can do ultra mode 0 to 2 on
// 40 connector cable and to 4 on 80 connector one.
// Clock unit is 15ns (66Mhz)
//
// 3 Values can be programmed:
// - Write data setup, which appears to match the cycle time. They
// also call it DIOW setup.
// - Ready to pause time (from spec)
// - Address setup. That one is weird. I don't see where exactly
// it fits in UDMA cycles, I got it's name from an obscure piece
// of commented out code in Darwin. They leave it to 0, we do as
// well, despite a comment that would lead to think it has a
// min value of 45ns.
// Apple also add 60ns to the write data setup (or cycle time ?) on
// reads.
//
pub const TR_66_UDMA_MASK: c_uint = 0xfff00000;
pub const TR_66_UDMA_EN: c_uint = 0x00100000 /* Enable Ultra mode for DMA */;
pub const TR_66_PIO_ADDRSETUP_MASK: c_uint = 0xe0000000 /* Address setup */;
pub const TR_66_PIO_ADDRSETUP_SHIFT: c_int = 29;
pub const TR_66_UDMA_RDY2PAUS_MASK: c_uint = 0x1e000000 /* Ready 2 pause time */;
pub const TR_66_UDMA_RDY2PAUS_SHIFT: c_int = 25;
pub const TR_66_UDMA_WRDATASETUP_MASK: c_uint = 0x01e00000 /* Write data setup time */;
pub const TR_66_UDMA_WRDATASETUP_SHIFT: c_int = 21;
pub const TR_66_MDMA_MASK: c_uint = 0x000ffc00;
pub const TR_66_MDMA_RECOVERY_MASK: c_uint = 0x000f8000;
pub const TR_66_MDMA_RECOVERY_SHIFT: c_int = 15;
pub const TR_66_MDMA_ACCESS_MASK: c_uint = 0x00007c00;
pub const TR_66_MDMA_ACCESS_SHIFT: c_int = 10;
pub const TR_66_PIO_MASK: c_uint = 0xe00003ff;
pub const TR_66_PIO_RECOVERY_MASK: c_uint = 0x000003e0;
pub const TR_66_PIO_RECOVERY_SHIFT: c_int = 5;
pub const TR_66_PIO_ACCESS_MASK: c_uint = 0x0000001f;
pub const TR_66_PIO_ACCESS_SHIFT: c_int = 0;
// 33Mhz cell, found in OHare, Heathrow (& Paddington) and KeyLargo
// Can do pio & mdma modes, clock unit is 30ns (33Mhz)
//
// The access time and recovery time can be programmed. Some older
// Darwin code base limit OHare to 150ns cycle time. I decided to do
// the same here fore safety against broken old hardware ;)
// The HalfTick bit, when set, adds half a clock (15ns) to the access
// time and removes one from recovery. It's not supported on KeyLargo
// implementation afaik. The E bit appears to be set for PIO mode 0 and
// is used to reach long timings used in this mode.
//
pub const TR_33_MDMA_MASK: c_uint = 0x003ff800;
pub const TR_33_MDMA_RECOVERY_MASK: c_uint = 0x001f0000;
pub const TR_33_MDMA_RECOVERY_SHIFT: c_int = 16;
pub const TR_33_MDMA_ACCESS_MASK: c_uint = 0x0000f800;
pub const TR_33_MDMA_ACCESS_SHIFT: c_int = 11;
pub const TR_33_MDMA_HALFTICK: c_uint = 0x00200000;
pub const TR_33_PIO_MASK: c_uint = 0x000007ff;
pub const TR_33_PIO_E: c_uint = 0x00000400;
pub const TR_33_PIO_RECOVERY_MASK: c_uint = 0x000003e0;
pub const TR_33_PIO_RECOVERY_SHIFT: c_int = 5;
pub const TR_33_PIO_ACCESS_MASK: c_uint = 0x0000001f;
pub const TR_33_PIO_ACCESS_SHIFT: c_int = 0;
//
// Interrupt register definitions. Only present on newer cells
// (Keylargo and later afaik) so we don't use it.
//
pub const IDE_INTR_DMA: c_uint = 0x80000000;
pub const IDE_INTR_DEVICE: c_uint = 0x40000000;
//
// FCR Register on Kauai. Not sure what bit 0x4 is  ...
//
pub const KAUAI_FCR_UATA_MAGIC: c_uint = 0x00000004;
pub const KAUAI_FCR_UATA_RESET_N: c_uint = 0x00000002;
pub const KAUAI_FCR_UATA_ENABLE: c_uint = 0x00000001;
// Allow up to 256 DBDMA commands per xfer
pub const MAX_DCMDS: c_int = 256;
// Don't let a DMA segment go all the way to 64K
pub const MAX_DBDMA_SEG: c_uint = 0xff00;

//
// The SCSI core requires the segment size to cover at least a page, so
// for 64K page size kernels it must be at least 64K. However the
// hardware can't handle 64K, so pata_macio_qc_prep() will split large
// requests. To handle the split requests the tablesize must be halved.
//

//
// Wait 1s for disk to answer on IDE bus after a hard reset
// of the device (via GPIO/FCR).
//
// Some devices seem to "pollute" the bus even after dropping
// the BSY bit (typically some combo drives slave on the UDMA
// bus) after a hard reset. Since we hard reset all drives on
// KeyLargo ATA66, we have to keep that delay around. I may end
// up not hard resetting anymore on these and keep the delay only
// for older interfaces instead (we have to reset when coming
// from MacOS...) --BenH.
//
pub const IDE_WAKEUP_DELAY_MS: c_int = 1000;
    struct pata_macio_timing;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pata_macio_priv {
    pub kind: c_int,
    pub aapl_bus_id: c_int,
    pub 1: int mediabay :,
    pub node: *mut device_node,
    pub mdev: *mut macio_dev,
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub irq: c_int,
    pub treg: [u32; 2][2],
    pub tfregs: *mut void __iomem,
    pub kauai_fcr: *mut void __iomem,
    pub dma_table_cpu: *mut *mut dbdma_cmd,
    pub dma_table_dma: dma_addr_t,
    pub host: *mut ata_host,
    pub timings: *const pata_macio_timing,
}

// Previous variants of this driver used to calculate timings
// for various variants of the chip and use tables for others.
//
// Not only was this confusing, but in addition, it isn't clear
// whether our calculation code was correct. It didn't entirely
// match the darwin code and whatever documentation I could find
// on these cells
//
// I decided to entirely rely on a table instead for this version
// of the driver. Also, because I don't really care about derated
// modes and really old HW other than making it work, I'm not going
// to calculate / snoop timing values for something else than the
// standard modes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pata_macio_timing {
    pub mode: c_int,
    pub /: *mut *mut u32 reg1; / Bits to set in first timing reg,
    pub /: *mut *mut u32 reg2; / Bits to set in second timing reg,
}

    static const struct pata_macio_timing pata_macio_ohare_timings[] = {
    { XFER_PIO_0,		0x00000526,	0, },
    { XFER_PIO_1,		0x00000085,	0, },
    { XFER_PIO_2,		0x00000025,	0, },
    { XFER_PIO_3,		0x00000025,	0, },
    { XFER_PIO_4,		0x00000025,	0, },
    { XFER_MW_DMA_0,	0x00074000,	0, },
    { XFER_MW_DMA_1,	0x00221000,	0, },
    { XFER_MW_DMA_2,	0x00211000,	0, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing pata_macio_heathrow_timings[] = {
    { XFER_PIO_0,		0x00000526,	0, },
    { XFER_PIO_1,		0x00000085,	0, },
    { XFER_PIO_2,		0x00000025,	0, },
    { XFER_PIO_3,		0x00000025,	0, },
    { XFER_PIO_4,		0x00000025,	0, },
    { XFER_MW_DMA_0,	0x00074000,	0, },
    { XFER_MW_DMA_1,	0x00221000,	0, },
    { XFER_MW_DMA_2,	0x00211000,	0, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing pata_macio_kl33_timings[] = {
    { XFER_PIO_0,		0x00000526,	0, },
    { XFER_PIO_1,		0x00000085,	0, },
    { XFER_PIO_2,		0x00000025,	0, },
    { XFER_PIO_3,		0x00000025,	0, },
    { XFER_PIO_4,		0x00000025,	0, },
    { XFER_MW_DMA_0,	0x00084000,	0, },
    { XFER_MW_DMA_1,	0x00021800,	0, },
    { XFER_MW_DMA_2,	0x00011800,	0, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing pata_macio_kl66_timings[] = {
    { XFER_PIO_0,		0x0000038c,	0, },
    { XFER_PIO_1,		0x0000020a,	0, },
    { XFER_PIO_2,		0x00000127,	0, },
    { XFER_PIO_3,		0x000000c6,	0, },
    { XFER_PIO_4,		0x00000065,	0, },
    { XFER_MW_DMA_0,	0x00084000,	0, },
    { XFER_MW_DMA_1,	0x00029800,	0, },
    { XFER_MW_DMA_2,	0x00019400,	0, },
    { XFER_UDMA_0,		0x19100000,	0, },
    { XFER_UDMA_1,		0x14d00000,	0, },
    { XFER_UDMA_2,		0x10900000,	0, },
    { XFER_UDMA_3,		0x0c700000,	0, },
    { XFER_UDMA_4,		0x0c500000,	0, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing pata_macio_kauai_timings[] = {
    { XFER_PIO_0,		0x08000a92,	0, },
    { XFER_PIO_1,		0x0800060f,	0, },
    { XFER_PIO_2,		0x0800038b,	0, },
    { XFER_PIO_3,		0x05000249,	0, },
    { XFER_PIO_4,		0x04000148,	0, },
    { XFER_MW_DMA_0,	0x00618000,	0, },
    { XFER_MW_DMA_1,	0x00209000,	0, },
    { XFER_MW_DMA_2,	0x00148000,	0, },
    { XFER_UDMA_0,		         0,	0x000070c1, },
    { XFER_UDMA_1,		         0,	0x00005d81, },
    { XFER_UDMA_2,		         0,	0x00004a61, },
    { XFER_UDMA_3,		         0,	0x00003a51, },
    { XFER_UDMA_4,		         0,	0x00002a31, },
    { XFER_UDMA_5,		         0,	0x00002921, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing pata_macio_shasta_timings[] = {
    { XFER_PIO_0,		0x0a000c97,	0, },
    { XFER_PIO_1,		0x07000712,	0, },
    { XFER_PIO_2,		0x040003cd,	0, },
    { XFER_PIO_3,		0x0500028b,	0, },
    { XFER_PIO_4,		0x0400010a,	0, },
    { XFER_MW_DMA_0,	0x00820800,	0, },
    { XFER_MW_DMA_1,	0x0028b000,	0, },
    { XFER_MW_DMA_2,	0x001ca000,	0, },
    { XFER_UDMA_0,		         0,	0x00035901, },
    { XFER_UDMA_1,		         0,	0x000348b1, },
    { XFER_UDMA_2,		         0,	0x00033881, },
    { XFER_UDMA_3,		         0,	0x00033861, },
    { XFER_UDMA_4,		         0,	0x00033841, },
    { XFER_UDMA_5,		         0,	0x00033031, },
    { XFER_UDMA_6,		         0,	0x00033021, },
    { -1, 0, 0 }
    };
    static const struct pata_macio_timing *pata_macio_find_timing(
    struct pata_macio_priv *priv,
    int mode)
    {
    int i;
    for (i = 0; priv.timings[i].mode > 0; i++) {
    if (priv.timings[i].mode == mode)
    return &priv.timings[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_apply_timings(ap: *mut ata_port, device: c_uint) {
    static void pata_macio_apply_timings(struct ata_port *ap, unsigned int device)
    {
    struct pata_macio_priv *priv = ap.private_data;
    void __iomem *rbase = ap.ioaddr.cmd_addr;
    if (priv.kind == controller_sh_ata6 ||
    priv.kind == controller_un_ata6 ||
    priv.kind == controller_k2_ata6) {
    writel(priv.treg[device][0], rbase + IDE_KAUAI_PIO_CONFIG);
    writel(priv.treg[device][1], rbase + IDE_KAUAI_ULTRA_CONFIG);
    } else
    writel(priv.treg[device][0], rbase + IDE_TIMING_CONFIG);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_dev_select(ap: *mut ata_port, device: c_uint) {
    static void pata_macio_dev_select(struct ata_port *ap, unsigned int device)
    {
    ata_sff_dev_select(ap, device);
// Apply timings
    pata_macio_apply_timings(ap, device);
    }
    static void pata_macio_set_timings(struct ata_port *ap,
    struct ata_device *adev)
    {
    struct pata_macio_priv *priv = ap.private_data;
    const struct pata_macio_timing *t;
    dev_dbg(priv.dev, "Set timings: DEV=%d,PIO=0x%x (%s),DMA=0x%x (%s)\n",
    adev.devno,
    adev.pio_mode,
    ata_mode_string(ata_xfer_mode2mask(adev.pio_mode)),
    adev.dma_mode,
    ata_mode_string(ata_xfer_mode2mask(adev.dma_mode)));
// First clear timings
    priv.treg[adev.devno][0] = priv.treg[adev.devno][1] = 0;
// Now get the PIO timings
    t = pata_macio_find_timing(priv, adev.pio_mode);
    if (t == core::ptr::null_mut()) {
    dev_warn(priv.dev, "Invalid PIO timing requested: 0x%x\n",
    adev.pio_mode);
    t = pata_macio_find_timing(priv, XFER_PIO_0);
    }
    BUG_ON(t == core::ptr::null_mut());
// PIO timings only ever use the first treg
    priv.treg[adev.devno][0] |= t.reg1;
// Now get DMA timings
    t = pata_macio_find_timing(priv, adev.dma_mode);
    if (t == core::ptr::null_mut() || (t.reg1 == 0 && t.reg2 == 0)) {
    dev_dbg(priv.dev, "DMA timing not set yet, using MW_DMA_0\n");
    t = pata_macio_find_timing(priv, XFER_MW_DMA_0);
    }
    BUG_ON(t == core::ptr::null_mut());
// DMA timings can use both tregs
    priv.treg[adev.devno][0] |= t.reg1;
    priv.treg[adev.devno][1] |= t.reg2;
    dev_dbg(priv.dev, " . %08x %08x\n",
    priv.treg[adev.devno][0],
    priv.treg[adev.devno][1]);
// Apply to hardware
    pata_macio_apply_timings(ap, adev.devno);
    }
//
// Blast some well known "safe" values to the timing registers at init or
// wakeup from sleep time, before we do real calculation
//
#[no_mangle]
unsafe extern "C" fn pata_macio_default_timings(priv: *mut pata_macio_priv) {
    static void pata_macio_default_timings(struct pata_macio_priv *priv)
    {
    unsigned int value, value2 = 0;
    switch(priv.kind) {
    case controller_sh_ata6:
    value = 0x0a820c97;
    value2 = 0x00033031;
    break;
    case controller_un_ata6:
    case controller_k2_ata6:
    value = 0x08618a92;
    value2 = 0x00002921;
    break;
    case controller_kl_ata4:
    value = 0x0008438c;
    break;
    case controller_kl_ata3:
    value = 0x00084526;
    break;
    case controller_heathrow:
    case controller_ohare:
    default:
    value = 0x00074526;
    break;
    }
    priv.treg[0][0] = priv.treg[1][0] = value;
    priv.treg[0][1] = priv.treg[1][1] = value2;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_cable_detect(ap: *mut ata_port) -> c_int {
    static int pata_macio_cable_detect(struct ata_port *ap)
    {
    struct pata_macio_priv *priv = ap.private_data;
// Get cable type from device-tree
    if (priv.kind == controller_kl_ata4 ||
    priv.kind == controller_un_ata6 ||
    priv.kind == controller_k2_ata6 ||
    priv.kind == controller_sh_ata6) {
    const char* cable = of_get_property(priv.node, "cable-type",
    core::ptr::null_mut());
    struct device_node *root = of_find_node_by_path("/");
    const char *model = of_get_property(root, "model", core::ptr::null_mut());
    of_node_put(root);
    if (cable && !strncmp(cable, "80-", 3)) {
// Some drives fail to detect 80c cable in PowerBook
// These machine use proprietary short IDE cable
// anyway
//
    if (!strncmp(model, "PowerBook", 9))
    return ATA_CBL_PATA40_SHORT;
    else
    return ATA_CBL_PATA80;
    }
    }
// G5's seem to have incorrect cable type in device-tree.
// Let's assume they always have a 80 conductor cable, this seem to
// be always the case unless the user mucked around
//
    if (of_device_is_compatible(priv.node, "K2-UATA") ||
    of_device_is_compatible(priv.node, "shasta-ata"))
    return ATA_CBL_PATA80;
// Anything else is 40 connectors
    return ATA_CBL_PATA40;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_qc_prep(qc: *mut ata_queued_cmd) -> enum ata_completion_errors {
    static enum ata_completion_errors pata_macio_qc_prep(struct ata_queued_cmd *qc)
    {
    let mut write: c_uint = (qc.tf.flags & ATA_TFLAG_WRITE);
    struct ata_port *ap = qc.ap;
    struct pata_macio_priv *priv = ap.private_data;
    struct scatterlist *sg;
    struct dbdma_cmd *table;
    unsigned int si, pi;
    dev_dbgdma(priv.dev, "%s: qc %p flags %lx, write %d dev %d\n",
    __func__, qc, qc.flags, write, qc.dev.devno);
    if (!(qc.flags & ATA_QCFLAG_DMAMAP))
    return AC_ERR_OK;
    table = (struct dbdma_cmd *) priv.dma_table_cpu;
    pi = 0;
    for_each_sg(qc.sg, sg, qc.n_elem, si) {
    u32 addr, sg_len, len;
// determine if physical DMA addr spans 64K boundary.
// Note h/w doesn't support 64-bit, so we unconditionally
// truncate dma_addr_t to u32.
//
    addr = (u32) sg_dma_address(sg);
    sg_len = sg_dma_len(sg);
    while (sg_len) {
// table overflow should never happen
    if (WARN_ON_ONCE(pi >= MAX_DCMDS))
    return AC_ERR_SYSTEM;
    len = (sg_len < MAX_DBDMA_SEG) ? sg_len : MAX_DBDMA_SEG;
    table.command = cpu_to_le16(write ? OUTPUT_MORE: INPUT_MORE);
    table.req_count = cpu_to_le16(len);
    table.phy_addr = cpu_to_le32(addr);
    table.cmd_dep = 0;
    table.xfer_status = 0;
    table.res_count = 0;
    addr += len;
    sg_len -= len;
    ++table;
    ++pi;
    }
    }
// Should never happen according to Tejun
    if (WARN_ON_ONCE(!pi))
    return AC_ERR_SYSTEM;
// Convert the last command to an input/output
    table--;
    table.command = cpu_to_le16(write ? OUTPUT_LAST: INPUT_LAST);
    table++;
// Add the stop command to the end of the list
    memset(table, 0, sizeof(struct dbdma_cmd));
    table.command = cpu_to_le16(DBDMA_STOP);
    dev_dbgdma(priv.dev, "%s: %d DMA list entries\n", __func__, pi);
    return AC_ERR_OK;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_freeze(ap: *mut ata_port) {
    static void pata_macio_freeze(struct ata_port *ap)
    {
    struct dbdma_regs __iomem *dma_regs = ap.ioaddr.bmdma_addr;
    if (dma_regs) {
    let mut timeout: c_uint = 1000000;
// Make sure DMA controller is stopped
    writel((RUN|PAUSE|FLUSH|WAKE|DEAD) << 16, &dma_regs.control);
    while (--timeout && (readl(&dma_regs.status) & RUN))
    udelay(1);
    }
    ata_sff_freeze(ap);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_bmdma_setup(qc: *mut ata_queued_cmd) {
    static void pata_macio_bmdma_setup(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct pata_macio_priv *priv = ap.private_data;
    struct dbdma_regs __iomem *dma_regs = ap.ioaddr.bmdma_addr;
    let mut dev: c_int = qc.dev.devno;
    dev_dbgdma(priv.dev, "%s: qc %p\n", __func__, qc);
// Make sure DMA commands updates are visible
    writel(priv.dma_table_dma, &dma_regs.cmdptr);
// On KeyLargo 66Mhz cell, we need to add 60ns to wrDataSetup on
// UDMA reads
//
    if (priv.kind == controller_kl_ata4 &&
    (priv.treg[dev][0] & TR_66_UDMA_EN)) {
    void __iomem *rbase = ap.ioaddr.cmd_addr;
    let mut reg: u32 = priv.treg[dev][0];
    if (!(qc.tf.flags & ATA_TFLAG_WRITE))
    reg += 0x00800000;
    writel(reg, rbase + IDE_TIMING_CONFIG);
    }
// issue r/w command
    ap.ops.sff_exec_command(ap, &qc.tf);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_bmdma_start(qc: *mut ata_queued_cmd) {
    static void pata_macio_bmdma_start(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct pata_macio_priv *priv = ap.private_data;
    struct dbdma_regs __iomem *dma_regs = ap.ioaddr.bmdma_addr;
    dev_dbgdma(priv.dev, "%s: qc %p\n", __func__, qc);
    writel((RUN << 16) | RUN, &dma_regs.control);
// Make sure it gets to the controller right now
    (void)readl(&dma_regs.control);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_bmdma_stop(qc: *mut ata_queued_cmd) {
    static void pata_macio_bmdma_stop(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct pata_macio_priv *priv = ap.private_data;
    struct dbdma_regs __iomem *dma_regs = ap.ioaddr.bmdma_addr;
    let mut timeout: c_uint = 1000000;
    dev_dbgdma(priv.dev, "%s: qc %p\n", __func__, qc);
// Stop the DMA engine and wait for it to full halt
    writel (((RUN|WAKE|DEAD) << 16), &dma_regs.control);
    while (--timeout && (readl(&dma_regs.status) & RUN))
    udelay(1);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_bmdma_status(ap: *mut ata_port) -> u8 {
    static u8 pata_macio_bmdma_status(struct ata_port *ap)
    {
    struct pata_macio_priv *priv = ap.private_data;
    struct dbdma_regs __iomem *dma_regs = ap.ioaddr.bmdma_addr;
    u32 dstat, rstat = ATA_DMA_INTR;
    let mut timeout: c_ulong = 0;
    dstat = readl(&dma_regs.status);
    dev_dbgdma(priv.dev, "%s: dstat=%x\n", __func__, dstat);
// We have two things to deal with here:
//
// - The dbdma won't stop if the command was started
// but completed with an error without transferring all
// datas. This happens when bad blocks are met during
// a multi-block transfer.
//
// - The dbdma fifo hasn't yet finished flushing to
// system memory when the disk interrupt occurs.
//
// First check for errors
    if ((dstat & (RUN|DEAD)) != RUN)
    rstat |= ATA_DMA_ERR;
// If ACTIVE is cleared, the STOP command has been hit and
// the transfer is complete. If not, we have to flush the
// channel.
//
    if ((dstat & ACTIVE) == 0)
    return rstat;
    dev_dbgdma(priv.dev, "%s: DMA still active, flushing...\n", __func__);
// If dbdma didn't execute the STOP command yet, the
// active bit is still set. We consider that we aren't
// sharing interrupts (which is hopefully the case with
// those controllers) and so we just try to flush the
// channel for pending data in the fifo
//
    udelay(1);
    writel((FLUSH << 16) | FLUSH, &dma_regs.control);
    for (;;) {
    udelay(1);
    dstat = readl(&dma_regs.status);
    if ((dstat & FLUSH) == 0)
    break;
    if (++timeout > 1000) {
    dev_warn(priv.dev, "timeout flushing DMA\n");
    rstat |= ATA_DMA_ERR;
    break;
    }
    }
    return rstat;
    }
// port_start is when we allocate the DMA command list
#[no_mangle]
unsafe extern "C" fn pata_macio_port_start(ap: *mut ata_port) -> c_int {
    static int pata_macio_port_start(struct ata_port *ap)
    {
    struct pata_macio_priv *priv = ap.private_data;
    if (ap.ioaddr.bmdma_addr == core::ptr::null_mut())
    return 0;
// Allocate space for the DBDMA commands.
//
// The +2 is +1 for the stop command and +1 to allow for
// aligning the start address to a multiple of 16 bytes.
//
    priv.dma_table_cpu =
    dmam_alloc_coherent(priv.dev,
    (MAX_DCMDS + 2) * sizeof(struct dbdma_cmd),
    &priv.dma_table_dma, GFP_KERNEL);
    if (priv.dma_table_cpu == core::ptr::null_mut()) {
    dev_err(priv.dev, "Unable to allocate DMA command list\n");
    ap.ioaddr.bmdma_addr = core::ptr::null_mut();
    ap.mwdma_mask = 0;
    ap.udma_mask = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_irq_clear(ap: *mut ata_port) {
    static void pata_macio_irq_clear(struct ata_port *ap)
    {
    struct pata_macio_priv *priv = ap.private_data;
// Nothing to do here
    dev_dbgdma(priv.dev, "%s\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_reset_hw(priv: *mut pata_macio_priv, resume: c_int) {
    static void pata_macio_reset_hw(struct pata_macio_priv *priv, int resume)
    {
    dev_dbg(priv.dev, "Enabling & resetting...\n");
    if (priv.mediabay)
    return;
    if (priv.kind == controller_ohare && !resume) {
// The code below is having trouble on some ohare machines
// (timing related ?). Until I can put my hand on one of these
// units, I keep the old way
//
    ppc_md.feature_call(PMAC_FTR_IDE_ENABLE, priv.node, 0, 1);
    } else {
    int rc;
// Reset and enable controller
    rc = ppc_md.feature_call(PMAC_FTR_IDE_RESET,
    priv.node, priv.aapl_bus_id, 1);
    ppc_md.feature_call(PMAC_FTR_IDE_ENABLE,
    priv.node, priv.aapl_bus_id, 1);
    msleep(10);
// Only bother waiting if there's a reset control
    if (rc == 0) {
    ppc_md.feature_call(PMAC_FTR_IDE_RESET,
    priv.node, priv.aapl_bus_id, 0);
    msleep(IDE_WAKEUP_DELAY_MS);
    }
    }
// If resuming a PCI device, restore the config space here
    if (priv.pdev && resume) {
    int rc;
    pci_restore_state(priv.pdev);
    rc = pcim_enable_device(priv.pdev);
    if (rc)
    dev_err(&priv.pdev.dev,
    "Failed to enable device after resume (%d)\n",
    rc);
    else
    pci_set_master(priv.pdev);
    }
// On Kauai, initialize the FCR. We don't perform a reset, doesn't really
// seem necessary and speeds up the boot process
//
    if (priv.kauai_fcr)
    writel(KAUAI_FCR_UATA_MAGIC |
    KAUAI_FCR_UATA_RESET_N |
    KAUAI_FCR_UATA_ENABLE, priv.kauai_fcr);
    }
// Hook the standard slave config to fixup some HW related alignment
// restrictions
//
    static int pata_macio_sdev_configure(struct scsi_device *sdev,
    struct queue_limits *lim)
    {
    struct ata_port *ap = ata_shost_to_port(sdev.host);
    struct pata_macio_priv *priv = ap.private_data;
    struct ata_device *dev;
    u16 cmd;
    int rc;
// First call original
    rc = ata_scsi_sdev_configure(sdev, lim);
    if (rc)
    return rc;
// This is lifted from sata_nv
    dev = &ap.link.device[sdev.id];
// OHare has issues with non cache aligned DMA on some chipsets
    if (priv.kind == controller_ohare) {
    lim.dma_alignment = 31;
    lim.dma_pad_mask = 31;
// Tell the world about it
    ata_dev_info(dev, "OHare alignment limits applied\n");
    return 0;
    }
// We only have issues with ATAPI
    if (dev.class != ATA_DEV_ATAPI)
    return 0;
// Shasta and K2 seem to have "issues" with reads ...
    if (priv.kind == controller_sh_ata6 || priv.kind == controller_k2_ata6) {
// Allright these are bad, apply restrictions
    lim.dma_alignment = 15;
    lim.dma_pad_mask = 15;
// We enable MWI and hack cache line size directly here, this
// is specific to this chipset and not normal values, we happen
// to somewhat know what we are doing here (which is basically
// to do the same Apple does and pray they did not get it wrong :-)
//
    BUG_ON(!priv.pdev);
    pci_write_config_byte(priv.pdev, PCI_CACHE_LINE_SIZE, 0x08);
    pci_read_config_word(priv.pdev, PCI_COMMAND, &cmd);
    pci_write_config_word(priv.pdev, PCI_COMMAND,
    cmd | PCI_COMMAND_INVALIDATE);
// Tell the world about it
    ata_dev_info(dev, "K2/Shasta alignment limits applied\n");
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pata_macio_do_suspend(priv: *mut pata_macio_priv, mesg: pm_message_t) -> c_int {
    static int pata_macio_do_suspend(struct pata_macio_priv *priv, pm_message_t mesg)
    {
// First, core libata suspend to do most of the work
    ata_host_suspend(priv.host, mesg);
// Restore to default timings
    pata_macio_default_timings(priv);
// Mask interrupt. Not strictly necessary but old driver did
// it and I'd rather not change that here
    disable_irq(priv.irq);
// The media bay will handle itself just fine
    if (priv.mediabay)
    return 0;
// Kauai has bus control FCRs directly here
    if (priv.kauai_fcr) {
    let mut fcr: u32 = readl(priv.kauai_fcr);
    fcr &= ~(KAUAI_FCR_UATA_RESET_N | KAUAI_FCR_UATA_ENABLE);
    writel(fcr, priv.kauai_fcr);
    }
// For PCI, save state and disable DMA. No need to call
// pci_set_power_state(), the HW doesn't do D states that
// way, the platform code will take care of suspending the
// ASIC properly
//
    if (priv.pdev) {
    pci_save_state(priv.pdev);
    pci_disable_device(priv.pdev);
    }
// Disable the bus on older machines and the cell on kauai
    ppc_md.feature_call(PMAC_FTR_IDE_ENABLE, priv.node,
    priv.aapl_bus_id, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_do_resume(priv: *mut pata_macio_priv) -> c_int {
    static int pata_macio_do_resume(struct pata_macio_priv *priv)
    {
// Reset and re-enable the HW
    pata_macio_reset_hw(priv, 1);
// Sanitize drive timings
    pata_macio_apply_timings(priv.host.ports[0], 0);
// We want our IRQ back !
    enable_irq(priv.irq);
// Let the libata core take it from there
    ata_host_resume(priv.host);
    return 0;
    }

    static const struct scsi_host_template pata_macio_sht = {
    __ATA_BASE_SHT(DRV_NAME),
    .sg_tablesize		= PATA_MACIO_SG_TABLESIZE,
// We may not need that strict one
    .dma_boundary		= ATA_DMA_BOUNDARY,
    .max_segment_size	= PATA_MACIO_MAX_SEGMENT_SIZE,
    .sdev_configure		= pata_macio_sdev_configure,
    .sdev_groups		= ata_common_sdev_groups,
    .can_queue		= ATA_DEF_QUEUE,
    .tag_alloc_policy_rr	= true,
    };
    static struct ata_port_operations pata_macio_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .freeze			= pata_macio_freeze,
    .set_piomode		= pata_macio_set_timings,
    .set_dmamode		= pata_macio_set_timings,
    .cable_detect		= pata_macio_cable_detect,
    .sff_dev_select		= pata_macio_dev_select,
    .qc_prep		= pata_macio_qc_prep,
    .bmdma_setup		= pata_macio_bmdma_setup,
    .bmdma_start		= pata_macio_bmdma_start,
    .bmdma_stop		= pata_macio_bmdma_stop,
    .bmdma_status		= pata_macio_bmdma_status,
    .port_start		= pata_macio_port_start,
    .sff_irq_clear		= pata_macio_irq_clear,
    };
#[no_mangle]
unsafe extern "C" fn pata_macio_invariants(priv: *mut pata_macio_priv) {
    static void pata_macio_invariants(struct pata_macio_priv *priv)
    {
    const int *bidp;
// Identify the type of controller
    if (of_device_is_compatible(priv.node, "shasta-ata")) {
    priv.kind = controller_sh_ata6;
    priv.timings = pata_macio_shasta_timings;
    } else if (of_device_is_compatible(priv.node, "kauai-ata")) {
    priv.kind = controller_un_ata6;
    priv.timings = pata_macio_kauai_timings;
    } else if (of_device_is_compatible(priv.node, "K2-UATA")) {
    priv.kind = controller_k2_ata6;
    priv.timings = pata_macio_kauai_timings;
    } else if (of_device_is_compatible(priv.node, "keylargo-ata")) {
    if (of_node_name_eq(priv.node, "ata-4")) {
    priv.kind = controller_kl_ata4;
    priv.timings = pata_macio_kl66_timings;
    } else {
    priv.kind = controller_kl_ata3;
    priv.timings = pata_macio_kl33_timings;
    }
    } else if (of_device_is_compatible(priv.node, "heathrow-ata")) {
    priv.kind = controller_heathrow;
    priv.timings = pata_macio_heathrow_timings;
    } else {
    priv.kind = controller_ohare;
    priv.timings = pata_macio_ohare_timings;
    }
// XXX FIXME --- setup priv->mediabay here
// Get Apple bus ID (for clock and ASIC control)
    bidp = of_get_property(priv.node, "AAPL,bus-id", core::ptr::null_mut());
    priv.aapl_bus_id =  bidp ? *bidp : 0;
// Fixup missing Apple bus ID in case of media-bay
    if (priv.mediabay && !bidp)
    priv.aapl_bus_id = 1;
    }
    static void pata_macio_setup_ios(struct ata_ioports *ioaddr,
    void __iomem * base, void __iomem * dma)
    {
// cmd_addr is the base of regs for that port
    ioaddr.cmd_addr	= base;
// taskfile registers
    ioaddr.data_addr	= base + (ATA_REG_DATA    << 4);
    ioaddr.error_addr	= base + (ATA_REG_ERR     << 4);
    ioaddr.feature_addr	= base + (ATA_REG_FEATURE << 4);
    ioaddr.nsect_addr	= base + (ATA_REG_NSECT   << 4);
    ioaddr.lbal_addr	= base + (ATA_REG_LBAL    << 4);
    ioaddr.lbam_addr	= base + (ATA_REG_LBAM    << 4);
    ioaddr.lbah_addr	= base + (ATA_REG_LBAH    << 4);
    ioaddr.device_addr	= base + (ATA_REG_DEVICE  << 4);
    ioaddr.status_addr	= base + (ATA_REG_STATUS  << 4);
    ioaddr.command_addr	= base + (ATA_REG_CMD     << 4);
    ioaddr.altstatus_addr	= base + 0x160;
    ioaddr.ctl_addr	= base + 0x160;
    ioaddr.bmdma_addr	= dma;
    }
    static void pmac_macio_calc_timing_masks(struct pata_macio_priv *priv,
    struct ata_port_info *pinfo)
    {
    let mut i: c_int = 0;
    pinfo.pio_mask		= 0;
    pinfo.mwdma_mask	= 0;
    pinfo.udma_mask	= 0;
    while (priv.timings[i].mode > 0) {
    let mut mask: c_uint = 1U << (priv.timings[i].mode & 0x0f);
    switch(priv.timings[i].mode & 0xf0) {
    case 0x00: /* PIO */
    pinfo.pio_mask |= (mask >> 8);
    break;
    case 0x20: /* MWDMA */
    pinfo.mwdma_mask |= mask;
    break;
    case 0x40: /* UDMA */
    pinfo.udma_mask |= mask;
    break;
    }
    i++;
    }
    dev_dbg(priv.dev, "Supported masks: PIO=%x, MWDMA=%x, UDMA=%x\n",
    pinfo.pio_mask, pinfo.mwdma_mask, pinfo.udma_mask);
    }
    static int pata_macio_common_init(struct pata_macio_priv *priv,
    resource_size_t tfregs,
    resource_size_t dmaregs,
    resource_size_t fcregs,
    unsigned long irq)
    {
    struct ata_port_info		pinfo;
    const struct ata_port_info	*ppi[] = { &pinfo, core::ptr::null_mut() };
    void __iomem			*dma_regs = core::ptr::null_mut();
// Fill up privates with various invariants collected from the
// device-tree
//
    pata_macio_invariants(priv);
// Make sure we have sane initial timings in the cache
    pata_macio_default_timings(priv);
// Allocate libata host for 1 port
    memset(&pinfo, 0, sizeof(struct ata_port_info));
    pmac_macio_calc_timing_masks(priv, &pinfo);
    pinfo.flags		= ATA_FLAG_SLAVE_POSS;
    pinfo.port_ops		= &pata_macio_ops;
    pinfo.private_data	= priv;
    priv.host = ata_host_alloc_pinfo(priv.dev, ppi, 1);
    if (priv.host == core::ptr::null_mut()) {
    dev_err(priv.dev, "Failed to allocate ATA port structure\n");
    return -ENOMEM;
    }
// Setup the private data in host too
    priv.host.private_data = priv;
// Map base registers
    priv.tfregs = devm_ioremap(priv.dev, tfregs, 0x100);
    if (priv.tfregs == core::ptr::null_mut()) {
    dev_err(priv.dev, "Failed to map ATA ports\n");
    return -ENOMEM;
    }
    priv.host.iomap = &priv.tfregs;
// Map DMA regs
    if (dmaregs != 0) {
    dma_regs = devm_ioremap(priv.dev, dmaregs,
    sizeof(struct dbdma_regs));
    if (dma_regs == core::ptr::null_mut())
    dev_warn(priv.dev, "Failed to map ATA DMA registers\n");
    }
// If chip has local feature control, map those regs too
    if (fcregs != 0) {
    priv.kauai_fcr = devm_ioremap(priv.dev, fcregs, 4);
    if (priv.kauai_fcr == core::ptr::null_mut()) {
    dev_err(priv.dev, "Failed to map ATA FCR register\n");
    return -ENOMEM;
    }
    }
// Setup port data structure
    pata_macio_setup_ios(&priv.host.ports[0].ioaddr,
    priv.tfregs, dma_regs);
    priv.host.ports[0].private_data = priv;
// hard-reset the controller
    pata_macio_reset_hw(priv, 0);
    pata_macio_apply_timings(priv.host.ports[0], 0);
// Enable bus master if necessary
    if (priv.pdev && dma_regs)
    pci_set_master(priv.pdev);
    dev_info(priv.dev, "Activating pata-macio chipset %s, Apple bus ID %d\n",
    macio_ata_names[priv.kind], priv.aapl_bus_id);
// Start it up
    priv.irq = irq;
    return ata_host_activate(priv.host, irq, ata_bmdma_interrupt, 0,
    &pata_macio_sht);
    }
    static int pata_macio_attach(struct macio_dev *mdev,
    const struct of_device_id *match)
    {
    struct pata_macio_priv	*priv;
    resource_size_t		tfregs, dmaregs = 0;
    unsigned long		irq;
    int			rc;
// Check for broken device-trees
    if (macio_resource_count(mdev) == 0) {
    dev_err(&mdev.ofdev.dev,
    "No addresses for controller\n");
    return -ENXIO;
    }
// Enable managed resources
    macio_enable_devres(mdev);
// Allocate and init private data structure
    priv = devm_kzalloc(&mdev.ofdev.dev,
    sizeof(struct pata_macio_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.node = of_node_get(mdev.ofdev.dev.of_node);
    priv.mdev = mdev;
    priv.dev = &mdev.ofdev.dev;
// Request memory resource for taskfile registers
    if (macio_request_resource(mdev, 0, "pata-macio")) {
    dev_err(&mdev.ofdev.dev,
    "Cannot obtain taskfile resource\n");
    return -EBUSY;
    }
    tfregs = macio_resource_start(mdev, 0);
// Request resources for DMA registers if any
    if (macio_resource_count(mdev) >= 2) {
    if (macio_request_resource(mdev, 1, "pata-macio-dma"))
    dev_err(&mdev.ofdev.dev,
    "Cannot obtain DMA resource\n");
    else
    dmaregs = macio_resource_start(mdev, 1);
    }
//
// Fixup missing IRQ for some old implementations with broken
// device-trees.
//
// This is a bit bogus, it should be fixed in the device-tree itself,
// via the existing macio fixups, based on the type of interrupt
// controller in the machine. However, I have no test HW for this case,
// and this trick works well enough on those old machines...
//
    if (macio_irq_count(mdev) == 0) {
    dev_warn(&mdev.ofdev.dev,
    "No interrupts for controller, using 13\n");
    irq = irq_create_mapping(core::ptr::null_mut(), 13);
    } else
    irq = macio_irq(mdev, 0);
// Prevvent media bay callbacks until fully registered
    lock_media_bay(priv.mdev.media_bay);
// Get register addresses and call common initialization
    rc = pata_macio_common_init(priv,
    tfregs,		/* Taskfile regs */
    dmaregs,		/* DBDMA regs */
    0,			/* Feature control */
    irq);
    unlock_media_bay(priv.mdev.media_bay);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_detach(mdev: *mut macio_dev) {
    static void pata_macio_detach(struct macio_dev *mdev)
    {
    struct ata_host *host = macio_get_drvdata(mdev);
    struct pata_macio_priv *priv = host.private_data;
    lock_media_bay(priv.mdev.media_bay);
// Make sure the mediabay callback doesn't try to access
// dead stuff
//
    priv.host.private_data = core::ptr::null_mut();
    ata_host_detach(host);
    unlock_media_bay(priv.mdev.media_bay);
    }

#[no_mangle]
unsafe extern "C" fn pata_macio_suspend(mdev: *mut macio_dev, mesg: pm_message_t) -> c_int {
    static int pata_macio_suspend(struct macio_dev *mdev, pm_message_t mesg)
    {
    struct ata_host *host = macio_get_drvdata(mdev);
    return pata_macio_do_suspend(host.private_data, mesg);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_resume(mdev: *mut macio_dev) -> c_int {
    static int pata_macio_resume(struct macio_dev *mdev)
    {
    struct ata_host *host = macio_get_drvdata(mdev);
    return pata_macio_do_resume(host.private_data);
    }

#[no_mangle]
unsafe extern "C" fn pata_macio_mb_event(mdev: *mut *mut macio_dev, mb_state: c_int) {
    static void pata_macio_mb_event(struct macio_dev* mdev, int mb_state)
    {
    struct ata_host *host = macio_get_drvdata(mdev);
    struct ata_port *ap;
    struct ata_eh_info *ehi;
    struct ata_device *dev;
    unsigned long flags;
    if (!host || !host.private_data)
    return;
    ap = host.ports[0];
    spin_lock_irqsave(ap.lock, flags);
    ehi = &ap.link.eh_info;
    if (mb_state == MB_CD) {
    ata_ehi_push_desc(ehi, "mediabay plug");
    ata_ehi_hotplugged(ehi);
    ata_port_freeze(ap);
    } else {
    ata_ehi_push_desc(ehi, "mediabay unplug");
    ata_for_each_dev(dev, &ap.link, ALL)
    dev.flags |= ATA_DFLAG_DETACH;
    ata_port_abort(ap);
    }
    spin_unlock_irqrestore(ap.lock, flags);
    }

    static int pata_macio_pci_attach(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct pata_macio_priv	*priv;
    struct device_node	*np;
    resource_size_t		rbase;
// We cannot use a MacIO controller without its OF device node
    np = pci_device_to_OF_node(pdev);
    if (np == core::ptr::null_mut()) {
    dev_err(&pdev.dev,
    "Cannot find OF device node for controller\n");
    return -ENODEV;
    }
// Check that it can be enabled
    if (pcim_enable_device(pdev)) {
    dev_err(&pdev.dev,
    "Cannot enable controller PCI device\n");
    return -ENXIO;
    }
// Allocate and init private data structure
    priv = devm_kzalloc(&pdev.dev,
    sizeof(struct pata_macio_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.node = of_node_get(np);
    priv.pdev = pdev;
    priv.dev = &pdev.dev;
// Get MMIO regions
    if (pcim_request_all_regions(pdev, "pata-macio")) {
    dev_err(&pdev.dev,
    "Cannot obtain PCI resources\n");
    return -EBUSY;
    }
// Get register addresses and call common initialization
    rbase = pci_resource_start(pdev, 0);
    if (pata_macio_common_init(priv,
    rbase + 0x2000,	/* Taskfile regs */
    rbase + 0x1000,	/* DBDMA regs */
    rbase,		/* Feature control */
    pdev.irq))
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_pci_detach(pdev: *mut pci_dev) {
    static void pata_macio_pci_detach(struct pci_dev *pdev)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    ata_host_detach(host);
    }

#[no_mangle]
unsafe extern "C" fn pata_macio_pci_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> c_int {
    static int pata_macio_pci_suspend(struct pci_dev *pdev, pm_message_t mesg)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    return pata_macio_do_suspend(host.private_data, mesg);
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_pci_resume(pdev: *mut pci_dev) -> c_int {
    static int pata_macio_pci_resume(struct pci_dev *pdev)
    {
    struct ata_host *host = pci_get_drvdata(pdev);
    return pata_macio_do_resume(host.private_data);
    }

    static const struct of_device_id pata_macio_match[] =
    {
    { .name = "IDE", },
    { .name = "ATA", },
    { .type = "ide", },
    { .type = "ata", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pata_macio_match);
    static struct macio_driver pata_macio_driver =
    {
    .driver = {
    .name 		= "pata-macio",
    .owner		= THIS_MODULE,
    .of_match_table	= pata_macio_match,
    },
    .probe		= pata_macio_attach,
    .remove		= pata_macio_detach,

    .suspend	= pata_macio_suspend,
    .resume		= pata_macio_resume,

    .mediabay_event	= pata_macio_mb_event,

    };
    static const struct pci_device_id pata_macio_pci_match[] = {
    { PCI_VDEVICE(APPLE, PCI_DEVICE_ID_APPLE_UNI_N_ATA) },
    { PCI_VDEVICE(APPLE, PCI_DEVICE_ID_APPLE_IPID_ATA100) },
    { PCI_VDEVICE(APPLE, PCI_DEVICE_ID_APPLE_K2_ATA100) },
    { PCI_VDEVICE(APPLE, PCI_DEVICE_ID_APPLE_SH_ATA) },
    { PCI_VDEVICE(APPLE, PCI_DEVICE_ID_APPLE_IPID2_ATA) },
    { }
    };
    static struct pci_driver pata_macio_pci_driver = {
    .name		= "pata-pci-macio",
    .id_table	= pata_macio_pci_match,
    .probe		= pata_macio_pci_attach,
    .remove		= pata_macio_pci_detach,

    .suspend	= pata_macio_pci_suspend,
    .resume		= pata_macio_pci_resume,

    };
    MODULE_DEVICE_TABLE(pci, pata_macio_pci_match);
#[no_mangle]
unsafe extern "C" fn pata_macio_init() -> int __init {
    static int __init pata_macio_init(void)
    {
    int rc;
    if (!machine_is(powermac))
    return -ENODEV;
    rc = pci_register_driver(&pata_macio_pci_driver);
    if (rc)
    return rc;
    rc = macio_register_driver(&pata_macio_driver);
    if (rc) {
    pci_unregister_driver(&pata_macio_pci_driver);
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pata_macio_exit() -> void __exit {
    static void __exit pata_macio_exit(void)
    {
    macio_unregister_driver(&pata_macio_driver);
    pci_unregister_driver(&pata_macio_pci_driver);
    }
    module_init(pata_macio_init);
    module_exit(pata_macio_exit);
    MODULE_AUTHOR("Benjamin Herrenschmidt");
    MODULE_DESCRIPTION("Apple MacIO PATA driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_VERSION);
