//! Automatically rewritten from C to Rust
//! Source: drivers/dma/altera-msgdma.c
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
// DMA driver for Altera mSGDMA IP core
//
// Copyright (C) 2017 Stefan Roese <sr@denx.de>
//
// Based on drivers/dma/xilinx/zynqmp_dma.c, which is:
// Copyright (C) 2016 Xilinx, Inc. All rights reserved.
//

pub const MSGDMA_DESC_NUM: c_int = 1024;
//
// struct msgdma_extended_desc - implements an extended descriptor
// @read_addr_lo: data buffer source address low bits
// @write_addr_lo: data buffer destination address low bits
// @len: the number of bytes to transfer per descriptor
// @burst_seq_num: bit 31:24 write burst
// bit 23:16 read burst
// bit 15:00 sequence number
// @stride: bit 31:16 write stride
// bit 15:00 read stride
// @read_addr_hi: data buffer source address high bits
// @write_addr_hi: data buffer destination address high bits
// @control: characteristics of the transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_extended_desc {
    pub read_addr_lo: u32,
    pub write_addr_lo: u32,
    pub len: u32,
    pub burst_seq_num: u32,
    pub stride: u32,
    pub read_addr_hi: u32,
    pub write_addr_hi: u32,
    pub control: u32,
}

// mSGDMA descriptor control field bit definitions

//
// Writing "1" the "go" bit commits the entire descriptor into the
// descriptor FIFO(s)
//

// Tx buffer control flags

    MSGDMA_DESC_CTL_TR_ERR_IRQ |	\
    MSGDMA_DESC_CTL_GO)

    MSGDMA_DESC_CTL_GO)

    MSGDMA_DESC_CTL_TR_COMP_IRQ |	\
    MSGDMA_DESC_CTL_TR_ERR_IRQ |	\
    MSGDMA_DESC_CTL_GO)

    MSGDMA_DESC_CTL_GEN_EOP |	\
    MSGDMA_DESC_CTL_TR_COMP_IRQ |	\
    MSGDMA_DESC_CTL_TR_ERR_IRQ |	\
    MSGDMA_DESC_CTL_GO)

    MSGDMA_DESC_CTL_END_ON_LEN |	\
    MSGDMA_DESC_CTL_TR_COMP_IRQ |	\
    MSGDMA_DESC_CTL_EARLY_IRQ |	\
    MSGDMA_DESC_CTL_TR_ERR_IRQ |	\
    MSGDMA_DESC_CTL_GO)
// mSGDMA extended descriptor stride definitions
pub const MSGDMA_DESC_STRIDE_RD: c_uint = 0x00000001;
pub const MSGDMA_DESC_STRIDE_WR: c_uint = 0x00010000;
pub const MSGDMA_DESC_STRIDE_RW: c_uint = 0x00010001;
// mSGDMA dispatcher control and status register map
pub const MSGDMA_CSR_STATUS: c_uint = 0x00	/* Read / Clear */;
pub const MSGDMA_CSR_CONTROL: c_uint = 0x04	/* Read / Write */;
pub const MSGDMA_CSR_RW_FILL_LEVEL: c_uint = 0x08	/* 31:16 - write fill level */;
// 15:00 - read fill level
pub const MSGDMA_CSR_RESP_FILL_LEVEL: c_uint = 0x0c	/* response FIFO fill level */;
pub const MSGDMA_CSR_RW_SEQ_NUM: c_uint = 0x10	/* 31:16 - write seq number */;
// 15:00 - read seq number
// mSGDMA CSR status register bit definitions

    MSGDMA_CSR_STAT_RESP_BUF_EMPTY)
// mSGDMA CSR control register bit definitions

// mSGDMA CSR fill level bits

// mSGDMA response register map
pub const MSGDMA_RESP_BYTES_TRANSFERRED: c_uint = 0x00;
pub const MSGDMA_RESP_STATUS: c_uint = 0x04;
// mSGDMA response register bit definitions

