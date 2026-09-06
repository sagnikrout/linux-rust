//! Automatically rewritten from C to Rust
//! Source: drivers/usb/musb/tusb6010_omap.c
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
// TUSB6010 USB 2.0 OTG Dual Role controller OMAP DMA interface
//
// Copyright (C) 2006 Nokia Corporation
// Tony Lindgren <tony@atomide.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tusb_dma_data {
    pub dmareq: i8,
    pub chan: *mut dma_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tusb_omap_dma_ch {
    pub musb: *mut musb,
    pub tbase: *mut void __iomem,
    pub phys_offset: c_ulong,
    pub epnum: c_int,
    pub tx: u8,
    pub hw_ep: *mut musb_hw_ep,
    pub dma_data: *mut tusb_dma_data,
    pub tusb_dma: *mut tusb_omap_dma,
    pub dma_addr: dma_addr_t,
    pub len: u32,
    pub packet_sz: u16,
    pub transfer_packet_sz: u16,
    pub transfer_len: u32,
    pub completed_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tusb_omap_dma {
    pub controller: dma_controller,
    pub tbase: *mut void __iomem,
    pub dma_pool: [tusb_dma_data; MAX_DMAREQ],
    pub multichannel:1: unsigned,
}

//
// Allocate dmareq0 to the current channel unless it's already taken
//
#[no_mangle]
pub unsafe extern "C" fn tusb_omap_use_shared_dmareq(chdat: *mut tusb_omap_dma_ch) -> c_int {
    static inline int tusb_omap_use_shared_dmareq(struct tusb_omap_dma_ch *chdat)
    {
    let mut reg: u32 = musb_readl(chdat.tbase, TUSB_DMA_EP_MAP);
    if (reg != 0) {
    dev_dbg(chdat.musb.controller, "ep%i dmareq0 is busy for ep%i\n",
    chdat.epnum, reg & 0xf);
    return -EAGAIN;
    }
    if (chdat.tx)
    reg = (1 << 4) | chdat.epnum;
    else
    reg = chdat.epnum;
    musb_writel(chdat.tbase, TUSB_DMA_EP_MAP, reg);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tusb_omap_free_shared_dmareq(chdat: *mut tusb_omap_dma_ch) {
    static inline void tusb_omap_free_shared_dmareq(struct tusb_omap_dma_ch *chdat)
    {
    let mut reg: u32 = musb_readl(chdat.tbase, TUSB_DMA_EP_MAP);
    if ((reg & 0xf) != chdat.epnum) {
    printk(KERN_ERR "ep%i trying to release dmareq0 for ep%i\n",
    chdat.epnum, reg & 0xf);
    return;
    }
    musb_writel(chdat.tbase, TUSB_DMA_EP_MAP, 0);
    }
//
// See also musb_dma_completion in plat_uds.c and musb_g_[tx|rx]() in
// musb_gadget.c.
//
#[no_mangle]
unsafe extern "C" fn tusb_omap_dma_cb(data: *mut c_void) {
    static void tusb_omap_dma_cb(void *data)
    {
    struct dma_channel	*channel = (struct dma_channel *)data;
    struct tusb_omap_dma_ch	*chdat = to_chdat(channel);
    struct tusb_omap_dma	*tusb_dma = chdat.tusb_dma;
    struct musb		*musb = chdat.musb;
    struct device		*dev = musb.controller;
    struct musb_hw_ep	*hw_ep = chdat.hw_ep;
    void __iomem		*ep_conf = hw_ep.conf;
    void __iomem		*mbase = musb.mregs;
    unsigned long		remaining, flags, pio;
    spin_lock_irqsave(&musb.lock, flags);
    dev_dbg(musb.controller, "ep%i %s dma callback\n",
    chdat.epnum, chdat.tx ? "tx" : "rx");
    if (chdat.tx)
    remaining = musb_readl(ep_conf, TUSB_EP_TX_OFFSET);
    else
    remaining = musb_readl(ep_conf, TUSB_EP_RX_OFFSET);
    remaining = TUSB_EP_CONFIG_XFR_SIZE(remaining);
// HW issue #10: XFR_SIZE may get corrupt on DMA (both async & sync)
    if (unlikely(remaining > chdat.transfer_len)) {
    dev_dbg(musb.controller, "Corrupt %s XFR_SIZE: 0x%08lx\n",
    chdat.tx ? "tx" : "rx", remaining);
    remaining = 0;
    }
    channel.actual_len = chdat.transfer_len - remaining;
    pio = chdat.len - channel.actual_len;
    dev_dbg(musb.controller, "DMA remaining %lu/%u\n", remaining, chdat.transfer_len);
// Transfer remaining 1 - 31 bytes
    if (pio > 0 && pio < 32) {
    u8	*buf;
    dev_dbg(musb.controller, "Using PIO for remaining %lu bytes\n", pio);
    buf = phys_to_virt((u32)chdat.dma_addr) + chdat.transfer_len;
    if (chdat.tx) {
    dma_unmap_single(dev, chdat.dma_addr,
    chdat.transfer_len,
    DMA_TO_DEVICE);
    musb_write_fifo(hw_ep, pio, buf);
    } else {
    dma_unmap_single(dev, chdat.dma_addr,
    chdat.transfer_len,
    DMA_FROM_DEVICE);
    musb_read_fifo(hw_ep, pio, buf);
    }
    channel.actual_len += pio;
    }
    if (!tusb_dma.multichannel)
    tusb_omap_free_shared_dmareq(chdat);
    channel.status = MUSB_DMA_STATUS_FREE;
    musb_dma_completion(musb, chdat.epnum, chdat.tx);
// We must terminate short tx transfers manually by setting TXPKTRDY.
// REVISIT: This same problem may occur with other MUSB dma as well.
// Easy to test with g_ether by pinging the MUSB board with ping -s54.
//
    if ((chdat.transfer_len < chdat.packet_sz)
    || (chdat.transfer_len % chdat.packet_sz != 0)) {
    u16	csr;
    if (chdat.tx) {
    dev_dbg(musb.controller, "terminating short tx packet\n");
    musb_ep_select(mbase, chdat.epnum);
    csr = musb_readw(hw_ep.regs, MUSB_TXCSR);
    csr |= MUSB_TXCSR_MODE | MUSB_TXCSR_TXPKTRDY
    | MUSB_TXCSR_P_WZC_BITS;
    musb_writew(hw_ep.regs, MUSB_TXCSR, csr);
    }
    }
    spin_unlock_irqrestore(&musb.lock, flags);
    }
    static int tusb_omap_dma_program(struct dma_channel *channel, u16 packet_sz,
    u8 rndis_mode, dma_addr_t dma_addr, u32 len)
    {
    struct tusb_omap_dma_ch		*chdat = to_chdat(channel);
    struct tusb_omap_dma		*tusb_dma = chdat.tusb_dma;
    struct musb			*musb = chdat.musb;
    struct device			*dev = musb.controller;
    struct musb_hw_ep		*hw_ep = chdat.hw_ep;
    void __iomem			*mbase = musb.mregs;
    void __iomem			*ep_conf = hw_ep.conf;
    let mut fifo_addr: dma_addr_t = hw_ep.fifo_sync;
    u32				dma_remaining;
    u16				csr;
    u32				psize;
    struct tusb_dma_data		*dma_data;
    struct dma_async_tx_descriptor	*dma_desc;
    struct dma_slave_config		dma_cfg;
    enum dma_transfer_direction	dma_dir;
    u32				port_window;
    int				ret;
    if (unlikely(dma_addr & 0x1) || (len < 32) || (len > packet_sz))
    return false;
//
// HW issue #10: Async dma will eventually corrupt the XFR_SIZE
// register which will cause missed DMA interrupt. We could try to
// use a timer for the callback, but it is unsafe as the XFR_SIZE
// register is corrupt, and we won't know if the DMA worked.
//
    if (dma_addr & 0x2)
    return false;
//
// Because of HW issue #10, it seems like mixing sync DMA and async
// PIO access can confuse the DMA. Make sure XFR_SIZE is reset before
// using the channel for DMA.
//
    if (chdat.tx)
    dma_remaining = musb_readl(ep_conf, TUSB_EP_TX_OFFSET);
    else
    dma_remaining = musb_readl(ep_conf, TUSB_EP_RX_OFFSET);
    dma_remaining = TUSB_EP_CONFIG_XFR_SIZE(dma_remaining);
    if (dma_remaining) {
    dev_dbg(musb.controller, "Busy %s dma, not using: %08x\n",
    chdat.tx ? "tx" : "rx", dma_remaining);
    return false;
    }
    chdat.transfer_len = len & ~0x1f;
    if (len < packet_sz)
    chdat.transfer_packet_sz = chdat.transfer_len;
    else
    chdat.transfer_packet_sz = packet_sz;
    dma_data = chdat.dma_data;
    if (!tusb_dma.multichannel) {
    if (tusb_omap_use_shared_dmareq(chdat) != 0) {
    dev_dbg(musb.controller, "could not get dma for ep%i\n", chdat.epnum);
    return false;
    }
    if (dma_data.dmareq < 0) {
// REVISIT: This should get blocked earlier, happens
// with MSC ErrorRecoveryTest
//
    WARN_ON(1);
    return false;
    }
    }
    chdat.packet_sz = packet_sz;
    chdat.len = len;
    channel.actual_len = 0;
    chdat.dma_addr = dma_addr;
    channel.status = MUSB_DMA_STATUS_BUSY;
// Since we're recycling dma areas, we need to clean or invalidate
    if (chdat.tx) {
    dma_dir = DMA_MEM_TO_DEV;
    dma_map_single(dev, phys_to_virt(dma_addr), len,
    DMA_TO_DEVICE);
    } else {
    dma_dir = DMA_DEV_TO_MEM;
    dma_map_single(dev, phys_to_virt(dma_addr), len,
    DMA_FROM_DEVICE);
    }
    memset(&dma_cfg, 0, sizeof(dma_cfg));
// Use 16-bit transfer if dma_addr is not 32-bit aligned
    if ((dma_addr & 0x3) == 0) {
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    port_window = 8;
    } else {
    dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    port_window = 16;
    fifo_addr = hw_ep.fifo_async;
    }
    dev_dbg(musb.controller,
    "ep%i %s dma: %pad len: %u(%u) packet_sz: %i(%i)\n",
    chdat.epnum, chdat.tx ? "tx" : "rx", &dma_addr,
    chdat.transfer_len, len, chdat.transfer_packet_sz, packet_sz);
    dma_cfg.src_addr = fifo_addr;
    dma_cfg.dst_addr = fifo_addr;
    dma_cfg.src_port_window_size = port_window;
    dma_cfg.src_maxburst = port_window;
    dma_cfg.dst_port_window_size = port_window;
    dma_cfg.dst_maxburst = port_window;
    ret = dmaengine_slave_config(dma_data.chan, &dma_cfg);
    if (ret) {
    dev_err(musb.controller, "DMA slave config failed: %d\n", ret);
    return false;
    }
    dma_desc = dmaengine_prep_slave_single(dma_data.chan, dma_addr,
    chdat.transfer_len, dma_dir,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!dma_desc) {
    dev_err(musb.controller, "DMA prep_slave_single failed\n");
    return false;
    }
    dma_desc.callback = tusb_omap_dma_cb;
    dma_desc.callback_param = channel;
    dmaengine_submit(dma_desc);
    dev_dbg(musb.controller,
    "ep%i %s using %i-bit %s dma from %pad to %pad\n",
    chdat.epnum, chdat.tx ? "tx" : "rx",
    dma_cfg.src_addr_width * 8,
    ((dma_addr & 0x3) == 0) ? "sync" : "async",
    (dma_dir == DMA_MEM_TO_DEV) ? &dma_addr : &fifo_addr,
    (dma_dir == DMA_MEM_TO_DEV) ? &fifo_addr : &dma_addr);
//
// Prepare MUSB for DMA transfer
//
    musb_ep_select(mbase, chdat.epnum);
    if (chdat.tx) {
    csr = musb_readw(hw_ep.regs, MUSB_TXCSR);
    csr |= (MUSB_TXCSR_AUTOSET | MUSB_TXCSR_DMAENAB
    | MUSB_TXCSR_DMAMODE | MUSB_TXCSR_MODE);
    csr &= ~MUSB_TXCSR_P_UNDERRUN;
    musb_writew(hw_ep.regs, MUSB_TXCSR, csr);
    } else {
    csr = musb_readw(hw_ep.regs, MUSB_RXCSR);
    csr |= MUSB_RXCSR_DMAENAB;
    csr &= ~(MUSB_RXCSR_AUTOCLEAR | MUSB_RXCSR_DMAMODE);
    musb_writew(hw_ep.regs, MUSB_RXCSR,
    csr | MUSB_RXCSR_P_WZC_BITS);
    }
// Start DMA transfer
    dma_async_issue_pending(dma_data.chan);
    if (chdat.tx) {
// Send transfer_packet_sz packets at a time
    psize = musb_readl(ep_conf, TUSB_EP_MAX_PACKET_SIZE_OFFSET);
    psize &= ~0x7ff;
    psize |= chdat.transfer_packet_sz;
    musb_writel(ep_conf, TUSB_EP_MAX_PACKET_SIZE_OFFSET, psize);
    musb_writel(ep_conf, TUSB_EP_TX_OFFSET,
    TUSB_EP_CONFIG_XFR_SIZE(chdat.transfer_len));
    } else {
// Receive transfer_packet_sz packets at a time
    psize = musb_readl(ep_conf, TUSB_EP_MAX_PACKET_SIZE_OFFSET);
    psize &= ~(0x7ff << 16);
    psize |= (chdat.transfer_packet_sz << 16);
    musb_writel(ep_conf, TUSB_EP_MAX_PACKET_SIZE_OFFSET, psize);
    musb_writel(ep_conf, TUSB_EP_RX_OFFSET,
    TUSB_EP_CONFIG_XFR_SIZE(chdat.transfer_len));
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tusb_omap_dma_abort(channel: *mut dma_channel) -> c_int {
    static int tusb_omap_dma_abort(struct dma_channel *channel)
    {
    struct tusb_omap_dma_ch	*chdat = to_chdat(channel);
    if (chdat.dma_data)
    dmaengine_terminate_all(chdat.dma_data.chan);
    channel.status = MUSB_DMA_STATUS_FREE;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tusb_omap_dma_allocate_dmareq(chdat: *mut tusb_omap_dma_ch) -> c_int {
    static inline int tusb_omap_dma_allocate_dmareq(struct tusb_omap_dma_ch *chdat)
    {
    let mut reg: u32 = musb_readl(chdat.tbase, TUSB_DMA_EP_MAP);
    int		i, dmareq_nr = -1;
    for (i = 0; i < MAX_DMAREQ; i++) {
    let mut cur: c_int = (reg & (0xf << (i * 5))) >> (i * 5);
    if (cur == 0) {
    dmareq_nr = i;
    break;
    }
    }
    if (dmareq_nr == -1)
    return -EAGAIN;
    reg |= (chdat.epnum << (dmareq_nr * 5));
    if (chdat.tx)
    reg |= ((1 << 4) << (dmareq_nr * 5));
    musb_writel(chdat.tbase, TUSB_DMA_EP_MAP, reg);
    chdat.dma_data = &chdat.tusb_dma.dma_pool[dmareq_nr];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tusb_omap_dma_free_dmareq(chdat: *mut tusb_omap_dma_ch) {
    static inline void tusb_omap_dma_free_dmareq(struct tusb_omap_dma_ch *chdat)
    {
    u32 reg;
    if (!chdat || !chdat.dma_data || chdat.dma_data.dmareq < 0)
    return;
    reg = musb_readl(chdat.tbase, TUSB_DMA_EP_MAP);
    reg &= ~(0x1f << (chdat.dma_data.dmareq * 5));
    musb_writel(chdat.tbase, TUSB_DMA_EP_MAP, reg);
    chdat.dma_data = core::ptr::null_mut();
    }
    static struct dma_channel *dma_channel_pool[MAX_DMAREQ];
    static struct dma_channel *
    tusb_omap_dma_allocate(struct dma_controller *c,
    struct musb_hw_ep *hw_ep,
    u8 tx)
    {
    int ret, i;
    struct tusb_omap_dma	*tusb_dma;
    struct musb		*musb;
    struct dma_channel	*channel = core::ptr::null_mut();
    struct tusb_omap_dma_ch	*chdat = core::ptr::null_mut();
    struct tusb_dma_data	*dma_data = core::ptr::null_mut();
    tusb_dma = container_of(c, struct tusb_omap_dma, controller);
    musb = tusb_dma.controller.musb;
// REVISIT: Why does dmareq5 not work?
    if (hw_ep.epnum == 0) {
    dev_dbg(musb.controller, "Not allowing DMA for ep0 %s\n", tx ? "tx" : "rx");
    return core::ptr::null_mut();
    }
    for (i = 0; i < MAX_DMAREQ; i++) {
    struct dma_channel *ch = dma_channel_pool[i];
    if (ch.status == MUSB_DMA_STATUS_UNKNOWN) {
    ch.status = MUSB_DMA_STATUS_FREE;
    channel = ch;
    chdat = ch.private_data;
    break;
    }
    }
    if (!channel)
    return core::ptr::null_mut();
    chdat.musb = tusb_dma.controller.musb;
    chdat.tbase = tusb_dma.tbase;
    chdat.hw_ep = hw_ep;
    chdat.epnum = hw_ep.epnum;
    chdat.completed_len = 0;
    chdat.tusb_dma = tusb_dma;
    if (tx)
    chdat.tx = 1;
    else
    chdat.tx = 0;
    channel.max_len = 0x7fffffff;
    channel.desired_mode = 0;
    channel.actual_len = 0;
    if (!chdat.dma_data) {
    if (tusb_dma.multichannel) {
    ret = tusb_omap_dma_allocate_dmareq(chdat);
    if (ret != 0)
    goto free_dmareq;
    } else {
    chdat.dma_data = &tusb_dma.dma_pool[0];
    }
    }
    dma_data = chdat.dma_data;
    dev_dbg(musb.controller, "ep%i %s dma: %s dmareq%i\n",
    chdat.epnum,
    chdat.tx ? "tx" : "rx",
    tusb_dma.multichannel ? "shared" : "dedicated",
    dma_data.dmareq);
    return channel;
    free_dmareq:
    tusb_omap_dma_free_dmareq(chdat);
    dev_dbg(musb.controller, "ep%i: Could not get a DMA channel\n", chdat.epnum);
    channel.status = MUSB_DMA_STATUS_UNKNOWN;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tusb_omap_dma_release(channel: *mut dma_channel) {
    static void tusb_omap_dma_release(struct dma_channel *channel)
    {
    struct tusb_omap_dma_ch	*chdat = to_chdat(channel);
    struct musb		*musb = chdat.musb;
    dev_dbg(musb.controller, "Release for ep%i\n", chdat.epnum);
    channel.status = MUSB_DMA_STATUS_UNKNOWN;
    dmaengine_terminate_sync(chdat.dma_data.chan);
    tusb_omap_dma_free_dmareq(chdat);
    channel = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn tusb_dma_controller_destroy(c: *mut dma_controller) {
    void tusb_dma_controller_destroy(struct dma_controller *c)
    {
    struct tusb_omap_dma	*tusb_dma;
    int			i;
    tusb_dma = container_of(c, struct tusb_omap_dma, controller);
    for (i = 0; i < MAX_DMAREQ; i++) {
    struct dma_channel *ch = dma_channel_pool[i];
    if (ch) {
    kfree(ch.private_data);
    kfree(ch);
    }
// Free up the DMA channels
    if (tusb_dma && tusb_dma.dma_pool[i].chan)
    dma_release_channel(tusb_dma.dma_pool[i].chan);
    }
    kfree(tusb_dma);
    }
    EXPORT_SYMBOL_GPL(tusb_dma_controller_destroy);
#[no_mangle]
unsafe extern "C" fn tusb_omap_allocate_dma_pool(tusb_dma: *mut tusb_omap_dma) -> c_int {
    static int tusb_omap_allocate_dma_pool(struct tusb_omap_dma *tusb_dma)
    {
    struct musb *musb = tusb_dma.controller.musb;
    int i;
    let mut ret: c_int = 0;
    for (i = 0; i < MAX_DMAREQ; i++) {
    struct tusb_dma_data *dma_data = &tusb_dma.dma_pool[i];
//
// Request DMA channels:
// - one channel in case of non multichannel mode
// - MAX_DMAREQ number of channels in multichannel mode
//
    if (i == 0 || tusb_dma.multichannel) {
    char ch_name[8];
    sprintf(ch_name, "dmareq%d", i);
    dma_data.chan = dma_request_chan(musb.controller,
    ch_name);
    if (IS_ERR(dma_data.chan)) {
    dev_err(musb.controller,
    "Failed to request %s\n", ch_name);
    ret = PTR_ERR(dma_data.chan);
    goto dma_error;
    }
    dma_data.dmareq = i;
    } else {
    dma_data.dmareq = -1;
    }
    }
    return 0;
    dma_error:
    for (; i >= 0; i--) {
    struct tusb_dma_data *dma_data = &tusb_dma.dma_pool[i];
    if (dma_data.dmareq >= 0)
    dma_release_channel(dma_data.chan);
    }
    return ret;
    }
    struct dma_controller *
    tusb_dma_controller_create(struct musb *musb, void __iomem *base)
    {
    void __iomem		*tbase = musb.ctrl_base;
    struct tusb_omap_dma	*tusb_dma;
    int			i;
// REVISIT: Get dmareq lines used from board-*.c
    musb_writel(musb.ctrl_base, TUSB_DMA_INT_MASK, 0x7fffffff);
    musb_writel(musb.ctrl_base, TUSB_DMA_EP_MAP, 0);
    musb_writel(tbase, TUSB_DMA_REQ_CONF,
    TUSB_DMA_REQ_CONF_BURST_SIZE(2)
    | TUSB_DMA_REQ_CONF_DMA_REQ_EN(0x3f)
    | TUSB_DMA_REQ_CONF_DMA_REQ_ASSER(2));
    tusb_dma = kzalloc_obj(struct tusb_omap_dma);
    if (!tusb_dma)
    goto out;
    tusb_dma.controller.musb = musb;
    tusb_dma.tbase = musb.ctrl_base;
    tusb_dma.controller.channel_alloc = tusb_omap_dma_allocate;
    tusb_dma.controller.channel_release = tusb_omap_dma_release;
    tusb_dma.controller.channel_program = tusb_omap_dma_program;
    tusb_dma.controller.channel_abort = tusb_omap_dma_abort;
    if (musb.tusb_revision >= TUSB_REV_30)
    tusb_dma.multichannel = 1;
    for (i = 0; i < MAX_DMAREQ; i++) {
    struct dma_channel	*ch;
    struct tusb_omap_dma_ch	*chdat;
    ch = kzalloc_obj(struct dma_channel);
    if (!ch)
    goto cleanup;
    dma_channel_pool[i] = ch;
    chdat = kzalloc_obj(struct tusb_omap_dma_ch);
    if (!chdat)
    goto cleanup;
    ch.status = MUSB_DMA_STATUS_UNKNOWN;
    ch.private_data = chdat;
    }
    if (tusb_omap_allocate_dma_pool(tusb_dma))
    goto cleanup;
    return &tusb_dma.controller;
    cleanup:
    musb_dma_controller_destroy(&tusb_dma.controller);
    out:
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(tusb_dma_controller_create);
