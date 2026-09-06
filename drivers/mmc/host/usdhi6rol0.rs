//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/usdhi6rol0.c
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
// Copyright (C) 2013-2014 Renesas Electronics Europe Ltd.
// Author: Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

pub const USDHI6_SD_CMD: c_uint = 0x0000;
pub const USDHI6_SD_PORT_SEL: c_uint = 0x0004;
pub const USDHI6_SD_ARG: c_uint = 0x0008;
pub const USDHI6_SD_STOP: c_uint = 0x0010;
pub const USDHI6_SD_SECCNT: c_uint = 0x0014;
pub const USDHI6_SD_RSP10: c_uint = 0x0018;
pub const USDHI6_SD_RSP32: c_uint = 0x0020;
pub const USDHI6_SD_RSP54: c_uint = 0x0028;
pub const USDHI6_SD_RSP76: c_uint = 0x0030;
pub const USDHI6_SD_INFO1: c_uint = 0x0038;
pub const USDHI6_SD_INFO2: c_uint = 0x003c;
pub const USDHI6_SD_INFO1_MASK: c_uint = 0x0040;
pub const USDHI6_SD_INFO2_MASK: c_uint = 0x0044;
pub const USDHI6_SD_CLK_CTRL: c_uint = 0x0048;
pub const USDHI6_SD_SIZE: c_uint = 0x004c;
pub const USDHI6_SD_OPTION: c_uint = 0x0050;
pub const USDHI6_SD_ERR_STS1: c_uint = 0x0058;
pub const USDHI6_SD_ERR_STS2: c_uint = 0x005c;
pub const USDHI6_SD_BUF0: c_uint = 0x0060;
pub const USDHI6_SDIO_MODE: c_uint = 0x0068;
pub const USDHI6_SDIO_INFO1: c_uint = 0x006c;
pub const USDHI6_SDIO_INFO1_MASK: c_uint = 0x0070;
pub const USDHI6_CC_EXT_MODE: c_uint = 0x01b0;
pub const USDHI6_SOFT_RST: c_uint = 0x01c0;
pub const USDHI6_VERSION: c_uint = 0x01c4;
pub const USDHI6_HOST_MODE: c_uint = 0x01c8;
pub const USDHI6_SDIF_MODE: c_uint = 0x01cc;
pub const USDHI6_SD_CMD_APP: c_uint = 0x0040;
pub const USDHI6_SD_CMD_MODE_RSP_AUTO: c_uint = 0x0000;
pub const USDHI6_SD_CMD_MODE_RSP_NONE: c_uint = 0x0300;
pub const USDHI6_SD_CMD_MODE_RSP_R1: c_uint = 0x0400	/* Also R5, R6, R7 */;
pub const USDHI6_SD_CMD_MODE_RSP_R1B: c_uint = 0x0500	/* R1b */;
pub const USDHI6_SD_CMD_MODE_RSP_R2: c_uint = 0x0600;
pub const USDHI6_SD_CMD_MODE_RSP_R3: c_uint = 0x0700	/* Also R4 */;
pub const USDHI6_SD_CMD_DATA: c_uint = 0x0800;
pub const USDHI6_SD_CMD_READ: c_uint = 0x1000;
pub const USDHI6_SD_CMD_MULTI: c_uint = 0x2000;
pub const USDHI6_SD_CMD_CMD12_AUTO_OFF: c_uint = 0x4000;

    USDHI6_SD_INFO2_CRC_ERR | USDHI6_SD_INFO2_END_ERR |	\
    USDHI6_SD_INFO2_TOUT | USDHI6_SD_INFO2_IWA_ERR |	\
    USDHI6_SD_INFO2_IRA_ERR | USDHI6_SD_INFO2_RSP_TOUT |	\
    USDHI6_SD_INFO2_ILA)

    USDHI6_SD_INFO1_CARD)

    USDHI6_SD_INFO2_BWE | 0x0800 | USDHI6_SD_INFO2_ILA)

pub const USDHI6_SD_OPTION_TIMEOUT_SHIFT: c_int = 4;

pub const USDHI6_SD_PORT_SEL_PORTS_SHIFT: c_int = 8;
pub const USDHI6_SD_CLK_CTRL_DIV_MASK: c_uint = 0xff;

    USDHI6_SDIO_INFO1_EXPUB52 | USDHI6_SDIO_INFO1_EXWT)