pub const MSGDMA_RESP_ERR_MASK: c_uint = 0xff;
//
// struct msgdma_sw_desc - implements a sw descriptor
// @async_tx: support for the async_tx api
// @hw_desc: associated HW descriptor
// @node: node to move from the free list to the tx list
// @tx_list: transmit list node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_sw_desc {
    pub async_tx: dma_async_tx_descriptor,
    pub hw_desc: msgdma_extended_desc,
    pub node: list_head,
    pub tx_list: list_head,
}

//
// struct msgdma_device - DMA device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_device {
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub irq_tasklet: tasklet_struct,
    pub pending_list: list_head,
    pub free_list: list_head,
    pub active_list: list_head,
    pub done_list: list_head,
    pub desc_free_cnt: u32,
    pub idle: bool,
    pub dmadev: dma_device,
    pub dmachan: dma_chan,
    pub hw_desq: dma_addr_t,
    pub sw_desq: *mut msgdma_sw_desc,
    pub npendings: c_uint,
    pub slave_cfg: dma_slave_config,
    pub irq: c_int,
// mSGDMA controller
    pub csr: *mut void __iomem,
// mSGDMA descriptors
    pub desc: *mut void __iomem,
// mSGDMA response
    pub resp: *mut void __iomem,
}

//
// msgdma_get_descriptor - Get the sw descriptor from the pool
// @mdev: Pointer to the Altera mSGDMA device structure
//
// Return: The sw descriptor
//
    static struct msgdma_sw_desc *msgdma_get_descriptor(struct msgdma_device *mdev)
    {
    struct msgdma_sw_desc *desc;
    unsigned long flags;
    spin_lock_irqsave(&mdev.lock, flags);
    desc = list_first_entry(&mdev.free_list, struct msgdma_sw_desc, node);
    list_del(&desc.node);
    spin_unlock_irqrestore(&mdev.lock, flags);
    INIT_LIST_HEAD(&desc.tx_list);
    return desc;
    }
//
// msgdma_free_descriptor - Issue pending transactions
// @mdev: Pointer to the Altera mSGDMA device structure
// @desc: Transaction descriptor pointer
//
    static void msgdma_free_descriptor(struct msgdma_device *mdev,
    struct msgdma_sw_desc *desc)
    {
    struct msgdma_sw_desc *child, *next;
    mdev.desc_free_cnt++;
    list_move_tail(&desc.node, &mdev.free_list);
    list_for_each_entry_safe(child, next, &desc.tx_list, node) {
    mdev.desc_free_cnt++;
    list_move_tail(&child.node, &mdev.free_list);
    }
    }
//
// msgdma_free_desc_list - Free descriptors list
// @mdev: Pointer to the Altera mSGDMA device structure
// @list: List to parse and delete the descriptor
//
    static void msgdma_free_desc_list(struct msgdma_device *mdev,
    struct list_head *list)
    {
    struct msgdma_sw_desc *desc, *next;
    list_for_each_entry_safe(desc, next, list, node)
    msgdma_free_descriptor(mdev, desc);
    }
//
// msgdma_desc_config - Configure the descriptor
// @desc: Hw descriptor pointer
// @dst: Destination buffer address
// @src: Source buffer address
// @len: Transfer length
// @stride: Read/write stride value to set
//
    static void msgdma_desc_config(struct msgdma_extended_desc *desc,
    dma_addr_t dst, dma_addr_t src, size_t len,
    u32 stride)
    {
// Set lower 32bits of src & dst addresses in the descriptor
    desc.read_addr_lo = lower_32_bits(src);
    desc.write_addr_lo = lower_32_bits(dst);
// Set upper 32bits of src & dst addresses in the descriptor
    desc.read_addr_hi = upper_32_bits(src);
    desc.write_addr_hi = upper_32_bits(dst);
    desc.len = len;
    desc.stride = stride;
    desc.burst_seq_num = 0;	/* 0 will result in max burst length */
//
// Don't set interrupt on xfer end yet, this will be done later
// for the "last" descriptor
//
    desc.control = MSGDMA_DESC_CTL_TR_ERR_IRQ | MSGDMA_DESC_CTL_GO |
    MSGDMA_DESC_CTL_END_ON_LEN;
    }
