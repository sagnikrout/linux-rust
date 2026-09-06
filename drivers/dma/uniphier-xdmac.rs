//! Automatically rewritten from C to Rust
//! Source: drivers/dma/uniphier-xdmac.c
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
// External DMA controller driver for UniPhier SoCs
// Copyright 2019 Socionext Inc.
// Author: Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

pub const XDMAC_CH_WIDTH: c_uint = 0x100;
pub const XDMAC_TFA: c_uint = 0x08;

pub const XDMAC_SADM: c_uint = 0x10;

pub const XDMAC_SADM_SAM_INC: c_int = 0;
pub const XDMAC_DADM: c_uint = 0x14;

pub const XDMAC_EXSAD: c_uint = 0x18;
pub const XDMAC_EXDAD: c_uint = 0x1c;
pub const XDMAC_SAD: c_uint = 0x20;
pub const XDMAC_DAD: c_uint = 0x24;
pub const XDMAC_ITS: c_uint = 0x28;

pub const XDMAC_TNUM: c_uint = 0x2c;

pub const XDMAC_TSS: c_uint = 0x30;

pub const XDMAC_IEN: c_uint = 0x34;

pub const XDMAC_STAT: c_uint = 0x40;

pub const XDMAC_IR: c_uint = 0x44;

pub const XDMAC_ID: c_uint = 0x48;

pub const XDMAC_MAX_CHANS: c_int = 16;
pub const XDMAC_INTERVAL_CLKS: c_int = 20;

