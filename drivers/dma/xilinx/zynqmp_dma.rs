//! Automatically rewritten from C to Rust
//! Source: drivers/dma/xilinx/zynqmp_dma.c
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
// DMA driver for Xilinx ZynqMP DMA Engine
//
// Copyright (C) 2016 Xilinx, Inc. All rights reserved.
//

// Register Offsets

pub const ZYNQMP_DMA_CTRL0: c_uint = 0x110;
pub const ZYNQMP_DMA_CTRL1: c_uint = 0x114;
pub const ZYNQMP_DMA_DATA_ATTR: c_uint = 0x120;
pub const ZYNQMP_DMA_DSCR_ATTR: c_uint = 0x124;
pub const ZYNQMP_DMA_SRC_DSCR_WRD0: c_uint = 0x128;
pub const ZYNQMP_DMA_SRC_DSCR_WRD1: c_uint = 0x12C;
pub const ZYNQMP_DMA_SRC_DSCR_WRD2: c_uint = 0x130;
pub const ZYNQMP_DMA_SRC_DSCR_WRD3: c_uint = 0x134;
pub const ZYNQMP_DMA_DST_DSCR_WRD0: c_uint = 0x138;
pub const ZYNQMP_DMA_DST_DSCR_WRD1: c_uint = 0x13C;
pub const ZYNQMP_DMA_DST_DSCR_WRD2: c_uint = 0x140;
pub const ZYNQMP_DMA_DST_DSCR_WRD3: c_uint = 0x144;
pub const ZYNQMP_DMA_SRC_START_LSB: c_uint = 0x158;
pub const ZYNQMP_DMA_SRC_START_MSB: c_uint = 0x15C;
pub const ZYNQMP_DMA_DST_START_LSB: c_uint = 0x160;
pub const ZYNQMP_DMA_DST_START_MSB: c_uint = 0x164;
pub const ZYNQMP_DMA_TOTAL_BYTE: c_uint = 0x188;
pub const ZYNQMP_DMA_RATE_CTRL: c_uint = 0x18C;
pub const ZYNQMP_DMA_IRQ_SRC_ACCT: c_uint = 0x190;
pub const ZYNQMP_DMA_IRQ_DST_ACCT: c_uint = 0x194;
pub const ZYNQMP_DMA_CTRL2: c_uint = 0x200;
// Interrupt registers bit field definitions

// Control 0 register bit field definitions

// Control 1 register bit field definitions

// Data Attribute register bit field definitions

pub const ZYNQMP_DMA_ARCACHE_OFST: c_int = 22;

pub const ZYNQMP_DMA_ARQOS_OFST: c_int = 18;

pub const ZYNQMP_DMA_ARLEN_OFST: c_int = 14;

pub const ZYNQMP_DMA_AWCACHE_OFST: c_int = 8;

pub const ZYNQMP_DMA_AWQOS_OFST: c_int = 4;

pub const ZYNQMP_DMA_AWLEN_OFST: c_int = 0;
// Descriptor Attribute register bit field definitions

pub const ZYNQMP_DMA_AXCACHE_OFST: c_int = 4;

pub const ZYNQMP_DMA_AXQOS_OFST: c_int = 0;
// Control register 2 bit field definitions

// Buffer Descriptor definitions
pub const ZYNQMP_DMA_DESC_CTRL_STOP: c_uint = 0x10;
pub const ZYNQMP_DMA_DESC_CTRL_COMP_INT: c_uint = 0x4;
pub const ZYNQMP_DMA_DESC_CTRL_SIZE_256: c_uint = 0x2;
pub const ZYNQMP_DMA_DESC_CTRL_COHRNT: c_uint = 0x1;
// Interrupt Mask specific definitions

    ZYNQMP_DMA_AXI_WR_DATA | \
    ZYNQMP_DMA_AXI_RD_DST_DSCR | \
    ZYNQMP_DMA_AXI_RD_SRC_DSCR | \
    ZYNQMP_DMA_INV_APB)

    ZYNQMP_DMA_IRQ_SRC_ACCT_ERR | \
    ZYNQMP_DMA_IRQ_DST_ACCT_ERR)

    ZYNQMP_DMA_INT_ERR | \
    ZYNQMP_DMA_INT_OVRFL | \
    ZYNQMP_DMA_DST_DSCR_DONE)
// Max number of descriptors per channel
pub const ZYNQMP_DMA_NUM_DESCS: c_int = 32;
// Max transfer size per descriptor
pub const ZYNQMP_DMA_MAX_TRANS_LEN: c_uint = 0x40000000;
// Max burst lengths

// Reset values for data attributes
pub const ZYNQMP_DMA_AXCACHE_VAL: c_uint = 0xF;
pub const ZYNQMP_DMA_SRC_ISSUE_RST_VAL: c_uint = 0x1F;
pub const ZYNQMP_DMA_IDS_DEFAULT_MASK: c_uint = 0xFFF;
// Bus width in bits
pub const ZYNQMP_DMA_BUS_WIDTH_64: c_int = 64;
pub const ZYNQMP_DMA_BUS_WIDTH_128: c_int = 128;
pub const ZDMA_PM_TIMEOUT: c_int = 100;

    common)

    async_tx)