//
// msgdma_desc_config_eod - Mark the descriptor as end descriptor
// @desc: Hw descriptor pointer
//
#[no_mangle]
unsafe extern "C" fn msgdma_desc_config_eod(desc: *mut msgdma_extended_desc) {
    static void msgdma_desc_config_eod(struct msgdma_extended_desc *desc)
    {
    desc.control |= MSGDMA_DESC_CTL_TR_COMP_IRQ;
    }
//
// msgdma_tx_submit - Submit DMA transaction
// @tx: Async transaction descriptor pointer
//
// Return: cookie value
//
#[no_mangle]
unsafe extern "C" fn msgdma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t msgdma_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct msgdma_device *mdev = to_mdev(tx.chan);
    struct msgdma_sw_desc *new;
    dma_cookie_t cookie;
    unsigned long flags;
    new = tx_to_desc(tx);
    spin_lock_irqsave(&mdev.lock, flags);
    cookie = dma_cookie_assign(tx);
    list_add_tail(&new.node, &mdev.pending_list);
    spin_unlock_irqrestore(&mdev.lock, flags);
    return cookie;
    }
//
// msgdma_prep_memcpy - prepare descriptors for memcpy transaction
// @dchan: DMA channel
// @dma_dst: Destination buffer address
// @dma_src: Source buffer address
// @len: Transfer length
// @flags: transfer ack flags
//
// Return: Async transaction descriptor on success and NULL on failure
//
    static struct dma_async_tx_descriptor *
    msgdma_prep_memcpy(struct dma_chan *dchan, dma_addr_t dma_dst,
    dma_addr_t dma_src, size_t len, ulong flags)
    {
    struct msgdma_device *mdev = to_mdev(dchan);
    struct msgdma_sw_desc *new, *first = core::ptr::null_mut();
    struct msgdma_extended_desc *desc;
    size_t copy;
    u32 desc_cnt;
    unsigned long irqflags;
    desc_cnt = DIV_ROUND_UP(len, MSGDMA_MAX_TRANS_LEN);
    spin_lock_irqsave(&mdev.lock, irqflags);
    if (desc_cnt > mdev.desc_free_cnt) {
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    dev_dbg(mdev.dev, "mdev %p descs are not available\n", mdev);
    return core::ptr::null_mut();
    }
    mdev.desc_free_cnt -= desc_cnt;
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    do {
// Allocate and populate the descriptor
    new = msgdma_get_descriptor(mdev);
    copy = min_t(size_t, len, MSGDMA_MAX_TRANS_LEN);
    desc = &new.hw_desc;
    msgdma_desc_config(desc, dma_dst, dma_src, copy,
    MSGDMA_DESC_STRIDE_RW);
    len -= copy;
    dma_src += copy;
    dma_dst += copy;
    if (!first)
    first = new;
    else
    list_add_tail(&new.node, &first.tx_list);
    } while (len);
    msgdma_desc_config_eod(desc);
    async_tx_ack(&first.async_tx);
    first.async_tx.flags = flags;
    return &first.async_tx;
    }
