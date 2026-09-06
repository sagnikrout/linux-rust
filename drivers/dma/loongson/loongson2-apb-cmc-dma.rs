//! Automatically rewritten from C to Rust
//! Source: drivers/dma/loongson/loongson2-apb-cmc-dma.c
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
// Loongson-2 Chain Multi-Channel DMA Controller driver
//
// Copyright (C) 2024-2026 Loongson Technology Corporation Limited
//

pub const LOONGSON2_CMCDMA_ISR: c_uint = 0x0	/* DMA Interrupt Status Register */;
pub const LOONGSON2_CMCDMA_IFCR: c_uint = 0x4	/* DMA Interrupt Flag Clear Register */;
pub const LOONGSON2_CMCDMA_CCR: c_uint = 0x8	/* DMA Channel Configuration Register */;
pub const LOONGSON2_CMCDMA_CNDTR: c_uint = 0xc	/* DMA Channel Transmit Count Register */;
pub const LOONGSON2_CMCDMA_CPAR: c_uint = 0x10	/* DMA Channel Peripheral Address Register */;
pub const LOONGSON2_CMCDMA_CMAR: c_uint = 0x14	/* DMA Channel Memory Address Register */;
// Bitfields of DMA interrupt status register

    (LOONGSON2_CMCDMA_TCI | LOONGSON2_CMCDMA_HTI | LOONGSON2_CMCDMA_TEI)
