//! Automatically rewritten from C to Rust
//! Source: drivers/dma/mmp_pdma.c
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
// Copyright 2012 Marvell International Ltd.
//

pub const DCSR: c_uint = 0x0000;
pub const DALGN: c_uint = 0x00a0;
pub const DINT: c_uint = 0x00f0;

pub const DCMD: c_uint = 0x020c;

pub const DRCMR_BASE: c_uint = 0x0100;
pub const DRCMR_EXT_BASE_K3: c_uint = 0x1000;
pub const DRCMR_EXT_BASE_DEFAULT: c_uint = 0x1100;
pub const DRCMR_REQ_LIMIT: c_int = 64;

pub const DRCMR_CHLNUM: c_uint = 0x1f	/* mask for Channel Number (read / write) */;
pub const DDADR_DESCADDR: c_uint = 0xfffffff0	/* Address of next descriptor (mask) */;

pub const DCMD_LENGTH: c_uint = 0x01fff		/* length mask (max = 8K - 1) */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_desc_hw {
    pub /: *mut *mut u32 ddadr; / Points to the next descriptor + flags,
    pub /: *mut *mut u32 dsadr; / DSADR value for the current transfer,
    pub /: *mut *mut u32 dtadr; / DTADR value for the current transfer,
    pub /: *mut *mut u32 dcmd; / DCMD value for the current transfer,
//
// The following 32-bit words are only used in the 64-bit, ie.
// LPAE (Long Physical Address Extension) mode.
// They are used to specify the high 32 bits of the descriptor's
// addresses.
//
    pub /: *mut *mut u32 ddadrh; / High 32-bit of DDADR,
    pub /: *mut *mut u32 dsadrh; / High 32-bit of DSADR,
    pub /: *mut *mut u32 dtadrh; / High 32-bit of DTADR,
    pub /: *mut *mut u32 rsvd; / reserved,
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_desc_sw {
    pub desc: mmp_pdma_desc_hw,
    pub node: list_head,
    pub tx_list: list_head,
    pub async_tx: dma_async_tx_descriptor,
}

    struct mmp_pdma_phy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_chan {
    pub dev: *mut device,
    pub chan: dma_chan,
    pub desc: dma_async_tx_descriptor,
    pub phy: *mut mmp_pdma_phy,
    pub dir: enum dma_transfer_direction,
    pub slave_config: dma_slave_config,
    pub channel: *mut *mut *mut mmp_pdma_desc_sw cyclic_first; / first desc_sw if,
// is in cyclic mode
// channel's basic info
    pub tasklet: tasklet_struct,
    pub dcmd: u32,
    pub drcmr: u32,
    pub dev_addr: u32,
// list for desc
    pub /: *mut *mut spinlock_t desc_lock; / Descriptor list lock,
    pub /: *mut *mut list_head chain_pending; / Link descriptors queue for pending,
    pub /: *mut *mut list_head chain_running; / Link descriptors queue for running,
    pub /: *mut *mut bool idle; / channel statue machine,
    pub byte_align: bool,
    pub /: *mut *mut *mut dma_pool desc_pool; / Descriptors pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_phy {
    pub idx: c_int,
    pub base: *mut void __iomem,
    pub vchan: *mut mmp_pdma_chan,
}

//
// struct mmp_pdma_ops - Operations for the MMP PDMA controller
//
// Hardware Register Operations (read/write hardware registers):
// @write_next_addr: Function to program address of next descriptor into
// DDADR/DDADRH
// @read_src_addr: Function to read the source address from DSADR/DSADRH
// @read_dst_addr: Function to read the destination address from DTADR/DTADRH
//
// Descriptor Memory Operations (manipulate descriptor structs in memory):
// @set_desc_next_addr: Function to set next descriptor address in descriptor
// @set_desc_src_addr: Function to set the source address in descriptor
// @set_desc_dst_addr: Function to set the destination address in descriptor
// @get_desc_src_addr: Function to get the source address from descriptor
// @get_desc_dst_addr: Function to get the destination address from descriptor
//
// Controller Configuration:
// @run_bits:   Control bits in DCSR register for channel start/stop
// @dma_width:  DMA addressing width in bits (32 or 64). Determines the
// DMA mask capability of the controller hardware.
// @drcmr_ext_base: Base DRCMR address for extended requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_ops {
// Hardware Register Operations
    pub addr): *mut *mut *mut void (write_next_addr)(struct mmp_pdma_phy phy, dma_addr_t,
    pub phy): *mut *mut u64 (read_src_addr)(struct mmp_pdma_phy,
    pub phy): *mut *mut u64 (read_dst_addr)(struct mmp_pdma_phy,
// Descriptor Memory Operations
    void (*set_desc_next_addr)(struct mmp_pdma_desc_hw *desc,
    pub addr): dma_addr_t,
    void (*set_desc_src_addr)(struct mmp_pdma_desc_hw *desc,
    pub addr): dma_addr_t,
    void (*set_desc_dst_addr)(struct mmp_pdma_desc_hw *desc,
    pub addr): dma_addr_t,
    pub desc): *const *const u64 (get_desc_src_addr)(struct mmp_pdma_desc_hw,
    pub desc): *const *const u64 (get_desc_dst_addr)(struct mmp_pdma_desc_hw,
// Controller Configuration
    pub run_bits: u32,
    pub dma_width: u32,
    pub drcmr_ext_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pdma_device {
    pub dma_channels: c_int,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub device: dma_device,
    pub phy: *mut mmp_pdma_phy,
    pub ops: *const mmp_pdma_ops,
    pub /: *mut *mut spinlock_t phy_lock; / protect alloc/free phy channels,
}

    container_of(tx, struct mmp_pdma_desc_sw, async_tx)

    container_of(lh, struct mmp_pdma_desc_sw, node)

    container_of(dchan, struct mmp_pdma_chan, chan)

    container_of(dmadev, struct mmp_pdma_device, device)
#[no_mangle]
unsafe extern "C" fn mmp_pdma_get_drcmr(pdev: *mut mmp_pdma_device, drcmr: u32) -> u32 {
    static u32 mmp_pdma_get_drcmr(struct mmp_pdma_device *pdev, u32 drcmr)
    {
    if (drcmr < DRCMR_REQ_LIMIT)
    return DRCMR_BASE + (drcmr << 2);
    return pdev.ops.drcmr_ext_base + ((drcmr - DRCMR_REQ_LIMIT) << 2);
    }
// For 32-bit PDMA
#[no_mangle]
unsafe extern "C" fn write_next_addr_32(phy: *mut mmp_pdma_phy, addr: dma_addr_t) {
    static void write_next_addr_32(struct mmp_pdma_phy *phy, dma_addr_t addr)
    {
    writel(addr, phy.base + DDADR(phy.idx));
    }
#[no_mangle]
unsafe extern "C" fn read_src_addr_32(phy: *mut mmp_pdma_phy) -> u64 {
    static u64 read_src_addr_32(struct mmp_pdma_phy *phy)
    {
    return readl(phy.base + DSADR(phy.idx));
    }
#[no_mangle]
unsafe extern "C" fn read_dst_addr_32(phy: *mut mmp_pdma_phy) -> u64 {
    static u64 read_dst_addr_32(struct mmp_pdma_phy *phy)
    {
    return readl(phy.base + DTADR(phy.idx));
    }
#[no_mangle]
unsafe extern "C" fn set_desc_next_addr_32(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_next_addr_32(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.ddadr = addr;
    }
#[no_mangle]
unsafe extern "C" fn set_desc_src_addr_32(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_src_addr_32(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.dsadr = addr;
    }
#[no_mangle]
unsafe extern "C" fn set_desc_dst_addr_32(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_dst_addr_32(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.dtadr = addr;
    }
#[no_mangle]
unsafe extern "C" fn get_desc_src_addr_32(desc: *const mmp_pdma_desc_hw) -> u64 {
    static u64 get_desc_src_addr_32(const struct mmp_pdma_desc_hw *desc)
    {
    return desc.dsadr;
    }
#[no_mangle]
unsafe extern "C" fn get_desc_dst_addr_32(desc: *const mmp_pdma_desc_hw) -> u64 {
    static u64 get_desc_dst_addr_32(const struct mmp_pdma_desc_hw *desc)
    {
    return desc.dtadr;
    }
// For 64-bit PDMA
#[no_mangle]
unsafe extern "C" fn write_next_addr_64(phy: *mut mmp_pdma_phy, addr: dma_addr_t) {
    static void write_next_addr_64(struct mmp_pdma_phy *phy, dma_addr_t addr)
    {
    writel(lower_32_bits(addr), phy.base + DDADR(phy.idx));
    writel(upper_32_bits(addr), phy.base + DDADRH(phy.idx));
    }
#[no_mangle]
unsafe extern "C" fn read_src_addr_64(phy: *mut mmp_pdma_phy) -> u64 {
    static u64 read_src_addr_64(struct mmp_pdma_phy *phy)
    {
    let mut low: u32 = readl(phy.base + DSADR(phy.idx));
    let mut high: u32 = readl(phy.base + DSADRH(phy.idx));
    return ((u64)high << 32) | low;
    }
#[no_mangle]
unsafe extern "C" fn read_dst_addr_64(phy: *mut mmp_pdma_phy) -> u64 {
    static u64 read_dst_addr_64(struct mmp_pdma_phy *phy)
    {
    let mut low: u32 = readl(phy.base + DTADR(phy.idx));
    let mut high: u32 = readl(phy.base + DTADRH(phy.idx));
    return ((u64)high << 32) | low;
    }
#[no_mangle]
unsafe extern "C" fn set_desc_next_addr_64(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_next_addr_64(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.ddadr = lower_32_bits(addr);
    desc.ddadrh = upper_32_bits(addr);
    }
#[no_mangle]
unsafe extern "C" fn set_desc_src_addr_64(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_src_addr_64(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.dsadr = lower_32_bits(addr);
    desc.dsadrh = upper_32_bits(addr);
    }
#[no_mangle]
unsafe extern "C" fn set_desc_dst_addr_64(desc: *mut mmp_pdma_desc_hw, addr: dma_addr_t) {
    static void set_desc_dst_addr_64(struct mmp_pdma_desc_hw *desc, dma_addr_t addr)
    {
    desc.dtadr = lower_32_bits(addr);
    desc.dtadrh = upper_32_bits(addr);
    }
#[no_mangle]
unsafe extern "C" fn get_desc_src_addr_64(desc: *const mmp_pdma_desc_hw) -> u64 {
    static u64 get_desc_src_addr_64(const struct mmp_pdma_desc_hw *desc)
    {
    return ((u64)desc.dsadrh << 32) | desc.dsadr;
    }
#[no_mangle]
unsafe extern "C" fn get_desc_dst_addr_64(desc: *const mmp_pdma_desc_hw) -> u64 {
    static u64 get_desc_dst_addr_64(const struct mmp_pdma_desc_hw *desc)
    {
    return ((u64)desc.dtadrh << 32) | desc.dtadr;
    }
    static int mmp_pdma_config_write(struct dma_chan *dchan,
    struct dma_slave_config *cfg,
    enum dma_transfer_direction direction);
#[no_mangle]
unsafe extern "C" fn enable_chan(phy: *mut mmp_pdma_phy) {
    static void enable_chan(struct mmp_pdma_phy *phy)
    {
    u32 reg, dalgn;
    struct mmp_pdma_device *pdev;
    if (!phy.vchan)
    return;
    pdev = to_mmp_pdma_dev(phy.vchan.chan.device);
    reg = mmp_pdma_get_drcmr(pdev, phy.vchan.drcmr);
    writel(DRCMR_MAPVLD | phy.idx, phy.base + reg);
    dalgn = readl(phy.base + DALGN);
    if (phy.vchan.byte_align)
    dalgn |= 1 << phy.idx;
    else
    dalgn &= ~(1 << phy.idx);
    writel(dalgn, phy.base + DALGN);
    reg = (phy.idx << 2) + DCSR;
    writel(readl(phy.base + reg) | pdev.ops.run_bits,
    phy.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn disable_chan(phy: *mut mmp_pdma_phy) {
    static void disable_chan(struct mmp_pdma_phy *phy)
    {
    u32 reg, dcsr;
    if (!phy)
    return;
    reg = (phy.idx << 2) + DCSR;
    dcsr = readl(phy.base + reg);
    if (phy.vchan) {
    struct mmp_pdma_device *pdev;
    pdev = to_mmp_pdma_dev(phy.vchan.chan.device);
    writel(dcsr & ~pdev.ops.run_bits, phy.base + reg);
    } else {
// If no vchan, just clear the RUN bit
    writel(dcsr & ~DCSR_RUN, phy.base + reg);
    }
    }
#[no_mangle]
unsafe extern "C" fn clear_chan_irq(phy: *mut mmp_pdma_phy) -> c_int {
    static int clear_chan_irq(struct mmp_pdma_phy *phy)
    {
    u32 dcsr;
    let mut dint: u32 = readl(phy.base + DINT);
    let mut reg: u32 = (phy.idx << 2) + DCSR;
    if (!(dint & BIT(phy.idx)))
    return -EAGAIN;
// clear irq
    dcsr = readl(phy.base + reg);
    writel(dcsr, phy.base + reg);
    if ((dcsr & DCSR_BUSERR) && (phy.vchan))
    dev_warn(phy.vchan.dev, "DCSR_BUSERR\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_chan_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mmp_pdma_chan_handler(int irq, void *dev_id)
    {
    struct mmp_pdma_phy *phy = dev_id;
    if (clear_chan_irq(phy) != 0)
    return IRQ_NONE;
    tasklet_schedule(&phy.vchan.tasklet);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_int_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mmp_pdma_int_handler(int irq, void *dev_id)
    {
    struct mmp_pdma_device *pdev = dev_id;
    struct mmp_pdma_phy *phy;
    let mut dint: u32 = readl(pdev.base + DINT);
    int i, ret;
    let mut irq_num: c_int = 0;
    while (dint) {
    i = __ffs(dint);
// only handle interrupts belonging to pdma driver
    if (i >= pdev.dma_channels)
    break;
    dint &= (dint - 1);
    phy = &pdev.phy[i];
    ret = mmp_pdma_chan_handler(irq, phy);
    if (ret == IRQ_HANDLED)
    irq_num++;
    }
    if (irq_num)
    return IRQ_HANDLED;
    return IRQ_NONE;
    }
// lookup free phy channel as descending priority
    static struct mmp_pdma_phy *lookup_phy(struct mmp_pdma_chan *pchan)
    {
    int prio, i;
    struct mmp_pdma_device *pdev = to_mmp_pdma_dev(pchan.chan.device);
    struct mmp_pdma_phy *phy, *found = core::ptr::null_mut();
    unsigned long flags;
//
// dma channel priorities
// ch 0 - 3,  16 - 19  <--> (0)
// ch 4 - 7,  20 - 23  <--> (1)
// ch 8 - 11, 24 - 27  <--> (2)
// ch 12 - 15, 28 - 31  <--> (3)
//
    spin_lock_irqsave(&pdev.phy_lock, flags);
    for (prio = 0; prio <= ((pdev.dma_channels - 1) & 0xf) >> 2; prio++) {
    for (i = 0; i < pdev.dma_channels; i++) {
    if (prio != (i & 0xf) >> 2)
    continue;
    phy = &pdev.phy[i];
    if (!phy.vchan) {
    phy.vchan = pchan;
    found = phy;
    goto out_unlock;
    }
    }
    }
    out_unlock:
    spin_unlock_irqrestore(&pdev.phy_lock, flags);
    return found;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_free_phy(pchan: *mut mmp_pdma_chan) {
    static void mmp_pdma_free_phy(struct mmp_pdma_chan *pchan)
    {
    struct mmp_pdma_device *pdev = to_mmp_pdma_dev(pchan.chan.device);
    unsigned long flags;
    u32 reg;
    if (!pchan.phy)
    return;
// clear the channel mapping in DRCMR
    reg = mmp_pdma_get_drcmr(pdev, pchan.drcmr);
    writel(0, pchan.phy.base + reg);
    spin_lock_irqsave(&pdev.phy_lock, flags);
    pchan.phy.vchan = core::ptr::null_mut();
    pchan.phy = core::ptr::null_mut();
    spin_unlock_irqrestore(&pdev.phy_lock, flags);
    }
//
// start_pending_queue - transfer any pending transactions
// pending list ==> running list
//
#[no_mangle]
unsafe extern "C" fn start_pending_queue(chan: *mut mmp_pdma_chan) {
    static void start_pending_queue(struct mmp_pdma_chan *chan)
    {
    struct mmp_pdma_desc_sw *desc;
    struct mmp_pdma_device *pdev = to_mmp_pdma_dev(chan.chan.device);
// still in running, irq will start the pending list
    if (!chan.idle) {
    dev_dbg(chan.dev, "DMA controller still busy\n");
    return;
    }
    if (list_empty(&chan.chain_pending)) {
// chance to re-fetch phy channel with higher prio
    mmp_pdma_free_phy(chan);
    dev_dbg(chan.dev, "no pending list\n");
    return;
    }
    if (!chan.phy) {
    chan.phy = lookup_phy(chan);
    if (!chan.phy) {
    dev_dbg(chan.dev, "no free dma channel\n");
    return;
    }
    }
//
// pending -> running
// reintilize pending list
//
    desc = list_first_entry(&chan.chain_pending,
    struct mmp_pdma_desc_sw, node);
    list_splice_tail_init(&chan.chain_pending, &chan.chain_running);
//
// Program the descriptor's address into the DMA controller,
// then start the DMA transaction
//
    pdev.ops.write_next_addr(chan.phy, desc.async_tx.phys);
    enable_chan(chan.phy);
    chan.idle = false;
    }
// desc->tx_list ==> pending list
#[no_mangle]
unsafe extern "C" fn mmp_pdma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t mmp_pdma_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(tx.chan);
    struct mmp_pdma_desc_sw *desc = tx_to_mmp_pdma_desc(tx);
    struct mmp_pdma_desc_sw *child;
    unsigned long flags;
    let mut cookie: dma_cookie_t = -EBUSY;
    spin_lock_irqsave(&chan.desc_lock, flags);
    list_for_each_entry(child, &desc.tx_list, node) {
    cookie = dma_cookie_assign(&child.async_tx);
    }
// softly link to pending list - desc->tx_list ==> pending list
    list_splice_tail_init(&desc.tx_list, &chan.chain_pending);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    return cookie;
    }
    static struct mmp_pdma_desc_sw *
    mmp_pdma_alloc_descriptor(struct mmp_pdma_chan *chan)
    {
    struct mmp_pdma_desc_sw *desc;
    dma_addr_t pdesc;
    desc = dma_pool_zalloc(chan.desc_pool, GFP_ATOMIC, &pdesc);
    if (!desc) {
    dev_err(chan.dev, "out of memory for link descriptor\n");
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&desc.tx_list);
    dma_async_tx_descriptor_init(&desc.async_tx, &chan.chan);
// each desc has submit
    desc.async_tx.tx_submit = mmp_pdma_tx_submit;
    desc.async_tx.phys = pdesc;
    return desc;
    }
//
// mmp_pdma_alloc_chan_resources - Allocate resources for DMA channel.
//
// This function will create a dma pool for descriptor allocation.
// Request irq only when channel is requested
// Return - The number of allocated descriptors.
//
#[no_mangle]
unsafe extern "C" fn mmp_pdma_alloc_chan_resources(dchan: *mut dma_chan) -> c_int {
    static int mmp_pdma_alloc_chan_resources(struct dma_chan *dchan)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    if (chan.desc_pool)
    return 1;
    chan.desc_pool = dma_pool_create(dev_name(&dchan.dev.device),
    chan.dev,
    sizeof(struct mmp_pdma_desc_sw),
    __alignof__(struct mmp_pdma_desc_sw),
    0);
    if (!chan.desc_pool) {
    dev_err(chan.dev, "unable to allocate descriptor pool\n");
    return -ENOMEM;
    }
    mmp_pdma_free_phy(chan);
    chan.idle = true;
    chan.dev_addr = 0;
    return 1;
    }
    static void mmp_pdma_free_desc_list(struct mmp_pdma_chan *chan,
    struct list_head *list)
    {
    struct mmp_pdma_desc_sw *desc, *_desc;
    list_for_each_entry_safe(desc, _desc, list, node) {
    list_del(&desc.node);
    dma_pool_free(chan.desc_pool, desc, desc.async_tx.phys);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_free_chan_resources(dchan: *mut dma_chan) {
    static void mmp_pdma_free_chan_resources(struct dma_chan *dchan)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    unsigned long flags;
    spin_lock_irqsave(&chan.desc_lock, flags);
    mmp_pdma_free_desc_list(chan, &chan.chain_pending);
    mmp_pdma_free_desc_list(chan, &chan.chain_running);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    dma_pool_destroy(chan.desc_pool);
    chan.desc_pool = core::ptr::null_mut();
    chan.idle = true;
    chan.dev_addr = 0;
    mmp_pdma_free_phy(chan);
    return;
    }
    static struct dma_async_tx_descriptor *
    mmp_pdma_prep_memcpy(struct dma_chan *dchan,
    dma_addr_t dma_dst, dma_addr_t dma_src,
    size_t len, unsigned long flags)
    {
    struct mmp_pdma_chan *chan;
    struct mmp_pdma_device *pdev;
    struct mmp_pdma_desc_sw *first = core::ptr::null_mut(), *prev = core::ptr::null_mut(), *new;
    let mut copy: usize = 0;
    if (!dchan || !len)
    return core::ptr::null_mut();
    pdev = to_mmp_pdma_dev(dchan.device);
    chan = to_mmp_pdma_chan(dchan);
    chan.byte_align = false;
    if (!chan.dir) {
    chan.dir = DMA_MEM_TO_MEM;
    chan.dcmd = DCMD_INCTRGADDR | DCMD_INCSRCADDR;
    chan.dcmd |= DCMD_BURST32;
    }
    do {
// Allocate the link descriptor from DMA pool
    new = mmp_pdma_alloc_descriptor(chan);
    if (!new) {
    dev_err(chan.dev, "no memory for desc\n");
    goto fail;
    }
    copy = min_t(size_t, len, PDMA_MAX_DESC_BYTES);
    if (dma_src & 0x7 || dma_dst & 0x7)
    chan.byte_align = true;
    new.desc.dcmd = chan.dcmd | (DCMD_LENGTH & copy);
    pdev.ops.set_desc_src_addr(&new.desc, dma_src);
    pdev.ops.set_desc_dst_addr(&new.desc, dma_dst);
    if (!first)
    first = new;
    else
    pdev.ops.set_desc_next_addr(&prev.desc,
    new.async_tx.phys);
    new.async_tx.cookie = 0;
    async_tx_ack(&new.async_tx);
    prev = new;
    len -= copy;
    if (chan.dir == DMA_MEM_TO_DEV) {
    dma_src += copy;
    } else if (chan.dir == DMA_DEV_TO_MEM) {
    dma_dst += copy;
    } else if (chan.dir == DMA_MEM_TO_MEM) {
    dma_src += copy;
    dma_dst += copy;
    }
// Insert the link descriptor to the LD ring
    list_add_tail(&new.node, &first.tx_list);
    } while (len);
    first.async_tx.flags = flags; /* client is in control of this ack */
    first.async_tx.cookie = -EBUSY;
// last desc and fire IRQ
    new.desc.ddadr = DDADR_STOP;
    new.desc.dcmd |= DCMD_ENDIRQEN;
    chan.cyclic_first = core::ptr::null_mut();
    return &first.async_tx;
    fail:
    if (first)
    mmp_pdma_free_desc_list(chan, &first.tx_list);
    return core::ptr::null_mut();
    }
    static struct dma_async_tx_descriptor *
    mmp_pdma_prep_slave_sg(struct dma_chan *dchan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    struct mmp_pdma_device *pdev = to_mmp_pdma_dev(dchan.device);
    struct mmp_pdma_desc_sw *first = core::ptr::null_mut(), *prev = core::ptr::null_mut(), *new = core::ptr::null_mut();
    size_t len, avail;
    struct scatterlist *sg;
    dma_addr_t addr;
    int i;
    if ((sgl == core::ptr::null_mut()) || (sg_len == 0))
    return core::ptr::null_mut();
    chan.byte_align = false;
    mmp_pdma_config_write(dchan, &chan.slave_config, dir);
    for_each_sg(sgl, sg, sg_len, i) {
    addr = sg_dma_address(sg);
    avail = sg_dma_len(sgl);
    do {
    len = min_t(size_t, avail, PDMA_MAX_DESC_BYTES);
    if (addr & 0x7)
    chan.byte_align = true;
// allocate and populate the descriptor
    new = mmp_pdma_alloc_descriptor(chan);
    if (!new) {
    dev_err(chan.dev, "no memory for desc\n");
    goto fail;
    }
    new.desc.dcmd = chan.dcmd | (DCMD_LENGTH & len);
    if (dir == DMA_MEM_TO_DEV) {
    pdev.ops.set_desc_src_addr(&new.desc, addr);
    new.desc.dtadr = chan.dev_addr;
    } else {
    new.desc.dsadr = chan.dev_addr;
    pdev.ops.set_desc_dst_addr(&new.desc, addr);
    }
    if (!first)
    first = new;
    else
    pdev.ops.set_desc_next_addr(&prev.desc,
    new.async_tx.phys);
    new.async_tx.cookie = 0;
    async_tx_ack(&new.async_tx);
    prev = new;
// Insert the link descriptor to the LD ring
    list_add_tail(&new.node, &first.tx_list);
// update metadata
    addr += len;
    avail -= len;
    } while (avail);
    }
    first.async_tx.cookie = -EBUSY;
    first.async_tx.flags = flags;
// last desc and fire IRQ
    new.desc.ddadr = DDADR_STOP;
    new.desc.dcmd |= DCMD_ENDIRQEN;
    chan.dir = dir;
    chan.cyclic_first = core::ptr::null_mut();
    return &first.async_tx;
    fail:
    if (first)
    mmp_pdma_free_desc_list(chan, &first.tx_list);
    return core::ptr::null_mut();
    }
    static struct dma_async_tx_descriptor *
    mmp_pdma_prep_dma_cyclic(struct dma_chan *dchan,
    dma_addr_t buf_addr, size_t len, size_t period_len,
    enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct mmp_pdma_chan *chan;
    struct mmp_pdma_device *pdev;
    struct mmp_pdma_desc_sw *first = core::ptr::null_mut(), *prev = core::ptr::null_mut(), *new;
    dma_addr_t dma_src, dma_dst;
    if (!dchan || !len || !period_len)
    return core::ptr::null_mut();
    pdev = to_mmp_pdma_dev(dchan.device);
// the buffer length must be a multiple of period_len
    if (len % period_len != 0)
    return core::ptr::null_mut();
    if (period_len > PDMA_MAX_DESC_BYTES)
    return core::ptr::null_mut();
    chan = to_mmp_pdma_chan(dchan);
    mmp_pdma_config_write(dchan, &chan.slave_config, direction);
    switch (direction) {
    case DMA_MEM_TO_DEV:
    dma_src = buf_addr;
    dma_dst = chan.dev_addr;
    break;
    case DMA_DEV_TO_MEM:
    dma_dst = buf_addr;
    dma_src = chan.dev_addr;
    break;
    default:
    dev_err(chan.dev, "Unsupported direction for cyclic DMA\n");
    return core::ptr::null_mut();
    }
    chan.dir = direction;
    do {
// Allocate the link descriptor from DMA pool
    new = mmp_pdma_alloc_descriptor(chan);
    if (!new) {
    dev_err(chan.dev, "no memory for desc\n");
    goto fail;
    }
    new.desc.dcmd = (chan.dcmd | DCMD_ENDIRQEN |
    (DCMD_LENGTH & period_len));
    pdev.ops.set_desc_src_addr(&new.desc, dma_src);
    pdev.ops.set_desc_dst_addr(&new.desc, dma_dst);
    if (!first)
    first = new;
    else
    pdev.ops.set_desc_next_addr(&prev.desc,
    new.async_tx.phys);
    new.async_tx.cookie = 0;
    async_tx_ack(&new.async_tx);
    prev = new;
    len -= period_len;
    if (chan.dir == DMA_MEM_TO_DEV)
    dma_src += period_len;
    else
    dma_dst += period_len;
// Insert the link descriptor to the LD ring
    list_add_tail(&new.node, &first.tx_list);
    } while (len);
    first.async_tx.flags = flags; /* client is in control of this ack */
    first.async_tx.cookie = -EBUSY;
// make the cyclic link
    pdev.ops.set_desc_next_addr(&new.desc, first.async_tx.phys);
    chan.cyclic_first = first;
    return &first.async_tx;
    fail:
    if (first)
    mmp_pdma_free_desc_list(chan, &first.tx_list);
    return core::ptr::null_mut();
    }
    static int mmp_pdma_config_write(struct dma_chan *dchan,
    struct dma_slave_config *cfg,
    enum dma_transfer_direction direction)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    let mut maxburst: u32 = 0, addr = 0;
    let mut width: enum dma_slave_buswidth = DMA_SLAVE_BUSWIDTH_UNDEFINED;
    if (!dchan)
    return -EINVAL;
    if (direction == DMA_DEV_TO_MEM) {
    chan.dcmd = DCMD_INCTRGADDR | DCMD_FLOWSRC;
    maxburst = cfg.src_maxburst;
    width = cfg.src_addr_width;
    addr = cfg.src_addr;
    } else if (direction == DMA_MEM_TO_DEV) {
    chan.dcmd = DCMD_INCSRCADDR | DCMD_FLOWTRG;
    maxburst = cfg.dst_maxburst;
    width = cfg.dst_addr_width;
    addr = cfg.dst_addr;
    }
    if (width == DMA_SLAVE_BUSWIDTH_1_BYTE)
    chan.dcmd |= DCMD_WIDTH1;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_SLAVE_BUSWIDTH_2_BYTES: width ==) -> else {
    else if (width == DMA_SLAVE_BUSWIDTH_2_BYTES)
    chan.dcmd |= DCMD_WIDTH2;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_SLAVE_BUSWIDTH_4_BYTES: width ==) -> else {
    else if (width == DMA_SLAVE_BUSWIDTH_4_BYTES)
    chan.dcmd |= DCMD_WIDTH4;
    if (maxburst == 8)
    chan.dcmd |= DCMD_BURST8;
#[no_mangle]
pub unsafe extern "C" fn if(16: maxburst ==) -> else {
    else if (maxburst == 16)
    chan.dcmd |= DCMD_BURST16;
#[no_mangle]
pub unsafe extern "C" fn if(32: maxburst ==) -> else {
    else if (maxburst == 32)
    chan.dcmd |= DCMD_BURST32;
    chan.dir = direction;
    chan.dev_addr = addr;
    return 0;
    }
    static int mmp_pdma_config(struct dma_chan *dchan,
    struct dma_slave_config *cfg)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    memcpy(&chan.slave_config, cfg, sizeof(*cfg));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_terminate_all(dchan: *mut dma_chan) -> c_int {
    static int mmp_pdma_terminate_all(struct dma_chan *dchan)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    unsigned long flags;
    if (!dchan)
    return -EINVAL;
    disable_chan(chan.phy);
    mmp_pdma_free_phy(chan);
    spin_lock_irqsave(&chan.desc_lock, flags);
    mmp_pdma_free_desc_list(chan, &chan.chain_pending);
    mmp_pdma_free_desc_list(chan, &chan.chain_running);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    chan.idle = true;
    return 0;
    }
    static unsigned int mmp_pdma_residue(struct mmp_pdma_chan *chan,
    dma_cookie_t cookie)
    {
    struct mmp_pdma_desc_sw *sw;
    struct mmp_pdma_device *pdev = to_mmp_pdma_dev(chan.chan.device);
    unsigned long flags;
    u64 curr;
    let mut residue: u32 = 0;
    let mut passed: bool = false;
    let mut cyclic: bool = chan.cyclic_first != core::ptr::null_mut();
//
// If the channel does not have a phy pointer anymore, it has already
// been completed. Therefore, its residue is 0.
//
    if (!chan.phy)
    return 0;
    if (chan.dir == DMA_DEV_TO_MEM)
    curr = pdev.ops.read_dst_addr(chan.phy);
    else
    curr = pdev.ops.read_src_addr(chan.phy);
    spin_lock_irqsave(&chan.desc_lock, flags);
    list_for_each_entry(sw, &chan.chain_running, node) {
    u64 start, end;
    u32 len;
    if (chan.dir == DMA_DEV_TO_MEM)
    start = pdev.ops.get_desc_dst_addr(&sw.desc);
    else
    start = pdev.ops.get_desc_src_addr(&sw.desc);
    len = sw.desc.dcmd & DCMD_LENGTH;
    end = start + len;
//
// 'passed' will be latched once we found the descriptor which
// lies inside the boundaries of the curr pointer. All
// descriptors that occur in the list _after_ we found that
// partially handled descriptor are still to be processed and
// are hence added to the residual bytes counter.
//
    if (passed) {
    residue += len;
    } else if (curr >= start && curr <= end) {
    residue += (u32)(end - curr);
    passed = true;
    }
//
// Descriptors that have the ENDIRQEN bit set mark the end of a
// transaction chain, and the cookie assigned with it has been
// returned previously from mmp_pdma_tx_submit().
//
// In case we have multiple transactions in the running chain,
// and the cookie does not match the one the user asked us
// about, reset the state variables and start over.
//
// This logic does not apply to cyclic transactions, where all
// descriptors have the ENDIRQEN bit set, and for which we
// can't have multiple transactions on one channel anyway.
//
    if (cyclic || !(sw.desc.dcmd & DCMD_ENDIRQEN))
    continue;
    if (sw.async_tx.cookie == cookie) {
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    return residue;
    } else {
    residue = 0;
    passed = false;
    }
    }
    spin_unlock_irqrestore(&chan.desc_lock, flags);
// We should only get here in case of cyclic transactions
    return residue;
    }
    static enum dma_status mmp_pdma_tx_status(struct dma_chan *dchan,
    dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    enum dma_status ret;
    ret = dma_cookie_status(dchan, cookie, txstate);
    if (likely(ret != DMA_ERROR))
    dma_set_residue(txstate, mmp_pdma_residue(chan, cookie));
    return ret;
    }
//
// mmp_pdma_issue_pending - Issue the DMA start command
// pending list ==> running list
//
#[no_mangle]
unsafe extern "C" fn mmp_pdma_issue_pending(dchan: *mut dma_chan) {
    static void mmp_pdma_issue_pending(struct dma_chan *dchan)
    {
    struct mmp_pdma_chan *chan = to_mmp_pdma_chan(dchan);
    unsigned long flags;
    spin_lock_irqsave(&chan.desc_lock, flags);
    start_pending_queue(chan);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    }
//
// dma_do_tasklet
// Do call back
// Start pending list
//
#[no_mangle]
unsafe extern "C" fn dma_do_tasklet(t: *mut tasklet_struct) {
    static void dma_do_tasklet(struct tasklet_struct *t)
    {
    struct mmp_pdma_chan *chan = from_tasklet(chan, t, tasklet);
    struct mmp_pdma_desc_sw *desc, *_desc;
    LIST_HEAD(chain_cleanup);
    unsigned long flags;
    struct dmaengine_desc_callback cb;
    if (chan.cyclic_first) {
    spin_lock_irqsave(&chan.desc_lock, flags);
    desc = chan.cyclic_first;
    dmaengine_desc_get_callback(&desc.async_tx, &cb);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    return;
    }
// submit pending list; callback for each desc; free desc
    spin_lock_irqsave(&chan.desc_lock, flags);
    list_for_each_entry_safe(desc, _desc, &chan.chain_running, node) {
//
// move the descriptors to a temporary list so we can drop
// the lock during the entire cleanup operation
//
    list_move(&desc.node, &chain_cleanup);
//
// Look for the first list entry which has the ENDIRQEN flag
// set. That is the descriptor we got an interrupt for, so
// complete that transaction and its cookie.
//
    if (desc.desc.dcmd & DCMD_ENDIRQEN) {
    let mut cookie: dma_cookie_t = desc.async_tx.cookie;
    dma_cookie_complete(&desc.async_tx);
    dev_dbg(chan.dev, "completed_cookie=%d\n", cookie);
    break;
    }
    }
//
// The hardware is idle and ready for more when the
// chain_running list is empty.
//
    chan.idle = list_empty(&chan.chain_running);
// Start any pending transactions automatically
    start_pending_queue(chan);
    spin_unlock_irqrestore(&chan.desc_lock, flags);
// Run the callback for each descriptor, in order
    list_for_each_entry_safe(desc, _desc, &chain_cleanup, node) {
    struct dma_async_tx_descriptor *txd = &desc.async_tx;
// Remove from the list of transactions
    list_del(&desc.node);
// Run the link descriptor callback function
    dmaengine_desc_get_callback(txd, &cb);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    dma_pool_free(chan.desc_pool, desc, txd.phys);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_remove(op: *mut platform_device) {
    static void mmp_pdma_remove(struct platform_device *op)
    {
    struct mmp_pdma_device *pdev = platform_get_drvdata(op);
    struct mmp_pdma_phy *phy;
    int i, irq = 0, irq_num = 0;
    if (op.dev.of_node)
    of_dma_controller_free(op.dev.of_node);
    for (i = 0; i < pdev.dma_channels; i++) {
    if (platform_get_irq(op, i) > 0)
    irq_num++;
    }
    if (irq_num != pdev.dma_channels) {
    irq = platform_get_irq(op, 0);
    devm_free_irq(&op.dev, irq, pdev);
    } else {
    for (i = 0; i < pdev.dma_channels; i++) {
    phy = &pdev.phy[i];
    irq = platform_get_irq(op, i);
    devm_free_irq(&op.dev, irq, phy);
    }
    }
    dma_async_device_unregister(&pdev.device);
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_chan_init(pdev: *mut mmp_pdma_device, idx: c_int, irq: c_int) -> c_int {
    static int mmp_pdma_chan_init(struct mmp_pdma_device *pdev, int idx, int irq)
    {
    struct mmp_pdma_phy *phy  = &pdev.phy[idx];
    struct mmp_pdma_chan *chan;
    int ret;
    chan = devm_kzalloc(pdev.dev, sizeof(*chan), GFP_KERNEL);
    if (chan == core::ptr::null_mut())
    return -ENOMEM;
    phy.idx = idx;
    phy.base = pdev.base;
    if (irq) {
    ret = devm_request_irq(pdev.dev, irq, mmp_pdma_chan_handler,
    IRQF_SHARED, "pdma", phy);
    if (ret) {
    dev_err(pdev.dev, "channel request irq fail!\n");
    return ret;
    }
    }
    spin_lock_init(&chan.desc_lock);
    chan.dev = pdev.dev;
    chan.chan.device = &pdev.device;
    tasklet_setup(&chan.tasklet, dma_do_tasklet);
    INIT_LIST_HEAD(&chan.chain_pending);
    INIT_LIST_HEAD(&chan.chain_running);
// register virt channel to dma engine
    list_add_tail(&chan.chan.device_node, &pdev.device.channels);
    return 0;
    }
    static const struct mmp_pdma_ops marvell_pdma_v1_ops = {
    .write_next_addr = write_next_addr_32,
    .read_src_addr = read_src_addr_32,
    .read_dst_addr = read_dst_addr_32,
    .set_desc_next_addr = set_desc_next_addr_32,
    .set_desc_src_addr = set_desc_src_addr_32,
    .set_desc_dst_addr = set_desc_dst_addr_32,
    .get_desc_src_addr = get_desc_src_addr_32,
    .get_desc_dst_addr = get_desc_dst_addr_32,
    .run_bits = (DCSR_RUN),
    .dma_width = 32,
    .drcmr_ext_base = DRCMR_EXT_BASE_DEFAULT,
    };
    static const struct mmp_pdma_ops spacemit_k1_pdma_ops = {
    .write_next_addr = write_next_addr_64,
    .read_src_addr = read_src_addr_64,
    .read_dst_addr = read_dst_addr_64,
    .set_desc_next_addr = set_desc_next_addr_64,
    .set_desc_src_addr = set_desc_src_addr_64,
    .set_desc_dst_addr = set_desc_dst_addr_64,
    .get_desc_src_addr = get_desc_src_addr_64,
    .get_desc_dst_addr = get_desc_dst_addr_64,
    .run_bits = (DCSR_RUN | DCSR_LPAEEN),
    .dma_width = 64,
    .drcmr_ext_base = DRCMR_EXT_BASE_DEFAULT,
    };
    static const struct mmp_pdma_ops spacemit_k3_pdma_ops = {
    .write_next_addr = write_next_addr_64,
    .read_src_addr = read_src_addr_64,
    .read_dst_addr = read_dst_addr_64,
    .set_desc_next_addr = set_desc_next_addr_64,
    .set_desc_src_addr = set_desc_src_addr_64,
    .set_desc_dst_addr = set_desc_dst_addr_64,
    .get_desc_src_addr = get_desc_src_addr_64,
    .get_desc_dst_addr = get_desc_dst_addr_64,
    .run_bits = (DCSR_RUN | DCSR_LPAEEN | DCSR_EORIRQEN | DCSR_EORSTOPEN),
    .dma_width = 64,
    .drcmr_ext_base = DRCMR_EXT_BASE_K3,
    };
    static const struct of_device_id mmp_pdma_dt_ids[] = {
    {
    .compatible = "marvell,pdma-1.0",
    .data = &marvell_pdma_v1_ops
    }, {
    .compatible = "spacemit,k1-pdma",
    .data = &spacemit_k1_pdma_ops
    }, {
    .compatible = "spacemit,k3-pdma",
    .data = &spacemit_k3_pdma_ops
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, mmp_pdma_dt_ids);
    static struct dma_chan *mmp_pdma_dma_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct mmp_pdma_device *d = ofdma.of_dma_data;
    struct dma_chan *chan;
    chan = dma_get_any_slave_channel(&d.device);
    if (!chan)
    return core::ptr::null_mut();
    to_mmp_pdma_chan(chan).drcmr = dma_spec.args[0];
    return chan;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pdma_probe(op: *mut platform_device) -> c_int {
    static int mmp_pdma_probe(struct platform_device *op)
    {
    struct mmp_pdma_device *pdev;
    struct mmp_dma_platdata *pdata = dev_get_platdata(&op.dev);
    struct clk *clk;
    struct reset_control *rst;
    int i, ret, irq = 0;
    let mut dma_channels: c_int = 0, irq_num = 0;
    const enum dma_slave_buswidth widths =
    DMA_SLAVE_BUSWIDTH_1_BYTE   | DMA_SLAVE_BUSWIDTH_2_BYTES |
    DMA_SLAVE_BUSWIDTH_4_BYTES;
    pdev = devm_kzalloc(&op.dev, sizeof(*pdev), GFP_KERNEL);
    if (!pdev)
    return -ENOMEM;
    pdev.dev = &op.dev;
    spin_lock_init(&pdev.phy_lock);
    pdev.base = devm_platform_ioremap_resource(op, 0);
    if (IS_ERR(pdev.base))
    return PTR_ERR(pdev.base);
    clk = devm_clk_get_optional_enabled(pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    rst = devm_reset_control_get_optional_exclusive_deasserted(pdev.dev,
    core::ptr::null_mut());
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    pdev.ops = of_device_get_match_data(&op.dev);
    if (!pdev.ops)
    return -ENODEV;
    if (pdev.dev.of_node) {
// Parse new and deprecated dma-channels properties
    if (of_property_read_u32(pdev.dev.of_node, "dma-channels",
    &dma_channels))
    of_property_read_u32(pdev.dev.of_node, "#dma-channels",
    &dma_channels);
    } else if (pdata && pdata.dma_channels) {
    dma_channels = pdata.dma_channels;
    } else {
    dma_channels = 32;	/* default 32 channel */
    }
    pdev.dma_channels = dma_channels;
    for (i = 0; i < dma_channels; i++) {
    if (platform_get_irq_optional(op, i) > 0)
    irq_num++;
    }
    pdev.phy = devm_kcalloc(pdev.dev, dma_channels, sizeof(*pdev.phy),
    GFP_KERNEL);
    if (pdev.phy == core::ptr::null_mut())
    return -ENOMEM;
    INIT_LIST_HEAD(&pdev.device.channels);
    if (irq_num != dma_channels) {
// all chan share one irq, demux inside
    irq = platform_get_irq(op, 0);
    ret = devm_request_irq(pdev.dev, irq, mmp_pdma_int_handler,
    IRQF_SHARED, "pdma", pdev);
    if (ret)
    return ret;
    }
    for (i = 0; i < dma_channels; i++) {
    irq = (irq_num != dma_channels) ? 0 : platform_get_irq(op, i);
    ret = mmp_pdma_chan_init(pdev, i, irq);
    if (ret)
    return ret;
    }
    dma_cap_set(DMA_SLAVE, pdev.device.cap_mask);
    dma_cap_set(DMA_MEMCPY, pdev.device.cap_mask);
    dma_cap_set(DMA_CYCLIC, pdev.device.cap_mask);
    dma_cap_set(DMA_PRIVATE, pdev.device.cap_mask);
    pdev.device.dev = &op.dev;
    pdev.device.device_alloc_chan_resources = mmp_pdma_alloc_chan_resources;
    pdev.device.device_free_chan_resources = mmp_pdma_free_chan_resources;
    pdev.device.device_tx_status = mmp_pdma_tx_status;
    pdev.device.device_prep_dma_memcpy = mmp_pdma_prep_memcpy;
    pdev.device.device_prep_slave_sg = mmp_pdma_prep_slave_sg;
    pdev.device.device_prep_dma_cyclic = mmp_pdma_prep_dma_cyclic;
    pdev.device.device_issue_pending = mmp_pdma_issue_pending;
    pdev.device.device_config = mmp_pdma_config;
    pdev.device.device_terminate_all = mmp_pdma_terminate_all;
    pdev.device.copy_align = DMAENGINE_ALIGN_8_BYTES;
    pdev.device.src_addr_widths = widths;
    pdev.device.dst_addr_widths = widths;
    pdev.device.directions = BIT(DMA_MEM_TO_DEV) | BIT(DMA_DEV_TO_MEM);
    pdev.device.residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
// Set DMA mask based on controller hardware capabilities
    dma_set_mask_and_coherent(pdev.dev,
    DMA_BIT_MASK(pdev.ops.dma_width));
    ret = dma_async_device_register(&pdev.device);
    if (ret) {
    dev_err(pdev.device.dev, "unable to register\n");
    return ret;
    }
    if (op.dev.of_node) {
// Device-tree DMA controller registration
    ret = of_dma_controller_register(op.dev.of_node,
    mmp_pdma_dma_xlate, pdev);
    if (ret < 0) {
    dev_err(&op.dev, "of_dma_controller_register failed\n");
    dma_async_device_unregister(&pdev.device);
    return ret;
    }
    }
    platform_set_drvdata(op, pdev);
    dev_info(pdev.device.dev, "initialized %d channels\n", dma_channels);
    return 0;
    }
    static const struct platform_device_id mmp_pdma_id_table[] = {
    { "mmp-pdma", },
    { },
    };
    static struct platform_driver mmp_pdma_driver = {
    .driver		= {
    .name	= "mmp-pdma",
    .of_match_table = mmp_pdma_dt_ids,
    },
    .id_table	= mmp_pdma_id_table,
    .probe		= mmp_pdma_probe,
    .remove		= mmp_pdma_remove,
    };
    module_platform_driver(mmp_pdma_driver);
    MODULE_DESCRIPTION("MARVELL MMP Peripheral DMA Driver");
    MODULE_AUTHOR("Marvell International Ltd.");
    MODULE_LICENSE("GPL v2");
