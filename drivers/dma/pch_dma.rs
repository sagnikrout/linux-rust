//! Automatically rewritten from C to Rust
//! Source: drivers/dma/pch_dma.c
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
// Topcliff PCH DMA controller driver
// Copyright (c) 2010 Intel Corporation
// Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
//

pub const DMA_CTL0_DISABLE: c_uint = 0x0;
pub const DMA_CTL0_SG: c_uint = 0x1;
pub const DMA_CTL0_ONESHOT: c_uint = 0x2;
pub const DMA_CTL0_MODE_MASK_BITS: c_uint = 0x3;
pub const DMA_CTL0_DIR_SHIFT_BITS: c_int = 2;
pub const DMA_CTL0_BITS_PER_CH: c_int = 4;
pub const DMA_CTL2_START_SHIFT_BITS: c_int = 8;

pub const DMA_STATUS_IDLE: c_uint = 0x0;
pub const DMA_STATUS_DESC_READ: c_uint = 0x1;
pub const DMA_STATUS_WAIT: c_uint = 0x2;
pub const DMA_STATUS_ACCESS: c_uint = 0x3;
pub const DMA_STATUS_BITS_PER_CH: c_int = 2;
pub const DMA_STATUS_MASK_BITS: c_uint = 0x3;
pub const DMA_STATUS_SHIFT_BITS: c_int = 16;

pub const DMA_DESC_WIDTH_SHIFT_BITS: c_int = 12;

