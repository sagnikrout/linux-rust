//! Automatically rewritten from C to Rust
//! Source: drivers/dma/imx-dma.c
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
// drivers/dma/imx-dma.c
//
// This file contains a driver for the Freescale i.MX DMA engine
// found on i.MX1/21/27
//
// Copyright 2010 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
// Copyright 2012 Javier Martin, Vista Silicon <javier.martin@vista-silicon.com>

pub const IMXDMA_MAX_CHAN_DESCRIPTORS: c_int = 16;
pub const IMX_DMA_CHANNELS: c_int = 16;
pub const IMX_DMA_2D_SLOTS: c_int = 2;
pub const IMX_DMA_2D_SLOT_A: c_int = 0;
pub const IMX_DMA_2D_SLOT_B: c_int = 1;

pub const DMA_DCR: c_uint = 0x00		/* Control Register */;
pub const DMA_DISR: c_uint = 0x04		/* Interrupt status Register */;
pub const DMA_DIMR: c_uint = 0x08		/* Interrupt mask Register */;
pub const DMA_DBTOSR: c_uint = 0x0c		/* Burst timeout status Register */;
pub const DMA_DRTOSR: c_uint = 0x10		/* Request timeout Register */;
pub const DMA_DSESR: c_uint = 0x14		/* Transfer Error Status Register */;
pub const DMA_DBOSR: c_uint = 0x18		/* Buffer overflow status Register */;
pub const DMA_DBTOCR: c_uint = 0x1c		/* Burst timeout control Register */;
pub const DMA_WSRA: c_uint = 0x40		/* W-Size Register A */;
pub const DMA_XSRA: c_uint = 0x44		/* X-Size Register A */;
pub const DMA_YSRA: c_uint = 0x48		/* Y-Size Register A */;
pub const DMA_WSRB: c_uint = 0x4c		/* W-Size Register B */;
pub const DMA_XSRB: c_uint = 0x50		/* X-Size Register B */;
pub const DMA_YSRB: c_uint = 0x54		/* Y-Size Register B */;

    enum  imxdma_prep_type {
    IMXDMA_DESC_MEMCPY,
    IMXDMA_DESC_INTERLEAVED,
    IMXDMA_DESC_SLAVE_SG,
    IMXDMA_DESC_CYCLIC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dma_2d_config {
    pub xsr: u16,
    pub ysr: u16,
    pub wsr: u16,
    pub count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imxdma_desc {
    pub node: list_head,
    pub desc: dma_async_tx_descriptor,
    pub status: enum dma_status,
    pub src: dma_addr_t,
    pub dest: dma_addr_t,
    pub len: usize,
    pub direction: enum dma_transfer_direction,
    pub type: enum imxdma_prep_type,
// For memcpy and interleaved
    pub config_port: c_uint,
    pub config_mem: c_uint,
// For interleaved transfers
    pub x: c_uint,
    pub y: c_uint,
    pub w: c_uint,
// For slave sg and cyclic
    pub sg: *mut scatterlist,
    pub sgcount: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imxdma_channel {
    pub hw_chaining: c_int,
    pub watchdog: timer_list,
    pub imxdma: *mut imxdma_engine,
    pub channel: c_uint,
    pub dma_tasklet: tasklet_struct,
    pub ld_free: list_head,
    pub ld_queue: list_head,
    pub ld_active: list_head,
    pub descs_allocated: c_int,
    pub word_size: enum dma_slave_buswidth,
    pub per_address: dma_addr_t,
    pub watermark_level: u32,
    pub chan: dma_chan,
    pub desc: dma_async_tx_descriptor,
    pub status: enum dma_status,
    pub dma_request: c_int,
    pub sg_list: *mut scatterlist,
    pub ccr_from_device: u32,
    pub ccr_to_device: u32,
    pub enabled_2d: bool,
    pub slot_2d: c_int,
    pub irq: c_uint,
    pub config: dma_slave_config,
}

    enum imx_dma_type {
    IMX1_DMA,
    IMX27_DMA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imxdma_engine {
    pub dev: *mut device,
    pub dma_device: dma_device,
    pub base: *mut void __iomem,
    pub dma_ahb: *mut clk,
    pub dma_ipg: *mut clk,
    pub lock: spinlock_t,
    pub slots_2d: [imx_dma_2d_config; IMX_DMA_2D_SLOTS],
    pub channel: [imxdma_channel; IMX_DMA_CHANNELS],
    pub devtype: enum imx_dma_type,
    pub irq: c_uint,
    pub irq_err: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imxdma_filter_data {
    pub imxdma: *mut imxdma_engine,
    pub request: c_int,
}

    static const struct of_device_id imx_dma_of_dev_id[] = {
    {
    .compatible = "fsl,imx1-dma", .data = (const void *)IMX1_DMA,
    }, {
    .compatible = "fsl,imx27-dma", .data = (const void *)IMX27_DMA,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, imx_dma_of_dev_id);
#[no_mangle]
pub unsafe extern "C" fn is_imx1_dma(imxdma: *mut imxdma_engine) -> c_int {
    static inline int is_imx1_dma(struct imxdma_engine *imxdma)
    {
    return imxdma.devtype == IMX1_DMA;
    }
#[no_mangle]
pub unsafe extern "C" fn is_imx27_dma(imxdma: *mut imxdma_engine) -> c_int {
    static inline int is_imx27_dma(struct imxdma_engine *imxdma)
    {
    return imxdma.devtype == IMX27_DMA;
    }
    static struct imxdma_channel *to_imxdma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct imxdma_channel, chan);
    }
#[no_mangle]
pub unsafe extern "C" fn imxdma_chan_is_doing_cyclic(imxdmac: *mut imxdma_channel) -> bool {
    static inline bool imxdma_chan_is_doing_cyclic(struct imxdma_channel *imxdmac)
    {
    struct imxdma_desc *desc;
    if (!list_empty(&imxdmac.ld_active)) {
    desc = list_first_entry(&imxdmac.ld_active, struct imxdma_desc,
    node);
    if (desc.type == IMXDMA_DESC_CYCLIC)
    return true;
    }
    return false;
    }
    static void imx_dmav1_writel(struct imxdma_engine *imxdma, unsigned val,
    unsigned offset)
    {
    __raw_writel(val, imxdma.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn imx_dmav1_readl(imxdma: *mut imxdma_engine, offset: unsigned) -> unsigned {
    static unsigned imx_dmav1_readl(struct imxdma_engine *imxdma, unsigned offset)
    {
    return __raw_readl(imxdma.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_hw_chain(imxdmac: *mut imxdma_channel) -> c_int {
    static int imxdma_hw_chain(struct imxdma_channel *imxdmac)
    {
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    if (is_imx27_dma(imxdma))
    return imxdmac.hw_chaining;
    else
    return 0;
    }
//
// imxdma_sg_next - prepare next chunk for scatter-gather DMA emulation
//
#[no_mangle]
pub unsafe extern "C" fn imxdma_sg_next(d: *mut imxdma_desc) {
    static inline void imxdma_sg_next(struct imxdma_desc *d)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(d.desc.chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct scatterlist *sg = d.sg;
    size_t now;
    now = min_t(size_t, d.len, sg_dma_len(sg));
    if (d.len != IMX_DMA_LENGTH_LOOP)
    d.len -= now;
    if (d.direction == DMA_DEV_TO_MEM)
    imx_dmav1_writel(imxdma, sg.dma_address,
    DMA_DAR(imxdmac.channel));
    else
    imx_dmav1_writel(imxdma, sg.dma_address,
    DMA_SAR(imxdmac.channel));
    imx_dmav1_writel(imxdma, now, DMA_CNTR(imxdmac.channel));
    dev_dbg(imxdma.dev, " %s channel: %d dst 0x%08x, src 0x%08x, "
    "size 0x%08x\n", __func__, imxdmac.channel,
    imx_dmav1_readl(imxdma, DMA_DAR(imxdmac.channel)),
    imx_dmav1_readl(imxdma, DMA_SAR(imxdmac.channel)),
    imx_dmav1_readl(imxdma, DMA_CNTR(imxdmac.channel)));
    }
#[no_mangle]
unsafe extern "C" fn imxdma_enable_hw(d: *mut imxdma_desc) {
    static void imxdma_enable_hw(struct imxdma_desc *d)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(d.desc.chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut channel: c_int = imxdmac.channel;
    unsigned long flags;
    dev_dbg(imxdma.dev, "%s channel %d\n", __func__, channel);
    local_irq_save(flags);
    imx_dmav1_writel(imxdma, 1 << channel, DMA_DISR);
    imx_dmav1_writel(imxdma, imx_dmav1_readl(imxdma, DMA_DIMR) &
    ~(1 << channel), DMA_DIMR);
    imx_dmav1_writel(imxdma, imx_dmav1_readl(imxdma, DMA_CCR(channel)) |
    CCR_CEN | CCR_ACRPT, DMA_CCR(channel));
    if (!is_imx1_dma(imxdma) &&
    d.sg && imxdma_hw_chain(imxdmac)) {
    d.sg = sg_next(d.sg);
    if (d.sg) {
    u32 tmp;
    imxdma_sg_next(d);
    tmp = imx_dmav1_readl(imxdma, DMA_CCR(channel));
    imx_dmav1_writel(imxdma, tmp | CCR_RPT | CCR_ACRPT,
    DMA_CCR(channel));
    }
    }
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_disable_hw(imxdmac: *mut imxdma_channel) {
    static void imxdma_disable_hw(struct imxdma_channel *imxdmac)
    {
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut channel: c_int = imxdmac.channel;
    unsigned long flags;
    dev_dbg(imxdma.dev, "%s channel %d\n", __func__, channel);
    if (imxdma_hw_chain(imxdmac))
    timer_delete(&imxdmac.watchdog);
    local_irq_save(flags);
    imx_dmav1_writel(imxdma, imx_dmav1_readl(imxdma, DMA_DIMR) |
    (1 << channel), DMA_DIMR);
    imx_dmav1_writel(imxdma, imx_dmav1_readl(imxdma, DMA_CCR(channel)) &
    ~CCR_CEN, DMA_CCR(channel));
    imx_dmav1_writel(imxdma, 1 << channel, DMA_DISR);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_watchdog(t: *mut timer_list) {
    static void imxdma_watchdog(struct timer_list *t)
    {
    struct imxdma_channel *imxdmac = timer_container_of(imxdmac, t,
    watchdog);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut channel: c_int = imxdmac.channel;
    imx_dmav1_writel(imxdma, 0, DMA_CCR(channel));
// Tasklet watchdog error handler
    tasklet_schedule(&imxdmac.dma_tasklet);
    dev_dbg(imxdma.dev, "channel %d: watchdog timeout!\n",
    imxdmac.channel);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_err_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t imxdma_err_handler(int irq, void *dev_id)
    {
    struct imxdma_engine *imxdma = dev_id;
    unsigned int err_mask;
    int i, disr;
    int errcode;
    disr = imx_dmav1_readl(imxdma, DMA_DISR);
    err_mask = imx_dmav1_readl(imxdma, DMA_DBTOSR) |
    imx_dmav1_readl(imxdma, DMA_DRTOSR) |
    imx_dmav1_readl(imxdma, DMA_DSESR)  |
    imx_dmav1_readl(imxdma, DMA_DBOSR);
    if (!err_mask)
    return IRQ_HANDLED;
    imx_dmav1_writel(imxdma, disr & err_mask, DMA_DISR);
    for (i = 0; i < IMX_DMA_CHANNELS; i++) {
    if (!(err_mask & (1 << i)))
    continue;
    errcode = 0;
    if (imx_dmav1_readl(imxdma, DMA_DBTOSR) & (1 << i)) {
    imx_dmav1_writel(imxdma, 1 << i, DMA_DBTOSR);
    errcode |= IMX_DMA_ERR_BURST;
    }
    if (imx_dmav1_readl(imxdma, DMA_DRTOSR) & (1 << i)) {
    imx_dmav1_writel(imxdma, 1 << i, DMA_DRTOSR);
    errcode |= IMX_DMA_ERR_REQUEST;
    }
    if (imx_dmav1_readl(imxdma, DMA_DSESR) & (1 << i)) {
    imx_dmav1_writel(imxdma, 1 << i, DMA_DSESR);
    errcode |= IMX_DMA_ERR_TRANSFER;
    }
    if (imx_dmav1_readl(imxdma, DMA_DBOSR) & (1 << i)) {
    imx_dmav1_writel(imxdma, 1 << i, DMA_DBOSR);
    errcode |= IMX_DMA_ERR_BUFFER;
    }
// Tasklet error handler
    tasklet_schedule(&imxdma.channel[i].dma_tasklet);
    dev_warn(imxdma.dev,
    "DMA timeout on channel %d -%s%s%s%s\n", i,
    errcode & IMX_DMA_ERR_BURST ?    " burst" : "",
    errcode & IMX_DMA_ERR_REQUEST ?  " request" : "",
    errcode & IMX_DMA_ERR_TRANSFER ? " transfer" : "",
    errcode & IMX_DMA_ERR_BUFFER ?   " buffer" : "");
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dma_irq_handle_channel(imxdmac: *mut imxdma_channel) {
    static void dma_irq_handle_channel(struct imxdma_channel *imxdmac)
    {
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut chno: c_int = imxdmac.channel;
    struct imxdma_desc *desc;
    unsigned long flags;
    spin_lock_irqsave(&imxdma.lock, flags);
    if (list_empty(&imxdmac.ld_active)) {
    spin_unlock_irqrestore(&imxdma.lock, flags);
    goto out;
    }
    desc = list_first_entry(&imxdmac.ld_active,
    struct imxdma_desc,
    node);
    spin_unlock_irqrestore(&imxdma.lock, flags);
    if (desc.sg) {
    u32 tmp;
    desc.sg = sg_next(desc.sg);
    if (desc.sg) {
    imxdma_sg_next(desc);
    tmp = imx_dmav1_readl(imxdma, DMA_CCR(chno));
    if (imxdma_hw_chain(imxdmac)) {
// FIXME: The timeout should probably be
// configurable
//
    mod_timer(&imxdmac.watchdog,
    jiffies + msecs_to_jiffies(500));
    tmp |= CCR_CEN | CCR_RPT | CCR_ACRPT;
    imx_dmav1_writel(imxdma, tmp, DMA_CCR(chno));
    } else {
    imx_dmav1_writel(imxdma, tmp & ~CCR_CEN,
    DMA_CCR(chno));
    tmp |= CCR_CEN;
    }
    imx_dmav1_writel(imxdma, tmp, DMA_CCR(chno));
    if (imxdma_chan_is_doing_cyclic(imxdmac))
// Tasklet progression
    tasklet_schedule(&imxdmac.dma_tasklet);
    return;
    }
    if (imxdma_hw_chain(imxdmac)) {
    timer_delete(&imxdmac.watchdog);
    return;
    }
    }
    out:
    imx_dmav1_writel(imxdma, 0, DMA_CCR(chno));
// Tasklet irq
    tasklet_schedule(&imxdmac.dma_tasklet);
    }
#[no_mangle]
unsafe extern "C" fn dma_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dma_irq_handler(int irq, void *dev_id)
    {
    struct imxdma_engine *imxdma = dev_id;
    int i, disr;
    if (!is_imx1_dma(imxdma))
    imxdma_err_handler(irq, dev_id);
    disr = imx_dmav1_readl(imxdma, DMA_DISR);
    dev_dbg(imxdma.dev, "%s called, disr=0x%08x\n", __func__, disr);
    imx_dmav1_writel(imxdma, disr, DMA_DISR);
    for (i = 0; i < IMX_DMA_CHANNELS; i++) {
    if (disr & (1 << i))
    dma_irq_handle_channel(&imxdma.channel[i]);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_xfer_desc(d: *mut imxdma_desc) -> c_int {
    static int imxdma_xfer_desc(struct imxdma_desc *d)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(d.desc.chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut slot: c_int = -1;
    int i;
// Configure and enable
    switch (d.type) {
    case IMXDMA_DESC_INTERLEAVED:
// Try to get a free 2D slot
    for (i = 0; i < IMX_DMA_2D_SLOTS; i++) {
    if ((imxdma.slots_2d[i].count > 0) &&
    ((imxdma.slots_2d[i].xsr != d.x) ||
    (imxdma.slots_2d[i].ysr != d.y) ||
    (imxdma.slots_2d[i].wsr != d.w)))
    continue;
    slot = i;
    break;
    }
    if (slot < 0)
    return -EBUSY;
    imxdma.slots_2d[slot].xsr = d.x;
    imxdma.slots_2d[slot].ysr = d.y;
    imxdma.slots_2d[slot].wsr = d.w;
    imxdma.slots_2d[slot].count++;
    imxdmac.slot_2d = slot;
    imxdmac.enabled_2d = true;
    if (slot == IMX_DMA_2D_SLOT_A) {
    d.config_mem &= ~CCR_MSEL_B;
    d.config_port &= ~CCR_MSEL_B;
    imx_dmav1_writel(imxdma, d.x, DMA_XSRA);
    imx_dmav1_writel(imxdma, d.y, DMA_YSRA);
    imx_dmav1_writel(imxdma, d.w, DMA_WSRA);
    } else {
    d.config_mem |= CCR_MSEL_B;
    d.config_port |= CCR_MSEL_B;
    imx_dmav1_writel(imxdma, d.x, DMA_XSRB);
    imx_dmav1_writel(imxdma, d.y, DMA_YSRB);
    imx_dmav1_writel(imxdma, d.w, DMA_WSRB);
    }
//
// We fall-through here intentionally, since a 2D transfer is
// similar to MEMCPY just adding the 2D slot configuration.
//
    fallthrough;
    case IMXDMA_DESC_MEMCPY:
    imx_dmav1_writel(imxdma, d.src, DMA_SAR(imxdmac.channel));
    imx_dmav1_writel(imxdma, d.dest, DMA_DAR(imxdmac.channel));
    imx_dmav1_writel(imxdma, d.config_mem | (d.config_port << 2),
    DMA_CCR(imxdmac.channel));
    imx_dmav1_writel(imxdma, d.len, DMA_CNTR(imxdmac.channel));
    dev_dbg(imxdma.dev,
    "%s channel: %d dest=0x%08llx src=0x%08llx dma_length=%zu\n",
    __func__, imxdmac.channel,
    (unsigned long long)d.dest,
    (unsigned long long)d.src, d.len);
    break;
// Cyclic transfer is the same as slave_sg with special sg configuration.
    case IMXDMA_DESC_CYCLIC:
    case IMXDMA_DESC_SLAVE_SG:
    if (d.direction == DMA_DEV_TO_MEM) {
    imx_dmav1_writel(imxdma, imxdmac.per_address,
    DMA_SAR(imxdmac.channel));
    imx_dmav1_writel(imxdma, imxdmac.ccr_from_device,
    DMA_CCR(imxdmac.channel));
    dev_dbg(imxdma.dev,
    "%s channel: %d sg=%p sgcount=%d total length=%zu dev_addr=0x%08llx (dev2mem)\n",
    __func__, imxdmac.channel,
    d.sg, d.sgcount, d.len,
    (unsigned long long)imxdmac.per_address);
    } else if (d.direction == DMA_MEM_TO_DEV) {
    imx_dmav1_writel(imxdma, imxdmac.per_address,
    DMA_DAR(imxdmac.channel));
    imx_dmav1_writel(imxdma, imxdmac.ccr_to_device,
    DMA_CCR(imxdmac.channel));
    dev_dbg(imxdma.dev,
    "%s channel: %d sg=%p sgcount=%d total length=%zu dev_addr=0x%08llx (mem2dev)\n",
    __func__, imxdmac.channel,
    d.sg, d.sgcount, d.len,
    (unsigned long long)imxdmac.per_address);
    } else {
    dev_err(imxdma.dev, "%s channel: %d bad dma mode\n",
    __func__, imxdmac.channel);
    return -EINVAL;
    }
    imxdma_sg_next(d);
    break;
    default:
    return -EINVAL;
    }
    imxdma_enable_hw(d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_tasklet(t: *mut tasklet_struct) {
    static void imxdma_tasklet(struct tasklet_struct *t)
    {
    struct imxdma_channel *imxdmac = from_tasklet(imxdmac, t, dma_tasklet);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc, *next_desc;
    unsigned long flags;
    spin_lock_irqsave(&imxdma.lock, flags);
    if (list_empty(&imxdmac.ld_active)) {
// Someone might have called terminate all
    spin_unlock_irqrestore(&imxdma.lock, flags);
    return;
    }
    desc = list_first_entry(&imxdmac.ld_active, struct imxdma_desc, node);
// If we are dealing with a cyclic descriptor, keep it on ld_active
// and dont mark the descriptor as complete.
// Only in non-cyclic cases it would be marked as complete
//
    if (imxdma_chan_is_doing_cyclic(imxdmac))
    goto out;
    else
    dma_cookie_complete(&desc.desc);
// Free 2D slot if it was an interleaved transfer
    if (imxdmac.enabled_2d) {
    imxdma.slots_2d[imxdmac.slot_2d].count--;
    imxdmac.enabled_2d = false;
    }
    list_move_tail(imxdmac.ld_active.next, &imxdmac.ld_free);
    if (!list_empty(&imxdmac.ld_queue)) {
    next_desc = list_first_entry(&imxdmac.ld_queue,
    struct imxdma_desc, node);
    list_move_tail(imxdmac.ld_queue.next, &imxdmac.ld_active);
    if (imxdma_xfer_desc(next_desc) < 0)
    dev_warn(imxdma.dev, "%s: channel: %d couldn't xfer desc\n",
    __func__, imxdmac.channel);
    }
    out:
    spin_unlock_irqrestore(&imxdma.lock, flags);
    dmaengine_desc_get_callback_invoke(&desc.desc, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn imxdma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int imxdma_terminate_all(struct dma_chan *chan)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    unsigned long flags;
    imxdma_disable_hw(imxdmac);
    spin_lock_irqsave(&imxdma.lock, flags);
    list_splice_tail_init(&imxdmac.ld_active, &imxdmac.ld_free);
    list_splice_tail_init(&imxdmac.ld_queue, &imxdmac.ld_free);
    spin_unlock_irqrestore(&imxdma.lock, flags);
    return 0;
    }
    static int imxdma_config_write(struct dma_chan *chan,
    struct dma_slave_config *dmaengine_cfg,
    enum dma_transfer_direction direction)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    let mut mode: c_uint = 0;
    if (direction == DMA_DEV_TO_MEM) {
    imxdmac.per_address = dmaengine_cfg.src_addr;
    imxdmac.watermark_level = dmaengine_cfg.src_maxburst;
    imxdmac.word_size = dmaengine_cfg.src_addr_width;
    } else {
    imxdmac.per_address = dmaengine_cfg.dst_addr;
    imxdmac.watermark_level = dmaengine_cfg.dst_maxburst;
    imxdmac.word_size = dmaengine_cfg.dst_addr_width;
    }
    switch (imxdmac.word_size) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    mode = IMX_DMA_MEMSIZE_8;
    break;
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    mode = IMX_DMA_MEMSIZE_16;
    break;
    default:
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    mode = IMX_DMA_MEMSIZE_32;
    break;
    }
    imxdmac.hw_chaining = 0;
    imxdmac.ccr_from_device = (mode | IMX_DMA_TYPE_FIFO) |
    ((IMX_DMA_MEMSIZE_32 | IMX_DMA_TYPE_LINEAR) << 2) |
    CCR_REN;
    imxdmac.ccr_to_device =
    (IMX_DMA_MEMSIZE_32 | IMX_DMA_TYPE_LINEAR) |
    ((mode | IMX_DMA_TYPE_FIFO) << 2) | CCR_REN;
    imx_dmav1_writel(imxdma, imxdmac.dma_request,
    DMA_RSSR(imxdmac.channel));
// Set burst length
    imx_dmav1_writel(imxdma, imxdmac.watermark_level *
    imxdmac.word_size, DMA_BLR(imxdmac.channel));
    return 0;
    }
    static int imxdma_config(struct dma_chan *chan,
    struct dma_slave_config *dmaengine_cfg)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    memcpy(&imxdmac.config, dmaengine_cfg, sizeof(*dmaengine_cfg));
    return 0;
    }
    static enum dma_status imxdma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    return dma_cookie_status(chan, cookie, txstate);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t imxdma_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(tx.chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    dma_cookie_t cookie;
    unsigned long flags;
    spin_lock_irqsave(&imxdma.lock, flags);
    list_move_tail(imxdmac.ld_free.next, &imxdmac.ld_queue);
    cookie = dma_cookie_assign(tx);
    spin_unlock_irqrestore(&imxdma.lock, flags);
    return cookie;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int imxdma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imx_dma_data *data = chan.private;
    if (data != core::ptr::null_mut())
    imxdmac.dma_request = data.dma_request;
    while (imxdmac.descs_allocated < IMXDMA_MAX_CHAN_DESCRIPTORS) {
    struct imxdma_desc *desc;
    desc = kzalloc_obj(*desc);
    if (!desc)
    break;
    dma_async_tx_descriptor_init(&desc.desc, chan);
    desc.desc.tx_submit = imxdma_tx_submit;
// txd.flags will be overwritten in prep funcs
    desc.desc.flags = DMA_CTRL_ACK;
    desc.status = DMA_COMPLETE;
    list_add_tail(&desc.node, &imxdmac.ld_free);
    imxdmac.descs_allocated++;
    }
    if (!imxdmac.descs_allocated)
    return -ENOMEM;
    return imxdmac.descs_allocated;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_free_chan_resources(chan: *mut dma_chan) {
    static void imxdma_free_chan_resources(struct dma_chan *chan)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc, *_desc;
    unsigned long flags;
    spin_lock_irqsave(&imxdma.lock, flags);
    imxdma_disable_hw(imxdmac);
    list_splice_tail_init(&imxdmac.ld_active, &imxdmac.ld_free);
    list_splice_tail_init(&imxdmac.ld_queue, &imxdmac.ld_free);
    spin_unlock_irqrestore(&imxdma.lock, flags);
    list_for_each_entry_safe(desc, _desc, &imxdmac.ld_free, node) {
    kfree(desc);
    imxdmac.descs_allocated--;
    }
    INIT_LIST_HEAD(&imxdmac.ld_free);
    kfree(imxdmac.sg_list);
    imxdmac.sg_list = core::ptr::null_mut();
    }
    static struct dma_async_tx_descriptor *imxdma_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction direction,
    unsigned long flags, void *context)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct scatterlist *sg;
    int i, dma_length = 0;
    struct imxdma_desc *desc;
    if (list_empty(&imxdmac.ld_free) ||
    imxdma_chan_is_doing_cyclic(imxdmac))
    return core::ptr::null_mut();
    desc = list_first_entry(&imxdmac.ld_free, struct imxdma_desc, node);
    for_each_sg(sgl, sg, sg_len, i) {
    dma_length += sg_dma_len(sg);
    }
    imxdma_config_write(chan, &imxdmac.config, direction);
    switch (imxdmac.word_size) {
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    if (sg_dma_len(sgl) & 3 || sgl.dma_address & 3)
    return core::ptr::null_mut();
    break;
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    if (sg_dma_len(sgl) & 1 || sgl.dma_address & 1)
    return core::ptr::null_mut();
    break;
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    break;
    default:
    return core::ptr::null_mut();
    }
    desc.type = IMXDMA_DESC_SLAVE_SG;
    desc.sg = sgl;
    desc.sgcount = sg_len;
    desc.len = dma_length;
    desc.direction = direction;
    if (direction == DMA_DEV_TO_MEM) {
    desc.src = imxdmac.per_address;
    } else {
    desc.dest = imxdmac.per_address;
    }
    desc.desc.callback = core::ptr::null_mut();
    desc.desc.callback_param = core::ptr::null_mut();
    return &desc.desc;
    }
    static struct dma_async_tx_descriptor *imxdma_prep_dma_cyclic(
    struct dma_chan *chan, dma_addr_t dma_addr, size_t buf_len,
    size_t period_len, enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc;
    int i;
    let mut periods: c_uint = buf_len / period_len;
    dev_dbg(imxdma.dev, "%s channel: %d buf_len=%zu period_len=%zu\n",
    __func__, imxdmac.channel, buf_len, period_len);
    if (list_empty(&imxdmac.ld_free) ||
    imxdma_chan_is_doing_cyclic(imxdmac))
    return core::ptr::null_mut();
    desc = list_first_entry(&imxdmac.ld_free, struct imxdma_desc, node);
    kfree(imxdmac.sg_list);
    imxdmac.sg_list = kzalloc_objs(struct scatterlist, periods + 1,
    GFP_ATOMIC);
    if (!imxdmac.sg_list)
    return core::ptr::null_mut();
    sg_init_table(imxdmac.sg_list, periods);
    for (i = 0; i < periods; i++) {
    sg_assign_page(&imxdmac.sg_list[i], core::ptr::null_mut());
    imxdmac.sg_list[i].offset = 0;
    imxdmac.sg_list[i].dma_address = dma_addr;
    sg_dma_len(&imxdmac.sg_list[i]) = period_len;
    dma_addr += period_len;
    }
// close the loop
    sg_chain(imxdmac.sg_list, periods + 1, imxdmac.sg_list);
    desc.type = IMXDMA_DESC_CYCLIC;
    desc.sg = imxdmac.sg_list;
    desc.sgcount = periods;
    desc.len = IMX_DMA_LENGTH_LOOP;
    desc.direction = direction;
    if (direction == DMA_DEV_TO_MEM) {
    desc.src = imxdmac.per_address;
    } else {
    desc.dest = imxdmac.per_address;
    }
    desc.desc.callback = core::ptr::null_mut();
    desc.desc.callback_param = core::ptr::null_mut();
    imxdma_config_write(chan, &imxdmac.config, direction);
    return &desc.desc;
    }
    static struct dma_async_tx_descriptor *imxdma_prep_dma_memcpy(
    struct dma_chan *chan, dma_addr_t dest,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc;
    dev_dbg(imxdma.dev, "%s channel: %d src=0x%llx dst=0x%llx len=%zu\n",
    __func__, imxdmac.channel, (unsigned long long)src,
    (unsigned long long)dest, len);
    if (list_empty(&imxdmac.ld_free) ||
    imxdma_chan_is_doing_cyclic(imxdmac))
    return core::ptr::null_mut();
    desc = list_first_entry(&imxdmac.ld_free, struct imxdma_desc, node);
    desc.type = IMXDMA_DESC_MEMCPY;
    desc.src = src;
    desc.dest = dest;
    desc.len = len;
    desc.direction = DMA_MEM_TO_MEM;
    desc.config_port = IMX_DMA_MEMSIZE_32 | IMX_DMA_TYPE_LINEAR;
    desc.config_mem = IMX_DMA_MEMSIZE_32 | IMX_DMA_TYPE_LINEAR;
    desc.desc.callback = core::ptr::null_mut();
    desc.desc.callback_param = core::ptr::null_mut();
    return &desc.desc;
    }
    static struct dma_async_tx_descriptor *imxdma_prep_dma_interleaved(
    struct dma_chan *chan, struct dma_interleaved_template *xt,
    unsigned long flags)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc;
    dev_dbg(imxdma.dev, "%s channel: %d src_start=0x%llx dst_start=0x%llx\n"
    "   src_sgl=%s dst_sgl=%s numf=%zu frame_size=%zu\n", __func__,
    imxdmac.channel, (unsigned long long)xt.src_start,
    (unsigned long long) xt.dst_start,
    str_true_false(xt.src_sgl), str_true_false(xt.dst_sgl),
    xt.numf, xt.frame_size);
    if (list_empty(&imxdmac.ld_free) ||
    imxdma_chan_is_doing_cyclic(imxdmac))
    return core::ptr::null_mut();
    if (xt.frame_size != 1 || xt.numf <= 0 || xt.dir != DMA_MEM_TO_MEM)
    return core::ptr::null_mut();
    desc = list_first_entry(&imxdmac.ld_free, struct imxdma_desc, node);
    desc.type = IMXDMA_DESC_INTERLEAVED;
    desc.src = xt.src_start;
    desc.dest = xt.dst_start;
    desc.x = xt.sgl[0].size;
    desc.y = xt.numf;
    desc.w = xt.sgl[0].icg + desc.x;
    desc.len = desc.x * desc.y;
    desc.direction = DMA_MEM_TO_MEM;
    desc.config_port = IMX_DMA_MEMSIZE_32;
    desc.config_mem = IMX_DMA_MEMSIZE_32;
    if (xt.src_sgl)
    desc.config_mem |= IMX_DMA_TYPE_2D;
    if (xt.dst_sgl)
    desc.config_port |= IMX_DMA_TYPE_2D;
    desc.desc.callback = core::ptr::null_mut();
    desc.desc.callback_param = core::ptr::null_mut();
    return &desc.desc;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_issue_pending(chan: *mut dma_chan) {
    static void imxdma_issue_pending(struct dma_chan *chan)
    {
    struct imxdma_channel *imxdmac = to_imxdma_chan(chan);
    struct imxdma_engine *imxdma = imxdmac.imxdma;
    struct imxdma_desc *desc;
    unsigned long flags;
    spin_lock_irqsave(&imxdma.lock, flags);
    if (list_empty(&imxdmac.ld_active) &&
    !list_empty(&imxdmac.ld_queue)) {
    desc = list_first_entry(&imxdmac.ld_queue,
    struct imxdma_desc, node);
    if (imxdma_xfer_desc(desc) < 0) {
    dev_warn(imxdma.dev,
    "%s: channel: %d couldn't issue DMA xfer\n",
    __func__, imxdmac.channel);
    } else {
    list_move_tail(imxdmac.ld_queue.next,
    &imxdmac.ld_active);
    }
    }
    spin_unlock_irqrestore(&imxdma.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool imxdma_filter_fn(struct dma_chan *chan, void *param)
    {
    struct imxdma_filter_data *fdata = param;
    struct imxdma_channel *imxdma_chan = to_imxdma_chan(chan);
    if (chan.device.dev != fdata.imxdma.dev)
    return false;
    imxdma_chan.dma_request = fdata.request;
    chan.private = core::ptr::null_mut();
    return true;
    }
    static struct dma_chan *imxdma_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    let mut count: c_int = dma_spec.args_count;
    struct imxdma_engine *imxdma = ofdma.of_dma_data;
    struct imxdma_filter_data fdata = {
    .imxdma = imxdma,
    };
    if (count != 1)
    return core::ptr::null_mut();
    fdata.request = dma_spec.args[0];
    return dma_request_channel(imxdma.dma_device.cap_mask,
    imxdma_filter_fn, &fdata);
    }
#[no_mangle]
unsafe extern "C" fn imxdma_probe(pdev: *mut platform_device) -> int __init {
    static int __init imxdma_probe(struct platform_device *pdev)
    {
    struct imxdma_engine *imxdma;
    int ret, i;
    int irq, irq_err;
    imxdma = devm_kzalloc(&pdev.dev, sizeof(*imxdma), GFP_KERNEL);
    if (!imxdma)
    return -ENOMEM;
    imxdma.dev = &pdev.dev;
    imxdma.devtype = (uintptr_t)of_device_get_match_data(&pdev.dev);
    imxdma.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imxdma.base))
    return PTR_ERR(imxdma.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    imxdma.dma_ipg = devm_clk_get(&pdev.dev, "ipg");
    if (IS_ERR(imxdma.dma_ipg))
    return PTR_ERR(imxdma.dma_ipg);
    imxdma.dma_ahb = devm_clk_get(&pdev.dev, "ahb");
    if (IS_ERR(imxdma.dma_ahb))
    return PTR_ERR(imxdma.dma_ahb);
    ret = clk_prepare_enable(imxdma.dma_ipg);
    if (ret)
    return ret;
    ret = clk_prepare_enable(imxdma.dma_ahb);
    if (ret)
    goto disable_dma_ipg_clk;
// reset DMA module
    imx_dmav1_writel(imxdma, DCR_DRST, DMA_DCR);
    if (is_imx1_dma(imxdma)) {
    ret = devm_request_irq(&pdev.dev, irq,
    dma_irq_handler, 0, "DMA", imxdma);
    if (ret) {
    dev_warn(imxdma.dev, "Can't register IRQ for DMA\n");
    goto disable_dma_ahb_clk;
    }
    imxdma.irq = irq;
    irq_err = platform_get_irq(pdev, 1);
    if (irq_err < 0) {
    ret = irq_err;
    goto disable_dma_ahb_clk;
    }
    ret = devm_request_irq(&pdev.dev, irq_err,
    imxdma_err_handler, 0, "DMA", imxdma);
    if (ret) {
    dev_warn(imxdma.dev, "Can't register ERRIRQ for DMA\n");
    goto disable_dma_ahb_clk;
    }
    imxdma.irq_err = irq_err;
    }
// enable DMA module
    imx_dmav1_writel(imxdma, DCR_DEN, DMA_DCR);
// clear all interrupts
    imx_dmav1_writel(imxdma, (1 << IMX_DMA_CHANNELS) - 1, DMA_DISR);
// disable interrupts
    imx_dmav1_writel(imxdma, (1 << IMX_DMA_CHANNELS) - 1, DMA_DIMR);
    INIT_LIST_HEAD(&imxdma.dma_device.channels);
    dma_cap_set(DMA_SLAVE, imxdma.dma_device.cap_mask);
    dma_cap_set(DMA_CYCLIC, imxdma.dma_device.cap_mask);
    dma_cap_set(DMA_MEMCPY, imxdma.dma_device.cap_mask);
    dma_cap_set(DMA_INTERLEAVE, imxdma.dma_device.cap_mask);
// Initialize 2D global parameters
    for (i = 0; i < IMX_DMA_2D_SLOTS; i++)
    imxdma.slots_2d[i].count = 0;
    spin_lock_init(&imxdma.lock);
// Initialize channel parameters
    for (i = 0; i < IMX_DMA_CHANNELS; i++) {
    struct imxdma_channel *imxdmac = &imxdma.channel[i];
    if (!is_imx1_dma(imxdma)) {
    ret = devm_request_irq(&pdev.dev, irq + i,
    dma_irq_handler, 0, "DMA", imxdma);
    if (ret) {
    dev_warn(imxdma.dev, "Can't register IRQ %d "
    "for DMA channel %d\n",
    irq + i, i);
    goto disable_dma_ahb_clk;
    }
    imxdmac.irq = irq + i;
    timer_setup(&imxdmac.watchdog, imxdma_watchdog, 0);
    }
    imxdmac.imxdma = imxdma;
    INIT_LIST_HEAD(&imxdmac.ld_queue);
    INIT_LIST_HEAD(&imxdmac.ld_free);
    INIT_LIST_HEAD(&imxdmac.ld_active);
    tasklet_setup(&imxdmac.dma_tasklet, imxdma_tasklet);
    imxdmac.chan.device = &imxdma.dma_device;
    dma_cookie_init(&imxdmac.chan);
    imxdmac.channel = i;
// Add the channel to the DMAC list
    list_add_tail(&imxdmac.chan.device_node,
    &imxdma.dma_device.channels);
    }
    imxdma.dma_device.dev = &pdev.dev;
    imxdma.dma_device.device_alloc_chan_resources = imxdma_alloc_chan_resources;
    imxdma.dma_device.device_free_chan_resources = imxdma_free_chan_resources;
    imxdma.dma_device.device_tx_status = imxdma_tx_status;
    imxdma.dma_device.device_prep_slave_sg = imxdma_prep_slave_sg;
    imxdma.dma_device.device_prep_dma_cyclic = imxdma_prep_dma_cyclic;
    imxdma.dma_device.device_prep_dma_memcpy = imxdma_prep_dma_memcpy;
    imxdma.dma_device.device_prep_interleaved_dma = imxdma_prep_dma_interleaved;
    imxdma.dma_device.device_config = imxdma_config;
    imxdma.dma_device.device_terminate_all = imxdma_terminate_all;
    imxdma.dma_device.device_issue_pending = imxdma_issue_pending;
    platform_set_drvdata(pdev, imxdma);
    imxdma.dma_device.copy_align = DMAENGINE_ALIGN_4_BYTES;
    dma_set_max_seg_size(imxdma.dma_device.dev, 0xffffff);
    ret = dma_async_device_register(&imxdma.dma_device);
    if (ret) {
    dev_err(&pdev.dev, "unable to register\n");
    goto disable_dma_ahb_clk;
    }
    if (pdev.dev.of_node) {
    ret = of_dma_controller_register(pdev.dev.of_node,
    imxdma_xlate, imxdma);
    if (ret) {
    dev_err(&pdev.dev, "unable to register of_dma_controller\n");
    goto err_of_dma_controller;
    }
    }
    return 0;
    err_of_dma_controller:
    dma_async_device_unregister(&imxdma.dma_device);
    disable_dma_ahb_clk:
    clk_disable_unprepare(imxdma.dma_ahb);
    disable_dma_ipg_clk:
    clk_disable_unprepare(imxdma.dma_ipg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imxdma_free_irq(pdev: *mut platform_device, imxdma: *mut imxdma_engine) {
    static void imxdma_free_irq(struct platform_device *pdev, struct imxdma_engine *imxdma)
    {
    int i;
    if (is_imx1_dma(imxdma)) {
    disable_irq(imxdma.irq);
    disable_irq(imxdma.irq_err);
    }
    for (i = 0; i < IMX_DMA_CHANNELS; i++) {
    struct imxdma_channel *imxdmac = &imxdma.channel[i];
    if (!is_imx1_dma(imxdma))
    disable_irq(imxdmac.irq);
    tasklet_kill(&imxdmac.dma_tasklet);
    }
    }
#[no_mangle]
unsafe extern "C" fn imxdma_remove(pdev: *mut platform_device) {
    static void imxdma_remove(struct platform_device *pdev)
    {
    struct imxdma_engine *imxdma = platform_get_drvdata(pdev);
    imxdma_free_irq(pdev, imxdma);
    dma_async_device_unregister(&imxdma.dma_device);
    if (pdev.dev.of_node)
    of_dma_controller_free(pdev.dev.of_node);
    clk_disable_unprepare(imxdma.dma_ipg);
    clk_disable_unprepare(imxdma.dma_ahb);
    }
    static struct platform_driver imxdma_driver = {
    .driver		= {
    .name	= "imx-dma",
    .of_match_table = imx_dma_of_dev_id,
    },
    .remove		= imxdma_remove,
    };
#[no_mangle]
unsafe extern "C" fn imxdma_module_init() -> int __init {
    static int __init imxdma_module_init(void)
    {
    return platform_driver_probe(&imxdma_driver, imxdma_probe);
    }
    subsys_initcall(imxdma_module_init);
    MODULE_AUTHOR("Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>");
    MODULE_DESCRIPTION("i.MX dma driver");
    MODULE_LICENSE("GPL");
