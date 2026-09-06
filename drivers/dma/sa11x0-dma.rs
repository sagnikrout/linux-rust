//! Automatically rewritten from C to Rust
//! Source: drivers/dma/sa11x0-dma.c
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
// SA11x0 DMAengine support
//
// Copyright (C) 2012 Russell King
// Derived in part from arch/arm/mach-sa1100/dma.c,
// Copyright (C) 2000, 2001 by Nicolas Pitre
//

pub const NR_PHY_CHAN: c_int = 6;
pub const DMA_ALIGN: c_int = 3;
pub const DMA_MAX_SIZE: c_uint = 0x1fff;
pub const DMA_CHUNK_SIZE: c_uint = 0x1000;
pub const DMA_DDAR: c_uint = 0x00;
pub const DMA_DCSR_S: c_uint = 0x04;
pub const DMA_DCSR_C: c_uint = 0x08;
pub const DMA_DCSR_R: c_uint = 0x0c;
pub const DMA_DBSA: c_uint = 0x10;
pub const DMA_DBTA: c_uint = 0x14;
pub const DMA_DBSB: c_uint = 0x18;
pub const DMA_DBTB: c_uint = 0x1c;
pub const DMA_SIZE: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_sg {
    pub addr: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_desc {
    pub vd: virt_dma_desc,
    pub ddar: u32,
    pub size: usize,
    pub period: unsigned,
    pub cyclic: bool,
    pub sglen: unsigned,
    pub __counted_by(sglen): sa11x0_dma_sg sg[],
}

    struct sa11x0_dma_phy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_chan {
    pub vc: virt_dma_chan,
// protected by c->vc.lock
    pub phy: *mut sa11x0_dma_phy,
    pub status: enum dma_status,
// protected by d->lock
    pub node: list_head,
    pub ddar: u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_phy {
    pub base: *mut void __iomem,
    pub dev: *mut sa11x0_dma_dev,
    pub num: unsigned,
    pub vchan: *mut sa11x0_dma_chan,
// Protected by c->vc.lock
    pub sg_load: unsigned,
    pub txd_load: *mut sa11x0_dma_desc,
    pub sg_done: unsigned,
    pub txd_done: *mut sa11x0_dma_desc,
    pub dbs: [u32; 2],
    pub dbt: [u32; 2],
    pub dcsr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_dev {
    pub slave: dma_device,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub task: tasklet_struct,
    pub chan_pending: list_head,
    pub phy: [sa11x0_dma_phy; NR_PHY_CHAN],
}

    static struct sa11x0_dma_chan *to_sa11x0_dma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct sa11x0_dma_chan, vc.chan);
    }
    static struct sa11x0_dma_dev *to_sa11x0_dma(struct dma_device *dmadev)
    {
    return container_of(dmadev, struct sa11x0_dma_dev, slave);
    }
    static struct sa11x0_dma_desc *sa11x0_dma_next_desc(struct sa11x0_dma_chan *c)
    {
    struct virt_dma_desc *vd = vchan_next_desc(&c.vc);
    return vd ? container_of(vd, struct sa11x0_dma_desc, vd) : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_free_desc(vd: *mut virt_dma_desc) {
    static void sa11x0_dma_free_desc(struct virt_dma_desc *vd)
    {
    kfree(container_of(vd, struct sa11x0_dma_desc, vd));
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_start_desc(p: *mut sa11x0_dma_phy, txd: *mut sa11x0_dma_desc) {
    static void sa11x0_dma_start_desc(struct sa11x0_dma_phy *p, struct sa11x0_dma_desc *txd)
    {
    list_del(&txd.vd.node);
    p.txd_load = txd;
    p.sg_load = 0;
    dev_vdbg(p.dev.slave.dev, "pchan %u: txd %p[%x]: starting: DDAR:%x\n",
    p.num, &txd.vd, txd.vd.tx.cookie, txd.ddar);
    }
    static void noinline sa11x0_dma_start_sg(struct sa11x0_dma_phy *p,
    struct sa11x0_dma_chan *c)
    {
    struct sa11x0_dma_desc *txd = p.txd_load;
    struct sa11x0_dma_sg *sg;
    void __iomem *base = p.base;
    unsigned dbsx, dbtx;
    u32 dcsr;
    if (!txd)
    return;
    dcsr = readl_relaxed(base + DMA_DCSR_R);
// Don't try to load the next transfer if both buffers are started
    if ((dcsr & (DCSR_STRTA | DCSR_STRTB)) == (DCSR_STRTA | DCSR_STRTB))
    return;
    if (p.sg_load == txd.sglen) {
    if (!txd.cyclic) {
    struct sa11x0_dma_desc *txn = sa11x0_dma_next_desc(c);
//
// We have reached the end of the current descriptor.
// Peek at the next descriptor, and if compatible with
// the current, start processing it.
//
    if (txn && txn.ddar == txd.ddar) {
    txd = txn;
    sa11x0_dma_start_desc(p, txn);
    } else {
    p.txd_load = core::ptr::null_mut();
    return;
    }
    } else {
// Cyclic: reset back to beginning
    p.sg_load = 0;
    }
    }
    sg = &txd.sg[p.sg_load++];
// Select buffer to load according to channel status
    if (((dcsr & (DCSR_BIU | DCSR_STRTB)) == (DCSR_BIU | DCSR_STRTB)) ||
    ((dcsr & (DCSR_BIU | DCSR_STRTA)) == 0)) {
    dbsx = DMA_DBSA;
    dbtx = DMA_DBTA;
    dcsr = DCSR_STRTA | DCSR_IE | DCSR_RUN;
    } else {
    dbsx = DMA_DBSB;
    dbtx = DMA_DBTB;
    dcsr = DCSR_STRTB | DCSR_IE | DCSR_RUN;
    }
    writel_relaxed(sg.addr, base + dbsx);
    writel_relaxed(sg.len, base + dbtx);
    writel(dcsr, base + DMA_DCSR_S);
    dev_dbg(p.dev.slave.dev, "pchan %u: load: DCSR:%02x DBS%c:%08x DBT%c:%08x\n",
    p.num, dcsr,
    'A' + (dbsx == DMA_DBSB), sg.addr,
    'A' + (dbtx == DMA_DBTB), sg.len);
    }
    static void noinline sa11x0_dma_complete(struct sa11x0_dma_phy *p,
    struct sa11x0_dma_chan *c)
    {
    struct sa11x0_dma_desc *txd = p.txd_done;
    if (++p.sg_done == txd.sglen) {
    if (!txd.cyclic) {
    vchan_cookie_complete(&txd.vd);
    p.sg_done = 0;
    p.txd_done = p.txd_load;
    if (!p.txd_done)
    tasklet_schedule(&p.dev.task);
    } else {
    if ((p.sg_done % txd.period) == 0)
    vchan_cyclic_callback(&txd.vd);
// Cyclic: reset back to beginning
    p.sg_done = 0;
    }
    }
    sa11x0_dma_start_sg(p, c);
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sa11x0_dma_irq(int irq, void *dev_id)
    {
    struct sa11x0_dma_phy *p = dev_id;
    struct sa11x0_dma_dev *d = p.dev;
    struct sa11x0_dma_chan *c;
    u32 dcsr;
    dcsr = readl_relaxed(p.base + DMA_DCSR_R);
    if (!(dcsr & (DCSR_ERROR | DCSR_DONEA | DCSR_DONEB)))
    return IRQ_NONE;
// Clear reported status bits
    writel_relaxed(dcsr & (DCSR_ERROR | DCSR_DONEA | DCSR_DONEB),
    p.base + DMA_DCSR_C);
    dev_dbg(d.slave.dev, "pchan %u: irq: DCSR:%02x\n", p.num, dcsr);
    if (dcsr & DCSR_ERROR) {
    dev_err(d.slave.dev, "pchan %u: error. DCSR:%02x DDAR:%08x DBSA:%08x DBTA:%08x DBSB:%08x DBTB:%08x\n",
    p.num, dcsr,
    readl_relaxed(p.base + DMA_DDAR),
    readl_relaxed(p.base + DMA_DBSA),
    readl_relaxed(p.base + DMA_DBTA),
    readl_relaxed(p.base + DMA_DBSB),
    readl_relaxed(p.base + DMA_DBTB));
    }
    c = p.vchan;
    if (c) {
    unsigned long flags;
    spin_lock_irqsave(&c.vc.lock, flags);
//
// Now that we're holding the lock, check that the vchan
// really is associated with this pchan before touching the
// hardware.  This should always succeed, because we won't
// change p->vchan or c->phy while the channel is actively
// transferring.
//
    if (c.phy == p) {
    if (dcsr & DCSR_DONEA)
    sa11x0_dma_complete(p, c);
    if (dcsr & DCSR_DONEB)
    sa11x0_dma_complete(p, c);
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_start_txd(c: *mut sa11x0_dma_chan) {
    static void sa11x0_dma_start_txd(struct sa11x0_dma_chan *c)
    {
    struct sa11x0_dma_desc *txd = sa11x0_dma_next_desc(c);
// If the issued list is empty, we have no further txds to process
    if (txd) {
    struct sa11x0_dma_phy *p = c.phy;
    sa11x0_dma_start_desc(p, txd);
    p.txd_done = txd;
    p.sg_done = 0;
// The channel should not have any transfers started
    WARN_ON(readl_relaxed(p.base + DMA_DCSR_R) &
    (DCSR_STRTA | DCSR_STRTB));
// Clear the run and start bits before changing DDAR
    writel_relaxed(DCSR_RUN | DCSR_STRTA | DCSR_STRTB,
    p.base + DMA_DCSR_C);
    writel_relaxed(txd.ddar, p.base + DMA_DDAR);
// Try to start both buffers
    sa11x0_dma_start_sg(p, c);
    sa11x0_dma_start_sg(p, c);
    }
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_tasklet(t: *mut tasklet_struct) {
    static void sa11x0_dma_tasklet(struct tasklet_struct *t)
    {
    struct sa11x0_dma_dev *d = from_tasklet(d, t, task);
    struct sa11x0_dma_phy *p;
    struct sa11x0_dma_chan *c;
    unsigned pch, pch_alloc = 0;
    dev_dbg(d.slave.dev, "tasklet enter\n");
    list_for_each_entry(c, &d.slave.channels, vc.chan.device_node) {
    spin_lock_irq(&c.vc.lock);
    p = c.phy;
    if (p && !p.txd_done) {
    sa11x0_dma_start_txd(c);
    if (!p.txd_done) {
// No current txd associated with this channel
    dev_dbg(d.slave.dev, "pchan %u: free\n", p.num);
// Mark this channel free
    c.phy = core::ptr::null_mut();
    p.vchan = core::ptr::null_mut();
    }
    }
    spin_unlock_irq(&c.vc.lock);
    }
    spin_lock_irq(&d.lock);
    for (pch = 0; pch < NR_PHY_CHAN; pch++) {
    p = &d.phy[pch];
    if (p.vchan == core::ptr::null_mut() && !list_empty(&d.chan_pending)) {
    c = list_first_entry(&d.chan_pending,
    struct sa11x0_dma_chan, node);
    list_del_init(&c.node);
    pch_alloc |= 1 << pch;
// Mark this channel allocated
    p.vchan = c;
    dev_dbg(d.slave.dev, "pchan %u: alloc vchan %p\n", pch, &c.vc);
    }
    }
    spin_unlock_irq(&d.lock);
    for (pch = 0; pch < NR_PHY_CHAN; pch++) {
    if (pch_alloc & (1 << pch)) {
    p = &d.phy[pch];
    c = p.vchan;
    spin_lock_irq(&c.vc.lock);
    c.phy = p;
    sa11x0_dma_start_txd(c);
    spin_unlock_irq(&c.vc.lock);
    }
    }
    dev_dbg(d.slave.dev, "tasklet exit\n");
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_free_chan_resources(chan: *mut dma_chan) {
    static void sa11x0_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    unsigned long flags;
    spin_lock_irqsave(&d.lock, flags);
    list_del_init(&c.node);
    spin_unlock_irqrestore(&d.lock, flags);
    vchan_free_chan_resources(&c.vc);
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_pos(p: *mut sa11x0_dma_phy) -> dma_addr_t {
    static dma_addr_t sa11x0_dma_pos(struct sa11x0_dma_phy *p)
    {
    unsigned reg;
    u32 dcsr;
    dcsr = readl_relaxed(p.base + DMA_DCSR_R);
    if ((dcsr & (DCSR_BIU | DCSR_STRTA)) == DCSR_STRTA ||
    (dcsr & (DCSR_BIU | DCSR_STRTB)) == DCSR_BIU)
    reg = DMA_DBSA;
    else
    reg = DMA_DBSB;
    return readl_relaxed(p.base + reg);
    }
    static enum dma_status sa11x0_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *state)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    struct sa11x0_dma_phy *p;
    struct virt_dma_desc *vd;
    unsigned long flags;
    enum dma_status ret;
    ret = dma_cookie_status(&c.vc.chan, cookie, state);
    if (ret == DMA_COMPLETE)
    return ret;
    if (!state)
    return c.status;
    spin_lock_irqsave(&c.vc.lock, flags);
    p = c.phy;
//
// If the cookie is on our issue queue, then the residue is
// its total size.
//
    vd = vchan_find_desc(&c.vc, cookie);
    if (vd) {
    state.residue = container_of(vd, struct sa11x0_dma_desc, vd).size;
    } else if (!p) {
    state.residue = 0;
    } else {
    struct sa11x0_dma_desc *txd;
    let mut bytes: usize = 0;
    if (p.txd_done && p.txd_done.vd.tx.cookie == cookie)
    txd = p.txd_done;
#[no_mangle]
pub unsafe extern "C" fn if(cookie: p->txd_load && p->txd_load->vd.tx.cookie ==) -> else {
    else if (p.txd_load && p.txd_load.vd.tx.cookie == cookie)
    txd = p.txd_load;
    else
    txd = core::ptr::null_mut();
    ret = c.status;
    if (txd) {
    let mut addr: dma_addr_t = sa11x0_dma_pos(p);
    unsigned i;
    dev_vdbg(d.slave.dev, "tx_status: addr:%pad\n", &addr);
    for (i = 0; i < txd.sglen; i++) {
    dev_vdbg(d.slave.dev, "tx_status: [%u] %x+%x\n",
    i, txd.sg[i].addr, txd.sg[i].len);
    if (addr >= txd.sg[i].addr &&
    addr < txd.sg[i].addr + txd.sg[i].len) {
    unsigned len;
    len = txd.sg[i].len -
    (addr - txd.sg[i].addr);
    dev_vdbg(d.slave.dev, "tx_status: [%u] +%x\n",
    i, len);
    bytes += len;
    i++;
    break;
    }
    }
    for (; i < txd.sglen; i++) {
    dev_vdbg(d.slave.dev, "tx_status: [%u] %x+%x ++\n",
    i, txd.sg[i].addr, txd.sg[i].len);
    bytes += txd.sg[i].len;
    }
    }
    state.residue = bytes;
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    dev_vdbg(d.slave.dev, "tx_status: bytes 0x%x\n", state.residue);
    return ret;
    }
//
// Move pending txds to the issued list, and re-init pending list.
// If not already pending, add this channel to the list of pending
// channels and trigger the tasklet to run.
//
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_issue_pending(chan: *mut dma_chan) {
    static void sa11x0_dma_issue_pending(struct dma_chan *chan)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    unsigned long flags;
    spin_lock_irqsave(&c.vc.lock, flags);
    if (vchan_issue_pending(&c.vc)) {
    if (!c.phy) {
    spin_lock(&d.lock);
    if (list_empty(&c.node)) {
    list_add_tail(&c.node, &d.chan_pending);
    tasklet_schedule(&d.task);
    dev_dbg(d.slave.dev, "vchan %p: issued\n", &c.vc);
    }
    spin_unlock(&d.lock);
    }
    } else
    dev_dbg(d.slave.dev, "vchan %p: nothing to issue\n", &c.vc);
    spin_unlock_irqrestore(&c.vc.lock, flags);
    }
    static struct dma_async_tx_descriptor *sa11x0_dma_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sg, unsigned int sglen,
    enum dma_transfer_direction dir, unsigned long flags, void *context)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_desc *txd;
    struct scatterlist *sgent;
    unsigned int i, j;
    let mut size: usize = 0;
// SA11x0 channels can only operate in their native direction
    if (dir != (c.ddar & DDAR_RW ? DMA_DEV_TO_MEM : DMA_MEM_TO_DEV)) {
    dev_err(chan.device.dev, "vchan %p: bad DMA direction: DDAR:%08x dir:%u\n",
    &c.vc, c.ddar, dir);
    return core::ptr::null_mut();
    }
// Do not allow zero-sized txds
    if (sglen == 0)
    return core::ptr::null_mut();
    for_each_sg(sg, sgent, sglen, i) {
    let mut addr: dma_addr_t = sg_dma_address(sgent);
    if (addr & DMA_ALIGN) {
    dev_dbg(chan.device.dev, "vchan %p: bad buffer alignment: %pad\n",
    &c.vc, &addr);
    return core::ptr::null_mut();
    }
    }
    j = sg_nents_for_dma(sg, sglen, DMA_MAX_SIZE & ~DMA_ALIGN);
    txd = kzalloc_flex(*txd, sg, j, GFP_ATOMIC);
    if (!txd) {
    dev_dbg(chan.device.dev, "vchan %p: kzalloc failed\n", &c.vc);
    return core::ptr::null_mut();
    }
    txd.sglen = j;
    j = 0;
    for_each_sg(sg, sgent, sglen, i) {
    let mut addr: dma_addr_t = sg_dma_address(sgent);
    let mut len: unsigned = sg_dma_len(sgent);
    size += len;
    do {
    let mut tlen: unsigned = len;
//
// Check whether the transfer will fit.  If not, try
// to split the transfer up such that we end up with
// equal chunks - but make sure that we preserve the
// alignment.  This avoids small segments.
//
    if (tlen > DMA_MAX_SIZE) {
    unsigned mult = DIV_ROUND_UP(tlen,
    DMA_MAX_SIZE & ~DMA_ALIGN);
    tlen = (tlen / mult) & ~DMA_ALIGN;
    }
    txd.sg[j].addr = addr;
    txd.sg[j].len = tlen;
    addr += tlen;
    len -= tlen;
    j++;
    } while (len);
    }
    txd.ddar = c.ddar;
    txd.size = size;
    dev_dbg(chan.device.dev, "vchan %p: txd %p: size %zu nr %u\n",
    &c.vc, &txd.vd, txd.size, txd.sglen);
    return vchan_tx_prep(&c.vc, &txd.vd, flags);
    }
    static struct dma_async_tx_descriptor *sa11x0_dma_prep_dma_cyclic(
    struct dma_chan *chan, dma_addr_t addr, size_t size, size_t period,
    enum dma_transfer_direction dir, unsigned long flags)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_desc *txd;
    unsigned i, j, k, sglen, sgperiod;
// SA11x0 channels can only operate in their native direction
    if (dir != (c.ddar & DDAR_RW ? DMA_DEV_TO_MEM : DMA_MEM_TO_DEV)) {
    dev_err(chan.device.dev, "vchan %p: bad DMA direction: DDAR:%08x dir:%u\n",
    &c.vc, c.ddar, dir);
    return core::ptr::null_mut();
    }
    sgperiod = DIV_ROUND_UP(period, DMA_MAX_SIZE & ~DMA_ALIGN);
    sglen = size * sgperiod / period;
// Do not allow zero-sized txds
    if (sglen == 0)
    return core::ptr::null_mut();
    txd = kzalloc_flex(*txd, sg, sglen, GFP_ATOMIC);
    if (!txd) {
    dev_dbg(chan.device.dev, "vchan %p: kzalloc failed\n", &c.vc);
    return core::ptr::null_mut();
    }
    txd.sglen = sglen;
    for (i = k = 0; i < size / period; i++) {
    size_t tlen, len = period;
    for (j = 0; j < sgperiod; j++, k++) {
    tlen = len;
    if (tlen > DMA_MAX_SIZE) {
    let mut mult: unsigned = DIV_ROUND_UP(tlen, DMA_MAX_SIZE & ~DMA_ALIGN);
    tlen = (tlen / mult) & ~DMA_ALIGN;
    }
    txd.sg[k].addr = addr;
    txd.sg[k].len = tlen;
    addr += tlen;
    len -= tlen;
    }
    WARN_ON(len != 0);
    }
    WARN_ON(k != sglen);
    txd.ddar = c.ddar;
    txd.size = size;
    txd.cyclic = 1;
    txd.period = sgperiod;
    return vchan_tx_prep(&c.vc, &txd.vd, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    }
    static int sa11x0_dma_device_config(struct dma_chan *chan,
    struct dma_slave_config *cfg)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    let mut ddar: u32 = c.ddar & ((0xf << 4) | DDAR_RW);
    dma_addr_t addr;
    enum dma_slave_buswidth width;
    u32 maxburst;
    if (ddar & DDAR_RW) {
    addr = cfg.src_addr;
    width = cfg.src_addr_width;
    maxburst = cfg.src_maxburst;
    } else {
    addr = cfg.dst_addr;
    width = cfg.dst_addr_width;
    maxburst = cfg.dst_maxburst;
    }
    if ((width != DMA_SLAVE_BUSWIDTH_1_BYTE &&
    width != DMA_SLAVE_BUSWIDTH_2_BYTES) ||
    (maxburst != 4 && maxburst != 8))
    return -EINVAL;
    if (width == DMA_SLAVE_BUSWIDTH_2_BYTES)
    ddar |= DDAR_DW;
    if (maxburst == 8)
    ddar |= DDAR_BS;
    dev_dbg(c.vc.chan.device.dev, "vchan %p: dma_slave_config addr %pad width %u burst %u\n",
    &c.vc, &addr, width, maxburst);
    c.ddar = ddar | (addr & 0xf0000000) | (addr & 0x003ffffc) << 6;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_device_pause(chan: *mut dma_chan) -> c_int {
    static int sa11x0_dma_device_pause(struct dma_chan *chan)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    struct sa11x0_dma_phy *p;
    unsigned long flags;
    dev_dbg(d.slave.dev, "vchan %p: pause\n", &c.vc);
    spin_lock_irqsave(&c.vc.lock, flags);
    if (c.status == DMA_IN_PROGRESS) {
    c.status = DMA_PAUSED;
    p = c.phy;
    if (p) {
    writel(DCSR_RUN | DCSR_IE, p.base + DMA_DCSR_C);
    } else {
    spin_lock(&d.lock);
    list_del_init(&c.node);
    spin_unlock(&d.lock);
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_device_resume(chan: *mut dma_chan) -> c_int {
    static int sa11x0_dma_device_resume(struct dma_chan *chan)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    struct sa11x0_dma_phy *p;
    unsigned long flags;
    dev_dbg(d.slave.dev, "vchan %p: resume\n", &c.vc);
    spin_lock_irqsave(&c.vc.lock, flags);
    if (c.status == DMA_PAUSED) {
    c.status = DMA_IN_PROGRESS;
    p = c.phy;
    if (p) {
    writel(DCSR_RUN | DCSR_IE, p.base + DMA_DCSR_S);
    } else if (!list_empty(&c.vc.desc_issued)) {
    spin_lock(&d.lock);
    list_add_tail(&c.node, &d.chan_pending);
    spin_unlock(&d.lock);
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_device_terminate_all(chan: *mut dma_chan) -> c_int {
    static int sa11x0_dma_device_terminate_all(struct dma_chan *chan)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    struct sa11x0_dma_dev *d = to_sa11x0_dma(chan.device);
    struct sa11x0_dma_phy *p;
    LIST_HEAD(head);
    unsigned long flags;
    dev_dbg(d.slave.dev, "vchan %p: terminate all\n", &c.vc);
// Clear the tx descriptor lists
    spin_lock_irqsave(&c.vc.lock, flags);
    vchan_get_all_descriptors(&c.vc, &head);
    p = c.phy;
    if (p) {
    dev_dbg(d.slave.dev, "pchan %u: terminating\n", p.num);
// vchan is assigned to a pchan - stop the channel
    writel(DCSR_RUN | DCSR_IE |
    DCSR_STRTA | DCSR_DONEA |
    DCSR_STRTB | DCSR_DONEB,
    p.base + DMA_DCSR_C);
    if (p.txd_load) {
    if (p.txd_load != p.txd_done)
    list_add_tail(&p.txd_load.vd.node, &head);
    p.txd_load = core::ptr::null_mut();
    }
    if (p.txd_done) {
    list_add_tail(&p.txd_done.vd.node, &head);
    p.txd_done = core::ptr::null_mut();
    }
    c.phy = core::ptr::null_mut();
    spin_lock(&d.lock);
    p.vchan = core::ptr::null_mut();
    spin_unlock(&d.lock);
    tasklet_schedule(&d.task);
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    vchan_dma_desc_free_list(&c.vc, &head);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa11x0_dma_channel_desc {
    pub ddar: u32,
    pub name: *const c_char,
}

    static const struct sa11x0_dma_channel_desc chan_desc[] = {
    CD(Ser0UDCTr, 0),
    CD(Ser0UDCRc, DDAR_RW),
    CD(Ser1SDLCTr, 0),
    CD(Ser1SDLCRc, DDAR_RW),
    CD(Ser1UARTTr, 0),
    CD(Ser1UARTRc, DDAR_RW),
    CD(Ser2ICPTr, 0),
    CD(Ser2ICPRc, DDAR_RW),
    CD(Ser3UARTTr, 0),
    CD(Ser3UARTRc, DDAR_RW),
    CD(Ser4MCP0Tr, 0),
    CD(Ser4MCP0Rc, DDAR_RW),
    CD(Ser4MCP1Tr, 0),
    CD(Ser4MCP1Rc, DDAR_RW),
    CD(Ser4SSPTr, 0),
    CD(Ser4SSPRc, DDAR_RW),
    };
    static const struct dma_slave_map sa11x0_dma_map[] = {
    { "sa11x0-ir", "tx", "Ser2ICPTr" },
    { "sa11x0-ir", "rx", "Ser2ICPRc" },
    { "sa11x0-ssp", "tx", "Ser4SSPTr" },
    { "sa11x0-ssp", "rx", "Ser4SSPRc" },
    };
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_filter_fn(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool sa11x0_dma_filter_fn(struct dma_chan *chan, void *param)
    {
    struct sa11x0_dma_chan *c = to_sa11x0_dma_chan(chan);
    const char *p = param;
    return !strcmp(c.name, p);
    }
    static int sa11x0_dma_init_dmadev(struct dma_device *dmadev,
    struct device *dev)
    {
    unsigned i;
    INIT_LIST_HEAD(&dmadev.channels);
    dmadev.dev = dev;
    dmadev.device_free_chan_resources = sa11x0_dma_free_chan_resources;
    dmadev.device_config = sa11x0_dma_device_config;
    dmadev.device_pause = sa11x0_dma_device_pause;
    dmadev.device_resume = sa11x0_dma_device_resume;
    dmadev.device_terminate_all = sa11x0_dma_device_terminate_all;
    dmadev.device_tx_status = sa11x0_dma_tx_status;
    dmadev.device_issue_pending = sa11x0_dma_issue_pending;
    for (i = 0; i < ARRAY_SIZE(chan_desc); i++) {
    struct sa11x0_dma_chan *c;
    c = kzalloc_obj(*c);
    if (!c) {
    dev_err(dev, "no memory for channel %u\n", i);
    return -ENOMEM;
    }
    c.status = DMA_IN_PROGRESS;
    c.ddar = chan_desc[i].ddar;
    c.name = chan_desc[i].name;
    INIT_LIST_HEAD(&c.node);
    c.vc.desc_free = sa11x0_dma_free_desc;
    vchan_init(&c.vc, dmadev);
    }
    return dma_async_device_register(dmadev);
    }
    static int sa11x0_dma_request_irq(struct platform_device *pdev, int nr,
    void *data)
    {
    let mut irq: c_int = platform_get_irq(pdev, nr);
    if (irq <= 0)
    return -ENXIO;
    return request_irq(irq, sa11x0_dma_irq, 0, dev_name(&pdev.dev), data);
    }
    static void sa11x0_dma_free_irq(struct platform_device *pdev, int nr,
    void *data)
    {
    let mut irq: c_int = platform_get_irq(pdev, nr);
    if (irq > 0)
    free_irq(irq, data);
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_free_channels(dmadev: *mut dma_device) {
    static void sa11x0_dma_free_channels(struct dma_device *dmadev)
    {
    struct sa11x0_dma_chan *c, *cn;
    list_for_each_entry_safe(c, cn, &dmadev.channels, vc.chan.device_node) {
    list_del(&c.vc.chan.device_node);
    tasklet_kill(&c.vc.task);
    kfree(c);
    }
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_probe(pdev: *mut platform_device) -> c_int {
    static int sa11x0_dma_probe(struct platform_device *pdev)
    {
    struct sa11x0_dma_dev *d;
    struct resource *res;
    unsigned i;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENXIO;
    d = kzalloc_obj(*d);
    if (!d) {
    ret = -ENOMEM;
    goto err_alloc;
    }
    spin_lock_init(&d.lock);
    INIT_LIST_HEAD(&d.chan_pending);
    d.slave.filter.fn = sa11x0_dma_filter_fn;
    d.slave.filter.mapcnt = ARRAY_SIZE(sa11x0_dma_map);
    d.slave.filter.map = sa11x0_dma_map;
    d.base = ioremap(res.start, resource_size(res));
    if (!d.base) {
    ret = -ENOMEM;
    goto err_ioremap;
    }
    tasklet_setup(&d.task, sa11x0_dma_tasklet);
    for (i = 0; i < NR_PHY_CHAN; i++) {
    struct sa11x0_dma_phy *p = &d.phy[i];
    p.dev = d;
    p.num = i;
    p.base = d.base + i * DMA_SIZE;
    writel_relaxed(DCSR_RUN | DCSR_IE | DCSR_ERROR |
    DCSR_DONEA | DCSR_STRTA | DCSR_DONEB | DCSR_STRTB,
    p.base + DMA_DCSR_C);
    writel_relaxed(0, p.base + DMA_DDAR);
    ret = sa11x0_dma_request_irq(pdev, i, p);
    if (ret) {
    while (i) {
    i--;
    sa11x0_dma_free_irq(pdev, i, &d.phy[i]);
    }
    goto err_irq;
    }
    }
    dma_cap_set(DMA_SLAVE, d.slave.cap_mask);
    dma_cap_set(DMA_CYCLIC, d.slave.cap_mask);
    d.slave.device_prep_slave_sg = sa11x0_dma_prep_slave_sg;
    d.slave.device_prep_dma_cyclic = sa11x0_dma_prep_dma_cyclic;
    d.slave.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    d.slave.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    d.slave.src_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) |
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES);
    d.slave.dst_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) |
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES);
    ret = sa11x0_dma_init_dmadev(&d.slave, &pdev.dev);
    if (ret) {
    dev_warn(d.slave.dev, "failed to register slave async device: %d\n",
    ret);
    goto err_slave_reg;
    }
    platform_set_drvdata(pdev, d);
    return 0;
    err_slave_reg:
    sa11x0_dma_free_channels(&d.slave);
    for (i = 0; i < NR_PHY_CHAN; i++)
    sa11x0_dma_free_irq(pdev, i, &d.phy[i]);
    err_irq:
    tasklet_kill(&d.task);
    iounmap(d.base);
    err_ioremap:
    kfree(d);
    err_alloc:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_remove(pdev: *mut platform_device) {
    static void sa11x0_dma_remove(struct platform_device *pdev)
    {
    struct sa11x0_dma_dev *d = platform_get_drvdata(pdev);
    unsigned pch;
    dma_async_device_unregister(&d.slave);
    sa11x0_dma_free_channels(&d.slave);
    for (pch = 0; pch < NR_PHY_CHAN; pch++)
    sa11x0_dma_free_irq(pdev, pch, &d.phy[pch]);
    tasklet_kill(&d.task);
    iounmap(d.base);
    kfree(d);
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_suspend(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int sa11x0_dma_suspend(struct device *dev)
    {
    struct sa11x0_dma_dev *d = dev_get_drvdata(dev);
    unsigned pch;
    for (pch = 0; pch < NR_PHY_CHAN; pch++) {
    struct sa11x0_dma_phy *p = &d.phy[pch];
    u32 dcsr, saved_dcsr;
    dcsr = saved_dcsr = readl_relaxed(p.base + DMA_DCSR_R);
    if (dcsr & DCSR_RUN) {
    writel(DCSR_RUN | DCSR_IE, p.base + DMA_DCSR_C);
    dcsr = readl_relaxed(p.base + DMA_DCSR_R);
    }
    saved_dcsr &= DCSR_RUN | DCSR_IE;
    if (dcsr & DCSR_BIU) {
    p.dbs[0] = readl_relaxed(p.base + DMA_DBSB);
    p.dbt[0] = readl_relaxed(p.base + DMA_DBTB);
    p.dbs[1] = readl_relaxed(p.base + DMA_DBSA);
    p.dbt[1] = readl_relaxed(p.base + DMA_DBTA);
    saved_dcsr |= (dcsr & DCSR_STRTA ? DCSR_STRTB : 0) |
    (dcsr & DCSR_STRTB ? DCSR_STRTA : 0);
    } else {
    p.dbs[0] = readl_relaxed(p.base + DMA_DBSA);
    p.dbt[0] = readl_relaxed(p.base + DMA_DBTA);
    p.dbs[1] = readl_relaxed(p.base + DMA_DBSB);
    p.dbt[1] = readl_relaxed(p.base + DMA_DBTB);
    saved_dcsr |= dcsr & (DCSR_STRTA | DCSR_STRTB);
    }
    p.dcsr = saved_dcsr;
    writel(DCSR_STRTA | DCSR_STRTB, p.base + DMA_DCSR_C);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int sa11x0_dma_resume(struct device *dev)
    {
    struct sa11x0_dma_dev *d = dev_get_drvdata(dev);
    unsigned pch;
    for (pch = 0; pch < NR_PHY_CHAN; pch++) {
    struct sa11x0_dma_phy *p = &d.phy[pch];
    struct sa11x0_dma_desc *txd = core::ptr::null_mut();
    let mut dcsr: u32 = readl_relaxed(p.base + DMA_DCSR_R);
    WARN_ON(dcsr & (DCSR_BIU | DCSR_STRTA | DCSR_STRTB | DCSR_RUN));
    if (p.txd_done)
    txd = p.txd_done;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: p->txd_load) -> else {
    else if (p.txd_load)
    txd = p.txd_load;
    if (!txd)
    continue;
    writel_relaxed(txd.ddar, p.base + DMA_DDAR);
    writel_relaxed(p.dbs[0], p.base + DMA_DBSA);
    writel_relaxed(p.dbt[0], p.base + DMA_DBTA);
    writel_relaxed(p.dbs[1], p.base + DMA_DBSB);
    writel_relaxed(p.dbt[1], p.base + DMA_DBTB);
    writel_relaxed(p.dcsr, p.base + DMA_DCSR_S);
    }
    return 0;
    }
    static const struct dev_pm_ops sa11x0_dma_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(sa11x0_dma_suspend, sa11x0_dma_resume)
    };
    static struct platform_driver sa11x0_dma_driver = {
    .driver = {
    .name	= "sa11x0-dma",
    .pm	= &sa11x0_dma_pm_ops,
    },
    .probe		= sa11x0_dma_probe,
    .remove		= sa11x0_dma_remove,
    };
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_init() -> int __init {
    static int __init sa11x0_dma_init(void)
    {
    return platform_driver_register(&sa11x0_dma_driver);
    }
    subsys_initcall(sa11x0_dma_init);
#[no_mangle]
unsafe extern "C" fn sa11x0_dma_exit() -> void __exit {
    static void __exit sa11x0_dma_exit(void)
    {
    platform_driver_unregister(&sa11x0_dma_driver);
    }
    module_exit(sa11x0_dma_exit);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("SA-11x0 DMA driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:sa11x0-dma");