// IRQ Register offset for Versal Gen 2
pub const IRQ_REG_OFFSET: c_uint = 0x308;
//
// struct zynqmp_dma_desc_ll - Hw linked list descriptor
// @addr: Buffer address
// @size: Size of the buffer
// @ctrl: Control word
// @nxtdscraddr: Next descriptor base address
// @rsvd: Reserved field and for Hw internal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dma_desc_ll {
    pub addr: u64,
    pub size: u32,
    pub ctrl: u32,
    pub nxtdscraddr: u64,
    pub rsvd: u64,
}

//
// struct zynqmp_dma_desc_sw - Per Transaction structure
// @src: Source address for simple mode dma
// @dst: Destination address for simple mode dma
// @len: Transfer length for simple mode dma
// @node: Node in the channel descriptor list
// @tx_list: List head for the current transfer
// @async_tx: Async transaction descriptor
// @src_v: Virtual address of the src descriptor
// @src_p: Physical address of the src descriptor
// @dst_v: Virtual address of the dst descriptor
// @dst_p: Physical address of the dst descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dma_desc_sw {
    pub src: u64,
    pub dst: u64,
    pub len: u32,
    pub node: list_head,
    pub tx_list: list_head,
    pub async_tx: dma_async_tx_descriptor,
    pub src_v: *mut zynqmp_dma_desc_ll,
    pub src_p: dma_addr_t,
    pub dst_v: *mut zynqmp_dma_desc_ll,
    pub dst_p: dma_addr_t,
}

//
// struct zynqmp_dma_chan - Driver specific DMA channel structure
// @zdev: Driver specific device structure
// @regs: Control registers offset
// @lock: Descriptor operation lock
// @pending_list: Descriptors waiting
// @free_list: Descriptors free
// @active_list: Descriptors active
// @sw_desc_pool: SW descriptor pool
// @done_list: Complete descriptors
// @common: DMA common channel
// @desc_pool_v: Statically allocated descriptor base
// @desc_pool_p: Physical allocated descriptor base
// @desc_free_cnt: Descriptor available count
// @dev: The dma device
// @irq: Channel IRQ
// @is_dmacoherent: Tells whether dma operations are coherent or not
// @tasklet: Cleanup work after irq
// @idle : Channel status;
// @desc_size: Size of the low level descriptor
// @err: Channel has errors
// @bus_width: Bus width
// @src_burst_len: Source burst length
// @dst_burst_len: Dest burst length
// @irq_offset: Irq register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dma_chan {
    pub zdev: *mut zynqmp_dma_device,
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
    pub pending_list: list_head,
    pub free_list: list_head,
    pub active_list: list_head,
    pub sw_desc_pool: *mut zynqmp_dma_desc_sw,
    pub done_list: list_head,
    pub common: dma_chan,
    pub desc_pool_v: *mut c_void,
    pub desc_pool_p: dma_addr_t,
    pub desc_free_cnt: u32,
    pub dev: *mut device,
    pub irq: c_int,
    pub is_dmacoherent: bool,
    pub tasklet: tasklet_struct,
    pub idle: bool,
    pub desc_size: usize,
    pub err: bool,
    pub bus_width: u32,
    pub src_burst_len: u32,
    pub dst_burst_len: u32,
    pub irq_offset: u32,
}

//
// struct zynqmp_dma_device - DMA device structure
// @dev: Device Structure
// @common: DMA device structure
// @chan: Driver specific DMA channel
// @clk_main: Pointer to main clock
// @clk_apb: Pointer to apb clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dma_device {
    pub dev: *mut device,
    pub common: dma_device,
    pub chan: *mut zynqmp_dma_chan,
    pub clk_main: *mut clk,
    pub clk_apb: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dma_config {
    pub offset: u32,
}

    static const struct zynqmp_dma_config versal2_dma_config = {
    .offset = IRQ_REG_OFFSET,
    };
    static inline void zynqmp_dma_writeq(struct zynqmp_dma_chan *chan, u32 reg,
    u64 value)
    {
    lo_hi_writeq(value, chan.regs + reg);
    }
//
// zynqmp_dma_update_desc_to_ctrlr - Updates descriptor to the controller
// @chan: ZynqMP DMA DMA channel pointer
// @desc: Transaction descriptor pointer
//
    static void zynqmp_dma_update_desc_to_ctrlr(struct zynqmp_dma_chan *chan,
    struct zynqmp_dma_desc_sw *desc)
    {
    dma_addr_t addr;
    addr = desc.src_p;
    zynqmp_dma_writeq(chan, ZYNQMP_DMA_SRC_START_LSB, addr);
    addr = desc.dst_p;
    zynqmp_dma_writeq(chan, ZYNQMP_DMA_DST_START_LSB, addr);
    }
//
// zynqmp_dma_desc_config_eod - Mark the descriptor as end descriptor
// @chan: ZynqMP DMA channel pointer
// @desc: Hw descriptor pointer
//
    static void zynqmp_dma_desc_config_eod(struct zynqmp_dma_chan *chan,
    void *desc)
    {
    struct zynqmp_dma_desc_ll *hw = (struct zynqmp_dma_desc_ll *)desc;
    hw.ctrl |= ZYNQMP_DMA_DESC_CTRL_STOP;
    hw++;
    hw.ctrl |= ZYNQMP_DMA_DESC_CTRL_COMP_INT | ZYNQMP_DMA_DESC_CTRL_STOP;
    }
