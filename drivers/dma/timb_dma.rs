//! Automatically rewritten from C to Rust
//! Source: drivers/dma/timb_dma.c
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
// timb_dma.c timberdale FPGA DMA driver
// Copyright (c) 2010 Intel Corporation
//
// Supports:
// Timberdale FPGA DMA engine
//

// Global DMA registers
pub const TIMBDMA_ACR: c_uint = 0x34;
pub const TIMBDMA_32BIT_ADDR: c_uint = 0x01;
pub const TIMBDMA_ISR: c_uint = 0x080000;
pub const TIMBDMA_IPR: c_uint = 0x080004;
pub const TIMBDMA_IER: c_uint = 0x080008;
// Channel specific registers
// RX instances base addresses are 0x00, 0x40, 0x80 ...
// TX instances base addresses are 0x18, 0x58, 0x98 ...
//
pub const TIMBDMA_INSTANCE_OFFSET: c_uint = 0x40;
pub const TIMBDMA_INSTANCE_TX_OFFSET: c_uint = 0x18;
// RX registers, relative the instance base
pub const TIMBDMA_OFFS_RX_DHAR: c_uint = 0x00;
pub const TIMBDMA_OFFS_RX_DLAR: c_uint = 0x04;
pub const TIMBDMA_OFFS_RX_LR: c_uint = 0x0C;
pub const TIMBDMA_OFFS_RX_BLR: c_uint = 0x10;
pub const TIMBDMA_OFFS_RX_ER: c_uint = 0x14;
pub const TIMBDMA_RX_EN: c_uint = 0x01;
// bytes per Row, video specific register
// which is placed after the TX registers...
//
pub const TIMBDMA_OFFS_RX_BPRR: c_uint = 0x30;
// TX registers, relative the instance base
pub const TIMBDMA_OFFS_TX_DHAR: c_uint = 0x00;
pub const TIMBDMA_OFFS_TX_DLAR: c_uint = 0x04;
pub const TIMBDMA_OFFS_TX_BLR: c_uint = 0x0C;
pub const TIMBDMA_OFFS_TX_LR: c_uint = 0x14;
pub const TIMB_DMA_DESC_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_dma_desc {
    pub desc_node: list_head,
    pub txd: dma_async_tx_descriptor,
    pub desc_list: *mut u8,
    pub desc_list_len: c_uint,
    pub interrupt: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_dma_chan {
    pub chan: dma_chan,
    pub membase: *mut void __iomem,
    pub structures,: *mut *mut spinlock_t lock; / Used to protect data,
    especially the lists and descriptors,
    from races between the tasklet and calls
    from above */
    pub ongoing: bool,
    pub active_list: list_head,
    pub queue: list_head,
    pub free_list: list_head,
    pub bytes_per_line: c_uint,
    pub direction: enum dma_transfer_direction,
    pub /: *mut *mut unsigned int descs; / Descriptors to allocate,
    pub /: *mut *mut unsigned int desc_elems; / number of elems per descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_dma {
    pub dma: dma_device,
    pub membase: *mut void __iomem,
    pub tasklet: tasklet_struct,
    pub channels: [timb_dma_chan; ],
}

    static struct device *chan2dev(struct dma_chan *chan)
    {
    return &chan.dev.device;
    }
    static struct device *chan2dmadev(struct dma_chan *chan)
    {
    return chan2dev(chan).parent.parent;
    }
    static struct timb_dma *tdchantotd(struct timb_dma_chan *td_chan)
    {
    let mut id: c_int = td_chan.chan.chan_id;
    return (struct timb_dma *)((u8 *)td_chan -
    id * sizeof(struct timb_dma_chan) - sizeof(struct timb_dma));
    }
// Must be called with the spinlock held
#[no_mangle]
unsafe extern "C" fn __td_enable_chan_irq(td_chan: *mut timb_dma_chan) {
    static void __td_enable_chan_irq(struct timb_dma_chan *td_chan)
    {
    let mut id: c_int = td_chan.chan.chan_id;
    struct timb_dma *td = tdchantotd(td_chan);
    u32 ier;
// enable interrupt for this channel
    ier = ioread32(td.membase + TIMBDMA_IER);
    ier |= 1 << id;
    dev_dbg(chan2dev(&td_chan.chan), "Enabling irq: %d, IER: 0x%x\n", id,
    ier);
    iowrite32(ier, td.membase + TIMBDMA_IER);
    }
// Should be called with the spinlock held
#[no_mangle]
unsafe extern "C" fn __td_dma_done_ack(td_chan: *mut timb_dma_chan) -> bool {
    static bool __td_dma_done_ack(struct timb_dma_chan *td_chan)
    {
    let mut id: c_int = td_chan.chan.chan_id;
    struct timb_dma *td = (struct timb_dma *)((u8 *)td_chan -
    id * sizeof(struct timb_dma_chan) - sizeof(struct timb_dma));
    u32 isr;
    let mut done: bool = false;
    dev_dbg(chan2dev(&td_chan.chan), "Checking irq: %d, td: %p\n", id, td);
    isr = ioread32(td.membase + TIMBDMA_ISR) & (1 << id);
    if (isr) {
    iowrite32(isr, td.membase + TIMBDMA_ISR);
    done = true;
    }
    return done;
    }
    static int td_fill_desc(struct timb_dma_chan *td_chan, u8 *dma_desc,
    struct scatterlist *sg, bool last)
    {
    if (sg_dma_len(sg) > USHRT_MAX) {
    dev_err(chan2dev(&td_chan.chan), "Too big sg element\n");
    return -EINVAL;
    }
// length must be word aligned
    if (sg_dma_len(sg) % sizeof(u32)) {
    dev_err(chan2dev(&td_chan.chan), "Incorrect length: %d\n",
    sg_dma_len(sg));
    return -EINVAL;
    }
    dev_dbg(chan2dev(&td_chan.chan), "desc: %p, addr: 0x%llx\n",
    dma_desc, (unsigned long long)sg_dma_address(sg));
    dma_desc[7] = (sg_dma_address(sg) >> 24) & 0xff;
    dma_desc[6] = (sg_dma_address(sg) >> 16) & 0xff;
    dma_desc[5] = (sg_dma_address(sg) >> 8) & 0xff;
    dma_desc[4] = (sg_dma_address(sg) >> 0) & 0xff;
    dma_desc[3] = (sg_dma_len(sg) >> 8) & 0xff;
    dma_desc[2] = (sg_dma_len(sg) >> 0) & 0xff;
    dma_desc[1] = 0x00;
    dma_desc[0] = 0x21 | (last ? 0x02 : 0); /* tran, valid */
    return 0;
    }
// Must be called with the spinlock held
#[no_mangle]
unsafe extern "C" fn __td_start_dma(td_chan: *mut timb_dma_chan) {
    static void __td_start_dma(struct timb_dma_chan *td_chan)
    {
    struct timb_dma_desc *td_desc;
    if (td_chan.ongoing) {
    dev_err(chan2dev(&td_chan.chan),
    "Transfer already ongoing\n");
    return;
    }
    td_desc = list_entry(td_chan.active_list.next, struct timb_dma_desc,
    desc_node);
    dev_dbg(chan2dev(&td_chan.chan),
    "td_chan: %p, chan: %d, membase: %p\n",
    td_chan, td_chan.chan.chan_id, td_chan.membase);
    if (td_chan.direction == DMA_DEV_TO_MEM) {
// descriptor address
    iowrite32(0, td_chan.membase + TIMBDMA_OFFS_RX_DHAR);
    iowrite32(td_desc.txd.phys, td_chan.membase +
    TIMBDMA_OFFS_RX_DLAR);
// Bytes per line
    iowrite32(td_chan.bytes_per_line, td_chan.membase +
    TIMBDMA_OFFS_RX_BPRR);
// enable RX
    iowrite32(TIMBDMA_RX_EN, td_chan.membase + TIMBDMA_OFFS_RX_ER);
    } else {
// address high
    iowrite32(0, td_chan.membase + TIMBDMA_OFFS_TX_DHAR);
    iowrite32(td_desc.txd.phys, td_chan.membase +
    TIMBDMA_OFFS_TX_DLAR);
    }
    td_chan.ongoing = true;
    if (td_desc.interrupt)
    __td_enable_chan_irq(td_chan);
    }
#[no_mangle]
unsafe extern "C" fn __td_finish(td_chan: *mut timb_dma_chan) {
    static void __td_finish(struct timb_dma_chan *td_chan)
    {
    struct dmaengine_desc_callback	cb;
    struct dma_async_tx_descriptor	*txd;
    struct timb_dma_desc		*td_desc;
// can happen if the descriptor is canceled
    if (list_empty(&td_chan.active_list))
    return;
    td_desc = list_entry(td_chan.active_list.next, struct timb_dma_desc,
    desc_node);
    txd = &td_desc.txd;
    dev_dbg(chan2dev(&td_chan.chan), "descriptor %u complete\n",
    txd.cookie);
// make sure to stop the transfer
    if (td_chan.direction == DMA_DEV_TO_MEM)
    iowrite32(0, td_chan.membase + TIMBDMA_OFFS_RX_ER);
// Currently no support for stopping DMA transfers
    else
    iowrite32(0, td_chan.membase + TIMBDMA_OFFS_TX_DLAR);
//
    dma_cookie_complete(txd);
    td_chan.ongoing = false;
    dmaengine_desc_get_callback(txd, &cb);
    list_move(&td_desc.desc_node, &td_chan.free_list);
    dma_descriptor_unmap(txd);
//
// The API requires that no submissions are done from a
// callback, so we don't need to drop the lock here
//
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn __td_ier_mask(td: *mut timb_dma) -> u32 {
    static u32 __td_ier_mask(struct timb_dma *td)
    {
    int i;
    let mut ret: u32 = 0;
    for (i = 0; i < td.dma.chancnt; i++) {
    struct timb_dma_chan *td_chan = td.channels + i;
    if (td_chan.ongoing) {
    struct timb_dma_desc *td_desc =
    list_entry(td_chan.active_list.next,
    struct timb_dma_desc, desc_node);
    if (td_desc.interrupt)
    ret |= 1 << i;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __td_start_next(td_chan: *mut timb_dma_chan) {
    static void __td_start_next(struct timb_dma_chan *td_chan)
    {
    struct timb_dma_desc *td_desc;
    BUG_ON(list_empty(&td_chan.queue));
    BUG_ON(td_chan.ongoing);
    td_desc = list_entry(td_chan.queue.next, struct timb_dma_desc,
    desc_node);
    dev_dbg(chan2dev(&td_chan.chan), "%s: started %u\n",
    __func__, td_desc.txd.cookie);
    list_move(&td_desc.desc_node, &td_chan.active_list);
    __td_start_dma(td_chan);
    }
#[no_mangle]
unsafe extern "C" fn td_tx_submit(txd: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t td_tx_submit(struct dma_async_tx_descriptor *txd)
    {
    struct timb_dma_desc *td_desc = container_of(txd, struct timb_dma_desc,
    txd);
    struct timb_dma_chan *td_chan = container_of(txd.chan,
    struct timb_dma_chan, chan);
    dma_cookie_t cookie;
    spin_lock_bh(&td_chan.lock);
    cookie = dma_cookie_assign(txd);
    if (list_empty(&td_chan.active_list)) {
    dev_dbg(chan2dev(txd.chan), "%s: started %u\n", __func__,
    txd.cookie);
    list_add_tail(&td_desc.desc_node, &td_chan.active_list);
    __td_start_dma(td_chan);
    } else {
    dev_dbg(chan2dev(txd.chan), "tx_submit: queued %u\n",
    txd.cookie);
    list_add_tail(&td_desc.desc_node, &td_chan.queue);
    }
    spin_unlock_bh(&td_chan.lock);
    return cookie;
    }
    static struct timb_dma_desc *td_alloc_init_desc(struct timb_dma_chan *td_chan)
    {
    struct dma_chan *chan = &td_chan.chan;
    struct timb_dma_desc *td_desc;
    int err;
    td_desc = kzalloc_obj(struct timb_dma_desc);
    if (!td_desc)
    goto out;
    td_desc.desc_list_len = td_chan.desc_elems * TIMB_DMA_DESC_SIZE;
    td_desc.desc_list = kzalloc(td_desc.desc_list_len, GFP_KERNEL);
    if (!td_desc.desc_list)
    goto err;
    dma_async_tx_descriptor_init(&td_desc.txd, chan);
    td_desc.txd.tx_submit = td_tx_submit;
    td_desc.txd.flags = DMA_CTRL_ACK;
    td_desc.txd.phys = dma_map_single(chan2dmadev(chan),
    td_desc.desc_list, td_desc.desc_list_len, DMA_TO_DEVICE);
    err = dma_mapping_error(chan2dmadev(chan), td_desc.txd.phys);
    if (err) {
    dev_err(chan2dev(chan), "DMA mapping error: %d\n", err);
    goto err;
    }
    return td_desc;
    err:
    kfree(td_desc.desc_list);
    kfree(td_desc);
    out:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn td_free_desc(td_desc: *mut timb_dma_desc) {
    static void td_free_desc(struct timb_dma_desc *td_desc)
    {
    dev_dbg(chan2dev(td_desc.txd.chan), "Freeing desc: %p\n", td_desc);
    dma_unmap_single(chan2dmadev(td_desc.txd.chan), td_desc.txd.phys,
    td_desc.desc_list_len, DMA_TO_DEVICE);
    kfree(td_desc.desc_list);
    kfree(td_desc);
    }
    static void td_desc_put(struct timb_dma_chan *td_chan,
    struct timb_dma_desc *td_desc)
    {
    dev_dbg(chan2dev(&td_chan.chan), "Putting desc: %p\n", td_desc);
    spin_lock_bh(&td_chan.lock);
    list_add(&td_desc.desc_node, &td_chan.free_list);
    spin_unlock_bh(&td_chan.lock);
    }
    static struct timb_dma_desc *td_desc_get(struct timb_dma_chan *td_chan)
    {
    struct timb_dma_desc *td_desc, *_td_desc;
    struct timb_dma_desc *ret = core::ptr::null_mut();
    spin_lock_bh(&td_chan.lock);
    list_for_each_entry_safe(td_desc, _td_desc, &td_chan.free_list,
    desc_node) {
    if (async_tx_test_ack(&td_desc.txd)) {
    list_del(&td_desc.desc_node);
    ret = td_desc;
    break;
    }
    dev_dbg(chan2dev(&td_chan.chan), "desc %p not ACKed\n",
    td_desc);
    }
    spin_unlock_bh(&td_chan.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn td_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int td_alloc_chan_resources(struct dma_chan *chan)
    {
    struct timb_dma_chan *td_chan =
    container_of(chan, struct timb_dma_chan, chan);
    int i;
    dev_dbg(chan2dev(chan), "%s: entry\n", __func__);
    BUG_ON(!list_empty(&td_chan.free_list));
    for (i = 0; i < td_chan.descs; i++) {
    struct timb_dma_desc *td_desc = td_alloc_init_desc(td_chan);
    if (!td_desc) {
    if (i)
    break;
    else {
    dev_err(chan2dev(chan),
    "Couldn't allocate any descriptors\n");
    return -ENOMEM;
    }
    }
    td_desc_put(td_chan, td_desc);
    }
    spin_lock_bh(&td_chan.lock);
    dma_cookie_init(chan);
    spin_unlock_bh(&td_chan.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn td_free_chan_resources(chan: *mut dma_chan) {
    static void td_free_chan_resources(struct dma_chan *chan)
    {
    struct timb_dma_chan *td_chan =
    container_of(chan, struct timb_dma_chan, chan);
    struct timb_dma_desc *td_desc, *_td_desc;
    LIST_HEAD(list);
    dev_dbg(chan2dev(chan), "%s: Entry\n", __func__);
// check that all descriptors are free
    BUG_ON(!list_empty(&td_chan.active_list));
    BUG_ON(!list_empty(&td_chan.queue));
    spin_lock_bh(&td_chan.lock);
    list_splice_init(&td_chan.free_list, &list);
    spin_unlock_bh(&td_chan.lock);
    list_for_each_entry_safe(td_desc, _td_desc, &list, desc_node) {
    dev_dbg(chan2dev(chan), "%s: Freeing desc: %p\n", __func__,
    td_desc);
    td_free_desc(td_desc);
    }
    }
    static enum dma_status td_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    enum dma_status ret;
    dev_dbg(chan2dev(chan), "%s: Entry\n", __func__);
    ret = dma_cookie_status(chan, cookie, txstate);
    dev_dbg(chan2dev(chan), "%s: exit, ret: %d\n", 	__func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn td_issue_pending(chan: *mut dma_chan) {
    static void td_issue_pending(struct dma_chan *chan)
    {
    struct timb_dma_chan *td_chan =
    container_of(chan, struct timb_dma_chan, chan);
    dev_dbg(chan2dev(chan), "%s: Entry\n", __func__);
    spin_lock_bh(&td_chan.lock);
    if (!list_empty(&td_chan.active_list))
// transfer ongoing
    if (__td_dma_done_ack(td_chan))
    __td_finish(td_chan);
    if (list_empty(&td_chan.active_list) && !list_empty(&td_chan.queue))
    __td_start_next(td_chan);
    spin_unlock_bh(&td_chan.lock);
    }
    static struct dma_async_tx_descriptor *td_prep_slave_sg(struct dma_chan *chan,
    struct scatterlist *sgl, unsigned int sg_len,
    enum dma_transfer_direction direction, unsigned long flags,
    void *context)
    {
    struct timb_dma_chan *td_chan =
    container_of(chan, struct timb_dma_chan, chan);
    struct timb_dma_desc *td_desc;
    struct scatterlist *sg;
    unsigned int i;
    let mut desc_usage: c_uint = 0;
    if (!sgl || !sg_len) {
    dev_err(chan2dev(chan), "%s: No SG list\n", __func__);
    return core::ptr::null_mut();
    }
// even channels are for RX, odd for TX
    if (td_chan.direction != direction) {
    dev_err(chan2dev(chan),
    "Requesting channel in wrong direction\n");
    return core::ptr::null_mut();
    }
    td_desc = td_desc_get(td_chan);
    if (!td_desc) {
    dev_err(chan2dev(chan), "Not enough descriptors available\n");
    return core::ptr::null_mut();
    }
    td_desc.interrupt = (flags & DMA_PREP_INTERRUPT) != 0;
    for_each_sg(sgl, sg, sg_len, i) {
    int err;
    if (desc_usage > td_desc.desc_list_len) {
    dev_err(chan2dev(chan), "No descriptor space\n");
    return core::ptr::null_mut();
    }
    err = td_fill_desc(td_chan, td_desc.desc_list + desc_usage, sg,
    i == (sg_len - 1));
    if (err) {
    dev_err(chan2dev(chan), "Failed to update desc: %d\n",
    err);
    td_desc_put(td_chan, td_desc);
    return core::ptr::null_mut();
    }
    desc_usage += TIMB_DMA_DESC_SIZE;
    }
    dma_sync_single_for_device(chan2dmadev(chan), td_desc.txd.phys,
    td_desc.desc_list_len, DMA_TO_DEVICE);
    return &td_desc.txd;
    }
#[no_mangle]
unsafe extern "C" fn td_terminate_all(chan: *mut dma_chan) -> c_int {
    static int td_terminate_all(struct dma_chan *chan)
    {
    struct timb_dma_chan *td_chan =
    container_of(chan, struct timb_dma_chan, chan);
    struct timb_dma_desc *td_desc, *_td_desc;
    dev_dbg(chan2dev(chan), "%s: Entry\n", __func__);
// first the easy part, put the queue into the free list
    spin_lock_bh(&td_chan.lock);
    list_for_each_entry_safe(td_desc, _td_desc, &td_chan.queue,
    desc_node)
    list_move(&td_desc.desc_node, &td_chan.free_list);
// now tear down the running
    __td_finish(td_chan);
    spin_unlock_bh(&td_chan.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn td_tasklet(t: *mut tasklet_struct) {
    static void td_tasklet(struct tasklet_struct *t)
    {
    struct timb_dma *td = from_tasklet(td, t, tasklet);
    u32 isr;
    u32 ipr;
    u32 ier;
    int i;
    isr = ioread32(td.membase + TIMBDMA_ISR);
    ipr = isr & __td_ier_mask(td);
// ack the interrupts
    iowrite32(ipr, td.membase + TIMBDMA_ISR);
    for (i = 0; i < td.dma.chancnt; i++)
    if (ipr & (1 << i)) {
    struct timb_dma_chan *td_chan = td.channels + i;
    spin_lock(&td_chan.lock);
    __td_finish(td_chan);
    if (!list_empty(&td_chan.queue))
    __td_start_next(td_chan);
    spin_unlock(&td_chan.lock);
    }
    ier = __td_ier_mask(td);
    iowrite32(ier, td.membase + TIMBDMA_IER);
    }
#[no_mangle]
unsafe extern "C" fn td_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t td_irq(int irq, void *devid)
    {
    struct timb_dma *td = devid;
    let mut ipr: u32 = ioread32(td.membase + TIMBDMA_IPR);
    if (ipr) {
// disable interrupts, will be re-enabled in tasklet
    iowrite32(0, td.membase + TIMBDMA_IER);
    tasklet_schedule(&td.tasklet);
    return IRQ_HANDLED;
    } else
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn td_probe(pdev: *mut platform_device) -> c_int {
    static int td_probe(struct platform_device *pdev)
    {
    struct timb_dma_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct timb_dma *td;
    struct resource *iomem;
    int irq;
    int err;
    int i;
    if (!pdata) {
    dev_err(&pdev.dev, "No platform data\n");
    return -EINVAL;
    }
    iomem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!iomem)
    return -EINVAL;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (!request_mem_region(iomem.start, resource_size(iomem),
    DRIVER_NAME))
    return -EBUSY;
    td  = kzalloc_flex(*td, channels, pdata.nr_channels);
    if (!td) {
    err = -ENOMEM;
    goto err_release_region;
    }
    dev_dbg(&pdev.dev, "Allocated TD: %p\n", td);
    td.membase = ioremap(iomem.start, resource_size(iomem));
    if (!td.membase) {
    dev_err(&pdev.dev, "Failed to remap I/O memory\n");
    err = -ENOMEM;
    goto err_free_mem;
    }
// 32bit addressing
    iowrite32(TIMBDMA_32BIT_ADDR, td.membase + TIMBDMA_ACR);
// disable and clear any interrupts
    iowrite32(0x0, td.membase + TIMBDMA_IER);
    iowrite32(0xFFFFFFFF, td.membase + TIMBDMA_ISR);
    tasklet_setup(&td.tasklet, td_tasklet);
    err = request_irq(irq, td_irq, IRQF_SHARED, DRIVER_NAME, td);
    if (err) {
    dev_err(&pdev.dev, "Failed to request IRQ\n");
    goto err_tasklet_kill;
    }
    td.dma.device_alloc_chan_resources	= td_alloc_chan_resources;
    td.dma.device_free_chan_resources	= td_free_chan_resources;
    td.dma.device_tx_status		= td_tx_status;
    td.dma.device_issue_pending		= td_issue_pending;
    dma_cap_set(DMA_SLAVE, td.dma.cap_mask);
    dma_cap_set(DMA_PRIVATE, td.dma.cap_mask);
    td.dma.device_prep_slave_sg = td_prep_slave_sg;
    td.dma.device_terminate_all = td_terminate_all;
    td.dma.dev = &pdev.dev;
    INIT_LIST_HEAD(&td.dma.channels);
    for (i = 0; i < pdata.nr_channels; i++) {
    struct timb_dma_chan *td_chan = &td.channels[i];
    struct timb_dma_platform_data_channel *pchan =
    pdata.channels + i;
// even channels are RX, odd are TX
    if ((i % 2) == pchan.rx) {
    dev_err(&pdev.dev, "Wrong channel configuration\n");
    err = -EINVAL;
    goto err_free_irq;
    }
    td_chan.chan.device = &td.dma;
    dma_cookie_init(&td_chan.chan);
    spin_lock_init(&td_chan.lock);
    INIT_LIST_HEAD(&td_chan.active_list);
    INIT_LIST_HEAD(&td_chan.queue);
    INIT_LIST_HEAD(&td_chan.free_list);
    td_chan.descs = pchan.descriptors;
    td_chan.desc_elems = pchan.descriptor_elements;
    td_chan.bytes_per_line = pchan.bytes_per_line;
    td_chan.direction = pchan.rx ? DMA_DEV_TO_MEM :
    DMA_MEM_TO_DEV;
    td_chan.membase = td.membase +
    (i / 2) * TIMBDMA_INSTANCE_OFFSET +
    (pchan.rx ? 0 : TIMBDMA_INSTANCE_TX_OFFSET);
    dev_dbg(&pdev.dev, "Chan: %d, membase: %p\n",
    i, td_chan.membase);
    list_add_tail(&td_chan.chan.device_node, &td.dma.channels);
    }
    err = dma_async_device_register(&td.dma);
    if (err) {
    dev_err(&pdev.dev, "Failed to register async device\n");
    goto err_free_irq;
    }
    platform_set_drvdata(pdev, td);
    dev_dbg(&pdev.dev, "Probe result: %d\n", err);
    return err;
    err_free_irq:
    free_irq(irq, td);
    err_tasklet_kill:
    tasklet_kill(&td.tasklet);
    iounmap(td.membase);
    err_free_mem:
    kfree(td);
    err_release_region:
    release_mem_region(iomem.start, resource_size(iomem));
    return err;
    }
#[no_mangle]
unsafe extern "C" fn td_remove(pdev: *mut platform_device) {
    static void td_remove(struct platform_device *pdev)
    {
    struct timb_dma *td = platform_get_drvdata(pdev);
    struct resource *iomem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    let mut irq: c_int = platform_get_irq(pdev, 0);
    dma_async_device_unregister(&td.dma);
    free_irq(irq, td);
    tasklet_kill(&td.tasklet);
    iounmap(td.membase);
    kfree(td);
    release_mem_region(iomem.start, resource_size(iomem));
    dev_dbg(&pdev.dev, "Removed...\n");
    }
    static struct platform_driver td_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    },
    .probe	= td_probe,
    .remove = td_remove,
    };
    module_platform_driver(td_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Timberdale DMA controller driver");
    MODULE_AUTHOR("Pelagicore AB <info@pelagicore.com>");
    MODULE_ALIAS("platform:"DRIVER_NAME);
