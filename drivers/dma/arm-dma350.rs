//! Automatically rewritten from C to Rust
//! Source: drivers/dma/arm-dma350.c
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
// Copyright (C) 2024-2025 Arm Limited
// Arm DMA-350 driver

pub const DMANSECCTRL: c_uint = 0x200;
pub const NSEC_CTRL: c_uint = 0x0c;

pub const DMAINFO: c_uint = 0x0f00;
pub const DMA_BUILDCFG0: c_uint = 0xb0;

pub const DMA_BUILDCFG1: c_uint = 0xb4;

pub const IIDR: c_uint = 0xc8;

pub const PRODUCTID_DMA350: c_uint = 0x3a0;
pub const IMPLEMENTER_ARM: c_uint = 0x43b;

pub const CH_CMD: c_uint = 0x00;

pub const CH_STATUS: c_uint = 0x04;

pub const CH_INTREN: c_uint = 0x08;

pub const CH_CTRL: c_uint = 0x0c;

pub const CH_SRCADDR: c_uint = 0x10;
pub const CH_SRCADDRHI: c_uint = 0x14;
pub const CH_DESADDR: c_uint = 0x18;
pub const CH_DESADDRHI: c_uint = 0x1c;
pub const CH_XSIZE: c_uint = 0x20;
pub const CH_XSIZEHI: c_uint = 0x24;
pub const CH_SRCTRANSCFG: c_uint = 0x28;
pub const CH_DESTRANSCFG: c_uint = 0x2c;

    FIELD_PREP(CH_CFG_MAXBURSTLEN, 0xf) |		\
    FIELD_PREP(CH_CFG_SHAREATTR, SHAREATTR_OSH) |	\
    FIELD_PREP(CH_CFG_MEMATTR, MEMATTR_DEVICE)

    FIELD_PREP(CH_CFG_MAXBURSTLEN, 0xf) |		\
    FIELD_PREP(CH_CFG_SHAREATTR, SHAREATTR_OSH) |	\
    FIELD_PREP(CH_CFG_MEMATTR, MEMATTR_NC)

    FIELD_PREP(CH_CFG_MAXBURSTLEN, 0xf) |		\
    FIELD_PREP(CH_CFG_SHAREATTR, SHAREATTR_ISH) |	\
    FIELD_PREP(CH_CFG_MEMATTR, MEMATTR_WB)
pub const CH_XADDRINC: c_uint = 0x30;

pub const CH_FILLVAL: c_uint = 0x38;
pub const CH_SRCTRIGINCFG: c_uint = 0x4c;
pub const CH_DESTRIGINCFG: c_uint = 0x50;
pub const CH_LINKATTR: c_uint = 0x70;

pub const CH_AUTOCFG: c_uint = 0x74;
pub const CH_LINKADDR: c_uint = 0x78;

pub const CH_LINKADDRHI: c_uint = 0x7c;
pub const CH_ERRINFO: c_uint = 0x90;

pub const CH_BUILDCFG0: c_uint = 0xf8;

