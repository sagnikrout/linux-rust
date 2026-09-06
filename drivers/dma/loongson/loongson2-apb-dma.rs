//! Automatically rewritten from C to Rust
//! Source: drivers/dma/loongson/loongson2-apb-dma.c
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
// Driver for the Loongson-2 APB DMA Controller
//
// Copyright (C) 2017-2023 Loongson Corporation
//

// Global Configuration Register
pub const LDMA_ORDER_ERG: c_uint = 0x0;
// Bitfield definitions
// Bitfields in Global Configuration Register

// Bitfields in ndesc_addr field of HW descriptor

// Bitfields in cmd field of HW descriptor

    BIT(DMA_SLAVE_BUSWIDTH_8_BYTES))

// --  descriptors  -----------------------------------------------------
//
// struct ls2x_dma_hw_desc - DMA HW descriptor
// @ndesc_addr: the next descriptor low address.
// @mem_addr: memory low address.
// @apb_addr: device buffer address.
// @len: length of a piece of carried content, in words.
// @step_len: length between two moved memory data blocks.
// @step_times: number of blocks to be carried in a single DMA operation.
// @cmd: descriptor command or state.
// @stats: DMA status.
// @high_ndesc_addr: the next descriptor high address.
// @high_mem_addr: memory high address.
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_dma_hw_desc {
    pub ndesc_addr: u32,
    pub mem_addr: u32,
    pub apb_addr: u32,
    pub len: u32,
    pub step_len: u32,
    pub step_times: u32,
    pub cmd: u32,
    pub stats: u32,
    pub high_ndesc_addr: u32,
    pub high_mem_addr: u32,
    pub reserved: [u32; 2],
    pub __packed: },
//
// struct ls2x_dma_sg - ls2x dma scatter gather entry
// @hw: the pointer to DMA HW descriptor.
// @llp: physical address of the DMA HW descriptor.
// @phys: destination or source address(mem).
// @len: number of Bytes to read.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_dma_sg {
    pub hw: *mut ls2x_dma_hw_desc,
    pub llp: dma_addr_t,
    pub phys: dma_addr_t,
    pub len: u32,
}

//
// struct ls2x_dma_desc - software descriptor
// @vdesc: pointer to the virtual dma descriptor.
// @cyclic: flag to dma cyclic
// @burst_size: burst size of transaction, in words.
// @desc_num: number of sg entries.
// @direction: transfer direction, to or from device.
// @status: dma controller status.
// @sg: array of sgs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_dma_desc {
    pub vdesc: virt_dma_desc,
    pub cyclic: bool,
    pub burst_size: usize,
    pub desc_num: u32,
    pub direction: enum dma_transfer_direction,
    pub status: enum dma_status,
    pub __counted_by(desc_num): ls2x_dma_sg sg[],
}

// --  Channels  --------------------------------------------------------
//
// struct ls2x_dma_chan - internal representation of an LS2X APB DMA channel
// @vchan: virtual dma channel entry.
// @desc: pointer to the ls2x sw dma descriptor.
// @pool: hw desc table
// @irq: irq line
// @sconfig: configuration for slave transfers, passed via .device_config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_dma_chan {
    pub vchan: virt_dma_chan,
    pub desc: *mut ls2x_dma_desc,
    pub pool: *mut c_void,
    pub irq: c_int,
    pub sconfig: dma_slave_config,
}

// --  Controller  ------------------------------------------------------
//
// struct ls2x_dma_priv - LS2X APB DMAC specific information
// @ddev: dmaengine dma_device object members
// @dma_clk: DMAC clock source
// @regs: memory mapped register base
// @lchan: channel to store ls2x_dma_chan structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls2x_dma_priv {
    pub ddev: dma_device,
    pub dma_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub lchan: ls2x_dma_chan,
}

