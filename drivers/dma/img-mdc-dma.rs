//! Automatically rewritten from C to Rust
//! Source: drivers/dma/img-mdc-dma.c
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
// IMG Multi-threaded DMA Controller (MDC)
//
// Copyright (C) 2009,2012,2013 Imagination Technologies Ltd.
// Copyright (C) 2014 Google, Inc.
//

pub const MDC_MAX_DMA_CHANNELS: c_int = 32;
pub const MDC_GENERAL_CONFIG: c_uint = 0x000;

pub const MDC_GENERAL_CONFIG_WIDTH_W_SHIFT: c_int = 4;
pub const MDC_GENERAL_CONFIG_WIDTH_W_MASK: c_uint = 0x7;

pub const MDC_GENERAL_CONFIG_WIDTH_R_SHIFT: c_int = 0;
pub const MDC_GENERAL_CONFIG_WIDTH_R_MASK: c_uint = 0x7;
pub const MDC_READ_PORT_CONFIG: c_uint = 0x004;
pub const MDC_READ_PORT_CONFIG_STHREAD_SHIFT: c_int = 28;
pub const MDC_READ_PORT_CONFIG_STHREAD_MASK: c_uint = 0xf;
pub const MDC_READ_PORT_CONFIG_RTHREAD_SHIFT: c_int = 24;
pub const MDC_READ_PORT_CONFIG_RTHREAD_MASK: c_uint = 0xf;
pub const MDC_READ_PORT_CONFIG_WTHREAD_SHIFT: c_int = 16;
pub const MDC_READ_PORT_CONFIG_WTHREAD_MASK: c_uint = 0xf;
pub const MDC_READ_PORT_CONFIG_BURST_SIZE_SHIFT: c_int = 4;
pub const MDC_READ_PORT_CONFIG_BURST_SIZE_MASK: c_uint = 0xff;

pub const MDC_READ_ADDRESS: c_uint = 0x008;
pub const MDC_WRITE_ADDRESS: c_uint = 0x00c;
pub const MDC_TRANSFER_SIZE: c_uint = 0x010;
pub const MDC_TRANSFER_SIZE_MASK: c_uint = 0xffffff;
pub const MDC_LIST_NODE_ADDRESS: c_uint = 0x014;
pub const MDC_CMDS_PROCESSED: c_uint = 0x018;
pub const MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT: c_int = 16;
pub const MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK: c_uint = 0x3f;

pub const MDC_CMDS_PROCESSED_CMDS_DONE_SHIFT: c_int = 0;
pub const MDC_CMDS_PROCESSED_CMDS_DONE_MASK: c_uint = 0x3f;
pub const MDC_CONTROL_AND_STATUS: c_uint = 0x01c;