//
// zynqmp_dma_config_sg_ll_desc - Configure the linked list descriptor
// @chan: ZynqMP DMA channel pointer
// @sdesc: Hw descriptor pointer
// @src: Source buffer address
// @dst: Destination buffer address
// @len: Transfer length
// @prev: Previous hw descriptor pointer
//
    static void zynqmp_dma_config_sg_ll_desc(struct zynqmp_dma_chan *chan,
    struct zynqmp_dma_desc_ll *sdesc,
    dma_addr_t src, dma_addr_t dst, size_t len,
    struct zynqmp_dma_desc_ll *prev)
    {
    struct zynqmp_dma_desc_ll *ddesc = sdesc + 1;
    sdesc.size = ddesc.size = len;
    sdesc.addr = src;
    ddesc.addr = dst;
    sdesc.ctrl = ddesc.ctrl = ZYNQMP_DMA_DESC_CTRL_SIZE_256;
    if (chan.is_dmacoherent) {
    sdesc.ctrl |= ZYNQMP_DMA_DESC_CTRL_COHRNT;
    ddesc.ctrl |= ZYNQMP_DMA_DESC_CTRL_COHRNT;
    }
    if (prev) {
    dma_addr_t addr = chan.desc_pool_p +
    ((uintptr_t)sdesc - (uintptr_t)chan.desc_pool_v);
    ddesc = prev + 1;
    prev.nxtdscraddr = addr;
    ddesc.nxtdscraddr = addr + ZYNQMP_DMA_DESC_SIZE(chan);
    }
    }
//
// zynqmp_dma_init - Initialize the channel
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_init(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_init(struct zynqmp_dma_chan *chan)
    {
    u32 val;
    writel(ZYNQMP_DMA_IDS_DEFAULT_MASK, chan.regs + ZYNQMP_DMA_IDS);
    val = readl(chan.regs + ZYNQMP_DMA_ISR);
    writel(val, chan.regs + ZYNQMP_DMA_ISR);
    if (chan.is_dmacoherent) {
    val = ZYNQMP_DMA_AXCOHRNT;
    val = (val & ~ZYNQMP_DMA_AXCACHE) |
    (ZYNQMP_DMA_AXCACHE_VAL << ZYNQMP_DMA_AXCACHE_OFST);
    writel(val, chan.regs + ZYNQMP_DMA_DSCR_ATTR);
    }
    val = readl(chan.regs + ZYNQMP_DMA_DATA_ATTR);
    if (chan.is_dmacoherent) {
    val = (val & ~ZYNQMP_DMA_ARCACHE) |
    (ZYNQMP_DMA_AXCACHE_VAL << ZYNQMP_DMA_ARCACHE_OFST);
    val = (val & ~ZYNQMP_DMA_AWCACHE) |
    (ZYNQMP_DMA_AXCACHE_VAL << ZYNQMP_DMA_AWCACHE_OFST);
    }
    writel(val, chan.regs + ZYNQMP_DMA_DATA_ATTR);
// Clearing the interrupt account registers
    val = readl(chan.regs + ZYNQMP_DMA_IRQ_SRC_ACCT);
    val = readl(chan.regs + ZYNQMP_DMA_IRQ_DST_ACCT);
    chan.idle = true;
    }
//
// zynqmp_dma_tx_submit - Submit DMA transaction
// @tx: Async transaction descriptor pointer
//
// Return: cookie value
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t zynqmp_dma_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct zynqmp_dma_chan *chan = to_chan(tx.chan);
    struct zynqmp_dma_desc_sw *desc, *new;
    dma_cookie_t cookie;
    unsigned long irqflags;
    new = tx_to_desc(tx);
    spin_lock_irqsave(&chan.lock, irqflags);
    cookie = dma_cookie_assign(tx);
    if (!list_empty(&chan.pending_list)) {
    desc = list_last_entry(&chan.pending_list,
    struct zynqmp_dma_desc_sw, node);
    if (!list_empty(&desc.tx_list))
    desc = list_last_entry(&desc.tx_list,
    struct zynqmp_dma_desc_sw, node);
    desc.src_v.nxtdscraddr = new.src_p;
    desc.src_v.ctrl &= ~ZYNQMP_DMA_DESC_CTRL_STOP;
    desc.dst_v.nxtdscraddr = new.dst_p;
    desc.dst_v.ctrl &= ~ZYNQMP_DMA_DESC_CTRL_STOP;
    }
    list_add_tail(&new.node, &chan.pending_list);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    return cookie;
    }
//
// zynqmp_dma_get_descriptor - Get the sw descriptor from the pool
// @chan: ZynqMP DMA channel pointer
//
// Return: The sw descriptor
//
    static struct zynqmp_dma_desc_sw *
    zynqmp_dma_get_descriptor(struct zynqmp_dma_chan *chan)
    {
    struct zynqmp_dma_desc_sw *desc;
    unsigned long irqflags;
    spin_lock_irqsave(&chan.lock, irqflags);
    desc = list_first_entry(&chan.free_list,
    struct zynqmp_dma_desc_sw, node);
    list_del(&desc.node);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    INIT_LIST_HEAD(&desc.tx_list);
// Clear the src and dst descriptor memory
    memset((void *)desc.src_v, 0, ZYNQMP_DMA_DESC_SIZE(chan));
    memset((void *)desc.dst_v, 0, ZYNQMP_DMA_DESC_SIZE(chan));
    return desc;
    }
