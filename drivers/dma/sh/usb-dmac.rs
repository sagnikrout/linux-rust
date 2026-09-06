//! Automatically rewritten from C to Rust
//! Source: drivers/dma/sh/usb-dmac.c
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
// Renesas USB DMA Controller Driver
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
// based on rcar-dmac.c
// Copyright (C) 2014 Renesas Electronics Inc.
// Author: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// struct usb_dmac_sg - Descriptor for a hardware transfer
// @mem_addr: memory address
// @size: transfer size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dmac_sg {
    pub mem_addr: dma_addr_t,
    pub size: u32,
}

//
// struct usb_dmac_desc - USB DMA Transfer Descriptor
// @vd: base virtual channel DMA transaction descriptor
// @direction: direction of the DMA transfer
// @sg_allocated_len: length of allocated sg
// @sg_len: length of sg
// @sg_index: index of sg
// @residue: residue after the DMAC completed a transfer
// @node: node for desc_got and desc_freed
// @done_cookie: cookie after the DMAC completed a transfer
// @sg: information for the transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dmac_desc {
    pub vd: virt_dma_desc,
    pub direction: enum dma_transfer_direction,
    pub sg_allocated_len: c_uint,
    pub sg_len: c_uint,
    pub sg_index: c_uint,
    pub residue: u32,
    pub node: list_head,
    pub done_cookie: dma_cookie_t,
    pub __counted_by(sg_allocated_len): usb_dmac_sg sg[],
}

//
// struct usb_dmac_chan - USB DMA Controller Channel
// @vc: base virtual DMA channel object
// @iomem: channel I/O memory base
// @index: index of this channel in the controller
// @irq: irq number of this channel
// @desc: the current descriptor
// @descs_allocated: number of descriptors allocated
// @desc_got: got descriptors
// @desc_freed: freed descriptors after the DMAC completed a transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dmac_chan {
    pub vc: virt_dma_chan,
    pub iomem: *mut void __iomem,
    pub index: c_uint,
    pub irq: c_int,
    pub desc: *mut usb_dmac_desc,
    pub descs_allocated: c_int,
    pub desc_got: list_head,
    pub desc_freed: list_head,
}

//
// struct usb_dmac - USB DMA Controller
// @engine: base DMA engine object
// @dev: the hardware device
// @iomem: remapped I/O memory base
// @n_channels: number of available channels
// @channels: array of DMAC channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dmac {
    pub engine: dma_device,
    pub dev: *mut device,
    pub iomem: *mut void __iomem,
    pub n_channels: c_uint,
    pub channels: *mut usb_dmac_chan,
}

// -----------------------------------------------------------------------------
// Registers
//

pub const USB_DMASWR: c_uint = 0x0008;

pub const USB_DMAOR: c_uint = 0x0060;

pub const USB_DMASAR: c_uint = 0x0000;
pub const USB_DMADAR: c_uint = 0x0004;
pub const USB_DMATCR: c_uint = 0x0008;
pub const USB_DMATCR_MASK: c_uint = 0x00ffffff;
pub const USB_DMACHCR: c_uint = 0x0014;

pub const USB_DMATEND: c_uint = 0x0018;
// Hardcode the xfer_shift to 5 (32bytes)
pub const USB_DMAC_XFER_SHIFT: c_int = 5;

