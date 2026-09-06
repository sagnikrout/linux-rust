//! Automatically rewritten from C to Rust
//! Source: drivers/ata/sata_rcar.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Renesas R-Car SATA driver
//
// Author: Vladimir Barinov <source@cogentembedded.com>
// Copyright (C) 2013-2015 Cogent Embedded, Inc.
// Copyright (C) 2013-2015 Renesas Solutions Corp.
//

// SH-Navi2G/ATAPI module compatible control registers
pub const ATAPI_CONTROL1_REG: c_uint = 0x180;
pub const ATAPI_STATUS_REG: c_uint = 0x184;
pub const ATAPI_INT_ENABLE_REG: c_uint = 0x188;
pub const ATAPI_DTB_ADR_REG: c_uint = 0x198;
pub const ATAPI_DMA_START_ADR_REG: c_uint = 0x19C;
pub const ATAPI_DMA_TRANS_CNT_REG: c_uint = 0x1A0;
pub const ATAPI_CONTROL2_REG: c_uint = 0x1A4;
pub const ATAPI_SIG_ST_REG: c_uint = 0x1B0;
pub const ATAPI_BYTE_SWAP_REG: c_uint = 0x1BC;
// ATAPI control 1 register (ATAPI_CONTROL1) bits

// ATAPI status register (ATAPI_STATUS) bits

// Interrupt enable register (ATAPI_INT_ENABLE) bits

// Access control registers for physical layer control register
pub const SATAPHYADDR_REG: c_uint = 0x200;
pub const SATAPHYWDATA_REG: c_uint = 0x204;
pub const SATAPHYACCEN_REG: c_uint = 0x208;
pub const SATAPHYRESET_REG: c_uint = 0x20C;
pub const SATAPHYRDATA_REG: c_uint = 0x210;
pub const SATAPHYACK_REG: c_uint = 0x214;
// Physical layer control address command register (SATAPHYADDR) bits

// Physical layer control enable register (SATAPHYACCEN) bits

// Physical layer control reset register (SATAPHYRESET) bits

// Physical layer control acknowledge register (SATAPHYACK) bits

// Serial-ATA HOST control registers
pub const BISTCONF_REG: c_uint = 0x102C;
pub const SDATA_REG: c_uint = 0x1100;
pub const SSDEVCON_REG: c_uint = 0x1204;
pub const SCRSSTS_REG: c_uint = 0x1400;
pub const SCRSERR_REG: c_uint = 0x1404;
pub const SCRSCON_REG: c_uint = 0x1408;
pub const SCRSACT_REG: c_uint = 0x140C;
pub const SATAINTSTAT_REG: c_uint = 0x1508;
pub const SATAINTMASK_REG: c_uint = 0x150C;
// SATA INT status register (SATAINTSTAT) bits

// SATA INT mask register (SATAINTSTAT) bits

pub const SATAINTMASK_ALL_GEN1: c_uint = 0x7ff;
pub const SATAINTMASK_ALL_GEN2: c_uint = 0xfff;

    SATAINTMASK_ATAMSK)
// Physical Layer Control Registers
pub const SATAPCTLR1_REG: c_uint = 0x43;
pub const SATAPCTLR2_REG: c_uint = 0x52;
pub const SATAPCTLR3_REG: c_uint = 0x5A;
pub const SATAPCTLR4_REG: c_uint = 0x60;
// Descriptor table word 0 bit (when DTA32M = 1)

