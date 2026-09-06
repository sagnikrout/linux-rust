//! Automatically rewritten from C to Rust
//! Source: drivers/ata/sata_dwc_460ex.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drivers/ata/sata_dwc_460ex.c
//
// Synopsys DesignWare Cores (DWC) SATA host driver
//
// Author: Mark Miesfeld <mmiesfeld@amcc.com>
//
// Ported from 2.6.19.2 to 2.6.25/26 by Stefan Roese <sr@denx.de>
// Copyright 2008 DENX Software Engineering
//
// Based on versions provided by AMCC and Synopsys which are:
// Copyright 2006 Applied Micro Circuits Corporation
// COPYRIGHT (C) 2005  SYNOPSYS, INC.  ALL RIGHTS RESERVED
//

// These two are defined in "libata.h"

    enum {
    SATA_DWC_MAX_PORTS = 1,
    SATA_DWC_SCR_OFFSET = 0x24,
    SATA_DWC_REG_OFFSET = 0x64,
    };
// DWC SATA Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_dwc_regs {
    pub /: *mut *mut u32 fptagr; / 1st party DMA tag,
    pub /: *mut *mut u32 fpbor; / 1st party DMA buffer offset,
    pub /: *mut *mut u32 fptcr; / 1st party DMA Xfr count,
    pub /: *mut *mut u32 dmacr; / DMA Control,
    pub /: *mut *mut u32 dbtsr; / DMA Burst Transac size,
    pub /: *mut *mut u32 intpr; / Interrupt Pending,
    pub /: *mut *mut u32 intmr; / Interrupt Mask,
    pub /: *mut *mut u32 errmr; / Error Mask,
    pub /: *mut *mut u32 llcr; / Link Layer Control,
    pub /: *mut *mut u32 phycr; / PHY Control,
    pub /: *mut *mut u32 physr; / PHY Status,
    pub /: *mut *mut u32 rxbistpd; / Recvd BIST pattern def register,
    pub /: *mut *mut u32 rxbistpd1; / Recvd BIST data dword1,
    pub /: *mut *mut u32 rxbistpd2; / Recvd BIST pattern data dword2,
    pub /: *mut *mut u32 txbistpd; / Trans BIST pattern def register,
    pub /: *mut *mut u32 txbistpd1; / Trans BIST data dword1,
    pub /: *mut *mut u32 txbistpd2; / Trans BIST data dword2,
    pub /: *mut *mut u32 bistcr; / BIST Control Register,
    pub /: *mut *mut u32 bistfctr; / BIST FIS Count Register,
    pub /: *mut *mut u32 bistsr; / BIST Status Register,
    pub /: *mut *mut u32 bistdecr; / BIST Dword Error count register,
    pub /: *mut *mut u32 res[15]; / Reserved locations,
    pub /: *mut *mut u32 testr; / Test Register,
    pub /: *mut *mut u32 versionr; / Version Register,
    pub /: *mut *mut u32 idr; / ID Register,
    pub /: *mut *mut u32 unimpl[192]; / Unimplemented,
    pub /: *mut *mut u32 dmadr[256]; / FIFO Locations in DMA Mode,
}

    enum {
    SCR_SCONTROL_DET_ENABLE	=	0x00000001,
    SCR_SSTATUS_DET_PRESENT	=	0x00000001,
    SCR_SERROR_DIAG_X	=	0x04000000,
// DWC SATA Register Operations
    SATA_DWC_TXFIFO_DEPTH	=	0x01FF,
    SATA_DWC_RXFIFO_DEPTH	=	0x01FF,
    SATA_DWC_DMACR_TMOD_TXCHEN =	0x00000004,
    SATA_DWC_DMACR_TXCHEN	= (0x00000001 | SATA_DWC_DMACR_TMOD_TXCHEN),
    SATA_DWC_DMACR_RXCHEN	= (0x00000002 | SATA_DWC_DMACR_TMOD_TXCHEN),
    SATA_DWC_DMACR_TXRXCH_CLEAR =	SATA_DWC_DMACR_TMOD_TXCHEN,
    SATA_DWC_INTPR_DMAT	=	0x00000001,
    SATA_DWC_INTPR_NEWFP	=	0x00000002,
    SATA_DWC_INTPR_PMABRT	=	0x00000004,
    SATA_DWC_INTPR_ERR	=	0x00000008,
    SATA_DWC_INTPR_NEWBIST	=	0x00000010,
    SATA_DWC_INTPR_IPF	=	0x10000000,
    SATA_DWC_INTMR_DMATM	=	0x00000001,
    SATA_DWC_INTMR_NEWFPM	=	0x00000002,
    SATA_DWC_INTMR_PMABRTM	=	0x00000004,
    SATA_DWC_INTMR_ERRM	=	0x00000008,
    SATA_DWC_INTMR_NEWBISTM	=	0x00000010,
    SATA_DWC_LLCR_SCRAMEN	=	0x00000001,
    SATA_DWC_LLCR_DESCRAMEN	=	0x00000002,
    SATA_DWC_LLCR_RPDEN	=	0x00000004,
// This is all error bits, zero's are reserved fields.
    SATA_DWC_SERROR_ERR_BITS =	0x0FFF0F03
    };

    SATA_DWC_DMACR_TMOD_TXCHEN)

    SATA_DWC_DMACR_TMOD_TXCHEN)

    << 16)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_dwc_device {
    pub /: *mut *mut *mut device dev; / generic device struct,
    pub /: *mut *mut *mut ata_probe_ent pe; / ptr to probe-ent,
    pub host: *mut ata_host,
    pub /: *mut *mut *mut sata_dwc_regs __iomem sata_dwc_regs; / DW SATA specific,
    pub sactive_issued: u32,
    pub sactive_queued: u32,
    pub phy: *mut phy,
    pub dmadr: phys_addr_t,

    pub dma: *mut dw_dma_chip,

}