pub const DMA_DESC_MAX_COUNT_1_BYTE: c_uint = 0x3FF;
pub const DMA_DESC_MAX_COUNT_2_BYTES: c_uint = 0x3FF;
pub const DMA_DESC_MAX_COUNT_4_BYTES: c_uint = 0x7FF;
pub const DMA_DESC_END_WITHOUT_IRQ: c_uint = 0x0;
pub const DMA_DESC_END_WITH_IRQ: c_uint = 0x1;
pub const DMA_DESC_FOLLOW_WITHOUT_IRQ: c_uint = 0x2;
pub const DMA_DESC_FOLLOW_WITH_IRQ: c_uint = 0x3;
pub const MAX_CHAN_NR: c_int = 12;
pub const DMA_MASK_CTL0_MODE: c_uint = 0x33333333;
pub const DMA_MASK_CTL2_MODE: c_uint = 0x00003333;
    let mut init_nr_desc_per_channel: static unsigned int = 64;
    module_param(init_nr_desc_per_channel, uint, 0644);
    MODULE_PARM_DESC(init_nr_desc_per_channel,
    "initial descriptors per channel (default: 64)");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_dma_desc_regs {
    pub dev_addr: u32,
    pub mem_addr: u32,
    pub size: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_dma_regs {
    pub dma_ctl0: u32,
    pub dma_ctl1: u32,
    pub dma_ctl2: u32,
    pub dma_ctl3: u32,
    pub dma_sts0: u32,
    pub dma_sts1: u32,
    pub dma_sts2: u32,
    pub reserved3: u32,
    pub desc: [pch_dma_desc_regs; MAX_CHAN_NR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_dma_desc {
    pub regs: pch_dma_desc_regs,
    pub txd: dma_async_tx_descriptor,
    pub desc_node: list_head,
    pub tx_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_dma_chan {
    pub chan: dma_chan,
    pub membase: *mut void __iomem,
    pub dir: enum dma_transfer_direction,
    pub tasklet: tasklet_struct,
    pub err_status: c_ulong,
    pub lock: spinlock_t,
    pub active_list: list_head,
    pub queue: list_head,
    pub free_list: list_head,
    pub descs_allocated: c_uint,
}

pub const PDC_DEV_ADDR: c_uint = 0x00;
pub const PDC_MEM_ADDR: c_uint = 0x04;
pub const PDC_SIZE: c_uint = 0x08;
pub const PDC_NEXT: c_uint = 0x0C;

    readl((pdc).membase + PDC_##name)

    writel((val), (pdc).membase + PDC_##name)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_dma {
    pub dma: dma_device,
    pub membase: *mut void __iomem,
    pub pool: *mut dma_pool,
    pub regs: pch_dma_regs,
    pub ch_regs: [pch_dma_desc_regs; MAX_CHAN_NR],
    pub channels: [pch_dma_chan; MAX_CHAN_NR],
}

pub const PCH_DMA_CTL0: c_uint = 0x00;
pub const PCH_DMA_CTL1: c_uint = 0x04;
pub const PCH_DMA_CTL2: c_uint = 0x08;
pub const PCH_DMA_CTL3: c_uint = 0x0C;
pub const PCH_DMA_STS0: c_uint = 0x10;
pub const PCH_DMA_STS1: c_uint = 0x14;
pub const PCH_DMA_STS2: c_uint = 0x18;

    readl((pd).membase + PCH_DMA_##name)

    writel((val), (pd).membase + PCH_DMA_##name)
    static inline
    struct pch_dma_desc *to_pd_desc(struct dma_async_tx_descriptor *txd)
    {
    return container_of(txd, struct pch_dma_desc, txd);
    }
    static inline struct pch_dma_chan *to_pd_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct pch_dma_chan, chan);
    }
    static inline struct pch_dma *to_pd(struct dma_device *ddev)
    {
    return container_of(ddev, struct pch_dma, dma);
    }
    static inline struct device *chan2dev(struct dma_chan *chan)
    {
    return &chan.dev.device;
    }
    static inline
    struct pch_dma_desc *pdc_first_active(struct pch_dma_chan *pd_chan)
    {
    return list_first_entry(&pd_chan.active_list,
    struct pch_dma_desc, desc_node);
    }
    static inline
    struct pch_dma_desc *pdc_first_queued(struct pch_dma_chan *pd_chan)
    {
    return list_first_entry(&pd_chan.queue,
    struct pch_dma_desc, desc_node);
    }
#[no_mangle]
unsafe extern "C" fn pdc_enable_irq(chan: *mut dma_chan, enable: c_int) {
    static void pdc_enable_irq(struct dma_chan *chan, int enable)
    {
    struct pch_dma *pd = to_pd(chan.device);
    u32 val;
    int pos;
    if (chan.chan_id < 8)
    pos = chan.chan_id;
    else
    pos = chan.chan_id + 8;
    val = dma_readl(pd, CTL2);
    if (enable)
    val |= 0x1 << pos;
    else
    val &= ~(0x1 << pos);
    dma_writel(pd, CTL2, val);
    dev_dbg(chan2dev(chan), "pdc_enable_irq: chan %d . %x\n",
    chan.chan_id, val);
    }
#[no_mangle]
unsafe extern "C" fn pdc_set_dir(chan: *mut dma_chan) {
    static void pdc_set_dir(struct dma_chan *chan)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    struct pch_dma *pd = to_pd(chan.device);
    u32 val;
    u32 mask_mode;
    u32 mask_ctl;
    if (chan.chan_id < 8) {
    val = dma_readl(pd, CTL0);
    mask_mode = DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * chan.chan_id);
    mask_ctl = DMA_MASK_CTL0_MODE & ~(DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * chan.chan_id));
    val &= mask_mode;
    if (pd_chan.dir == DMA_MEM_TO_DEV)
    val |= 0x1 << (DMA_CTL0_BITS_PER_CH * chan.chan_id +
    DMA_CTL0_DIR_SHIFT_BITS);
    else
    val &= ~(0x1 << (DMA_CTL0_BITS_PER_CH * chan.chan_id +
    DMA_CTL0_DIR_SHIFT_BITS));
    val |= mask_ctl;
    dma_writel(pd, CTL0, val);
    } else {
    int ch = chan.chan_id - 8; /* ch8-.0 ch9-.1 ... ch11.3 */
    val = dma_readl(pd, CTL3);
    mask_mode = DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * ch);
    mask_ctl = DMA_MASK_CTL2_MODE & ~(DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * ch));
    val &= mask_mode;
    if (pd_chan.dir == DMA_MEM_TO_DEV)
    val |= 0x1 << (DMA_CTL0_BITS_PER_CH * ch +
    DMA_CTL0_DIR_SHIFT_BITS);
    else
    val &= ~(0x1 << (DMA_CTL0_BITS_PER_CH * ch +
    DMA_CTL0_DIR_SHIFT_BITS));
    val |= mask_ctl;
    dma_writel(pd, CTL3, val);
    }
    dev_dbg(chan2dev(chan), "pdc_set_dir: chan %d . %x\n",
    chan.chan_id, val);
    }
#[no_mangle]
unsafe extern "C" fn pdc_set_mode(chan: *mut dma_chan, mode: u32) {
    static void pdc_set_mode(struct dma_chan *chan, u32 mode)
    {
    struct pch_dma *pd = to_pd(chan.device);
    u32 val;
    u32 mask_ctl;
    u32 mask_dir;
    if (chan.chan_id < 8) {
    mask_ctl = DMA_MASK_CTL0_MODE & ~(DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * chan.chan_id));
    mask_dir = 1 << (DMA_CTL0_BITS_PER_CH * chan.chan_id +\
    DMA_CTL0_DIR_SHIFT_BITS);
    val = dma_readl(pd, CTL0);
    val &= mask_dir;
    val |= mode << (DMA_CTL0_BITS_PER_CH * chan.chan_id);
    val |= mask_ctl;
    dma_writel(pd, CTL0, val);
    } else {
    int ch = chan.chan_id - 8; /* ch8-.0 ch9-.1 ... ch11.3 */
    mask_ctl = DMA_MASK_CTL2_MODE & ~(DMA_CTL0_MODE_MASK_BITS <<
    (DMA_CTL0_BITS_PER_CH * ch));
    mask_dir = 1 << (DMA_CTL0_BITS_PER_CH * ch +\
    DMA_CTL0_DIR_SHIFT_BITS);
    val = dma_readl(pd, CTL3);
    val &= mask_dir;
    val |= mode << (DMA_CTL0_BITS_PER_CH * ch);
    val |= mask_ctl;
    dma_writel(pd, CTL3, val);
    }
    dev_dbg(chan2dev(chan), "pdc_set_mode: chan %d . %x\n",
    chan.chan_id, val);
    }
#[no_mangle]
unsafe extern "C" fn pdc_get_status0(pd_chan: *mut pch_dma_chan) -> u32 {
    static u32 pdc_get_status0(struct pch_dma_chan *pd_chan)
    {
    struct pch_dma *pd = to_pd(pd_chan.chan.device);
    u32 val;
    val = dma_readl(pd, STS0);
    return DMA_STATUS_MASK_BITS & (val >> (DMA_STATUS_SHIFT_BITS +
    DMA_STATUS_BITS_PER_CH * pd_chan.chan.chan_id));
    }
#[no_mangle]
unsafe extern "C" fn pdc_get_status2(pd_chan: *mut pch_dma_chan) -> u32 {
    static u32 pdc_get_status2(struct pch_dma_chan *pd_chan)
    {
    struct pch_dma *pd = to_pd(pd_chan.chan.device);
    u32 val;
    val = dma_readl(pd, STS2);
    return DMA_STATUS_MASK_BITS & (val >> (DMA_STATUS_SHIFT_BITS +
    DMA_STATUS_BITS_PER_CH * (pd_chan.chan.chan_id - 8)));
    }
#[no_mangle]
unsafe extern "C" fn pdc_is_idle(pd_chan: *mut pch_dma_chan) -> bool {
    static bool pdc_is_idle(struct pch_dma_chan *pd_chan)
    {
    u32 sts;
    if (pd_chan.chan.chan_id < 8)
    sts = pdc_get_status0(pd_chan);
    else
    sts = pdc_get_status2(pd_chan);
    if (sts == DMA_STATUS_IDLE)
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pdc_dostart(pd_chan: *mut pch_dma_chan, desc: *mut *mut pch_dma_desc) {
    static void pdc_dostart(struct pch_dma_chan *pd_chan, struct pch_dma_desc* desc)
    {
    if (!pdc_is_idle(pd_chan)) {
    dev_err(chan2dev(&pd_chan.chan),
    "BUG: Attempt to start non-idle channel\n");
    return;
    }
    dev_dbg(chan2dev(&pd_chan.chan), "chan %d . dev_addr: %x\n",
    pd_chan.chan.chan_id, desc.regs.dev_addr);
    dev_dbg(chan2dev(&pd_chan.chan), "chan %d . mem_addr: %x\n",
    pd_chan.chan.chan_id, desc.regs.mem_addr);
    dev_dbg(chan2dev(&pd_chan.chan), "chan %d . size: %x\n",
    pd_chan.chan.chan_id, desc.regs.size);
    dev_dbg(chan2dev(&pd_chan.chan), "chan %d . next: %x\n",
    pd_chan.chan.chan_id, desc.regs.next);
    if (list_empty(&desc.tx_list)) {
    channel_writel(pd_chan, DEV_ADDR, desc.regs.dev_addr);
    channel_writel(pd_chan, MEM_ADDR, desc.regs.mem_addr);
    channel_writel(pd_chan, SIZE, desc.regs.size);
    channel_writel(pd_chan, NEXT, desc.regs.next);
    pdc_set_mode(&pd_chan.chan, DMA_CTL0_ONESHOT);
    } else {
    channel_writel(pd_chan, NEXT, desc.txd.phys);
    pdc_set_mode(&pd_chan.chan, DMA_CTL0_SG);
    }
    }
    static void pdc_chain_complete(struct pch_dma_chan *pd_chan,
    struct pch_dma_desc *desc)
    {
    struct dma_async_tx_descriptor *txd = &desc.txd;
    struct dmaengine_desc_callback cb;
    dmaengine_desc_get_callback(txd, &cb);
    list_splice_init(&desc.tx_list, &pd_chan.free_list);
    list_move(&desc.desc_node, &pd_chan.free_list);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn pdc_complete_all(pd_chan: *mut pch_dma_chan) {
    static void pdc_complete_all(struct pch_dma_chan *pd_chan)
    {
    struct pch_dma_desc *desc, *_d;
    LIST_HEAD(list);
    BUG_ON(!pdc_is_idle(pd_chan));
    if (!list_empty(&pd_chan.queue))
    pdc_dostart(pd_chan, pdc_first_queued(pd_chan));
    list_splice_init(&pd_chan.active_list, &list);
    list_splice_init(&pd_chan.queue, &pd_chan.active_list);
    list_for_each_entry_safe(desc, _d, &list, desc_node)
    pdc_chain_complete(pd_chan, desc);
    }
#[no_mangle]
unsafe extern "C" fn pdc_handle_error(pd_chan: *mut pch_dma_chan) {
    static void pdc_handle_error(struct pch_dma_chan *pd_chan)
    {
    struct pch_dma_desc *bad_desc;
    bad_desc = pdc_first_active(pd_chan);
    list_del(&bad_desc.desc_node);
    list_splice_init(&pd_chan.queue, pd_chan.active_list.prev);
    if (!list_empty(&pd_chan.active_list))
    pdc_dostart(pd_chan, pdc_first_active(pd_chan));
    dev_crit(chan2dev(&pd_chan.chan), "Bad descriptor submitted\n");
    dev_crit(chan2dev(&pd_chan.chan), "descriptor cookie: %d\n",
    bad_desc.txd.cookie);
    pdc_chain_complete(pd_chan, bad_desc);
    }
#[no_mangle]
unsafe extern "C" fn pdc_advance_work(pd_chan: *mut pch_dma_chan) {
    static void pdc_advance_work(struct pch_dma_chan *pd_chan)
    {
    if (list_empty(&pd_chan.active_list) ||
    list_is_singular(&pd_chan.active_list)) {
    pdc_complete_all(pd_chan);
    } else {
    pdc_chain_complete(pd_chan, pdc_first_active(pd_chan));
    pdc_dostart(pd_chan, pdc_first_active(pd_chan));
    }
    }
#[no_mangle]
unsafe extern "C" fn pd_tx_submit(txd: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t pd_tx_submit(struct dma_async_tx_descriptor *txd)
    {
    struct pch_dma_desc *desc = to_pd_desc(txd);
    struct pch_dma_chan *pd_chan = to_pd_chan(txd.chan);
    spin_lock(&pd_chan.lock);
    if (list_empty(&pd_chan.active_list)) {
    list_add_tail(&desc.desc_node, &pd_chan.active_list);
    pdc_dostart(pd_chan, desc);
    } else {
    list_add_tail(&desc.desc_node, &pd_chan.queue);
    }
    spin_unlock(&pd_chan.lock);
    return 0;
    }
    static struct pch_dma_desc *pdc_alloc_desc(struct dma_chan *chan, gfp_t flags)
    {
    struct pch_dma_desc *desc = core::ptr::null_mut();
    struct pch_dma *pd = to_pd(chan.device);
    dma_addr_t addr;
    desc = dma_pool_zalloc(pd.pool, flags, &addr);
    if (desc) {
    INIT_LIST_HEAD(&desc.tx_list);
    dma_async_tx_descriptor_init(&desc.txd, chan);
    desc.txd.tx_submit = pd_tx_submit;
    desc.txd.flags = DMA_CTRL_ACK;
    desc.txd.phys = addr;
    }
    return desc;
    }
    static struct pch_dma_desc *pdc_desc_get(struct pch_dma_chan *pd_chan)
    {
    struct pch_dma_desc *desc, *_d;
    struct pch_dma_desc *ret = core::ptr::null_mut();
    let mut i: c_int = 0;
    spin_lock(&pd_chan.lock);
    list_for_each_entry_safe(desc, _d, &pd_chan.free_list, desc_node) {
    i++;
    if (async_tx_test_ack(&desc.txd)) {
    list_del(&desc.desc_node);
    ret = desc;
    break;
    }
    dev_dbg(chan2dev(&pd_chan.chan), "desc %p not ACKed\n", desc);
    }
    spin_unlock(&pd_chan.lock);
    dev_dbg(chan2dev(&pd_chan.chan), "scanned %d descriptors\n", i);
    if (!ret) {
    ret = pdc_alloc_desc(&pd_chan.chan, GFP_ATOMIC);
    if (ret) {
    spin_lock(&pd_chan.lock);
    pd_chan.descs_allocated++;
    spin_unlock(&pd_chan.lock);
    } else {
    dev_err(chan2dev(&pd_chan.chan),
    "failed to alloc desc\n");
    }
    }
    return ret;
    }
    static void pdc_desc_put(struct pch_dma_chan *pd_chan,
    struct pch_dma_desc *desc)
    {
    if (desc) {
    spin_lock(&pd_chan.lock);
    list_splice_init(&desc.tx_list, &pd_chan.free_list);
    list_add(&desc.desc_node, &pd_chan.free_list);
    spin_unlock(&pd_chan.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn pd_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int pd_alloc_chan_resources(struct dma_chan *chan)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    struct pch_dma_desc *desc;
    LIST_HEAD(tmp_list);
    int i;
    if (!pdc_is_idle(pd_chan)) {
    dev_dbg(chan2dev(chan), "DMA channel not idle ?\n");
    return -EIO;
    }
    if (!list_empty(&pd_chan.free_list))
    return pd_chan.descs_allocated;
    for (i = 0; i < init_nr_desc_per_channel; i++) {
    desc = pdc_alloc_desc(chan, GFP_KERNEL);
    if (!desc) {
    dev_warn(chan2dev(chan),
    "Only allocated %d initial descriptors\n", i);
    break;
    }
    list_add_tail(&desc.desc_node, &tmp_list);
    }
    spin_lock_irq(&pd_chan.lock);
    list_splice(&tmp_list, &pd_chan.free_list);
    pd_chan.descs_allocated = i;
    dma_cookie_init(chan);
    spin_unlock_irq(&pd_chan.lock);
    pdc_enable_irq(chan, 1);
    return pd_chan.descs_allocated;
    }
#[no_mangle]
unsafe extern "C" fn pd_free_chan_resources(chan: *mut dma_chan) {
    static void pd_free_chan_resources(struct dma_chan *chan)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    struct pch_dma *pd = to_pd(chan.device);
    struct pch_dma_desc *desc, *_d;
    LIST_HEAD(tmp_list);
    BUG_ON(!pdc_is_idle(pd_chan));
    BUG_ON(!list_empty(&pd_chan.active_list));
    BUG_ON(!list_empty(&pd_chan.queue));
    spin_lock_irq(&pd_chan.lock);
    list_splice_init(&pd_chan.free_list, &tmp_list);
    pd_chan.descs_allocated = 0;
    spin_unlock_irq(&pd_chan.lock);
    list_for_each_entry_safe(desc, _d, &tmp_list, desc_node)
    dma_pool_free(pd.pool, desc, desc.txd.phys);
    pdc_enable_irq(chan, 0);
    }
    static enum dma_status pd_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    return dma_cookie_status(chan, cookie, txstate);
    }
#[no_mangle]
unsafe extern "C" fn pd_issue_pending(chan: *mut dma_chan) {
    static void pd_issue_pending(struct dma_chan *chan)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    if (pdc_is_idle(pd_chan)) {
    spin_lock(&pd_chan.lock);
    pdc_advance_work(pd_chan);
    spin_unlock(&pd_chan.lock);
    }
    }
    static struct dma_async_tx_descriptor *pd_prep_slave_sg(struct dma_chan *chan,
    struct scatterlist *sgl, unsigned int sg_len,
    enum dma_transfer_direction direction, unsigned long flags,
    void *context)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    struct pch_dma_slave *pd_slave = chan.private;
    struct pch_dma_desc *first = core::ptr::null_mut();
    struct pch_dma_desc *prev = core::ptr::null_mut();
    struct pch_dma_desc *desc = core::ptr::null_mut();
    struct scatterlist *sg;
    dma_addr_t reg;
    int i;
    if (unlikely(!sg_len)) {
    dev_info(chan2dev(chan), "prep_slave_sg: length is zero!\n");
    return core::ptr::null_mut();
    }
    if (direction == DMA_DEV_TO_MEM)
    reg = pd_slave.rx_reg;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_MEM_TO_DEV: direction ==) -> else {
    else if (direction == DMA_MEM_TO_DEV)
    reg = pd_slave.tx_reg;
    else
    return core::ptr::null_mut();
    pd_chan.dir = direction;
    pdc_set_dir(chan);
    for_each_sg(sgl, sg, sg_len, i) {
    desc = pdc_desc_get(pd_chan);
    if (!desc)
    goto err_desc_get;
    desc.regs.dev_addr = reg;
    desc.regs.mem_addr = sg_dma_address(sg);
    desc.regs.size = sg_dma_len(sg);
    desc.regs.next = DMA_DESC_FOLLOW_WITHOUT_IRQ;
    switch (pd_slave.width) {
    case PCH_DMA_WIDTH_1_BYTE:
    if (desc.regs.size > DMA_DESC_MAX_COUNT_1_BYTE)
    goto err_desc_get;
    desc.regs.size |= DMA_DESC_WIDTH_1_BYTE;
    break;
    case PCH_DMA_WIDTH_2_BYTES:
    if (desc.regs.size > DMA_DESC_MAX_COUNT_2_BYTES)
    goto err_desc_get;
    desc.regs.size |= DMA_DESC_WIDTH_2_BYTES;
    break;
    case PCH_DMA_WIDTH_4_BYTES:
    if (desc.regs.size > DMA_DESC_MAX_COUNT_4_BYTES)
    goto err_desc_get;
    desc.regs.size |= DMA_DESC_WIDTH_4_BYTES;
    break;
    default:
    goto err_desc_get;
    }
    if (!first) {
    first = desc;
    } else {
    prev.regs.next |= desc.txd.phys;
    list_add_tail(&desc.desc_node, &first.tx_list);
    }
    prev = desc;
    }
    if (flags & DMA_PREP_INTERRUPT)
    desc.regs.next = DMA_DESC_END_WITH_IRQ;
    else
    desc.regs.next = DMA_DESC_END_WITHOUT_IRQ;
    first.txd.cookie = -EBUSY;
    desc.txd.flags = flags;
    return &first.txd;
    err_desc_get:
    dev_err(chan2dev(chan), "failed to get desc or wrong parameters\n");
    pdc_desc_put(pd_chan, first);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pd_device_terminate_all(chan: *mut dma_chan) -> c_int {
    static int pd_device_terminate_all(struct dma_chan *chan)
    {
    struct pch_dma_chan *pd_chan = to_pd_chan(chan);
    struct pch_dma_desc *desc, *_d;
    LIST_HEAD(list);
    spin_lock_irq(&pd_chan.lock);
    pdc_set_mode(&pd_chan.chan, DMA_CTL0_DISABLE);
    list_splice_init(&pd_chan.active_list, &list);
    list_splice_init(&pd_chan.queue, &list);
    list_for_each_entry_safe(desc, _d, &list, desc_node)
    pdc_chain_complete(pd_chan, desc);
    spin_unlock_irq(&pd_chan.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pdc_tasklet(t: *mut tasklet_struct) {
    static void pdc_tasklet(struct tasklet_struct *t)
    {
    struct pch_dma_chan *pd_chan = from_tasklet(pd_chan, t, tasklet);
    unsigned long flags;
    if (!pdc_is_idle(pd_chan)) {
    dev_err(chan2dev(&pd_chan.chan),
    "BUG: handle non-idle channel in tasklet\n");
    return;
    }
    spin_lock_irqsave(&pd_chan.lock, flags);
    if (test_and_clear_bit(0, &pd_chan.err_status))
    pdc_handle_error(pd_chan);
    else
    pdc_advance_work(pd_chan);
    spin_unlock_irqrestore(&pd_chan.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn pd_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t pd_irq(int irq, void *devid)
    {
    struct pch_dma *pd = (struct pch_dma *)devid;
    struct pch_dma_chan *pd_chan;
    u32 sts0;
    u32 sts2;
    int i;
    let mut ret0: c_int = IRQ_NONE;
    let mut ret2: c_int = IRQ_NONE;
    sts0 = dma_readl(pd, STS0);
    sts2 = dma_readl(pd, STS2);
    dev_dbg(pd.dma.dev, "pd_irq sts0: %x\n", sts0);
    for (i = 0; i < pd.dma.chancnt; i++) {
    pd_chan = &pd.channels[i];
    if (i < 8) {
    if (sts0 & DMA_STATUS_IRQ(i)) {
    if (sts0 & DMA_STATUS0_ERR(i))
    set_bit(0, &pd_chan.err_status);
    tasklet_schedule(&pd_chan.tasklet);
    ret0 = IRQ_HANDLED;
    }
    } else {
    if (sts2 & DMA_STATUS_IRQ(i - 8)) {
    if (sts2 & DMA_STATUS2_ERR(i))
    set_bit(0, &pd_chan.err_status);
    tasklet_schedule(&pd_chan.tasklet);
    ret2 = IRQ_HANDLED;
    }
    }
    }
// clear interrupt bits in status register
    if (ret0)
    dma_writel(pd, STS0, sts0);
    if (ret2)
    dma_writel(pd, STS2, sts2);
    return ret0 | ret2;
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_save_regs(pd: *mut pch_dma) -> void __maybe_unused {
    static void __maybe_unused pch_dma_save_regs(struct pch_dma *pd)
    {
    struct pch_dma_chan *pd_chan;
    struct dma_chan *chan, *_c;
    let mut i: c_int = 0;
    pd.regs.dma_ctl0 = dma_readl(pd, CTL0);
    pd.regs.dma_ctl1 = dma_readl(pd, CTL1);
    pd.regs.dma_ctl2 = dma_readl(pd, CTL2);
    pd.regs.dma_ctl3 = dma_readl(pd, CTL3);
    list_for_each_entry_safe(chan, _c, &pd.dma.channels, device_node) {
    pd_chan = to_pd_chan(chan);
    pd.ch_regs[i].dev_addr = channel_readl(pd_chan, DEV_ADDR);
    pd.ch_regs[i].mem_addr = channel_readl(pd_chan, MEM_ADDR);
    pd.ch_regs[i].size = channel_readl(pd_chan, SIZE);
    pd.ch_regs[i].next = channel_readl(pd_chan, NEXT);
    i++;
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_restore_regs(pd: *mut pch_dma) -> void __maybe_unused {
    static void __maybe_unused pch_dma_restore_regs(struct pch_dma *pd)
    {
    struct pch_dma_chan *pd_chan;
    struct dma_chan *chan, *_c;
    let mut i: c_int = 0;
    dma_writel(pd, CTL0, pd.regs.dma_ctl0);
    dma_writel(pd, CTL1, pd.regs.dma_ctl1);
    dma_writel(pd, CTL2, pd.regs.dma_ctl2);
    dma_writel(pd, CTL3, pd.regs.dma_ctl3);
    list_for_each_entry_safe(chan, _c, &pd.dma.channels, device_node) {
    pd_chan = to_pd_chan(chan);
    channel_writel(pd_chan, DEV_ADDR, pd.ch_regs[i].dev_addr);
    channel_writel(pd_chan, MEM_ADDR, pd.ch_regs[i].mem_addr);
    channel_writel(pd_chan, SIZE, pd.ch_regs[i].size);
    channel_writel(pd_chan, NEXT, pd.ch_regs[i].next);
    i++;
    }
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pch_dma_suspend(struct device *dev)
    {
    struct pch_dma *pd = dev_get_drvdata(dev);
    if (pd)
    pch_dma_save_regs(pd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused pch_dma_resume(struct device *dev)
    {
    struct pch_dma *pd = dev_get_drvdata(dev);
    if (pd)
    pch_dma_restore_regs(pd);
    return 0;
    }
    static int pch_dma_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct pch_dma *pd;
    struct pch_dma_regs *regs;
    unsigned int nr_channels;
    int err;
    int i;
    nr_channels = id.driver_data;
    pd = kzalloc_obj(*pd);
    if (!pd)
    return -ENOMEM;
    pci_set_drvdata(pdev, pd);
    err = pci_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "Cannot enable PCI device\n");
    goto err_free_mem;
    }
    if (!(pci_resource_flags(pdev, 1) & IORESOURCE_MEM)) {
    dev_err(&pdev.dev, "Cannot find proper base address\n");
    err = -ENODEV;
    goto err_disable_pdev;
    }
    err = pci_request_regions(pdev, DRV_NAME);
    if (err) {
    dev_err(&pdev.dev, "Cannot obtain PCI resources\n");
    goto err_disable_pdev;
    }
    err = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (err) {
    dev_err(&pdev.dev, "Cannot set proper DMA config\n");
    goto err_free_res;
    }
    regs = pd.membase = pci_iomap(pdev, 1, 0);
    if (!pd.membase) {
    dev_err(&pdev.dev, "Cannot map MMIO registers\n");
    err = -ENOMEM;
    goto err_free_res;
    }
    pci_set_master(pdev);
    pd.dma.dev = &pdev.dev;
    err = request_irq(pdev.irq, pd_irq, IRQF_SHARED, DRV_NAME, pd);
    if (err) {
    dev_err(&pdev.dev, "Failed to request IRQ\n");
    goto err_iounmap;
    }
    pd.pool = dma_pool_create("pch_dma_desc_pool", &pdev.dev,
    sizeof(struct pch_dma_desc), 4, 0);
    if (!pd.pool) {
    dev_err(&pdev.dev, "Failed to alloc DMA descriptors\n");
    err = -ENOMEM;
    goto err_free_irq;
    }
    INIT_LIST_HEAD(&pd.dma.channels);
    for (i = 0; i < nr_channels; i++) {
    struct pch_dma_chan *pd_chan = &pd.channels[i];
    pd_chan.chan.device = &pd.dma;
    dma_cookie_init(&pd_chan.chan);
    pd_chan.membase = &regs.desc[i];
    spin_lock_init(&pd_chan.lock);
    INIT_LIST_HEAD(&pd_chan.active_list);
    INIT_LIST_HEAD(&pd_chan.queue);
    INIT_LIST_HEAD(&pd_chan.free_list);
    tasklet_setup(&pd_chan.tasklet, pdc_tasklet);
    list_add_tail(&pd_chan.chan.device_node, &pd.dma.channels);
    }
    dma_cap_zero(pd.dma.cap_mask);
    dma_cap_set(DMA_PRIVATE, pd.dma.cap_mask);
    dma_cap_set(DMA_SLAVE, pd.dma.cap_mask);
    pd.dma.device_alloc_chan_resources = pd_alloc_chan_resources;
    pd.dma.device_free_chan_resources = pd_free_chan_resources;
    pd.dma.device_tx_status = pd_tx_status;
    pd.dma.device_issue_pending = pd_issue_pending;
    pd.dma.device_prep_slave_sg = pd_prep_slave_sg;
    pd.dma.device_terminate_all = pd_device_terminate_all;
    err = dma_async_device_register(&pd.dma);
    if (err) {
    dev_err(&pdev.dev, "Failed to register DMA device\n");
    goto err_free_pool;
    }
    return 0;
    err_free_pool:
    dma_pool_destroy(pd.pool);
    err_free_irq:
    free_irq(pdev.irq, pd);
    err_iounmap:
    pci_iounmap(pdev, pd.membase);
    err_free_res:
    pci_release_regions(pdev);
    err_disable_pdev:
    pci_disable_device(pdev);
    err_free_mem:
    kfree(pd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pch_dma_remove(pdev: *mut pci_dev) {
    static void pch_dma_remove(struct pci_dev *pdev)
    {
    struct pch_dma *pd = pci_get_drvdata(pdev);
    struct pch_dma_chan *pd_chan;
    struct dma_chan *chan, *_c;
    if (pd) {
    dma_async_device_unregister(&pd.dma);
    free_irq(pdev.irq, pd);
    list_for_each_entry_safe(chan, _c, &pd.dma.channels,
    device_node) {
    pd_chan = to_pd_chan(chan);
    tasklet_kill(&pd_chan.tasklet);
    }
    dma_pool_destroy(pd.pool);
    pci_iounmap(pdev, pd.membase);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    kfree(pd);
    }
    }
// PCI Device ID of DMA device
pub const PCI_DEVICE_ID_EG20T_PCH_DMA_8CH: c_uint = 0x8810;
pub const PCI_DEVICE_ID_EG20T_PCH_DMA_4CH: c_uint = 0x8815;
pub const PCI_DEVICE_ID_ML7213_DMA1_8CH: c_uint = 0x8026;
pub const PCI_DEVICE_ID_ML7213_DMA2_8CH: c_uint = 0x802B;
pub const PCI_DEVICE_ID_ML7213_DMA3_4CH: c_uint = 0x8034;
pub const PCI_DEVICE_ID_ML7213_DMA4_12CH: c_uint = 0x8032;
pub const PCI_DEVICE_ID_ML7223_DMA1_4CH: c_uint = 0x800B;
pub const PCI_DEVICE_ID_ML7223_DMA2_4CH: c_uint = 0x800E;
pub const PCI_DEVICE_ID_ML7223_DMA3_4CH: c_uint = 0x8017;
pub const PCI_DEVICE_ID_ML7223_DMA4_4CH: c_uint = 0x803B;
pub const PCI_DEVICE_ID_ML7831_DMA1_8CH: c_uint = 0x8810;
pub const PCI_DEVICE_ID_ML7831_DMA2_4CH: c_uint = 0x8815;
    static const struct pci_device_id pch_dma_id_table[] = {
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_EG20T_PCH_DMA_8CH), 8 },
    { PCI_VDEVICE(INTEL, PCI_DEVICE_ID_EG20T_PCH_DMA_4CH), 4 },
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7213_DMA1_8CH), 8}, /* UART Video */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7213_DMA2_8CH), 8}, /* PCMIF SPI */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7213_DMA3_4CH), 4}, /* FPGA */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7213_DMA4_12CH), 12}, /* I2S */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7223_DMA1_4CH), 4}, /* UART */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7223_DMA2_4CH), 4}, /* Video SPI */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7223_DMA3_4CH), 4}, /* Security */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7223_DMA4_4CH), 4}, /* FPGA */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7831_DMA1_8CH), 8}, /* UART */
    { PCI_VDEVICE(ROHM, PCI_DEVICE_ID_ML7831_DMA2_4CH), 4}, /* SPI */
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, pch_dma_id_table);
    static SIMPLE_DEV_PM_OPS(pch_dma_pm_ops, pch_dma_suspend, pch_dma_resume);
    static struct pci_driver pch_dma_driver = {
    .name		= DRV_NAME,
    .id_table	= pch_dma_id_table,
    .probe		= pch_dma_probe,
    .remove		= pch_dma_remove,
    .driver.pm	= &pch_dma_pm_ops,
    };
    module_pci_driver(pch_dma_driver);
    MODULE_DESCRIPTION("Intel EG20T PCH / LAPIS Semicon ML7213/ML7223/ML7831 IOH "
    "DMA controller driver");
    MODULE_AUTHOR("Yong Wang <yong.y.wang@intel.com>");
    MODULE_LICENSE("GPL v2");
