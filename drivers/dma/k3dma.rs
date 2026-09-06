//! Automatically rewritten from C to Rust
//! Source: drivers/dma/k3dma.c
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
// Copyright (c) 2013 - 2015 Linaro Ltd.
// Copyright (c) 2013 HiSilicon Limited.
//

pub const DMA_MAX_SIZE: c_uint = 0x1ffc;
pub const DMA_CYCLIC_MAX_PERIOD: c_uint = 0x1000;

pub const INT_STAT: c_uint = 0x00;
pub const INT_TC1: c_uint = 0x04;
pub const INT_TC2: c_uint = 0x08;
pub const INT_ERR1: c_uint = 0x0c;
pub const INT_ERR2: c_uint = 0x10;
pub const INT_TC1_MASK: c_uint = 0x18;
pub const INT_TC2_MASK: c_uint = 0x1c;
pub const INT_ERR1_MASK: c_uint = 0x20;
pub const INT_ERR2_MASK: c_uint = 0x24;
pub const INT_TC1_RAW: c_uint = 0x600;
pub const INT_TC2_RAW: c_uint = 0x608;
pub const INT_ERR1_RAW: c_uint = 0x610;
pub const INT_ERR2_RAW: c_uint = 0x618;
pub const CH_PRI: c_uint = 0x688;
pub const CH_STAT: c_uint = 0x690;
pub const CX_CUR_CNT: c_uint = 0x704;
pub const CX_LLI: c_uint = 0x800;
pub const CX_CNT1: c_uint = 0x80c;
pub const CX_CNT0: c_uint = 0x810;
pub const CX_SRC: c_uint = 0x814;
pub const CX_DST: c_uint = 0x818;
pub const CX_CFG: c_uint = 0x81c;
pub const CX_LLI_CHAIN_EN: c_uint = 0x2;
pub const CX_CFG_EN: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_desc_hw {
    pub lli: u32,
    pub reserved: [u32; 3],
    pub count: u32,
    pub saddr: u32,
    pub daddr: u32,
    pub config: u32,
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_dma_desc_sw {
    pub vd: virt_dma_desc,
    pub desc_hw_lli: dma_addr_t,
    pub desc_num: usize,
    pub size: usize,
    pub desc_hw: *mut k3_desc_hw,
}

    struct k3_dma_phy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_dma_chan {
    pub ccfg: u32,
    pub vc: virt_dma_chan,
    pub phy: *mut k3_dma_phy,
    pub node: list_head,
    pub dev_addr: dma_addr_t,
    pub status: enum dma_status,
    pub cyclic: bool,
    pub slave_config: dma_slave_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_dma_phy {
    pub idx: u32,
    pub base: *mut void __iomem,
    pub vchan: *mut k3_dma_chan,
    pub ds_run: *mut k3_dma_desc_sw,
    pub ds_done: *mut k3_dma_desc_sw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_dma_dev {
    pub slave: dma_device,
    pub base: *mut void __iomem,
    pub task: tasklet_struct,
    pub lock: spinlock_t,
    pub chan_pending: list_head,
    pub phy: *mut k3_dma_phy,
    pub chans: *mut k3_dma_chan,
    pub clk: *mut clk,
    pub pool: *mut dma_pool,
    pub dma_channels: u32,
    pub dma_requests: u32,
    pub dma_channel_mask: u32,
    pub irq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3dma_soc_data {
    pub flags: c_ulong,
}

    static int k3_dma_config_write(struct dma_chan *chan,
    enum dma_transfer_direction dir,
    struct dma_slave_config *cfg);
    static struct k3_dma_chan *to_k3_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct k3_dma_chan, vc.chan);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_pause_dma(phy: *mut k3_dma_phy, on: bool) {
    static void k3_dma_pause_dma(struct k3_dma_phy *phy, bool on)
    {
    let mut val: u32 = 0;
    if (on) {
    val = readl_relaxed(phy.base + CX_CFG);
    val |= CX_CFG_EN;
    writel_relaxed(val, phy.base + CX_CFG);
    } else {
    val = readl_relaxed(phy.base + CX_CFG);
    val &= ~CX_CFG_EN;
    writel_relaxed(val, phy.base + CX_CFG);
    }
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_terminate_chan(phy: *mut k3_dma_phy, d: *mut k3_dma_dev) {
    static void k3_dma_terminate_chan(struct k3_dma_phy *phy, struct k3_dma_dev *d)
    {
    let mut val: u32 = 0;
    k3_dma_pause_dma(phy, false);
    val = 0x1 << phy.idx;
    writel_relaxed(val, d.base + INT_TC1_RAW);
    writel_relaxed(val, d.base + INT_TC2_RAW);
    writel_relaxed(val, d.base + INT_ERR1_RAW);
    writel_relaxed(val, d.base + INT_ERR2_RAW);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_set_desc(phy: *mut k3_dma_phy, hw: *mut k3_desc_hw) {
    static void k3_dma_set_desc(struct k3_dma_phy *phy, struct k3_desc_hw *hw)
    {
    writel_relaxed(hw.lli, phy.base + CX_LLI);
    writel_relaxed(hw.count, phy.base + CX_CNT0);
    writel_relaxed(hw.saddr, phy.base + CX_SRC);
    writel_relaxed(hw.daddr, phy.base + CX_DST);
    writel_relaxed(hw.config, phy.base + CX_CFG);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_get_curr_cnt(d: *mut k3_dma_dev, phy: *mut k3_dma_phy) -> u32 {
    static u32 k3_dma_get_curr_cnt(struct k3_dma_dev *d, struct k3_dma_phy *phy)
    {
    let mut cnt: u32 = 0;
    cnt = readl_relaxed(d.base + CX_CUR_CNT + phy.idx * 0x10);
    cnt &= 0xffff;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_get_curr_lli(phy: *mut k3_dma_phy) -> u32 {
    static u32 k3_dma_get_curr_lli(struct k3_dma_phy *phy)
    {
    return readl_relaxed(phy.base + CX_LLI);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_get_chan_stat(d: *mut k3_dma_dev) -> u32 {
    static u32 k3_dma_get_chan_stat(struct k3_dma_dev *d)
    {
    return readl_relaxed(d.base + CH_STAT);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_enable_dma(d: *mut k3_dma_dev, on: bool) {
    static void k3_dma_enable_dma(struct k3_dma_dev *d, bool on)
    {
    if (on) {
// set same priority
    writel_relaxed(0x0, d.base + CH_PRI);
// unmask irq
    writel_relaxed(0xffff, d.base + INT_TC1_MASK);
    writel_relaxed(0xffff, d.base + INT_TC2_MASK);
    writel_relaxed(0xffff, d.base + INT_ERR1_MASK);
    writel_relaxed(0xffff, d.base + INT_ERR2_MASK);
    } else {
// mask irq
    writel_relaxed(0x0, d.base + INT_TC1_MASK);
    writel_relaxed(0x0, d.base + INT_TC2_MASK);
    writel_relaxed(0x0, d.base + INT_ERR1_MASK);
    writel_relaxed(0x0, d.base + INT_ERR2_MASK);
    }
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_int_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t k3_dma_int_handler(int irq, void *dev_id)
    {
    struct k3_dma_dev *d = (struct k3_dma_dev *)dev_id;
    struct k3_dma_phy *p;
    struct k3_dma_chan *c;
    let mut stat: u32 = readl_relaxed(d.base + INT_STAT);
    let mut tc1: u32 = readl_relaxed(d.base + INT_TC1);
    let mut tc2: u32 = readl_relaxed(d.base + INT_TC2);
    let mut err1: u32 = readl_relaxed(d.base + INT_ERR1);
    let mut err2: u32 = readl_relaxed(d.base + INT_ERR2);
    u32 i, irq_chan = 0;
    while (stat) {
    i = __ffs(stat);
    stat &= ~BIT(i);
    if (likely(tc1 & BIT(i)) || (tc2 & BIT(i))) {
    p = &d.phy[i];
    c = p.vchan;
    if (c && (tc1 & BIT(i))) {
    spin_lock(&c.vc.lock);
    if (p.ds_run != core::ptr::null_mut()) {
    vchan_cookie_complete(&p.ds_run.vd);
    p.ds_done = p.ds_run;
    p.ds_run = core::ptr::null_mut();
    }
    spin_unlock(&c.vc.lock);
    }
    if (c && (tc2 & BIT(i))) {
    spin_lock(&c.vc.lock);
    if (p.ds_run != core::ptr::null_mut())
    vchan_cyclic_callback(&p.ds_run.vd);
    spin_unlock(&c.vc.lock);
    }
    irq_chan |= BIT(i);
    }
    if (unlikely((err1 & BIT(i)) || (err2 & BIT(i))))
    dev_warn(d.slave.dev, "DMA ERR\n");
    }
    writel_relaxed(irq_chan, d.base + INT_TC1_RAW);
    writel_relaxed(irq_chan, d.base + INT_TC2_RAW);
    writel_relaxed(err1, d.base + INT_ERR1_RAW);
    writel_relaxed(err2, d.base + INT_ERR2_RAW);
    if (irq_chan)
    tasklet_schedule(&d.task);
    if (irq_chan || err1 || err2)
    return IRQ_HANDLED;
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_start_txd(c: *mut k3_dma_chan) -> c_int {
    static int k3_dma_start_txd(struct k3_dma_chan *c)
    {
    struct k3_dma_dev *d = to_k3_dma(c.vc.chan.device);
    struct virt_dma_desc *vd = vchan_next_desc(&c.vc);
    if (!c.phy)
    return -EAGAIN;
    if (BIT(c.phy.idx) & k3_dma_get_chan_stat(d))
    return -EAGAIN;
// Avoid losing track of  ds_run if a transaction is in flight
    if (c.phy.ds_run)
    return -EAGAIN;
    if (vd) {
    struct k3_dma_desc_sw *ds =
    container_of(vd, struct k3_dma_desc_sw, vd);
//
// fetch and remove request from vc->desc_issued
// so vc->desc_issued only contains desc pending
//
    list_del(&ds.vd.node);
    c.phy.ds_run = ds;
    c.phy.ds_done = core::ptr::null_mut();
// start dma
    k3_dma_set_desc(c.phy, &ds.desc_hw[0]);
    return 0;
    }
    c.phy.ds_run = core::ptr::null_mut();
    c.phy.ds_done = core::ptr::null_mut();
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_tasklet(t: *mut tasklet_struct) {
    static void k3_dma_tasklet(struct tasklet_struct *t)
    {
    struct k3_dma_dev *d = from_tasklet(d, t, task);
    struct k3_dma_phy *p;
    struct k3_dma_chan *c, *cn;
    unsigned pch, pch_alloc = 0;
// check new dma request of running channel in vc->desc_issued
    list_for_each_entry_safe(c, cn, &d.slave.channels, vc.chan.device_node) {
    spin_lock_irq(&c.vc.lock);
    p = c.phy;
    if (p && p.ds_done) {
    if (k3_dma_start_txd(c)) {
// No current txd associated with this channel
    dev_dbg(d.slave.dev, "pchan %u: free\n", p.idx);
// Mark this channel free
    c.phy = core::ptr::null_mut();
    p.vchan = core::ptr::null_mut();
    }
    }
    spin_unlock_irq(&c.vc.lock);
    }
// check new channel request in d->chan_pending
    spin_lock_irq(&d.lock);
    for (pch = 0; pch < d.dma_channels; pch++) {
    if (!(d.dma_channel_mask & (1 << pch)))
    continue;
    p = &d.phy[pch];
    if (p.vchan == core::ptr::null_mut() && !list_empty(&d.chan_pending)) {
    c = list_first_entry(&d.chan_pending,
    struct k3_dma_chan, node);
// remove from d->chan_pending
    list_del_init(&c.node);
    pch_alloc |= 1 << pch;
// Mark this channel allocated
    p.vchan = c;
    c.phy = p;
    dev_dbg(d.slave.dev, "pchan %u: alloc vchan %p\n", pch, &c.vc);
    }
    }
    spin_unlock_irq(&d.lock);
    for (pch = 0; pch < d.dma_channels; pch++) {
    if (!(d.dma_channel_mask & (1 << pch)))
    continue;
    if (pch_alloc & (1 << pch)) {
    p = &d.phy[pch];
    c = p.vchan;
    if (c) {
    spin_lock_irq(&c.vc.lock);
    k3_dma_start_txd(c);
    spin_unlock_irq(&c.vc.lock);
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_free_chan_resources(chan: *mut dma_chan) {
    static void k3_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    unsigned long flags;
    spin_lock_irqsave(&d.lock, flags);
    list_del_init(&c.node);
    spin_unlock_irqrestore(&d.lock, flags);
    vchan_free_chan_resources(&c.vc);
    c.ccfg = 0;
    }
    static enum dma_status k3_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *state)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    struct k3_dma_phy *p;
    struct virt_dma_desc *vd;
    unsigned long flags;
    enum dma_status ret;
    let mut bytes: usize = 0;
    ret = dma_cookie_status(&c.vc.chan, cookie, state);
    if (ret == DMA_COMPLETE)
    return ret;
    spin_lock_irqsave(&c.vc.lock, flags);
    p = c.phy;
    ret = c.status;
//
// If the cookie is on our issue queue, then the residue is
// its total size.
//
    vd = vchan_find_desc(&c.vc, cookie);
    if (vd && !c.cyclic) {
    bytes = container_of(vd, struct k3_dma_desc_sw, vd).size;
    } else if ((!p) || (!p.ds_run)) {
    bytes = 0;
    } else {
    struct k3_dma_desc_sw *ds = p.ds_run;
    let mut clli: u32 = 0, index = 0;
    bytes = k3_dma_get_curr_cnt(d, p);
    clli = k3_dma_get_curr_lli(p);
    index = ((clli - ds.desc_hw_lli) /
    sizeof(struct k3_desc_hw)) + 1;
    for (; index < ds.desc_num; index++) {
    bytes += ds.desc_hw[index].count;
// end of lli
    if (!ds.desc_hw[index].lli)
    break;
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    dma_set_residue(state, bytes);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_issue_pending(chan: *mut dma_chan) {
    static void k3_dma_issue_pending(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    unsigned long flags;
    spin_lock_irqsave(&c.vc.lock, flags);
// add request to vc->desc_issued
    if (vchan_issue_pending(&c.vc)) {
    spin_lock(&d.lock);
    if (!c.phy) {
    if (list_empty(&c.node)) {
// if new channel, add chan_pending
    list_add_tail(&c.node, &d.chan_pending);
// check in tasklet
    tasklet_schedule(&d.task);
    dev_dbg(d.slave.dev, "vchan %p: issued\n", &c.vc);
    }
    }
    spin_unlock(&d.lock);
    } else
    dev_dbg(d.slave.dev, "vchan %p: nothing to issue\n", &c.vc);
    spin_unlock_irqrestore(&c.vc.lock, flags);
    }
    static void k3_dma_fill_desc(struct k3_dma_desc_sw *ds, dma_addr_t dst,
    dma_addr_t src, size_t len, u32 num, u32 ccfg)
    {
    if (num != ds.desc_num - 1)
    ds.desc_hw[num].lli = ds.desc_hw_lli + (num + 1) *
    sizeof(struct k3_desc_hw);
    ds.desc_hw[num].lli |= CX_LLI_CHAIN_EN;
    ds.desc_hw[num].count = len;
    ds.desc_hw[num].saddr = src;
    ds.desc_hw[num].daddr = dst;
    ds.desc_hw[num].config = ccfg;
    }
    static struct k3_dma_desc_sw *k3_dma_alloc_desc_resource(int num,
    struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_desc_sw *ds;
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    let mut lli_limit: c_int = LLI_BLOCK_SIZE / sizeof(struct k3_desc_hw);
    if (num > lli_limit) {
    dev_dbg(chan.device.dev, "vch %p: sg num %d exceed max %d\n",
    &c.vc, num, lli_limit);
    return core::ptr::null_mut();
    }
    ds = kzalloc_obj(*ds, GFP_NOWAIT);
    if (!ds)
    return core::ptr::null_mut();
    ds.desc_hw = dma_pool_zalloc(d.pool, GFP_NOWAIT, &ds.desc_hw_lli);
    if (!ds.desc_hw) {
    dev_dbg(chan.device.dev, "vch %p: dma alloc fail\n", &c.vc);
    kfree(ds);
    return core::ptr::null_mut();
    }
    ds.desc_num = num;
    return ds;
    }
    static struct dma_async_tx_descriptor *k3_dma_prep_memcpy(
    struct dma_chan *chan,	dma_addr_t dst, dma_addr_t src,
    size_t len, unsigned long flags)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_desc_sw *ds;
    let mut copy: usize = 0;
    let mut num: c_int = 0;
    if (!len)
    return core::ptr::null_mut();
    num = DIV_ROUND_UP(len, DMA_MAX_SIZE);
    ds = k3_dma_alloc_desc_resource(num, chan);
    if (!ds)
    return core::ptr::null_mut();
    c.cyclic = 0;
    ds.size = len;
    num = 0;
    if (!c.ccfg) {
// default is memtomem, without calling device_config
    c.ccfg = CX_CFG_SRCINCR | CX_CFG_DSTINCR | CX_CFG_EN;
    c.ccfg |= (0xf << 20) | (0xf << 24);	/* burst = 16 */
    c.ccfg |= (0x3 << 12) | (0x3 << 16);	/* width = 64 bit */
    }
    do {
    copy = min_t(size_t, len, DMA_MAX_SIZE);
    k3_dma_fill_desc(ds, dst, src, copy, num++, c.ccfg);
    src += copy;
    dst += copy;
    len -= copy;
    } while (len);
    ds.desc_hw[num-1].lli = 0;	/* end of link */
    return vchan_tx_prep(&c.vc, &ds.vd, flags);
    }
    static struct dma_async_tx_descriptor *k3_dma_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sgl, unsigned int sglen,
    enum dma_transfer_direction dir, unsigned long flags, void *context)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_desc_sw *ds;
    size_t len, avail, total = 0;
    struct scatterlist *sg;
    dma_addr_t addr, src = 0, dst = 0;
    int num, i;
    if (sgl == core::ptr::null_mut())
    return core::ptr::null_mut();
    c.cyclic = 0;
    num = sg_nents_for_dma(sgl, sglen, DMA_MAX_SIZE);
    ds = k3_dma_alloc_desc_resource(num, chan);
    if (!ds)
    return core::ptr::null_mut();
    num = 0;
    k3_dma_config_write(chan, dir, &c.slave_config);
    for_each_sg(sgl, sg, sglen, i) {
    addr = sg_dma_address(sg);
    avail = sg_dma_len(sg);
    total += avail;
    do {
    len = min_t(size_t, avail, DMA_MAX_SIZE);
    if (dir == DMA_MEM_TO_DEV) {
    src = addr;
    dst = c.dev_addr;
    } else if (dir == DMA_DEV_TO_MEM) {
    src = c.dev_addr;
    dst = addr;
    }
    k3_dma_fill_desc(ds, dst, src, len, num++, c.ccfg);
    addr += len;
    avail -= len;
    } while (avail);
    }
    ds.desc_hw[num-1].lli = 0;	/* end of link */
    ds.size = total;
    return vchan_tx_prep(&c.vc, &ds.vd, flags);
    }
    static struct dma_async_tx_descriptor *
    k3_dma_prep_dma_cyclic(struct dma_chan *chan, dma_addr_t buf_addr,
    size_t buf_len, size_t period_len,
    enum dma_transfer_direction dir,
    unsigned long flags)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_desc_sw *ds;
    size_t len, avail, total = 0;
    dma_addr_t addr, src = 0, dst = 0;
    let mut num: c_int = 1, since = 0;
    let mut modulo: usize = DMA_CYCLIC_MAX_PERIOD;
    let mut en_tc2: u32 = 0;
    dev_dbg(chan.device.dev, "%s: buf %pad, dst %pad, buf len %zu, period_len = %zu, dir %d\n",
    __func__, &buf_addr, &to_k3_chan(chan).dev_addr,
    buf_len, period_len, (int)dir);
    avail = buf_len;
    if (avail > modulo)
    num += DIV_ROUND_UP(avail, modulo) - 1;
    ds = k3_dma_alloc_desc_resource(num, chan);
    if (!ds)
    return core::ptr::null_mut();
    c.cyclic = 1;
    addr = buf_addr;
    avail = buf_len;
    total = avail;
    num = 0;
    k3_dma_config_write(chan, dir, &c.slave_config);
    if (period_len < modulo)
    modulo = period_len;
    do {
    len = min_t(size_t, avail, modulo);
    if (dir == DMA_MEM_TO_DEV) {
    src = addr;
    dst = c.dev_addr;
    } else if (dir == DMA_DEV_TO_MEM) {
    src = c.dev_addr;
    dst = addr;
    }
    since += len;
    if (since >= period_len) {
// descriptor asks for TC2 interrupt on completion
    en_tc2 = CX_CFG_NODEIRQ;
    since -= period_len;
    } else
    en_tc2 = 0;
    k3_dma_fill_desc(ds, dst, src, len, num++, c.ccfg | en_tc2);
    addr += len;
    avail -= len;
    } while (avail);
// "Cyclic" == end of link points back to start of link
    ds.desc_hw[num - 1].lli |= ds.desc_hw_lli;
    ds.size = total;
    return vchan_tx_prep(&c.vc, &ds.vd, flags);
    }
    static int k3_dma_config(struct dma_chan *chan,
    struct dma_slave_config *cfg)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    memcpy(&c.slave_config, cfg, sizeof(*cfg));
    return 0;
    }
    static int k3_dma_config_write(struct dma_chan *chan,
    enum dma_transfer_direction dir,
    struct dma_slave_config *cfg)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    let mut maxburst: u32 = 0, val = 0;
    let mut width: enum dma_slave_buswidth = DMA_SLAVE_BUSWIDTH_UNDEFINED;
    if (dir == DMA_DEV_TO_MEM) {
    c.ccfg = CX_CFG_DSTINCR;
    c.dev_addr = cfg.src_addr;
    maxburst = cfg.src_maxburst;
    width = cfg.src_addr_width;
    } else if (dir == DMA_MEM_TO_DEV) {
    c.ccfg = CX_CFG_SRCINCR;
    c.dev_addr = cfg.dst_addr;
    maxburst = cfg.dst_maxburst;
    width = cfg.dst_addr_width;
    }
    switch (width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    case DMA_SLAVE_BUSWIDTH_8_BYTES:
    val =  __ffs(width);
    break;
    default:
    val = 3;
    break;
    }
    c.ccfg |= (val << 12) | (val << 16);
    if ((maxburst == 0) || (maxburst > 16))
    val = 15;
    else
    val = maxburst - 1;
    c.ccfg |= (val << 20) | (val << 24);
    c.ccfg |= CX_CFG_MEM2PER | CX_CFG_EN;
// specific request line
    c.ccfg |= c.vc.chan.chan_id << 4;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_free_desc(vd: *mut virt_dma_desc) {
    static void k3_dma_free_desc(struct virt_dma_desc *vd)
    {
    struct k3_dma_desc_sw *ds =
    container_of(vd, struct k3_dma_desc_sw, vd);
    struct k3_dma_dev *d = to_k3_dma(vd.tx.chan.device);
    dma_pool_free(d.pool, ds.desc_hw, ds.desc_hw_lli);
    kfree(ds);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int k3_dma_terminate_all(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    struct k3_dma_phy *p = c.phy;
    unsigned long flags;
    LIST_HEAD(head);
    dev_dbg(d.slave.dev, "vchan %p: terminate all\n", &c.vc);
// Prevent this channel being scheduled
    spin_lock(&d.lock);
    list_del_init(&c.node);
    spin_unlock(&d.lock);
// Clear the tx descriptor lists
    spin_lock_irqsave(&c.vc.lock, flags);
    vchan_get_all_descriptors(&c.vc, &head);
    if (p) {
// vchan is assigned to a pchan - stop the channel
    k3_dma_terminate_chan(p, d);
    c.phy = core::ptr::null_mut();
    p.vchan = core::ptr::null_mut();
    if (p.ds_run) {
    vchan_terminate_vdesc(&p.ds_run.vd);
    p.ds_run = core::ptr::null_mut();
    }
    p.ds_done = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    vchan_dma_desc_free_list(&c.vc, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_synchronize(chan: *mut dma_chan) {
    static void k3_dma_synchronize(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    vchan_synchronize(&c.vc);
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_transfer_pause(chan: *mut dma_chan) -> c_int {
    static int k3_dma_transfer_pause(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    struct k3_dma_phy *p = c.phy;
    dev_dbg(d.slave.dev, "vchan %p: pause\n", &c.vc);
    if (c.status == DMA_IN_PROGRESS) {
    c.status = DMA_PAUSED;
    if (p) {
    k3_dma_pause_dma(p, false);
    } else {
    spin_lock(&d.lock);
    list_del_init(&c.node);
    spin_unlock(&d.lock);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_transfer_resume(chan: *mut dma_chan) -> c_int {
    static int k3_dma_transfer_resume(struct dma_chan *chan)
    {
    struct k3_dma_chan *c = to_k3_chan(chan);
    struct k3_dma_dev *d = to_k3_dma(chan.device);
    struct k3_dma_phy *p = c.phy;
    unsigned long flags;
    dev_dbg(d.slave.dev, "vchan %p: resume\n", &c.vc);
    spin_lock_irqsave(&c.vc.lock, flags);
    if (c.status == DMA_PAUSED) {
    c.status = DMA_IN_PROGRESS;
    if (p) {
    k3_dma_pause_dma(p, true);
    } else if (!list_empty(&c.vc.desc_issued)) {
    spin_lock(&d.lock);
    list_add_tail(&c.node, &d.chan_pending);
    spin_unlock(&d.lock);
    }
    }
    spin_unlock_irqrestore(&c.vc.lock, flags);
    return 0;
    }
    static const struct k3dma_soc_data k3_v1_dma_data = {
    .flags = 0,
    };
    static const struct k3dma_soc_data asp_v1_dma_data = {
    .flags = K3_FLAG_NOCLK,
    };
    static const struct of_device_id k3_pdma_dt_ids[] = {
    { .compatible = "hisilicon,k3-dma-1.0",
    .data = &k3_v1_dma_data
    },
    { .compatible = "hisilicon,hisi-pcm-asp-dma-1.0",
    .data = &asp_v1_dma_data
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, k3_pdma_dt_ids);
    static struct dma_chan *k3_of_dma_simple_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct k3_dma_dev *d = ofdma.of_dma_data;
    let mut request: c_uint = dma_spec.args[0];
    if (request >= d.dma_requests)
    return core::ptr::null_mut();
    return dma_get_slave_channel(&(d.chans[request].vc.chan));
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_probe(op: *mut platform_device) -> c_int {
    static int k3_dma_probe(struct platform_device *op)
    {
    const struct k3dma_soc_data *soc_data;
    struct k3_dma_dev *d;
    int i, ret, irq = 0;
    d = devm_kzalloc(&op.dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    soc_data = device_get_match_data(&op.dev);
    if (!soc_data)
    return -EINVAL;
    d.base = devm_platform_ioremap_resource(op, 0);
    if (IS_ERR(d.base))
    return PTR_ERR(d.base);
    of_property_read_u32((&op.dev).of_node,
    "dma-channels", &d.dma_channels);
    of_property_read_u32((&op.dev).of_node,
    "dma-requests", &d.dma_requests);
    ret = of_property_read_u32((&op.dev).of_node,
    "dma-channel-mask", &d.dma_channel_mask);
    if (ret) {
    dev_warn(&op.dev,
    "dma-channel-mask doesn't exist, considering all as available.\n");
    d.dma_channel_mask = (u32)~0UL;
    }
    if (!(soc_data.flags & K3_FLAG_NOCLK)) {
    d.clk = devm_clk_get(&op.dev, core::ptr::null_mut());
    if (IS_ERR(d.clk)) {
    dev_err(&op.dev, "no dma clk\n");
    return PTR_ERR(d.clk);
    }
    }
    irq = platform_get_irq(op, 0);
    ret = devm_request_irq(&op.dev, irq,
    k3_dma_int_handler, 0, DRIVER_NAME, d);
    if (ret)
    return ret;
    d.irq = irq;
// A DMA memory pool for LLIs, align on 32-byte boundary
    d.pool = dmam_pool_create(DRIVER_NAME, &op.dev,
    LLI_BLOCK_SIZE, 32, 0);
    if (!d.pool)
    return -ENOMEM;
// init phy channel
    d.phy = devm_kcalloc(&op.dev,
    d.dma_channels, sizeof(struct k3_dma_phy), GFP_KERNEL);
    if (d.phy == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < d.dma_channels; i++) {
    struct k3_dma_phy *p;
    if (!(d.dma_channel_mask & BIT(i)))
    continue;
    p = &d.phy[i];
    p.idx = i;
    p.base = d.base + i * 0x40;
    }
    INIT_LIST_HEAD(&d.slave.channels);
    dma_cap_set(DMA_SLAVE, d.slave.cap_mask);
    dma_cap_set(DMA_MEMCPY, d.slave.cap_mask);
    dma_cap_set(DMA_CYCLIC, d.slave.cap_mask);
    d.slave.dev = &op.dev;
    d.slave.device_free_chan_resources = k3_dma_free_chan_resources;
    d.slave.device_tx_status = k3_dma_tx_status;
    d.slave.device_prep_dma_memcpy = k3_dma_prep_memcpy;
    d.slave.device_prep_slave_sg = k3_dma_prep_slave_sg;
    d.slave.device_prep_dma_cyclic = k3_dma_prep_dma_cyclic;
    d.slave.device_issue_pending = k3_dma_issue_pending;
    d.slave.device_config = k3_dma_config;
    d.slave.device_pause = k3_dma_transfer_pause;
    d.slave.device_resume = k3_dma_transfer_resume;
    d.slave.device_terminate_all = k3_dma_terminate_all;
    d.slave.device_synchronize = k3_dma_synchronize;
    d.slave.copy_align = DMAENGINE_ALIGN_8_BYTES;
// init virtual channel
    d.chans = devm_kcalloc(&op.dev,
    d.dma_requests, sizeof(struct k3_dma_chan), GFP_KERNEL);
    if (d.chans == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < d.dma_requests; i++) {
    struct k3_dma_chan *c = &d.chans[i];
    c.status = DMA_IN_PROGRESS;
    INIT_LIST_HEAD(&c.node);
    c.vc.desc_free = k3_dma_free_desc;
    vchan_init(&c.vc, &d.slave);
    }
// Enable clock before accessing registers
    ret = clk_prepare_enable(d.clk);
    if (ret < 0) {
    dev_err(&op.dev, "clk_prepare_enable failed: %d\n", ret);
    return ret;
    }
    k3_dma_enable_dma(d, true);
    ret = dma_async_device_register(&d.slave);
    if (ret)
    goto dma_async_register_fail;
    ret = of_dma_controller_register((&op.dev).of_node,
    k3_of_dma_simple_xlate, d);
    if (ret)
    goto of_dma_register_fail;
    spin_lock_init(&d.lock);
    INIT_LIST_HEAD(&d.chan_pending);
    tasklet_setup(&d.task, k3_dma_tasklet);
    platform_set_drvdata(op, d);
    dev_info(&op.dev, "initialized\n");
    return 0;
    of_dma_register_fail:
    dma_async_device_unregister(&d.slave);
    dma_async_register_fail:
    clk_disable_unprepare(d.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_remove(op: *mut platform_device) {
    static void k3_dma_remove(struct platform_device *op)
    {
    struct k3_dma_chan *c, *cn;
    struct k3_dma_dev *d = platform_get_drvdata(op);
    dma_async_device_unregister(&d.slave);
    of_dma_controller_free((&op.dev).of_node);
    devm_free_irq(&op.dev, d.irq, d);
    list_for_each_entry_safe(c, cn, &d.slave.channels, vc.chan.device_node) {
    list_del(&c.vc.chan.device_node);
    tasklet_kill(&c.vc.task);
    }
    tasklet_kill(&d.task);
    clk_disable_unprepare(d.clk);
    }

#[no_mangle]
unsafe extern "C" fn k3_dma_suspend_dev(dev: *mut device) -> c_int {
    static int k3_dma_suspend_dev(struct device *dev)
    {
    struct k3_dma_dev *d = dev_get_drvdata(dev);
    let mut stat: u32 = 0;
    stat = k3_dma_get_chan_stat(d);
    if (stat) {
    dev_warn(d.slave.dev,
    "chan %d is running fail to suspend\n", stat);
    return -1;
    }
    k3_dma_enable_dma(d, false);
    clk_disable_unprepare(d.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn k3_dma_resume_dev(dev: *mut device) -> c_int {
    static int k3_dma_resume_dev(struct device *dev)
    {
    struct k3_dma_dev *d = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    ret = clk_prepare_enable(d.clk);
    if (ret < 0) {
    dev_err(d.slave.dev, "clk_prepare_enable failed: %d\n", ret);
    return ret;
    }
    k3_dma_enable_dma(d, true);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(k3_dma_pmops, k3_dma_suspend_dev, k3_dma_resume_dev);
    static struct platform_driver k3_pdma_driver = {
    .driver		= {
    .name	= DRIVER_NAME,
    .pm	= &k3_dma_pmops,
    .of_match_table = k3_pdma_dt_ids,
    },
    .probe		= k3_dma_probe,
    .remove		= k3_dma_remove,
    };
    module_platform_driver(k3_pdma_driver);
    MODULE_DESCRIPTION("HiSilicon k3 DMA Driver");
    MODULE_LICENSE("GPL v2");