pub const MDC_ACTIVE_TRANSFER_SIZE: c_uint = 0x030;
pub const MDC_GLOBAL_CONFIG_A: c_uint = 0x900;
pub const MDC_GLOBAL_CONFIG_A_THREAD_ID_WIDTH_SHIFT: c_int = 16;
pub const MDC_GLOBAL_CONFIG_A_THREAD_ID_WIDTH_MASK: c_uint = 0xff;
pub const MDC_GLOBAL_CONFIG_A_DMA_CONTEXTS_SHIFT: c_int = 8;
pub const MDC_GLOBAL_CONFIG_A_DMA_CONTEXTS_MASK: c_uint = 0xff;
pub const MDC_GLOBAL_CONFIG_A_SYS_DAT_WIDTH_SHIFT: c_int = 0;
pub const MDC_GLOBAL_CONFIG_A_SYS_DAT_WIDTH_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdc_hw_list_desc {
    pub gen_conf: u32,
    pub readport_conf: u32,
    pub read_addr: u32,
    pub write_addr: u32,
    pub xfer_size: u32,
    pub node_addr: u32,
    pub cmds_done: u32,
    pub ctrl_status: u32,
//
// Not part of the list descriptor, but instead used by the CPU to
// traverse the list.
//
    pub next_desc: *mut mdc_hw_list_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdc_tx_desc {
    pub chan: *mut mdc_chan,
    pub vd: virt_dma_desc,
    pub list_phys: dma_addr_t,
    pub list: *mut mdc_hw_list_desc,
    pub cyclic: bool,
    pub cmd_loaded: bool,
    pub list_len: c_uint,
    pub list_period_len: c_uint,
    pub list_xfer_size: usize,
    pub list_cmds_done: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdc_chan {
    pub mdma: *mut mdc_dma,
    pub vc: virt_dma_chan,
    pub config: dma_slave_config,
    pub desc: *mut mdc_tx_desc,
    pub irq: c_int,
    pub periph: c_uint,
    pub thread: c_uint,
    pub chan_nr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdc_dma_soc_data {
    pub mchan): *mut *mut void (enable_chan)(struct mdc_chan,
    pub mchan): *mut *mut void (disable_chan)(struct mdc_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdc_dma {
    pub dma_dev: dma_device,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub desc_pool: *mut dma_pool,
    pub periph_regs: *mut regmap,
    pub lock: spinlock_t,
    pub nr_threads: c_uint,
    pub nr_channels: c_uint,
    pub bus_width: c_uint,
    pub max_burst_mult: c_uint,
    pub max_xfer_size: c_uint,
    pub soc: *const mdc_dma_soc_data,
    pub channels: [mdc_chan; MDC_MAX_DMA_CHANNELS],
}

#[no_mangle]
pub unsafe extern "C" fn mdc_readl(mdma: *mut mdc_dma, reg: u32) -> u32 {
    static inline u32 mdc_readl(struct mdc_dma *mdma, u32 reg)
    {
    return readl(mdma.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mdc_writel(mdma: *mut mdc_dma, val: u32, reg: u32) {
    static inline void mdc_writel(struct mdc_dma *mdma, u32 val, u32 reg)
    {
    writel(val, mdma.regs + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mdc_chan_readl(mchan: *mut mdc_chan, reg: u32) -> u32 {
    static inline u32 mdc_chan_readl(struct mdc_chan *mchan, u32 reg)
    {
    return mdc_readl(mchan.mdma, mchan.chan_nr * 0x040 + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mdc_chan_writel(mchan: *mut mdc_chan, val: u32, reg: u32) {
    static inline void mdc_chan_writel(struct mdc_chan *mchan, u32 val, u32 reg)
    {
    mdc_writel(mchan.mdma, val, mchan.chan_nr * 0x040 + reg);
    }
    static inline struct mdc_chan *to_mdc_chan(struct dma_chan *c)
    {
    return container_of(to_virt_chan(c), struct mdc_chan, vc);
    }
    static inline struct mdc_tx_desc *to_mdc_desc(struct dma_async_tx_descriptor *t)
    {
    struct virt_dma_desc *vdesc = container_of(t, struct virt_dma_desc, tx);
    return container_of(vdesc, struct mdc_tx_desc, vd);
    }
    static inline struct device *mdma2dev(struct mdc_dma *mdma)
    {
    return mdma.dma_dev.dev;
    }
#[no_mangle]
pub unsafe extern "C" fn to_mdc_width(bytes: c_uint) -> c_uint {
    static inline unsigned int to_mdc_width(unsigned int bytes)
    {
    return ffs(bytes) - 1;
    }
    static inline void mdc_set_read_width(struct mdc_hw_list_desc *ldesc,
    unsigned int bytes)
    {
    ldesc.gen_conf |= to_mdc_width(bytes) <<
    MDC_GENERAL_CONFIG_WIDTH_R_SHIFT;
    }
    static inline void mdc_set_write_width(struct mdc_hw_list_desc *ldesc,
    unsigned int bytes)
    {
    ldesc.gen_conf |= to_mdc_width(bytes) <<
    MDC_GENERAL_CONFIG_WIDTH_W_SHIFT;
    }
    static void mdc_list_desc_config(struct mdc_chan *mchan,
    struct mdc_hw_list_desc *ldesc,
    enum dma_transfer_direction dir,
    dma_addr_t src, dma_addr_t dst, size_t len)
    {
    struct mdc_dma *mdma = mchan.mdma;
    unsigned int max_burst, burst_size;
    ldesc.gen_conf = MDC_GENERAL_CONFIG_IEN | MDC_GENERAL_CONFIG_LIST_IEN |
    MDC_GENERAL_CONFIG_LEVEL_INT | MDC_GENERAL_CONFIG_PHYSICAL_W |
    MDC_GENERAL_CONFIG_PHYSICAL_R;
    ldesc.readport_conf =
    (mchan.thread << MDC_READ_PORT_CONFIG_STHREAD_SHIFT) |
    (mchan.thread << MDC_READ_PORT_CONFIG_RTHREAD_SHIFT) |
    (mchan.thread << MDC_READ_PORT_CONFIG_WTHREAD_SHIFT);
    ldesc.read_addr = src;
    ldesc.write_addr = dst;
    ldesc.xfer_size = len - 1;
    ldesc.node_addr = 0;
    ldesc.cmds_done = 0;
    ldesc.ctrl_status = MDC_CONTROL_AND_STATUS_LIST_EN |
    MDC_CONTROL_AND_STATUS_EN;
    ldesc.next_desc = core::ptr::null_mut();
    if (IS_ALIGNED(dst, mdma.bus_width) &&
    IS_ALIGNED(src, mdma.bus_width))
    max_burst = mdma.bus_width * mdma.max_burst_mult;
    else
    max_burst = mdma.bus_width * (mdma.max_burst_mult - 1);
    if (dir == DMA_MEM_TO_DEV) {
    ldesc.gen_conf |= MDC_GENERAL_CONFIG_INC_R;
    ldesc.readport_conf |= MDC_READ_PORT_CONFIG_DREQ_ENABLE;
    mdc_set_read_width(ldesc, mdma.bus_width);
    mdc_set_write_width(ldesc, mchan.config.dst_addr_width);
    burst_size = min(max_burst, mchan.config.dst_maxburst *
    mchan.config.dst_addr_width);
    } else if (dir == DMA_DEV_TO_MEM) {
    ldesc.gen_conf |= MDC_GENERAL_CONFIG_INC_W;
    ldesc.readport_conf |= MDC_READ_PORT_CONFIG_DREQ_ENABLE;
    mdc_set_read_width(ldesc, mchan.config.src_addr_width);
    mdc_set_write_width(ldesc, mdma.bus_width);
    burst_size = min(max_burst, mchan.config.src_maxburst *
    mchan.config.src_addr_width);
    } else {
    ldesc.gen_conf |= MDC_GENERAL_CONFIG_INC_R |
    MDC_GENERAL_CONFIG_INC_W;
    mdc_set_read_width(ldesc, mdma.bus_width);
    mdc_set_write_width(ldesc, mdma.bus_width);
    burst_size = max_burst;
    }
    ldesc.readport_conf |= (burst_size - 1) <<
    MDC_READ_PORT_CONFIG_BURST_SIZE_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn mdc_list_desc_free(mdesc: *mut mdc_tx_desc) {
    static void mdc_list_desc_free(struct mdc_tx_desc *mdesc)
    {
    struct mdc_dma *mdma = mdesc.chan.mdma;
    struct mdc_hw_list_desc *curr, *next;
    dma_addr_t curr_phys, next_phys;
    curr = mdesc.list;
    curr_phys = mdesc.list_phys;
    while (curr) {
    next = curr.next_desc;
    next_phys = curr.node_addr;
    dma_pool_free(mdma.desc_pool, curr, curr_phys);
    curr = next;
    curr_phys = next_phys;
    }
    }
#[no_mangle]
unsafe extern "C" fn mdc_desc_free(vd: *mut virt_dma_desc) {
    static void mdc_desc_free(struct virt_dma_desc *vd)
    {
    struct mdc_tx_desc *mdesc = to_mdc_desc(&vd.tx);
    mdc_list_desc_free(mdesc);
    kfree(mdesc);
    }
    static struct dma_async_tx_descriptor *mdc_prep_dma_memcpy(
    struct dma_chan *chan, dma_addr_t dest, dma_addr_t src, size_t len,
    unsigned long flags)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct mdc_dma *mdma = mchan.mdma;
    struct mdc_tx_desc *mdesc;
    struct mdc_hw_list_desc *curr, *prev = core::ptr::null_mut();
    dma_addr_t curr_phys;
    if (!len)
    return core::ptr::null_mut();
    mdesc = kzalloc_obj(*mdesc, GFP_NOWAIT);
    if (!mdesc)
    return core::ptr::null_mut();
    mdesc.chan = mchan;
    mdesc.list_xfer_size = len;
    while (len > 0) {
    size_t xfer_size;
    curr = dma_pool_alloc(mdma.desc_pool, GFP_NOWAIT, &curr_phys);
    if (!curr)
    goto free_desc;
    if (prev) {
    prev.node_addr = curr_phys;
    prev.next_desc = curr;
    } else {
    mdesc.list_phys = curr_phys;
    mdesc.list = curr;
    }
    xfer_size = min_t(size_t, mdma.max_xfer_size, len);
    mdc_list_desc_config(mchan, curr, DMA_MEM_TO_MEM, src, dest,
    xfer_size);
    prev = curr;
    mdesc.list_len++;
    src += xfer_size;
    dest += xfer_size;
    len -= xfer_size;
    }
    return vchan_tx_prep(&mchan.vc, &mdesc.vd, flags);
    free_desc:
    mdc_desc_free(&mdesc.vd);
    return core::ptr::null_mut();
    }
    static int mdc_check_slave_width(struct mdc_chan *mchan,
    enum dma_transfer_direction dir)
    {
    enum dma_slave_buswidth width;
    if (dir == DMA_MEM_TO_DEV)
    width = mchan.config.dst_addr_width;
    else
    width = mchan.config.src_addr_width;
    switch (width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    case DMA_SLAVE_BUSWIDTH_8_BYTES:
    break;
    default:
    return -EINVAL;
    }
    if (width > mchan.mdma.bus_width)
    return -EINVAL;
    return 0;
    }
    static struct dma_async_tx_descriptor *mdc_prep_dma_cyclic(
    struct dma_chan *chan, dma_addr_t buf_addr, size_t buf_len,
    size_t period_len, enum dma_transfer_direction dir,
    unsigned long flags)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct mdc_dma *mdma = mchan.mdma;
    struct mdc_tx_desc *mdesc;
    struct mdc_hw_list_desc *curr, *prev = core::ptr::null_mut();
    dma_addr_t curr_phys;
    if (!buf_len && !period_len)
    return core::ptr::null_mut();
    if (!is_slave_direction(dir))
    return core::ptr::null_mut();
    if (mdc_check_slave_width(mchan, dir) < 0)
    return core::ptr::null_mut();
    mdesc = kzalloc_obj(*mdesc, GFP_NOWAIT);
    if (!mdesc)
    return core::ptr::null_mut();
    mdesc.chan = mchan;
    mdesc.cyclic = true;
    mdesc.list_xfer_size = buf_len;
    mdesc.list_period_len = DIV_ROUND_UP(period_len,
    mdma.max_xfer_size);
    while (buf_len > 0) {
    let mut remainder: usize = min(period_len, buf_len);
    while (remainder > 0) {
    size_t xfer_size;
    curr = dma_pool_alloc(mdma.desc_pool, GFP_NOWAIT,
    &curr_phys);
    if (!curr)
    goto free_desc;
    if (!prev) {
    mdesc.list_phys = curr_phys;
    mdesc.list = curr;
    } else {
    prev.node_addr = curr_phys;
    prev.next_desc = curr;
    }
    xfer_size = min_t(size_t, mdma.max_xfer_size,
    remainder);
    if (dir == DMA_MEM_TO_DEV) {
    mdc_list_desc_config(mchan, curr, dir,
    buf_addr,
    mchan.config.dst_addr,
    xfer_size);
    } else {
    mdc_list_desc_config(mchan, curr, dir,
    mchan.config.src_addr,
    buf_addr,
    xfer_size);
    }
    prev = curr;
    mdesc.list_len++;
    buf_addr += xfer_size;
    buf_len -= xfer_size;
    remainder -= xfer_size;
    }
    }
    prev.node_addr = mdesc.list_phys;
    return vchan_tx_prep(&mchan.vc, &mdesc.vd, flags);
    free_desc:
    mdc_desc_free(&mdesc.vd);
    return core::ptr::null_mut();
    }
    static struct dma_async_tx_descriptor *mdc_prep_slave_sg(
    struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct mdc_dma *mdma = mchan.mdma;
    struct mdc_tx_desc *mdesc;
    struct scatterlist *sg;
    struct mdc_hw_list_desc *curr, *prev = core::ptr::null_mut();
    dma_addr_t curr_phys;
    unsigned int i;
    if (!sgl)
    return core::ptr::null_mut();
    if (!is_slave_direction(dir))
    return core::ptr::null_mut();
    if (mdc_check_slave_width(mchan, dir) < 0)
    return core::ptr::null_mut();
    mdesc = kzalloc_obj(*mdesc, GFP_NOWAIT);
    if (!mdesc)
    return core::ptr::null_mut();
    mdesc.chan = mchan;
    for_each_sg(sgl, sg, sg_len, i) {
    let mut buf: dma_addr_t = sg_dma_address(sg);
    let mut buf_len: usize = sg_dma_len(sg);
    while (buf_len > 0) {
    size_t xfer_size;
    curr = dma_pool_alloc(mdma.desc_pool, GFP_NOWAIT,
    &curr_phys);
    if (!curr)
    goto free_desc;
    if (!prev) {
    mdesc.list_phys = curr_phys;
    mdesc.list = curr;
    } else {
    prev.node_addr = curr_phys;
    prev.next_desc = curr;
    }
    xfer_size = min_t(size_t, mdma.max_xfer_size,
    buf_len);
    if (dir == DMA_MEM_TO_DEV) {
    mdc_list_desc_config(mchan, curr, dir, buf,
    mchan.config.dst_addr,
    xfer_size);
    } else {
    mdc_list_desc_config(mchan, curr, dir,
    mchan.config.src_addr,
    buf, xfer_size);
    }
    prev = curr;
    mdesc.list_len++;
    mdesc.list_xfer_size += xfer_size;
    buf += xfer_size;
    buf_len -= xfer_size;
    }
    }
    return vchan_tx_prep(&mchan.vc, &mdesc.vd, flags);
    free_desc:
    mdc_desc_free(&mdesc.vd);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mdc_issue_desc(mchan: *mut mdc_chan) {
    static void mdc_issue_desc(struct mdc_chan *mchan)
    {
    struct mdc_dma *mdma = mchan.mdma;
    struct virt_dma_desc *vd;
    struct mdc_tx_desc *mdesc;
    u32 val;
    vd = vchan_next_desc(&mchan.vc);
    if (!vd)
    return;
    list_del(&vd.node);
    mdesc = to_mdc_desc(&vd.tx);
    mchan.desc = mdesc;
    dev_dbg(mdma2dev(mdma), "Issuing descriptor on channel %d\n",
    mchan.chan_nr);
    mdma.soc.enable_chan(mchan);
    val = mdc_chan_readl(mchan, MDC_GENERAL_CONFIG);
    val |= MDC_GENERAL_CONFIG_LIST_IEN | MDC_GENERAL_CONFIG_IEN |
    MDC_GENERAL_CONFIG_LEVEL_INT | MDC_GENERAL_CONFIG_PHYSICAL_W |
    MDC_GENERAL_CONFIG_PHYSICAL_R;
    mdc_chan_writel(mchan, val, MDC_GENERAL_CONFIG);
    val = (mchan.thread << MDC_READ_PORT_CONFIG_STHREAD_SHIFT) |
    (mchan.thread << MDC_READ_PORT_CONFIG_RTHREAD_SHIFT) |
    (mchan.thread << MDC_READ_PORT_CONFIG_WTHREAD_SHIFT);
    mdc_chan_writel(mchan, val, MDC_READ_PORT_CONFIG);
    mdc_chan_writel(mchan, mdesc.list_phys, MDC_LIST_NODE_ADDRESS);
    val = mdc_chan_readl(mchan, MDC_CONTROL_AND_STATUS);
    val |= MDC_CONTROL_AND_STATUS_LIST_EN;
    mdc_chan_writel(mchan, val, MDC_CONTROL_AND_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn mdc_issue_pending(chan: *mut dma_chan) {
    static void mdc_issue_pending(struct dma_chan *chan)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&mchan.vc.lock, flags);
    if (vchan_issue_pending(&mchan.vc) && !mchan.desc)
    mdc_issue_desc(mchan);
    spin_unlock_irqrestore(&mchan.vc.lock, flags);
    }
    static enum dma_status mdc_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *txstate)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct mdc_tx_desc *mdesc;
    struct virt_dma_desc *vd;
    unsigned long flags;
    let mut bytes: usize = 0;
    int ret;
    ret = dma_cookie_status(chan, cookie, txstate);
    if (ret == DMA_COMPLETE)
    return ret;
    if (!txstate)
    return ret;
    spin_lock_irqsave(&mchan.vc.lock, flags);
    vd = vchan_find_desc(&mchan.vc, cookie);
    if (vd) {
    mdesc = to_mdc_desc(&vd.tx);
    bytes = mdesc.list_xfer_size;
    } else if (mchan.desc && mchan.desc.vd.tx.cookie == cookie) {
    struct mdc_hw_list_desc *ldesc;
    u32 val1, val2, done, processed, residue;
    int i, cmds;
    mdesc = mchan.desc;
//
// Determine the number of commands that haven't been
// processed (handled by the IRQ handler) yet.
//
    do {
    val1 = mdc_chan_readl(mchan, MDC_CMDS_PROCESSED) &
    ~MDC_CMDS_PROCESSED_INT_ACTIVE;
    residue = mdc_chan_readl(mchan,
    MDC_ACTIVE_TRANSFER_SIZE);
    val2 = mdc_chan_readl(mchan, MDC_CMDS_PROCESSED) &
    ~MDC_CMDS_PROCESSED_INT_ACTIVE;
    } while (val1 != val2);
    done = (val1 >> MDC_CMDS_PROCESSED_CMDS_DONE_SHIFT) &
    MDC_CMDS_PROCESSED_CMDS_DONE_MASK;
    processed = (val1 >> MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT) &
    MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK;
    cmds = (done - processed) %
    (MDC_CMDS_PROCESSED_CMDS_DONE_MASK + 1);
//
// If the command loaded event hasn't been processed yet, then
// the difference above includes an extra command.
//
    if (!mdesc.cmd_loaded)
    cmds--;
    else
    cmds += mdesc.list_cmds_done;
    bytes = mdesc.list_xfer_size;
    ldesc = mdesc.list;
    for (i = 0; i < cmds; i++) {
    bytes -= ldesc.xfer_size + 1;
    ldesc = ldesc.next_desc;
    }
    if (ldesc) {
    if (residue != MDC_TRANSFER_SIZE_MASK)
    bytes -= ldesc.xfer_size - residue;
    else
    bytes -= ldesc.xfer_size + 1;
    }
    }
    spin_unlock_irqrestore(&mchan.vc.lock, flags);
    dma_set_residue(txstate, bytes);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mdc_get_new_events(mchan: *mut mdc_chan) -> c_uint {
    static unsigned int mdc_get_new_events(struct mdc_chan *mchan)
    {
    u32 val, processed, done1, done2;
    unsigned int ret;
    val = mdc_chan_readl(mchan, MDC_CMDS_PROCESSED);
    processed = (val >> MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT) &
    MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK;
//
// CMDS_DONE may have incremented between reading CMDS_PROCESSED
// and clearing INT_ACTIVE.  Re-read CMDS_PROCESSED to ensure we
// didn't miss a command completion.
//
    do {
    val = mdc_chan_readl(mchan, MDC_CMDS_PROCESSED);
    done1 = (val >> MDC_CMDS_PROCESSED_CMDS_DONE_SHIFT) &
    MDC_CMDS_PROCESSED_CMDS_DONE_MASK;
    val &= ~((MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK <<
    MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT) |
    MDC_CMDS_PROCESSED_INT_ACTIVE);
    val |= done1 << MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT;
    mdc_chan_writel(mchan, val, MDC_CMDS_PROCESSED);
    val = mdc_chan_readl(mchan, MDC_CMDS_PROCESSED);
    done2 = (val >> MDC_CMDS_PROCESSED_CMDS_DONE_SHIFT) &
    MDC_CMDS_PROCESSED_CMDS_DONE_MASK;
    } while (done1 != done2);
    if (done1 >= processed)
    ret = done1 - processed;
    else
    ret = ((MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK + 1) -
    processed) + done1;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mdc_terminate_all(chan: *mut dma_chan) -> c_int {
    static int mdc_terminate_all(struct dma_chan *chan)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    unsigned long flags;
    LIST_HEAD(head);
    spin_lock_irqsave(&mchan.vc.lock, flags);
    mdc_chan_writel(mchan, MDC_CONTROL_AND_STATUS_CANCEL,
    MDC_CONTROL_AND_STATUS);
    if (mchan.desc) {
    vchan_terminate_vdesc(&mchan.desc.vd);
    mchan.desc = core::ptr::null_mut();
    }
    vchan_get_all_descriptors(&mchan.vc, &head);
    mdc_get_new_events(mchan);
    spin_unlock_irqrestore(&mchan.vc.lock, flags);
    vchan_dma_desc_free_list(&mchan.vc, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdc_synchronize(chan: *mut dma_chan) {
    static void mdc_synchronize(struct dma_chan *chan)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    vchan_synchronize(&mchan.vc);
    }
    static int mdc_slave_config(struct dma_chan *chan,
    struct dma_slave_config *config)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&mchan.vc.lock, flags);
    mchan.config = *config;
    spin_unlock_irqrestore(&mchan.vc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdc_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int mdc_alloc_chan_resources(struct dma_chan *chan)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct device *dev = mdma2dev(mchan.mdma);
    return pm_runtime_get_sync(dev);
    }
#[no_mangle]
unsafe extern "C" fn mdc_free_chan_resources(chan: *mut dma_chan) {
    static void mdc_free_chan_resources(struct dma_chan *chan)
    {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    struct mdc_dma *mdma = mchan.mdma;
    struct device *dev = mdma2dev(mdma);
    mdc_terminate_all(chan);
    mdma.soc.disable_chan(mchan);
    pm_runtime_put(dev);
    }
#[no_mangle]
unsafe extern "C" fn mdc_chan_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mdc_chan_irq(int irq, void *dev_id)
    {
    struct mdc_chan *mchan = (struct mdc_chan *)dev_id;
    struct mdc_tx_desc *mdesc;
    unsigned int i, new_events;
    spin_lock(&mchan.vc.lock);
    dev_dbg(mdma2dev(mchan.mdma), "IRQ on channel %d\n", mchan.chan_nr);
    new_events = mdc_get_new_events(mchan);
    if (!new_events)
    goto out;
    mdesc = mchan.desc;
    if (!mdesc) {
    dev_warn(mdma2dev(mchan.mdma),
    "IRQ with no active descriptor on channel %d\n",
    mchan.chan_nr);
    goto out;
    }
    for (i = 0; i < new_events; i++) {
//
// The first interrupt in a transfer indicates that the
// command list has been loaded, not that a command has
// been completed.
//
    if (!mdesc.cmd_loaded) {
    mdesc.cmd_loaded = true;
    continue;
    }
    mdesc.list_cmds_done++;
    if (mdesc.cyclic) {
    mdesc.list_cmds_done %= mdesc.list_len;
    if (mdesc.list_cmds_done % mdesc.list_period_len == 0)
    vchan_cyclic_callback(&mdesc.vd);
    } else if (mdesc.list_cmds_done == mdesc.list_len) {
    mchan.desc = core::ptr::null_mut();
    vchan_cookie_complete(&mdesc.vd);
    mdc_issue_desc(mchan);
    break;
    }
    }
    out:
    spin_unlock(&mchan.vc.lock);
    return IRQ_HANDLED;
    }
    static struct dma_chan *mdc_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct mdc_dma *mdma = ofdma.of_dma_data;
    struct dma_chan *chan;
    if (dma_spec.args_count != 3)
    return core::ptr::null_mut();
    list_for_each_entry(chan, &mdma.dma_dev.channels, device_node) {
    struct mdc_chan *mchan = to_mdc_chan(chan);
    if (!(dma_spec.args[1] & BIT(mchan.chan_nr)))
    continue;
    if (dma_get_slave_channel(chan)) {
    mchan.periph = dma_spec.args[0];
    mchan.thread = dma_spec.args[2];
    return chan;
    }
    }
    return core::ptr::null_mut();
    }

pub const PISTACHIO_CR_PERIPH_DMA_ROUTE_MASK: c_uint = 0x3f;
#[no_mangle]
unsafe extern "C" fn pistachio_mdc_enable_chan(mchan: *mut mdc_chan) {
    static void pistachio_mdc_enable_chan(struct mdc_chan *mchan)
    {
    struct mdc_dma *mdma = mchan.mdma;
    regmap_update_bits(mdma.periph_regs,
    PISTACHIO_CR_PERIPH_DMA_ROUTE(mchan.chan_nr),
    PISTACHIO_CR_PERIPH_DMA_ROUTE_MASK <<
    PISTACHIO_CR_PERIPH_DMA_ROUTE_SHIFT(mchan.chan_nr),
    mchan.periph <<
    PISTACHIO_CR_PERIPH_DMA_ROUTE_SHIFT(mchan.chan_nr));
    }
#[no_mangle]
unsafe extern "C" fn pistachio_mdc_disable_chan(mchan: *mut mdc_chan) {
    static void pistachio_mdc_disable_chan(struct mdc_chan *mchan)
    {
    struct mdc_dma *mdma = mchan.mdma;
    regmap_update_bits(mdma.periph_regs,
    PISTACHIO_CR_PERIPH_DMA_ROUTE(mchan.chan_nr),
    PISTACHIO_CR_PERIPH_DMA_ROUTE_MASK <<
    PISTACHIO_CR_PERIPH_DMA_ROUTE_SHIFT(mchan.chan_nr),
    0);
    }
    static const struct mdc_dma_soc_data pistachio_mdc_data = {
    .enable_chan = pistachio_mdc_enable_chan,
    .disable_chan = pistachio_mdc_disable_chan,
    };
    static const struct of_device_id mdc_dma_of_match[] = {
    { .compatible = "img,pistachio-mdc-dma", .data = &pistachio_mdc_data, },
    { },
    };
    MODULE_DEVICE_TABLE(of, mdc_dma_of_match);
#[no_mangle]
unsafe extern "C" fn img_mdc_runtime_suspend(dev: *mut device) -> c_int {
    static int img_mdc_runtime_suspend(struct device *dev)
    {
    struct mdc_dma *mdma = dev_get_drvdata(dev);
    clk_disable_unprepare(mdma.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_mdc_runtime_resume(dev: *mut device) -> c_int {
    static int img_mdc_runtime_resume(struct device *dev)
    {
    struct mdc_dma *mdma = dev_get_drvdata(dev);
    return clk_prepare_enable(mdma.clk);
    }
#[no_mangle]
unsafe extern "C" fn mdc_dma_probe(pdev: *mut platform_device) -> c_int {
    static int mdc_dma_probe(struct platform_device *pdev)
    {
    struct mdc_dma *mdma;
    unsigned int i;
    u32 val;
    int ret;
    mdma = devm_kzalloc(&pdev.dev, sizeof(*mdma), GFP_KERNEL);
    if (!mdma)
    return -ENOMEM;
    platform_set_drvdata(pdev, mdma);
    mdma.soc = of_device_get_match_data(&pdev.dev);
    mdma.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mdma.regs))
    return PTR_ERR(mdma.regs);
    mdma.periph_regs = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "img,cr-periph");
    if (IS_ERR(mdma.periph_regs))
    return PTR_ERR(mdma.periph_regs);
    mdma.clk = devm_clk_get(&pdev.dev, "sys");
    if (IS_ERR(mdma.clk))
    return PTR_ERR(mdma.clk);
    dma_cap_zero(mdma.dma_dev.cap_mask);
    dma_cap_set(DMA_SLAVE, mdma.dma_dev.cap_mask);
    dma_cap_set(DMA_PRIVATE, mdma.dma_dev.cap_mask);
    dma_cap_set(DMA_CYCLIC, mdma.dma_dev.cap_mask);
    dma_cap_set(DMA_MEMCPY, mdma.dma_dev.cap_mask);
    val = mdc_readl(mdma, MDC_GLOBAL_CONFIG_A);
    mdma.nr_channels = (val >> MDC_GLOBAL_CONFIG_A_DMA_CONTEXTS_SHIFT) &
    MDC_GLOBAL_CONFIG_A_DMA_CONTEXTS_MASK;
    mdma.nr_threads =
    1 << ((val >> MDC_GLOBAL_CONFIG_A_THREAD_ID_WIDTH_SHIFT) &
    MDC_GLOBAL_CONFIG_A_THREAD_ID_WIDTH_MASK);
    mdma.bus_width =
    (1 << ((val >> MDC_GLOBAL_CONFIG_A_SYS_DAT_WIDTH_SHIFT) &
    MDC_GLOBAL_CONFIG_A_SYS_DAT_WIDTH_MASK)) / 8;
//
// Although transfer sizes of up to MDC_TRANSFER_SIZE_MASK + 1 bytes
// are supported, this makes it possible for the value reported in
// MDC_ACTIVE_TRANSFER_SIZE to be ambiguous - an active transfer size
// of MDC_TRANSFER_SIZE_MASK may indicate either that 0 bytes or
// MDC_TRANSFER_SIZE_MASK + 1 bytes are remaining.  To eliminate this
// ambiguity, restrict transfer sizes to one bus-width less than the
// actual maximum.
//
    mdma.max_xfer_size = MDC_TRANSFER_SIZE_MASK + 1 - mdma.bus_width;
    of_property_read_u32(pdev.dev.of_node, "dma-channels",
    &mdma.nr_channels);
    ret = of_property_read_u32(pdev.dev.of_node,
    "img,max-burst-multiplier",
    &mdma.max_burst_mult);
    if (ret)
    return ret;
    mdma.dma_dev.dev = &pdev.dev;
    mdma.dma_dev.device_prep_slave_sg = mdc_prep_slave_sg;
    mdma.dma_dev.device_prep_dma_cyclic = mdc_prep_dma_cyclic;
    mdma.dma_dev.device_prep_dma_memcpy = mdc_prep_dma_memcpy;
    mdma.dma_dev.device_alloc_chan_resources = mdc_alloc_chan_resources;
    mdma.dma_dev.device_free_chan_resources = mdc_free_chan_resources;
    mdma.dma_dev.device_tx_status = mdc_tx_status;
    mdma.dma_dev.device_issue_pending = mdc_issue_pending;
    mdma.dma_dev.device_terminate_all = mdc_terminate_all;
    mdma.dma_dev.device_synchronize = mdc_synchronize;
    mdma.dma_dev.device_config = mdc_slave_config;
    mdma.dma_dev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    mdma.dma_dev.residue_granularity = DMA_RESIDUE_GRANULARITY_BURST;
    for (i = 1; i <= mdma.bus_width; i <<= 1) {
    mdma.dma_dev.src_addr_widths |= BIT(i);
    mdma.dma_dev.dst_addr_widths |= BIT(i);
    }
    INIT_LIST_HEAD(&mdma.dma_dev.channels);
    for (i = 0; i < mdma.nr_channels; i++) {
    struct mdc_chan *mchan = &mdma.channels[i];
    mchan.mdma = mdma;
    mchan.chan_nr = i;
    mchan.irq = platform_get_irq(pdev, i);
    if (mchan.irq < 0)
    return mchan.irq;
    ret = devm_request_irq(&pdev.dev, mchan.irq, mdc_chan_irq,
    IRQ_TYPE_LEVEL_HIGH,
    dev_name(&pdev.dev), mchan);
    if (ret < 0)
    return ret;
    mchan.vc.desc_free = mdc_desc_free;
    vchan_init(&mchan.vc, &mdma.dma_dev);
    }
    mdma.desc_pool = dmam_pool_create(dev_name(&pdev.dev), &pdev.dev,
    sizeof(struct mdc_hw_list_desc),
    4, 0);
    if (!mdma.desc_pool)
    return -ENOMEM;
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = img_mdc_runtime_resume(&pdev.dev);
    if (ret)
    return ret;
    }
    ret = dma_async_device_register(&mdma.dma_dev);
    if (ret)
    goto suspend;
    ret = of_dma_controller_register(pdev.dev.of_node, mdc_of_xlate, mdma);
    if (ret)
    goto unregister;
    dev_info(&pdev.dev, "MDC with %u channels and %u threads\n",
    mdma.nr_channels, mdma.nr_threads);
    return 0;
    unregister:
    dma_async_device_unregister(&mdma.dma_dev);
    suspend:
    if (!pm_runtime_enabled(&pdev.dev))
    img_mdc_runtime_suspend(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mdc_dma_remove(pdev: *mut platform_device) {
    static void mdc_dma_remove(struct platform_device *pdev)
    {
    struct mdc_dma *mdma = platform_get_drvdata(pdev);
    struct mdc_chan *mchan, *next;
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&mdma.dma_dev);
    list_for_each_entry_safe(mchan, next, &mdma.dma_dev.channels,
    vc.chan.device_node) {
    list_del(&mchan.vc.chan.device_node);
    devm_free_irq(&pdev.dev, mchan.irq, mchan);
    tasklet_kill(&mchan.vc.task);
    }
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_mdc_runtime_suspend(&pdev.dev);
    }

#[no_mangle]
unsafe extern "C" fn img_mdc_suspend_late(dev: *mut device) -> c_int {
    static int img_mdc_suspend_late(struct device *dev)
    {
    struct mdc_dma *mdma = dev_get_drvdata(dev);
    int i;
// Check that all channels are idle
    for (i = 0; i < mdma.nr_channels; i++) {
    struct mdc_chan *mchan = &mdma.channels[i];
    if (unlikely(mchan.desc))
    return -EBUSY;
    }
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn img_mdc_resume_early(dev: *mut device) -> c_int {
    static int img_mdc_resume_early(struct device *dev)
    {
    return pm_runtime_force_resume(dev);
    }

    static const struct dev_pm_ops img_mdc_pm_ops = {
    SET_RUNTIME_PM_OPS(img_mdc_runtime_suspend,
    img_mdc_runtime_resume, core::ptr::null_mut())
    SET_LATE_SYSTEM_SLEEP_PM_OPS(img_mdc_suspend_late,
    img_mdc_resume_early)
    };
    static struct platform_driver mdc_dma_driver = {
    .driver = {
    .name = "img-mdc-dma",
    .pm = &img_mdc_pm_ops,
    .of_match_table = mdc_dma_of_match,
    },
    .probe = mdc_dma_probe,
    .remove = mdc_dma_remove,
    };
    module_platform_driver(mdc_dma_driver);
    MODULE_DESCRIPTION("IMG Multi-threaded DMA Controller (MDC) driver");
    MODULE_AUTHOR("Andrew Bresticker <abrestic@chromium.org>");
    MODULE_LICENSE("GPL v2");