// Bitfields of DMA channel x Configuration Register

    (LOONGSON2_CMCDMA_CCR_PINC | LOONGSON2_CMCDMA_CCR_MINC | LOONGSON2_CMCDMA_CCR_PL_MASK)

    (LOONGSON2_CMCDMA_CCR_TCIE | LOONGSON2_CMCDMA_CCR_HTIE | LOONGSON2_CMCDMA_CCR_TEIE)

    (LOONGSON2_CMCDMA_CCR_CFG_MASK | LOONGSON2_CMCDMA_CCR_IRQ_MASK)

    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_chan_reg {
    pub ccr: u32,
    pub cndtr: u32,
    pub cpar: u32,
    pub cmar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_sg_req {
    pub len: u32,
    pub chan_reg: loongson2_cmc_dma_chan_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_desc {
    pub vdesc: virt_dma_desc,
    pub cyclic: bool,
    pub num_sgs: u32,
    pub __counted_by(num_sgs): loongson2_cmc_dma_sg_req sg_req[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_chan {
    pub vchan: virt_dma_chan,
    pub dma_sconfig: dma_slave_config,
    pub desc: *mut loongson2_cmc_dma_desc,
    pub id: u32,
    pub irq: u32,
    pub next_sg: u32,
    pub chan_reg: loongson2_cmc_dma_chan_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_dev {
    pub ddev: dma_device,
    pub dma_clk: *mut clk,
    pub base: *mut void __iomem,
    pub nr_channels: u32,
    pub chan_reg_offset: u32,
    pub __counted_by(nr_channels): loongson2_cmc_dma_chan chan[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_cmc_dma_config {
    pub max_channels: u32,
    pub chan_reg_offset: u32,
}

    static const struct loongson2_cmc_dma_config ls2k0300_cmc_dma_config = {
    .max_channels = 8,
    .chan_reg_offset = 0x14,
    };
    static const struct loongson2_cmc_dma_config ls2k3000_cmc_dma_config = {
    .max_channels = 4,
    .chan_reg_offset = 0x18,
    };
    static struct loongson2_cmc_dma_dev *lmdma_get_dev(struct loongson2_cmc_dma_chan *lchan)
    {
    return container_of(lchan.vchan.chan.device, struct loongson2_cmc_dma_dev, ddev);
    }
    static struct loongson2_cmc_dma_chan *to_lmdma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct loongson2_cmc_dma_chan, vchan.chan);
    }
    static struct loongson2_cmc_dma_desc *to_lmdma_desc(struct virt_dma_desc *vdesc)
    {
    return container_of(vdesc, struct loongson2_cmc_dma_desc, vdesc);
    }
    static struct device *chan2dev(struct loongson2_cmc_dma_chan *lchan)
    {
    return &lchan.vchan.chan.dev.device;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_read(lddev: *mut loongson2_cmc_dma_dev, reg: u32, id: u32) -> u32 {
    static u32 loongson2_cmc_dma_read(struct loongson2_cmc_dma_dev *lddev, u32 reg, u32 id)
    {
    return readl(lddev.base + (reg + lddev.chan_reg_offset * id));
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_write(lddev: *mut loongson2_cmc_dma_dev, reg: u32, id: u32, val: u32) {
    static void loongson2_cmc_dma_write(struct loongson2_cmc_dma_dev *lddev, u32 reg, u32 id, u32 val)
    {
    writel(val, lddev.base + (reg + lddev.chan_reg_offset * id));
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_get_width(width: enum dma_slave_buswidth) -> c_int {
    static int loongson2_cmc_dma_get_width(enum dma_slave_buswidth width)
    {
    switch (width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    return ffs(width) - 1;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int {
    static int loongson2_cmc_dma_slave_config(struct dma_chan *chan, struct dma_slave_config *config)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    memcpy(&lchan.dma_sconfig, config, sizeof(*config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_irq_clear(lchan: *mut loongson2_cmc_dma_chan, flags: u32) {
    static void loongson2_cmc_dma_irq_clear(struct loongson2_cmc_dma_chan *lchan, u32 flags)
    {
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    u32 ifcr;
    ifcr = flags << (4 * lchan.id);
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_IFCR, 0, ifcr);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_stop(lchan: *mut loongson2_cmc_dma_chan) {
    static void loongson2_cmc_dma_stop(struct loongson2_cmc_dma_chan *lchan)
    {
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    u32 ccr;
    ccr = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_CCR, lchan.id);
    ccr &= ~(LOONGSON2_CMCDMA_CCR_IRQ_MASK | LOONGSON2_CMCDMA_CCR_EN);
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CCR, lchan.id, ccr);
    loongson2_cmc_dma_irq_clear(lchan, LOONGSON2_CMCDMA_MASKI);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int loongson2_cmc_dma_terminate_all(struct dma_chan *chan)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    LIST_HEAD(head);
    scoped_guard(spinlock_irqsave, &lchan.vchan.lock) {
    if (lchan.desc) {
    vchan_terminate_vdesc(&lchan.desc.vdesc);
    loongson2_cmc_dma_stop(lchan);
    lchan.desc = core::ptr::null_mut();
    }
    vchan_get_all_descriptors(&lchan.vchan, &head);
    }
    vchan_dma_desc_free_list(&lchan.vchan, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_synchronize(chan: *mut dma_chan) {
    static void loongson2_cmc_dma_synchronize(struct dma_chan *chan)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    vchan_synchronize(&lchan.vchan);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_start_transfer(lchan: *mut loongson2_cmc_dma_chan) {
    static void loongson2_cmc_dma_start_transfer(struct loongson2_cmc_dma_chan *lchan)
    {
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    struct loongson2_cmc_dma_sg_req *sg_req;
    struct loongson2_cmc_dma_chan_reg *reg;
    struct virt_dma_desc *vdesc;
    loongson2_cmc_dma_stop(lchan);
    if (!lchan.desc) {
    vdesc = vchan_next_desc(&lchan.vchan);
    if (!vdesc)
    return;
    list_del(&vdesc.node);
    lchan.desc = to_lmdma_desc(vdesc);
    lchan.next_sg = 0;
    }
    if (lchan.next_sg == lchan.desc.num_sgs)
    lchan.next_sg = 0;
    sg_req = &lchan.desc.sg_req[lchan.next_sg];
    reg = &sg_req.chan_reg;
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CCR, lchan.id, reg.ccr);
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CNDTR, lchan.id, reg.cndtr);
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CPAR, lchan.id, reg.cpar);
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CMAR, lchan.id, reg.cmar);
    lchan.next_sg++;
// Start DMA
    reg.ccr |= LOONGSON2_CMCDMA_CCR_EN;
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CCR, lchan.id, reg.ccr);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_configure_next_sg(lchan: *mut loongson2_cmc_dma_chan) {
    static void loongson2_cmc_dma_configure_next_sg(struct loongson2_cmc_dma_chan *lchan)
    {
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    struct loongson2_cmc_dma_sg_req *sg_req;
    u32 ccr, id = lchan.id;
    if (lchan.next_sg == lchan.desc.num_sgs)
    lchan.next_sg = 0;
// Stop to update mem addr
    ccr = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_CCR, id);
    ccr &= ~LOONGSON2_CMCDMA_CCR_EN;
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CCR, id, ccr);
    sg_req = &lchan.desc.sg_req[lchan.next_sg];
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CMAR, id, sg_req.chan_reg.cmar);
// Start transition
    ccr |= LOONGSON2_CMCDMA_CCR_EN;
    loongson2_cmc_dma_write(lddev, LOONGSON2_CMCDMA_CCR, id, ccr);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_handle_chan_done(lchan: *mut loongson2_cmc_dma_chan) {
    static void loongson2_cmc_dma_handle_chan_done(struct loongson2_cmc_dma_chan *lchan)
    {
    if (!lchan.desc)
    return;
    if (lchan.desc.cyclic) {
    vchan_cyclic_callback(&lchan.desc.vdesc);
// LOONGSON2_CMCDMA_CCR_CIRC mode don't need update register
    if (lchan.desc.num_sgs == 1)
    return;
    loongson2_cmc_dma_configure_next_sg(lchan);
    lchan.next_sg++;
    } else {
    if (lchan.next_sg == lchan.desc.num_sgs) {
    vchan_cookie_complete(&lchan.desc.vdesc);
    lchan.desc = core::ptr::null_mut();
    }
    loongson2_cmc_dma_start_transfer(lchan);
    }
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_chan_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t loongson2_cmc_dma_chan_irq(int irq, void *devid)
    {
    struct loongson2_cmc_dma_chan *lchan = devid;
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    struct device *dev = chan2dev(lchan);
    u32 ists, status, ccr;
    scoped_guard(spinlock, &lchan.vchan.lock) {
    ccr = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_CCR, lchan.id);
    ists = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_ISR, 0);
    status = (ists >> (4 * lchan.id)) & LOONGSON2_CMCDMA_MASKI;
    loongson2_cmc_dma_irq_clear(lchan, status);
    if (status & LOONGSON2_CMCDMA_TCI) {
    loongson2_cmc_dma_handle_chan_done(lchan);
    status &= ~LOONGSON2_CMCDMA_TCI;
    }
    if (status & LOONGSON2_CMCDMA_HTI)
    status &= ~LOONGSON2_CMCDMA_HTI;
    if (status & LOONGSON2_CMCDMA_TEI) {
    dev_err(dev, "DMA Transform Error.\n");
    if (!(ccr & LOONGSON2_CMCDMA_CCR_EN))
    dev_err(dev, "Channel disabled by HW.\n");
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_issue_pending(chan: *mut dma_chan) {
    static void loongson2_cmc_dma_issue_pending(struct dma_chan *chan)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    guard(spinlock_irqsave)(&lchan.vchan.lock);
    if (vchan_issue_pending(&lchan.vchan) && !lchan.desc) {
    dev_dbg(chan2dev(lchan), "vchan %pK: issued\n", &lchan.vchan);
    loongson2_cmc_dma_start_transfer(lchan);
    }
    }
    static int loongson2_cmc_dma_set_xfer_param(struct loongson2_cmc_dma_chan *lchan,
    enum dma_transfer_direction direction,
    enum dma_slave_buswidth *buswidth, u32 buf_len)
    {
    let mut sconfig: dma_slave_config = lchan.dma_sconfig;
    struct device *dev = chan2dev(lchan);
    int dev_width;
    u32 ccr;
    switch (direction) {
    case DMA_MEM_TO_DEV:
    dev_width = loongson2_cmc_dma_get_width(sconfig.dst_addr_width);
    if (dev_width < 0) {
    dev_err(dev, "DMA_MEM_TO_DEV bus width not supported\n");
    return dev_width;
    }
    lchan.chan_reg.cpar = sconfig.dst_addr;
    ccr = LOONGSON2_CMCDMA_CCR_DIR;
// buswidth = sconfig.dst_addr_width;
    break;
    case DMA_DEV_TO_MEM:
    dev_width = loongson2_cmc_dma_get_width(sconfig.src_addr_width);
    if (dev_width < 0) {
    dev_err(dev, "DMA_DEV_TO_MEM bus width not supported\n");
    return dev_width;
    }
    lchan.chan_reg.cpar = sconfig.src_addr;
    ccr = LOONGSON2_CMCDMA_CCR_MINC;
// buswidth = sconfig.src_addr_width;
    break;
    default:
    return -EINVAL;
    }
    ccr |= FIELD_PREP(LOONGSON2_CMCDMA_CCR_PSIZE_MASK, dev_width) |
    FIELD_PREP(LOONGSON2_CMCDMA_CCR_MSIZE_MASK, dev_width);
// Set DMA control register
    lchan.chan_reg.ccr &= ~(LOONGSON2_CMCDMA_CCR_PSIZE_MASK | LOONGSON2_CMCDMA_CCR_MSIZE_MASK);
    lchan.chan_reg.ccr |= ccr;
    return 0;
    }
    static struct dma_async_tx_descriptor *
    loongson2_cmc_dma_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl, u32 sg_len,
    enum dma_transfer_direction direction,
    unsigned long flags, void *context)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    struct loongson2_cmc_dma_desc *desc;
    enum dma_slave_buswidth buswidth;
    struct scatterlist *sg;
    u32 num_items, i;
    int ret;
    desc = kzalloc_flex(*desc, sg_req, sg_len, GFP_NOWAIT);
    if (!desc)
    return ERR_PTR(-ENOMEM);
    for_each_sg(sgl, sg, sg_len, i) {
    ret = loongson2_cmc_dma_set_xfer_param(lchan, direction, &buswidth, sg_dma_len(sg));
    if (ret)
    return ERR_PTR(ret);
    num_items = DIV_ROUND_UP(sg_dma_len(sg), buswidth);
    if (num_items >= LOONSON2_CMCDMA_MAX_DATA_ITEMS) {
    dev_err(chan2dev(lchan), "Number of items not supported\n");
    kfree(desc);
    return ERR_PTR(-EINVAL);
    }
    desc.sg_req[i].len = sg_dma_len(sg);
    desc.sg_req[i].chan_reg.ccr = lchan.chan_reg.ccr;
    desc.sg_req[i].chan_reg.cpar = lchan.chan_reg.cpar;
    desc.sg_req[i].chan_reg.cmar = sg_dma_address(sg);
    desc.sg_req[i].chan_reg.cndtr = num_items;
    }
    desc.num_sgs = sg_len;
    desc.cyclic = false;
    return vchan_tx_prep(&lchan.vchan, &desc.vdesc, flags);
    }
    static struct dma_async_tx_descriptor *
    loongson2_cmc_dma_prep_dma_cyclic(struct dma_chan *chan, dma_addr_t buf_addr, size_t buf_len,
    size_t period_len, enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    struct loongson2_cmc_dma_desc *desc;
    enum dma_slave_buswidth buswidth;
    u32 num_periods, num_items, i;
    int ret;
    if (unlikely(buf_len % period_len))
    return ERR_PTR(-EINVAL);
    ret = loongson2_cmc_dma_set_xfer_param(lchan, direction, &buswidth, period_len);
    if (ret)
    return ERR_PTR(ret);
    num_items = DIV_ROUND_UP(period_len, buswidth);
    if (num_items >= LOONSON2_CMCDMA_MAX_DATA_ITEMS) {
    dev_err(chan2dev(lchan), "Number of items not supported\n");
    return ERR_PTR(-EINVAL);
    }
// Enable Circular mode
    if (buf_len == period_len)
    lchan.chan_reg.ccr |= LOONGSON2_CMCDMA_CCR_CIRC;
    num_periods = DIV_ROUND_UP(buf_len, period_len);
    desc = kzalloc_flex(*desc, sg_req, num_periods, GFP_NOWAIT);
    if (!desc)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < num_periods; i++) {
    desc.sg_req[i].len = period_len;
    desc.sg_req[i].chan_reg.ccr = lchan.chan_reg.ccr;
    desc.sg_req[i].chan_reg.cpar = lchan.chan_reg.cpar;
    desc.sg_req[i].chan_reg.cmar = buf_addr;
    desc.sg_req[i].chan_reg.cndtr = num_items;
    buf_addr += period_len;
    }
    desc.num_sgs = num_periods;
    desc.cyclic = true;
    return vchan_tx_prep(&lchan.vchan, &desc.vdesc, flags);
    }
    static size_t loongson2_cmc_dma_desc_residue(struct loongson2_cmc_dma_chan *lchan,
    struct loongson2_cmc_dma_desc *desc, u32 next_sg)
    {
    struct loongson2_cmc_dma_dev *lddev = lmdma_get_dev(lchan);
    u32 residue, width, ndtr, ccr, i;
    ccr = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_CCR, lchan.id);
    width = FIELD_GET(LOONGSON2_CMCDMA_CCR_PSIZE_MASK, ccr);
    ndtr = loongson2_cmc_dma_read(lddev, LOONGSON2_CMCDMA_CNDTR, lchan.id);
    residue = ndtr << width;
    if (lchan.desc.cyclic && next_sg == 0)
    return residue;
    for (i = next_sg; i < desc.num_sgs; i++)
    residue += desc.sg_req[i].len;
    return residue;
    }
    static enum dma_status loongson2_cmc_dma_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *state)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    struct virt_dma_desc *vdesc;
    enum dma_status status;
    status = dma_cookie_status(chan, cookie, state);
    if (status == DMA_COMPLETE || !state)
    return status;
    scoped_guard(spinlock_irqsave, &lchan.vchan.lock) {
    vdesc = vchan_find_desc(&lchan.vchan, cookie);
    if (lchan.desc && cookie == lchan.desc.vdesc.tx.cookie)
    state.residue = loongson2_cmc_dma_desc_residue(lchan, lchan.desc,
    lchan.next_sg);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: vdesc) -> else {
    else if (vdesc)
    state.residue = loongson2_cmc_dma_desc_residue(lchan,
    to_lmdma_desc(vdesc), 0);
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_free_chan_resources(chan: *mut dma_chan) {
    static void loongson2_cmc_dma_free_chan_resources(struct dma_chan *chan)
    {
    vchan_free_chan_resources(to_virt_chan(chan));
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_desc_free(vdesc: *mut virt_dma_desc) {
    static void loongson2_cmc_dma_desc_free(struct virt_dma_desc *vdesc)
    {
    kfree(to_lmdma_desc(vdesc));
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_acpi_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool loongson2_cmc_dma_acpi_filter(struct dma_chan *chan, void *param)
    {
    struct loongson2_cmc_dma_chan *lchan = to_lmdma_chan(chan);
    struct acpi_dma_spec *dma_spec = param;
    memset(&lchan.chan_reg, 0, sizeof(struct loongson2_cmc_dma_chan_reg));
    lchan.chan_reg.ccr = dma_spec.chan_id & LOONGSON2_CMCDMA_STREAM_MASK;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_acpi_controller_register(lddev: *mut loongson2_cmc_dma_dev) -> c_int {
    static int loongson2_cmc_dma_acpi_controller_register(struct loongson2_cmc_dma_dev *lddev)
    {
    struct device *dev = lddev.ddev.dev;
    struct acpi_dma_filter_info *info;
    if (!is_acpi_node(dev_fwnode(dev)))
    return 0;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    dma_cap_zero(info.dma_cap);
    info.dma_cap = lddev.ddev.cap_mask;
    info.filter_fn = loongson2_cmc_dma_acpi_filter;
    return devm_acpi_dma_controller_register(dev, acpi_dma_simple_xlate, info);
    }
    static struct dma_chan *loongson2_cmc_dma_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct loongson2_cmc_dma_dev *lddev = ofdma.of_dma_data;
    struct device *dev = lddev.ddev.dev;
    struct loongson2_cmc_dma_chan *lchan;
    struct dma_chan *chan;
    if (dma_spec.args_count < 2)
    return ERR_PTR(-EINVAL);
    if (dma_spec.args[0] >= lddev.nr_channels) {
    dev_err(dev, "Invalid channel id.\n");
    return ERR_PTR(-EINVAL);
    }
    lchan = &lddev.chan[dma_spec.args[0]];
    chan = dma_get_slave_channel(&lchan.vchan.chan);
    if (!chan) {
    dev_err(dev, "No more channels available.\n");
    return ERR_PTR(-EINVAL);
    }
    memset(&lchan.chan_reg, 0, sizeof(struct loongson2_cmc_dma_chan_reg));
    lchan.chan_reg.ccr = dma_spec.args[1] & LOONGSON2_CMCDMA_STREAM_MASK;
    return chan;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_of_controller_register(lddev: *mut loongson2_cmc_dma_dev) -> c_int {
    static int loongson2_cmc_dma_of_controller_register(struct loongson2_cmc_dma_dev *lddev)
    {
    struct device *dev = lddev.ddev.dev;
    if (!is_of_node(dev_fwnode(dev)))
    return 0;
    return of_dma_controller_register(dev.of_node, loongson2_cmc_dma_of_xlate, lddev);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_probe(pdev: *mut platform_device) -> c_int {
    static int loongson2_cmc_dma_probe(struct platform_device *pdev)
    {
    const struct loongson2_cmc_dma_config *config;
    struct loongson2_cmc_dma_chan *lchan;
    struct loongson2_cmc_dma_dev *lddev;
    struct device *dev = &pdev.dev;
    struct dma_device *ddev;
    u32 nr_chans, i;
    int ret;
    config = (const struct loongson2_cmc_dma_config *)device_get_match_data(dev);
    if (!config)
    return -EINVAL;
    ret = device_property_read_u32(dev, "dma-channels", &nr_chans);
    if (ret || nr_chans > config.max_channels) {
    dev_err(dev, "missing or invalid dma-channels property\n");
    nr_chans = config.max_channels;
    }
    lddev = devm_kzalloc(dev, struct_size(lddev, chan, nr_chans), GFP_KERNEL);
    if (!lddev)
    return -ENOMEM;
    lddev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lddev.base))
    return PTR_ERR(lddev.base);
    platform_set_drvdata(pdev, lddev);
    lddev.nr_channels = nr_chans;
    lddev.chan_reg_offset = config.chan_reg_offset;
    lddev.dma_clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(lddev.dma_clk))
    return dev_err_probe(dev, PTR_ERR(lddev.dma_clk), "Failed to get dma clock\n");
    ddev = &lddev.ddev;
    ddev.dev = dev;
    dma_cap_zero(ddev.cap_mask);
    dma_cap_set(DMA_SLAVE, ddev.cap_mask);
    dma_cap_set(DMA_PRIVATE, ddev.cap_mask);
    dma_cap_set(DMA_CYCLIC, ddev.cap_mask);
    ddev.device_free_chan_resources = loongson2_cmc_dma_free_chan_resources;
    ddev.device_config = loongson2_cmc_dma_slave_config;
    ddev.device_prep_slave_sg = loongson2_cmc_dma_prep_slave_sg;
    ddev.device_prep_dma_cyclic = loongson2_cmc_dma_prep_dma_cyclic;
    ddev.device_issue_pending = loongson2_cmc_dma_issue_pending;
    ddev.device_synchronize = loongson2_cmc_dma_synchronize;
    ddev.device_tx_status = loongson2_cmc_dma_tx_status;
    ddev.device_terminate_all = loongson2_cmc_dma_terminate_all;
    ddev.max_sg_burst = LOONSON2_CMCDMA_MAX_DATA_ITEMS;
    ddev.src_addr_widths = LOONGSON2_CMCDMA_BUSWIDTHS;
    ddev.dst_addr_widths = LOONGSON2_CMCDMA_BUSWIDTHS;
    ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    INIT_LIST_HEAD(&ddev.channels);
    for (i = 0; i < nr_chans; i++) {
    lchan = &lddev.chan[i];
    lchan.id = i;
    lchan.vchan.desc_free = loongson2_cmc_dma_desc_free;
    vchan_init(&lchan.vchan, ddev);
    }
    ret = dmaenginem_async_device_register(ddev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register DMA engine device.\n");
    for (i = 0; i < nr_chans; i++) {
    lchan = &lddev.chan[i];
    lchan.irq = platform_get_irq(pdev, i);
    if (lchan.irq < 0)
    return lchan.irq;
    ret = devm_request_irq(dev, lchan.irq, loongson2_cmc_dma_chan_irq, IRQF_SHARED,
    dev_name(chan2dev(lchan)), lchan);
    if (ret)
    return ret;
    }
    ret = loongson2_cmc_dma_acpi_controller_register(lddev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register dma controller with ACPI.\n");
    ret = loongson2_cmc_dma_of_controller_register(lddev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register dma controller with FDT.\n");
    dev_info(dev, "Loongson-2 Multi-Channel DMA Controller registered successfully.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_cmc_dma_remove(pdev: *mut platform_device) {
    static void loongson2_cmc_dma_remove(struct platform_device *pdev)
    {
    of_dma_controller_free(pdev.dev.of_node);
    }
    static const struct of_device_id loongson2_cmc_dma_of_match[] = {
    { .compatible = "loongson,ls2k0300-dma", .data = &ls2k0300_cmc_dma_config },
    { .compatible = "loongson,ls2k3000-dma", .data = &ls2k3000_cmc_dma_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, loongson2_cmc_dma_of_match);
    static const struct acpi_device_id loongson2_cmc_dma_acpi_match[] = {
    { "LOON0014", .driver_data = (kernel_ulong_t)&ls2k3000_cmc_dma_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, loongson2_cmc_dma_acpi_match);
    static struct platform_driver loongson2_cmc_dma_driver = {
    .driver = {
    .name = "loongson2-apb-cmc-dma",
    .of_match_table = loongson2_cmc_dma_of_match,
    .acpi_match_table = loongson2_cmc_dma_acpi_match,
    },
    .probe = loongson2_cmc_dma_probe,
    .remove = loongson2_cmc_dma_remove,
    };
    module_platform_driver(loongson2_cmc_dma_driver);
    MODULE_DESCRIPTION("Loongson-2 Chain Multi-Channel DMA Controller driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
