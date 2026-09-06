//! Automatically rewritten from C to Rust
//! Source: drivers/dma/milbeaut-xdmac.c
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
// Copyright (C) 2019 Linaro Ltd.
// Copyright (C) 2019 Socionext Inc.

// global register
pub const M10V_XDACS: c_uint = 0x00;
// channel local register
pub const M10V_XDTBC: c_uint = 0x10;
pub const M10V_XDSSA: c_uint = 0x14;
pub const M10V_XDDSA: c_uint = 0x18;
pub const M10V_XDSAC: c_uint = 0x1C;
pub const M10V_XDDAC: c_uint = 0x20;
pub const M10V_XDDCC: c_uint = 0x24;
pub const M10V_XDDES: c_uint = 0x28;
pub const M10V_XDDPC: c_uint = 0x2C;
pub const M10V_XDDSD: c_uint = 0x30;

pub const M10V_DEFBS: c_uint = 0x3;
pub const M10V_DEFBL: c_uint = 0xf;

pub const M10V_XDDSD_IS_NORMAL: c_uint = 0x8;

    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES) | \
    BIT(DMA_SLAVE_BUSWIDTH_8_BYTES))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct milbeaut_xdmac_desc {
    pub vd: virt_dma_desc,
    pub len: usize,
    pub src: dma_addr_t,
    pub dst: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct milbeaut_xdmac_chan {
    pub vc: virt_dma_chan,
    pub md: *mut milbeaut_xdmac_desc,
    pub reg_ch_base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct milbeaut_xdmac_device {
    pub ddev: dma_device,
    pub reg_base: *mut void __iomem,
    pub channels: [milbeaut_xdmac_chan; ],
}

    static struct milbeaut_xdmac_chan *
    to_milbeaut_xdmac_chan(struct virt_dma_chan *vc)
    {
    return container_of(vc, struct milbeaut_xdmac_chan, vc);
    }
    static struct milbeaut_xdmac_desc *
    to_milbeaut_xdmac_desc(struct virt_dma_desc *vd)
    {
    return container_of(vd, struct milbeaut_xdmac_desc, vd);
    }
// mc->vc.lock must be held by caller
    static struct milbeaut_xdmac_desc *
    milbeaut_xdmac_next_desc(struct milbeaut_xdmac_chan *mc)
    {
    struct virt_dma_desc *vd;
    vd = vchan_next_desc(&mc.vc);
    if (!vd) {
    mc.md = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    list_del(&vd.node);
    mc.md = to_milbeaut_xdmac_desc(vd);
    return mc.md;
    }
// mc->vc.lock must be held by caller
    static void milbeaut_chan_start(struct milbeaut_xdmac_chan *mc,
    struct milbeaut_xdmac_desc *md)
    {
    u32 val;
// Setup the channel
    val = md.len - 1;
    writel_relaxed(val, mc.reg_ch_base + M10V_XDTBC);
    val = md.src;
    writel_relaxed(val, mc.reg_ch_base + M10V_XDSSA);
    val = md.dst;
    writel_relaxed(val, mc.reg_ch_base + M10V_XDDSA);
    val = readl_relaxed(mc.reg_ch_base + M10V_XDSAC);
    val &= ~(M10V_XDSAC_SBS | M10V_XDSAC_SBL);
    val |= FIELD_PREP(M10V_XDSAC_SBS, M10V_DEFBS) |
    FIELD_PREP(M10V_XDSAC_SBL, M10V_DEFBL);
    writel_relaxed(val, mc.reg_ch_base + M10V_XDSAC);
    val = readl_relaxed(mc.reg_ch_base + M10V_XDDAC);
    val &= ~(M10V_XDDAC_DBS | M10V_XDDAC_DBL);
    val |= FIELD_PREP(M10V_XDDAC_DBS, M10V_DEFBS) |
    FIELD_PREP(M10V_XDDAC_DBL, M10V_DEFBL);
    writel_relaxed(val, mc.reg_ch_base + M10V_XDDAC);
// Start the channel
    val = readl_relaxed(mc.reg_ch_base + M10V_XDDES);
    val &= ~(M10V_XDDES_CE | M10V_XDDES_SE | M10V_XDDES_TF |
    M10V_XDDES_EI | M10V_XDDES_TI);
    val |= FIELD_PREP(M10V_XDDES_CE, 1) | FIELD_PREP(M10V_XDDES_SE, 1) |
    FIELD_PREP(M10V_XDDES_TF, 1) | FIELD_PREP(M10V_XDDES_EI, 1) |
    FIELD_PREP(M10V_XDDES_TI, 1);
    writel_relaxed(val, mc.reg_ch_base + M10V_XDDES);
    }
// mc->vc.lock must be held by caller
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_start(mc: *mut milbeaut_xdmac_chan) {
    static void milbeaut_xdmac_start(struct milbeaut_xdmac_chan *mc)
    {
    struct milbeaut_xdmac_desc *md;
    md = milbeaut_xdmac_next_desc(mc);
    if (md)
    milbeaut_chan_start(mc, md);
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t milbeaut_xdmac_interrupt(int irq, void *dev_id)
    {
    struct milbeaut_xdmac_chan *mc = dev_id;
    struct milbeaut_xdmac_desc *md;
    u32 val;
    spin_lock(&mc.vc.lock);
// Ack and Stop
    val = FIELD_PREP(M10V_XDDSD_IS_MASK, 0x0);
    writel_relaxed(val, mc.reg_ch_base + M10V_XDDSD);
    md = mc.md;
    if (!md)
    goto out;
    vchan_cookie_complete(&md.vd);
    milbeaut_xdmac_start(mc);
    out:
    spin_unlock(&mc.vc.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_free_chan_resources(chan: *mut dma_chan) {
    static void milbeaut_xdmac_free_chan_resources(struct dma_chan *chan)
    {
    vchan_free_chan_resources(to_virt_chan(chan));
    }
    static struct dma_async_tx_descriptor *
    milbeaut_xdmac_prep_memcpy(struct dma_chan *chan, dma_addr_t dst,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct milbeaut_xdmac_desc *md;
    md = kzalloc_obj(*md, GFP_NOWAIT);
    if (!md)
    return core::ptr::null_mut();
    md.len = len;
    md.src = src;
    md.dst = dst;
    return vchan_tx_prep(vc, &md.vd, flags);
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_terminate_all(chan: *mut dma_chan) -> c_int {
    static int milbeaut_xdmac_terminate_all(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct milbeaut_xdmac_chan *mc = to_milbeaut_xdmac_chan(vc);
    unsigned long flags;
    u32 val;
    LIST_HEAD(head);
    spin_lock_irqsave(&vc.lock, flags);
// Halt the channel
    val = readl(mc.reg_ch_base + M10V_XDDES);
    val &= ~M10V_XDDES_CE;
    val |= FIELD_PREP(M10V_XDDES_CE, 0);
    writel(val, mc.reg_ch_base + M10V_XDDES);
    if (mc.md) {
    vchan_terminate_vdesc(&mc.md.vd);
    mc.md = core::ptr::null_mut();
    }
    vchan_get_all_descriptors(vc, &head);
    spin_unlock_irqrestore(&vc.lock, flags);
    vchan_dma_desc_free_list(vc, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_synchronize(chan: *mut dma_chan) {
    static void milbeaut_xdmac_synchronize(struct dma_chan *chan)
    {
    vchan_synchronize(to_virt_chan(chan));
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_issue_pending(chan: *mut dma_chan) {
    static void milbeaut_xdmac_issue_pending(struct dma_chan *chan)
    {
    struct virt_dma_chan *vc = to_virt_chan(chan);
    struct milbeaut_xdmac_chan *mc = to_milbeaut_xdmac_chan(vc);
    unsigned long flags;
    spin_lock_irqsave(&vc.lock, flags);
    if (vchan_issue_pending(vc) && !mc.md)
    milbeaut_xdmac_start(mc);
    spin_unlock_irqrestore(&vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_desc_free(vd: *mut virt_dma_desc) {
    static void milbeaut_xdmac_desc_free(struct virt_dma_desc *vd)
    {
    kfree(to_milbeaut_xdmac_desc(vd));
    }
    static int milbeaut_xdmac_chan_init(struct platform_device *pdev,
    struct milbeaut_xdmac_device *mdev,
    int chan_id)
    {
    struct device *dev = &pdev.dev;
    struct milbeaut_xdmac_chan *mc = &mdev.channels[chan_id];
    char *irq_name;
    int irq, ret;
    irq = platform_get_irq(pdev, chan_id);
    if (irq < 0)
    return irq;
    irq_name = devm_kasprintf(dev, GFP_KERNEL, "milbeaut-xdmac-%d",
    chan_id);
    if (!irq_name)
    return -ENOMEM;
    ret = devm_request_irq(dev, irq, milbeaut_xdmac_interrupt,
    IRQF_SHARED, irq_name, mc);
    if (ret)
    return ret;
    mc.reg_ch_base = mdev.reg_base + chan_id * 0x30;
    mc.vc.desc_free = milbeaut_xdmac_desc_free;
    vchan_init(&mc.vc, &mdev.ddev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_xdmac(mdev: *mut milbeaut_xdmac_device) {
    static void enable_xdmac(struct milbeaut_xdmac_device *mdev)
    {
    unsigned int val;
    val = readl(mdev.reg_base + M10V_XDACS);
    val |= M10V_XDACS_XE;
    writel(val, mdev.reg_base + M10V_XDACS);
    }
#[no_mangle]
unsafe extern "C" fn disable_xdmac(mdev: *mut milbeaut_xdmac_device) {
    static void disable_xdmac(struct milbeaut_xdmac_device *mdev)
    {
    unsigned int val;
    val = readl(mdev.reg_base + M10V_XDACS);
    val &= ~M10V_XDACS_XE;
    writel(val, mdev.reg_base + M10V_XDACS);
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_probe(pdev: *mut platform_device) -> c_int {
    static int milbeaut_xdmac_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct milbeaut_xdmac_device *mdev;
    struct dma_device *ddev;
    int nr_chans, ret, i;
    nr_chans = platform_irq_count(pdev);
    if (nr_chans < 0)
    return nr_chans;
    mdev = devm_kzalloc(dev, struct_size(mdev, channels, nr_chans),
    GFP_KERNEL);
    if (!mdev)
    return -ENOMEM;
    mdev.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mdev.reg_base))
    return PTR_ERR(mdev.reg_base);
    ddev = &mdev.ddev;
    ddev.dev = dev;
    dma_cap_set(DMA_MEMCPY, ddev.cap_mask);
    ddev.src_addr_widths = MLB_XDMAC_BUSWIDTHS;
    ddev.dst_addr_widths = MLB_XDMAC_BUSWIDTHS;
    ddev.device_free_chan_resources = milbeaut_xdmac_free_chan_resources;
    ddev.device_prep_dma_memcpy = milbeaut_xdmac_prep_memcpy;
    ddev.device_terminate_all = milbeaut_xdmac_terminate_all;
    ddev.device_synchronize = milbeaut_xdmac_synchronize;
    ddev.device_tx_status = dma_cookie_status;
    ddev.device_issue_pending = milbeaut_xdmac_issue_pending;
    INIT_LIST_HEAD(&ddev.channels);
    for (i = 0; i < nr_chans; i++) {
    ret = milbeaut_xdmac_chan_init(pdev, mdev, i);
    if (ret)
    return ret;
    }
    enable_xdmac(mdev);
    ret = dma_async_device_register(ddev);
    if (ret)
    goto disable_xdmac;
    ret = of_dma_controller_register(dev.of_node,
    of_dma_simple_xlate, mdev);
    if (ret)
    goto unregister_dmac;
    platform_set_drvdata(pdev, mdev);
    return 0;
    unregister_dmac:
    dma_async_device_unregister(ddev);
    disable_xdmac:
    disable_xdmac(mdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn milbeaut_xdmac_remove(pdev: *mut platform_device) {
    static void milbeaut_xdmac_remove(struct platform_device *pdev)
    {
    struct milbeaut_xdmac_device *mdev = platform_get_drvdata(pdev);
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
    milbeaut_xdmac_free_chan_resources(chan);
    }
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&mdev.ddev);
    disable_xdmac(mdev);
    }
    static const struct of_device_id milbeaut_xdmac_match[] = {
    { .compatible = "socionext,milbeaut-m10v-xdmac" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, milbeaut_xdmac_match);
    static struct platform_driver milbeaut_xdmac_driver = {
    .probe = milbeaut_xdmac_probe,
    .remove = milbeaut_xdmac_remove,
    .driver = {
    .name = "milbeaut-m10v-xdmac",
    .of_match_table = milbeaut_xdmac_match,
    },
    };
    module_platform_driver(milbeaut_xdmac_driver);
    MODULE_DESCRIPTION("Milbeaut XDMAC DmaEngine driver");
    MODULE_LICENSE("GPL v2");