pub const CH_BUILDCFG1: c_uint = 0xfc;

    enum ch_ctrl_donetype {
    CH_CTRL_DONETYPE_NONE = 0,
    CH_CTRL_DONETYPE_CMD = 1,
    CH_CTRL_DONETYPE_CYCLE = 3
    };
    enum ch_ctrl_xtype {
    CH_CTRL_XTYPE_DISABLE = 0,
    CH_CTRL_XTYPE_CONTINUE = 1,
    CH_CTRL_XTYPE_WRAP = 2,
    CH_CTRL_XTYPE_FILL = 3
    };
    enum ch_cfg_shareattr {
    SHAREATTR_NSH = 0,
    SHAREATTR_OSH = 2,
    SHAREATTR_ISH = 3
    };
    enum ch_cfg_memattr {
    MEMATTR_DEVICE = 0x00,
    MEMATTR_NC = 0x44,
    MEMATTR_WB = 0xff
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d350_desc {
    pub vd: virt_dma_desc,
    pub command: [u32; 16],
    pub xsize: u16,
    pub xsizehi: u16,
    pub tsz: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d350_chan {
    pub vc: virt_dma_chan,
    pub desc: *mut d350_desc,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub status: enum dma_status,
    pub cookie: dma_cookie_t,
    pub residue: u32,
    pub tsz: u8,
    pub has_trig: bool,
    pub has_wrap: bool,
    pub coherent: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d350 {
    pub dma: dma_device,
    pub nchan: c_int,
    pub nreq: c_int,
    pub __counted_by(nchan): d350_chan channels[],
}

    static inline struct d350_chan *to_d350_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct d350_chan, vc.chan);
    }
    static inline struct d350_desc *to_d350_desc(struct virt_dma_desc *vd)
    {
    return container_of(vd, struct d350_desc, vd);
    }
#[no_mangle]
unsafe extern "C" fn d350_desc_free(vd: *mut virt_dma_desc) {
    static void d350_desc_free(struct virt_dma_desc *vd)
    {
    kfree(to_d350_desc(vd));
    }
    static struct dma_async_tx_descriptor *d350_prep_memcpy(struct dma_chan *chan,
    dma_addr_t dest, dma_addr_t src, size_t len, unsigned long flags)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    struct d350_desc *desc;
    u32 *cmd;
    desc = kzalloc_obj(*desc, GFP_NOWAIT);
    if (!desc)
    return core::ptr::null_mut();
    desc.tsz = __ffs(len | dest | src | (1 << dch.tsz));
    desc.xsize = lower_16_bits(len >> desc.tsz);
    desc.xsizehi = upper_16_bits(len >> desc.tsz);
    cmd = desc.command;
    cmd[0] = LINK_CTRL | LINK_SRCADDR | LINK_SRCADDRHI | LINK_DESADDR |
    LINK_DESADDRHI | LINK_XSIZE | LINK_XSIZEHI | LINK_SRCTRANSCFG |
    LINK_DESTRANSCFG | LINK_XADDRINC | LINK_LINKADDR;
    cmd[1] = FIELD_PREP(CH_CTRL_TRANSIZE, desc.tsz) |
    FIELD_PREP(CH_CTRL_XTYPE, CH_CTRL_XTYPE_CONTINUE) |
    FIELD_PREP(CH_CTRL_DONETYPE, CH_CTRL_DONETYPE_CMD);
    cmd[2] = lower_32_bits(src);
    cmd[3] = upper_32_bits(src);
    cmd[4] = lower_32_bits(dest);
    cmd[5] = upper_32_bits(dest);
    cmd[6] = FIELD_PREP(CH_XY_SRC, desc.xsize) | FIELD_PREP(CH_XY_DES, desc.xsize);
    cmd[7] = FIELD_PREP(CH_XY_SRC, desc.xsizehi) | FIELD_PREP(CH_XY_DES, desc.xsizehi);
    cmd[8] = dch.coherent ? TRANSCFG_WB : TRANSCFG_NC;
    cmd[9] = dch.coherent ? TRANSCFG_WB : TRANSCFG_NC;
    cmd[10] = FIELD_PREP(CH_XY_SRC, 1) | FIELD_PREP(CH_XY_DES, 1);
    cmd[11] = 0;
    return vchan_tx_prep(&dch.vc, &desc.vd, flags);
    }
    static struct dma_async_tx_descriptor *d350_prep_memset(struct dma_chan *chan,
    dma_addr_t dest, int value, size_t len, unsigned long flags)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    struct d350_desc *desc;
    u32 *cmd;
    desc = kzalloc_obj(*desc, GFP_NOWAIT);
    if (!desc)
    return core::ptr::null_mut();
    desc.tsz = __ffs(len | dest | (1 << dch.tsz));
    desc.xsize = lower_16_bits(len >> desc.tsz);
    desc.xsizehi = upper_16_bits(len >> desc.tsz);
    cmd = desc.command;
    cmd[0] = LINK_CTRL | LINK_DESADDR | LINK_DESADDRHI |
    LINK_XSIZE | LINK_XSIZEHI | LINK_DESTRANSCFG |
    LINK_XADDRINC | LINK_FILLVAL | LINK_LINKADDR;
    cmd[1] = FIELD_PREP(CH_CTRL_TRANSIZE, desc.tsz) |
    FIELD_PREP(CH_CTRL_XTYPE, CH_CTRL_XTYPE_FILL) |
    FIELD_PREP(CH_CTRL_DONETYPE, CH_CTRL_DONETYPE_CMD);
    cmd[2] = lower_32_bits(dest);
    cmd[3] = upper_32_bits(dest);
    cmd[4] = FIELD_PREP(CH_XY_DES, desc.xsize);
    cmd[5] = FIELD_PREP(CH_XY_DES, desc.xsizehi);
    cmd[6] = dch.coherent ? TRANSCFG_WB : TRANSCFG_NC;
    cmd[7] = FIELD_PREP(CH_XY_DES, 1);
    cmd[8] = (u8)value * 0x01010101;
    cmd[9] = 0;
    return vchan_tx_prep(&dch.vc, &desc.vd, flags);
    }