//
// Allow one extra special slot for commands and DMA management
// to account for libata internal commands.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_dwc_device_port {
    pub hsdev: *mut sata_dwc_device,
    pub cmd_issued: [c_int; SATA_DWC_QCMD_MAX],
    pub dma_pending: [c_int; SATA_DWC_QCMD_MAX],
// DMA info
    pub chan: *mut dma_chan,
    pub desc: [*mut dma_async_tx_descriptor; SATA_DWC_QCMD_MAX],
    pub dma_interrupt_count: u32,
}

//
// Commonly used DWC SATA driver macros
//

    enum {
    SATA_DWC_CMD_ISSUED_NOT		= 0,
    SATA_DWC_CMD_ISSUED_PEND	= 1,
    SATA_DWC_CMD_ISSUED_EXEC	= 2,
    SATA_DWC_CMD_ISSUED_NODATA	= 3,
    SATA_DWC_DMA_PENDING_NONE	= 0,
    SATA_DWC_DMA_PENDING_TX		= 1,
    SATA_DWC_DMA_PENDING_RX		= 2,
    };
//
// Prototypes
//
    static void sata_dwc_bmdma_start_by_tag(struct ata_queued_cmd *qc, u8 tag);
    static int sata_dwc_qc_complete(struct ata_port *ap, struct ata_queued_cmd *qc);
    static void sata_dwc_dma_xfer_complete(struct ata_port *ap);
    static void sata_dwc_clear_dmacr(struct sata_dwc_device_port *hsdevp, u8 tag);

    static struct dw_dma_slave sata_dwc_dma_dws = {
    .src_id = 0,
    .dst_id = 0,
    .m_master = 1,
    .p_master = 0,
    };
