//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_arasan_cf.c
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


//
// drivers/ata/pata_arasan_cf.c
//
// Arasan Compact Flash host controller source file
//
// Copyright (C) 2011 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//
// The Arasan CompactFlash Device Controller IP core has three basic modes of
// operation: PC card ATA using I/O mode, PC card ATA using memory mode, PC card
// ATA using true IDE modes. This driver supports only True IDE mode currently.
//
// Arasan CF Controller shares global irq register with Arasan XD Controller.
//
// Tested on arch/arm/mach-spear13xx
//

// Registers
// CompactFlash Interface Status
pub const CFI_STS: c_uint = 0x000;

// IRQ
pub const IRQ_STS: c_uint = 0x004;
// Interrupt Enable
pub const IRQ_EN: c_uint = 0x008;

    TRUE_IDE_MODE_IRQ)

    BUF_AVAIL_IRQ | XFER_DONE_IRQ)
// Operation Mode
pub const OP_MODE: c_uint = 0x00C;

// CF Interface Clock Configuration
pub const CLK_CFG: c_uint = 0x010;

// CF Timing Mode Configuration
pub const TM_CFG: c_uint = 0x014;

pub const TRUEIDE_PIO_TIMING_SHIFT: c_int = 4;

pub const TRUEIDE_MWORD_DMA_TIMING_SHIFT: c_int = 7;

pub const ULTRA_DMA_TIMING_SHIFT: c_int = 10;
// CF Transfer Address
pub const XFER_ADDR: c_uint = 0x014;

pub const MAX_XFER_COUNT: c_uint = 0x20000u;
// Transfer Control
pub const XFER_CTR: c_uint = 0x01C;

// Write Data Port
pub const WRITE_PORT: c_uint = 0x024;
// Read Data Port
pub const READ_PORT: c_uint = 0x028;
// ATA Data Port
pub const ATA_DATA_PORT: c_uint = 0x030;

// ATA Error/Features
pub const ATA_ERR_FTR: c_uint = 0x034;
// ATA Sector Count
pub const ATA_SC: c_uint = 0x038;
// ATA Sector Number
pub const ATA_SN: c_uint = 0x03C;
// ATA Cylinder Low
pub const ATA_CL: c_uint = 0x040;
// ATA Cylinder High
pub const ATA_CH: c_uint = 0x044;
// ATA Select Card/Head
pub const ATA_SH: c_uint = 0x048;
// ATA Status-Command
pub const ATA_STS_CMD: c_uint = 0x04C;
// ATA Alternate Status/Device Control
pub const ATA_ASTS_DCTR: c_uint = 0x050;
// Extended Write Data Port 0x200-0x3FC
pub const EXT_WRITE_PORT: c_uint = 0x200;
// Extended Read Data Port 0x400-0x5FC
pub const EXT_READ_PORT: c_uint = 0x400;
pub const FIFO_SIZE: c_uint = 0x200u;
// Global Interrupt Status
pub const GIRQ_STS: c_uint = 0x800;
// Global Interrupt Status enable
pub const GIRQ_STS_EN: c_uint = 0x804;
// Global Interrupt Signal enable
pub const GIRQ_SGN_EN: c_uint = 0x808;

// Compact Flash Controller Dev Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arasan_cf_dev {
// pointer to ata_host structure
    pub host: *mut ata_host,
// clk structure
    pub clk: *mut clk,
// physical base address of controller
    pub pbase: dma_addr_t,
// virtual base address of controller
    pub vbase: *mut void __iomem,
// irq number
    pub irq: c_int,
// status to be updated to framework regarding DMA transfer
    pub dma_status: u8,
// Card is present or Not
    pub card_present: u8,
// dma specific
// Completion for transfer complete interrupt from controller
    pub cf_completion: completion,
// Completion for DMA transfer complete.
    pub dma_completion: completion,
// Dma channel allocated
    pub dma_chan: *mut dma_chan,
// Mask for DMA transfers
    pub mask: dma_cap_mask_t,
// DMA transfer work
    pub work: work_struct,
// DMA delayed finish work
    pub dwork: delayed_work,