pub const USDHI6_MIN_DMA: c_int = 64;
pub const USDHI6_REQ_TIMEOUT_MS: c_int = 4000;
    enum usdhi6_wait_for {
    USDHI6_WAIT_FOR_REQUEST,
    USDHI6_WAIT_FOR_CMD,
    USDHI6_WAIT_FOR_MREAD,
    USDHI6_WAIT_FOR_MWRITE,
    USDHI6_WAIT_FOR_READ,
    USDHI6_WAIT_FOR_WRITE,
    USDHI6_WAIT_FOR_DATA_END,
    USDHI6_WAIT_FOR_STOP,
    USDHI6_WAIT_FOR_DMA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usdhi6_page {
    pub page: *mut page,
    pub /: *mut *mut *mut void mapped; / mapped page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usdhi6_host {
    pub mmc: *mut mmc_host,
    pub mrq: *mut mmc_request,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
// SG memory handling
// Common for multiple and single block requests
    pub /: *mut *mut usdhi6_page pg; / current page from an SG,
    pub /: *mut *mut *mut void blk_page; / either a mapped page, or the bounce buffer,
    pub /: *mut *mut size_t offset; / offset within a page, including sg->offset,
// Blocks, crossing a page boundary
    pub head_len: usize,
    pub head_pg: usdhi6_page,
// A bounce buffer for unaligned blocks or blocks, crossing a page boundary
    pub bounce_sg: scatterlist,
    pub bounce_buf: [u8; 512],
// Multiple block requests only
    pub /: *mut *mut *mut scatterlist sg; / current SG segment,
    pub /: *mut *mut int page_idx; / page index within an SG segment,
    pub wait: enum usdhi6_wait_for,
    pub status_mask: u32,
    pub status2_mask: u32,
    pub sdio_mask: u32,
    pub io_error: u32,
    pub irq_status: u32,
    pub imclk: c_ulong,
    pub rate: c_ulong,
    pub app_cmd: bool,
// Timeout handling
    pub timeout_work: delayed_work,
    pub timeout: c_ulong,
// DMA support
    pub chan_rx: *mut dma_chan,
    pub chan_tx: *mut dma_chan,
    pub dma_active: bool,
// Pin control
    pub pinctrl: *mut pinctrl,
    pub pins_uhs: *mut pinctrl_state,
}

// I/O primitives
#[no_mangle]
unsafe extern "C" fn usdhi6_write(host: *mut usdhi6_host, reg: u32, data: u32) {
    static void usdhi6_write(struct usdhi6_host *host, u32 reg, u32 data)
    {
    iowrite32(data, host.base + reg);
    dev_vdbg(mmc_dev(host.mmc), "%s(0x%p + 0x%x) = 0x%x\n", __func__,
    host.base, reg, data);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_write16(host: *mut usdhi6_host, reg: u32, data: u16) {
    static void usdhi6_write16(struct usdhi6_host *host, u32 reg, u16 data)
    {
    iowrite16(data, host.base + reg);
    dev_vdbg(mmc_dev(host.mmc), "%s(0x%p + 0x%x) = 0x%x\n", __func__,
    host.base, reg, data);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_read(host: *mut usdhi6_host, reg: u32) -> u32 {
    static u32 usdhi6_read(struct usdhi6_host *host, u32 reg)
    {
    let mut data: u32 = ioread32(host.base + reg);
    dev_vdbg(mmc_dev(host.mmc), "%s(0x%p + 0x%x) = 0x%x\n", __func__,
    host.base, reg, data);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_read16(host: *mut usdhi6_host, reg: u32) -> u16 {
    static u16 usdhi6_read16(struct usdhi6_host *host, u32 reg)
    {
    let mut data: u16 = ioread16(host.base + reg);
    dev_vdbg(mmc_dev(host.mmc), "%s(0x%p + 0x%x) = 0x%x\n", __func__,
    host.base, reg, data);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_irq_enable(host: *mut usdhi6_host, info1: u32, info2: u32) {
    static void usdhi6_irq_enable(struct usdhi6_host *host, u32 info1, u32 info2)
    {
    host.status_mask = USDHI6_SD_INFO1_IRQ & ~info1;
    host.status2_mask = USDHI6_SD_INFO2_IRQ & ~info2;
    usdhi6_write(host, USDHI6_SD_INFO1_MASK, host.status_mask);
    usdhi6_write(host, USDHI6_SD_INFO2_MASK, host.status2_mask);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_wait_for_resp(host: *mut usdhi6_host) {
    static void usdhi6_wait_for_resp(struct usdhi6_host *host)
    {
    usdhi6_irq_enable(host, USDHI6_SD_INFO1_RSP_END |
    USDHI6_SD_INFO1_ACCESS_END | USDHI6_SD_INFO1_CARD_CD,
    USDHI6_SD_INFO2_ERR);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_wait_for_brwe(host: *mut usdhi6_host, read: bool) {
    static void usdhi6_wait_for_brwe(struct usdhi6_host *host, bool read)
    {
    usdhi6_irq_enable(host, USDHI6_SD_INFO1_ACCESS_END |
    USDHI6_SD_INFO1_CARD_CD, USDHI6_SD_INFO2_ERR |
    (read ? USDHI6_SD_INFO2_BRE : USDHI6_SD_INFO2_BWE));
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_only_cd(host: *mut usdhi6_host) {
    static void usdhi6_only_cd(struct usdhi6_host *host)
    {
// Mask all except card hotplug
    usdhi6_irq_enable(host, USDHI6_SD_INFO1_CARD_CD, 0);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_mask_all(host: *mut usdhi6_host) {
    static void usdhi6_mask_all(struct usdhi6_host *host)
    {
    usdhi6_irq_enable(host, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_error_code(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_error_code(struct usdhi6_host *host)
    {
    u32 err;
    usdhi6_write(host, USDHI6_SD_STOP, USDHI6_SD_STOP_STP);
    if (host.io_error &
    (USDHI6_SD_INFO2_RSP_TOUT | USDHI6_SD_INFO2_TOUT)) {
    let mut rsp54: u32 = usdhi6_read(host, USDHI6_SD_RSP54);
    let mut opc: c_int = host.mrq ? host.mrq.cmd.opcode : -1;
    err = usdhi6_read(host, USDHI6_SD_ERR_STS2);
// Response timeout is often normal, don't spam the log
    if (host.wait == USDHI6_WAIT_FOR_CMD)
    dev_dbg(mmc_dev(host.mmc),
    "T-out sts 0x%x, resp 0x%x, state %u, CMD%d\n",
    err, rsp54, host.wait, opc);
    else
    dev_warn(mmc_dev(host.mmc),
    "T-out sts 0x%x, resp 0x%x, state %u, CMD%d\n",
    err, rsp54, host.wait, opc);
    return -ETIMEDOUT;
    }
    err = usdhi6_read(host, USDHI6_SD_ERR_STS1);
    if (err != USDHI6_SD_ERR_STS1_CRC_NO_ERROR)
    dev_warn(mmc_dev(host.mmc), "Err sts 0x%x, state %u, CMD%d\n",
    err, host.wait, host.mrq ? host.mrq.cmd.opcode : -1);
    if (host.io_error & USDHI6_SD_INFO2_ILA)
    return -EILSEQ;
    return -EIO;
    }
// Scatter-Gather management
//
// In PIO mode we have to map each page separately, using kmap(). That way
// adjacent pages are mapped to non-adjacent virtual addresses. That's why we
// have to use a bounce buffer for blocks, crossing page boundaries. Such blocks
// have been observed with an SDIO WiFi card (b43 driver).
//
    static void usdhi6_blk_bounce(struct usdhi6_host *host,
    struct scatterlist *sg)
    {
    struct mmc_data *data = host.mrq.data;
    let mut blk_head: usize = host.head_len;
    dev_dbg(mmc_dev(host.mmc), "%s(): CMD%u of %u SG: %ux%u @ 0x%x\n",
    __func__, host.mrq.cmd.opcode, data.sg_len,
    data.blksz, data.blocks, sg.offset);
    host.head_pg.page	= host.pg.page;
    host.head_pg.mapped	= host.pg.mapped;
    host.pg.page		= host.pg.page + 1;
    host.pg.mapped		= kmap(host.pg.page);
    host.blk_page = host.bounce_buf;
    host.offset = 0;
    if (data.flags & MMC_DATA_READ)
    return;
    memcpy(host.bounce_buf, host.head_pg.mapped + PAGE_SIZE - blk_head,
    blk_head);
    memcpy(host.bounce_buf + blk_head, host.pg.mapped,
    data.blksz - blk_head);
    }
// Only called for multiple block IO
#[no_mangle]
unsafe extern "C" fn usdhi6_sg_prep(host: *mut usdhi6_host) {
    static void usdhi6_sg_prep(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    struct mmc_data *data = mrq.data;
    usdhi6_write(host, USDHI6_SD_SECCNT, data.blocks);
    host.sg = data.sg;
// TODO: if we always map, this is redundant
    host.offset = host.sg.offset;
    }
// Map the first page in an SG segment: common for multiple and single block IO
    static void *usdhi6_sg_map(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    struct scatterlist *sg = data.sg_len > 1 ? host.sg : data.sg;
    let mut head: usize = PAGE_SIZE - sg.offset;
    let mut blk_head: usize = head % data.blksz;
    WARN(host.pg.page, "%p not properly unmapped!\n", host.pg.page);
    if (WARN(sg_dma_len(sg) % data.blksz,
    "SG size %u isn't a multiple of block size %u\n",
    sg_dma_len(sg), data.blksz))
    return core::ptr::null_mut();
    host.pg.page = sg_page(sg);
    host.pg.mapped = kmap(host.pg.page);
    host.offset = sg.offset;
//
// Block size must be a power of 2 for multi-block transfers,
// therefore blk_head is equal for all pages in this SG
//
    host.head_len = blk_head;
    if (head < data.blksz)
//
// The first block in the SG crosses a page boundary.
// Max blksz = 512, so blocks can only span 2 pages
//
    usdhi6_blk_bounce(host, sg);
    else
    host.blk_page = host.pg.mapped;
    dev_dbg(mmc_dev(host.mmc), "Mapped %p (%lx) at %p + %u for CMD%u @ 0x%p\n",
    host.pg.page, page_to_pfn(host.pg.page), host.pg.mapped,
    sg.offset, host.mrq.cmd.opcode, host.mrq);
    return host.blk_page + host.offset;
    }
// Unmap the current page: common for multiple and single block IO
#[no_mangle]
unsafe extern "C" fn usdhi6_sg_unmap(host: *mut usdhi6_host, force: bool) {
    static void usdhi6_sg_unmap(struct usdhi6_host *host, bool force)
    {
    struct mmc_data *data = host.mrq.data;
    struct page *page = host.head_pg.page;
    if (page) {
// Previous block was cross-page boundary
    struct scatterlist *sg = data.sg_len > 1 ?
    host.sg : data.sg;
    let mut blk_head: usize = host.head_len;
    if (!data.error && data.flags & MMC_DATA_READ) {
    memcpy(host.head_pg.mapped + PAGE_SIZE - blk_head,
    host.bounce_buf, blk_head);
    memcpy(host.pg.mapped, host.bounce_buf + blk_head,
    data.blksz - blk_head);
    }
    flush_dcache_page(page);
    kunmap(page);
    host.head_pg.page = core::ptr::null_mut();
    if (!force && sg_dma_len(sg) + sg.offset >
    (host.page_idx << PAGE_SHIFT) + data.blksz - blk_head)
// More blocks in this SG, don't unmap the next page
    return;
    }
    page = host.pg.page;
    if (!page)
    return;
    flush_dcache_page(page);
    kunmap(page);
    host.pg.page = core::ptr::null_mut();
    }
// Called from MMC_WRITE_MULTIPLE_BLOCK or MMC_READ_MULTIPLE_BLOCK
#[no_mangle]
unsafe extern "C" fn usdhi6_sg_advance(host: *mut usdhi6_host) {
    static void usdhi6_sg_advance(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    size_t done, total;
// New offset: set at the end of the previous block
    if (host.head_pg.page) {
// Finished a cross-page block, jump to the new page
    host.page_idx++;
    host.offset = data.blksz - host.head_len;
    host.blk_page = host.pg.mapped;
    usdhi6_sg_unmap(host, false);
    } else {
    host.offset += data.blksz;
// The completed block didn't cross a page boundary
    if (host.offset == PAGE_SIZE) {
// If required, we'll map the page below
    host.offset = 0;
    host.page_idx++;
    }
    }
//
// Now host->blk_page + host->offset point at the end of our last block
// and host->page_idx is the index of the page, in which our new block
// is located, if any
//
    done = (host.page_idx << PAGE_SHIFT) + host.offset;
    total = host.sg.offset + sg_dma_len(host.sg);
    dev_dbg(mmc_dev(host.mmc), "%s(): %zu of %zu @ %zu\n", __func__,
    done, total, host.offset);
    if (done < total && host.offset) {
// More blocks in this page
    if (host.offset + data.blksz > PAGE_SIZE)
// We approached at a block, that spans 2 pages
    usdhi6_blk_bounce(host, host.sg);
    return;
    }
// Finished current page or an SG segment
    usdhi6_sg_unmap(host, false);
    if (done == total) {
//
// End of an SG segment or the complete SG: jump to the next
// segment, we'll map it later in usdhi6_blk_read() or
// usdhi6_blk_write()
//
    struct scatterlist *next = sg_next(host.sg);
    host.page_idx = 0;
    if (!next)
    host.wait = USDHI6_WAIT_FOR_DATA_END;
    host.sg = next;
    if (WARN(next && sg_dma_len(next) % data.blksz,
    "SG size %u isn't a multiple of block size %u\n",
    sg_dma_len(next), data.blksz))
    data.error = -EINVAL;
    return;
    }
// We cannot get here after crossing a page border
// Next page in the same SG
    host.pg.page = sg_page(host.sg) + host.page_idx;
    host.pg.mapped = kmap(host.pg.page);
    host.blk_page = host.pg.mapped;
    dev_dbg(mmc_dev(host.mmc), "Mapped %p (%lx) at %p for CMD%u @ 0x%p\n",
    host.pg.page, page_to_pfn(host.pg.page), host.pg.mapped,
    host.mrq.cmd.opcode, host.mrq);
    }
// DMA handling
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_release(host: *mut usdhi6_host) {
    static void usdhi6_dma_release(struct usdhi6_host *host)
    {
    host.dma_active = false;
    if (host.chan_tx) {
    struct dma_chan *chan = host.chan_tx;
    host.chan_tx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    if (host.chan_rx) {
    struct dma_chan *chan = host.chan_rx;
    host.chan_rx = core::ptr::null_mut();
    dma_release_channel(chan);
    }
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_stop_unmap(host: *mut usdhi6_host) {
    static void usdhi6_dma_stop_unmap(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    if (!host.dma_active)
    return;
    usdhi6_write(host, USDHI6_CC_EXT_MODE, 0);
    host.dma_active = false;
    if (data.flags & MMC_DATA_READ)
    dma_unmap_sg(host.chan_rx.device.dev, data.sg,
    data.sg_len, DMA_FROM_DEVICE);
    else
    dma_unmap_sg(host.chan_tx.device.dev, data.sg,
    data.sg_len, DMA_TO_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_complete(arg: *mut c_void) {
    static void usdhi6_dma_complete(void *arg)
    {
    struct usdhi6_host *host = arg;
    struct mmc_request *mrq = host.mrq;
    if (WARN(!mrq || !mrq.data, "%s: core::ptr::null_mut() data in DMA completion for %p!\n",
    dev_name(mmc_dev(host.mmc)), mrq))
    return;
    dev_dbg(mmc_dev(host.mmc), "%s(): CMD%u DMA completed\n", __func__,
    mrq.cmd.opcode);
    usdhi6_dma_stop_unmap(host);
    usdhi6_wait_for_brwe(host, mrq.data.flags & MMC_DATA_READ);
    }
    static int usdhi6_dma_setup(struct usdhi6_host *host, struct dma_chan *chan,
    enum dma_transfer_direction dir)
    {
    struct mmc_data *data = host.mrq.data;
    struct scatterlist *sg = data.sg;
    struct dma_async_tx_descriptor *desc = core::ptr::null_mut();
    let mut cookie: dma_cookie_t = -EINVAL;
    enum dma_data_direction data_dir;
    int ret;
    switch (dir) {
    case DMA_MEM_TO_DEV:
    data_dir = DMA_TO_DEVICE;
    break;
    case DMA_DEV_TO_MEM:
    data_dir = DMA_FROM_DEVICE;
    break;
    default:
    return -EINVAL;
    }
    ret = dma_map_sg(chan.device.dev, sg, data.sg_len, data_dir);
    if (ret > 0) {
    host.dma_active = true;
    desc = dmaengine_prep_slave_sg(chan, sg, ret, dir,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    }
    if (desc) {
    desc.callback = usdhi6_dma_complete;
    desc.callback_param = host;
    cookie = dmaengine_submit(desc);
    }
    dev_dbg(mmc_dev(host.mmc), "%s(): mapped %d . %d, cookie %d @ %p\n",
    __func__, data.sg_len, ret, cookie, desc);
    if (cookie < 0) {
// DMA failed, fall back to PIO
    if (ret >= 0)
    ret = cookie;
    usdhi6_dma_release(host);
    dev_warn(mmc_dev(host.mmc),
    "DMA failed: %d, falling back to PIO\n", ret);
    }
    return cookie;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_start(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_dma_start(struct usdhi6_host *host)
    {
    if (!host.chan_rx || !host.chan_tx)
    return -ENODEV;
    if (host.mrq.data.flags & MMC_DATA_READ)
    return usdhi6_dma_setup(host, host.chan_rx, DMA_DEV_TO_MEM);
    return usdhi6_dma_setup(host, host.chan_tx, DMA_MEM_TO_DEV);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_kill(host: *mut usdhi6_host) {
    static void usdhi6_dma_kill(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    dev_dbg(mmc_dev(host.mmc), "%s(): SG of %u: %ux%u\n",
    __func__, data.sg_len, data.blocks, data.blksz);
// Abort DMA
    if (data.flags & MMC_DATA_READ)
    dmaengine_terminate_sync(host.chan_rx);
    else
    dmaengine_terminate_sync(host.chan_tx);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_check_error(host: *mut usdhi6_host) {
    static void usdhi6_dma_check_error(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    dev_dbg(mmc_dev(host.mmc), "%s(): IO error %d, status 0x%x\n",
    __func__, host.io_error, usdhi6_read(host, USDHI6_SD_INFO1));
    if (host.io_error) {
    data.error = usdhi6_error_code(host);
    data.bytes_xfered = 0;
    usdhi6_dma_kill(host);
    usdhi6_dma_release(host);
    dev_warn(mmc_dev(host.mmc),
    "DMA failed: %d, falling back to PIO\n", data.error);
    return;
    }
//
// The datasheet tells us to check a response from the card, whereas
// responses only come after the command phase, not after the data
// phase. Let's check anyway.
//
    if (host.irq_status & USDHI6_SD_INFO1_RSP_END)
    dev_warn(mmc_dev(host.mmc), "Unexpected response received!\n");
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_kick(host: *mut usdhi6_host) {
    static void usdhi6_dma_kick(struct usdhi6_host *host)
    {
    if (host.mrq.data.flags & MMC_DATA_READ)
    dma_async_issue_pending(host.chan_rx);
    else
    dma_async_issue_pending(host.chan_tx);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_dma_request(host: *mut usdhi6_host, start: phys_addr_t) {
    static void usdhi6_dma_request(struct usdhi6_host *host, phys_addr_t start)
    {
    struct dma_slave_config cfg = {
    .src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    .dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    };
    int ret;
    host.chan_tx = dma_request_chan(mmc_dev(host.mmc), "tx");
    dev_dbg(mmc_dev(host.mmc), "%s: TX: got channel %p\n", __func__,
    host.chan_tx);
    if (IS_ERR(host.chan_tx)) {
    host.chan_tx = core::ptr::null_mut();
    return;
    }
    cfg.direction = DMA_MEM_TO_DEV;
    cfg.dst_addr = start + USDHI6_SD_BUF0;
    cfg.dst_maxburst = 128;	/* 128 words * 4 bytes = 512 bytes */
    cfg.src_addr = 0;
    ret = dmaengine_slave_config(host.chan_tx, &cfg);
    if (ret < 0)
    goto e_release_tx;
    host.chan_rx = dma_request_chan(mmc_dev(host.mmc), "rx");
    dev_dbg(mmc_dev(host.mmc), "%s: RX: got channel %p\n", __func__,
    host.chan_rx);
    if (IS_ERR(host.chan_rx)) {
    host.chan_rx = core::ptr::null_mut();
    goto e_release_tx;
    }
    cfg.direction = DMA_DEV_TO_MEM;
    cfg.src_addr = cfg.dst_addr;
    cfg.src_maxburst = 128;	/* 128 words * 4 bytes = 512 bytes */
    cfg.dst_addr = 0;
    ret = dmaengine_slave_config(host.chan_rx, &cfg);
    if (ret < 0)
    goto e_release_rx;
    return;
    e_release_rx:
    dma_release_channel(host.chan_rx);
    host.chan_rx = core::ptr::null_mut();
    e_release_tx:
    dma_release_channel(host.chan_tx);
    host.chan_tx = core::ptr::null_mut();
    }
// API helpers
#[no_mangle]
unsafe extern "C" fn usdhi6_clk_set(host: *mut usdhi6_host, ios: *mut mmc_ios) {
    static void usdhi6_clk_set(struct usdhi6_host *host, struct mmc_ios *ios)
    {
    let mut rate: c_ulong = ios.clock;
    u32 val;
    unsigned int i;
    for (i = 1000; i; i--) {
    if (usdhi6_read(host, USDHI6_SD_INFO2) & USDHI6_SD_INFO2_SCLKDIVEN)
    break;
    usleep_range(10, 100);
    }
    if (!i) {
    dev_err(mmc_dev(host.mmc), "SD bus busy, clock set aborted\n");
    return;
    }
    val = usdhi6_read(host, USDHI6_SD_CLK_CTRL) & ~USDHI6_SD_CLK_CTRL_DIV_MASK;
    if (rate) {
    unsigned long new_rate;
    if (host.imclk <= rate) {
    if (ios.timing != MMC_TIMING_UHS_DDR50) {
// Cannot have 1-to-1 clock in DDR mode
    new_rate = host.imclk;
    val |= 0xff;
    } else {
    new_rate = host.imclk / 2;
    }
    } else {
    unsigned long div =
    roundup_pow_of_two(DIV_ROUND_UP(host.imclk, rate));
    val |= div >> 2;
    new_rate = host.imclk / div;
    }
    if (host.rate == new_rate)
    return;
    host.rate = new_rate;
    dev_dbg(mmc_dev(host.mmc), "target %lu, div %u, set %lu\n",
    rate, (val & 0xff) << 2, new_rate);
    }
//
// if old or new rate is equal to input rate, have to switch the clock
// off before changing and on after
//
    if (host.imclk == rate || host.imclk == host.rate || !rate)
    usdhi6_write(host, USDHI6_SD_CLK_CTRL,
    val & ~USDHI6_SD_CLK_CTRL_SCLKEN);
    if (!rate) {
    host.rate = 0;
    return;
    }
    usdhi6_write(host, USDHI6_SD_CLK_CTRL, val);
    if (host.imclk == rate || host.imclk == host.rate ||
    !(val & USDHI6_SD_CLK_CTRL_SCLKEN))
    usdhi6_write(host, USDHI6_SD_CLK_CTRL,
    val | USDHI6_SD_CLK_CTRL_SCLKEN);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_set_power(host: *mut usdhi6_host, ios: *mut mmc_ios) {
    static void usdhi6_set_power(struct usdhi6_host *host, struct mmc_ios *ios)
    {
    struct mmc_host *mmc = host.mmc;
    if (!IS_ERR(mmc.supply.vmmc))
// Errors ignored...
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc,
    ios.power_mode ? ios.vdd : 0);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_reset(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_reset(struct usdhi6_host *host)
    {
    int i;
    usdhi6_write(host, USDHI6_SOFT_RST, USDHI6_SOFT_RST_RESERVED);
    cpu_relax();
    usdhi6_write(host, USDHI6_SOFT_RST, USDHI6_SOFT_RST_RESERVED | USDHI6_SOFT_RST_RESET);
    for (i = 1000; i; i--)
    if (usdhi6_read(host, USDHI6_SOFT_RST) & USDHI6_SOFT_RST_RESET)
    break;
    return i ? 0 : -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void usdhi6_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
    u32 option, mode;
    int ret;
    dev_dbg(mmc_dev(mmc), "%uHz, OCR: %u, power %u, bus-width %u, timing %u\n",
    ios.clock, ios.vdd, ios.power_mode, ios.bus_width, ios.timing);
    switch (ios.power_mode) {
    case MMC_POWER_OFF:
    usdhi6_set_power(host, ios);
    usdhi6_only_cd(host);
    break;
    case MMC_POWER_UP:
//
// We only also touch USDHI6_SD_OPTION from .request(), which
// cannot race with MMC_POWER_UP
//
    ret = usdhi6_reset(host);
    if (ret < 0) {
    dev_err(mmc_dev(mmc), "Cannot reset the interface!\n");
    } else {
    usdhi6_set_power(host, ios);
    usdhi6_only_cd(host);
    }
    break;
    case MMC_POWER_ON:
    option = usdhi6_read(host, USDHI6_SD_OPTION);
//
// The eMMC standard only allows 4 or 8 bits in the DDR mode,
// the same probably holds for SD cards. We check here anyway,
// since the datasheet explicitly requires 4 bits for DDR.
//
    if (ios.bus_width == MMC_BUS_WIDTH_1) {
    if (ios.timing == MMC_TIMING_UHS_DDR50)
    dev_err(mmc_dev(mmc),
    "4 bits are required for DDR\n");
    option |= USDHI6_SD_OPTION_WIDTH_1;
    mode = 0;
    } else {
    option &= ~USDHI6_SD_OPTION_WIDTH_1;
    mode = ios.timing == MMC_TIMING_UHS_DDR50;
    }
    usdhi6_write(host, USDHI6_SD_OPTION, option);
    usdhi6_write(host, USDHI6_SDIF_MODE, mode);
    break;
    }
    if (host.rate != ios.clock)
    usdhi6_clk_set(host, ios);
    }
// This is data timeout. Response timeout is fixed to 640 clock cycles
#[no_mangle]
unsafe extern "C" fn usdhi6_timeout_set(host: *mut usdhi6_host) {
    static void usdhi6_timeout_set(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    u32 val;
    unsigned long ticks;
    if (!mrq.data)
    ticks = host.rate / 1000 * mrq.cmd.busy_timeout;
    else
    ticks = host.rate / 1000000 * (mrq.data.timeout_ns / 1000) +
    mrq.data.timeout_clks;
    if (!ticks || ticks > 1 << 27)
// Max timeout
    val = 14;
#[no_mangle]
pub unsafe extern "C" fn if(13: ticks < 1 <<) -> else {
    else if (ticks < 1 << 13)
// Min timeout
    val = 0;
    else
    val = order_base_2(ticks) - 13;
    dev_dbg(mmc_dev(host.mmc), "Set %s timeout %lu ticks @ %lu Hz\n",
    mrq.data ? "data" : "cmd", ticks, host.rate);
// Timeout Counter mask: 0xf0
    usdhi6_write(host, USDHI6_SD_OPTION, (val << USDHI6_SD_OPTION_TIMEOUT_SHIFT) |
    (usdhi6_read(host, USDHI6_SD_OPTION) & ~USDHI6_SD_OPTION_TIMEOUT_MASK));
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_request_done(host: *mut usdhi6_host) {
    static void usdhi6_request_done(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    struct mmc_data *data = mrq.data;
    if (WARN(host.pg.page || host.head_pg.page,
    "Page %p or %p not unmapped: wait %u, CMD%d(%c) @ +0x%zx %ux%u in SG%u!\n",
    host.pg.page, host.head_pg.page, host.wait, mrq.cmd.opcode,
    data ? (data.flags & MMC_DATA_READ ? 'R' : 'W') : '-',
    data ? host.offset : 0, data ? data.blocks : 0,
    data ? data.blksz : 0, data ? data.sg_len : 0))
    usdhi6_sg_unmap(host, true);
    if (mrq.cmd.error ||
    (data && data.error) ||
    (mrq.stop && mrq.stop.error))
    dev_dbg(mmc_dev(host.mmc), "%s(CMD%d: %ux%u): err %d %d %d\n",
    __func__, mrq.cmd.opcode, data ? data.blocks : 0,
    data ? data.blksz : 0,
    mrq.cmd.error,
    data ? data.error : 1,
    mrq.stop ? mrq.stop.error : 1);
// Disable DMA
    usdhi6_write(host, USDHI6_CC_EXT_MODE, 0);
    host.wait = USDHI6_WAIT_FOR_REQUEST;
    host.mrq = core::ptr::null_mut();
    mmc_request_done(host.mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_cmd_flags(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_cmd_flags(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    struct mmc_command *cmd = mrq.cmd;
    let mut opc: u16 = cmd.opcode;
    if (host.app_cmd) {
    host.app_cmd = false;
    opc |= USDHI6_SD_CMD_APP;
    }
    if (mrq.data) {
    opc |= USDHI6_SD_CMD_DATA;
    if (mrq.data.flags & MMC_DATA_READ)
    opc |= USDHI6_SD_CMD_READ;
    if (cmd.opcode == MMC_READ_MULTIPLE_BLOCK ||
    cmd.opcode == MMC_WRITE_MULTIPLE_BLOCK ||
    (cmd.opcode == SD_IO_RW_EXTENDED &&
    mrq.data.blocks > 1)) {
    opc |= USDHI6_SD_CMD_MULTI;
    if (!mrq.stop)
    opc |= USDHI6_SD_CMD_CMD12_AUTO_OFF;
    }
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_NONE:
    opc |= USDHI6_SD_CMD_MODE_RSP_NONE;
    break;
    case MMC_RSP_R1:
    opc |= USDHI6_SD_CMD_MODE_RSP_R1;
    break;
    case MMC_RSP_R1B:
    opc |= USDHI6_SD_CMD_MODE_RSP_R1B;
    break;
    case MMC_RSP_R2:
    opc |= USDHI6_SD_CMD_MODE_RSP_R2;
    break;
    case MMC_RSP_R3:
    opc |= USDHI6_SD_CMD_MODE_RSP_R3;
    break;
    default:
    dev_warn(mmc_dev(host.mmc),
    "Unknown response type %d\n",
    mmc_resp_type(cmd));
    return -EINVAL;
    }
    }
    return opc;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_rq_start(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_rq_start(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    struct mmc_command *cmd = mrq.cmd;
    struct mmc_data *data = mrq.data;
    let mut opc: c_int = usdhi6_cmd_flags(host);
    int i;
    if (opc < 0)
    return opc;
    for (i = 1000; i; i--) {
    if (!(usdhi6_read(host, USDHI6_SD_INFO2) & USDHI6_SD_INFO2_CBSY))
    break;
    usleep_range(10, 100);
    }
    if (!i) {
    dev_dbg(mmc_dev(host.mmc), "Command active, request aborted\n");
    return -EAGAIN;
    }
    if (data) {
    bool use_dma;
    let mut ret: c_int = 0;
    host.page_idx = 0;
    if (cmd.opcode == SD_IO_RW_EXTENDED && data.blocks > 1) {
    switch (data.blksz) {
    case 512:
    break;
    case 32:
    case 64:
    case 128:
    case 256:
    if (mrq.stop)
    ret = -EINVAL;
    break;
    default:
    ret = -EINVAL;
    }
    } else if ((cmd.opcode == MMC_READ_MULTIPLE_BLOCK ||
    cmd.opcode == MMC_WRITE_MULTIPLE_BLOCK) &&
    data.blksz != 512) {
    ret = -EINVAL;
    }
    if (ret < 0) {
    dev_warn(mmc_dev(host.mmc), "%s(): %u blocks of %u bytes\n",
    __func__, data.blocks, data.blksz);
    return -EINVAL;
    }
    if (cmd.opcode == MMC_READ_MULTIPLE_BLOCK ||
    cmd.opcode == MMC_WRITE_MULTIPLE_BLOCK ||
    (cmd.opcode == SD_IO_RW_EXTENDED &&
    data.blocks > 1))
    usdhi6_sg_prep(host);
    usdhi6_write(host, USDHI6_SD_SIZE, data.blksz);
    if ((data.blksz >= USDHI6_MIN_DMA ||
    data.blocks > 1) &&
    (data.blksz % 4 ||
    data.sg.offset % 4))
    dev_dbg(mmc_dev(host.mmc),
    "Bad SG of %u: %ux%u @ %u\n", data.sg_len,
    data.blksz, data.blocks, data.sg.offset);
// Enable DMA for USDHI6_MIN_DMA bytes or more
    use_dma = data.blksz >= USDHI6_MIN_DMA &&
    !(data.blksz % 4) &&
    usdhi6_dma_start(host) >= DMA_MIN_COOKIE;
    if (use_dma)
    usdhi6_write(host, USDHI6_CC_EXT_MODE, USDHI6_CC_EXT_MODE_SDRW);
    dev_dbg(mmc_dev(host.mmc),
    "%s(): request opcode %u, %u blocks of %u bytes in %u segments, %s %s @+0x%x%s\n",
    __func__, cmd.opcode, data.blocks, data.blksz,
    data.sg_len, use_dma ? "DMA" : "PIO",
    data.flags & MMC_DATA_READ ? "read" : "write",
    data.sg.offset, mrq.stop ? " + stop" : "");
    } else {
    dev_dbg(mmc_dev(host.mmc), "%s(): request opcode %u\n",
    __func__, cmd.opcode);
    }
// We have to get a command completion interrupt with DMA too
    usdhi6_wait_for_resp(host);
    host.wait = USDHI6_WAIT_FOR_CMD;
    schedule_delayed_work(&host.timeout_work, host.timeout);
// SEC bit is required to enable block counting by the core
    usdhi6_write(host, USDHI6_SD_STOP,
    data && data.blocks > 1 ? USDHI6_SD_STOP_SEC : 0);
    usdhi6_write(host, USDHI6_SD_ARG, cmd.arg);
// Kick command execution
    usdhi6_write(host, USDHI6_SD_CMD, opc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void usdhi6_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
    int ret;
    cancel_delayed_work_sync(&host.timeout_work);
    host.mrq = mrq;
    host.sg = core::ptr::null_mut();
    usdhi6_timeout_set(host);
    ret = usdhi6_rq_start(host);
    if (ret < 0) {
    mrq.cmd.error = ret;
    usdhi6_request_done(host);
    }
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_get_cd(mmc: *mut mmc_host) -> c_int {
    static int usdhi6_get_cd(struct mmc_host *mmc)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
// Read is atomic, no need to lock
    let mut status: u32 = usdhi6_read(host, USDHI6_SD_INFO1) & USDHI6_SD_INFO1_CD;
//
// level	status.CD	CD_ACTIVE_HIGH	card present
// 1	0		0		0
// 1	0		1		1
// 0	1		0		1
// 0	1		1		0
//
    return !status ^ !(mmc.caps2 & MMC_CAP2_CD_ACTIVE_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_get_ro(mmc: *mut mmc_host) -> c_int {
    static int usdhi6_get_ro(struct mmc_host *mmc)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
// No locking as above
    let mut status: u32 = usdhi6_read(host, USDHI6_SD_INFO1) & USDHI6_SD_INFO1_WP;
//
// level	status.WP	RO_ACTIVE_HIGH	card read-only
// 1	0		0		0
// 1	0		1		1
// 0	1		0		1
// 0	1		1		0
//
    return !status ^ !(mmc.caps2 & MMC_CAP2_RO_ACTIVE_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void usdhi6_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
    dev_dbg(mmc_dev(mmc), "%s(): %sable\n", __func__, enable ? "en" : "dis");
    if (enable) {
    host.sdio_mask = USDHI6_SDIO_INFO1_IRQ & ~USDHI6_SDIO_INFO1_IOIRQ;
    usdhi6_write(host, USDHI6_SDIO_INFO1_MASK, host.sdio_mask);
    usdhi6_write(host, USDHI6_SDIO_MODE, 1);
    } else {
    usdhi6_write(host, USDHI6_SDIO_MODE, 0);
    usdhi6_write(host, USDHI6_SDIO_INFO1_MASK, USDHI6_SDIO_INFO1_IRQ);
    host.sdio_mask = USDHI6_SDIO_INFO1_IRQ;
    }
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_set_pinstates(host: *mut usdhi6_host, voltage: c_int) -> c_int {
    static int usdhi6_set_pinstates(struct usdhi6_host *host, int voltage)
    {
    if (IS_ERR(host.pins_uhs))
    return 0;
    switch (voltage) {
    case MMC_SIGNAL_VOLTAGE_180:
    case MMC_SIGNAL_VOLTAGE_120:
    return pinctrl_select_state(host.pinctrl,
    host.pins_uhs);
    default:
    return pinctrl_select_default_state(mmc_dev(host.mmc));
    }
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_sig_volt_switch(mmc: *mut mmc_host, ios: *mut mmc_ios) -> c_int {
    static int usdhi6_sig_volt_switch(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    int ret;
    ret = mmc_regulator_set_vqmmc(mmc, ios);
    if (ret < 0)
    return ret;
    ret = usdhi6_set_pinstates(mmc_priv(mmc), ios.signal_voltage);
    if (ret)
    dev_warn_once(mmc_dev(mmc),
    "Failed to set pinstate err=%d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_card_busy(mmc: *mut mmc_host) -> c_int {
    static int usdhi6_card_busy(struct mmc_host *mmc)
    {
    struct usdhi6_host *host = mmc_priv(mmc);
    let mut tmp: u32 = usdhi6_read(host, USDHI6_SD_INFO2);
// Card is busy if it is pulling dat[0] low
    return !(tmp & USDHI6_SD_INFO2_SDDAT0);
    }
    static const struct mmc_host_ops usdhi6_ops = {
    .request	= usdhi6_request,
    .set_ios	= usdhi6_set_ios,
    .get_cd		= usdhi6_get_cd,
    .get_ro		= usdhi6_get_ro,
    .enable_sdio_irq = usdhi6_enable_sdio_irq,
    .start_signal_voltage_switch = usdhi6_sig_volt_switch,
    .card_busy = usdhi6_card_busy,
    };
// State machine handlers
#[no_mangle]
unsafe extern "C" fn usdhi6_resp_cmd12(host: *mut usdhi6_host) {
    static void usdhi6_resp_cmd12(struct usdhi6_host *host)
    {
    struct mmc_command *cmd = host.mrq.stop;
    cmd.resp[0] = usdhi6_read(host, USDHI6_SD_RSP10);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_resp_read(host: *mut usdhi6_host) {
    static void usdhi6_resp_read(struct usdhi6_host *host)
    {
    struct mmc_command *cmd = host.mrq.cmd;
    u32 *rsp = cmd.resp, tmp = 0;
    int i;
//
// RSP10	39-8
// RSP32	71-40
// RSP54	103-72
// RSP76	127-104
// R2-type response:
// resp[0]	= r[127..96]
// resp[1]	= r[95..64]
// resp[2]	= r[63..32]
// resp[3]	= r[31..0]
// Other responses:
// resp[0]	= r[39..8]
//
    if (mmc_resp_type(cmd) == MMC_RSP_NONE)
    return;
    if (!(host.irq_status & USDHI6_SD_INFO1_RSP_END)) {
    dev_err(mmc_dev(host.mmc),
    "CMD%d: response expected but is missing!\n", cmd.opcode);
    return;
    }
    if (mmc_resp_type(cmd) & MMC_RSP_136)
    for (i = 0; i < 4; i++) {
    if (i)
    rsp[3 - i] = tmp >> 24;
    tmp = usdhi6_read(host, USDHI6_SD_RSP10 + i * 8);
    rsp[3 - i] |= tmp << 8;
    }
    else if (cmd.opcode == MMC_READ_MULTIPLE_BLOCK ||
    cmd.opcode == MMC_WRITE_MULTIPLE_BLOCK)
// Read RSP54 to avoid conflict with auto CMD12
    rsp[0] = usdhi6_read(host, USDHI6_SD_RSP54);
    else
    rsp[0] = usdhi6_read(host, USDHI6_SD_RSP10);
    dev_dbg(mmc_dev(host.mmc), "Response 0x%x\n", rsp[0]);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_blk_read(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_blk_read(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i, rest;
    if (host.io_error) {
    data.error = usdhi6_error_code(host);
    goto error;
    }
    if (host.pg.page) {
    p = host.blk_page + host.offset;
    } else {
    p = usdhi6_sg_map(host);
    if (!p) {
    data.error = -ENOMEM;
    goto error;
    }
    }
    for (i = 0; i < data.blksz / 4; i++, p++)
// p = usdhi6_read(host, USDHI6_SD_BUF0);
    rest = data.blksz % 4;
    for (i = 0; i < (rest + 1) / 2; i++) {
    let mut d: u16 = usdhi6_read16(host, USDHI6_SD_BUF0);
    ((u8 *)p)[2 * i] = ((u8 *)&d)[0];
    if (rest > 1 && !i)
    ((u8 *)p)[2 * i + 1] = ((u8 *)&d)[1];
    }
    return 0;
    error:
    dev_dbg(mmc_dev(host.mmc), "%s(): %d\n", __func__, data.error);
    host.wait = USDHI6_WAIT_FOR_REQUEST;
    return data.error;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_blk_write(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_blk_write(struct usdhi6_host *host)
    {
    struct mmc_data *data = host.mrq.data;
    u32 *p;
    int i, rest;
    if (host.io_error) {
    data.error = usdhi6_error_code(host);
    goto error;
    }
    if (host.pg.page) {
    p = host.blk_page + host.offset;
    } else {
    p = usdhi6_sg_map(host);
    if (!p) {
    data.error = -ENOMEM;
    goto error;
    }
    }
    for (i = 0; i < data.blksz / 4; i++, p++)
    usdhi6_write(host, USDHI6_SD_BUF0, *p);
    rest = data.blksz % 4;
    for (i = 0; i < (rest + 1) / 2; i++) {
    u16 d;
    ((u8 *)&d)[0] = ((u8 *)p)[2 * i];
    if (rest > 1 && !i)
    ((u8 *)&d)[1] = ((u8 *)p)[2 * i + 1];
    else
    ((u8 *)&d)[1] = 0;
    usdhi6_write16(host, USDHI6_SD_BUF0, d);
    }
    return 0;
    error:
    dev_dbg(mmc_dev(host.mmc), "%s(): %d\n", __func__, data.error);
    host.wait = USDHI6_WAIT_FOR_REQUEST;
    return data.error;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_stop_cmd(host: *mut usdhi6_host) -> c_int {
    static int usdhi6_stop_cmd(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    switch (mrq.cmd.opcode) {
    case MMC_READ_MULTIPLE_BLOCK:
    case MMC_WRITE_MULTIPLE_BLOCK:
    if (mrq.stop.opcode == MMC_STOP_TRANSMISSION) {
    host.wait = USDHI6_WAIT_FOR_STOP;
    return 0;
    }
    fallthrough;	/* Unsupported STOP command */
    default:
    dev_err(mmc_dev(host.mmc),
    "unsupported stop CMD%d for CMD%d\n",
    mrq.stop.opcode, mrq.cmd.opcode);
    mrq.stop.error = -EOPNOTSUPP;
    }
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_end_cmd(host: *mut usdhi6_host) -> bool {
    static bool usdhi6_end_cmd(struct usdhi6_host *host)
    {
    struct mmc_request *mrq = host.mrq;
    struct mmc_command *cmd = mrq.cmd;
    if (host.io_error) {
    cmd.error = usdhi6_error_code(host);
    return false;
    }
    usdhi6_resp_read(host);
    if (!mrq.data)
    return false;
    if (host.dma_active) {
    usdhi6_dma_kick(host);
    if (!mrq.stop)
    host.wait = USDHI6_WAIT_FOR_DMA;
#[no_mangle]
pub unsafe extern "C" fn if(0: usdhi6_stop_cmd(host) <) -> else {
    else if (usdhi6_stop_cmd(host) < 0)
    return false;
    } else if (mrq.data.flags & MMC_DATA_READ) {
    if (cmd.opcode == MMC_READ_MULTIPLE_BLOCK ||
    (cmd.opcode == SD_IO_RW_EXTENDED &&
    mrq.data.blocks > 1))
    host.wait = USDHI6_WAIT_FOR_MREAD;
    else
    host.wait = USDHI6_WAIT_FOR_READ;
    } else {
    if (cmd.opcode == MMC_WRITE_MULTIPLE_BLOCK ||
    (cmd.opcode == SD_IO_RW_EXTENDED &&
    mrq.data.blocks > 1))
    host.wait = USDHI6_WAIT_FOR_MWRITE;
    else
    host.wait = USDHI6_WAIT_FOR_WRITE;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_read_block(host: *mut usdhi6_host) -> bool {
    static bool usdhi6_read_block(struct usdhi6_host *host)
    {
// ACCESS_END IRQ is already unmasked
    let mut ret: c_int = usdhi6_blk_read(host);
//
// Have to force unmapping both pages: the single block could have been
// cross-page, in which case for single-block IO host->page_idx == 0.
// So, if we don't force, the second page won't be unmapped.
//
    usdhi6_sg_unmap(host, true);
    if (ret < 0)
    return false;
    host.wait = USDHI6_WAIT_FOR_DATA_END;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_mread_block(host: *mut usdhi6_host) -> bool {
    static bool usdhi6_mread_block(struct usdhi6_host *host)
    {
    let mut ret: c_int = usdhi6_blk_read(host);
    if (ret < 0)
    return false;
    usdhi6_sg_advance(host);
    return !host.mrq.data.error &&
    (host.wait != USDHI6_WAIT_FOR_DATA_END || !host.mrq.stop);
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_write_block(host: *mut usdhi6_host) -> bool {
    static bool usdhi6_write_block(struct usdhi6_host *host)
    {
    let mut ret: c_int = usdhi6_blk_write(host);
// See comment in usdhi6_read_block()
    usdhi6_sg_unmap(host, true);
    if (ret < 0)
    return false;
    host.wait = USDHI6_WAIT_FOR_DATA_END;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_mwrite_block(host: *mut usdhi6_host) -> bool {
    static bool usdhi6_mwrite_block(struct usdhi6_host *host)
    {
    let mut ret: c_int = usdhi6_blk_write(host);
    if (ret < 0)
    return false;
    usdhi6_sg_advance(host);
    return !host.mrq.data.error &&
    (host.wait != USDHI6_WAIT_FOR_DATA_END || !host.mrq.stop);
    }
// Interrupt & timeout handlers
#[no_mangle]
unsafe extern "C" fn usdhi6_sd_bh(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t usdhi6_sd_bh(int irq, void *dev_id)
    {
    struct usdhi6_host *host = dev_id;
    struct mmc_request *mrq;
    struct mmc_command *cmd;
    struct mmc_data *data;
    let mut io_wait: bool = false;
    cancel_delayed_work_sync(&host.timeout_work);
    mrq = host.mrq;
    if (!mrq)
    return IRQ_HANDLED;
    cmd = mrq.cmd;
    data = mrq.data;
    switch (host.wait) {
    case USDHI6_WAIT_FOR_REQUEST:
// We're too late, the timeout has already kicked in
    return IRQ_HANDLED;
    case USDHI6_WAIT_FOR_CMD:
// Wait for data?
    io_wait = usdhi6_end_cmd(host);
    break;
    case USDHI6_WAIT_FOR_MREAD:
// Wait for more data?
    io_wait = usdhi6_mread_block(host);
    break;
    case USDHI6_WAIT_FOR_READ:
// Wait for data end?
    io_wait = usdhi6_read_block(host);
    break;
    case USDHI6_WAIT_FOR_MWRITE:
// Wait data to write?
    io_wait = usdhi6_mwrite_block(host);
    break;
    case USDHI6_WAIT_FOR_WRITE:
// Wait for data end?
    io_wait = usdhi6_write_block(host);
    break;
    case USDHI6_WAIT_FOR_DMA:
    usdhi6_dma_check_error(host);
    break;
    case USDHI6_WAIT_FOR_STOP:
    usdhi6_write(host, USDHI6_SD_STOP, 0);
    if (host.io_error) {
    let mut ret: c_int = usdhi6_error_code(host);
    if (mrq.stop)
    mrq.stop.error = ret;
    else
    mrq.data.error = ret;
    dev_warn(mmc_dev(host.mmc), "%s(): %d\n", __func__, ret);
    break;
    }
    usdhi6_resp_cmd12(host);
    mrq.stop.error = 0;
    break;
    case USDHI6_WAIT_FOR_DATA_END:
    if (host.io_error) {
    mrq.data.error = usdhi6_error_code(host);
    dev_warn(mmc_dev(host.mmc), "%s(): %d\n", __func__,
    mrq.data.error);
    }
    break;
    default:
    cmd.error = -EFAULT;
    dev_err(mmc_dev(host.mmc), "Invalid state %u\n", host.wait);
    usdhi6_request_done(host);
    return IRQ_HANDLED;
    }
    if (io_wait) {
    schedule_delayed_work(&host.timeout_work, host.timeout);
// Wait for more data or ACCESS_END
    if (!host.dma_active)
    usdhi6_wait_for_brwe(host, mrq.data.flags & MMC_DATA_READ);
    return IRQ_HANDLED;
    }
    if (!cmd.error) {
    if (data) {
    if (!data.error) {
    if (host.wait != USDHI6_WAIT_FOR_STOP &&
    host.mrq.stop &&
    !host.mrq.stop.error &&
    !usdhi6_stop_cmd(host)) {
// Sending STOP
    usdhi6_wait_for_resp(host);
    schedule_delayed_work(&host.timeout_work,
    host.timeout);
    return IRQ_HANDLED;
    }
    data.bytes_xfered = data.blocks * data.blksz;
    } else {
// Data error: might need to unmap the last page
    dev_warn(mmc_dev(host.mmc), "%s(): data error %d\n",
    __func__, data.error);
    usdhi6_sg_unmap(host, true);
    }
    } else if (cmd.opcode == MMC_APP_CMD) {
    host.app_cmd = true;
    }
    }
    usdhi6_request_done(host);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_sd(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t usdhi6_sd(int irq, void *dev_id)
    {
    struct usdhi6_host *host = dev_id;
    u16 status, status2, error;
    status = usdhi6_read(host, USDHI6_SD_INFO1) & ~host.status_mask &
    ~USDHI6_SD_INFO1_CARD;
    status2 = usdhi6_read(host, USDHI6_SD_INFO2) & ~host.status2_mask;
    usdhi6_only_cd(host);
    dev_dbg(mmc_dev(host.mmc),
    "IRQ status = 0x%08x, status2 = 0x%08x\n", status, status2);
    if (!status && !status2)
    return IRQ_NONE;
    error = status2 & USDHI6_SD_INFO2_ERR;
// Ack / clear interrupts
    if (USDHI6_SD_INFO1_IRQ & status)
    usdhi6_write(host, USDHI6_SD_INFO1,
    0xffff & ~(USDHI6_SD_INFO1_IRQ & status));
    if (USDHI6_SD_INFO2_IRQ & status2) {
    if (error)
// In error cases BWE and BRE aren't cleared automatically
    status2 |= USDHI6_SD_INFO2_BWE | USDHI6_SD_INFO2_BRE;
    usdhi6_write(host, USDHI6_SD_INFO2,
    0xffff & ~(USDHI6_SD_INFO2_IRQ & status2));
    }
    host.io_error = error;
    host.irq_status = status;
    if (error) {
// Don't pollute the log with unsupported command timeouts
    if (host.wait != USDHI6_WAIT_FOR_CMD ||
    error != USDHI6_SD_INFO2_RSP_TOUT)
    dev_warn(mmc_dev(host.mmc),
    "%s(): INFO2 error bits 0x%08x\n",
    __func__, error);
    else
    dev_dbg(mmc_dev(host.mmc),
    "%s(): INFO2 error bits 0x%08x\n",
    __func__, error);
    }
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_sdio(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t usdhi6_sdio(int irq, void *dev_id)
    {
    struct usdhi6_host *host = dev_id;
    let mut status: u32 = usdhi6_read(host, USDHI6_SDIO_INFO1) & ~host.sdio_mask;
    dev_dbg(mmc_dev(host.mmc), "%s(): status 0x%x\n", __func__, status);
    if (!status)
    return IRQ_NONE;
    usdhi6_write(host, USDHI6_SDIO_INFO1, ~status);
    mmc_signal_sdio_irq(host.mmc);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_cd(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t usdhi6_cd(int irq, void *dev_id)
    {
    struct usdhi6_host *host = dev_id;
    struct mmc_host *mmc = host.mmc;
    u16 status;
// We're only interested in hotplug events here
    status = usdhi6_read(host, USDHI6_SD_INFO1) & ~host.status_mask &
    USDHI6_SD_INFO1_CARD;
    if (!status)
    return IRQ_NONE;
// Ack
    usdhi6_write(host, USDHI6_SD_INFO1, ~status);
    if (!work_pending(&mmc.detect.work) &&
    (((status & USDHI6_SD_INFO1_CARD_INSERT) &&
    !mmc.card) ||
    ((status & USDHI6_SD_INFO1_CARD_EJECT) &&
    mmc.card)))
    mmc_detect_change(mmc, msecs_to_jiffies(100));
    return IRQ_HANDLED;
    }
//
// Actually this should not be needed, if the built-in timeout works reliably in
// the both PIO cases and DMA never fails. But if DMA does fail, a timeout
// handler might be the only way to catch the error.
//
#[no_mangle]
unsafe extern "C" fn usdhi6_timeout_work(work: *mut work_struct) {
    static void usdhi6_timeout_work(struct work_struct *work)
    {
    struct delayed_work *d = to_delayed_work(work);
    struct usdhi6_host *host = container_of(d, struct usdhi6_host, timeout_work);
    struct mmc_request *mrq = host.mrq;
    struct mmc_data *data = mrq ? mrq.data : core::ptr::null_mut();
    struct scatterlist *sg;
    dev_warn(mmc_dev(host.mmc),
    "%s timeout wait %u CMD%d: IRQ 0x%08x:0x%08x, last IRQ 0x%08x\n",
    host.dma_active ? "DMA" : "PIO",
    host.wait, mrq ? mrq.cmd.opcode : -1,
    usdhi6_read(host, USDHI6_SD_INFO1),
    usdhi6_read(host, USDHI6_SD_INFO2), host.irq_status);
    if (host.dma_active) {
    usdhi6_dma_kill(host);
    usdhi6_dma_stop_unmap(host);
    }
    switch (host.wait) {
    default:
    dev_err(mmc_dev(host.mmc), "Invalid state %u\n", host.wait);
    fallthrough;	/* mrq can be core::ptr::null_mut(), but is impossible */
    case USDHI6_WAIT_FOR_CMD:
    usdhi6_error_code(host);
    if (mrq)
    mrq.cmd.error = -ETIMEDOUT;
    break;
    case USDHI6_WAIT_FOR_STOP:
    usdhi6_error_code(host);
    mrq.stop.error = -ETIMEDOUT;
    break;
    case USDHI6_WAIT_FOR_DMA:
    case USDHI6_WAIT_FOR_MREAD:
    case USDHI6_WAIT_FOR_MWRITE:
    case USDHI6_WAIT_FOR_READ:
    case USDHI6_WAIT_FOR_WRITE:
    sg = host.sg ?: data.sg;
    dev_dbg(mmc_dev(host.mmc),
    "%c: page #%u @ +0x%zx %ux%u in SG%u. Current SG %u bytes @ %u\n",
    data.flags & MMC_DATA_READ ? 'R' : 'W', host.page_idx,
    host.offset, data.blocks, data.blksz, data.sg_len,
    sg_dma_len(sg), sg.offset);
    usdhi6_sg_unmap(host, true);
    fallthrough;	/* page unmapped in USDHI6_WAIT_FOR_DATA_END */
    case USDHI6_WAIT_FOR_DATA_END:
    usdhi6_error_code(host);
    data.error = -ETIMEDOUT;
    }
    if (mrq)
    usdhi6_request_done(host);
    }
// Probe / release
    static const struct of_device_id usdhi6_of_match[] = {
    {.compatible = "renesas,usdhi6rol0"},
    {}
    };
    MODULE_DEVICE_TABLE(of, usdhi6_of_match);
#[no_mangle]
unsafe extern "C" fn usdhi6_probe(pdev: *mut platform_device) -> c_int {
    static int usdhi6_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mmc_host *mmc;
    struct usdhi6_host *host;
    struct resource *res;
    int irq_cd, irq_sd, irq_sdio;
    u32 version;
    int ret;
    if (!dev.of_node)
    return -ENODEV;
    irq_cd = platform_get_irq_byname(pdev, "card detect");
    irq_sd = platform_get_irq_byname(pdev, "data");
    irq_sdio = platform_get_irq_byname(pdev, "SDIO");
    if (irq_sd < 0)
    return irq_sd;
    if (irq_sdio < 0)
    return irq_sdio;
    mmc = devm_mmc_alloc_host(dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    ret = mmc_regulator_get_supply(mmc);
    if (ret)
    return ret;
    ret = mmc_of_parse(mmc);
    if (ret < 0)
    return ret;
    host		= mmc_priv(mmc);
    host.mmc	= mmc;
    host.wait	= USDHI6_WAIT_FOR_REQUEST;
    host.timeout	= msecs_to_jiffies(USDHI6_REQ_TIMEOUT_MS);
//
// We use a fixed timeout of 4s, hence inform the core about it. A
// future improvement should instead respect the cmd->busy_timeout.
//
    mmc.max_busy_timeout = USDHI6_REQ_TIMEOUT_MS;
    host.pinctrl = devm_pinctrl_get(&pdev.dev);
    if (IS_ERR(host.pinctrl))
    return PTR_ERR(host.pinctrl);
    host.pins_uhs = pinctrl_lookup_state(host.pinctrl, "state_uhs");
    host.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(host.base))
    return PTR_ERR(host.base);
    host.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return PTR_ERR(host.clk);
    host.imclk = clk_get_rate(host.clk);
    ret = clk_prepare_enable(host.clk);
    if (ret < 0)
    return ret;
    version = usdhi6_read(host, USDHI6_VERSION);
    if ((version & 0xfff) != 0xa0d) {
    ret = -EPERM;
    dev_err(dev, "Version not recognized %x\n", version);
    goto e_clk_off;
    }
    dev_info(dev, "A USDHI6ROL0 SD host detected with %d ports\n",
    usdhi6_read(host, USDHI6_SD_PORT_SEL) >> USDHI6_SD_PORT_SEL_PORTS_SHIFT);
    usdhi6_mask_all(host);
    if (irq_cd >= 0) {
    ret = devm_request_irq(dev, irq_cd, usdhi6_cd, 0,
    dev_name(dev), host);
    if (ret < 0)
    goto e_clk_off;
    } else {
    mmc.caps |= MMC_CAP_NEEDS_POLL;
    }
    ret = devm_request_threaded_irq(dev, irq_sd, usdhi6_sd, usdhi6_sd_bh, 0,
    dev_name(dev), host);
    if (ret < 0)
    goto e_clk_off;
    ret = devm_request_irq(dev, irq_sdio, usdhi6_sdio, 0,
    dev_name(dev), host);
    if (ret < 0)
    goto e_clk_off;
    INIT_DELAYED_WORK(&host.timeout_work, usdhi6_timeout_work);
    usdhi6_dma_request(host, res.start);
    mmc.ops = &usdhi6_ops;
    mmc.caps |= MMC_CAP_SD_HIGHSPEED | MMC_CAP_MMC_HIGHSPEED |
    MMC_CAP_SDIO_IRQ;
// Set .max_segs to some random number. Feel free to adjust.
    mmc.max_segs = 32;
    mmc.max_blk_size = 512;
    mmc.max_req_size = PAGE_SIZE * mmc.max_segs;
    mmc.max_blk_count = mmc.max_req_size / mmc.max_blk_size;
//
// Setting .max_seg_size to 1 page would simplify our page-mapping code,
// But OTOH, having large segments makes DMA more efficient. We could
// check, whether we managed to get DMA and fall back to 1 page
// segments, but if we do manage to obtain DMA and then it fails at
// run-time and we fall back to PIO, we will continue getting large
// segments. So, we wouldn't be able to get rid of the code anyway.
//
    mmc.max_seg_size = mmc.max_req_size;
    if (!mmc.f_max)
    mmc.f_max = host.imclk;
    mmc.f_min = host.imclk / 512;
    platform_set_drvdata(pdev, host);
    ret = mmc_add_host(mmc);
    if (ret < 0)
    goto e_release_dma;
    return 0;
    e_release_dma:
    usdhi6_dma_release(host);
    e_clk_off:
    clk_disable_unprepare(host.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usdhi6_remove(pdev: *mut platform_device) {
    static void usdhi6_remove(struct platform_device *pdev)
    {
    struct usdhi6_host *host = platform_get_drvdata(pdev);
    mmc_remove_host(host.mmc);
    usdhi6_mask_all(host);
    cancel_delayed_work_sync(&host.timeout_work);
    usdhi6_dma_release(host);
    clk_disable_unprepare(host.clk);
    }
    static struct platform_driver usdhi6_driver = {
    .probe		= usdhi6_probe,
    .remove		= usdhi6_remove,
    .driver		= {
    .name	= "usdhi6rol0",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = usdhi6_of_match,
    },
    };
    module_platform_driver(usdhi6_driver);
    MODULE_DESCRIPTION("Renesas usdhi6rol0 SD/SDIO host driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:usdhi6rol0");
    MODULE_AUTHOR("Guennadi Liakhovetski <g.liakhovetski@gmx.de>");