// --  Helper functions  ------------------------------------------------
    static inline struct ls2x_dma_desc *to_ldma_desc(struct virt_dma_desc *vdesc)
    {
    return container_of(vdesc, struct ls2x_dma_desc, vdesc);
    }
    static inline struct ls2x_dma_chan *to_ldma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct ls2x_dma_chan, vchan.chan);
    }
    static inline struct ls2x_dma_priv *to_ldma_priv(struct dma_device *ddev)
    {
    return container_of(ddev, struct ls2x_dma_priv, ddev);
    }
    static struct device *chan2dev(struct dma_chan *chan)
    {
    return &chan.dev.device;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dma_desc_free(vdesc: *mut virt_dma_desc) {
    static void ls2x_dma_desc_free(struct virt_dma_desc *vdesc)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(vdesc.tx.chan);
    struct ls2x_dma_desc *desc = to_ldma_desc(vdesc);
    int i;
    for (i = 0; i < desc.desc_num; i++) {
    if (desc.sg[i].hw)
    dma_pool_free(lchan.pool, desc.sg[i].hw,
    desc.sg[i].llp);
    }
    kfree(desc);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dma_write_cmd(lchan: *mut ls2x_dma_chan, cmd: bool) {
    static void ls2x_dma_write_cmd(struct ls2x_dma_chan *lchan, bool cmd)
    {
    struct ls2x_dma_priv *priv = to_ldma_priv(lchan.vchan.chan.device);
    u64 val;
    val = lo_hi_readq(priv.regs + LDMA_ORDER_ERG) & ~LDMA_CONFIG_MASK;
    val |= LDMA_64BIT_EN | cmd;
    lo_hi_writeq(val, priv.regs + LDMA_ORDER_ERG);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dma_start_transfer(lchan: *mut ls2x_dma_chan) {
    static void ls2x_dma_start_transfer(struct ls2x_dma_chan *lchan)
    {
    struct ls2x_dma_priv *priv = to_ldma_priv(lchan.vchan.chan.device);
    struct ls2x_dma_sg *ldma_sg;
    struct virt_dma_desc *vdesc;
    u64 val;
// Get the next descriptor
    vdesc = vchan_next_desc(&lchan.vchan);
    if (!vdesc) {
    lchan.desc = core::ptr::null_mut();
    return;
    }
    list_del(&vdesc.node);
    lchan.desc = to_ldma_desc(vdesc);
    ldma_sg = &lchan.desc.sg[0];
// Start DMA
    lo_hi_writeq(0, priv.regs + LDMA_ORDER_ERG);
    val = (ldma_sg.llp & ~LDMA_CONFIG_MASK) | LDMA_64BIT_EN | LDMA_START;
    lo_hi_writeq(val, priv.regs + LDMA_ORDER_ERG);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dmac_detect_burst(lchan: *mut ls2x_dma_chan) -> usize {
    static size_t ls2x_dmac_detect_burst(struct ls2x_dma_chan *lchan)
    {
    u32 maxburst, buswidth;
// Reject definitely invalid configurations
    if ((lchan.sconfig.src_addr_width & LDMA_SLAVE_BUSWIDTHS) &&
    (lchan.sconfig.dst_addr_width & LDMA_SLAVE_BUSWIDTHS))
    return 0;
    if (lchan.sconfig.direction == DMA_MEM_TO_DEV) {
    maxburst = lchan.sconfig.dst_maxburst;
    buswidth = lchan.sconfig.dst_addr_width;
    } else {
    maxburst = lchan.sconfig.src_maxburst;
    buswidth = lchan.sconfig.src_addr_width;
    }
// If maxburst is zero, fallback to LDMA_MAX_TRANS_LEN
    return maxburst ? (maxburst * buswidth) >> 2 : LDMA_MAX_TRANS_LEN;
    }
    static void ls2x_dma_fill_desc(struct ls2x_dma_chan *lchan, u32 sg_index,
    struct ls2x_dma_desc *desc)
    {
    struct ls2x_dma_sg *ldma_sg = &desc.sg[sg_index];
    u32 num_segments, segment_size;
    if (desc.direction == DMA_MEM_TO_DEV) {
    ldma_sg.hw.cmd = LDMA_INT | LDMA_DATA_DIRECTION;
    ldma_sg.hw.apb_addr = lchan.sconfig.dst_addr;
    } else {
    ldma_sg.hw.cmd = LDMA_INT;
    ldma_sg.hw.apb_addr = lchan.sconfig.src_addr;
    }
    ldma_sg.hw.mem_addr = lower_32_bits(ldma_sg.phys);
    ldma_sg.hw.high_mem_addr = upper_32_bits(ldma_sg.phys);
// Split into multiple equally sized segments if necessary
    num_segments = DIV_ROUND_UP((ldma_sg.len + 3) >> 2, desc.burst_size);
    segment_size = DIV_ROUND_UP((ldma_sg.len + 3) >> 2, num_segments);
// Word count register takes input in words
    ldma_sg.hw.len = segment_size;
    ldma_sg.hw.step_times = num_segments;
    ldma_sg.hw.step_len = 0;
// lets make a link list
    if (sg_index) {
    desc.sg[sg_index - 1].hw.ndesc_addr = ldma_sg.llp | LDMA_DESC_EN;
    desc.sg[sg_index - 1].hw.high_ndesc_addr = upper_32_bits(ldma_sg.llp);
    }
    }
// --  DMA Engine API  --------------------------------------------------
//
// ls2x_dma_alloc_chan_resources - allocate resources for DMA channel
// @chan: allocate descriptor resources for this channel
//
// return - the number of allocated descriptors
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int ls2x_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
// Create a pool of consistent memory blocks for hardware descriptors
    lchan.pool = dma_pool_create(dev_name(chan2dev(chan)),
    chan.device.dev, PAGE_SIZE,
    __alignof__(struct ls2x_dma_hw_desc), 0);
    if (!lchan.pool) {
    dev_err(chan2dev(chan), "No memory for descriptors\n");
    return -ENOMEM;
    }
    return 1;
    }
//
// ls2x_dma_free_chan_resources - free all channel resources
// @chan: DMA channel
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_free_chan_resources(chan: *mut dma_chan) {
    static void ls2x_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    vchan_free_chan_resources(to_virt_chan(chan));
    dma_pool_destroy(lchan.pool);
    lchan.pool = core::ptr::null_mut();
    }
//
// ls2x_dma_prep_slave_sg - prepare descriptors for a DMA_SLAVE transaction
// @chan: DMA channel
// @sgl: scatterlist to transfer to/from
// @sg_len: number of entries in @scatterlist
// @direction: DMA direction
// @flags: tx descriptor status flags
// @context: transaction context (ignored)
//
// Return: Async transaction descriptor on success and NULL on failure
//
    static struct dma_async_tx_descriptor *
    ls2x_dma_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    u32 sg_len, enum dma_transfer_direction direction,
    unsigned long flags, void *context)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    struct ls2x_dma_desc *desc;
    struct scatterlist *sg;
    size_t burst_size;
    int i;
    if (unlikely(!sg_len || !is_slave_direction(direction)))
    return core::ptr::null_mut();
    burst_size = ls2x_dmac_detect_burst(lchan);
    if (!burst_size)
    return core::ptr::null_mut();
    desc = kzalloc_flex(*desc, sg, sg_len, GFP_NOWAIT);
    if (!desc)
    return core::ptr::null_mut();
    desc.desc_num = sg_len;
    desc.direction = direction;
    desc.burst_size = burst_size;
    for_each_sg(sgl, sg, sg_len, i) {
    struct ls2x_dma_sg *ldma_sg = &desc.sg[i];
// Allocate DMA capable memory for hardware descriptor
    ldma_sg.hw = dma_pool_alloc(lchan.pool, GFP_NOWAIT, &ldma_sg.llp);
    if (!ldma_sg.hw) {
    desc.desc_num = i;
    ls2x_dma_desc_free(&desc.vdesc);
    return core::ptr::null_mut();
    }
    ldma_sg.phys = sg_dma_address(sg);
    ldma_sg.len = sg_dma_len(sg);
    ls2x_dma_fill_desc(lchan, i, desc);
    }
// Setting the last descriptor enable bit
    desc.sg[sg_len - 1].hw.ndesc_addr &= ~LDMA_DESC_EN;
    desc.status = DMA_IN_PROGRESS;
    return vchan_tx_prep(&lchan.vchan, &desc.vdesc, flags);
    }
//
// ls2x_dma_prep_dma_cyclic - prepare the cyclic DMA transfer
// @chan: the DMA channel to prepare
// @buf_addr: physical DMA address where the buffer starts
// @buf_len: total number of bytes for the entire buffer
// @period_len: number of bytes for each period
// @direction: transfer direction, to or from device
// @flags: tx descriptor status flags
//
// Return: Async transaction descriptor on success and NULL on failure
//
    static struct dma_async_tx_descriptor *
    ls2x_dma_prep_dma_cyclic(struct dma_chan *chan, dma_addr_t buf_addr, size_t buf_len,
    size_t period_len, enum dma_transfer_direction direction,
    unsigned long flags)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    struct ls2x_dma_desc *desc;
    size_t burst_size;
    u32 num_periods;
    int i;
    if (unlikely(!buf_len || !period_len))
    return core::ptr::null_mut();
    if (unlikely(!is_slave_direction(direction)))
    return core::ptr::null_mut();
    burst_size = ls2x_dmac_detect_burst(lchan);
    if (!burst_size)
    return core::ptr::null_mut();
    num_periods = buf_len / period_len;
    desc = kzalloc_flex(*desc, sg, num_periods, GFP_NOWAIT);
    if (!desc)
    return core::ptr::null_mut();
    desc.desc_num = num_periods;
    desc.direction = direction;
    desc.burst_size = burst_size;
// Build cyclic linked list
    for (i = 0; i < num_periods; i++) {
    struct ls2x_dma_sg *ldma_sg = &desc.sg[i];
// Allocate DMA capable memory for hardware descriptor
    ldma_sg.hw = dma_pool_alloc(lchan.pool, GFP_NOWAIT, &ldma_sg.llp);
    if (!ldma_sg.hw) {
    desc.desc_num = i;
    ls2x_dma_desc_free(&desc.vdesc);
    return core::ptr::null_mut();
    }
    ldma_sg.phys = buf_addr + period_len * i;
    ldma_sg.len = period_len;
    ls2x_dma_fill_desc(lchan, i, desc);
    }
// Lets make a cyclic list
    desc.sg[num_periods - 1].hw.ndesc_addr = desc.sg[0].llp | LDMA_DESC_EN;
    desc.sg[num_periods - 1].hw.high_ndesc_addr = upper_32_bits(desc.sg[0].llp);
    desc.cyclic = true;
    desc.status = DMA_IN_PROGRESS;
    return vchan_tx_prep(&lchan.vchan, &desc.vdesc, flags);
    }
//
// ls2x_slave_config - set slave configuration for channel
// @chan: dma channel
// @cfg: slave configuration
//
// Sets slave configuration for channel
//
    static int ls2x_dma_slave_config(struct dma_chan *chan,
    struct dma_slave_config *config)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    memcpy(&lchan.sconfig, config, sizeof(*config));
    return 0;
    }
//
// ls2x_dma_issue_pending - push pending transactions to the hardware
// @chan: channel
//
// When this function is called, all pending transactions are pushed to the
// hardware and executed.
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_issue_pending(chan: *mut dma_chan) {
    static void ls2x_dma_issue_pending(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    guard(spinlock_irqsave)(&lchan.vchan.lock);
    if (vchan_issue_pending(&lchan.vchan) && !lchan.desc)
    ls2x_dma_start_transfer(lchan);
    }
//
// ls2x_dma_terminate_all - terminate all transactions
// @chan: channel
//
// Stops all DMA transactions.
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int ls2x_dma_terminate_all(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    LIST_HEAD(head);
    scoped_guard(spinlock_irqsave, &lchan.vchan.lock) {
// Setting stop cmd
    ls2x_dma_write_cmd(lchan, LDMA_STOP);
    if (lchan.desc) {
    vchan_terminate_vdesc(&lchan.desc.vdesc);
    lchan.desc = core::ptr::null_mut();
    }
    vchan_get_all_descriptors(&lchan.vchan, &head);
    }
    vchan_dma_desc_free_list(&lchan.vchan, &head);
    return 0;
    }
//
// ls2x_dma_synchronize - Synchronizes the termination of transfers to the
// current context.
// @chan: channel
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_synchronize(chan: *mut dma_chan) {
    static void ls2x_dma_synchronize(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    vchan_synchronize(&lchan.vchan);
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dma_pause(chan: *mut dma_chan) -> c_int {
    static int ls2x_dma_pause(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    guard(spinlock_irqsave)(&lchan.vchan.lock);
    if (lchan.desc && lchan.desc.status == DMA_IN_PROGRESS) {
    ls2x_dma_write_cmd(lchan, LDMA_STOP);
    lchan.desc.status = DMA_PAUSED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls2x_dma_resume(chan: *mut dma_chan) -> c_int {
    static int ls2x_dma_resume(struct dma_chan *chan)
    {
    struct ls2x_dma_chan *lchan = to_ldma_chan(chan);
    guard(spinlock_irqsave)(&lchan.vchan.lock);
    if (lchan.desc && lchan.desc.status == DMA_PAUSED) {
    lchan.desc.status = DMA_IN_PROGRESS;
    ls2x_dma_write_cmd(lchan, LDMA_START);
    }
    return 0;
    }
//
// ls2x_dma_isr - LS2X DMA Interrupt handler
// @irq: IRQ number
// @dev_id: Pointer to ls2x_dma_chan
//
// Return: IRQ_HANDLED/IRQ_NONE
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls2x_dma_isr(int irq, void *dev_id)
    {
    struct ls2x_dma_chan *lchan = dev_id;
    struct ls2x_dma_desc *desc;
    scoped_guard(spinlock, &lchan.vchan.lock) {
    desc = lchan.desc;
    if (desc) {
    if (desc.cyclic) {
    vchan_cyclic_callback(&desc.vdesc);
    } else {
    desc.status = DMA_COMPLETE;
    vchan_cookie_complete(&desc.vdesc);
    ls2x_dma_start_transfer(lchan);
    }
// ls2x_dma_start_transfer() updates lchan->desc
    if (!lchan.desc)
    ls2x_dma_write_cmd(lchan, LDMA_STOP);
    }
    }
    return IRQ_HANDLED;
    }
    static int ls2x_dma_chan_init(struct platform_device *pdev,
    struct ls2x_dma_priv *priv)
    {
    struct ls2x_dma_chan *lchan = &priv.lchan;
    struct device *dev = &pdev.dev;
    int ret;
    lchan.irq = platform_get_irq(pdev, 0);
    if (lchan.irq < 0)
    return lchan.irq;
    ret = devm_request_irq(dev, lchan.irq, ls2x_dma_isr, IRQF_TRIGGER_RISING,
    dev_name(&pdev.dev), lchan);
    if (ret)
    return ret;
// Initialize channels related values
    INIT_LIST_HEAD(&priv.ddev.channels);
    lchan.vchan.desc_free = ls2x_dma_desc_free;
    vchan_init(&lchan.vchan, &priv.ddev);
    return 0;
    }
//
// ls2x_dma_probe - Driver probe function
// @pdev: Pointer to the platform_device structure
//
// Return: '0' on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_probe(pdev: *mut platform_device) -> c_int {
    static int ls2x_dma_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ls2x_dma_priv *priv;
    struct dma_device *ddev;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "devm_platform_ioremap_resource failed.\n");
    priv.dma_clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.dma_clk))
    return dev_err_probe(dev, PTR_ERR(priv.dma_clk), "Couldn't start the clock.\n");
    ret = ls2x_dma_chan_init(pdev, priv);
    if (ret)
    return ret;
    ddev = &priv.ddev;
    ddev.dev = dev;
    dma_cap_zero(ddev.cap_mask);
    dma_cap_set(DMA_SLAVE, ddev.cap_mask);
    dma_cap_set(DMA_CYCLIC, ddev.cap_mask);
    ddev.device_alloc_chan_resources = ls2x_dma_alloc_chan_resources;
    ddev.device_free_chan_resources = ls2x_dma_free_chan_resources;
    ddev.device_tx_status = dma_cookie_status;
    ddev.device_issue_pending = ls2x_dma_issue_pending;
    ddev.device_prep_slave_sg = ls2x_dma_prep_slave_sg;
    ddev.device_prep_dma_cyclic = ls2x_dma_prep_dma_cyclic;
    ddev.device_config = ls2x_dma_slave_config;
    ddev.device_terminate_all = ls2x_dma_terminate_all;
    ddev.device_synchronize = ls2x_dma_synchronize;
    ddev.device_pause = ls2x_dma_pause;
    ddev.device_resume = ls2x_dma_resume;
    ddev.src_addr_widths = LDMA_SLAVE_BUSWIDTHS;
    ddev.dst_addr_widths = LDMA_SLAVE_BUSWIDTHS;
    ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    ret = dmaenginem_async_device_register(&priv.ddev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to register DMA engine device.\n");
    ret = of_dma_controller_register(dev.of_node, of_dma_xlate_by_chan_id, priv);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to register dma controller.\n");
    platform_set_drvdata(pdev, priv);
    dev_info(dev, "Loongson LS2X APB DMA driver registered successfully.\n");
    return 0;
    }
//
// ls2x_dma_remove - Driver remove function
// @pdev: Pointer to the platform_device structure
//
#[no_mangle]
unsafe extern "C" fn ls2x_dma_remove(pdev: *mut platform_device) {
    static void ls2x_dma_remove(struct platform_device *pdev)
    {
    of_dma_controller_free(pdev.dev.of_node);
    }
    static const struct of_device_id ls2x_dma_of_match_table[] = {
    { .compatible = "loongson,ls2k1000-apbdma" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ls2x_dma_of_match_table);
    static struct platform_driver ls2x_dmac_driver = {
    .probe		= ls2x_dma_probe,
    .remove		= ls2x_dma_remove,
    .driver = {
    .name	= "ls2x-apbdma",
    .of_match_table	= ls2x_dma_of_match_table,
    },
    };
    module_platform_driver(ls2x_dmac_driver);
    MODULE_DESCRIPTION("Loongson-2 APB DMA Controller driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