// cut lower bit for maintain alignment of maximum transfer size

    (BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) | \
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_8_BYTES))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_xdmac_desc_node {
    pub src: dma_addr_t,
    pub dst: dma_addr_t,
    pub burst_size: u32,
    pub nr_burst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_xdmac_desc {
    pub vd: virt_dma_desc,
    pub nr_node: c_uint,
    pub cur_node: c_uint,
    pub dir: enum dma_transfer_direction,
    pub __counted_by(nr_node): uniphier_xdmac_desc_node nodes[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_xdmac_chan {
    pub vc: virt_dma_chan,
    pub xdev: *mut uniphier_xdmac_device,
    pub xd: *mut uniphier_xdmac_desc,
    pub reg_ch_base: *mut void __iomem,
    pub sconfig: dma_slave_config,
    pub id: c_int,
    pub req_factor: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_xdmac_device {
    pub ddev: dma_device,
    pub reg_base: *mut void __iomem,
    pub nr_chans: c_int,
    pub __counted_by(nr_chans): uniphier_xdmac_chan channels[],
}

    static struct uniphier_xdmac_chan *
    to_uniphier_xdmac_chan(struct virt_dma_chan *vc)
    {
    return container_of(vc, struct uniphier_xdmac_chan, vc);
    }
    static struct uniphier_xdmac_desc *
    to_uniphier_xdmac_desc(struct virt_dma_desc *vd)
    {
    return container_of(vd, struct uniphier_xdmac_desc, vd);
    }
// xc->vc.lock must be held by caller
    static struct uniphier_xdmac_desc *
    uniphier_xdmac_next_desc(struct uniphier_xdmac_chan *xc)
    {
    struct virt_dma_desc *vd;
    vd = vchan_next_desc(&xc.vc);
    if (!vd)
    return core::ptr::null_mut();
    list_del(&vd.node);
    return to_uniphier_xdmac_desc(vd);
    }
// xc->vc.lock must be held by caller
    static void uniphier_xdmac_chan_start(struct uniphier_xdmac_chan *xc,
    struct uniphier_xdmac_desc *xd)
    {
    u32 src_mode, src_width;
    u32 dst_mode, dst_width;
    dma_addr_t src_addr, dst_addr;
    u32 val, its, tnum;
    enum dma_slave_buswidth buswidth;
    src_addr = xd.nodes[xd.cur_node].src;
    dst_addr = xd.nodes[xd.cur_node].dst;
    its      = xd.nodes[xd.cur_node].burst_size;
    tnum     = xd.nodes[xd.cur_node].nr_burst;
//
// The width of MEM side must be 4 or 8 bytes, that does not
// affect that of DEV side and transfer size.
//
    if (xd.dir == DMA_DEV_TO_MEM) {
    src_mode = XDMAC_SADM_SAM_FIXED;
    buswidth = xc.sconfig.src_addr_width;
    } else {
    src_mode = XDMAC_SADM_SAM_INC;
    buswidth = DMA_SLAVE_BUSWIDTH_8_BYTES;
    }
    src_width = FIELD_PREP(XDMAC_SADM_STW_MASK, __ffs(buswidth));
    if (xd.dir == DMA_MEM_TO_DEV) {
    dst_mode = XDMAC_DADM_DAM_FIXED;
    buswidth = xc.sconfig.dst_addr_width;
    } else {
    dst_mode = XDMAC_DADM_DAM_INC;
    buswidth = DMA_SLAVE_BUSWIDTH_8_BYTES;
    }
    dst_width = FIELD_PREP(XDMAC_DADM_DTW_MASK, __ffs(buswidth));
// setup transfer factor
    val = FIELD_PREP(XDMAC_TFA_MCNT_MASK, XDMAC_INTERVAL_CLKS);
    val |= FIELD_PREP(XDMAC_TFA_MASK, xc.req_factor);
    writel(val, xc.reg_ch_base + XDMAC_TFA);
// setup the channel
    writel(lower_32_bits(src_addr), xc.reg_ch_base + XDMAC_SAD);
    writel(upper_32_bits(src_addr), xc.reg_ch_base + XDMAC_EXSAD);
    writel(lower_32_bits(dst_addr), xc.reg_ch_base + XDMAC_DAD);
    writel(upper_32_bits(dst_addr), xc.reg_ch_base + XDMAC_EXDAD);
    src_mode |= src_width;
    dst_mode |= dst_width;
    writel(src_mode, xc.reg_ch_base + XDMAC_SADM);
    writel(dst_mode, xc.reg_ch_base + XDMAC_DADM);
    writel(its, xc.reg_ch_base + XDMAC_ITS);
    writel(tnum, xc.reg_ch_base + XDMAC_TNUM);
// enable interrupt
    writel(XDMAC_IEN_ENDIEN | XDMAC_IEN_ERRIEN,
    xc.reg_ch_base + XDMAC_IEN);
// start XDMAC
    val = readl(xc.reg_ch_base + XDMAC_TSS);
    val |= XDMAC_TSS_REQ;
    writel(val, xc.reg_ch_base + XDMAC_TSS);
    }
// xc->vc.lock must be held by caller
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_chan_stop(xc: *mut uniphier_xdmac_chan) -> c_int {
    static int uniphier_xdmac_chan_stop(struct uniphier_xdmac_chan *xc)
    {
    u32 val;
// disable interrupt
    val = readl(xc.reg_ch_base + XDMAC_IEN);
    val &= ~(XDMAC_IEN_ENDIEN | XDMAC_IEN_ERRIEN);
    writel(val, xc.reg_ch_base + XDMAC_IEN);
// stop XDMAC
    val = readl(xc.reg_ch_base + XDMAC_TSS);
    val &= ~XDMAC_TSS_REQ;
    writel(0, xc.reg_ch_base + XDMAC_TSS);
// wait until transfer is stopped
    return readl_poll_timeout_atomic(xc.reg_ch_base + XDMAC_STAT, val,
    !(val & XDMAC_STAT_TENF), 100, 1000);
    }
// xc->vc.lock must be held by caller
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_start(xc: *mut uniphier_xdmac_chan) {
    static void uniphier_xdmac_start(struct uniphier_xdmac_chan *xc)
    {
    struct uniphier_xdmac_desc *xd;
    xd = uniphier_xdmac_next_desc(xc);
    if (xd)
    uniphier_xdmac_chan_start(xc, xd);
// set desc to chan regardless of xd is null
    xc.xd = xd;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_chan_irq(xc: *mut uniphier_xdmac_chan) {
    static void uniphier_xdmac_chan_irq(struct uniphier_xdmac_chan *xc)
    {
    u32 stat;
    int ret;
    spin_lock(&xc.vc.lock);
    stat = readl(xc.reg_ch_base + XDMAC_ID);
    if (stat & XDMAC_ID_ERRIDF) {
    ret = uniphier_xdmac_chan_stop(xc);
    if (ret)
    dev_err(xc.xdev.ddev.dev,
    "DMA transfer error with aborting issue\n");
    else
    dev_err(xc.xdev.ddev.dev,
    "DMA transfer error\n");
    } else if ((stat & XDMAC_ID_ENDIDF) && xc.xd) {
    xc.xd.cur_node++;
    if (xc.xd.cur_node >= xc.xd.nr_node) {
    vchan_cookie_complete(&xc.xd.vd);
    uniphier_xdmac_start(xc);
    } else {
    uniphier_xdmac_chan_start(xc, xc.xd);
    }
    }
// write bits to clear
    writel(stat, xc.reg_ch_base + XDMAC_IR);
    spin_unlock(&xc.vc.lock);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_xdmac_irq_handler(int irq, void *dev_id)
    {
    struct uniphier_xdmac_device *xdev = dev_id;
    int i;
    for (i = 0; i < xdev.nr_chans; i++)
    uniphier_xdmac_chan_irq(&xdev.channels[i]);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_free_chan_resources(chan: *mut dma_chan) {
    static void uniphier_xdmac_free_chan_resources(struct dma_chan *chan)
    {
    vchan_free_chan_resources(to_virt_chan(chan));
    }
    static struct dma_async_tx_descriptor *
    uniphier_xdmac_prep_dma_memcpy(struct dma_chan *chan, dma_addr_t dst,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_xdmac_desc *xd;
    unsigned int nr;
    size_t burst_size, tlen;
    int i;
    if (len > XDMAC_MAX_WORD_SIZE * XDMAC_MAX_WORDS)
    return core::ptr::null_mut();
    nr = 1 + len / XDMAC_MAX_WORD_SIZE;
    xd = kzalloc_flex(*xd, nodes, nr, GFP_NOWAIT);
    if (!xd)
    return core::ptr::null_mut();
    xd.nr_node = nr;
    for (i = 0; i < nr; i++) {
    burst_size = min_t(size_t, len, XDMAC_MAX_WORD_SIZE);
    xd.nodes[i].src = src;
    xd.nodes[i].dst = dst;
    xd.nodes[i].burst_size = burst_size;
    xd.nodes[i].nr_burst = len / burst_size;
    tlen = rounddown(len, burst_size);
    src += tlen;
    dst += tlen;
    len -= tlen;
    }
    xd.dir = DMA_MEM_TO_MEM;
    xd.cur_node = 0;
    return vchan_tx_prep(vc, &xd.vd, flags);
    }
    static struct dma_async_tx_descriptor *
    uniphier_xdmac_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len,
    enum dma_transfer_direction direction,
    unsigned long flags, void *context)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_xdmac_chan *xc = to_uniphier_xdmac_chan(vc);
    struct uniphier_xdmac_desc *xd;
    struct scatterlist *sg;
    enum dma_slave_buswidth buswidth;
    u32 maxburst;
    int i;
    if (!is_slave_direction(direction))
    return core::ptr::null_mut();
    if (direction == DMA_DEV_TO_MEM) {
    buswidth = xc.sconfig.src_addr_width;
    maxburst = xc.sconfig.src_maxburst;
    } else {
    buswidth = xc.sconfig.dst_addr_width;
    maxburst = xc.sconfig.dst_maxburst;
    }
    if (!maxburst)
    maxburst = 1;
    if (maxburst > xc.xdev.ddev.max_burst) {
    dev_err(xc.xdev.ddev.dev,
    "Exceed maximum number of burst words\n");
    return core::ptr::null_mut();
    }
    xd = kzalloc_flex(*xd, nodes, sg_len, GFP_NOWAIT);
    if (!xd)
    return core::ptr::null_mut();
    xd.nr_node = sg_len;
    for_each_sg(sgl, sg, sg_len, i) {
    xd.nodes[i].src = (direction == DMA_DEV_TO_MEM)
    ? xc.sconfig.src_addr : sg_dma_address(sg);
    xd.nodes[i].dst = (direction == DMA_MEM_TO_DEV)
    ? xc.sconfig.dst_addr : sg_dma_address(sg);
    xd.nodes[i].burst_size = maxburst * buswidth;
    xd.nodes[i].nr_burst =
    sg_dma_len(sg) / xd.nodes[i].burst_size;
//
// Currently transfer that size doesn't align the unit size
// (the number of burst words * bus-width) is not allowed,
// because the driver does not support the way to transfer
// residue size. As a matter of fact, in order to transfer
// arbitrary size, 'src_maxburst' or 'dst_maxburst' of
// dma_slave_config must be 1.
//
    if (sg_dma_len(sg) % xd.nodes[i].burst_size) {
    dev_err(xc.xdev.ddev.dev,
    "Unaligned transfer size: %d", sg_dma_len(sg));
    kfree(xd);
    return core::ptr::null_mut();
    }
    if (xd.nodes[i].nr_burst > XDMAC_MAX_WORDS) {
    dev_err(xc.xdev.ddev.dev,
    "Exceed maximum transfer size");
    kfree(xd);
    return core::ptr::null_mut();
    }
    }
    xd.dir = direction;
    xd.cur_node = 0;
    return vchan_tx_prep(vc, &xd.vd, flags);
    }
    static int uniphier_xdmac_slave_config(struct dma_chan *chan,
    struct dma_slave_config *config)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_xdmac_chan *xc = to_uniphier_xdmac_chan(vc);
    memcpy(&xc.sconfig, config, sizeof(*config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_terminate_all(chan: *mut dma_chan) -> c_int {
    static int uniphier_xdmac_terminate_all(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_xdmac_chan *xc = to_uniphier_xdmac_chan(vc);
    unsigned long flags;
    let mut ret: c_int = 0;
    LIST_HEAD(head);
    spin_lock_irqsave(&vc.lock, flags);
    if (xc.xd) {
    vchan_terminate_vdesc(&xc.xd.vd);
    xc.xd = core::ptr::null_mut();
    ret = uniphier_xdmac_chan_stop(xc);
    }
    vchan_get_all_descriptors(vc, &head);
    spin_unlock_irqrestore(&vc.lock, flags);
    vchan_dma_desc_free_list(vc, &head);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_synchronize(chan: *mut dma_chan) {
    static void uniphier_xdmac_synchronize(struct dma_chan *chan)
    {
    vchan_synchronize(to_virt_chan(chan));
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_issue_pending(chan: *mut dma_chan) {
    static void uniphier_xdmac_issue_pending(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct uniphier_xdmac_chan *xc = to_uniphier_xdmac_chan(vc);
    unsigned long flags;
    spin_lock_irqsave(&vc.lock, flags);
    if (vchan_issue_pending(vc) && !xc.xd)
    uniphier_xdmac_start(xc);
    spin_unlock_irqrestore(&vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_desc_free(vd: *mut virt_dma_desc) {
    static void uniphier_xdmac_desc_free(struct virt_dma_desc *vd)
    {
    kfree(to_uniphier_xdmac_desc(vd));
    }
    static void uniphier_xdmac_chan_init(struct uniphier_xdmac_device *xdev,
    int ch)
    {
    struct uniphier_xdmac_chan *xc = &xdev.channels[ch];
    xc.xdev = xdev;
    xc.reg_ch_base = xdev.reg_base + XDMAC_CH_WIDTH * ch;
    xc.vc.desc_free = uniphier_xdmac_desc_free;
    vchan_init(&xc.vc, &xdev.ddev);
    }
    static struct dma_chan *of_dma_uniphier_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct uniphier_xdmac_device *xdev = ofdma.of_dma_data;
    let mut chan_id: c_int = dma_spec.args[0];
    if (chan_id >= xdev.nr_chans)
    return core::ptr::null_mut();
    xdev.channels[chan_id].id = chan_id;
    xdev.channels[chan_id].req_factor = dma_spec.args[1];
    return dma_get_slave_channel(&xdev.channels[chan_id].vc.chan);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_xdmac_probe(struct platform_device *pdev)
    {
    struct uniphier_xdmac_device *xdev;
    struct device *dev = &pdev.dev;
    struct dma_device *ddev;
    int irq;
    int nr_chans;
    int i, ret;
    if (of_property_read_u32(dev.of_node, "dma-channels", &nr_chans))
    return -EINVAL;
    if (nr_chans > XDMAC_MAX_CHANS)
    nr_chans = XDMAC_MAX_CHANS;
    xdev = devm_kzalloc(dev, struct_size(xdev, channels, nr_chans),
    GFP_KERNEL);
    if (!xdev)
    return -ENOMEM;
    xdev.nr_chans = nr_chans;
    xdev.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xdev.reg_base))
    return PTR_ERR(xdev.reg_base);
    ddev = &xdev.ddev;
    ddev.dev = dev;
    dma_cap_zero(ddev.cap_mask);
    dma_cap_set(DMA_MEMCPY, ddev.cap_mask);
    dma_cap_set(DMA_SLAVE, ddev.cap_mask);
    ddev.src_addr_widths = UNIPHIER_XDMAC_BUSWIDTHS;
    ddev.dst_addr_widths = UNIPHIER_XDMAC_BUSWIDTHS;
    ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV) |
    BIT(DMA_MEM_TO_MEM);
    ddev.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    ddev.max_burst = XDMAC_MAX_WORDS;
    ddev.device_free_chan_resources = uniphier_xdmac_free_chan_resources;
    ddev.device_prep_dma_memcpy = uniphier_xdmac_prep_dma_memcpy;
    ddev.device_prep_slave_sg = uniphier_xdmac_prep_slave_sg;
    ddev.device_config = uniphier_xdmac_slave_config;
    ddev.device_terminate_all = uniphier_xdmac_terminate_all;
    ddev.device_synchronize = uniphier_xdmac_synchronize;
    ddev.device_tx_status = dma_cookie_status;
    ddev.device_issue_pending = uniphier_xdmac_issue_pending;
    INIT_LIST_HEAD(&ddev.channels);
    for (i = 0; i < nr_chans; i++)
    uniphier_xdmac_chan_init(xdev, i);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, uniphier_xdmac_irq_handler,
    IRQF_SHARED, "xdmac", xdev);
    if (ret) {
    dev_err(dev, "Failed to request IRQ\n");
    return ret;
    }
    ret = dma_async_device_register(ddev);
    if (ret) {
    dev_err(dev, "Failed to register XDMA device\n");
    return ret;
    }
    ret = of_dma_controller_register(dev.of_node,
    of_dma_uniphier_xlate, xdev);
    if (ret) {
    dev_err(dev, "Failed to register XDMA controller\n");
    goto out_unregister_dmac;
    }
    platform_set_drvdata(pdev, xdev);
    dev_info(&pdev.dev, "UniPhier XDMAC driver (%d channels)\n",
    nr_chans);
    return 0;
    out_unregister_dmac:
    dma_async_device_unregister(ddev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_xdmac_remove(pdev: *mut platform_device) {
    static void uniphier_xdmac_remove(struct platform_device *pdev)
    {
    struct uniphier_xdmac_device *xdev = platform_get_drvdata(pdev);
    struct dma_device *ddev = &xdev.ddev;
    struct dma_chan *chan;
    int ret;
//
// Before reaching here, almost all descriptors have been freed by the
// ->device_free_chan_resources() hook. However, each channel might
// be still holding one descriptor that was on-flight at that moment.
// Terminate it to make sure this hardware is no longer running. Then,
// free the channel resources once again to avoid memory leak.
//
    list_for_each_entry(chan, &ddev.channels, device_node) {
    ret = dmaengine_terminate_sync(chan);
    if (ret) {
//
// This results in resource leakage and maybe also
// use-after-free errors as e.g. *xdev is kfreed.
//
    dev_alert(&pdev.dev, "Failed to terminate channel %d (%pe)\n",
    chan.chan_id, ERR_PTR(ret));
    return;
    }
    uniphier_xdmac_free_chan_resources(chan);
    }
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(ddev);
    }
    static const struct of_device_id uniphier_xdmac_match[] = {
    { .compatible = "socionext,uniphier-xdmac" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_xdmac_match);
    static struct platform_driver uniphier_xdmac_driver = {
    .probe = uniphier_xdmac_probe,
    .remove = uniphier_xdmac_remove,
    .driver = {
    .name = "uniphier-xdmac",
    .of_match_table = uniphier_xdmac_match,
    },
    };
    module_platform_driver(uniphier_xdmac_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier external DMA controller driver");
    MODULE_LICENSE("GPL v2");