// qc to be transferred using DMA
    pub qc: *mut ata_queued_cmd,
}

    static const struct scsi_host_template arasan_cf_sht = {
    ATA_BASE_SHT(DRIVER_NAME),
    .dma_boundary = 0xFFFFFFFFUL,
    };
#[no_mangle]
unsafe extern "C" fn cf_dumpregs(acdev: *mut arasan_cf_dev) {
    static void cf_dumpregs(struct arasan_cf_dev *acdev)
    {
    struct device *dev = acdev.host.dev;
    dev_dbg(dev, ": =========== REGISTER DUMP ===========");
    dev_dbg(dev, ": CFI_STS: %x", readl(acdev.vbase + CFI_STS));
    dev_dbg(dev, ": IRQ_STS: %x", readl(acdev.vbase + IRQ_STS));
    dev_dbg(dev, ": IRQ_EN: %x", readl(acdev.vbase + IRQ_EN));
    dev_dbg(dev, ": OP_MODE: %x", readl(acdev.vbase + OP_MODE));
    dev_dbg(dev, ": CLK_CFG: %x", readl(acdev.vbase + CLK_CFG));
    dev_dbg(dev, ": TM_CFG: %x", readl(acdev.vbase + TM_CFG));
    dev_dbg(dev, ": XFER_CTR: %x", readl(acdev.vbase + XFER_CTR));
    dev_dbg(dev, ": GIRQ_STS: %x", readl(acdev.vbase + GIRQ_STS));
    dev_dbg(dev, ": GIRQ_STS_EN: %x", readl(acdev.vbase + GIRQ_STS_EN));
    dev_dbg(dev, ": GIRQ_SGN_EN: %x", readl(acdev.vbase + GIRQ_SGN_EN));
    dev_dbg(dev, ": =====================================");
    }
// Enable/Disable global interrupts shared between CF and XD ctrlr.
#[no_mangle]
unsafe extern "C" fn cf_ginterrupt_enable(acdev: *mut arasan_cf_dev, enable: bool) {
    static void cf_ginterrupt_enable(struct arasan_cf_dev *acdev, bool enable)
    {
// enable should be 0 or 1
    writel(enable, acdev.vbase + GIRQ_STS_EN);
    writel(enable, acdev.vbase + GIRQ_SGN_EN);
    }
// Enable/Disable CF interrupts
    static inline void
    cf_interrupt_enable(struct arasan_cf_dev *acdev, u32 mask, bool enable)
    {
    let mut val: u32 = readl(acdev.vbase + IRQ_EN);
// clear & enable/disable irqs
    if (enable) {
    writel(mask, acdev.vbase + IRQ_STS);
    writel(val | mask, acdev.vbase + IRQ_EN);
    } else
    writel(val & ~mask, acdev.vbase + IRQ_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn cf_card_reset(acdev: *mut arasan_cf_dev) {
    static inline void cf_card_reset(struct arasan_cf_dev *acdev)
    {
    let mut val: u32 = readl(acdev.vbase + OP_MODE);
    writel(val | CARD_RESET, acdev.vbase + OP_MODE);
    udelay(200);
    writel(val & ~CARD_RESET, acdev.vbase + OP_MODE);
    }
#[no_mangle]
pub unsafe extern "C" fn cf_ctrl_reset(acdev: *mut arasan_cf_dev) {
    static inline void cf_ctrl_reset(struct arasan_cf_dev *acdev)
    {
    writel(readl(acdev.vbase + OP_MODE) & ~CFHOST_ENB,
    acdev.vbase + OP_MODE);
    writel(readl(acdev.vbase + OP_MODE) | CFHOST_ENB,
    acdev.vbase + OP_MODE);
    }
#[no_mangle]
unsafe extern "C" fn cf_card_detect(acdev: *mut arasan_cf_dev, hotplugged: bool) {
    static void cf_card_detect(struct arasan_cf_dev *acdev, bool hotplugged)
    {
    struct ata_port *ap = acdev.host.ports[0];
    struct ata_eh_info *ehi = &ap.link.eh_info;
    let mut val: u32 = readl(acdev.vbase + CFI_STS);
// Both CD1 & CD2 should be low if card inserted completely
    if (!(val & (CARD_DETECT1 | CARD_DETECT2))) {
    if (acdev.card_present)
    return;
    acdev.card_present = 1;
    cf_card_reset(acdev);
    } else {
    if (!acdev.card_present)
    return;
    acdev.card_present = 0;
    }
    if (hotplugged) {
    ata_ehi_hotplugged(ehi);
    ata_port_freeze(ap);
    }
    }
#[no_mangle]
unsafe extern "C" fn cf_init(acdev: *mut arasan_cf_dev) -> c_int {
    static int cf_init(struct arasan_cf_dev *acdev)
    {
    struct arasan_cf_pdata *pdata = dev_get_platdata(acdev.host.dev);
    unsigned int if_clk;
    unsigned long flags;
    let mut ret: c_int = 0;
    ret = clk_prepare_enable(acdev.clk);
    if (ret) {
    dev_dbg(acdev.host.dev, "clock enable failed");
    return ret;
    }
    ret = clk_set_rate(acdev.clk, 166000000);
    if (ret) {
    dev_warn(acdev.host.dev, "clock set rate failed");
    clk_disable_unprepare(acdev.clk);
    return ret;
    }
    spin_lock_irqsave(&acdev.host.lock, flags);
// configure CF interface clock
// TODO: read from device tree
    if_clk = CF_IF_CLK_166M;
    if (pdata && pdata.cf_if_clk <= CF_IF_CLK_200M)
    if_clk = pdata.cf_if_clk;
    writel(if_clk, acdev.vbase + CLK_CFG);
    writel(TRUE_IDE_MODE | CFHOST_ENB, acdev.vbase + OP_MODE);
    cf_interrupt_enable(acdev, CARD_DETECT_IRQ, 1);
    cf_ginterrupt_enable(acdev, 1);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cf_exit(acdev: *mut arasan_cf_dev) {
    static void cf_exit(struct arasan_cf_dev *acdev)
    {
    unsigned long flags;
    spin_lock_irqsave(&acdev.host.lock, flags);
    cf_ginterrupt_enable(acdev, 0);
    cf_interrupt_enable(acdev, TRUE_IDE_IRQS, 0);
    cf_card_reset(acdev);
    writel(readl(acdev.vbase + OP_MODE) & ~CFHOST_ENB,
    acdev.vbase + OP_MODE);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    clk_disable_unprepare(acdev.clk);
    }
#[no_mangle]
unsafe extern "C" fn dma_callback(dev: *mut c_void) {
    static void dma_callback(void *dev)
    {
    struct arasan_cf_dev *acdev = dev;
    complete(&acdev.dma_completion);
    }
#[no_mangle]
pub unsafe extern "C" fn dma_complete(acdev: *mut arasan_cf_dev) {
    static inline void dma_complete(struct arasan_cf_dev *acdev)
    {
    struct ata_queued_cmd *qc = acdev.qc;
    unsigned long flags;
    acdev.qc = core::ptr::null_mut();
    ata_sff_interrupt(acdev.irq, acdev.host);
    spin_lock_irqsave(&acdev.host.lock, flags);
    if (unlikely(qc.err_mask) && ata_is_dma(qc.tf.protocol))
    ata_ehi_push_desc(&qc.ap.link.eh_info, "DMA Failed: Timeout");
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn wait4buf(acdev: *mut arasan_cf_dev) -> c_int {
    static inline int wait4buf(struct arasan_cf_dev *acdev)
    {
    if (!wait_for_completion_timeout(&acdev.cf_completion, TIMEOUT)) {
    let mut rw: u32 = acdev.qc.tf.flags & ATA_TFLAG_WRITE;
    dev_err(acdev.host.dev, "%s TimeOut\n", rw ? "write" : "read");
    return -ETIMEDOUT;
    }
// Check if PIO Error interrupt has occurred
    if (acdev.dma_status & ATA_DMA_ERR)
    return -EAGAIN;
    return 0;
    }
    static int
    dma_xfer(struct arasan_cf_dev *acdev, dma_addr_t src, dma_addr_t dest, u32 len)
    {
    struct dma_async_tx_descriptor *tx;
    struct dma_chan *chan = acdev.dma_chan;
    dma_cookie_t cookie;
    let mut flags: c_ulong = DMA_PREP_INTERRUPT;
    let mut ret: c_int = 0;
    tx = chan.device.device_prep_dma_memcpy(chan, dest, src, len, flags);
    if (!tx) {
    dev_err(acdev.host.dev, "device_prep_dma_memcpy failed\n");
    return -EAGAIN;
    }
    tx.callback = dma_callback;
    tx.callback_param = acdev;
    cookie = tx.tx_submit(tx);
    ret = dma_submit_error(cookie);
    if (ret) {
    dev_err(acdev.host.dev, "dma_submit_error\n");
    return ret;
    }
    chan.device.device_issue_pending(chan);
// Wait for DMA to complete
    if (!wait_for_completion_timeout(&acdev.dma_completion, TIMEOUT)) {
    dmaengine_terminate_all(chan);
    dev_err(acdev.host.dev, "wait_for_completion_timeout\n");
    return -ETIMEDOUT;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sg_xfer(acdev: *mut arasan_cf_dev, sg: *mut scatterlist) -> c_int {
    static int sg_xfer(struct arasan_cf_dev *acdev, struct scatterlist *sg)
    {
    let mut dest: dma_addr_t = 0, src = 0;
    u32 xfer_cnt, sglen, dma_len, xfer_ctr;
    let mut write: u32 = acdev.qc.tf.flags & ATA_TFLAG_WRITE;
    unsigned long flags;
    let mut ret: c_int = 0;
    sglen = sg_dma_len(sg);
    if (write) {
    src = sg_dma_address(sg);
    dest = acdev.pbase + EXT_WRITE_PORT;
    } else {
    dest = sg_dma_address(sg);
    src = acdev.pbase + EXT_READ_PORT;
    }
//
// For each sg:
// MAX_XFER_COUNT data will be transferred before we get transfer
// complete interrupt. Between after FIFO_SIZE data
// buffer available interrupt will be generated. At this time we will
// fill FIFO again: max FIFO_SIZE data.
//
    while (sglen) {
    xfer_cnt = min(sglen, MAX_XFER_COUNT);
    spin_lock_irqsave(&acdev.host.lock, flags);
    xfer_ctr = readl(acdev.vbase + XFER_CTR) &
    ~XFER_COUNT_MASK;
    writel(xfer_ctr | xfer_cnt | XFER_START,
    acdev.vbase + XFER_CTR);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
// continue dma xfers until current sg is completed
    while (xfer_cnt) {
// wait for read to complete
    if (!write) {
    ret = wait4buf(acdev);
    if (ret)
    goto fail;
    }
// read/write FIFO in chunk of FIFO_SIZE
    dma_len = min(xfer_cnt, FIFO_SIZE);
    ret = dma_xfer(acdev, src, dest, dma_len);
    if (ret) {
    dev_err(acdev.host.dev, "dma failed\n");
    goto fail;
    }
    if (write)
    src += dma_len;
    else
    dest += dma_len;
    sglen -= dma_len;
    xfer_cnt -= dma_len;
// wait for write to complete
    if (write) {
    ret = wait4buf(acdev);
    if (ret)
    goto fail;
    }
    }
    }
    fail:
    spin_lock_irqsave(&acdev.host.lock, flags);
    writel(readl(acdev.vbase + XFER_CTR) & ~XFER_START,
    acdev.vbase + XFER_CTR);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    return ret;
    }
//
// This routine uses External DMA controller to read/write data to FIFO of CF
// controller. There are two xfer related interrupt supported by CF controller:
// - buf_avail: This interrupt is generated as soon as we have buffer of 512
// bytes available for reading or empty buffer available for writing.
// - xfer_done: This interrupt is generated on transfer of "xfer_size" amount of
// data to/from FIFO. xfer_size is programmed in XFER_CTR register.
//
// Max buffer size = FIFO_SIZE = 512 Bytes.
// Max xfer_size = MAX_XFER_COUNT = 256 KB.
//
#[no_mangle]
unsafe extern "C" fn data_xfer(work: *mut work_struct) {
    static void data_xfer(struct work_struct *work)
    {
    struct arasan_cf_dev *acdev = container_of(work, struct arasan_cf_dev,
    work);
    struct ata_queued_cmd *qc = acdev.qc;
    struct scatterlist *sg;
    unsigned long flags;
    u32 temp;
    let mut ret: c_int = 0;
// request dma channels
// dma_request_channel may sleep, so calling from process context
    acdev.dma_chan = dma_request_chan(acdev.host.dev, "data");
    if (IS_ERR(acdev.dma_chan)) {
    dev_err_probe(acdev.host.dev, PTR_ERR(acdev.dma_chan),
    "Unable to get dma_chan\n");
    acdev.dma_chan = core::ptr::null_mut();
    goto chan_request_fail;
    }
    for_each_sg(qc.sg, sg, qc.n_elem, temp) {
    ret = sg_xfer(acdev, sg);
    if (ret)
    break;
    }
    dma_release_channel(acdev.dma_chan);
    acdev.dma_chan = core::ptr::null_mut();
// data xferred successfully
    if (!ret) {
    u32 status;
    spin_lock_irqsave(&acdev.host.lock, flags);
    status = ioread8(qc.ap.ioaddr.altstatus_addr);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    if (status & (ATA_BUSY | ATA_DRQ)) {
    ata_sff_queue_delayed_work(&acdev.dwork, 1);
    return;
    }
    goto sff_intr;
    }
    cf_dumpregs(acdev);
    chan_request_fail:
    spin_lock_irqsave(&acdev.host.lock, flags);
// error when transferring data to/from memory
    qc.err_mask |= AC_ERR_HOST_BUS;
    qc.ap.hsm_task_state = HSM_ST_ERR;
    cf_ctrl_reset(acdev);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    sff_intr:
    dma_complete(acdev);
    }
#[no_mangle]
unsafe extern "C" fn delayed_finish(work: *mut work_struct) {
    static void delayed_finish(struct work_struct *work)
    {
    struct arasan_cf_dev *acdev = container_of(work, struct arasan_cf_dev,
    dwork.work);
    struct ata_queued_cmd *qc = acdev.qc;
    unsigned long flags;
    u8 status;
    spin_lock_irqsave(&acdev.host.lock, flags);
    status = ioread8(qc.ap.ioaddr.altstatus_addr);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    if (status & (ATA_BUSY | ATA_DRQ))
    ata_sff_queue_delayed_work(&acdev.dwork, 1);
    else
    dma_complete(acdev);
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t arasan_cf_interrupt(int irq, void *dev)
    {
    struct arasan_cf_dev *acdev = ((struct ata_host *)dev).private_data;
    unsigned long flags;
    u32 irqsts;
    irqsts = readl(acdev.vbase + GIRQ_STS);
    if (!(irqsts & GIRQ_CF))
    return IRQ_NONE;
    spin_lock_irqsave(&acdev.host.lock, flags);
    irqsts = readl(acdev.vbase + IRQ_STS);
    writel(irqsts, acdev.vbase + IRQ_STS);		/* clear irqs */
    writel(GIRQ_CF, acdev.vbase + GIRQ_STS);	/* clear girqs */
// handle only relevant interrupts
    irqsts &= ~IGNORED_IRQS;
    if (irqsts & CARD_DETECT_IRQ) {
    cf_card_detect(acdev, 1);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    return IRQ_HANDLED;
    }
    if (irqsts & PIO_XFER_ERR_IRQ) {
    acdev.dma_status = ATA_DMA_ERR;
    writel(readl(acdev.vbase + XFER_CTR) & ~XFER_START,
    acdev.vbase + XFER_CTR);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    complete(&acdev.cf_completion);
    dev_err(acdev.host.dev, "pio xfer err irq\n");
    return IRQ_HANDLED;
    }
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    if (irqsts & BUF_AVAIL_IRQ) {
    complete(&acdev.cf_completion);
    return IRQ_HANDLED;
    }
    if (irqsts & XFER_DONE_IRQ) {
    struct ata_queued_cmd *qc = acdev.qc;
// Send Complete only for write
    if (qc.tf.flags & ATA_TFLAG_WRITE)
    complete(&acdev.cf_completion);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_freeze(ap: *mut ata_port) {
    static void arasan_cf_freeze(struct ata_port *ap)
    {
    struct arasan_cf_dev *acdev = ap.host.private_data;
// stop transfer and reset controller
    writel(readl(acdev.vbase + XFER_CTR) & ~XFER_START,
    acdev.vbase + XFER_CTR);
    cf_ctrl_reset(acdev);
    acdev.dma_status = ATA_DMA_ERR;
    ata_sff_dma_pause(ap);
    ata_sff_freeze(ap);
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_error_handler(ap: *mut ata_port) {
    static void arasan_cf_error_handler(struct ata_port *ap)
    __must_hold(&ap.host.eh_mutex)
    {
    struct arasan_cf_dev *acdev = ap.host.private_data;
//
// DMA transfers using an external DMA controller may be scheduled.
// Abort them before handling error. Refer data_xfer() for further
// details.
//
    cancel_work_sync(&acdev.work);
    cancel_delayed_work_sync(&acdev.dwork);
    return ata_sff_error_handler(ap);
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_dma_start(acdev: *mut arasan_cf_dev) {
    static void arasan_cf_dma_start(struct arasan_cf_dev *acdev)
    {
    struct ata_queued_cmd *qc = acdev.qc;
    struct ata_port *ap = qc.ap;
    struct ata_taskfile *tf = &qc.tf;
    let mut xfer_ctr: u32 = readl(acdev.vbase + XFER_CTR) & ~XFER_DIR_MASK;
    let mut write: u32 = tf.flags & ATA_TFLAG_WRITE;
    xfer_ctr |= write ? XFER_WRITE : XFER_READ;
    writel(xfer_ctr, acdev.vbase + XFER_CTR);
    ap.ops.sff_exec_command(ap, tf);
    ata_sff_queue_work(&acdev.work);
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_qc_issue(qc: *mut ata_queued_cmd) -> c_uint {
    static unsigned int arasan_cf_qc_issue(struct ata_queued_cmd *qc)
    {
    struct ata_port *ap = qc.ap;
    struct arasan_cf_dev *acdev = ap.host.private_data;
// defer PIO handling to sff_qc_issue
    if (!ata_is_dma(qc.tf.protocol))
    return ata_sff_qc_issue(qc);
// select the device
    ata_wait_idle(ap);
    ata_sff_dev_select(ap, qc.dev.devno);
    ata_wait_idle(ap);
// start the command
    switch (qc.tf.protocol) {
    case ATA_PROT_DMA:
    WARN_ON_ONCE(qc.tf.flags & ATA_TFLAG_POLLING);
    trace_ata_tf_load(ap, &qc.tf);
    ap.ops.sff_tf_load(ap, &qc.tf);
    acdev.dma_status = 0;
    acdev.qc = qc;
    trace_ata_bmdma_start(ap, &qc.tf, qc.tag);
    arasan_cf_dma_start(acdev);
    ap.hsm_task_state = HSM_ST_LAST;
    break;
    default:
    WARN_ON(1);
    return AC_ERR_SYSTEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_set_piomode(ap: *mut ata_port, adev: *mut ata_device) {
    static void arasan_cf_set_piomode(struct ata_port *ap, struct ata_device *adev)
    {
    struct arasan_cf_dev *acdev = ap.host.private_data;
    let mut pio: u8 = adev.pio_mode - XFER_PIO_0;
    unsigned long flags;
    u32 val;
// Arasan ctrl supports Mode0 -> Mode6
    if (pio > 6) {
    dev_err(ap.dev, "Unknown PIO mode\n");
    return;
    }
    spin_lock_irqsave(&acdev.host.lock, flags);
    val = readl(acdev.vbase + OP_MODE) &
    ~(ULTRA_DMA_ENB | MULTI_WORD_DMA_ENB | DRQ_BLOCK_SIZE_MASK);
    writel(val, acdev.vbase + OP_MODE);
    val = readl(acdev.vbase + TM_CFG) & ~TRUEIDE_PIO_TIMING_MASK;
    val |= pio << TRUEIDE_PIO_TIMING_SHIFT;
    writel(val, acdev.vbase + TM_CFG);
    cf_interrupt_enable(acdev, BUF_AVAIL_IRQ | XFER_DONE_IRQ, 0);
    cf_interrupt_enable(acdev, PIO_XFER_ERR_IRQ, 1);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_set_dmamode(ap: *mut ata_port, adev: *mut ata_device) {
    static void arasan_cf_set_dmamode(struct ata_port *ap, struct ata_device *adev)
    {
    struct arasan_cf_dev *acdev = ap.host.private_data;
    u32 opmode, tmcfg, dma_mode = adev.dma_mode;
    unsigned long flags;
    spin_lock_irqsave(&acdev.host.lock, flags);
    opmode = readl(acdev.vbase + OP_MODE) &
    ~(MULTI_WORD_DMA_ENB | ULTRA_DMA_ENB);
    tmcfg = readl(acdev.vbase + TM_CFG);
    if ((dma_mode >= XFER_UDMA_0) && (dma_mode <= XFER_UDMA_6)) {
    opmode |= ULTRA_DMA_ENB;
    tmcfg &= ~ULTRA_DMA_TIMING_MASK;
    tmcfg |= (dma_mode - XFER_UDMA_0) << ULTRA_DMA_TIMING_SHIFT;
    } else if ((dma_mode >= XFER_MW_DMA_0) && (dma_mode <= XFER_MW_DMA_4)) {
    opmode |= MULTI_WORD_DMA_ENB;
    tmcfg &= ~TRUEIDE_MWORD_DMA_TIMING_MASK;
    tmcfg |= (dma_mode - XFER_MW_DMA_0) <<
    TRUEIDE_MWORD_DMA_TIMING_SHIFT;
    } else {
    dev_err(ap.dev, "Unknown DMA mode\n");
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    return;
    }
    writel(opmode, acdev.vbase + OP_MODE);
    writel(tmcfg, acdev.vbase + TM_CFG);
    writel(DMA_XFER_MODE, acdev.vbase + XFER_CTR);
    cf_interrupt_enable(acdev, PIO_XFER_ERR_IRQ, 0);
    cf_interrupt_enable(acdev, BUF_AVAIL_IRQ | XFER_DONE_IRQ, 1);
    spin_unlock_irqrestore(&acdev.host.lock, flags);
    }
    static struct ata_port_operations arasan_cf_ops = {
    .inherits = &ata_sff_port_ops,
    .freeze = arasan_cf_freeze,
    .error_handler = arasan_cf_error_handler,
    .qc_issue = arasan_cf_qc_issue,
    .set_piomode = arasan_cf_set_piomode,
    .set_dmamode = arasan_cf_set_dmamode,
    };
#[no_mangle]
unsafe extern "C" fn arasan_cf_probe(pdev: *mut platform_device) -> c_int {
    static int arasan_cf_probe(struct platform_device *pdev)
    {
    struct arasan_cf_dev *acdev;
    struct arasan_cf_pdata *pdata = dev_get_platdata(&pdev.dev);
    struct ata_host *host;
    struct ata_port *ap;
    struct resource *res;
    u32 quirk;
    let mut irq_handler: irq_handler_t = core::ptr::null_mut();
    int ret;
    acdev = devm_kzalloc(&pdev.dev, sizeof(*acdev), GFP_KERNEL);
    if (!acdev)
    return -ENOMEM;
    if (pdata)
    quirk = pdata.quirk;
    else
    quirk = CF_BROKEN_UDMA; /* as it is on spear1340 */
//
// If there's an error getting IRQ (or we do get IRQ0),
// support only PIO
//
    ret = platform_get_irq(pdev, 0);
    if (ret == -EPROBE_DEFER)
    return ret;
    if (ret > 0) {
    acdev.irq = ret;
    irq_handler = arasan_cf_interrupt;
    } else	{
    quirk |= CF_BROKEN_MWDMA | CF_BROKEN_UDMA;
    }
    acdev.vbase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(acdev.vbase))
    return PTR_ERR(acdev.vbase);
    acdev.pbase = res.start;
    acdev.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(acdev.clk)) {
    dev_warn(&pdev.dev, "Clock not found\n");
    return PTR_ERR(acdev.clk);
    }
// allocate host
    host = ata_host_alloc(&pdev.dev, 1);
    if (!host) {
    dev_warn(&pdev.dev, "alloc host fail\n");
    return -ENOMEM;
    }
    ap = host.ports[0];
    host.private_data = acdev;
    acdev.host = host;
    ap.ops = &arasan_cf_ops;
    ap.pio_mask = ATA_PIO6;
    ap.mwdma_mask = ATA_MWDMA4;
    ap.udma_mask = ATA_UDMA6;
    init_completion(&acdev.cf_completion);
    init_completion(&acdev.dma_completion);
    INIT_WORK(&acdev.work, data_xfer);
    INIT_DELAYED_WORK(&acdev.dwork, delayed_finish);
    dma_cap_set(DMA_MEMCPY, acdev.mask);
// Handle platform specific quirks
    if (quirk) {
    if (quirk & CF_BROKEN_PIO) {
    ap.ops.set_piomode = core::ptr::null_mut();
    ap.pio_mask = 0;
    }
    if (quirk & CF_BROKEN_MWDMA)
    ap.mwdma_mask = 0;
    if (quirk & CF_BROKEN_UDMA)
    ap.udma_mask = 0;
    }
    ap.flags |= ATA_FLAG_PIO_POLLING | ATA_FLAG_NO_ATAPI;
    ap.ioaddr.cmd_addr = acdev.vbase + ATA_DATA_PORT;
    ap.ioaddr.data_addr = acdev.vbase + ATA_DATA_PORT;
    ap.ioaddr.error_addr = acdev.vbase + ATA_ERR_FTR;
    ap.ioaddr.feature_addr = acdev.vbase + ATA_ERR_FTR;
    ap.ioaddr.nsect_addr = acdev.vbase + ATA_SC;
    ap.ioaddr.lbal_addr = acdev.vbase + ATA_SN;
    ap.ioaddr.lbam_addr = acdev.vbase + ATA_CL;
    ap.ioaddr.lbah_addr = acdev.vbase + ATA_CH;
    ap.ioaddr.device_addr = acdev.vbase + ATA_SH;
    ap.ioaddr.status_addr = acdev.vbase + ATA_STS_CMD;
    ap.ioaddr.command_addr = acdev.vbase + ATA_STS_CMD;
    ap.ioaddr.altstatus_addr = acdev.vbase + ATA_ASTS_DCTR;
    ap.ioaddr.ctl_addr = acdev.vbase + ATA_ASTS_DCTR;
    ata_port_desc(ap, "phy_addr %llx virt_addr %p",
    (unsigned long long) res.start, acdev.vbase);
    ret = cf_init(acdev);
    if (ret)
    return ret;
    cf_card_detect(acdev, 0);
    ret = ata_host_activate(host, acdev.irq, irq_handler, 0,
    &arasan_cf_sht);
    if (!ret)
    return 0;
    cf_exit(acdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_remove(pdev: *mut platform_device) {
    static void arasan_cf_remove(struct platform_device *pdev)
    {
    struct ata_host *host = platform_get_drvdata(pdev);
    struct arasan_cf_dev *acdev = host.ports[0].private_data;
    ata_host_detach(host);
    cf_exit(acdev);
    }

#[no_mangle]
unsafe extern "C" fn arasan_cf_suspend(dev: *mut device) -> c_int {
    static int arasan_cf_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct arasan_cf_dev *acdev = host.ports[0].private_data;
    if (acdev.dma_chan)
    dmaengine_terminate_all(acdev.dma_chan);
    cf_exit(acdev);
    ata_host_suspend(host, PMSG_SUSPEND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arasan_cf_resume(dev: *mut device) -> c_int {
    static int arasan_cf_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct arasan_cf_dev *acdev = host.ports[0].private_data;
    cf_init(acdev);
    ata_host_resume(host);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(arasan_cf_pm_ops, arasan_cf_suspend, arasan_cf_resume);

    static const struct of_device_id arasan_cf_id_table[] = {
    { .compatible = "arasan,cf-spear1340" },
    {}
    };
    MODULE_DEVICE_TABLE(of, arasan_cf_id_table);

    static struct platform_driver arasan_cf_driver = {
    .probe		= arasan_cf_probe,
    .remove		= arasan_cf_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .pm	= &arasan_cf_pm_ops,
    .of_match_table = of_match_ptr(arasan_cf_id_table),
    },
    };
    module_platform_driver(arasan_cf_driver);
    MODULE_AUTHOR("Viresh Kumar <vireshk@kernel.org>");
    MODULE_DESCRIPTION("Arasan ATA Compact Flash driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRIVER_NAME);