//
// zynqmp_dma_free_descriptor - Issue pending transactions
// @chan: ZynqMP DMA channel pointer
// @sdesc: Transaction descriptor pointer
//
    static void zynqmp_dma_free_descriptor(struct zynqmp_dma_chan *chan,
    struct zynqmp_dma_desc_sw *sdesc)
    {
    struct zynqmp_dma_desc_sw *child, *next;
    chan.desc_free_cnt++;
    list_move_tail(&sdesc.node, &chan.free_list);
    list_for_each_entry_safe(child, next, &sdesc.tx_list, node) {
    chan.desc_free_cnt++;
    list_move_tail(&child.node, &chan.free_list);
    }
    }
//
// zynqmp_dma_free_desc_list - Free descriptors list
// @chan: ZynqMP DMA channel pointer
// @list: List to parse and delete the descriptor
//
    static void zynqmp_dma_free_desc_list(struct zynqmp_dma_chan *chan,
    struct list_head *list)
    {
    struct zynqmp_dma_desc_sw *desc, *next;
    list_for_each_entry_safe(desc, next, list, node)
    zynqmp_dma_free_descriptor(chan, desc);
    }
//
// zynqmp_dma_alloc_chan_resources - Allocate channel resources
// @dchan: DMA channel
//
// Return: Number of descriptors on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_alloc_chan_resources(dchan: *mut dma_chan) -> c_int {
    static int zynqmp_dma_alloc_chan_resources(struct dma_chan *dchan)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    struct zynqmp_dma_desc_sw *desc;
    int i, ret;
    ret = pm_runtime_resume_and_get(chan.dev);
    if (ret < 0)
    return ret;
    chan.sw_desc_pool = kzalloc_objs(*desc, ZYNQMP_DMA_NUM_DESCS);
    if (!chan.sw_desc_pool)
    return -ENOMEM;
    chan.idle = true;
    chan.desc_free_cnt = ZYNQMP_DMA_NUM_DESCS;
    INIT_LIST_HEAD(&chan.free_list);
    for (i = 0; i < ZYNQMP_DMA_NUM_DESCS; i++) {
    desc = chan.sw_desc_pool + i;
    dma_async_tx_descriptor_init(&desc.async_tx, &chan.common);
    desc.async_tx.tx_submit = zynqmp_dma_tx_submit;
    list_add_tail(&desc.node, &chan.free_list);
    }
    chan.desc_pool_v = dma_alloc_coherent(chan.dev,
    (2 * ZYNQMP_DMA_DESC_SIZE(chan) *
    ZYNQMP_DMA_NUM_DESCS),
    &chan.desc_pool_p, GFP_KERNEL);
    if (!chan.desc_pool_v)
    return -ENOMEM;
    for (i = 0; i < ZYNQMP_DMA_NUM_DESCS; i++) {
    desc = chan.sw_desc_pool + i;
    desc.src_v = (struct zynqmp_dma_desc_ll *) (chan.desc_pool_v +
    (i * ZYNQMP_DMA_DESC_SIZE(chan) * 2));
    desc.dst_v = (struct zynqmp_dma_desc_ll *) (desc.src_v + 1);
    desc.src_p = chan.desc_pool_p +
    (i * ZYNQMP_DMA_DESC_SIZE(chan) * 2);
    desc.dst_p = desc.src_p + ZYNQMP_DMA_DESC_SIZE(chan);
    }
    return ZYNQMP_DMA_NUM_DESCS;
    }