pub const SATA_RCAR_DMA_BOUNDARY: c_uint = 0x1FFFFFFFUL;
// Gen2 Physical Layer Control Registers
pub const RCAR_GEN2_PHY_CTL1_REG: c_uint = 0x1704;
pub const RCAR_GEN2_PHY_CTL1: c_uint = 0x34180002;
pub const RCAR_GEN2_PHY_CTL1_SS: c_uint = 0xC180	/* Spread Spectrum */;
pub const RCAR_GEN2_PHY_CTL2_REG: c_uint = 0x170C;
pub const RCAR_GEN2_PHY_CTL2: c_uint = 0x00002303;
pub const RCAR_GEN2_PHY_CTL3_REG: c_uint = 0x171C;
pub const RCAR_GEN2_PHY_CTL3: c_uint = 0x000B0194;
pub const RCAR_GEN2_PHY_CTL4_REG: c_uint = 0x1724;
pub const RCAR_GEN2_PHY_CTL4: c_uint = 0x00030994;
pub const RCAR_GEN2_PHY_CTL5_REG: c_uint = 0x1740;
pub const RCAR_GEN2_PHY_CTL5: c_uint = 0x03004001;

    enum sata_rcar_type {
    RCAR_GEN1_SATA,
    RCAR_GEN2_SATA,
    RCAR_GEN3_SATA,
    RCAR_R8A7790_ES1_SATA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_rcar_priv {
    pub base: *mut void __iomem,
    pub sataint_mask: u32,
    pub type: enum sata_rcar_type,
}

#[no_mangle]
unsafe extern "C" fn sata_rcar_gen1_phy_preinit(priv: *mut sata_rcar_priv) {
    static void sata_rcar_gen1_phy_preinit(struct sata_rcar_priv *priv)
    {
    void __iomem *base = priv.base;
// idle state
    iowrite32(0, base + SATAPHYADDR_REG);
// reset
    iowrite32(SATAPHYRESET_PHYRST, base + SATAPHYRESET_REG);
    udelay(10);
// deassert reset
    iowrite32(0, base + SATAPHYRESET_REG);
    }
    static void sata_rcar_gen1_phy_write(struct sata_rcar_priv *priv, u16 reg,
    u32 val, int group)
    {
    void __iomem *base = priv.base;
    int timeout;
// deassert reset
    iowrite32(0, base + SATAPHYRESET_REG);
// lane 1
    iowrite32(SATAPHYACCEN_PHYLANE, base + SATAPHYACCEN_REG);
// write phy register value
    iowrite32(val, base + SATAPHYWDATA_REG);
// set register group
    if (group)
    reg |= SATAPHYADDR_PHYRATEMODE;
// write command
    iowrite32(SATAPHYADDR_PHYCMD_WRITE | reg, base + SATAPHYADDR_REG);
// wait for ack
    for (timeout = 0; timeout < 100; timeout++) {
    val = ioread32(base + SATAPHYACK_REG);
    if (val & SATAPHYACK_PHYACK)
    break;
    }
    if (timeout >= 100)
    pr_err("%s timeout\n", __func__);
// idle state
    iowrite32(0, base + SATAPHYADDR_REG);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_gen1_phy_init(priv: *mut sata_rcar_priv) {
    static void sata_rcar_gen1_phy_init(struct sata_rcar_priv *priv)
    {
    sata_rcar_gen1_phy_preinit(priv);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR1_REG, 0x00200188, 0);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR1_REG, 0x00200188, 1);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR3_REG, 0x0000A061, 0);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR2_REG, 0x20000000, 0);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR2_REG, 0x20000000, 1);
    sata_rcar_gen1_phy_write(priv, SATAPCTLR4_REG, 0x28E80000, 0);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_gen2_phy_init(priv: *mut sata_rcar_priv) {
    static void sata_rcar_gen2_phy_init(struct sata_rcar_priv *priv)
    {
    void __iomem *base = priv.base;
    iowrite32(RCAR_GEN2_PHY_CTL1, base + RCAR_GEN2_PHY_CTL1_REG);
    iowrite32(RCAR_GEN2_PHY_CTL2, base + RCAR_GEN2_PHY_CTL2_REG);
    iowrite32(RCAR_GEN2_PHY_CTL3, base + RCAR_GEN2_PHY_CTL3_REG);
    iowrite32(RCAR_GEN2_PHY_CTL4, base + RCAR_GEN2_PHY_CTL4_REG);
    iowrite32(RCAR_GEN2_PHY_CTL5 | RCAR_GEN2_PHY_CTL5_DC |
    RCAR_GEN2_PHY_CTL5_TR, base + RCAR_GEN2_PHY_CTL5_REG);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_freeze(ap: *mut ata_port) {
    static void sata_rcar_freeze(struct ata_port *ap)
    {
    struct sata_rcar_priv *priv = ap.host.private_data;
// mask
    iowrite32(priv.sataint_mask, priv.base + SATAINTMASK_REG);
    ata_sff_freeze(ap);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_thaw(ap: *mut ata_port) {
    static void sata_rcar_thaw(struct ata_port *ap)
    {
    struct sata_rcar_priv *priv = ap.host.private_data;
    void __iomem *base = priv.base;
// ack
    iowrite32(~(u32)SATA_RCAR_INT_MASK, base + SATAINTSTAT_REG);
    ata_sff_thaw(ap);
// unmask
    iowrite32(priv.sataint_mask & ~SATA_RCAR_INT_MASK, base + SATAINTMASK_REG);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_ioread16_rep(reg: *mut void __iomem, buffer: *mut c_void, count: c_int) {
    static void sata_rcar_ioread16_rep(void __iomem *reg, void *buffer, int count)
    {
    u16 *ptr = buffer;
    while (count--) {
    let mut data: u16 = ioread32(reg);
// ptr++ = data;
    }
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_iowrite16_rep(reg: *mut void __iomem, buffer: *mut c_void, count: c_int) {
    static void sata_rcar_iowrite16_rep(void __iomem *reg, void *buffer, int count)
    {
    const u16 *ptr = buffer;
    while (count--)
    iowrite32(*ptr++, reg);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_check_status(ap: *mut ata_port) -> u8 {
    static u8 sata_rcar_check_status(struct ata_port *ap)
    {
    return ioread32(ap.ioaddr.status_addr);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_check_altstatus(ap: *mut ata_port) -> u8 {
    static u8 sata_rcar_check_altstatus(struct ata_port *ap)
    {
    return ioread32(ap.ioaddr.altstatus_addr);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_set_devctl(ap: *mut ata_port, ctl: u8) {
    static void sata_rcar_set_devctl(struct ata_port *ap, u8 ctl)
    {
    iowrite32(ctl, ap.ioaddr.ctl_addr);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_dev_select(ap: *mut ata_port, device: c_uint) {
    static void sata_rcar_dev_select(struct ata_port *ap, unsigned int device)
    {
    iowrite32(ATA_DEVICE_OBS, ap.ioaddr.device_addr);
    ata_sff_pause(ap);	/* needed; also flushes, for mmio */
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_ata_devchk(ap: *mut ata_port, device: c_uint) -> bool {
    static bool sata_rcar_ata_devchk(struct ata_port *ap, unsigned int device)
    {
    struct ata_ioports *ioaddr = &ap.ioaddr;
    u8 nsect, lbal;
    sata_rcar_dev_select(ap, device);
    iowrite32(0x55, ioaddr.nsect_addr);
    iowrite32(0xaa, ioaddr.lbal_addr);
    iowrite32(0xaa, ioaddr.nsect_addr);
    iowrite32(0x55, ioaddr.lbal_addr);
    iowrite32(0x55, ioaddr.nsect_addr);
    iowrite32(0xaa, ioaddr.lbal_addr);
    nsect = ioread32(ioaddr.nsect_addr);
    lbal  = ioread32(ioaddr.lbal_addr);
    if (nsect == 0x55 && lbal == 0xaa)
    return true;	/* found a device */
    return false;		/* nothing found */
    }
    static int sata_rcar_wait_after_reset(struct ata_link *link,
    unsigned long deadline)
    {
    struct ata_port *ap = link.ap;
    ata_msleep(ap, ATA_WAIT_AFTER_RESET);
    return ata_sff_wait_ready(link, deadline);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bus_softreset(ap: *mut ata_port, deadline: c_ulong) -> c_int {
    static int sata_rcar_bus_softreset(struct ata_port *ap, unsigned long deadline)
    {
    struct ata_ioports *ioaddr = &ap.ioaddr;
// software reset.  causes dev0 to be selected
    iowrite32(ap.ctl, ioaddr.ctl_addr);
    udelay(20);
    iowrite32(ap.ctl | ATA_SRST, ioaddr.ctl_addr);
    udelay(20);
    iowrite32(ap.ctl, ioaddr.ctl_addr);
    ap.last_ctl = ap.ctl;
// wait the port to become ready
    return sata_rcar_wait_after_reset(&ap.link, deadline);
    }
    static int sata_rcar_softreset(struct ata_link *link, unsigned int *classes,
    unsigned long deadline)
    {
    struct ata_port *ap = link.ap;
    let mut devmask: c_uint = 0;
    int rc;
    u8 err;
// determine if device 0 is present
    if (sata_rcar_ata_devchk(ap, 0))
    devmask |= 1 << 0;
// issue bus reset
    rc = sata_rcar_bus_softreset(ap, deadline);
// if link is occupied, -ENODEV too is an error
    if (rc && (rc != -ENODEV || sata_scr_valid(link))) {
    ata_link_err(link, "SRST failed (errno=%d)\n", rc);
    return rc;
    }
// determine by signature whether we have ATA or ATAPI devices
    classes[0] = ata_sff_dev_classify(&link.device[0], devmask, &err);
    return 0;
    }
    static void sata_rcar_tf_load(struct ata_port *ap,
    const struct ata_taskfile *tf)
    {
    struct ata_ioports *ioaddr = &ap.ioaddr;
    let mut is_addr: c_uint = tf.flags & ATA_TFLAG_ISADDR;
    if (tf.ctl != ap.last_ctl) {
    iowrite32(tf.ctl, ioaddr.ctl_addr);
    ap.last_ctl = tf.ctl;
    ata_wait_idle(ap);
    }
    if (is_addr && (tf.flags & ATA_TFLAG_LBA48)) {
    iowrite32(tf.hob_feature, ioaddr.feature_addr);
    iowrite32(tf.hob_nsect, ioaddr.nsect_addr);
    iowrite32(tf.hob_lbal, ioaddr.lbal_addr);
    iowrite32(tf.hob_lbam, ioaddr.lbam_addr);
    iowrite32(tf.hob_lbah, ioaddr.lbah_addr);
    }
    if (is_addr) {
    iowrite32(tf.feature, ioaddr.feature_addr);
    iowrite32(tf.nsect, ioaddr.nsect_addr);
    iowrite32(tf.lbal, ioaddr.lbal_addr);
    iowrite32(tf.lbam, ioaddr.lbam_addr);
    iowrite32(tf.lbah, ioaddr.lbah_addr);
    }
    if (tf.flags & ATA_TFLAG_DEVICE)
    iowrite32(tf.device, ioaddr.device_addr);
    ata_wait_idle(ap);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile) {
    static void sata_rcar_tf_read(struct ata_port *ap, struct ata_taskfile *tf)
    {
    struct ata_ioports *ioaddr = &ap.ioaddr;
    tf.status = sata_rcar_check_status(ap);
    tf.error = ioread32(ioaddr.error_addr);
    tf.nsect = ioread32(ioaddr.nsect_addr);
    tf.lbal = ioread32(ioaddr.lbal_addr);
    tf.lbam = ioread32(ioaddr.lbam_addr);
    tf.lbah = ioread32(ioaddr.lbah_addr);
    tf.device = ioread32(ioaddr.device_addr);
    if (tf.flags & ATA_TFLAG_LBA48) {
    iowrite32(tf.ctl | ATA_HOB, ioaddr.ctl_addr);
    tf.hob_feature = ioread32(ioaddr.error_addr);
    tf.hob_nsect = ioread32(ioaddr.nsect_addr);
    tf.hob_lbal = ioread32(ioaddr.lbal_addr);
    tf.hob_lbam = ioread32(ioaddr.lbam_addr);
    tf.hob_lbah = ioread32(ioaddr.lbah_addr);
    iowrite32(tf.ctl, ioaddr.ctl_addr);
    ap.last_ctl = tf.ctl;
    }
    }
    static void sata_rcar_exec_command(struct ata_port *ap,
    const struct ata_taskfile *tf)
    {
    iowrite32(tf.command, ap.ioaddr.command_addr);
    ata_sff_pause(ap);
    }
    static unsigned int sata_rcar_data_xfer(struct ata_queued_cmd *qc,
    unsigned char *buf,
    unsigned int buflen, int rw)
    {
    struct ata_port *ap = qc.dev.link.ap;
    void __iomem *data_addr = ap.ioaddr.data_addr;
    let mut words: c_uint = buflen >> 1;
// Transfer multiple of 2 bytes
    if (rw == READ)
    sata_rcar_ioread16_rep(data_addr, buf, words);
    else
    sata_rcar_iowrite16_rep(data_addr, buf, words);
// Transfer trailing byte, if any.
    if (unlikely(buflen & 0x01)) {
    unsigned char pad[2] = { };
// Point buf to the tail of buffer
    buf += buflen - 1;
//
// Use io*16_rep() accessors here as well to avoid pointlessly
// swapping bytes to and from on the big endian machines...
//
    if (rw == READ) {
    sata_rcar_ioread16_rep(data_addr, pad, 1);
// buf = pad[0];
    } else {
    pad[0] = *buf;
    sata_rcar_iowrite16_rep(data_addr, pad, 1);
    }
    words++;
    }
    return words << 1;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_drain_fifo(qc: *mut ata_queued_cmd) {
    static void sata_rcar_drain_fifo(struct ata_queued_cmd *qc)
    {
    int count;
    struct ata_port *ap;
// We only need to flush incoming data when a command was running
    if (qc == core::ptr::null_mut() || qc.dma_dir == DMA_TO_DEVICE)
    return;
    ap = qc.ap;
// Drain up to 64K of data before we give up this recovery method
    for (count = 0; (ap.ops.sff_check_status(ap) & ATA_DRQ) &&
    count < 65536; count += 2)
    ioread32(ap.ioaddr.data_addr);
    if (count)
    ata_port_dbg(ap, "drained %d bytes to clear DRQ\n", count);
    }
    static int sata_rcar_scr_read(struct ata_link *link, unsigned int sc_reg,
    u32 *val)
    {
    if (sc_reg > SCR_ACTIVE)
    return -EINVAL;
// val = ioread32(link->ap->ioaddr.scr_addr + (sc_reg << 2));
    return 0;
    }
    static int sata_rcar_scr_write(struct ata_link *link, unsigned int sc_reg,
    u32 val)
    {
    if (sc_reg > SCR_ACTIVE)
    return -EINVAL;
    iowrite32(val, link.ap.ioaddr.scr_addr + (sc_reg << 2));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bmdma_fill_sg(qc: *mut ata_queued_cmd) {
    static void sata_rcar_bmdma_fill_sg(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct ata_bmdma_prd *prd = ap.bmdma_prd;
    struct scatterlist *sg;
    unsigned int si;
    for_each_sg(qc.sg, sg, qc.n_elem, si) {
    u32 addr, sg_len;
//
// Note: h/w doesn't support 64-bit, so we unconditionally
// truncate dma_addr_t to u32.
//
    addr = (u32)sg_dma_address(sg);
    sg_len = sg_dma_len(sg);
    prd[si].addr = cpu_to_le32(addr);
    prd[si].flags_len = cpu_to_le32(sg_len);
    }
// end-of-table flag
    prd[si - 1].addr |= cpu_to_le32(SATA_RCAR_DTEND);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_qc_prep(qc: *mut ata_queued_cmd) -> enum ata_completion_errors {
    static enum ata_completion_errors sata_rcar_qc_prep(struct ata_queued_cmd *qc)
    {
    if (!(qc.flags & ATA_QCFLAG_DMAMAP))
    return AC_ERR_OK;
    sata_rcar_bmdma_fill_sg(qc);
    return AC_ERR_OK;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bmdma_setup(qc: *mut ata_queued_cmd) {
    static void sata_rcar_bmdma_setup(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    let mut rw: c_uint = qc.tf.flags & ATA_TFLAG_WRITE;
    struct sata_rcar_priv *priv = ap.host.private_data;
    void __iomem *base = priv.base;
    u32 dmactl;
// load PRD table addr.
    mb();   /* make sure PRD table writes are visible to controller */
    iowrite32(ap.bmdma_prd_dma, base + ATAPI_DTB_ADR_REG);
// specify data direction, triple-check start bit is clear
    dmactl = ioread32(base + ATAPI_CONTROL1_REG);
    dmactl &= ~(ATAPI_CONTROL1_RW | ATAPI_CONTROL1_STOP);
    if (dmactl & ATAPI_CONTROL1_START) {
    dmactl &= ~ATAPI_CONTROL1_START;
    dmactl |= ATAPI_CONTROL1_STOP;
    }
    if (!rw)
    dmactl |= ATAPI_CONTROL1_RW;
    iowrite32(dmactl, base + ATAPI_CONTROL1_REG);
// issue r/w command
    ap.ops.sff_exec_command(ap, &qc.tf);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bmdma_start(qc: *mut ata_queued_cmd) {
    static void sata_rcar_bmdma_start(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct sata_rcar_priv *priv = ap.host.private_data;
    void __iomem *base = priv.base;
    u32 dmactl;
// start host DMA transaction
    dmactl = ioread32(base + ATAPI_CONTROL1_REG);
    dmactl &= ~ATAPI_CONTROL1_STOP;
    dmactl |= ATAPI_CONTROL1_START;
    iowrite32(dmactl, base + ATAPI_CONTROL1_REG);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bmdma_stop(qc: *mut ata_queued_cmd) {
    static void sata_rcar_bmdma_stop(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct sata_rcar_priv *priv = ap.host.private_data;
    void __iomem *base = priv.base;
    u32 dmactl;
// force termination of DMA transfer if active
    dmactl = ioread32(base + ATAPI_CONTROL1_REG);
    if (dmactl & ATAPI_CONTROL1_START) {
    dmactl &= ~ATAPI_CONTROL1_START;
    dmactl |= ATAPI_CONTROL1_STOP;
    iowrite32(dmactl, base + ATAPI_CONTROL1_REG);
    }
// one-PIO-cycle guaranteed wait, per spec, for HDMA1:0 transition
    ata_sff_dma_pause(ap);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_bmdma_status(ap: *mut ata_port) -> u8 {
    static u8 sata_rcar_bmdma_status(struct ata_port *ap)
    {
    struct sata_rcar_priv *priv = ap.host.private_data;
    let mut host_stat: u8 = 0;
    u32 status;
    status = ioread32(priv.base + ATAPI_STATUS_REG);
    if (status & ATAPI_STATUS_DEVINT)
    host_stat |= ATA_DMA_INTR;
    if (status & ATAPI_STATUS_ACT)
    host_stat |= ATA_DMA_ACTIVE;
    return host_stat;
    }
    static const struct scsi_host_template sata_rcar_sht = {
    ATA_BASE_SHT(DRV_NAME),
//
// This controller allows transfer chunks up to 512MB which cross 64KB
// boundaries, therefore the DMA limits are more relaxed than standard
// ATA SFF.
//
    .sg_tablesize		= ATA_MAX_PRD,
    .dma_boundary		= SATA_RCAR_DMA_BOUNDARY,
    };
    static struct ata_port_operations sata_rcar_port_ops = {
    .inherits		= &ata_bmdma_port_ops,
    .freeze			= sata_rcar_freeze,
    .thaw			= sata_rcar_thaw,
    .reset.softreset	= sata_rcar_softreset,
    .scr_read		= sata_rcar_scr_read,
    .scr_write		= sata_rcar_scr_write,
    .sff_dev_select		= sata_rcar_dev_select,
    .sff_set_devctl		= sata_rcar_set_devctl,
    .sff_check_status	= sata_rcar_check_status,
    .sff_check_altstatus	= sata_rcar_check_altstatus,
    .sff_tf_load		= sata_rcar_tf_load,
    .sff_tf_read		= sata_rcar_tf_read,
    .sff_exec_command	= sata_rcar_exec_command,
    .sff_data_xfer		= sata_rcar_data_xfer,
    .sff_drain_fifo		= sata_rcar_drain_fifo,
    .qc_prep		= sata_rcar_qc_prep,
    .bmdma_setup		= sata_rcar_bmdma_setup,
    .bmdma_start		= sata_rcar_bmdma_start,
    .bmdma_stop		= sata_rcar_bmdma_stop,
    .bmdma_status		= sata_rcar_bmdma_status,
    };
#[no_mangle]
unsafe extern "C" fn sata_rcar_serr_interrupt(ap: *mut ata_port) {
    static void sata_rcar_serr_interrupt(struct ata_port *ap)
    {
    struct sata_rcar_priv *priv = ap.host.private_data;
    struct ata_eh_info *ehi = &ap.link.eh_info;
    let mut freeze: c_int = 0;
    u32 serror;
    serror = ioread32(priv.base + SCRSERR_REG);
    if (!serror)
    return;
    ata_port_dbg(ap, "SError @host_intr: 0x%x\n", serror);
// first, analyze and record host port events
    ata_ehi_clear_desc(ehi);
    if (serror & (SERR_DEV_XCHG | SERR_PHYRDY_CHG)) {
// Setup a soft-reset EH action
    ata_ehi_hotplugged(ehi);
    ata_ehi_push_desc(ehi, "%s", "hotplug");
    freeze = serror & SERR_COMM_WAKE ? 0 : 1;
    }
// freeze or abort
    if (freeze)
    ata_port_freeze(ap);
    else
    ata_port_abort(ap);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_ata_interrupt(ap: *mut ata_port) {
    static void sata_rcar_ata_interrupt(struct ata_port *ap)
    {
    struct ata_queued_cmd *qc;
    let mut handled: c_int = 0;
    qc = ata_qc_from_tag(ap, ap.link.active_tag);
    if (qc)
    handled |= ata_bmdma_port_intr(ap, qc);
// be sure to clear ATA interrupt
    if (!handled)
    sata_rcar_check_status(ap);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t sata_rcar_interrupt(int irq, void *dev_instance)
    {
    struct ata_host *host = dev_instance;
    struct sata_rcar_priv *priv = host.private_data;
    void __iomem *base = priv.base;
    let mut handled: c_uint = 0;
    struct ata_port *ap;
    u32 sataintstat;
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    sataintstat = ioread32(base + SATAINTSTAT_REG);
    sataintstat &= SATA_RCAR_INT_MASK;
    if (!sataintstat)
    goto done;
// ack
    iowrite32(~sataintstat & priv.sataint_mask, base + SATAINTSTAT_REG);
    ap = host.ports[0];
    if (sataintstat & SATAINTSTAT_ATA)
    sata_rcar_ata_interrupt(ap);
    if (sataintstat & SATAINTSTAT_SERR)
    sata_rcar_serr_interrupt(ap);
    handled = 1;
    done:
    spin_unlock_irqrestore(&host.lock, flags);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_setup_port(host: *mut ata_host) {
    static void sata_rcar_setup_port(struct ata_host *host)
    {
    struct ata_port *ap = host.ports[0];
    struct ata_ioports *ioaddr = &ap.ioaddr;
    struct sata_rcar_priv *priv = host.private_data;
    void __iomem *base = priv.base;
    ap.ops		= &sata_rcar_port_ops;
    ap.pio_mask	= ATA_PIO4;
    ap.udma_mask	= ATA_UDMA6;
    ap.flags	|= ATA_FLAG_SATA;
    if (priv.type == RCAR_R8A7790_ES1_SATA)
    ap.flags	|= ATA_FLAG_NO_DIPM;
    ioaddr.cmd_addr = base + SDATA_REG;
    ioaddr.ctl_addr = base + SSDEVCON_REG;
    ioaddr.scr_addr = base + SCRSSTS_REG;
    ioaddr.altstatus_addr = ioaddr.ctl_addr;
    ioaddr.data_addr	= ioaddr.cmd_addr + (ATA_REG_DATA << 2);
    ioaddr.error_addr	= ioaddr.cmd_addr + (ATA_REG_ERR << 2);
    ioaddr.feature_addr	= ioaddr.cmd_addr + (ATA_REG_FEATURE << 2);
    ioaddr.nsect_addr	= ioaddr.cmd_addr + (ATA_REG_NSECT << 2);
    ioaddr.lbal_addr	= ioaddr.cmd_addr + (ATA_REG_LBAL << 2);
    ioaddr.lbam_addr	= ioaddr.cmd_addr + (ATA_REG_LBAM << 2);
    ioaddr.lbah_addr	= ioaddr.cmd_addr + (ATA_REG_LBAH << 2);
    ioaddr.device_addr	= ioaddr.cmd_addr + (ATA_REG_DEVICE << 2);
    ioaddr.status_addr	= ioaddr.cmd_addr + (ATA_REG_STATUS << 2);
    ioaddr.command_addr	= ioaddr.cmd_addr + (ATA_REG_CMD << 2);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_init_module(priv: *mut sata_rcar_priv) {
    static void sata_rcar_init_module(struct sata_rcar_priv *priv)
    {
    void __iomem *base = priv.base;
    u32 val;
// SATA-IP reset state
    val = ioread32(base + ATAPI_CONTROL1_REG);
    val |= ATAPI_CONTROL1_RESET;
    iowrite32(val, base + ATAPI_CONTROL1_REG);
// ISM mode, PRD mode, DTEND flag at bit 0
    val = ioread32(base + ATAPI_CONTROL1_REG);
    val |= ATAPI_CONTROL1_ISM;
    val |= ATAPI_CONTROL1_DESE;
    val |= ATAPI_CONTROL1_DTA32M;
    iowrite32(val, base + ATAPI_CONTROL1_REG);
// Release the SATA-IP from the reset state
    val = ioread32(base + ATAPI_CONTROL1_REG);
    val &= ~ATAPI_CONTROL1_RESET;
    iowrite32(val, base + ATAPI_CONTROL1_REG);
// ack and mask
    iowrite32(0, base + SATAINTSTAT_REG);
    iowrite32(priv.sataint_mask, base + SATAINTMASK_REG);
// enable interrupts
    iowrite32(ATAPI_INT_ENABLE_SATAINT, base + ATAPI_INT_ENABLE_REG);
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_init_controller(host: *mut ata_host) {
    static void sata_rcar_init_controller(struct ata_host *host)
    {
    struct sata_rcar_priv *priv = host.private_data;
    priv.sataint_mask = SATAINTMASK_ALL_GEN2;
// reset and setup phy
    switch (priv.type) {
    case RCAR_GEN1_SATA:
    priv.sataint_mask = SATAINTMASK_ALL_GEN1;
    sata_rcar_gen1_phy_init(priv);
    break;
    case RCAR_GEN2_SATA:
    case RCAR_R8A7790_ES1_SATA:
    sata_rcar_gen2_phy_init(priv);
    break;
    case RCAR_GEN3_SATA:
    break;
    default:
    dev_warn(host.dev, "SATA phy is not initialized\n");
    break;
    }
    sata_rcar_init_module(priv);
    }
    static const struct of_device_id sata_rcar_match[] = {
    {
// Deprecated by "renesas,sata-r8a7779"
    .compatible = "renesas,rcar-sata",
    .data = (void *)RCAR_GEN1_SATA,
    },
    {
    .compatible = "renesas,sata-r8a7779",
    .data = (void *)RCAR_GEN1_SATA,
    },
    {
    .compatible = "renesas,sata-r8a7790",
    .data = (void *)RCAR_GEN2_SATA
    },
    {
    .compatible = "renesas,sata-r8a7790-es1",
    .data = (void *)RCAR_R8A7790_ES1_SATA
    },
    {
    .compatible = "renesas,sata-r8a7791",
    .data = (void *)RCAR_GEN2_SATA
    },
    {
    .compatible = "renesas,sata-r8a7793",
    .data = (void *)RCAR_GEN2_SATA
    },
    {
    .compatible = "renesas,sata-r8a7795",
    .data = (void *)RCAR_GEN3_SATA
    },
    {
    .compatible = "renesas,rcar-gen2-sata",
    .data = (void *)RCAR_GEN2_SATA
    },
    {
    .compatible = "renesas,rcar-gen3-sata",
    .data = (void *)RCAR_GEN3_SATA
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sata_rcar_match);
#[no_mangle]
unsafe extern "C" fn sata_rcar_probe(pdev: *mut platform_device) -> c_int {
    static int sata_rcar_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ata_host *host;
    struct sata_rcar_priv *priv;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    priv = devm_kzalloc(dev, sizeof(struct sata_rcar_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.type = (unsigned long)of_device_get_match_data(dev);
    pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    goto err_pm_put;
    host = ata_host_alloc(dev, 1);
    if (!host) {
    ret = -ENOMEM;
    goto err_pm_put;
    }
    host.private_data = priv;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    ret = PTR_ERR(priv.base);
    goto err_pm_put;
    }
// setup port
    sata_rcar_setup_port(host);
// initialize host controller
    sata_rcar_init_controller(host);
    ret = ata_host_activate(host, irq, sata_rcar_interrupt, 0,
    &sata_rcar_sht);
    if (!ret)
    return 0;
    err_pm_put:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_remove(pdev: *mut platform_device) {
    static void sata_rcar_remove(struct platform_device *pdev)
    {
    struct ata_host *host = platform_get_drvdata(pdev);
    struct sata_rcar_priv *priv = host.private_data;
    void __iomem *base = priv.base;
    ata_host_detach(host);
// disable interrupts
    iowrite32(0, base + ATAPI_INT_ENABLE_REG);
// ack and mask
    iowrite32(0, base + SATAINTSTAT_REG);
    iowrite32(priv.sataint_mask, base + SATAINTMASK_REG);
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }

#[no_mangle]
unsafe extern "C" fn sata_rcar_suspend(dev: *mut device) -> c_int {
    static int sata_rcar_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct sata_rcar_priv *priv = host.private_data;
    void __iomem *base = priv.base;
    ata_host_suspend(host, PMSG_SUSPEND);
// disable interrupts
    iowrite32(0, base + ATAPI_INT_ENABLE_REG);
// mask
    iowrite32(priv.sataint_mask, base + SATAINTMASK_REG);
    pm_runtime_put(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_resume(dev: *mut device) -> c_int {
    static int sata_rcar_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct sata_rcar_priv *priv = host.private_data;
    void __iomem *base = priv.base;
    int ret;
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put(dev);
    return ret;
    }
    if (priv.type == RCAR_GEN3_SATA) {
    sata_rcar_init_module(priv);
    } else {
// ack and mask
    iowrite32(0, base + SATAINTSTAT_REG);
    iowrite32(priv.sataint_mask, base + SATAINTMASK_REG);
// enable interrupts
    iowrite32(ATAPI_INT_ENABLE_SATAINT,
    base + ATAPI_INT_ENABLE_REG);
    }
    ata_host_resume(host);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_rcar_restore(dev: *mut device) -> c_int {
    static int sata_rcar_restore(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    pm_runtime_put(dev);
    return ret;
    }
    sata_rcar_setup_port(host);
// initialize host controller
    sata_rcar_init_controller(host);
    ata_host_resume(host);
    return 0;
    }
    static const struct dev_pm_ops sata_rcar_pm_ops = {
    .suspend	= sata_rcar_suspend,
    .resume		= sata_rcar_resume,
    .freeze		= sata_rcar_suspend,
    .thaw		= sata_rcar_resume,
    .poweroff	= sata_rcar_suspend,
    .restore	= sata_rcar_restore,
    };

    static struct platform_driver sata_rcar_driver = {
    .probe		= sata_rcar_probe,
    .remove		= sata_rcar_remove,
    .driver = {
    .name		= DRV_NAME,
    .of_match_table	= sata_rcar_match,

    .pm		= &sata_rcar_pm_ops,

    },
    };
    module_platform_driver(sata_rcar_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Vladimir Barinov");
    MODULE_DESCRIPTION("Renesas R-Car SATA controller low level driver");