#[no_mangle]
unsafe extern "C" fn sata_dwc_dma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool sata_dwc_dma_filter(struct dma_chan *chan, void *param)
    {
    struct dw_dma_slave *dws = &sata_dwc_dma_dws;
    if (dws.dma_dev != chan.device.dev)
    return false;
    chan.private = dws;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_dma_get_channel_old(hsdevp: *mut sata_dwc_device_port) -> c_int {
    static int sata_dwc_dma_get_channel_old(struct sata_dwc_device_port *hsdevp)
    {
    struct sata_dwc_device *hsdev = hsdevp.hsdev;
    struct dw_dma_slave *dws = &sata_dwc_dma_dws;
    struct device *dev = hsdev.dev;
    dma_cap_mask_t mask;
    dws.dma_dev = dev;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
// Acquire DMA channel
    hsdevp.chan = dma_request_channel(mask, sata_dwc_dma_filter, hsdevp);
    if (!hsdevp.chan) {
    dev_err(dev, "%s: dma channel unavailable\n", __func__);
    return -EAGAIN;
    }
    return 0;
    }
    static int sata_dwc_dma_init_old(struct platform_device *pdev,
    struct sata_dwc_device *hsdev)
    {
    struct device *dev = &pdev.dev;
    hsdev.dma = devm_kzalloc(dev, sizeof(*hsdev.dma), GFP_KERNEL);
    if (!hsdev.dma)
    return -ENOMEM;
    hsdev.dma.dev = dev;
    hsdev.dma.id = pdev.id;
// Get SATA DMA interrupt number
    hsdev.dma.irq = platform_get_irq(pdev, 1);
    if (hsdev.dma.irq < 0)
    return hsdev.dma.irq;
// Get physical SATA DMA register base address
    hsdev.dma.regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(hsdev.dma.regs))
    return PTR_ERR(hsdev.dma.regs);
// Initialize AHB DMAC
    return dw_dma_probe(hsdev.dma);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_dma_exit_old(hsdev: *mut sata_dwc_device) {
    static void sata_dwc_dma_exit_old(struct sata_dwc_device *hsdev)
    {
    if (!hsdev.dma)
    return;
    dw_dma_remove(hsdev.dma);
    }

    static const char *get_prot_descript(u8 protocol)
    {
    switch (protocol) {
    case ATA_PROT_NODATA:
    return "ATA no data";
    case ATA_PROT_PIO:
    return "ATA PIO";
    case ATA_PROT_DMA:
    return "ATA DMA";
    case ATA_PROT_NCQ:
    return "ATA NCQ";
    case ATA_PROT_NCQ_NODATA:
    return "ATA NCQ no data";
    case ATAPI_PROT_NODATA:
    return "ATAPI no data";
    case ATAPI_PROT_PIO:
    return "ATAPI PIO";
    case ATAPI_PROT_DMA:
    return "ATAPI DMA";
    default:
    return "unknown";
    }
    }
#[no_mangle]
unsafe extern "C" fn dma_dwc_xfer_done(hsdev_instance: *mut c_void) {
    static void dma_dwc_xfer_done(void *hsdev_instance)
    {
    unsigned long flags;
    struct sata_dwc_device *hsdev = hsdev_instance;
    struct ata_host *host = (struct ata_host *)hsdev.host;
    struct ata_port *ap;
    struct sata_dwc_device_port *hsdevp;
    let mut tag: u8 = 0;
    let mut port: c_uint = 0;
    spin_lock_irqsave(&host.lock, flags);
    ap = host.ports[port];
    hsdevp = HSDEVP_FROM_AP(ap);
    tag = ap.link.active_tag;
//
// Each DMA command produces 2 interrupts.  Only
// complete the command after both interrupts have been
// seen. (See sata_dwc_isr())
//
    hsdevp.dma_interrupt_count++;
    sata_dwc_clear_dmacr(hsdevp, tag);
    if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_NONE) {
    dev_err(ap.dev, "DMA not pending tag=0x%02x pending=%d\n",
    tag, hsdevp.dma_pending[tag]);
    }
    if ((hsdevp.dma_interrupt_count % 2) == 0)
    sata_dwc_dma_xfer_complete(ap);
    spin_unlock_irqrestore(&host.lock, flags);
    }
    static struct dma_async_tx_descriptor *dma_dwc_xfer_setup(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    struct sata_dwc_device *hsdev = HSDEV_FROM_AP(ap);
    struct dma_slave_config sconf;
    struct dma_async_tx_descriptor *desc;
    if (qc.dma_dir == DMA_DEV_TO_MEM) {
    sconf.src_addr = hsdev.dmadr;
    sconf.device_fc = false;
    } else {	/* DMA_MEM_TO_DEV */
    sconf.dst_addr = hsdev.dmadr;
    sconf.device_fc = false;
    }
    sconf.direction = qc.dma_dir;
    sconf.src_maxburst = AHB_DMA_BRST_DFLT / 4;	/* in items */
    sconf.dst_maxburst = AHB_DMA_BRST_DFLT / 4;	/* in items */
    sconf.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    sconf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dmaengine_slave_config(hsdevp.chan, &sconf);
// Convert SG list to linked list of items (LLIs) for AHB DMA
    desc = dmaengine_prep_slave_sg(hsdevp.chan, qc.sg, qc.n_elem,
    qc.dma_dir,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc)
    return core::ptr::null_mut();
    desc.callback = dma_dwc_xfer_done;
    desc.callback_param = hsdev;
    dev_dbg(hsdev.dev, "%s sg: 0x%p, count: %d addr: %pa\n", __func__,
    qc.sg, qc.n_elem, &hsdev.dmadr);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_scr_read(link: *mut ata_link, scr: c_uint, val: *mut u32) -> c_int {
    static int sata_dwc_scr_read(struct ata_link *link, unsigned int scr, u32 *val)
    {
    if (scr > SCR_NOTIFICATION) {
    dev_err(link.ap.dev, "%s: Incorrect SCR offset 0x%02x\n",
    __func__, scr);
    return -EINVAL;
    }
// val = sata_dwc_readl(link->ap->ioaddr.scr_addr + (scr * 4));
    dev_dbg(link.ap.dev, "%s: id=%d reg=%d val=0x%08x\n", __func__,
    link.ap.print_id, scr, *val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_scr_write(link: *mut ata_link, scr: c_uint, val: u32) -> c_int {
    static int sata_dwc_scr_write(struct ata_link *link, unsigned int scr, u32 val)
    {
    dev_dbg(link.ap.dev, "%s: id=%d reg=%d val=0x%08x\n", __func__,
    link.ap.print_id, scr, val);
    if (scr > SCR_NOTIFICATION) {
    dev_err(link.ap.dev, "%s: Incorrect SCR offset 0x%02x\n",
    __func__, scr);
    return -EINVAL;
    }
    sata_dwc_writel(link.ap.ioaddr.scr_addr + (scr * 4), val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clear_serror(ap: *mut ata_port) {
    static void clear_serror(struct ata_port *ap)
    {
    u32 val;
    sata_dwc_scr_read(&ap.link, SCR_ERROR, &val);
    sata_dwc_scr_write(&ap.link, SCR_ERROR, val);
    }
#[no_mangle]
unsafe extern "C" fn clear_interrupt_bit(hsdev: *mut sata_dwc_device, bit: u32) {
    static void clear_interrupt_bit(struct sata_dwc_device *hsdev, u32 bit)
    {
    sata_dwc_writel(&hsdev.sata_dwc_regs.intpr, bit);
    }
#[no_mangle]
unsafe extern "C" fn qcmd_tag_to_mask(tag: u8) -> u32 {
    static u32 qcmd_tag_to_mask(u8 tag)
    {
    return 0x00000001 << (tag & 0x1f);
    }
// See ahci.c
    static void sata_dwc_error_intr(struct ata_port *ap,
    struct sata_dwc_device *hsdev, uint intpr)
    {
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    struct ata_eh_info *ehi = &ap.link.eh_info;
    let mut err_mask: c_uint = 0, action = 0;
    struct ata_queued_cmd *qc;
    u32 serror;
    u8 status, tag;
    ata_ehi_clear_desc(ehi);
    sata_dwc_scr_read(&ap.link, SCR_ERROR, &serror);
    status = ap.ops.sff_check_status(ap);
    tag = ap.link.active_tag;
    dev_err(ap.dev,
    "%s SCR_ERROR=0x%08x intpr=0x%08x status=0x%08x dma_intp=%d pending=%d issued=%d",
    __func__, serror, intpr, status, hsdevp.dma_interrupt_count,
    hsdevp.dma_pending[tag], hsdevp.cmd_issued[tag]);
// Clear error register and interrupt bit
    clear_serror(ap);
    clear_interrupt_bit(hsdev, SATA_DWC_INTPR_ERR);
// This is the only error happening now.  TODO check for exact error
    err_mask |= AC_ERR_HOST_BUS;
    action |= ATA_EH_RESET;
// Pass this on to EH
    ehi.serror |= serror;
    ehi.action |= action;
    qc = ata_qc_from_tag(ap, tag);
    if (qc)
    qc.err_mask |= err_mask;
    else
    ehi.err_mask |= err_mask;
    ata_port_abort(ap);
    }
//
// Function : sata_dwc_isr
// arguments : irq, void *dev_instance, struct pt_regs *regs
// Return value : irqreturn_t - status of IRQ
// This Interrupt handler called via port ops registered function.
// .irq_handler = sata_dwc_isr
//
#[no_mangle]
unsafe extern "C" fn sata_dwc_isr(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t sata_dwc_isr(int irq, void *dev_instance)
    {
    struct ata_host *host = (struct ata_host *)dev_instance;
    struct sata_dwc_device *hsdev = HSDEV_FROM_HOST(host);
    struct ata_port *ap;
    struct ata_queued_cmd *qc;
    unsigned long flags;
    u8 status, tag;
    int handled, port = 0;
    uint intpr, sactive, sactive2, tag_mask;
    struct sata_dwc_device_port *hsdevp;
    hsdev.sactive_issued = 0;
    spin_lock_irqsave(&host.lock, flags);
// Read the interrupt register
    intpr = sata_dwc_readl(&hsdev.sata_dwc_regs.intpr);
    ap = host.ports[port];
    hsdevp = HSDEVP_FROM_AP(ap);
    dev_dbg(ap.dev, "%s intpr=0x%08x active_tag=%d\n", __func__, intpr,
    ap.link.active_tag);
// Check for error interrupt
    if (intpr & SATA_DWC_INTPR_ERR) {
    sata_dwc_error_intr(ap, hsdev, intpr);
    handled = 1;
    goto DONE;
    }
// Check for DMA SETUP FIS (FP DMA) interrupt
    if (intpr & SATA_DWC_INTPR_NEWFP) {
    clear_interrupt_bit(hsdev, SATA_DWC_INTPR_NEWFP);
    tag = (u8)(sata_dwc_readl(&hsdev.sata_dwc_regs.fptagr));
    dev_dbg(ap.dev, "%s: NEWFP tag=%d\n", __func__, tag);
    if (hsdevp.cmd_issued[tag] != SATA_DWC_CMD_ISSUED_PEND)
    dev_warn(ap.dev, "CMD tag=%d not pending?\n", tag);
    hsdev.sactive_issued |= qcmd_tag_to_mask(tag);
    qc = ata_qc_from_tag(ap, tag);
    if (unlikely(!qc)) {
    dev_err(ap.dev, "failed to get qc");
    handled = 1;
    goto DONE;
    }
//
// Start FP DMA for NCQ command.  At this point the tag is the
// active tag.  It is the tag that matches the command about to
// be completed.
//
    trace_ata_bmdma_start(ap, &qc.tf, tag);
    qc.ap.link.active_tag = tag;
    sata_dwc_bmdma_start_by_tag(qc, tag);
    handled = 1;
    goto DONE;
    }
    sata_dwc_scr_read(&ap.link, SCR_ACTIVE, &sactive);
    tag_mask = (hsdev.sactive_issued | sactive) ^ sactive;
// If no sactive issued and tag_mask is zero then this is not NCQ
    if (hsdev.sactive_issued == 0 && tag_mask == 0) {
    if (ap.link.active_tag == ATA_TAG_POISON)
    tag = 0;
    else
    tag = ap.link.active_tag;
    qc = ata_qc_from_tag(ap, tag);
// DEV interrupt w/ no active qc?
    if (unlikely(!qc || (qc.tf.flags & ATA_TFLAG_POLLING))) {
    dev_err(ap.dev,
    "%s interrupt with no active qc qc=%p\n",
    __func__, qc);
    ap.ops.sff_check_status(ap);
    handled = 1;
    goto DONE;
    }
    status = ap.ops.sff_check_status(ap);
    qc.ap.link.active_tag = tag;
    hsdevp.cmd_issued[tag] = SATA_DWC_CMD_ISSUED_NOT;
    if (status & ATA_ERR) {
    dev_dbg(ap.dev, "interrupt ATA_ERR (0x%x)\n", status);
    sata_dwc_qc_complete(ap, qc);
    handled = 1;
    goto DONE;
    }
    dev_dbg(ap.dev, "%s non-NCQ cmd interrupt, protocol: %s\n",
    __func__, get_prot_descript(qc.tf.protocol));
    DRVSTILLBUSY:
    if (ata_is_dma(qc.tf.protocol)) {
//
// Each DMA transaction produces 2 interrupts. The DMAC
// transfer complete interrupt and the SATA controller
// operation done interrupt. The command should be
// completed only after both interrupts are seen.
//
    hsdevp.dma_interrupt_count++;
    if (hsdevp.dma_pending[tag] == \
    SATA_DWC_DMA_PENDING_NONE) {
    dev_err(ap.dev,
    "%s: DMA not pending intpr=0x%08x status=0x%08x pending=%d\n",
    __func__, intpr, status,
    hsdevp.dma_pending[tag]);
    }
    if ((hsdevp.dma_interrupt_count % 2) == 0)
    sata_dwc_dma_xfer_complete(ap);
    } else if (ata_is_pio(qc.tf.protocol)) {
    ata_sff_hsm_move(ap, qc, status, 0);
    handled = 1;
    goto DONE;
    } else {
    if (unlikely(sata_dwc_qc_complete(ap, qc)))
    goto DRVSTILLBUSY;
    }
    handled = 1;
    goto DONE;
    }
//
// This is a NCQ command. At this point we need to figure out for which
// tags we have gotten a completion interrupt.  One interrupt may serve
// as completion for more than one operation when commands are queued
// (NCQ).  We need to process each completed command.
//
// process completed commands
    sata_dwc_scr_read(&ap.link, SCR_ACTIVE, &sactive);
    tag_mask = (hsdev.sactive_issued | sactive) ^ sactive;
    if (sactive != 0 || hsdev.sactive_issued > 1 || tag_mask > 1) {
    dev_dbg(ap.dev,
    "%s NCQ:sactive=0x%08x  sactive_issued=0x%08x tag_mask=0x%08x\n",
    __func__, sactive, hsdev.sactive_issued, tag_mask);
    }
    if ((tag_mask | hsdev.sactive_issued) != hsdev.sactive_issued) {
    dev_warn(ap.dev,
    "Bad tag mask?  sactive=0x%08x sactive_issued=0x%08x  tag_mask=0x%08x\n",
    sactive, hsdev.sactive_issued, tag_mask);
    }
// read just to clear ... not bad if currently still busy
    status = ap.ops.sff_check_status(ap);
    dev_dbg(ap.dev, "%s ATA status register=0x%x\n", __func__, status);
    while (tag_mask) {
    tag = __ffs(tag_mask);
    tag_mask &= ~(1U << tag);
    qc = ata_qc_from_tag(ap, tag);
    if (unlikely(!qc)) {
    dev_err(ap.dev, "failed to get qc");
    handled = 1;
    goto DONE;
    }
// To be picked up by completion functions
    qc.ap.link.active_tag = tag;
    hsdevp.cmd_issued[tag] = SATA_DWC_CMD_ISSUED_NOT;
// Let libata/scsi layers handle error
    if (status & ATA_ERR) {
    dev_dbg(ap.dev, "%s ATA_ERR (0x%x)\n", __func__,
    status);
    sata_dwc_qc_complete(ap, qc);
    handled = 1;
    goto DONE;
    }
// Process completed command
    dev_dbg(ap.dev, "%s NCQ command, protocol: %s\n", __func__,
    get_prot_descript(qc.tf.protocol));
    if (ata_is_dma(qc.tf.protocol)) {
    hsdevp.dma_interrupt_count++;
    if (hsdevp.dma_pending[tag] == \
    SATA_DWC_DMA_PENDING_NONE)
    dev_warn(ap.dev, "%s: DMA not pending?\n",
    __func__);
    if ((hsdevp.dma_interrupt_count % 2) == 0)
    sata_dwc_dma_xfer_complete(ap);
    } else {
    if (unlikely(sata_dwc_qc_complete(ap, qc)))
    goto STILLBUSY;
    }
    continue;
    STILLBUSY:
    ap.stats.idle_irq++;
    dev_warn(ap.dev, "STILL BUSY IRQ ata%d: irq trap\n",
    ap.print_id);
    } /* while tag_mask */
//
// Check to see if any commands completed while we were processing our
// initial set of completed commands (read status clears interrupts,
// so we might miss a completed command interrupt if one came in while
// we were processing --we read status as part of processing a completed
// command).
//
    sata_dwc_scr_read(&ap.link, SCR_ACTIVE, &sactive2);
    if (sactive2 != sactive) {
    dev_dbg(ap.dev,
    "More completed - sactive=0x%x sactive2=0x%x\n",
    sactive, sactive2);
    }
    handled = 1;
    DONE:
    spin_unlock_irqrestore(&host.lock, flags);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_clear_dmacr(hsdevp: *mut sata_dwc_device_port, tag: u8) {
    static void sata_dwc_clear_dmacr(struct sata_dwc_device_port *hsdevp, u8 tag)
    {
    struct sata_dwc_device *hsdev = HSDEV_FROM_HSDEVP(hsdevp);
    let mut dmacr: u32 = sata_dwc_readl(&hsdev.sata_dwc_regs.dmacr);
    if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_RX) {
    dmacr = SATA_DWC_DMACR_RX_CLEAR(dmacr);
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr, dmacr);
    } else if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_TX) {
    dmacr = SATA_DWC_DMACR_TX_CLEAR(dmacr);
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr, dmacr);
    } else {
//
// This should not happen, it indicates the driver is out of
// sync.  If it does happen, clear dmacr anyway.
//
    dev_err(hsdev.dev,
    "%s DMA protocol RX and TX DMA not pending tag=0x%02x pending=%d dmacr: 0x%08x\n",
    __func__, tag, hsdevp.dma_pending[tag], dmacr);
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr,
    SATA_DWC_DMACR_TXRXCH_CLEAR);
    }
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_dma_xfer_complete(ap: *mut ata_port) {
    static void sata_dwc_dma_xfer_complete(struct ata_port *ap)
    {
    struct ata_queued_cmd *qc;
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    struct sata_dwc_device *hsdev = HSDEV_FROM_AP(ap);
    let mut tag: u8 = 0;
    tag = ap.link.active_tag;
    qc = ata_qc_from_tag(ap, tag);
    if (!qc) {
    dev_err(ap.dev, "failed to get qc");
    return;
    }
    if (ata_is_dma(qc.tf.protocol)) {
    if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_NONE) {
    dev_err(ap.dev,
    "%s DMA protocol RX and TX DMA not pending dmacr: 0x%08x\n",
    __func__,
    sata_dwc_readl(&hsdev.sata_dwc_regs.dmacr));
    }
    hsdevp.dma_pending[tag] = SATA_DWC_DMA_PENDING_NONE;
    sata_dwc_qc_complete(ap, qc);
    ap.link.active_tag = ATA_TAG_POISON;
    } else {
    sata_dwc_qc_complete(ap, qc);
    }
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_qc_complete(ap: *mut ata_port, qc: *mut ata_queued_cmd) -> c_int {
    static int sata_dwc_qc_complete(struct ata_port *ap, struct ata_queued_cmd *qc)
    {
    let mut status: u8 = 0;
    let mut mask: u32 = 0x0;
    let mut tag: u8 = qc.hw_tag;
    struct sata_dwc_device *hsdev = HSDEV_FROM_AP(ap);
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    hsdev.sactive_queued = 0;
    if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_TX)
    dev_err(ap.dev, "TX DMA PENDING\n");
#[no_mangle]
pub unsafe extern "C" fn if(SATA_DWC_DMA_PENDING_RX: hsdevp->dma_pending[tag] ==) -> else {
    else if (hsdevp.dma_pending[tag] == SATA_DWC_DMA_PENDING_RX)
    dev_err(ap.dev, "RX DMA PENDING\n");
    dev_dbg(ap.dev,
    "QC complete cmd=0x%02x status=0x%02x ata%u: protocol=%d\n",
    qc.tf.command, status, ap.print_id, qc.tf.protocol);
// clear active bit
    mask = (~(qcmd_tag_to_mask(tag)));
    hsdev.sactive_queued = hsdev.sactive_queued & mask;
    hsdev.sactive_issued = hsdev.sactive_issued & mask;
    ata_qc_complete(qc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_enable_interrupts(hsdev: *mut sata_dwc_device) {
    static void sata_dwc_enable_interrupts(struct sata_dwc_device *hsdev)
    {
// Enable selective interrupts by setting the interrupt maskregister
    sata_dwc_writel(&hsdev.sata_dwc_regs.intmr,
    SATA_DWC_INTMR_ERRM |
    SATA_DWC_INTMR_NEWFPM |
    SATA_DWC_INTMR_PMABRTM |
    SATA_DWC_INTMR_DMATM);
//
// Unmask the error bits that should trigger an error interrupt by
// setting the error mask register.
//
    sata_dwc_writel(&hsdev.sata_dwc_regs.errmr, SATA_DWC_SERROR_ERR_BITS);
    dev_dbg(hsdev.dev, "%s: INTMR = 0x%08x, ERRMR = 0x%08x\n",
    __func__, sata_dwc_readl(&hsdev.sata_dwc_regs.intmr),
    sata_dwc_readl(&hsdev.sata_dwc_regs.errmr));
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_setup_port(port: *mut ata_ioports, base: *mut void __iomem) {
    static void sata_dwc_setup_port(struct ata_ioports *port, void __iomem *base)
    {
    port.cmd_addr		= base + 0x00;
    port.data_addr		= base + 0x00;
    port.error_addr	= base + 0x04;
    port.feature_addr	= base + 0x04;
    port.nsect_addr	= base + 0x08;
    port.lbal_addr		= base + 0x0c;
    port.lbam_addr		= base + 0x10;
    port.lbah_addr		= base + 0x14;
    port.device_addr	= base + 0x18;
    port.command_addr	= base + 0x1c;
    port.status_addr	= base + 0x1c;
    port.altstatus_addr	= base + 0x20;
    port.ctl_addr		= base + 0x20;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_dma_get_channel(hsdevp: *mut sata_dwc_device_port) -> c_int {
    static int sata_dwc_dma_get_channel(struct sata_dwc_device_port *hsdevp)
    {
    struct sata_dwc_device *hsdev = hsdevp.hsdev;
    struct device *dev = hsdev.dev;

    if (!of_property_present(dev.of_node, "dmas"))
    return sata_dwc_dma_get_channel_old(hsdevp);

    hsdevp.chan = dma_request_chan(dev, "sata-dma");
    if (IS_ERR(hsdevp.chan)) {
    dev_err(dev, "failed to allocate dma channel: %ld\n",
    PTR_ERR(hsdevp.chan));
    return PTR_ERR(hsdevp.chan);
    }
    return 0;
    }
//
// Function : sata_dwc_port_start
// arguments : struct ata_ioports *port
// Return value : returns 0 if success, error code otherwise
// This function allocates the scatter gather LLI table for AHB DMA
//
#[no_mangle]
unsafe extern "C" fn sata_dwc_port_start(ap: *mut ata_port) -> c_int {
    static int sata_dwc_port_start(struct ata_port *ap)
    {
    let mut err: c_int = 0;
    struct sata_dwc_device *hsdev;
    struct sata_dwc_device_port *hsdevp = core::ptr::null_mut();
    struct device *pdev;
    int i;
    hsdev = HSDEV_FROM_AP(ap);
    dev_dbg(ap.dev, "%s: port_no=%d\n", __func__, ap.port_no);
    hsdev.host = ap.host;
    pdev = ap.host.dev;
    if (!pdev) {
    dev_err(ap.dev, "%s: no ap.host.dev\n", __func__);
    err = -ENODEV;
    goto CLEANUP;
    }
// Allocate Port Struct
    hsdevp = kzalloc_obj(*hsdevp);
    if (!hsdevp) {
    err = -ENOMEM;
    goto CLEANUP;
    }
    hsdevp.hsdev = hsdev;
    err = sata_dwc_dma_get_channel(hsdevp);
    if (err)
    goto CLEANUP_ALLOC;
    err = phy_power_on(hsdev.phy);
    if (err)
    goto CLEANUP_ALLOC;
    for (i = 0; i < SATA_DWC_QCMD_MAX; i++)
    hsdevp.cmd_issued[i] = SATA_DWC_CMD_ISSUED_NOT;
    ap.bmdma_prd = core::ptr::null_mut();	/* set these so libata doesn't use them */
    ap.bmdma_prd_dma = 0;
    if (ap.port_no == 0)  {
    dev_dbg(ap.dev, "%s: clearing TXCHEN, RXCHEN in DMAC\n",
    __func__);
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr,
    SATA_DWC_DMACR_TXRXCH_CLEAR);
    dev_dbg(ap.dev, "%s: setting burst size in DBTSR\n",
    __func__);
    sata_dwc_writel(&hsdev.sata_dwc_regs.dbtsr,
    (SATA_DWC_DBTSR_MWR(AHB_DMA_BRST_DFLT) |
    SATA_DWC_DBTSR_MRD(AHB_DMA_BRST_DFLT)));
    }
// Clear any error bits before libata starts issuing commands
    clear_serror(ap);
    ap.private_data = hsdevp;
    dev_dbg(ap.dev, "%s: done\n", __func__);
    return 0;
    CLEANUP_ALLOC:
    kfree(hsdevp);
    CLEANUP:
    dev_dbg(ap.dev, "%s: fail. ap.id = %d\n", __func__, ap.print_id);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_port_stop(ap: *mut ata_port) {
    static void sata_dwc_port_stop(struct ata_port *ap)
    {
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    struct sata_dwc_device *hsdev = HSDEV_FROM_AP(ap);
    dev_dbg(ap.dev, "%s: ap.id = %d\n", __func__, ap.print_id);
    dmaengine_terminate_sync(hsdevp.chan);
    dma_release_channel(hsdevp.chan);
    phy_power_off(hsdev.phy);
    kfree(hsdevp);
    ap.private_data = core::ptr::null_mut();
    }
//
// Function : sata_dwc_exec_command_by_tag
// arguments : ata_port *ap, ata_taskfile *tf, u8 tag, u32 cmd_issued
// Return value : None
// This function keeps track of individual command tag ids and calls
// ata_exec_command in libata
//
    static void sata_dwc_exec_command_by_tag(struct ata_port *ap,
    struct ata_taskfile *tf,
    u8 tag, u32 cmd_issued)
    {
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    hsdevp.cmd_issued[tag] = cmd_issued;
//
// Clear SError before executing a new command.
// sata_dwc_scr_write and read can not be used here. Clearing the PM
// managed SError register for the disk needs to be done before the
// task file is loaded.
//
    clear_serror(ap);
    ata_sff_exec_command(ap, tf);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_bmdma_setup_by_tag(qc: *mut ata_queued_cmd, tag: u8) {
    static void sata_dwc_bmdma_setup_by_tag(struct ata_queued_cmd *qc, u8 tag)
    {
    sata_dwc_exec_command_by_tag(qc.ap, &qc.tf, tag,
    SATA_DWC_CMD_ISSUED_PEND);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_bmdma_setup(qc: *mut ata_queued_cmd) {
    static void sata_dwc_bmdma_setup(struct ata_queued_cmd *qc)
    {
    let mut tag: u8 = qc.hw_tag;
    if (!ata_is_ncq(qc.tf.protocol))
    tag = 0;
    sata_dwc_bmdma_setup_by_tag(qc, tag);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_bmdma_start_by_tag(qc: *mut ata_queued_cmd, tag: u8) {
    static void sata_dwc_bmdma_start_by_tag(struct ata_queued_cmd *qc, u8 tag)
    {
    int start_dma;
    u32 reg;
    struct sata_dwc_device *hsdev = HSDEV_FROM_QC(qc);
    struct ata_port *ap = qc.ap;
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    struct dma_async_tx_descriptor *desc = hsdevp.desc[tag];
    let mut dir: c_int = qc.dma_dir;
    if (hsdevp.cmd_issued[tag] != SATA_DWC_CMD_ISSUED_NOT) {
    start_dma = 1;
    if (dir == DMA_TO_DEVICE)
    hsdevp.dma_pending[tag] = SATA_DWC_DMA_PENDING_TX;
    else
    hsdevp.dma_pending[tag] = SATA_DWC_DMA_PENDING_RX;
    } else {
    dev_err(ap.dev,
    "%s: Command not pending cmd_issued=%d (tag=%d) DMA NOT started\n",
    __func__, hsdevp.cmd_issued[tag], tag);
    start_dma = 0;
    }
    if (start_dma) {
    sata_dwc_scr_read(&ap.link, SCR_ERROR, &reg);
    if (reg & SATA_DWC_SERROR_ERR_BITS) {
    dev_err(ap.dev, "%s: ****** SError=0x%08x ******\n",
    __func__, reg);
    }
    if (dir == DMA_TO_DEVICE)
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr,
    SATA_DWC_DMACR_TXCHEN);
    else
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr,
    SATA_DWC_DMACR_RXCHEN);
// Enable AHB DMA transfer on the specified channel
    dmaengine_submit(desc);
    dma_async_issue_pending(hsdevp.chan);
    }
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_bmdma_start(qc: *mut ata_queued_cmd) {
    static void sata_dwc_bmdma_start(struct ata_queued_cmd *qc)
    {
    let mut tag: u8 = qc.hw_tag;
    if (!ata_is_ncq(qc.tf.protocol))
    tag = 0;
    sata_dwc_bmdma_start_by_tag(qc, tag);
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    static unsigned int sata_dwc_qc_issue(struct ata_queued_cmd *qc)
    {
    u32 sactive;
    let mut tag: u8 = qc.hw_tag;
    struct ata_port *ap = qc.ap;
    struct sata_dwc_device_port *hsdevp = HSDEVP_FROM_AP(ap);
    if (!ata_is_ncq(qc.tf.protocol))
    tag = 0;
    if (ata_is_dma(qc.tf.protocol)) {
    hsdevp.desc[tag] = dma_dwc_xfer_setup(qc);
    if (!hsdevp.desc[tag])
    return AC_ERR_SYSTEM;
    } else {
    hsdevp.desc[tag] = core::ptr::null_mut();
    }
    if (ata_is_ncq(qc.tf.protocol)) {
    sata_dwc_scr_read(&ap.link, SCR_ACTIVE, &sactive);
    sactive |= (0x00000001 << tag);
    sata_dwc_scr_write(&ap.link, SCR_ACTIVE, sactive);
    trace_ata_tf_load(ap, &qc.tf);
    ap.ops.sff_tf_load(ap, &qc.tf);
    trace_ata_exec_command(ap, &qc.tf, tag);
    sata_dwc_exec_command_by_tag(ap, &qc.tf, tag,
    SATA_DWC_CMD_ISSUED_PEND);
    } else {
    return ata_bmdma_qc_issue(qc);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_error_handler(ap: *mut ata_port) {
    static void sata_dwc_error_handler(struct ata_port *ap)
    __must_hold(&ap.host.eh_mutex)
    {
    ata_sff_error_handler(ap);
    }
    static int sata_dwc_hardreset(struct ata_link *link, unsigned int *class,
    unsigned long deadline)
    {
    struct sata_dwc_device *hsdev = HSDEV_FROM_AP(link.ap);
    int ret;
    ret = sata_sff_hardreset(link, class, deadline);
    sata_dwc_enable_interrupts(hsdev);
// Reconfigure the DMA control register
    sata_dwc_writel(&hsdev.sata_dwc_regs.dmacr,
    SATA_DWC_DMACR_TXRXCH_CLEAR);
// Reconfigure the DMA Burst Transaction Size register
    sata_dwc_writel(&hsdev.sata_dwc_regs.dbtsr,
    SATA_DWC_DBTSR_MWR(AHB_DMA_BRST_DFLT) |
    SATA_DWC_DBTSR_MRD(AHB_DMA_BRST_DFLT));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_dev_select(ap: *mut ata_port, device: c_uint) {
    static void sata_dwc_dev_select(struct ata_port *ap, unsigned int device)
    {
// SATA DWC is master only
    }
//
// scsi mid-layer and libata interface structures
//
    static const struct scsi_host_template sata_dwc_sht = {
    ATA_NCQ_SHT(DRV_NAME),
//
// test-only: Currently this driver doesn't handle NCQ
// correctly. We enable NCQ but set the queue depth to a
// max of 1. This will get fixed in in a future release.
//
    .sg_tablesize		= LIBATA_MAX_PRD,
// .can_queue		= ATA_MAX_QUEUE,
//
// Make sure a LLI block is not created that will span 8K max FIS
// boundary. If the block spans such a FIS boundary, there is a chance
// that a DMA burst will cross that boundary -- this results in an
// error in the host controller.
//
    .dma_boundary		= 0x1fff /* ATA_DMA_BOUNDARY */,
    };
    static struct ata_port_operations sata_dwc_ops = {
    .inherits		= &ata_sff_port_ops,
    .error_handler		= sata_dwc_error_handler,
    .reset.hardreset	= sata_dwc_hardreset,
    .qc_issue		= sata_dwc_qc_issue,
    .scr_read		= sata_dwc_scr_read,
    .scr_write		= sata_dwc_scr_write,
    .port_start		= sata_dwc_port_start,
    .port_stop		= sata_dwc_port_stop,
    .sff_dev_select		= sata_dwc_dev_select,
    .bmdma_setup		= sata_dwc_bmdma_setup,
    .bmdma_start		= sata_dwc_bmdma_start,
    };
    static const struct ata_port_info sata_dwc_port_info[] = {
    {
    .flags		= ATA_FLAG_SATA | ATA_FLAG_NCQ,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &sata_dwc_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn sata_dwc_probe(ofdev: *mut platform_device) -> c_int {
    static int sata_dwc_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct sata_dwc_device *hsdev;
    u32 idr, versionr;
    char *ver = (char *)&versionr;
    void __iomem *base;
    let mut err: c_int = 0;
    int irq;
    struct ata_host *host;
    let mut pi: ata_port_info = sata_dwc_port_info[0];
    const struct ata_port_info *ppi[] = { &pi, core::ptr::null_mut() };
    struct resource *res;
// Allocate DWC SATA device
    host = ata_host_alloc_pinfo(dev, ppi, SATA_DWC_MAX_PORTS);
    hsdev = devm_kzalloc(dev, sizeof(*hsdev), GFP_KERNEL);
    if (!host || !hsdev)
    return -ENOMEM;
    host.private_data = hsdev;
// Ioremap SATA registers
    base = devm_platform_get_and_ioremap_resource(ofdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    dev_dbg(dev, "ioremap done for SATA register address\n");
// Synopsys DWC SATA specific Registers
    hsdev.sata_dwc_regs = base + SATA_DWC_REG_OFFSET;
    hsdev.dmadr = res.start + SATA_DWC_REG_OFFSET + offsetof(struct sata_dwc_regs, dmadr);
// Setup port
    host.ports[0].ioaddr.cmd_addr = base;
    host.ports[0].ioaddr.scr_addr = base + SATA_DWC_SCR_OFFSET;
    sata_dwc_setup_port(&host.ports[0].ioaddr, base);
// Read the ID and Version Registers
    idr = sata_dwc_readl(&hsdev.sata_dwc_regs.idr);
    versionr = sata_dwc_readl(&hsdev.sata_dwc_regs.versionr);
    dev_notice(dev, "id %d, controller version %c.%c%c\n", idr, ver[0], ver[1], ver[2]);
// Save dev for later use in dev_xxx() routines
    hsdev.dev = dev;
// Get SATA interrupt number
    irq = platform_get_irq(ofdev, 0);
    if (irq < 0)
    return irq;

    if (!of_property_present(dev.of_node, "dmas")) {
    err = sata_dwc_dma_init_old(ofdev, hsdev);
    if (err)
    return err;
    }

    hsdev.phy = devm_phy_optional_get(dev, "sata-phy");
    if (IS_ERR(hsdev.phy))
    return PTR_ERR(hsdev.phy);
    err = phy_init(hsdev.phy);
    if (err)
    goto error_out;
//
// Now, register with libATA core, this will also initiate the
// device discovery process, invoking our port_start() handler &
// error_handler() to execute a dummy Softreset EH session
//
    err = ata_host_activate(host, irq, sata_dwc_isr, 0, &sata_dwc_sht);
    if (err)
    dev_err(dev, "failed to activate host");
// Enable SATA Interrupts
    sata_dwc_enable_interrupts(hsdev);
    return 0;
    error_out:
    phy_exit(hsdev.phy);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sata_dwc_remove(ofdev: *mut platform_device) {
    static void sata_dwc_remove(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct ata_host *host = dev_get_drvdata(dev);
    struct sata_dwc_device *hsdev = host.private_data;
    ata_host_detach(host);
    phy_exit(hsdev.phy);

// Free SATA DMA resources
    sata_dwc_dma_exit_old(hsdev);

    dev_dbg(dev, "done\n");
    }
    static const struct of_device_id sata_dwc_match[] = {
    { .compatible = "amcc,sata-460ex", },
    {}
    };
    MODULE_DEVICE_TABLE(of, sata_dwc_match);
    static struct platform_driver sata_dwc_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = sata_dwc_match,
    },
    .probe = sata_dwc_probe,
    .remove = sata_dwc_remove,
    };
    module_platform_driver(sata_dwc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mark Miesfeld <mmiesfeld@amcc.com>");
    MODULE_DESCRIPTION("DesignWare Cores SATA controller low level driver");
    MODULE_VERSION(DRV_VERSION);