//
// zynqmp_dma_start - Start DMA channel
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_start(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_start(struct zynqmp_dma_chan *chan)
    {
    writel(ZYNQMP_DMA_INT_EN_DEFAULT_MASK, chan.regs + ZYNQMP_DMA_IER);
    writel(0, chan.regs + ZYNQMP_DMA_TOTAL_BYTE);
    chan.idle = false;
    writel(ZYNQMP_DMA_ENABLE, chan.regs + ZYNQMP_DMA_CTRL2);
    }
//
// zynqmp_dma_handle_ovfl_int - Process the overflow interrupt
// @chan: ZynqMP DMA channel pointer
// @status: Interrupt status value
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_handle_ovfl_int(chan: *mut zynqmp_dma_chan, status: u32) {
    static void zynqmp_dma_handle_ovfl_int(struct zynqmp_dma_chan *chan, u32 status)
    {
    if (status & ZYNQMP_DMA_BYTE_CNT_OVRFL)
    writel(0, chan.regs + ZYNQMP_DMA_TOTAL_BYTE);
    if (status & ZYNQMP_DMA_IRQ_DST_ACCT_ERR)
    readl(chan.regs + ZYNQMP_DMA_IRQ_DST_ACCT);
    if (status & ZYNQMP_DMA_IRQ_SRC_ACCT_ERR)
    readl(chan.regs + ZYNQMP_DMA_IRQ_SRC_ACCT);
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_config(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_config(struct zynqmp_dma_chan *chan)
    {
    u32 val, burst_val;
    val = readl(chan.regs + ZYNQMP_DMA_CTRL0);
    val |= ZYNQMP_DMA_POINT_TYPE_SG;
    writel(val, chan.regs + ZYNQMP_DMA_CTRL0);
    val = readl(chan.regs + ZYNQMP_DMA_DATA_ATTR);
    burst_val = __ilog2_u32(chan.src_burst_len);
    val = (val & ~ZYNQMP_DMA_ARLEN) |
    ((burst_val << ZYNQMP_DMA_ARLEN_OFST) & ZYNQMP_DMA_ARLEN);
    burst_val = __ilog2_u32(chan.dst_burst_len);
    val = (val & ~ZYNQMP_DMA_AWLEN) |
    ((burst_val << ZYNQMP_DMA_AWLEN_OFST) & ZYNQMP_DMA_AWLEN);
    writel(val, chan.regs + ZYNQMP_DMA_DATA_ATTR);
    }
//
// zynqmp_dma_device_config - Zynqmp dma device configuration
// @dchan: DMA channel
// @config: DMA device config
//
// Return: 0 always
//
    static int zynqmp_dma_device_config(struct dma_chan *dchan,
    struct dma_slave_config *config)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    chan.src_burst_len = clamp(config.src_maxburst, 1U,
    ZYNQMP_DMA_MAX_SRC_BURST_LEN);
    chan.dst_burst_len = clamp(config.dst_maxburst, 1U,
    ZYNQMP_DMA_MAX_DST_BURST_LEN);
    return 0;
    }
//
// zynqmp_dma_start_transfer - Initiate the new transfer
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_start_transfer(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_start_transfer(struct zynqmp_dma_chan *chan)
    {
    struct zynqmp_dma_desc_sw *desc;
    if (!chan.idle)
    return;
    zynqmp_dma_config(chan);
    desc = list_first_entry_or_null(&chan.pending_list,
    struct zynqmp_dma_desc_sw, node);
    if (!desc)
    return;
    list_splice_tail_init(&chan.pending_list, &chan.active_list);
    zynqmp_dma_update_desc_to_ctrlr(chan, desc);
    zynqmp_dma_start(chan);
    }
//
// zynqmp_dma_chan_desc_cleanup - Cleanup the completed descriptors
// @chan: ZynqMP DMA channel
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_chan_desc_cleanup(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_chan_desc_cleanup(struct zynqmp_dma_chan *chan)
    {
    struct zynqmp_dma_desc_sw *desc, *next;
    unsigned long irqflags;
    spin_lock_irqsave(&chan.lock, irqflags);
    list_for_each_entry_safe(desc, next, &chan.done_list, node) {
    struct dmaengine_desc_callback cb;
    dmaengine_desc_get_callback(&desc.async_tx, &cb);
    if (dmaengine_desc_callback_valid(&cb)) {
    spin_unlock_irqrestore(&chan.lock, irqflags);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    spin_lock_irqsave(&chan.lock, irqflags);
    }
// Run any dependencies, then free the descriptor
    zynqmp_dma_free_descriptor(chan, desc);
    }
    spin_unlock_irqrestore(&chan.lock, irqflags);
    }
//
// zynqmp_dma_complete_descriptor - Mark the active descriptor as complete
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_complete_descriptor(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_complete_descriptor(struct zynqmp_dma_chan *chan)
    {
    struct zynqmp_dma_desc_sw *desc;
    desc = list_first_entry_or_null(&chan.active_list,
    struct zynqmp_dma_desc_sw, node);
    if (!desc)
    return;
    list_del(&desc.node);
    dma_cookie_complete(&desc.async_tx);
    list_add_tail(&desc.node, &chan.done_list);
    }
//
// zynqmp_dma_issue_pending - Issue pending transactions
// @dchan: DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_issue_pending(dchan: *mut dma_chan) {
    static void zynqmp_dma_issue_pending(struct dma_chan *dchan)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    unsigned long irqflags;
    spin_lock_irqsave(&chan.lock, irqflags);
    zynqmp_dma_start_transfer(chan);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    }
//
// zynqmp_dma_free_descriptors - Free channel descriptors
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_free_descriptors(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_free_descriptors(struct zynqmp_dma_chan *chan)
    {
    unsigned long irqflags;
    spin_lock_irqsave(&chan.lock, irqflags);
    zynqmp_dma_free_desc_list(chan, &chan.active_list);
    zynqmp_dma_free_desc_list(chan, &chan.pending_list);
    zynqmp_dma_free_desc_list(chan, &chan.done_list);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    }
//
// zynqmp_dma_free_chan_resources - Free channel resources
// @dchan: DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_free_chan_resources(dchan: *mut dma_chan) {
    static void zynqmp_dma_free_chan_resources(struct dma_chan *dchan)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    zynqmp_dma_free_descriptors(chan);
    dma_free_coherent(chan.dev,
    (2 * ZYNQMP_DMA_DESC_SIZE(chan) * ZYNQMP_DMA_NUM_DESCS),
    chan.desc_pool_v, chan.desc_pool_p);
    kfree(chan.sw_desc_pool);
    pm_runtime_put_autosuspend(chan.dev);
    }
//
// zynqmp_dma_reset - Reset the channel
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_reset(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_reset(struct zynqmp_dma_chan *chan)
    {
    unsigned long irqflags;
    writel(ZYNQMP_DMA_IDS_DEFAULT_MASK, chan.regs + ZYNQMP_DMA_IDS);
    spin_lock_irqsave(&chan.lock, irqflags);
    zynqmp_dma_complete_descriptor(chan);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    zynqmp_dma_chan_desc_cleanup(chan);
    zynqmp_dma_free_descriptors(chan);
    zynqmp_dma_init(chan);
    }
//
// zynqmp_dma_irq_handler - ZynqMP DMA Interrupt handler
// @irq: IRQ number
// @data: Pointer to the ZynqMP DMA channel structure
//
// Return: IRQ_HANDLED/IRQ_NONE
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t zynqmp_dma_irq_handler(int irq, void *data)
    {
    struct zynqmp_dma_chan *chan = (struct zynqmp_dma_chan *)data;
    u32 isr, imr, status;
    let mut ret: irqreturn_t = IRQ_NONE;
    isr = readl(chan.regs + ZYNQMP_DMA_ISR);
    imr = readl(chan.regs + ZYNQMP_DMA_IMR);
    status = isr & ~imr;
    writel(isr, chan.regs + ZYNQMP_DMA_ISR);
    if (status & ZYNQMP_DMA_INT_DONE) {
    tasklet_schedule(&chan.tasklet);
    ret = IRQ_HANDLED;
    }
    if (status & ZYNQMP_DMA_DONE)
    chan.idle = true;
    if (status & ZYNQMP_DMA_INT_ERR) {
    chan.err = true;
    tasklet_schedule(&chan.tasklet);
    dev_err(chan.dev, "Channel %p has errors\n", chan);
    ret = IRQ_HANDLED;
    }
    if (status & ZYNQMP_DMA_INT_OVRFL) {
    zynqmp_dma_handle_ovfl_int(chan, status);
    dev_dbg(chan.dev, "Channel %p overflow interrupt\n", chan);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
//
// zynqmp_dma_do_tasklet - Schedule completion tasklet
// @t: Pointer to the ZynqMP DMA channel structure
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_do_tasklet(t: *mut tasklet_struct) {
    static void zynqmp_dma_do_tasklet(struct tasklet_struct *t)
    {
    struct zynqmp_dma_chan *chan = from_tasklet(chan, t, tasklet);
    u32 count;
    unsigned long irqflags;
    if (chan.err) {
    zynqmp_dma_reset(chan);
    chan.err = false;
    return;
    }
    spin_lock_irqsave(&chan.lock, irqflags);
    count = readl(chan.regs + ZYNQMP_DMA_IRQ_DST_ACCT);
    while (count) {
    zynqmp_dma_complete_descriptor(chan);
    count--;
    }
    spin_unlock_irqrestore(&chan.lock, irqflags);
    zynqmp_dma_chan_desc_cleanup(chan);
    if (chan.idle) {
    spin_lock_irqsave(&chan.lock, irqflags);
    zynqmp_dma_start_transfer(chan);
    spin_unlock_irqrestore(&chan.lock, irqflags);
    }
    }
//
// zynqmp_dma_device_terminate_all - Aborts all transfers on a channel
// @dchan: DMA channel pointer
//
// Return: Always '0'
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_device_terminate_all(dchan: *mut dma_chan) -> c_int {
    static int zynqmp_dma_device_terminate_all(struct dma_chan *dchan)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    writel(ZYNQMP_DMA_IDS_DEFAULT_MASK, chan.regs + ZYNQMP_DMA_IDS);
    zynqmp_dma_free_descriptors(chan);
    return 0;
    }
//
// zynqmp_dma_synchronize - Synchronizes the termination of a transfers to the current context.
// @dchan: DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_synchronize(dchan: *mut dma_chan) {
    static void zynqmp_dma_synchronize(struct dma_chan *dchan)
    {
    struct zynqmp_dma_chan *chan = to_chan(dchan);
    tasklet_kill(&chan.tasklet);
    }
//
// zynqmp_dma_prep_memcpy - prepare descriptors for memcpy transaction
// @dchan: DMA channel
// @dma_dst: Destination buffer address
// @dma_src: Source buffer address
// @len: Transfer length
// @flags: transfer ack flags
//
// Return: Async transaction descriptor on success and NULL on failure
//
    static struct dma_async_tx_descriptor *zynqmp_dma_prep_memcpy(
    struct dma_chan *dchan, dma_addr_t dma_dst,
    dma_addr_t dma_src, size_t len, ulong flags)
    {
    struct zynqmp_dma_chan *chan;
    struct zynqmp_dma_desc_sw *new, *first = core::ptr::null_mut();
    void *desc = core::ptr::null_mut(), *prev = core::ptr::null_mut();
    size_t copy;
    u32 desc_cnt;
    unsigned long irqflags;
    chan = to_chan(dchan);
    desc_cnt = DIV_ROUND_UP(len, ZYNQMP_DMA_MAX_TRANS_LEN);
    spin_lock_irqsave(&chan.lock, irqflags);
    if (desc_cnt > chan.desc_free_cnt) {
    spin_unlock_irqrestore(&chan.lock, irqflags);
    dev_dbg(chan.dev, "chan %p descs are not available\n", chan);
    return core::ptr::null_mut();
    }
    chan.desc_free_cnt = chan.desc_free_cnt - desc_cnt;
    spin_unlock_irqrestore(&chan.lock, irqflags);
    do {
// Allocate and populate the descriptor
    new = zynqmp_dma_get_descriptor(chan);
    copy = min_t(size_t, len, ZYNQMP_DMA_MAX_TRANS_LEN);
    desc = (struct zynqmp_dma_desc_ll *)new.src_v;
    zynqmp_dma_config_sg_ll_desc(chan, desc, dma_src,
    dma_dst, copy, prev);
    prev = desc;
    len -= copy;
    dma_src += copy;
    dma_dst += copy;
    if (!first)
    first = new;
    else
    list_add_tail(&new.node, &first.tx_list);
    } while (len);
    zynqmp_dma_desc_config_eod(chan, desc);
    async_tx_ack(&first.async_tx);
    first.async_tx.flags = (enum dma_ctrl_flags)flags;
    return &first.async_tx;
    }
//
// zynqmp_dma_chan_remove - Channel remove function
// @chan: ZynqMP DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_chan_remove(chan: *mut zynqmp_dma_chan) {
    static void zynqmp_dma_chan_remove(struct zynqmp_dma_chan *chan)
    {
    if (!chan)
    return;
    if (chan.irq)
    devm_free_irq(chan.zdev.dev, chan.irq, chan);
    tasklet_kill(&chan.tasklet);
    list_del(&chan.common.device_node);
    }
//
// zynqmp_dma_chan_probe - Per Channel Probing
// @zdev: Driver specific device structure
// @pdev: Pointer to the platform_device structure
//
// Return: '0' on success and failure value on error
//
    static int zynqmp_dma_chan_probe(struct zynqmp_dma_device *zdev,
    struct platform_device *pdev)
    {
    struct zynqmp_dma_chan *chan;
    struct device_node *node = pdev.dev.of_node;
    const struct zynqmp_dma_config *match_data;
    int err;
    chan = devm_kzalloc(zdev.dev, sizeof(*chan), GFP_KERNEL);
    if (!chan)
    return -ENOMEM;
    chan.dev = zdev.dev;
    chan.zdev = zdev;
    chan.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chan.regs))
    return PTR_ERR(chan.regs);
    chan.bus_width = ZYNQMP_DMA_BUS_WIDTH_64;
    chan.dst_burst_len = ZYNQMP_DMA_MAX_DST_BURST_LEN;
    chan.src_burst_len = ZYNQMP_DMA_MAX_SRC_BURST_LEN;
    err = of_property_read_u32(node, "xlnx,bus-width", &chan.bus_width);
    if (err < 0) {
    dev_err(&pdev.dev, "missing xlnx,bus-width property\n");
    return err;
    }
    if (chan.bus_width != ZYNQMP_DMA_BUS_WIDTH_64 &&
    chan.bus_width != ZYNQMP_DMA_BUS_WIDTH_128) {
    dev_err(zdev.dev, "invalid bus-width value");
    return -EINVAL;
    }
    match_data = of_device_get_match_data(&pdev.dev);
    if (match_data)
    chan.irq_offset = match_data.offset;
    chan.is_dmacoherent =  of_property_read_bool(node, "dma-coherent");
    zdev.chan = chan;
    tasklet_setup(&chan.tasklet, zynqmp_dma_do_tasklet);
    spin_lock_init(&chan.lock);
    INIT_LIST_HEAD(&chan.active_list);
    INIT_LIST_HEAD(&chan.pending_list);
    INIT_LIST_HEAD(&chan.done_list);
    INIT_LIST_HEAD(&chan.free_list);
    dma_cookie_init(&chan.common);
    chan.common.device = &zdev.common;
    list_add_tail(&chan.common.device_node, &zdev.common.channels);
    zynqmp_dma_init(chan);
    chan.irq = platform_get_irq(pdev, 0);
    if (chan.irq < 0)
    return -ENXIO;
    err = devm_request_irq(&pdev.dev, chan.irq, zynqmp_dma_irq_handler, 0,
    "zynqmp-dma", chan);
    if (err)
    return err;
    chan.desc_size = sizeof(struct zynqmp_dma_desc_ll);
    chan.idle = true;
    return 0;
    }
//
// of_zynqmp_dma_xlate - Translation function
// @dma_spec: Pointer to DMA specifier as found in the device tree
// @ofdma: Pointer to DMA controller data
//
// Return: DMA channel pointer on success and NULL on error
//
    static struct dma_chan *of_zynqmp_dma_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct zynqmp_dma_device *zdev = ofdma.of_dma_data;
    return dma_get_slave_channel(&zdev.chan.common);
    }
//
// zynqmp_dma_suspend - Suspend method for the driver
// @dev:	Address of the device structure
//
// Put the driver into low power mode.
// Return: 0 on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused zynqmp_dma_suspend(struct device *dev)
    {
    if (!device_may_wakeup(dev))
    return pm_runtime_force_suspend(dev);
    return 0;
    }
//
// zynqmp_dma_resume - Resume from suspend
// @dev:	Address of the device structure
//
// Resume operation after suspend.
// Return: 0 on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused zynqmp_dma_resume(struct device *dev)
    {
    if (!device_may_wakeup(dev))
    return pm_runtime_force_resume(dev);
    return 0;
    }
//
// zynqmp_dma_runtime_suspend - Runtime suspend method for the driver
// @dev:	Address of the device structure
//
// Put the driver into low power mode.
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused zynqmp_dma_runtime_suspend(struct device *dev)
    {
    struct zynqmp_dma_device *zdev = dev_get_drvdata(dev);
    clk_disable_unprepare(zdev.clk_main);
    clk_disable_unprepare(zdev.clk_apb);
    return 0;
    }
//
// zynqmp_dma_runtime_resume - Runtime suspend method for the driver
// @dev:	Address of the device structure
//
// Put the driver into low power mode.
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused zynqmp_dma_runtime_resume(struct device *dev)
    {
    struct zynqmp_dma_device *zdev = dev_get_drvdata(dev);
    int err;
    err = clk_prepare_enable(zdev.clk_main);
    if (err) {
    dev_err(dev, "Unable to enable main clock.\n");
    return err;
    }
    err = clk_prepare_enable(zdev.clk_apb);
    if (err) {
    dev_err(dev, "Unable to enable apb clock.\n");
    clk_disable_unprepare(zdev.clk_main);
    return err;
    }
    return 0;
    }
    static const struct dev_pm_ops zynqmp_dma_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(zynqmp_dma_suspend, zynqmp_dma_resume)
    SET_RUNTIME_PM_OPS(zynqmp_dma_runtime_suspend,
    zynqmp_dma_runtime_resume, core::ptr::null_mut())
    };
//
// zynqmp_dma_probe - Driver probe function
// @pdev: Pointer to the platform_device structure
//
// Return: '0' on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_probe(pdev: *mut platform_device) -> c_int {
    static int zynqmp_dma_probe(struct platform_device *pdev)
    {
    struct zynqmp_dma_device *zdev;
    struct dma_device *p;
    int ret;
    zdev = devm_kzalloc(&pdev.dev, sizeof(*zdev), GFP_KERNEL);
    if (!zdev)
    return -ENOMEM;
    zdev.dev = &pdev.dev;
    INIT_LIST_HEAD(&zdev.common.channels);
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(44));
    if (ret) {
    dev_err(&pdev.dev, "DMA not available for address range\n");
    return ret;
    }
    dma_cap_set(DMA_MEMCPY, zdev.common.cap_mask);
    p = &zdev.common;
    p.device_prep_dma_memcpy = zynqmp_dma_prep_memcpy;
    p.device_terminate_all = zynqmp_dma_device_terminate_all;
    p.device_synchronize = zynqmp_dma_synchronize;
    p.device_issue_pending = zynqmp_dma_issue_pending;
    p.device_alloc_chan_resources = zynqmp_dma_alloc_chan_resources;
    p.device_free_chan_resources = zynqmp_dma_free_chan_resources;
    p.device_tx_status = dma_cookie_status;
    p.device_config = zynqmp_dma_device_config;
    p.dev = &pdev.dev;
    zdev.clk_main = devm_clk_get(&pdev.dev, "clk_main");
    if (IS_ERR(zdev.clk_main))
    return dev_err_probe(&pdev.dev, PTR_ERR(zdev.clk_main),
    "main clock not found.\n");
    zdev.clk_apb = devm_clk_get(&pdev.dev, "clk_apb");
    if (IS_ERR(zdev.clk_apb))
    return dev_err_probe(&pdev.dev, PTR_ERR(zdev.clk_apb),
    "apb clock not found.\n");
    platform_set_drvdata(pdev, zdev);
    pm_runtime_set_autosuspend_delay(zdev.dev, ZDMA_PM_TIMEOUT);
    pm_runtime_use_autosuspend(zdev.dev);
    pm_runtime_enable(zdev.dev);
    ret = pm_runtime_resume_and_get(zdev.dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "device wakeup failed.\n");
    pm_runtime_disable(zdev.dev);
    }
    if (!pm_runtime_enabled(zdev.dev)) {
    ret = zynqmp_dma_runtime_resume(zdev.dev);
    if (ret)
    return ret;
    }
    ret = zynqmp_dma_chan_probe(zdev, pdev);
    if (ret) {
    dev_err_probe(&pdev.dev, ret, "Probing channel failed\n");
    goto err_disable_pm;
    }
    p.dst_addr_widths = BIT(zdev.chan.bus_width / 8);
    p.src_addr_widths = BIT(zdev.chan.bus_width / 8);
    ret = dma_async_device_register(&zdev.common);
    if (ret) {
    dev_err(zdev.dev, "failed to register the dma device\n");
    goto free_chan_resources;
    }
    ret = of_dma_controller_register(pdev.dev.of_node,
    of_zynqmp_dma_xlate, zdev);
    if (ret) {
    dev_err_probe(&pdev.dev, ret, "Unable to register DMA to DT\n");
    dma_async_device_unregister(&zdev.common);
    goto free_chan_resources;
    }
    pm_runtime_put_sync_autosuspend(zdev.dev);
    return 0;
    free_chan_resources:
    zynqmp_dma_chan_remove(zdev.chan);
    err_disable_pm:
    if (!pm_runtime_enabled(zdev.dev))
    zynqmp_dma_runtime_suspend(zdev.dev);
    pm_runtime_disable(zdev.dev);
    return ret;
    }
//
// zynqmp_dma_remove - Driver remove function
// @pdev: Pointer to the platform_device structure
//
#[no_mangle]
unsafe extern "C" fn zynqmp_dma_remove(pdev: *mut platform_device) {
    static void zynqmp_dma_remove(struct platform_device *pdev)
    {
    struct zynqmp_dma_device *zdev = platform_get_drvdata(pdev);
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&zdev.common);
    zynqmp_dma_chan_remove(zdev.chan);
    pm_runtime_disable(zdev.dev);
    if (!pm_runtime_status_suspended(zdev.dev))
    zynqmp_dma_runtime_suspend(zdev.dev);
    }
    static const struct of_device_id zynqmp_dma_of_match[] = {
    { .compatible = "amd,versal2-dma-1.0", .data = &versal2_dma_config },
    { .compatible = "xlnx,zynqmp-dma-1.0", },
    {}
    };
    MODULE_DEVICE_TABLE(of, zynqmp_dma_of_match);
    static struct platform_driver zynqmp_dma_driver = {
    .driver = {
    .name = "xilinx-zynqmp-dma",
    .of_match_table = zynqmp_dma_of_match,
    .pm = &zynqmp_dma_dev_pm_ops,
    },
    .probe = zynqmp_dma_probe,
    .remove = zynqmp_dma_remove,
    .shutdown = zynqmp_dma_remove,
    };
    module_platform_driver(zynqmp_dma_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Xilinx, Inc.");
    MODULE_DESCRIPTION("Xilinx ZynqMP DMA driver");