#[no_mangle]
unsafe extern "C" fn d350_pause(chan: *mut dma_chan) -> c_int {
    static int d350_pause(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&dch.vc.lock, flags);
    if (dch.status == DMA_IN_PROGRESS) {
    writel_relaxed(CH_CMD_PAUSE, dch.base + CH_CMD);
    dch.status = DMA_PAUSED;
    }
    spin_unlock_irqrestore(&dch.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d350_resume(chan: *mut dma_chan) -> c_int {
    static int d350_resume(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&dch.vc.lock, flags);
    if (dch.status == DMA_PAUSED) {
    writel_relaxed(CH_CMD_RESUME, dch.base + CH_CMD);
    dch.status = DMA_IN_PROGRESS;
    }
    spin_unlock_irqrestore(&dch.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d350_get_residue(dch: *mut d350_chan) -> u32 {
    static u32 d350_get_residue(struct d350_chan *dch)
    {
    u32 res, xsize, xsizehi, hi_new;
    int retries = 3; /* 1st time unlucky, 2nd improbable, 3rd just broken */
    hi_new = readl_relaxed(dch.base + CH_XSIZEHI);
    do {
    xsizehi = hi_new;
    xsize = readl_relaxed(dch.base + CH_XSIZE);
    hi_new = readl_relaxed(dch.base + CH_XSIZEHI);
    } while (xsizehi != hi_new && --retries);
    res = FIELD_GET(CH_XY_DES, xsize);
    res |= FIELD_GET(CH_XY_DES, xsizehi) << 16;
    return res << dch.desc.tsz;
    }
#[no_mangle]
unsafe extern "C" fn d350_terminate_all(chan: *mut dma_chan) -> c_int {
    static int d350_terminate_all(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    unsigned long flags;
    LIST_HEAD(list);
    spin_lock_irqsave(&dch.vc.lock, flags);
    writel_relaxed(CH_CMD_STOP, dch.base + CH_CMD);
    if (dch.desc) {
    if (dch.status != DMA_ERROR)
    vchan_terminate_vdesc(&dch.desc.vd);
    dch.desc = core::ptr::null_mut();
    dch.status = DMA_COMPLETE;
    }
    vchan_get_all_descriptors(&dch.vc, &list);
    list_splice_tail(&list, &dch.vc.desc_terminated);
    spin_unlock_irqrestore(&dch.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d350_synchronize(chan: *mut dma_chan) {
    static void d350_synchronize(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    vchan_synchronize(&dch.vc);
    }
#[no_mangle]
unsafe extern "C" fn d350_desc_bytes(desc: *mut d350_desc) -> u32 {
    static u32 d350_desc_bytes(struct d350_desc *desc)
    {
    return ((u32)desc.xsizehi << 16 | desc.xsize) << desc.tsz;
    }
    static enum dma_status d350_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *state)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    struct virt_dma_desc *vd;
    enum dma_status status;
    unsigned long flags;
    let mut residue: u32 = 0;
    status = dma_cookie_status(chan, cookie, state);
    spin_lock_irqsave(&dch.vc.lock, flags);
    if (cookie == dch.cookie) {
    status = dch.status;
    if (status == DMA_IN_PROGRESS || status == DMA_PAUSED)
    dch.residue = d350_get_residue(dch);
    residue = dch.residue;
    } else if ((vd = vchan_find_desc(&dch.vc, cookie))) {
    residue = d350_desc_bytes(to_d350_desc(vd));
    } else if (status == DMA_IN_PROGRESS) {
// Somebody else terminated it?
    status = DMA_ERROR;
    }
    spin_unlock_irqrestore(&dch.vc.lock, flags);
    dma_set_residue(state, residue);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn d350_start_next(dch: *mut d350_chan) {
    static void d350_start_next(struct d350_chan *dch)
    {
    u32 hdr, *reg;
    dch.desc = to_d350_desc(vchan_next_desc(&dch.vc));
    if (!dch.desc)
    return;
    list_del(&dch.desc.vd.node);
    dch.status = DMA_IN_PROGRESS;
    dch.cookie = dch.desc.vd.tx.cookie;
    dch.residue = d350_desc_bytes(dch.desc);
    hdr = dch.desc.command[0];
    reg = &dch.desc.command[1];
    if (hdr & LINK_INTREN)
    writel_relaxed(*reg++, dch.base + CH_INTREN);
    if (hdr & LINK_CTRL)
    writel_relaxed(*reg++, dch.base + CH_CTRL);
    if (hdr & LINK_SRCADDR)
    writel_relaxed(*reg++, dch.base + CH_SRCADDR);
    if (hdr & LINK_SRCADDRHI)
    writel_relaxed(*reg++, dch.base + CH_SRCADDRHI);
    if (hdr & LINK_DESADDR)
    writel_relaxed(*reg++, dch.base + CH_DESADDR);
    if (hdr & LINK_DESADDRHI)
    writel_relaxed(*reg++, dch.base + CH_DESADDRHI);
    if (hdr & LINK_XSIZE)
    writel_relaxed(*reg++, dch.base + CH_XSIZE);
    if (hdr & LINK_XSIZEHI)
    writel_relaxed(*reg++, dch.base + CH_XSIZEHI);
    if (hdr & LINK_SRCTRANSCFG)
    writel_relaxed(*reg++, dch.base + CH_SRCTRANSCFG);
    if (hdr & LINK_DESTRANSCFG)
    writel_relaxed(*reg++, dch.base + CH_DESTRANSCFG);
    if (hdr & LINK_XADDRINC)
    writel_relaxed(*reg++, dch.base + CH_XADDRINC);
    if (hdr & LINK_FILLVAL)
    writel_relaxed(*reg++, dch.base + CH_FILLVAL);
    if (hdr & LINK_SRCTRIGINCFG)
    writel_relaxed(*reg++, dch.base + CH_SRCTRIGINCFG);
    if (hdr & LINK_DESTRIGINCFG)
    writel_relaxed(*reg++, dch.base + CH_DESTRIGINCFG);
    if (hdr & LINK_AUTOCFG)
    writel_relaxed(*reg++, dch.base + CH_AUTOCFG);
    if (hdr & LINK_LINKADDR)
    writel_relaxed(*reg++, dch.base + CH_LINKADDR);
    if (hdr & LINK_LINKADDRHI)
    writel_relaxed(*reg++, dch.base + CH_LINKADDRHI);
    writel(CH_CMD_ENABLE, dch.base + CH_CMD);
    }
#[no_mangle]
unsafe extern "C" fn d350_issue_pending(chan: *mut dma_chan) {
    static void d350_issue_pending(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&dch.vc.lock, flags);
    if (vchan_issue_pending(&dch.vc) && !dch.desc)
    d350_start_next(dch);
    spin_unlock_irqrestore(&dch.vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn d350_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t d350_irq(int irq, void *data)
    {
    struct d350_chan *dch = data;
    struct device *dev = dch.vc.chan.device.dev;
    struct virt_dma_desc *vd = &dch.desc.vd;
    u32 ch_status;
    ch_status = readl(dch.base + CH_STATUS);
    if (!ch_status)
    return IRQ_NONE;
    if (ch_status & CH_STAT_INTR_ERR) {
    let mut errinfo: u32 = readl_relaxed(dch.base + CH_ERRINFO);
    if (errinfo & (CH_ERRINFO_AXIRDPOISERR | CH_ERRINFO_AXIRDRESPERR))
    vd.tx_result.result = DMA_TRANS_READ_FAILED;
#[no_mangle]
pub unsafe extern "C" fn if(CH_ERRINFO_AXIWRRESPERR: errinfo &) -> else {
    else if (errinfo & CH_ERRINFO_AXIWRRESPERR)
    vd.tx_result.result = DMA_TRANS_WRITE_FAILED;
    else
    vd.tx_result.result = DMA_TRANS_ABORTED;
    vd.tx_result.residue = d350_get_residue(dch);
    } else if (!(ch_status & CH_STAT_INTR_DONE)) {
    dev_warn(dev, "Unexpected IRQ source? 0x%08x\n", ch_status);
    }
    writel_relaxed(ch_status, dch.base + CH_STATUS);
    spin_lock(&dch.vc.lock);
    vchan_cookie_complete(vd);
    if (ch_status & CH_STAT_INTR_DONE) {
    dch.status = DMA_COMPLETE;
    dch.residue = 0;
    d350_start_next(dch);
    } else {
    dch.status = DMA_ERROR;
    dch.residue = vd.tx_result.residue;
    }
    spin_unlock(&dch.vc.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn d350_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int d350_alloc_chan_resources(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    int ret = request_irq(dch.irq, d350_irq, IRQF_SHARED,
    dev_name(&dch.vc.chan.dev.device), dch);
    if (!ret)
    writel_relaxed(CH_INTREN_DONE | CH_INTREN_ERR, dch.base + CH_INTREN);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn d350_free_chan_resources(chan: *mut dma_chan) {
    static void d350_free_chan_resources(struct dma_chan *chan)
    {
    struct d350_chan *dch = to_d350_chan(chan);
    writel_relaxed(0, dch.base + CH_INTREN);
    free_irq(dch.irq, dch);
    vchan_free_chan_resources(&dch.vc);
    }
#[no_mangle]
unsafe extern "C" fn d350_probe(pdev: *mut platform_device) -> c_int {
    static int d350_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct d350 *dmac;
    void __iomem *base;
    u32 reg;
    int ret, nchan, dw, aw, r, p;
    bool coherent, memset;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    reg = readl_relaxed(base + DMAINFO + IIDR);
    r = FIELD_GET(IIDR_VARIANT, reg);
    p = FIELD_GET(IIDR_REVISION, reg);
    if (FIELD_GET(IIDR_IMPLEMENTER, reg) != IMPLEMENTER_ARM ||
    FIELD_GET(IIDR_PRODUCTID, reg) != PRODUCTID_DMA350)
    return dev_err_probe(dev, -ENODEV, "Not a DMA-350!");
    reg = readl_relaxed(base + DMAINFO + DMA_BUILDCFG0);
    nchan = FIELD_GET(DMA_CFG_NUM_CHANNELS, reg) + 1;
    dw = 1 << FIELD_GET(DMA_CFG_DATA_WIDTH, reg);
    aw = FIELD_GET(DMA_CFG_ADDR_WIDTH, reg) + 1;
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(aw));
    coherent = device_get_dma_attr(dev) == DEV_DMA_COHERENT;
    dmac = devm_kzalloc(dev, struct_size(dmac, channels, nchan), GFP_KERNEL);
    if (!dmac)
    return -ENOMEM;
    dmac.nchan = nchan;
    reg = readl_relaxed(base + DMAINFO + DMA_BUILDCFG1);
    dmac.nreq = FIELD_GET(DMA_CFG_NUM_TRIGGER_IN, reg);
    dev_dbg(dev, "DMA-350 r%dp%d with %d channels, %d requests\n", r, p, dmac.nchan, dmac.nreq);
    dmac.dma.dev = dev;
    for (int i = min(dw, 16); i > 0; i /= 2) {
    dmac.dma.src_addr_widths |= BIT(i);
    dmac.dma.dst_addr_widths |= BIT(i);
    }
    dmac.dma.directions = BIT(DMA_MEM_TO_MEM);
    dmac.dma.descriptor_reuse = true;
    dmac.dma.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    dmac.dma.device_alloc_chan_resources = d350_alloc_chan_resources;
    dmac.dma.device_free_chan_resources = d350_free_chan_resources;
    dma_cap_set(DMA_MEMCPY, dmac.dma.cap_mask);
    dmac.dma.device_prep_dma_memcpy = d350_prep_memcpy;
    dmac.dma.device_pause = d350_pause;
    dmac.dma.device_resume = d350_resume;
    dmac.dma.device_terminate_all = d350_terminate_all;
    dmac.dma.device_synchronize = d350_synchronize;
    dmac.dma.device_tx_status = d350_tx_status;
    dmac.dma.device_issue_pending = d350_issue_pending;
    INIT_LIST_HEAD(&dmac.dma.channels);
    reg = readl_relaxed(base + DMANSECCTRL + NSEC_CTRL);
    writel_relaxed(reg | INTREN_ANYCHINTR_EN,
    base + DMANSECCTRL + NSEC_CTRL);
// Would be nice to have per-channel caps for this...
    memset = true;
    for (int i = 0; i < nchan; i++) {
    struct d350_chan *dch = &dmac.channels[i];
    dch.base = base + DMACH(i);
    writel_relaxed(CH_CMD_CLEAR, dch.base + CH_CMD);
    reg = readl_relaxed(dch.base + CH_BUILDCFG1);
    if (!(FIELD_GET(CH_CFG_HAS_CMDLINK, reg))) {
    dev_warn(dev, "No command link support on channel %d\n", i);
    continue;
    }
    dch.irq = platform_get_irq(pdev, i);
    if (dch.irq < 0)
    return dev_err_probe(dev, dch.irq,
    "Failed to get IRQ for channel %d\n", i);
    dch.has_wrap = FIELD_GET(CH_CFG_HAS_WRAP, reg);
    dch.has_trig = FIELD_GET(CH_CFG_HAS_TRIGIN, reg) &
    FIELD_GET(CH_CFG_HAS_TRIGSEL, reg);
// Fill is a special case of Wrap
    memset &= dch.has_wrap;
    reg = readl_relaxed(dch.base + CH_BUILDCFG0);
    dch.tsz = FIELD_GET(CH_CFG_DATA_WIDTH, reg);
    reg = FIELD_PREP(CH_LINK_SHAREATTR, coherent ? SHAREATTR_ISH : SHAREATTR_OSH);
    reg |= FIELD_PREP(CH_LINK_MEMATTR, coherent ? MEMATTR_WB : MEMATTR_NC);
    writel_relaxed(reg, dch.base + CH_LINKATTR);
    dch.vc.desc_free = d350_desc_free;
    vchan_init(&dch.vc, &dmac.dma);
    }
    if (memset) {
    dma_cap_set(DMA_MEMSET, dmac.dma.cap_mask);
    dmac.dma.device_prep_dma_memset = d350_prep_memset;
    }
    platform_set_drvdata(pdev, dmac);
    ret = dma_async_device_register(&dmac.dma);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register DMA device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d350_remove(pdev: *mut platform_device) {
    static void d350_remove(struct platform_device *pdev)
    {
    struct d350 *dmac = platform_get_drvdata(pdev);
    dma_async_device_unregister(&dmac.dma);
    }
    static const struct of_device_id d350_of_match[] __maybe_unused = {
    { .compatible = "arm,dma-350" },
    {}
    };
    MODULE_DEVICE_TABLE(of, d350_of_match);
    static struct platform_driver d350_driver = {
    .driver = {
    .name = "arm-dma350",
    .of_match_table = of_match_ptr(d350_of_match),
    },
    .probe = d350_probe,
    .remove = d350_remove,
    };
    module_platform_driver(d350_driver);
    MODULE_AUTHOR("Robin Murphy <robin.murphy@arm.com>");
    MODULE_DESCRIPTION("Arm DMA-350 driver");
    MODULE_LICENSE("GPL v2");