// for descriptors
pub const USB_DMAC_INITIAL_NR_DESC: c_int = 16;
pub const USB_DMAC_INITIAL_NR_SG: c_int = 8;
// -----------------------------------------------------------------------------
// Device access
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_write(dmac: *mut usb_dmac, reg: u32, data: u32) {
    static void usb_dmac_write(struct usb_dmac *dmac, u32 reg, u32 data)
    {
    writel(data, dmac.iomem + reg);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_read(dmac: *mut usb_dmac, reg: u32) -> u32 {
    static u32 usb_dmac_read(struct usb_dmac *dmac, u32 reg)
    {
    return readl(dmac.iomem + reg);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_read(chan: *mut usb_dmac_chan, reg: u32) -> u32 {
    static u32 usb_dmac_chan_read(struct usb_dmac_chan *chan, u32 reg)
    {
    return readl(chan.iomem + reg);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_write(chan: *mut usb_dmac_chan, reg: u32, data: u32) {
    static void usb_dmac_chan_write(struct usb_dmac_chan *chan, u32 reg, u32 data)
    {
    writel(data, chan.iomem + reg);
    }
// -----------------------------------------------------------------------------
// Initialization and configuration
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_is_busy(chan: *mut usb_dmac_chan) -> bool {
    static bool usb_dmac_chan_is_busy(struct usb_dmac_chan *chan)
    {
    let mut chcr: u32 = usb_dmac_chan_read(chan, USB_DMACHCR);
    return (chcr & (USB_DMACHCR_DE | USB_DMACHCR_TE)) == USB_DMACHCR_DE;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_calc_tend(size: u32) -> u32 {
    static u32 usb_dmac_calc_tend(u32 size)
    {
//
// Please refer to the Figure "Example of Final Transaction Valid
// Data Transfer Enable (EDTEN) Setting" in the data sheet.
//
    return 0xffffffff << (32 - (size % USB_DMAC_XFER_SIZE ?	:
    USB_DMAC_XFER_SIZE));
    }
// This function is already held by vc.lock
    static void usb_dmac_chan_start_sg(struct usb_dmac_chan *chan,
    unsigned int index)
    {
    struct usb_dmac_desc *desc = chan.desc;
    struct usb_dmac_sg *sg = desc.sg + index;
    let mut src_addr: dma_addr_t = 0, dst_addr = 0;
    WARN_ON_ONCE(usb_dmac_chan_is_busy(chan));
    if (desc.direction == DMA_DEV_TO_MEM)
    dst_addr = sg.mem_addr;
    else
    src_addr = sg.mem_addr;
    dev_dbg(chan.vc.chan.device.dev,
    "chan%u: queue sg %p: %u@%pad . %pad\n",
    chan.index, sg, sg.size, &src_addr, &dst_addr);
    usb_dmac_chan_write(chan, USB_DMASAR, src_addr & 0xffffffff);
    usb_dmac_chan_write(chan, USB_DMADAR, dst_addr & 0xffffffff);
    usb_dmac_chan_write(chan, USB_DMATCR,
    DIV_ROUND_UP(sg.size, USB_DMAC_XFER_SIZE));
    usb_dmac_chan_write(chan, USB_DMATEND, usb_dmac_calc_tend(sg.size));
    usb_dmac_chan_write(chan, USB_DMACHCR, USB_DMAC_CHCR_TS |
    USB_DMACHCR_NULLE | USB_DMACHCR_IE | USB_DMACHCR_DE);
    }
// This function is already held by vc.lock
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_start_desc(chan: *mut usb_dmac_chan) {
    static void usb_dmac_chan_start_desc(struct usb_dmac_chan *chan)
    {
    struct virt_dma_desc *vd;
    vd = vchan_next_desc(&chan.vc);
    if (!vd) {
    chan.desc = core::ptr::null_mut();
    return;
    }
//
// Remove this request from vc->desc_issued. Otherwise, this driver
// will get the previous value from vchan_next_desc() after a transfer
// was completed.
//
    list_del(&vd.node);
    chan.desc = to_usb_dmac_desc(vd);
    chan.desc.sg_index = 0;
    usb_dmac_chan_start_sg(chan, 0);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_init(dmac: *mut usb_dmac) -> c_int {
    static int usb_dmac_init(struct usb_dmac *dmac)
    {
    u16 dmaor;
// Clear all channels and enable the DMAC globally.
    usb_dmac_write(dmac, USB_DMAOR, USB_DMAOR_DME);
    dmaor = usb_dmac_read(dmac, USB_DMAOR);
    if ((dmaor & (USB_DMAOR_AE | USB_DMAOR_DME)) != USB_DMAOR_DME) {
    dev_warn(dmac.dev, "DMAOR initialization failed.\n");
    return -EIO;
    }
    return 0;
    }
// -----------------------------------------------------------------------------
// Descriptors allocation and free
//
    static int usb_dmac_desc_alloc(struct usb_dmac_chan *chan, unsigned int sg_len,
    gfp_t gfp)
    {
    struct usb_dmac_desc *desc;
    unsigned long flags;
    desc = kzalloc_flex(*desc, sg, sg_len, gfp);
    if (!desc)
    return -ENOMEM;
    desc.sg_allocated_len = sg_len;
    INIT_LIST_HEAD(&desc.node);
    spin_lock_irqsave(&chan.vc.lock, flags);
    list_add_tail(&desc.node, &chan.desc_freed);
    spin_unlock_irqrestore(&chan.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_desc_free(chan: *mut usb_dmac_chan) {
    static void usb_dmac_desc_free(struct usb_dmac_chan *chan)
    {
    struct usb_dmac_desc *desc, *_desc;
    LIST_HEAD(list);
    list_splice_init(&chan.desc_freed, &list);
    list_splice_init(&chan.desc_got, &list);
    list_for_each_entry_safe(desc, _desc, &list, node) {
    list_del(&desc.node);
    kfree(desc);
    }
    chan.descs_allocated = 0;
    }
    static struct usb_dmac_desc *usb_dmac_desc_get(struct usb_dmac_chan *chan,
    unsigned int sg_len, gfp_t gfp)
    {
    struct usb_dmac_desc *desc = core::ptr::null_mut();
    unsigned long flags;
// Get a freed descriptor
    spin_lock_irqsave(&chan.vc.lock, flags);
    list_for_each_entry(desc, &chan.desc_freed, node) {
    if (sg_len <= desc.sg_allocated_len) {
    list_move_tail(&desc.node, &chan.desc_got);
    spin_unlock_irqrestore(&chan.vc.lock, flags);
    return desc;
    }
    }
    spin_unlock_irqrestore(&chan.vc.lock, flags);
// Allocate a new descriptor
    if (!usb_dmac_desc_alloc(chan, sg_len, gfp)) {
// If allocated the desc, it was added to tail of the list
    spin_lock_irqsave(&chan.vc.lock, flags);
    desc = list_last_entry(&chan.desc_freed, struct usb_dmac_desc,
    node);
    list_move_tail(&desc.node, &chan.desc_got);
    spin_unlock_irqrestore(&chan.vc.lock, flags);
    return desc;
    }
    return core::ptr::null_mut();
    }
    static void usb_dmac_desc_put(struct usb_dmac_chan *chan,
    struct usb_dmac_desc *desc)
    {
    unsigned long flags;
    spin_lock_irqsave(&chan.vc.lock, flags);
    list_move_tail(&desc.node, &chan.desc_freed);
    spin_unlock_irqrestore(&chan.vc.lock, flags);
    }
// -----------------------------------------------------------------------------
// Stop and reset
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_soft_reset(uchan: *mut usb_dmac_chan) {
    static void usb_dmac_soft_reset(struct usb_dmac_chan *uchan)
    {
    struct dma_chan *chan = &uchan.vc.chan;
    struct usb_dmac *dmac = to_usb_dmac(chan.device);
    int i;
// Don't issue soft reset if any one of channels is busy
    for (i = 0; i < dmac.n_channels; ++i) {
    if (usb_dmac_chan_is_busy(uchan))
    return;
    }
    usb_dmac_write(dmac, USB_DMAOR, 0);
    usb_dmac_write(dmac, USB_DMASWR, USB_DMASWR_SWR);
    udelay(100);
    usb_dmac_write(dmac, USB_DMASWR, 0);
    usb_dmac_write(dmac, USB_DMAOR, 1);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_halt(chan: *mut usb_dmac_chan) {
    static void usb_dmac_chan_halt(struct usb_dmac_chan *chan)
    {
    let mut chcr: u32 = usb_dmac_chan_read(chan, USB_DMACHCR);
    chcr &= ~(USB_DMACHCR_IE | USB_DMACHCR_TE | USB_DMACHCR_DE);
    usb_dmac_chan_write(chan, USB_DMACHCR, chcr);
    usb_dmac_soft_reset(chan);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_stop(dmac: *mut usb_dmac) {
    static void usb_dmac_stop(struct usb_dmac *dmac)
    {
    usb_dmac_write(dmac, USB_DMAOR, 0);
    }
// -----------------------------------------------------------------------------
// DMA engine operations
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int usb_dmac_alloc_chan_resources(struct dma_chan *chan)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    int ret;
    while (uchan.descs_allocated < USB_DMAC_INITIAL_NR_DESC) {
    ret = usb_dmac_desc_alloc(uchan, USB_DMAC_INITIAL_NR_SG,
    GFP_KERNEL);
    if (ret < 0) {
    usb_dmac_desc_free(uchan);
    return ret;
    }
    uchan.descs_allocated++;
    }
    return pm_runtime_get_sync(chan.device.dev);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_free_chan_resources(chan: *mut dma_chan) {
    static void usb_dmac_free_chan_resources(struct dma_chan *chan)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    unsigned long flags;
// Protect against ISR
    spin_lock_irqsave(&uchan.vc.lock, flags);
    usb_dmac_chan_halt(uchan);
    spin_unlock_irqrestore(&uchan.vc.lock, flags);
    usb_dmac_desc_free(uchan);
    vchan_free_chan_resources(&uchan.vc);
    pm_runtime_put(chan.device.dev);
    }
    static struct dma_async_tx_descriptor *
    usb_dmac_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long dma_flags, void *context)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    struct usb_dmac_desc *desc;
    struct scatterlist *sg;
    int i;
    if (!sg_len) {
    dev_warn(chan.device.dev,
    "%s: bad parameter: len=%d\n", __func__, sg_len);
    return core::ptr::null_mut();
    }
    desc = usb_dmac_desc_get(uchan, sg_len, GFP_NOWAIT);
    if (!desc)
    return core::ptr::null_mut();
    desc.direction = dir;
    desc.sg_len = sg_len;
    for_each_sg(sgl, sg, sg_len, i) {
    desc.sg[i].mem_addr = sg_dma_address(sg);
    desc.sg[i].size = sg_dma_len(sg);
    }
    return vchan_tx_prep(&uchan.vc, &desc.vd, dma_flags);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_terminate_all(chan: *mut dma_chan) -> c_int {
    static int usb_dmac_chan_terminate_all(struct dma_chan *chan)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    struct usb_dmac_desc *desc, *_desc;
    unsigned long flags;
    LIST_HEAD(head);
    LIST_HEAD(list);
    spin_lock_irqsave(&uchan.vc.lock, flags);
    usb_dmac_chan_halt(uchan);
    vchan_get_all_descriptors(&uchan.vc, &head);
    if (uchan.desc)
    uchan.desc = core::ptr::null_mut();
    list_splice_init(&uchan.desc_got, &list);
    list_for_each_entry_safe(desc, _desc, &list, node)
    list_move_tail(&desc.node, &uchan.desc_freed);
    spin_unlock_irqrestore(&uchan.vc.lock, flags);
    vchan_dma_desc_free_list(&uchan.vc, &head);
    return 0;
    }
    static unsigned int usb_dmac_get_current_residue(struct usb_dmac_chan *chan,
    struct usb_dmac_desc *desc,
    unsigned int sg_index)
    {
    struct usb_dmac_sg *sg = desc.sg + sg_index;
    let mut mem_addr: u32 = sg.mem_addr & 0xffffffff;
    let mut residue: c_uint = sg.size;
//
// We cannot use USB_DMATCR to calculate residue because USB_DMATCR
// has unsuited value to calculate.
//
    if (desc.direction == DMA_DEV_TO_MEM)
    residue -= usb_dmac_chan_read(chan, USB_DMADAR) - mem_addr;
    else
    residue -= usb_dmac_chan_read(chan, USB_DMASAR) - mem_addr;
    return residue;
    }
    static u32 usb_dmac_chan_get_residue_if_complete(struct usb_dmac_chan *chan,
    dma_cookie_t cookie)
    {
    struct usb_dmac_desc *desc;
    let mut residue: u32 = 0;
    list_for_each_entry_reverse(desc, &chan.desc_freed, node) {
    if (desc.done_cookie == cookie) {
    residue = desc.residue;
    break;
    }
    }
    return residue;
    }
    static u32 usb_dmac_chan_get_residue(struct usb_dmac_chan *chan,
    dma_cookie_t cookie)
    {
    let mut residue: u32 = 0;
    struct virt_dma_desc *vd;
    struct usb_dmac_desc *desc = chan.desc;
    int i;
    if (!desc) {
    vd = vchan_find_desc(&chan.vc, cookie);
    if (!vd)
    return 0;
    desc = to_usb_dmac_desc(vd);
    }
// Compute the size of all usb_dmac_sg still to be transferred
    for (i = desc.sg_index + 1; i < desc.sg_len; i++)
    residue += desc.sg[i].size;
// Add the residue for the current sg
    residue += usb_dmac_get_current_residue(chan, desc, desc.sg_index);
    return residue;
    }
    static enum dma_status usb_dmac_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    enum dma_status status;
    let mut residue: c_uint = 0;
    unsigned long flags;
    status = dma_cookie_status(chan, cookie, txstate);
// a client driver will get residue after DMA_COMPLETE
    if (!txstate)
    return status;
    spin_lock_irqsave(&uchan.vc.lock, flags);
    if (status == DMA_COMPLETE)
    residue = usb_dmac_chan_get_residue_if_complete(uchan, cookie);
    else
    residue = usb_dmac_chan_get_residue(uchan, cookie);
    spin_unlock_irqrestore(&uchan.vc.lock, flags);
    dma_set_residue(txstate, residue);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_issue_pending(chan: *mut dma_chan) {
    static void usb_dmac_issue_pending(struct dma_chan *chan)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&uchan.vc.lock, flags);
    if (vchan_issue_pending(&uchan.vc) && !uchan.desc)
    usb_dmac_chan_start_desc(uchan);
    spin_unlock_irqrestore(&uchan.vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_virt_desc_free(vd: *mut virt_dma_desc) {
    static void usb_dmac_virt_desc_free(struct virt_dma_desc *vd)
    {
    struct usb_dmac_desc *desc = to_usb_dmac_desc(vd);
    struct usb_dmac_chan *chan = to_usb_dmac_chan(vd.tx.chan);
    usb_dmac_desc_put(chan, desc);
    }
// -----------------------------------------------------------------------------
// IRQ handling
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_isr_transfer_end(chan: *mut usb_dmac_chan) {
    static void usb_dmac_isr_transfer_end(struct usb_dmac_chan *chan)
    {
    struct usb_dmac_desc *desc = chan.desc;
    BUG_ON(!desc);
    if (++desc.sg_index < desc.sg_len) {
    usb_dmac_chan_start_sg(chan, desc.sg_index);
    } else {
    desc.residue = usb_dmac_get_current_residue(chan, desc,
    desc.sg_index - 1);
    desc.done_cookie = desc.vd.tx.cookie;
    desc.vd.tx_result.result = DMA_TRANS_NOERROR;
    desc.vd.tx_result.residue = desc.residue;
    vchan_cookie_complete(&desc.vd);
// Restart the next transfer if this driver has a next desc
    usb_dmac_chan_start_desc(chan);
    }
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_isr_channel(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t usb_dmac_isr_channel(int irq, void *dev)
    {
    struct usb_dmac_chan *chan = dev;
    let mut ret: irqreturn_t = IRQ_NONE;
    let mut mask: u32 = 0;
    u32 chcr;
    let mut xfer_end: bool = false;
    spin_lock(&chan.vc.lock);
    chcr = usb_dmac_chan_read(chan, USB_DMACHCR);
    if (chcr & (USB_DMACHCR_TE | USB_DMACHCR_SP)) {
    mask |= USB_DMACHCR_DE | USB_DMACHCR_TE | USB_DMACHCR_SP;
    if (chcr & USB_DMACHCR_DE)
    xfer_end = true;
    ret |= IRQ_HANDLED;
    }
    if (chcr & USB_DMACHCR_NULL) {
// An interruption of TE will happen after we set FTE
    mask |= USB_DMACHCR_NULL;
    chcr |= USB_DMACHCR_FTE;
    ret |= IRQ_HANDLED;
    }
    if (mask)
    usb_dmac_chan_write(chan, USB_DMACHCR, chcr & ~mask);
    if (xfer_end)
    usb_dmac_isr_transfer_end(chan);
    spin_unlock(&chan.vc.lock);
    return ret;
    }
// -----------------------------------------------------------------------------
// OF xlate and channel filter
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_chan_filter(chan: *mut dma_chan, arg: *mut c_void) -> bool {
    static bool usb_dmac_chan_filter(struct dma_chan *chan, void *arg)
    {
    struct usb_dmac_chan *uchan = to_usb_dmac_chan(chan);
    struct of_phandle_args *dma_spec = arg;
// USB-DMAC should be used with fixed usb controller's FIFO
    if (uchan.index != dma_spec.args[0])
    return false;
    return true;
    }
    static struct dma_chan *usb_dmac_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct dma_chan *chan;
    dma_cap_mask_t mask;
    if (dma_spec.args_count != 1)
    return core::ptr::null_mut();
// Only slave DMA channels can be allocated via DT
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    chan = __dma_request_channel(&mask, usb_dmac_chan_filter, dma_spec,
    ofdma.of_node);
    if (!chan)
    return core::ptr::null_mut();
    return chan;
    }
// -----------------------------------------------------------------------------
// Power management
//
#[no_mangle]
unsafe extern "C" fn usb_dmac_runtime_suspend(dev: *mut device) -> c_int {
    static int usb_dmac_runtime_suspend(struct device *dev)
    {
    struct usb_dmac *dmac = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < dmac.n_channels; ++i) {
    if (!dmac.channels[i].iomem)
    break;
    usb_dmac_chan_halt(&dmac.channels[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_runtime_resume(dev: *mut device) -> c_int {
    static int usb_dmac_runtime_resume(struct device *dev)
    {
    struct usb_dmac *dmac = dev_get_drvdata(dev);
    return usb_dmac_init(dmac);
    }
    static const struct dev_pm_ops usb_dmac_pm = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    RUNTIME_PM_OPS(usb_dmac_runtime_suspend, usb_dmac_runtime_resume, core::ptr::null_mut())
    };
// -----------------------------------------------------------------------------
// Probe and remove
//
    static int usb_dmac_chan_probe(struct usb_dmac *dmac,
    struct usb_dmac_chan *uchan,
    u8 index)
    {
    struct platform_device *pdev = to_platform_device(dmac.dev);
    char pdev_irqname[6];
    char *irqname;
    int ret;
    uchan.index = index;
    uchan.iomem = dmac.iomem + USB_DMAC_CHAN_OFFSET(index);
// Request the channel interrupt.
    scnprintf(pdev_irqname, sizeof(pdev_irqname), "ch%u", index);
    uchan.irq = platform_get_irq_byname(pdev, pdev_irqname);
    if (uchan.irq < 0)
    return -ENODEV;
    irqname = devm_kasprintf(dmac.dev, GFP_KERNEL, "%s:%u",
    dev_name(dmac.dev), index);
    if (!irqname)
    return -ENOMEM;
    ret = devm_request_irq(dmac.dev, uchan.irq, usb_dmac_isr_channel,
    IRQF_SHARED, irqname, uchan);
    if (ret) {
    dev_err(dmac.dev, "failed to request IRQ %u (%d)\n",
    uchan.irq, ret);
    return ret;
    }
    uchan.vc.desc_free = usb_dmac_virt_desc_free;
    vchan_init(&uchan.vc, &dmac.engine);
    INIT_LIST_HEAD(&uchan.desc_freed);
    INIT_LIST_HEAD(&uchan.desc_got);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_parse_of(dev: *mut device, dmac: *mut usb_dmac) -> c_int {
    static int usb_dmac_parse_of(struct device *dev, struct usb_dmac *dmac)
    {
    struct device_node *np = dev.of_node;
    int ret;
    ret = of_property_read_u32(np, "dma-channels", &dmac.n_channels);
    if (ret < 0) {
    dev_err(dev, "unable to read dma-channels property\n");
    return ret;
    }
    if (dmac.n_channels <= 0 || dmac.n_channels >= 100) {
    dev_err(dev, "invalid number of channels %u\n",
    dmac.n_channels);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_probe(pdev: *mut platform_device) -> c_int {
    static int usb_dmac_probe(struct platform_device *pdev)
    {
    let mut widths: enum dma_slave_buswidth = USB_DMAC_SLAVE_BUSWIDTH;
    struct dma_device *engine;
    struct usb_dmac *dmac;
    int ret;
    u8 i;
    dmac = devm_kzalloc(&pdev.dev, sizeof(*dmac), GFP_KERNEL);
    if (!dmac)
    return -ENOMEM;
    dmac.dev = &pdev.dev;
    platform_set_drvdata(pdev, dmac);
    ret = usb_dmac_parse_of(&pdev.dev, dmac);
    if (ret < 0)
    return ret;
    dmac.channels = devm_kcalloc(&pdev.dev, dmac.n_channels,
    sizeof(*dmac.channels), GFP_KERNEL);
    if (!dmac.channels)
    return -ENOMEM;
// Request resources.
    dmac.iomem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dmac.iomem))
    return PTR_ERR(dmac.iomem);
// Enable runtime PM and initialize the device.
    pm_runtime_enable(&pdev.dev);
    ret = pm_runtime_get_sync(&pdev.dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "runtime PM get sync failed (%d)\n", ret);
    goto error_pm;
    }
    ret = usb_dmac_init(dmac);
    if (ret) {
    dev_err(&pdev.dev, "failed to reset device\n");
    goto error;
    }
// Initialize the channels.
    INIT_LIST_HEAD(&dmac.engine.channels);
    for (i = 0; i < dmac.n_channels; ++i) {
    ret = usb_dmac_chan_probe(dmac, &dmac.channels[i], i);
    if (ret < 0)
    goto error;
    }
// Register the DMAC as a DMA provider for DT.
    ret = of_dma_controller_register(pdev.dev.of_node, usb_dmac_of_xlate,
    core::ptr::null_mut());
    if (ret < 0)
    goto error;
//
// Register the DMA engine device.
//
// Default transfer size of 32 bytes requires 32-byte alignment.
//
    engine = &dmac.engine;
    dma_cap_set(DMA_SLAVE, engine.cap_mask);
    engine.dev = &pdev.dev;
    engine.src_addr_widths = widths;
    engine.dst_addr_widths = widths;
    engine.directions = BIT(DMA_MEM_TO_DEV) | BIT(DMA_DEV_TO_MEM);
    engine.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    engine.device_alloc_chan_resources = usb_dmac_alloc_chan_resources;
    engine.device_free_chan_resources = usb_dmac_free_chan_resources;
    engine.device_prep_slave_sg = usb_dmac_prep_slave_sg;
    engine.device_terminate_all = usb_dmac_chan_terminate_all;
    engine.device_tx_status = usb_dmac_tx_status;
    engine.device_issue_pending = usb_dmac_issue_pending;
    ret = dma_async_device_register(engine);
    if (ret < 0)
    goto error;
    pm_runtime_put(&pdev.dev);
    return 0;
    error:
    of_dma_controller_free(pdev.dev.of_node);
    error_pm:
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    static void usb_dmac_chan_remove(struct usb_dmac *dmac,
    struct usb_dmac_chan *uchan)
    {
    usb_dmac_chan_halt(uchan);
    devm_free_irq(dmac.dev, uchan.irq, uchan);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_remove(pdev: *mut platform_device) {
    static void usb_dmac_remove(struct platform_device *pdev)
    {
    struct usb_dmac *dmac = platform_get_drvdata(pdev);
    u8 i;
    for (i = 0; i < dmac.n_channels; ++i)
    usb_dmac_chan_remove(dmac, &dmac.channels[i]);
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&dmac.engine);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn usb_dmac_shutdown(pdev: *mut platform_device) {
    static void usb_dmac_shutdown(struct platform_device *pdev)
    {
    struct usb_dmac *dmac = platform_get_drvdata(pdev);
    usb_dmac_stop(dmac);
    }
    static const struct of_device_id usb_dmac_of_ids[] = {
    { .compatible = "renesas,usb-dmac", },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, usb_dmac_of_ids);
    static struct platform_driver usb_dmac_driver = {
    .driver		= {
    .pm	= pm_ptr(&usb_dmac_pm),
    .name	= "usb-dmac",
    .of_match_table = usb_dmac_of_ids,
    },
    .probe		= usb_dmac_probe,
    .remove		= usb_dmac_remove,
    .shutdown	= usb_dmac_shutdown,
    };
    module_platform_driver(usb_dmac_driver);
    MODULE_DESCRIPTION("Renesas USB DMA Controller Driver");
    MODULE_AUTHOR("Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>");
    MODULE_LICENSE("GPL v2");
