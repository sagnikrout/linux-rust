//! Automatically rewritten from C to Rust
//! Source: drivers/dma/uniphier-mdmac.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>

// registers common for all channels
pub const UNIPHIER_MDMAC_CMD: c_uint = 0x000	/* issue DMA start/abort */;

// per-channel registers
pub const UNIPHIER_MDMAC_CH_OFFSET: c_uint = 0x100;
pub const UNIPHIER_MDMAC_CH_STRIDE: c_uint = 0x040;
pub const UNIPHIER_MDMAC_CH_IRQ_STAT: c_uint = 0x010	/* current hw status (RO) */;
pub const UNIPHIER_MDMAC_CH_IRQ_REQ: c_uint = 0x014	/* latched STAT (WOC) */;
pub const UNIPHIER_MDMAC_CH_IRQ_EN: c_uint = 0x018	/* IRQ enable mask */;
pub const UNIPHIER_MDMAC_CH_IRQ_DET: c_uint = 0x01c	/* REQ & EN (RO) */;

pub const UNIPHIER_MDMAC_CH_SRC_MODE: c_uint = 0x020	/* mode of source */;
pub const UNIPHIER_MDMAC_CH_DEST_MODE: c_uint = 0x024	/* mode of destination */;

pub const UNIPHIER_MDMAC_CH_SRC_ADDR: c_uint = 0x028	/* source address */;
pub const UNIPHIER_MDMAC_CH_DEST_ADDR: c_uint = 0x02c	/* destination address */;
pub const UNIPHIER_MDMAC_CH_SIZE: c_uint = 0x030	/* transfer bytes */;

    (BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) | \
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_3_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_mdmac_desc {
    pub vd: virt_dma_desc,
    pub sgl: *mut scatterlist,
    pub sg_len: c_uint,
    pub sg_cur: c_uint,
    pub dir: enum dma_transfer_direction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_mdmac_chan {
    pub vc: virt_dma_chan,
    pub mdev: *mut uniphier_mdmac_device,
    pub md: *mut uniphier_mdmac_desc,
    pub reg_ch_base: *mut void __iomem,
    pub chan_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_mdmac_device {
    pub ddev: dma_device,
    pub clk: *mut clk,
    pub reg_base: *mut void __iomem,
    pub channels: [uniphier_mdmac_chan; ],
}

    static struct uniphier_mdmac_chan *
    to_uniphier_mdmac_chan(struct virt_dma_chan *vc)
    {
    return container_of(vc, struct uniphier_mdmac_chan, vc);
    }
    static struct uniphier_mdmac_desc *
    to_uniphier_mdmac_desc(struct virt_dma_desc *vd)
    {
    return container_of(vd, struct uniphier_mdmac_desc, vd);
    }
// mc->vc.lock must be held by caller
    static struct uniphier_mdmac_desc *
    uniphier_mdmac_next_desc(struct uniphier_mdmac_chan *mc)
    {
    struct virt_dma_desc *vd;
    vd = vchan_next_desc(&mc.vc);
    if (!vd) {
    mc.md = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    list_del(&vd.node);
    mc.md = to_uniphier_mdmac_desc(vd);
    return mc.md;
    }
// mc->vc.lock must be held by caller
    static void uniphier_mdmac_handle(struct uniphier_mdmac_chan *mc,
    struct uniphier_mdmac_desc *md)
    {
    struct uniphier_mdmac_device *mdev = mc.mdev;
    struct scatterlist *sg;
    let mut irq_flag: u32 = UNIPHIER_MDMAC_CH_IRQ__DONE;
    u32 src_mode, src_addr, dest_mode, dest_addr, chunk_size;
    sg = &md.sgl[md.sg_cur];
    if (md.dir == DMA_MEM_TO_DEV) {
    src_mode = UNIPHIER_MDMAC_CH_MODE__ADDR_INC;
    src_addr = sg_dma_address(sg);
    dest_mode = UNIPHIER_MDMAC_CH_MODE__ADDR_FIXED;
    dest_addr = 0;
    } else {
    src_mode = UNIPHIER_MDMAC_CH_MODE__ADDR_FIXED;
    src_addr = 0;
    dest_mode = UNIPHIER_MDMAC_CH_MODE__ADDR_INC;
    dest_addr = sg_dma_address(sg);
    }
    chunk_size = sg_dma_len(sg);
    writel(src_mode, mc.reg_ch_base + UNIPHIER_MDMAC_CH_SRC_MODE);
    writel(dest_mode, mc.reg_ch_base + UNIPHIER_MDMAC_CH_DEST_MODE);
    writel(src_addr, mc.reg_ch_base + UNIPHIER_MDMAC_CH_SRC_ADDR);
    writel(dest_addr, mc.reg_ch_base + UNIPHIER_MDMAC_CH_DEST_ADDR);
    writel(chunk_size, mc.reg_ch_base + UNIPHIER_MDMAC_CH_SIZE);
// write 1 to clear
    writel(irq_flag, mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_REQ);
    writel(irq_flag, mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_EN);
    writel(BIT(mc.chan_id), mdev.reg_base + UNIPHIER_MDMAC_CMD);
    }
// mc->vc.lock must be held by caller
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_start(mc: *mut uniphier_mdmac_chan) {
    static void uniphier_mdmac_start(struct uniphier_mdmac_chan *mc)
    {
    struct uniphier_mdmac_desc *md;
    md = uniphier_mdmac_next_desc(mc);
    if (md)
    uniphier_mdmac_handle(mc, md);
    }
// mc->vc.lock must be held by caller
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_abort(mc: *mut uniphier_mdmac_chan) -> c_int {
    static int uniphier_mdmac_abort(struct uniphier_mdmac_chan *mc)
    {
    struct uniphier_mdmac_device *mdev = mc.mdev;
    let mut irq_flag: u32 = UNIPHIER_MDMAC_CH_IRQ__ABORT;
    u32 val;
// write 1 to clear
    writel(irq_flag, mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_REQ);
    writel(UNIPHIER_MDMAC_CMD_ABORT | BIT(mc.chan_id),
    mdev.reg_base + UNIPHIER_MDMAC_CMD);
//
// Abort should be accepted soon. We poll the bit here instead of
// waiting for the interrupt.
//
    return readl_poll_timeout(mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_REQ,
    val, val & irq_flag, 0, 20);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_mdmac_interrupt(int irq, void *dev_id)
    {
    struct uniphier_mdmac_chan *mc = dev_id;
    struct uniphier_mdmac_desc *md;
    let mut ret: irqreturn_t = IRQ_HANDLED;
    u32 irq_stat;
    spin_lock(&mc.vc.lock);
    irq_stat = readl(mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_DET);
//
// Some channels share a single interrupt line. If the IRQ status is 0,
// this is probably triggered by a different channel.
//
    if (!irq_stat) {
    ret = IRQ_NONE;
    goto out;
    }
// write 1 to clear
    writel(irq_stat, mc.reg_ch_base + UNIPHIER_MDMAC_CH_IRQ_REQ);
//
// UNIPHIER_MDMAC_CH_IRQ__DONE interrupt is asserted even when the DMA
// is aborted. To distinguish the normal completion and the abort,
// check mc->md. If it is NULL, we are aborting.
//
    md = mc.md;
    if (!md)
    goto out;
    md.sg_cur++;
    if (md.sg_cur >= md.sg_len) {
    vchan_cookie_complete(&md.vd);
    md = uniphier_mdmac_next_desc(mc);
    if (!md)
    goto out;
    }
    uniphier_mdmac_handle(mc, md);
    out:
    spin_unlock(&mc.vc.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_free_chan_resources(chan: *mut dma_chan) {
    static void uniphier_mdmac_free_chan_resources(struct dma_chan *chan)
    {
    vchan_free_chan_resources(to_virt_chan(chan));
    }
    static struct dma_async_tx_descriptor *
    uniphier_mdmac_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len,
    enum dma_transfer_direction direction,
    unsigned long flags, void *context)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_mdmac_desc *md;
    if (!is_slave_direction(direction))
    return core::ptr::null_mut();
    md = kzalloc_obj(*md, GFP_NOWAIT);
    if (!md)
    return core::ptr::null_mut();
    md.sgl = sgl;
    md.sg_len = sg_len;
    md.dir = direction;
    return vchan_tx_prep(vc, &md.vd, flags);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_terminate_all(chan: *mut dma_chan) -> c_int {
    static int uniphier_mdmac_terminate_all(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_mdmac_chan *mc = to_uniphier_mdmac_chan(vc);
    unsigned long flags;
    let mut ret: c_int = 0;
    LIST_HEAD(head);
    spin_lock_irqsave(&vc.lock, flags);
    if (mc.md) {
    vchan_terminate_vdesc(&mc.md.vd);
    mc.md = core::ptr::null_mut();
    ret = uniphier_mdmac_abort(mc);
    }
    vchan_get_all_descriptors(vc, &head);
    spin_unlock_irqrestore(&vc.lock, flags);
    vchan_dma_desc_free_list(vc, &head);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_synchronize(chan: *mut dma_chan) {
    static void uniphier_mdmac_synchronize(struct dma_chan *chan)
    {
    vchan_synchronize(to_virt_chan(chan));
    }
    static enum dma_status uniphier_mdmac_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    struct virt_dma_chan *vc;
    struct virt_dma_desc *vd;
    struct uniphier_mdmac_chan *mc;
    struct uniphier_mdmac_desc *md = core::ptr::null_mut();
    enum dma_status stat;
    unsigned long flags;
    int i;
    stat = dma_cookie_status(chan, cookie, txstate);
// Return immediately if we do not need to compute the residue.
    if (stat == DMA_COMPLETE || !txstate)
    return stat;
    vc = to_virt_chan(chan);
    spin_lock_irqsave(&vc.lock, flags);
    mc = to_uniphier_mdmac_chan(vc);
    if (mc.md && mc.md.vd.tx.cookie == cookie) {
// residue from the on-flight chunk
    txstate.residue = readl(mc.reg_ch_base +
    UNIPHIER_MDMAC_CH_SIZE);
    md = mc.md;
    }
    if (!md) {
    vd = vchan_find_desc(vc, cookie);
    if (vd)
    md = to_uniphier_mdmac_desc(vd);
    }
    if (md) {
// residue from the queued chunks
    for (i = md.sg_cur; i < md.sg_len; i++)
    txstate.residue += sg_dma_len(&md.sgl[i]);
    }
    spin_unlock_irqrestore(&vc.lock, flags);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_issue_pending(chan: *mut dma_chan) {
    static void uniphier_mdmac_issue_pending(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_mdmac_chan *mc = to_uniphier_mdmac_chan(vc);
    unsigned long flags;
    spin_lock_irqsave(&vc.lock, flags);
    if (vchan_issue_pending(vc) && !mc.md)
    uniphier_mdmac_start(mc);
    spin_unlock_irqrestore(&vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_desc_free(vd: *mut virt_dma_desc) {
    static void uniphier_mdmac_desc_free(struct virt_dma_desc *vd)
    {
    kfree(to_uniphier_mdmac_desc(vd));
    }
    static int uniphier_mdmac_chan_init(struct platform_device *pdev,
    struct uniphier_mdmac_device *mdev,
    int chan_id)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_mdmac_chan *mc = &mdev.channels[chan_id];
    char *irq_name;
    int irq, ret;
    irq = platform_get_irq(pdev, chan_id);
    if (irq < 0)
    return irq;
    irq_name = devm_kasprintf(dev, GFP_KERNEL, "uniphier-mio-dmac-ch%d",
    chan_id);
    if (!irq_name)
    return -ENOMEM;
    ret = devm_request_irq(dev, irq, uniphier_mdmac_interrupt,
    IRQF_SHARED, irq_name, mc);
    if (ret)
    return ret;
    mc.mdev = mdev;
    mc.reg_ch_base = mdev.reg_base + UNIPHIER_MDMAC_CH_OFFSET +
    UNIPHIER_MDMAC_CH_STRIDE * chan_id;
    mc.chan_id = chan_id;
    mc.vc.desc_free = uniphier_mdmac_desc_free;
    vchan_init(&mc.vc, &mdev.ddev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_mdmac_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_mdmac_device *mdev;
    struct dma_device *ddev;
    int nr_chans, ret, i;
    nr_chans = platform_irq_count(pdev);
    if (nr_chans < 0)
    return nr_chans;
    ret = dma_set_mask(dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
    mdev = devm_kzalloc(dev, struct_size(mdev, channels, nr_chans),
    GFP_KERNEL);
    if (!mdev)
    return -ENOMEM;
    mdev.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mdev.reg_base))
    return PTR_ERR(mdev.reg_base);
    mdev.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(mdev.clk)) {
    dev_err(dev, "failed to get clock\n");
    return PTR_ERR(mdev.clk);
    }
    ret = clk_prepare_enable(mdev.clk);
    if (ret)
    return ret;
    ddev = &mdev.ddev;
    ddev.dev = dev;
    dma_cap_set(DMA_PRIVATE, ddev.cap_mask);
    ddev.src_addr_widths = UNIPHIER_MDMAC_SLAVE_BUSWIDTHS;
    ddev.dst_addr_widths = UNIPHIER_MDMAC_SLAVE_BUSWIDTHS;
    ddev.directions = BIT(DMA_MEM_TO_DEV) | BIT(DMA_DEV_TO_MEM);
    ddev.residue_granularity = DMA_RESIDUE_GRANULARITY_SEGMENT;
    ddev.device_free_chan_resources = uniphier_mdmac_free_chan_resources;
    ddev.device_prep_slave_sg = uniphier_mdmac_prep_slave_sg;
    ddev.device_terminate_all = uniphier_mdmac_terminate_all;
    ddev.device_synchronize = uniphier_mdmac_synchronize;
    ddev.device_tx_status = uniphier_mdmac_tx_status;
    ddev.device_issue_pending = uniphier_mdmac_issue_pending;
    INIT_LIST_HEAD(&ddev.channels);
    for (i = 0; i < nr_chans; i++) {
    ret = uniphier_mdmac_chan_init(pdev, mdev, i);
    if (ret)
    goto disable_clk;
    }
    ret = dma_async_device_register(ddev);
    if (ret)
    goto disable_clk;
    ret = of_dma_controller_register(dev.of_node, of_dma_xlate_by_chan_id,
    ddev);
    if (ret)
    goto unregister_dmac;
    platform_set_drvdata(pdev, mdev);
    return 0;
    unregister_dmac:
    dma_async_device_unregister(ddev);
    disable_clk:
    clk_disable_unprepare(mdev.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_mdmac_remove(pdev: *mut platform_device) {
    static void uniphier_mdmac_remove(struct platform_device *pdev)
    {
    struct uniphier_mdmac_device *mdev = platform_get_drvdata(pdev);
    struct dma_chan *chan;
    int ret;
//
// Before reaching here, almost all descriptors have been freed by the
// ->device_free_chan_resources() hook. However, each channel might
// be still holding one descriptor that was on-flight at that moment.
// Terminate it to make sure this hardware is no longer running. Then,
// free the channel resources once again to avoid memory leak.
//
    list_for_each_entry(chan, &mdev.ddev.channels, device_node) {
    ret = dmaengine_terminate_sync(chan);
    if (ret) {
//
// This results in resource leakage and maybe also
// use-after-free errors as e.g. *mdev is kfreed.
//
    dev_alert(&pdev.dev, "Failed to terminate channel %d (%pe)\n",
    chan.chan_id, ERR_PTR(ret));
    return;
    }
    uniphier_mdmac_free_chan_resources(chan);
    }
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&mdev.ddev);
    clk_disable_unprepare(mdev.clk);
    }
    static const struct of_device_id uniphier_mdmac_match[] = {
    { .compatible = "socionext,uniphier-mio-dmac" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_mdmac_match);
    static struct platform_driver uniphier_mdmac_driver = {
    .probe = uniphier_mdmac_probe,
    .remove = uniphier_mdmac_remove,
    .driver = {
    .name = "uniphier-mio-dmac",
    .of_match_table = uniphier_mdmac_match,
    },
    };
    module_platform_driver(uniphier_mdmac_driver);
    MODULE_AUTHOR("Masahiro Yamada <yamada.masahiro@socionext.com>");
    MODULE_DESCRIPTION("UniPhier MIO DMAC driver");
    MODULE_LICENSE("GPL v2");