//
// msgdma_prep_slave_sg - prepare descriptors for a slave sg transaction
//
// @dchan: DMA channel
// @sgl: Destination scatter list
// @sg_len: Number of entries in destination scatter list
// @dir: DMA transfer direction
// @flags: transfer ack flags
// @context: transfer context (unused)
//
    static struct dma_async_tx_descriptor *
    msgdma_prep_slave_sg(struct dma_chan *dchan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct msgdma_device *mdev = to_mdev(dchan);
    struct dma_slave_config *cfg = &mdev.slave_cfg;
    struct msgdma_sw_desc *new, *first = core::ptr::null_mut();
    void *desc = core::ptr::null_mut();
    size_t len, avail;
    dma_addr_t dma_dst, dma_src;
    u32 desc_cnt;
    u32 stride;
    unsigned long irqflags;
    desc_cnt = sg_nents_for_dma(sgl, sg_len, MSGDMA_MAX_TRANS_LEN);
    spin_lock_irqsave(&mdev.lock, irqflags);
    if (desc_cnt > mdev.desc_free_cnt) {
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    dev_dbg(mdev.dev, "mdev %p descs are not available\n", mdev);
    return core::ptr::null_mut();
    }
    mdev.desc_free_cnt -= desc_cnt;
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    avail = sg_dma_len(sgl);
// Run until we are out of scatterlist entries
    while (true) {
// Allocate and populate the descriptor
    new = msgdma_get_descriptor(mdev);
    desc = &new.hw_desc;
    len = min_t(size_t, avail, MSGDMA_MAX_TRANS_LEN);
    if (dir == DMA_MEM_TO_DEV) {
    dma_src = sg_dma_address(sgl) + sg_dma_len(sgl) - avail;
    dma_dst = cfg.dst_addr;
    stride = MSGDMA_DESC_STRIDE_RD;
    } else {
    dma_src = cfg.src_addr;
    dma_dst = sg_dma_address(sgl) + sg_dma_len(sgl) - avail;
    stride = MSGDMA_DESC_STRIDE_WR;
    }
    msgdma_desc_config(desc, dma_dst, dma_src, len, stride);
    avail -= len;
    if (!first)
    first = new;
    else
    list_add_tail(&new.node, &first.tx_list);
// Fetch the next scatterlist entry
    if (avail == 0) {
    if (sg_len == 0)
    break;
    sgl = sg_next(sgl);
    if (sgl == core::ptr::null_mut())
    break;
    sg_len--;
    avail = sg_dma_len(sgl);
    }
    }
    msgdma_desc_config_eod(desc);
    first.async_tx.flags = flags;
    return &first.async_tx;
    }
    static int msgdma_dma_config(struct dma_chan *dchan,
    struct dma_slave_config *config)
    {
    struct msgdma_device *mdev = to_mdev(dchan);
    memcpy(&mdev.slave_cfg, config, sizeof(*config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msgdma_reset(mdev: *mut msgdma_device) {
    static void msgdma_reset(struct msgdma_device *mdev)
    {
    u32 val;
    int ret;
// Reset mSGDMA
    iowrite32(MSGDMA_CSR_STAT_MASK, mdev.csr + MSGDMA_CSR_STATUS);
    iowrite32(MSGDMA_CSR_CTL_RESET, mdev.csr + MSGDMA_CSR_CONTROL);
    ret = readl_poll_timeout(mdev.csr + MSGDMA_CSR_STATUS, val,
    (val & MSGDMA_CSR_STAT_RESETTING) == 0,
    1, 10000);
    if (ret)
    dev_err(mdev.dev, "DMA channel did not reset\n");
// Clear all status bits
    iowrite32(MSGDMA_CSR_STAT_MASK, mdev.csr + MSGDMA_CSR_STATUS);
// Enable the DMA controller including interrupts
    iowrite32(MSGDMA_CSR_CTL_STOP_ON_ERR | MSGDMA_CSR_CTL_STOP_ON_EARLY |
    MSGDMA_CSR_CTL_GLOBAL_INTR, mdev.csr + MSGDMA_CSR_CONTROL);
    mdev.idle = true;
    };
    static void msgdma_copy_one(struct msgdma_device *mdev,
    struct msgdma_sw_desc *desc)
    {
    void __iomem *hw_desc = mdev.desc;
// Ensure control is the last field — required for correct FIFO flush ordering
    static_assert(offsetof(struct msgdma_extended_desc, control) ==
    sizeof(struct msgdma_extended_desc) - sizeof(u32),
    "control must be the last field in msgdma_extended_desc");
//
// Check if the DESC FIFO it not full. If its full, we need to wait
// for at least one entry to become free again
//
    while (ioread32(mdev.csr + MSGDMA_CSR_STATUS) &
    MSGDMA_CSR_STAT_DESC_BUF_FULL)
    mdelay(1);
// Ensure control is the last field — required for correct FIFO flush ordering
    static_assert(offsetof(struct msgdma_extended_desc, control) ==
    sizeof(struct msgdma_extended_desc) - sizeof(u32),
    "control must be the last field in msgdma_extended_desc");
//
// Copy the descriptor into the descriptor FIFO of the DMA controller,
// excluding the control word. The FIFO is flushed and the descriptor
// becomes valid once the control word is written last.
//
    memcpy_toio(hw_desc, &desc.hw_desc,
    offsetof(struct msgdma_extended_desc, control));
// Write control word last to flush this descriptor into the FIFO
    mdev.idle = false;
    wmb();
    iowrite32(desc.hw_desc.control, hw_desc +
    offsetof(struct msgdma_extended_desc, control));
    wmb();
    }
//
// msgdma_copy_desc_to_fifo - copy descriptor(s) into controller FIFO
// @mdev: Pointer to the Altera mSGDMA device structure
// @desc: Transaction descriptor pointer
//
    static void msgdma_copy_desc_to_fifo(struct msgdma_device *mdev,
    struct msgdma_sw_desc *desc)
    {
    struct msgdma_sw_desc *sdesc, *next;
    msgdma_copy_one(mdev, desc);
    list_for_each_entry_safe(sdesc, next, &desc.tx_list, node)
    msgdma_copy_one(mdev, sdesc);
    }
//
// msgdma_start_transfer - Initiate the new transfer
// @mdev: Pointer to the Altera mSGDMA device structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_start_transfer(mdev: *mut msgdma_device) {
    static void msgdma_start_transfer(struct msgdma_device *mdev)
    {
    struct msgdma_sw_desc *desc;
    if (!mdev.idle)
    return;
    desc = list_first_entry_or_null(&mdev.pending_list,
    struct msgdma_sw_desc, node);
    if (!desc)
    return;
    list_splice_tail_init(&mdev.pending_list, &mdev.active_list);
    msgdma_copy_desc_to_fifo(mdev, desc);
    }
//
// msgdma_issue_pending - Issue pending transactions
// @chan: DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn msgdma_issue_pending(chan: *mut dma_chan) {
    static void msgdma_issue_pending(struct dma_chan *chan)
    {
    struct msgdma_device *mdev = to_mdev(chan);
    unsigned long flags;
    spin_lock_irqsave(&mdev.lock, flags);
    msgdma_start_transfer(mdev);
    spin_unlock_irqrestore(&mdev.lock, flags);
    }
//
// msgdma_chan_desc_cleanup - Cleanup the completed descriptors
// @mdev: Pointer to the Altera mSGDMA device structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_chan_desc_cleanup(mdev: *mut msgdma_device) {
    static void msgdma_chan_desc_cleanup(struct msgdma_device *mdev)
    {
    struct msgdma_sw_desc *desc, *next;
    unsigned long irqflags;
    spin_lock_irqsave(&mdev.lock, irqflags);
    list_for_each_entry_safe(desc, next, &mdev.done_list, node) {
    struct dmaengine_desc_callback cb;
    dmaengine_desc_get_callback(&desc.async_tx, &cb);
    if (dmaengine_desc_callback_valid(&cb)) {
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    spin_lock_irqsave(&mdev.lock, irqflags);
    }
// Run any dependencies, then free the descriptor
    msgdma_free_descriptor(mdev, desc);
    }
    spin_unlock_irqrestore(&mdev.lock, irqflags);
    }
//
// msgdma_complete_descriptor - Mark the active descriptor as complete
// @mdev: Pointer to the Altera mSGDMA device structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_complete_descriptor(mdev: *mut msgdma_device) {
    static void msgdma_complete_descriptor(struct msgdma_device *mdev)
    {
    struct msgdma_sw_desc *desc;
    desc = list_first_entry_or_null(&mdev.active_list,
    struct msgdma_sw_desc, node);
    if (!desc)
    return;
    list_del(&desc.node);
    dma_cookie_complete(&desc.async_tx);
    list_add_tail(&desc.node, &mdev.done_list);
    }
//
// msgdma_free_descriptors - Free channel descriptors
// @mdev: Pointer to the Altera mSGDMA device structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_free_descriptors(mdev: *mut msgdma_device) {
    static void msgdma_free_descriptors(struct msgdma_device *mdev)
    {
    msgdma_free_desc_list(mdev, &mdev.active_list);
    msgdma_free_desc_list(mdev, &mdev.pending_list);
    msgdma_free_desc_list(mdev, &mdev.done_list);
    }
//
// msgdma_free_chan_resources - Free channel resources
// @dchan: DMA channel pointer
//
#[no_mangle]
unsafe extern "C" fn msgdma_free_chan_resources(dchan: *mut dma_chan) {
    static void msgdma_free_chan_resources(struct dma_chan *dchan)
    {
    struct msgdma_device *mdev = to_mdev(dchan);
    unsigned long flags;
    spin_lock_irqsave(&mdev.lock, flags);
    msgdma_free_descriptors(mdev);
    spin_unlock_irqrestore(&mdev.lock, flags);
    kfree(mdev.sw_desq);
    }
//
// msgdma_alloc_chan_resources - Allocate channel resources
// @dchan: DMA channel
//
// Return: Number of descriptors on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn msgdma_alloc_chan_resources(dchan: *mut dma_chan) -> c_int {
    static int msgdma_alloc_chan_resources(struct dma_chan *dchan)
    {
    struct msgdma_device *mdev = to_mdev(dchan);
    struct msgdma_sw_desc *desc;
    int i;
    mdev.sw_desq = kzalloc_objs(*desc, MSGDMA_DESC_NUM, GFP_NOWAIT);
    if (!mdev.sw_desq)
    return -ENOMEM;
    mdev.idle = true;
    mdev.desc_free_cnt = MSGDMA_DESC_NUM;
    INIT_LIST_HEAD(&mdev.free_list);
    for (i = 0; i < MSGDMA_DESC_NUM; i++) {
    desc = mdev.sw_desq + i;
    dma_async_tx_descriptor_init(&desc.async_tx, &mdev.dmachan);
    desc.async_tx.tx_submit = msgdma_tx_submit;
    list_add_tail(&desc.node, &mdev.free_list);
    }
    return MSGDMA_DESC_NUM;
    }
//
// msgdma_tasklet - Schedule completion tasklet
// @t: Pointer to the Altera sSGDMA channel structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_tasklet(t: *mut tasklet_struct) {
    static void msgdma_tasklet(struct tasklet_struct *t)
    {
    struct msgdma_device *mdev = from_tasklet(mdev, t, irq_tasklet);
    u32 count;
    u32 __maybe_unused size;
    u32 __maybe_unused status;
    unsigned long flags;
    spin_lock_irqsave(&mdev.lock, flags);
    if (mdev.resp) {
// Read number of responses that are available
    count = ioread32(mdev.csr + MSGDMA_CSR_RESP_FILL_LEVEL);
    dev_dbg(mdev.dev, "%s (%d): response count=%d\n",
    __func__, __LINE__, count);
    } else {
    count = 1;
    }
    while (count--) {
//
// Read both longwords to purge this response from the FIFO
// On Avalon-MM implementations, size and status do not
// have any real values, like transferred bytes or error
// bits. So we need to just drop these values.
//
    if (mdev.resp) {
    size = ioread32(mdev.resp +
    MSGDMA_RESP_BYTES_TRANSFERRED);
    status = ioread32(mdev.resp +
    MSGDMA_RESP_STATUS);
    }
    msgdma_complete_descriptor(mdev);
    }
    spin_unlock_irqrestore(&mdev.lock, flags);
    msgdma_chan_desc_cleanup(mdev);
    }
//
// msgdma_irq_handler - Altera mSGDMA Interrupt handler
// @irq: IRQ number
// @data: Pointer to the Altera mSGDMA device structure
//
// Return: IRQ_HANDLED/IRQ_NONE
//
#[no_mangle]
unsafe extern "C" fn msgdma_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t msgdma_irq_handler(int irq, void *data)
    {
    struct msgdma_device *mdev = data;
    u32 status;
    status = ioread32(mdev.csr + MSGDMA_CSR_STATUS);
    if ((status & MSGDMA_CSR_STAT_BUSY) == 0) {
// Start next transfer if the DMA controller is idle
    spin_lock(&mdev.lock);
    mdev.idle = true;
    msgdma_start_transfer(mdev);
    spin_unlock(&mdev.lock);
    }
    tasklet_schedule(&mdev.irq_tasklet);
// Clear interrupt in mSGDMA controller
    iowrite32(MSGDMA_CSR_STAT_IRQ, mdev.csr + MSGDMA_CSR_STATUS);
    return IRQ_HANDLED;
    }
//
// msgdma_dev_remove() - Device remove function
// @mdev: Pointer to the Altera mSGDMA device structure
//
#[no_mangle]
unsafe extern "C" fn msgdma_dev_remove(mdev: *mut msgdma_device) {
    static void msgdma_dev_remove(struct msgdma_device *mdev)
    {
    if (!mdev)
    return;
    devm_free_irq(mdev.dev, mdev.irq, mdev);
    tasklet_kill(&mdev.irq_tasklet);
    list_del(&mdev.dmachan.device_node);
    }
    static int request_and_map(struct platform_device *pdev, const char *name,
    struct resource **res, void __iomem **ptr,
    bool optional)
    {
    struct resource *region;
    struct device *device = &pdev.dev;
// res = platform_get_resource_byname(pdev, IORESOURCE_MEM, name);
    if (*res == core::ptr::null_mut()) {
    if (optional) {
// ptr = NULL;
    dev_info(device, "optional resource %s not defined\n",
    name);
    return 0;
    }
    dev_err(device, "mandatory resource %s not defined\n", name);
    return -ENODEV;
    }
    region = devm_request_mem_region(device, (*res).start,
    resource_size(*res), dev_name(device));
    if (region == core::ptr::null_mut()) {
    dev_err(device, "unable to request %s\n", name);
    return -EBUSY;
    }
// ptr = devm_ioremap(device, region->start,
    resource_size(region));
    if (*ptr == core::ptr::null_mut()) {
    dev_err(device, "ioremap of %s failed!", name);
    return -ENOMEM;
    }
    return 0;
    }
//
// msgdma_probe - Driver probe function
// @pdev: Pointer to the platform_device structure
//
// Return: '0' on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn msgdma_probe(pdev: *mut platform_device) -> c_int {
    static int msgdma_probe(struct platform_device *pdev)
    {
    struct msgdma_device *mdev;
    struct dma_device *dma_dev;
    struct resource *dma_res;
    int ret;
    mdev = devm_kzalloc(&pdev.dev, sizeof(*mdev), GFP_NOWAIT);
    if (!mdev)
    return -ENOMEM;
    mdev.dev = &pdev.dev;
// Map CSR space
    ret = request_and_map(pdev, "csr", &dma_res, &mdev.csr, false);
    if (ret)
    return ret;
// Map (extended) descriptor space
    ret = request_and_map(pdev, "desc", &dma_res, &mdev.desc, false);
    if (ret)
    return ret;
// Map response space
    ret = request_and_map(pdev, "resp", &dma_res, &mdev.resp, true);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, mdev);
// Get interrupt nr from platform data
    mdev.irq = platform_get_irq(pdev, 0);
    if (mdev.irq < 0)
    return -ENXIO;
    ret = devm_request_irq(&pdev.dev, mdev.irq, msgdma_irq_handler,
    0, dev_name(&pdev.dev), mdev);
    if (ret)
    return ret;
    tasklet_setup(&mdev.irq_tasklet, msgdma_tasklet);
    dma_cookie_init(&mdev.dmachan);
    spin_lock_init(&mdev.lock);
    INIT_LIST_HEAD(&mdev.active_list);
    INIT_LIST_HEAD(&mdev.pending_list);
    INIT_LIST_HEAD(&mdev.done_list);
    INIT_LIST_HEAD(&mdev.free_list);
    dma_dev = &mdev.dmadev;
// Set DMA capabilities
    dma_cap_zero(dma_dev.cap_mask);
    dma_cap_set(DMA_MEMCPY, dma_dev.cap_mask);
    dma_cap_set(DMA_SLAVE, dma_dev.cap_mask);
    dma_dev.src_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    dma_dev.dst_addr_widths = BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    dma_dev.directions = BIT(DMA_MEM_TO_DEV) | BIT(DMA_DEV_TO_MEM) |
    BIT(DMA_MEM_TO_MEM);
    dma_dev.residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
// Init DMA link list
    INIT_LIST_HEAD(&dma_dev.channels);
// Set base routines
    dma_dev.device_tx_status = dma_cookie_status;
    dma_dev.device_issue_pending = msgdma_issue_pending;
    dma_dev.dev = &pdev.dev;
    dma_dev.copy_align = DMAENGINE_ALIGN_4_BYTES;
    dma_dev.device_prep_dma_memcpy = msgdma_prep_memcpy;
    dma_dev.device_prep_slave_sg = msgdma_prep_slave_sg;
    dma_dev.device_config = msgdma_dma_config;
    dma_dev.device_alloc_chan_resources = msgdma_alloc_chan_resources;
    dma_dev.device_free_chan_resources = msgdma_free_chan_resources;
    mdev.dmachan.device = dma_dev;
    list_add_tail(&mdev.dmachan.device_node, &dma_dev.channels);
// Set DMA mask to 64 bits
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (ret) {
    dev_warn(&pdev.dev, "unable to set coherent mask to 64");
    goto fail;
    }
    msgdma_reset(mdev);
    ret = dma_async_device_register(dma_dev);
    if (ret)
    goto fail;
    ret = of_dma_controller_register(pdev.dev.of_node,
    of_dma_xlate_by_chan_id, dma_dev);
    if (ret == -EINVAL)
    dev_warn(&pdev.dev, "device was not probed from DT");
#[no_mangle]
pub unsafe extern "C" fn if(-ENODEV: ret && ret !=) -> else {
    else if (ret && ret != -ENODEV)
    goto fail;
    dev_notice(&pdev.dev, "Altera mSGDMA driver probe success\n");
    return 0;
    fail:
    msgdma_dev_remove(mdev);
    return ret;
    }
//
// msgdma_remove() - Driver remove function
// @pdev: Pointer to the platform_device structure
//
// Return: Always '0'
//
#[no_mangle]
unsafe extern "C" fn msgdma_remove(pdev: *mut platform_device) {
    static void msgdma_remove(struct platform_device *pdev)
    {
    struct msgdma_device *mdev = platform_get_drvdata(pdev);
    if (pdev.dev.of_node)
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&mdev.dmadev);
    msgdma_dev_remove(mdev);
    dev_notice(&pdev.dev, "Altera mSGDMA driver removed\n");
    }

    static const struct of_device_id msgdma_match[] = {
    { .compatible = "altr,socfpga-msgdma", },
    { }
    };
    MODULE_DEVICE_TABLE(of, msgdma_match);

    static struct platform_driver msgdma_driver = {
    .driver = {
    .name = "altera-msgdma",
    .of_match_table = of_match_ptr(msgdma_match),
    },
    .probe = msgdma_probe,
    .remove = msgdma_remove,
    };
    module_platform_driver(msgdma_driver);
    MODULE_ALIAS("platform:altera-msgdma");
    MODULE_DESCRIPTION("Altera mSGDMA driver");
    MODULE_AUTHOR("Stefan Roese <sr@denx.de>");
    MODULE_LICENSE("GPL");
