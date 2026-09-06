//! Automatically rewritten from C to Rust
//! Source: drivers/dma/sun4i-dma.c
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
// Copyright (C) 2014 Emilio López
// Emilio López <emilio@elopez.com.ar>
//

// Common macros to normal and dedicated DMA registers

pub const SUN4I_MAX_BURST: c_int = 8;
pub const SUNIV_MAX_BURST: c_int = 4;
// Normal DMA register values
// Normal DMA source/destination data request type values
pub const SUN4I_NDMA_DRQ_TYPE_SDRAM: c_uint = 0x16;

pub const SUNIV_NDMA_DRQ_TYPE_SDRAM: c_uint = 0x11;

// Normal DMA register layout
// Dedicated DMA source/destination address mode values
pub const SUN4I_NDMA_ADDR_MODE_LINEAR: c_int = 0;
pub const SUN4I_NDMA_ADDR_MODE_IO: c_int = 1;
// Normal DMA configuration register layout

// Dedicated DMA register values
// Dedicated DMA source/destination address mode values
pub const SUN4I_DDMA_ADDR_MODE_LINEAR: c_int = 0;
pub const SUN4I_DDMA_ADDR_MODE_IO: c_int = 1;
pub const SUN4I_DDMA_ADDR_MODE_HORIZONTAL_PAGE: c_int = 2;
pub const SUN4I_DDMA_ADDR_MODE_VERTICAL_PAGE: c_int = 3;
// Dedicated DMA source/destination data request type values
pub const SUN4I_DDMA_DRQ_TYPE_SDRAM: c_uint = 0x1;

pub const SUNIV_DDMA_DRQ_TYPE_SDRAM: c_uint = 0x1;

// Dedicated DMA register layout
// Dedicated DMA configuration register layout

// Dedicated DMA parameter register layout

// DMA register offsets
// General register offsets
pub const SUN4I_DMA_IRQ_ENABLE_REG: c_uint = 0x0;
pub const SUN4I_DMA_IRQ_PENDING_STATUS_REG: c_uint = 0x4;
// Normal DMA register offsets

pub const SUN4I_NDMA_CFG_REG: c_uint = 0x0;
pub const SUN4I_NDMA_SRC_ADDR_REG: c_uint = 0x4;
pub const SUN4I_NDMA_DST_ADDR_REG: c_uint = 0x8;
pub const SUN4I_NDMA_BYTE_COUNT_REG: c_uint = 0xC;
// Dedicated DMA register offsets

pub const SUN4I_DDMA_CFG_REG: c_uint = 0x0;
pub const SUN4I_DDMA_SRC_ADDR_REG: c_uint = 0x4;
pub const SUN4I_DDMA_DST_ADDR_REG: c_uint = 0x8;
pub const SUN4I_DDMA_BYTE_COUNT_REG: c_uint = 0xC;
pub const SUN4I_DDMA_PARA_REG: c_uint = 0x18;
// DMA Driver
//
// Normal DMA has 8 channels, and Dedicated DMA has another 8, so
// that's 16 channels. As for endpoints, there's 29 and 21
// respectively. Given that the Normal DMA endpoints (other than
// SDRAM) can be used as tx/rx, we need 78 vchans in total
//
pub const SUN4I_NDMA_NR_MAX_CHANNELS: c_int = 8;
pub const SUN4I_DDMA_NR_MAX_CHANNELS: c_int = 8;

    (SUN4I_NDMA_NR_MAX_CHANNELS + SUN4I_DDMA_NR_MAX_CHANNELS)

pub const SUN4I_DDMA_NR_MAX_VCHANS: c_int = 21;

    (SUN4I_NDMA_NR_MAX_VCHANS + SUN4I_DDMA_NR_MAX_VCHANS)
pub const SUNIV_NDMA_NR_MAX_CHANNELS: c_int = 4;
pub const SUNIV_DDMA_NR_MAX_CHANNELS: c_int = 4;

pub const SUNIV_DDMA_NR_MAX_VCHANS: c_int = 10;
// This set of SUN4I_DDMA timing parameters were found experimentally while
// working with the SPI driver and seem to make it behave correctly

    (SUN4I_DDMA_PARA_DST_DATA_BLK_SIZE(1) |			\
    SUN4I_DDMA_PARA_SRC_DATA_BLK_SIZE(1) |				\
    SUN4I_DDMA_PARA_DST_WAIT_CYCLES(2) |				\
    SUN4I_DDMA_PARA_SRC_WAIT_CYCLES(2))
//
// Normal DMA supports individual transfers (segments) up to 128k.
// Dedicated DMA supports transfers up to 16M. We can only report
// one size limit, so we have to use the smaller value.
//

//
// Hardware channels / ports representation
//
// The hardware is used in several SoCs, with differing numbers
// of channels and endpoints. This structure ties those numbers
// to a certain compatible string.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_config {
    pub ndma_nr_max_channels: u32,
    pub ndma_nr_max_vchans: u32,
    pub ddma_nr_max_channels: u32,
    pub ddma_nr_max_vchans: u32,
    pub dma_nr_max_channels: u32,
    pub data_width): *mut *mut *mut void (set_dst_data_width)(u32 p_cfg, s8,
    pub data_width): *mut *mut *mut void (set_src_data_width)(u32 p_cfg, s8,
    pub maxburst): *mut *mut int (convert_burst)(u32,
    pub ndma_drq_sdram: u8,
    pub ddma_drq_sdram: u8,
    pub max_burst: u8,
    pub has_reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_pchan {
// Register base of channel
    pub base: *mut void __iomem,
// vchan currently being serviced
    pub vchan: *mut sun4i_dma_vchan,
// Is this a dedicated pchan?
    pub is_dedicated: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_vchan {
    pub vc: virt_dma_chan,
    pub cfg: dma_slave_config,
    pub pchan: *mut sun4i_dma_pchan,
    pub processing: *mut sun4i_dma_promise,
    pub contract: *mut sun4i_dma_contract,
    pub endpoint: u8,
    pub is_dedicated: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_promise {
    pub cfg: u32,
    pub para: u32,
    pub src: dma_addr_t,
    pub dst: dma_addr_t,
    pub len: usize,
    pub list: list_head,
}

// A contract is a set of promises
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_contract {
    pub vd: virt_dma_desc,
    pub demands: list_head,
    pub completed_demands: list_head,
    pub 1: bool is_cyclic :,
    pub 1: bool use_half_int :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_dma_dev {
    pub pchans_used: *mut c_ulong,
    pub slave: dma_device,
    pub pchans: *mut sun4i_dma_pchan,
    pub vchans: *mut sun4i_dma_vchan,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub irq: c_int,
    pub lock: spinlock_t,
    pub cfg: *const sun4i_dma_config,
    pub rst: *mut reset_control,
}

    static struct sun4i_dma_dev *to_sun4i_dma_dev(struct dma_device *dev)
    {
    return container_of(dev, struct sun4i_dma_dev, slave);
    }
    static struct sun4i_dma_vchan *to_sun4i_dma_vchan(struct dma_chan *chan)
    {
    return container_of(chan, struct sun4i_dma_vchan, vc.chan);
    }
    static struct sun4i_dma_contract *to_sun4i_dma_contract(struct virt_dma_desc *vd)
    {
    return container_of(vd, struct sun4i_dma_contract, vd);
    }
    static struct device *chan2dev(struct dma_chan *chan)
    {
    return &chan.dev.device;
    }
#[no_mangle]
unsafe extern "C" fn set_dst_data_width_a10(p_cfg: *mut u32, data_width: i8) {
    static void set_dst_data_width_a10(u32 *p_cfg, s8 data_width)
    {
// p_cfg |= SUN4I_DMA_CFG_DST_DATA_WIDTH(data_width);
    }
#[no_mangle]
unsafe extern "C" fn set_src_data_width_a10(p_cfg: *mut u32, data_width: i8) {
    static void set_src_data_width_a10(u32 *p_cfg, s8 data_width)
    {
// p_cfg |= SUN4I_DMA_CFG_SRC_DATA_WIDTH(data_width);
    }
#[no_mangle]
unsafe extern "C" fn set_dst_data_width_f1c100s(p_cfg: *mut u32, data_width: i8) {
    static void set_dst_data_width_f1c100s(u32 *p_cfg, s8 data_width)
    {
// p_cfg |= SUNIV_DMA_CFG_DST_DATA_WIDTH(data_width);
    }
#[no_mangle]
unsafe extern "C" fn set_src_data_width_f1c100s(p_cfg: *mut u32, data_width: i8) {
    static void set_src_data_width_f1c100s(u32 *p_cfg, s8 data_width)
    {
// p_cfg |= SUNIV_DMA_CFG_SRC_DATA_WIDTH(data_width);
    }
#[no_mangle]
unsafe extern "C" fn convert_burst_a10(maxburst: u32) -> c_int {
    static int convert_burst_a10(u32 maxburst)
    {
    if (maxburst > 8)
    return -EINVAL;
// 1 -> 0, 4 -> 1, 8 -> 2
    return (maxburst >> 2);
    }
#[no_mangle]
unsafe extern "C" fn convert_burst_f1c100s(maxburst: u32) -> c_int {
    static int convert_burst_f1c100s(u32 maxburst)
    {
    if (maxburst > 4)
    return -EINVAL;
// 1 -> 0, 4 -> 1
    return (maxburst >> 2);
    }
#[no_mangle]
unsafe extern "C" fn convert_buswidth(addr_width: enum dma_slave_buswidth) -> c_int {
    static int convert_buswidth(enum dma_slave_buswidth addr_width)
    {
    if (addr_width > DMA_SLAVE_BUSWIDTH_4_BYTES)
    return -EINVAL;
// 8 (1 byte) -> 0, 16 (2 bytes) -> 1, 32 (4 bytes) -> 2
    return (addr_width >> 1);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_free_chan_resources(chan: *mut dma_chan) {
    static void sun4i_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    vchan_free_chan_resources(&vchan.vc);
    }
    static struct sun4i_dma_pchan *find_and_use_pchan(struct sun4i_dma_dev *priv,
    struct sun4i_dma_vchan *vchan)
    {
    struct sun4i_dma_pchan *pchan = core::ptr::null_mut(), *pchans = priv.pchans;
    unsigned long flags;
    int i, max;
//
// pchans 0-priv->cfg->ndma_nr_max_channels are normal, and
// priv->cfg->ndma_nr_max_channels+ are dedicated ones
//
    if (vchan.is_dedicated) {
    i = priv.cfg.ndma_nr_max_channels;
    max = priv.cfg.dma_nr_max_channels;
    } else {
    i = 0;
    max = priv.cfg.ndma_nr_max_channels;
    }
    spin_lock_irqsave(&priv.lock, flags);
    for_each_clear_bit_from(i, priv.pchans_used, max) {
    pchan = &pchans[i];
    pchan.vchan = vchan;
    set_bit(i, priv.pchans_used);
    break;
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return pchan;
    }
    static void release_pchan(struct sun4i_dma_dev *priv,
    struct sun4i_dma_pchan *pchan)
    {
    unsigned long flags;
    let mut nr: c_int = pchan - priv.pchans;
    spin_lock_irqsave(&priv.lock, flags);
    pchan.vchan = core::ptr::null_mut();
    clear_bit(nr, priv.pchans_used);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    static void configure_pchan(struct sun4i_dma_pchan *pchan,
    struct sun4i_dma_promise *d)
    {
//
// Configure addresses and misc parameters depending on type
// SUN4I_DDMA has an extra field with timing parameters
//
    if (pchan.is_dedicated) {
    writel_relaxed(d.src, pchan.base + SUN4I_DDMA_SRC_ADDR_REG);
    writel_relaxed(d.dst, pchan.base + SUN4I_DDMA_DST_ADDR_REG);
    writel_relaxed(d.len, pchan.base + SUN4I_DDMA_BYTE_COUNT_REG);
    writel_relaxed(d.para, pchan.base + SUN4I_DDMA_PARA_REG);
    writel_relaxed(d.cfg, pchan.base + SUN4I_DDMA_CFG_REG);
    } else {
    writel_relaxed(d.src, pchan.base + SUN4I_NDMA_SRC_ADDR_REG);
    writel_relaxed(d.dst, pchan.base + SUN4I_NDMA_DST_ADDR_REG);
    writel_relaxed(d.len, pchan.base + SUN4I_NDMA_BYTE_COUNT_REG);
    writel_relaxed(d.cfg, pchan.base + SUN4I_NDMA_CFG_REG);
    }
    }
    static void set_pchan_interrupt(struct sun4i_dma_dev *priv,
    struct sun4i_dma_pchan *pchan,
    int half, int end)
    {
    u32 reg;
    let mut pchan_number: c_int = pchan - priv.pchans;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    reg = readl_relaxed(priv.base + SUN4I_DMA_IRQ_ENABLE_REG);
    if (half)
    reg |= BIT(pchan_number * 2);
    else
    reg &= ~BIT(pchan_number * 2);
    if (end)
    reg |= BIT(pchan_number * 2 + 1);
    else
    reg &= ~BIT(pchan_number * 2 + 1);
    writel_relaxed(reg, priv.base + SUN4I_DMA_IRQ_ENABLE_REG);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
//
// Execute pending operations on a vchan
//
// When given a vchan, this function will try to acquire a suitable
// pchan and, if successful, will configure it to fulfill a promise
// from the next pending contract.
//
// This function must be called with &vchan->vc.lock held.
//
    static int __execute_vchan_pending(struct sun4i_dma_dev *priv,
    struct sun4i_dma_vchan *vchan)
    {
    struct sun4i_dma_promise *promise = core::ptr::null_mut();
    struct sun4i_dma_contract *contract = core::ptr::null_mut();
    struct sun4i_dma_pchan *pchan;
    struct virt_dma_desc *vd;
    int ret;
    lockdep_assert_held(&vchan.vc.lock);
// We need a pchan to do anything, so secure one if available
    pchan = find_and_use_pchan(priv, vchan);
    if (!pchan)
    return -EBUSY;
//
// Channel endpoints must not be repeated, so if this vchan
// has already submitted some work, we can't do anything else
//
    if (vchan.processing) {
    dev_dbg(chan2dev(&vchan.vc.chan),
    "processing something to this endpoint already\n");
    ret = -EBUSY;
    goto release_pchan;
    }
    do {
// Figure out which contract we're working with today
    vd = vchan_next_desc(&vchan.vc);
    if (!vd) {
    dev_dbg(chan2dev(&vchan.vc.chan),
    "No pending contract found");
    ret = 0;
    goto release_pchan;
    }
    contract = to_sun4i_dma_contract(vd);
    if (list_empty(&contract.demands)) {
// The contract has been completed so mark it as such
    list_del(&contract.vd.node);
    vchan_cookie_complete(&contract.vd);
    dev_dbg(chan2dev(&vchan.vc.chan),
    "Empty contract found and marked complete");
    }
    } while (list_empty(&contract.demands));
// Now find out what we need to do
    promise = list_first_entry(&contract.demands,
    struct sun4i_dma_promise, list);
    vchan.processing = promise;
// ... and make it reality
    if (promise) {
    vchan.contract = contract;
    vchan.pchan = pchan;
    set_pchan_interrupt(priv, pchan, contract.use_half_int, 1);
    configure_pchan(pchan, promise);
    }
    return 0;
    release_pchan:
    release_pchan(priv, pchan);
    return ret;
    }
    static int sanitize_config(struct dma_slave_config *sconfig,
    enum dma_transfer_direction direction)
    {
    switch (direction) {
    case DMA_MEM_TO_DEV:
    if ((sconfig.dst_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED) ||
    !sconfig.dst_maxburst)
    return -EINVAL;
    if (sconfig.src_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED)
    sconfig.src_addr_width = sconfig.dst_addr_width;
    if (!sconfig.src_maxburst)
    sconfig.src_maxburst = sconfig.dst_maxburst;
    break;
    case DMA_DEV_TO_MEM:
    if ((sconfig.src_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED) ||
    !sconfig.src_maxburst)
    return -EINVAL;
    if (sconfig.dst_addr_width == DMA_SLAVE_BUSWIDTH_UNDEFINED)
    sconfig.dst_addr_width = sconfig.src_addr_width;
    if (!sconfig.dst_maxburst)
    sconfig.dst_maxburst = sconfig.src_maxburst;
    break;
    default:
    return 0;
    }
    return 0;
    }
//
// Generate a promise, to be used in a normal DMA contract.
//
// A NDMA promise contains all the information required to program the
// normal part of the DMA Engine and get data copied. A non-executed
// promise will live in the demands list on a contract. Once it has been
// completed, it will be moved to the completed demands list for later freeing.
// All linked promises will be freed when the corresponding contract is freed
//
    static struct sun4i_dma_promise *
    generate_ndma_promise(struct dma_chan *chan, dma_addr_t src, dma_addr_t dest,
    size_t len, struct dma_slave_config *sconfig,
    enum dma_transfer_direction direction)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_promise *promise;
    int ret;
    ret = sanitize_config(sconfig, direction);
    if (ret)
    return core::ptr::null_mut();
    promise = kzalloc_obj(*promise, GFP_NOWAIT);
    if (!promise)
    return core::ptr::null_mut();
    promise.src = src;
    promise.dst = dest;
    promise.len = len;
    promise.cfg = SUN4I_DMA_CFG_LOADING |
    SUN4I_NDMA_CFG_BYTE_COUNT_MODE_REMAIN;
    dev_dbg(chan2dev(chan),
    "src burst %d, dst burst %d, src buswidth %d, dst buswidth %d",
    sconfig.src_maxburst, sconfig.dst_maxburst,
    sconfig.src_addr_width, sconfig.dst_addr_width);
// Source burst
    ret = priv.cfg.convert_burst(sconfig.src_maxburst);
    if (ret < 0)
    goto fail;
    promise.cfg |= SUN4I_DMA_CFG_SRC_BURST_LENGTH(ret);
// Destination burst
    ret = priv.cfg.convert_burst(sconfig.dst_maxburst);
    if (ret < 0)
    goto fail;
    promise.cfg |= SUN4I_DMA_CFG_DST_BURST_LENGTH(ret);
// Source bus width
    ret = convert_buswidth(sconfig.src_addr_width);
    if (ret < 0)
    goto fail;
    priv.cfg.set_src_data_width(&promise.cfg, ret);
// Destination bus width
    ret = convert_buswidth(sconfig.dst_addr_width);
    if (ret < 0)
    goto fail;
    priv.cfg.set_dst_data_width(&promise.cfg, ret);
    return promise;
    fail:
    kfree(promise);
    return core::ptr::null_mut();
    }
//
// Generate a promise, to be used in a dedicated DMA contract.
//
// A DDMA promise contains all the information required to program the
// Dedicated part of the DMA Engine and get data copied. A non-executed
// promise will live in the demands list on a contract. Once it has been
// completed, it will be moved to the completed demands list for later freeing.
// All linked promises will be freed when the corresponding contract is freed
//
    static struct sun4i_dma_promise *
    generate_ddma_promise(struct dma_chan *chan, dma_addr_t src, dma_addr_t dest,
    size_t len, struct dma_slave_config *sconfig)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_promise *promise;
    int ret;
    promise = kzalloc_obj(*promise, GFP_NOWAIT);
    if (!promise)
    return core::ptr::null_mut();
    promise.src = src;
    promise.dst = dest;
    promise.len = len;
    promise.cfg = SUN4I_DMA_CFG_LOADING |
    SUN4I_DDMA_CFG_BYTE_COUNT_MODE_REMAIN;
// Source burst
    ret = priv.cfg.convert_burst(sconfig.src_maxburst);
    if (ret < 0)
    goto fail;
    promise.cfg |= SUN4I_DMA_CFG_SRC_BURST_LENGTH(ret);
// Destination burst
    ret = priv.cfg.convert_burst(sconfig.dst_maxburst);
    if (ret < 0)
    goto fail;
    promise.cfg |= SUN4I_DMA_CFG_DST_BURST_LENGTH(ret);
// Source bus width
    ret = convert_buswidth(sconfig.src_addr_width);
    if (ret < 0)
    goto fail;
    priv.cfg.set_src_data_width(&promise.cfg, ret);
// Destination bus width
    ret = convert_buswidth(sconfig.dst_addr_width);
    if (ret < 0)
    goto fail;
    priv.cfg.set_dst_data_width(&promise.cfg, ret);
    return promise;
    fail:
    kfree(promise);
    return core::ptr::null_mut();
    }
//
// Generate a contract
//
// Contracts function as DMA descriptors. As our hardware does not support
// linked lists, we need to implement SG via software. We use a contract
// to hold all the pieces of the request and process them serially one
// after another. Each piece is represented as a promise.
//
    static struct sun4i_dma_contract *generate_dma_contract(void)
    {
    struct sun4i_dma_contract *contract;
    contract = kzalloc_obj(*contract, GFP_NOWAIT);
    if (!contract)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&contract.demands);
    INIT_LIST_HEAD(&contract.completed_demands);
    return contract;
    }
//
// Get next promise on a cyclic transfer
//
// Cyclic contracts contain a series of promises which are executed on a
// loop. This function returns the next promise from a cyclic contract,
// so it can be programmed into the hardware.
//
    static struct sun4i_dma_promise *
    get_next_cyclic_promise(struct sun4i_dma_contract *contract)
    {
    struct sun4i_dma_promise *promise;
    promise = list_first_entry_or_null(&contract.demands,
    struct sun4i_dma_promise, list);
    if (!promise) {
    list_splice_init(&contract.completed_demands,
    &contract.demands);
    promise = list_first_entry(&contract.demands,
    struct sun4i_dma_promise, list);
    }
    return promise;
    }
//
// Free a contract and all its associated promises
//
#[no_mangle]
unsafe extern "C" fn sun4i_dma_free_contract(vd: *mut virt_dma_desc) {
    static void sun4i_dma_free_contract(struct virt_dma_desc *vd)
    {
    struct sun4i_dma_contract *contract = to_sun4i_dma_contract(vd);
    struct sun4i_dma_promise *promise, *tmp;
// Free all the demands and completed demands
    list_for_each_entry_safe(promise, tmp, &contract.demands, list)
    kfree(promise);
    list_for_each_entry_safe(promise, tmp, &contract.completed_demands, list)
    kfree(promise);
    kfree(contract);
    }
    static struct dma_async_tx_descriptor *
    sun4i_dma_prep_dma_memcpy(struct dma_chan *chan, dma_addr_t dest,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    struct dma_slave_config *sconfig = &vchan.cfg;
    struct sun4i_dma_promise *promise;
    struct sun4i_dma_contract *contract;
    contract = generate_dma_contract();
    if (!contract)
    return core::ptr::null_mut();
//
// We can only do the copy to bus aligned addresses, so
// choose the best one so we get decent performance. We also
// maximize the burst size for this same reason.
//
    sconfig.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    sconfig.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    sconfig.src_maxburst = priv.cfg.max_burst;
    sconfig.dst_maxburst = priv.cfg.max_burst;
    if (vchan.is_dedicated)
    promise = generate_ddma_promise(chan, src, dest, len, sconfig);
    else
    promise = generate_ndma_promise(chan, src, dest, len, sconfig,
    DMA_MEM_TO_MEM);
    if (!promise) {
    kfree(contract);
    return core::ptr::null_mut();
    }
// Configure memcpy mode
    if (vchan.is_dedicated) {
    promise.cfg |=
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(priv.cfg.ddma_drq_sdram) |
    SUN4I_DMA_CFG_DST_DRQ_TYPE(priv.cfg.ddma_drq_sdram);
    } else {
    promise.cfg |=
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(priv.cfg.ndma_drq_sdram) |
    SUN4I_DMA_CFG_DST_DRQ_TYPE(priv.cfg.ndma_drq_sdram);
    }
// Fill the contract with our only promise
    list_add_tail(&promise.list, &contract.demands);
// And add it to the vchan
    return vchan_tx_prep(&vchan.vc, &contract.vd, flags);
    }
    static struct dma_async_tx_descriptor *
    sun4i_dma_prep_dma_cyclic(struct dma_chan *chan, dma_addr_t buf, size_t len,
    size_t period_len, enum dma_transfer_direction dir,
    unsigned long flags)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    struct dma_slave_config *sconfig = &vchan.cfg;
    struct sun4i_dma_promise *promise;
    struct sun4i_dma_contract *contract;
    dma_addr_t src, dest;
    u32 endpoints;
    int nr_periods, offset, plength, i;
    u8 ram_type, io_mode, linear_mode;
    if (!is_slave_direction(dir)) {
    dev_err(chan2dev(chan), "Invalid DMA direction\n");
    return core::ptr::null_mut();
    }
    contract = generate_dma_contract();
    if (!contract)
    return core::ptr::null_mut();
    contract.is_cyclic = 1;
    if (vchan.is_dedicated) {
    io_mode = SUN4I_DDMA_ADDR_MODE_IO;
    linear_mode = SUN4I_DDMA_ADDR_MODE_LINEAR;
    ram_type = priv.cfg.ddma_drq_sdram;
    } else {
    io_mode = SUN4I_NDMA_ADDR_MODE_IO;
    linear_mode = SUN4I_NDMA_ADDR_MODE_LINEAR;
    ram_type = priv.cfg.ndma_drq_sdram;
    }
    if (dir == DMA_MEM_TO_DEV) {
    src = buf;
    dest = sconfig.dst_addr;
    endpoints = SUN4I_DMA_CFG_DST_DRQ_TYPE(vchan.endpoint) |
    SUN4I_DMA_CFG_DST_ADDR_MODE(io_mode) |
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(ram_type) |
    SUN4I_DMA_CFG_SRC_ADDR_MODE(linear_mode);
    } else {
    src = sconfig.src_addr;
    dest = buf;
    endpoints = SUN4I_DMA_CFG_DST_DRQ_TYPE(ram_type) |
    SUN4I_DMA_CFG_DST_ADDR_MODE(linear_mode) |
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(vchan.endpoint) |
    SUN4I_DMA_CFG_SRC_ADDR_MODE(io_mode);
    }
//
// We will be using half done interrupts to make two periods
// out of a promise, so we need to program the DMA engine less
// often
//
// The engine can interrupt on half-transfer, so we can use
// this feature to program the engine half as often as if we
// didn't use it (keep in mind the hardware doesn't support
// linked lists).
//
// Say you have a set of periods (| marks the start/end, I for
// interrupt, P for programming the engine to do a new
// transfer), the easy but slow way would be to do
//
// |---|---|---|---| (periods / promises)
// P  I,P I,P I,P  I
//
// Using half transfer interrupts you can do
//
// |-------|-------| (promises as configured on hw)
// |---|---|---|---| (periods)
// P   I  I,P  I   I
//
// Which requires half the engine programming for the same
// functionality.
//
// This only works if two periods fit in a single promise. That will
// always be the case for dedicated DMA, where the hardware has a much
// larger maximum transfer size than advertised to clients.
//
    if (vchan.is_dedicated || period_len <= SUN4I_NDMA_MAX_SEG_SIZE / 2) {
    period_len *= 2;
    contract.use_half_int = 1;
    }
    nr_periods = DIV_ROUND_UP(len, period_len);
    for (i = 0; i < nr_periods; i++) {
// Calculate the offset in the buffer and the length needed
    offset = i * period_len;
    plength = min((len - offset), period_len);
    if (dir == DMA_MEM_TO_DEV)
    src = buf + offset;
    else
    dest = buf + offset;
// Make the promise
    if (vchan.is_dedicated)
    promise = generate_ddma_promise(chan, src, dest,
    plength, sconfig);
    else
    promise = generate_ndma_promise(chan, src, dest,
    plength, sconfig, dir);
    if (!promise) {
// TODO: should we free everything?
    return core::ptr::null_mut();
    }
    promise.cfg |= endpoints;
// Then add it to the contract
    list_add_tail(&promise.list, &contract.demands);
    }
// And add it to the vchan
    return vchan_tx_prep(&vchan.vc, &contract.vd, flags);
    }
    static struct dma_async_tx_descriptor *
    sun4i_dma_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    struct dma_slave_config *sconfig = &vchan.cfg;
    struct sun4i_dma_promise *promise;
    struct sun4i_dma_contract *contract;
    u8 ram_type, io_mode, linear_mode;
    struct scatterlist *sg;
    dma_addr_t srcaddr, dstaddr;
    u32 endpoints, para;
    int i;
    if (!sgl)
    return core::ptr::null_mut();
    if (!is_slave_direction(dir)) {
    dev_err(chan2dev(chan), "Invalid DMA direction\n");
    return core::ptr::null_mut();
    }
    contract = generate_dma_contract();
    if (!contract)
    return core::ptr::null_mut();
    if (vchan.is_dedicated) {
    io_mode = SUN4I_DDMA_ADDR_MODE_IO;
    linear_mode = SUN4I_DDMA_ADDR_MODE_LINEAR;
    ram_type = priv.cfg.ddma_drq_sdram;
    } else {
    io_mode = SUN4I_NDMA_ADDR_MODE_IO;
    linear_mode = SUN4I_NDMA_ADDR_MODE_LINEAR;
    ram_type = priv.cfg.ndma_drq_sdram;
    }
    if (dir == DMA_MEM_TO_DEV)
    endpoints = SUN4I_DMA_CFG_DST_DRQ_TYPE(vchan.endpoint) |
    SUN4I_DMA_CFG_DST_ADDR_MODE(io_mode) |
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(ram_type) |
    SUN4I_DMA_CFG_SRC_ADDR_MODE(linear_mode);
    else
    endpoints = SUN4I_DMA_CFG_DST_DRQ_TYPE(ram_type) |
    SUN4I_DMA_CFG_DST_ADDR_MODE(linear_mode) |
    SUN4I_DMA_CFG_SRC_DRQ_TYPE(vchan.endpoint) |
    SUN4I_DMA_CFG_SRC_ADDR_MODE(io_mode);
    for_each_sg(sgl, sg, sg_len, i) {
// Figure out addresses
    if (dir == DMA_MEM_TO_DEV) {
    srcaddr = sg_dma_address(sg);
    dstaddr = sconfig.dst_addr;
    } else {
    srcaddr = sconfig.src_addr;
    dstaddr = sg_dma_address(sg);
    }
//
// These are the magic DMA engine timings that keep SPI going.
// I haven't seen any interface on DMAEngine to configure
// timings, and so far they seem to work for everything we
// support, so I've kept them here. I don't know if other
// devices need different timings because, as usual, we only
// have the "para" bitfield meanings, but no comment on what
// the values should be when doing a certain operation :|
//
    para = SUN4I_DDMA_MAGIC_SPI_PARAMETERS;
// And make a suitable promise
    if (vchan.is_dedicated)
    promise = generate_ddma_promise(chan, srcaddr, dstaddr,
    sg_dma_len(sg),
    sconfig);
    else
    promise = generate_ndma_promise(chan, srcaddr, dstaddr,
    sg_dma_len(sg),
    sconfig, dir);
    if (!promise)
    return core::ptr::null_mut(); /* TODO: should we free everything? */
    promise.cfg |= endpoints;
    promise.para = para;
// Then add it to the contract
    list_add_tail(&promise.list, &contract.demands);
    }
//
// Once we've got all the promises ready, add the contract
// to the pending list on the vchan
//
    return vchan_tx_prep(&vchan.vc, &contract.vd, flags);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int sun4i_dma_terminate_all(struct dma_chan *chan)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    struct sun4i_dma_pchan *pchan = vchan.pchan;
    LIST_HEAD(head);
    unsigned long flags;
    spin_lock_irqsave(&vchan.vc.lock, flags);
    vchan_get_all_descriptors(&vchan.vc, &head);
    spin_unlock_irqrestore(&vchan.vc.lock, flags);
//
// Clearing the configuration register will halt the pchan. Interrupts
// may still trigger, so don't forget to disable them.
//
    if (pchan) {
    if (pchan.is_dedicated)
    writel(0, pchan.base + SUN4I_DDMA_CFG_REG);
    else
    writel(0, pchan.base + SUN4I_NDMA_CFG_REG);
    set_pchan_interrupt(priv, pchan, 0, 0);
    release_pchan(priv, pchan);
    }
    spin_lock_irqsave(&vchan.vc.lock, flags);
// Clear these so the vchan is usable again
    vchan.processing = core::ptr::null_mut();
    vchan.pchan = core::ptr::null_mut();
    spin_unlock_irqrestore(&vchan.vc.lock, flags);
    vchan_dma_desc_free_list(&vchan.vc, &head);
    return 0;
    }
    static int sun4i_dma_config(struct dma_chan *chan,
    struct dma_slave_config *config)
    {
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    memcpy(&vchan.cfg, config, sizeof(*config));
    return 0;
    }
    static struct dma_chan *sun4i_dma_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct sun4i_dma_dev *priv = ofdma.of_dma_data;
    struct sun4i_dma_vchan *vchan;
    struct dma_chan *chan;
    let mut is_dedicated: u8 = dma_spec.args[0];
    let mut endpoint: u8 = dma_spec.args[1];
// Check if type is Normal or Dedicated
    if (is_dedicated != 0 && is_dedicated != 1)
    return core::ptr::null_mut();
// Make sure the endpoint looks sane
    if ((is_dedicated && endpoint >= SUN4I_DDMA_DRQ_TYPE_LIMIT) ||
    (!is_dedicated && endpoint >= SUN4I_NDMA_DRQ_TYPE_LIMIT))
    return core::ptr::null_mut();
    chan = dma_get_any_slave_channel(&priv.slave);
    if (!chan)
    return core::ptr::null_mut();
// Assign the endpoint to the vchan
    vchan = to_sun4i_dma_vchan(chan);
    vchan.is_dedicated = is_dedicated;
    vchan.endpoint = endpoint;
    return chan;
    }
    static enum dma_status sun4i_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie,
    struct dma_tx_state *state)
    {
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    struct sun4i_dma_pchan *pchan = vchan.pchan;
    struct sun4i_dma_contract *contract;
    struct sun4i_dma_promise *promise;
    struct virt_dma_desc *vd;
    unsigned long flags;
    enum dma_status ret;
    let mut bytes: usize = 0;
    ret = dma_cookie_status(chan, cookie, state);
    if (!state || (ret == DMA_COMPLETE))
    return ret;
    spin_lock_irqsave(&vchan.vc.lock, flags);
    vd = vchan_find_desc(&vchan.vc, cookie);
    if (!vd)
    goto exit;
    contract = to_sun4i_dma_contract(vd);
    list_for_each_entry(promise, &contract.demands, list)
    bytes += promise.len;
//
// The hardware is configured to return the remaining byte
// quantity. If possible, replace the first listed element's
// full size with the actual remaining amount
//
    promise = list_first_entry_or_null(&contract.demands,
    struct sun4i_dma_promise, list);
    if (promise && pchan) {
    bytes -= promise.len;
    if (pchan.is_dedicated)
    bytes += readl(pchan.base + SUN4I_DDMA_BYTE_COUNT_REG);
    else
    bytes += readl(pchan.base + SUN4I_NDMA_BYTE_COUNT_REG);
    }
    exit:
    dma_set_residue(state, bytes);
    spin_unlock_irqrestore(&vchan.vc.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_issue_pending(chan: *mut dma_chan) {
    static void sun4i_dma_issue_pending(struct dma_chan *chan)
    {
    struct sun4i_dma_dev *priv = to_sun4i_dma_dev(chan.device);
    struct sun4i_dma_vchan *vchan = to_sun4i_dma_vchan(chan);
    unsigned long flags;
    spin_lock_irqsave(&vchan.vc.lock, flags);
//
// If there are pending transactions for this vchan, push one of
// them into the engine to get the ball rolling.
//
    if (vchan_issue_pending(&vchan.vc))
    __execute_vchan_pending(priv, vchan);
    spin_unlock_irqrestore(&vchan.vc.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun4i_dma_interrupt(int irq, void *dev_id)
    {
    struct sun4i_dma_dev *priv = dev_id;
    struct sun4i_dma_pchan *pchans = priv.pchans, *pchan;
    struct sun4i_dma_vchan *vchan;
    struct sun4i_dma_contract *contract;
    struct sun4i_dma_promise *promise;
    unsigned long pendirq, irqs, disableirqs;
    int bit, i, free_room, allow_mitigation = 1;
    pendirq = readl_relaxed(priv.base + SUN4I_DMA_IRQ_PENDING_STATUS_REG);
    handle_pending:
    disableirqs = 0;
    free_room = 0;
    for_each_set_bit(bit, &pendirq, 32) {
    pchan = &pchans[bit >> 1];
    vchan = pchan.vchan;
    if (!vchan) /* a terminated channel may still interrupt */
    continue;
    contract = vchan.contract;
//
// Disable the IRQ and free the pchan if it's an end
// interrupt (odd bit)
//
    if (bit & 1) {
    spin_lock(&vchan.vc.lock);
//
// Move the promise into the completed list now that
// we're done with it
//
    list_move_tail(&vchan.processing.list,
    &contract.completed_demands);
//
// Cyclic DMA transfers are special:
// - There's always something we can dispatch
// - We need to run the callback
// - Latency is very important, as this is used by audio
// We therefore just cycle through the list and dispatch
// whatever we have here, reusing the pchan. There's
// no need to run the thread after this.
//
// For non-cyclic transfers we need to look around,
// so we can program some more work, or notify the
// client that their transfers have been completed.
//
    if (contract.is_cyclic) {
    promise = get_next_cyclic_promise(contract);
    vchan.processing = promise;
    configure_pchan(pchan, promise);
    vchan_cyclic_callback(&contract.vd);
    } else {
    vchan.processing = core::ptr::null_mut();
    vchan.pchan = core::ptr::null_mut();
    free_room = 1;
    disableirqs |= BIT(bit);
    release_pchan(priv, pchan);
    }
    spin_unlock(&vchan.vc.lock);
    } else {
// Half done interrupt
    if (contract.is_cyclic)
    vchan_cyclic_callback(&contract.vd);
    else
    disableirqs |= BIT(bit);
    }
    }
// Disable the IRQs for events we handled
    spin_lock(&priv.lock);
    irqs = readl_relaxed(priv.base + SUN4I_DMA_IRQ_ENABLE_REG);
    writel_relaxed(irqs & ~disableirqs,
    priv.base + SUN4I_DMA_IRQ_ENABLE_REG);
    spin_unlock(&priv.lock);
// Writing 1 to the pending field will clear the pending interrupt
    writel_relaxed(pendirq, priv.base + SUN4I_DMA_IRQ_PENDING_STATUS_REG);
//
// If a pchan was freed, we may be able to schedule something else,
// so have a look around
//
    if (free_room) {
    for (i = 0; i < SUN4I_DMA_NR_MAX_VCHANS; i++) {
    vchan = &priv.vchans[i];
    spin_lock(&vchan.vc.lock);
    __execute_vchan_pending(priv, vchan);
    spin_unlock(&vchan.vc.lock);
    }
    }
//
// Handle newer interrupts if some showed up, but only do it once
// to avoid a too long a loop
//
    if (allow_mitigation) {
    pendirq = readl_relaxed(priv.base +
    SUN4I_DMA_IRQ_PENDING_STATUS_REG);
    if (pendirq) {
    allow_mitigation = 0;
    goto handle_pending;
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_dma_probe(struct platform_device *pdev)
    {
    struct sun4i_dma_dev *priv;
    int i, j, ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.cfg = of_device_get_match_data(&pdev.dev);
    if (!priv.cfg)
    return -ENODEV;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq < 0)
    return priv.irq;
    priv.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.clk),
    "Couldn't start the clock\n");
    if (priv.cfg.has_reset) {
    priv.rst = devm_reset_control_get_exclusive_deasserted(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.rst),
    "Failed to get reset control\n");
    }
    platform_set_drvdata(pdev, priv);
    spin_lock_init(&priv.lock);
    dma_set_max_seg_size(&pdev.dev, SUN4I_DMA_MAX_SEG_SIZE);
    dma_cap_zero(priv.slave.cap_mask);
    dma_cap_set(DMA_PRIVATE, priv.slave.cap_mask);
    dma_cap_set(DMA_MEMCPY, priv.slave.cap_mask);
    dma_cap_set(DMA_CYCLIC, priv.slave.cap_mask);
    dma_cap_set(DMA_SLAVE, priv.slave.cap_mask);
    INIT_LIST_HEAD(&priv.slave.channels);
    priv.slave.device_free_chan_resources	= sun4i_dma_free_chan_resources;
    priv.slave.device_tx_status		= sun4i_dma_tx_status;
    priv.slave.device_issue_pending	= sun4i_dma_issue_pending;
    priv.slave.device_prep_slave_sg	= sun4i_dma_prep_slave_sg;
    priv.slave.device_prep_dma_memcpy	= sun4i_dma_prep_dma_memcpy;
    priv.slave.device_prep_dma_cyclic	= sun4i_dma_prep_dma_cyclic;
    priv.slave.device_config		= sun4i_dma_config;
    priv.slave.device_terminate_all	= sun4i_dma_terminate_all;
    priv.slave.copy_align			= 2;
    priv.slave.src_addr_widths		= BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) |
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) |
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    priv.slave.dst_addr_widths		= BIT(DMA_SLAVE_BUSWIDTH_1_BYTE) |
    BIT(DMA_SLAVE_BUSWIDTH_2_BYTES) |
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES);
    priv.slave.directions			= BIT(DMA_DEV_TO_MEM) |
    BIT(DMA_MEM_TO_DEV);
    priv.slave.residue_granularity		= DMA_RESIDUE_GRANULARITY_BURST;
    priv.slave.dev = &pdev.dev;
    priv.pchans = devm_kcalloc(&pdev.dev, priv.cfg.dma_nr_max_channels,
    sizeof(struct sun4i_dma_pchan), GFP_KERNEL);
    priv.vchans = devm_kcalloc(&pdev.dev, SUN4I_DMA_NR_MAX_VCHANS,
    sizeof(struct sun4i_dma_vchan), GFP_KERNEL);
    priv.pchans_used = devm_kcalloc(&pdev.dev,
    BITS_TO_LONGS(priv.cfg.dma_nr_max_channels),
    sizeof(unsigned long), GFP_KERNEL);
    if (!priv.vchans || !priv.pchans || !priv.pchans_used)
    return -ENOMEM;
//
// [0..priv->cfg->ndma_nr_max_channels) are normal pchans, and
// [priv->cfg->ndma_nr_max_channels..priv->cfg->dma_nr_max_channels) are
// dedicated ones
//
    for (i = 0; i < priv.cfg.ndma_nr_max_channels; i++)
    priv.pchans[i].base = priv.base +
    SUN4I_NDMA_CHANNEL_REG_BASE(i);
    for (j = 0; i < priv.cfg.dma_nr_max_channels; i++, j++) {
    priv.pchans[i].base = priv.base +
    SUN4I_DDMA_CHANNEL_REG_BASE(j);
    priv.pchans[i].is_dedicated = 1;
    }
    for (i = 0; i < SUN4I_DMA_NR_MAX_VCHANS; i++) {
    struct sun4i_dma_vchan *vchan = &priv.vchans[i];
    spin_lock_init(&vchan.vc.lock);
    vchan.vc.desc_free = sun4i_dma_free_contract;
    vchan_init(&vchan.vc, &priv.slave);
    }
//
// Make sure the IRQs are all disabled and accounted for. The bootloader
// likes to leave these dirty
//
    writel(0, priv.base + SUN4I_DMA_IRQ_ENABLE_REG);
    writel(0xFFFFFFFF, priv.base + SUN4I_DMA_IRQ_PENDING_STATUS_REG);
    ret = devm_request_irq(&pdev.dev, priv.irq, sun4i_dma_interrupt,
    0, dev_name(&pdev.dev), priv);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Cannot request IRQ\n");
    ret = dmaenginem_async_device_register(&priv.slave);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register DMA engine device\n");
    ret = of_dma_controller_register(pdev.dev.of_node, sun4i_dma_of_xlate,
    priv);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register translation function\n");
    dev_dbg(&pdev.dev, "Successfully probed SUN4I_DMA\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_dma_remove(pdev: *mut platform_device) {
    static void sun4i_dma_remove(struct platform_device *pdev)
    {
    struct sun4i_dma_dev *priv = platform_get_drvdata(pdev);
// Disable IRQ so no more work is scheduled
    disable_irq(priv.irq);
    of_dma_controller_free(pdev.dev.of_node);
    }
    static struct sun4i_dma_config sun4i_a10_dma_cfg = {
    .ndma_nr_max_channels	= SUN4I_NDMA_NR_MAX_CHANNELS,
    .ndma_nr_max_vchans	= SUN4I_NDMA_NR_MAX_VCHANS,
    .ddma_nr_max_channels	= SUN4I_DDMA_NR_MAX_CHANNELS,
    .ddma_nr_max_vchans	= SUN4I_DDMA_NR_MAX_VCHANS,
    .dma_nr_max_channels	= SUN4I_DMA_NR_MAX_CHANNELS,
    .set_dst_data_width	= set_dst_data_width_a10,
    .set_src_data_width	= set_src_data_width_a10,
    .convert_burst		= convert_burst_a10,
    .ndma_drq_sdram		= SUN4I_NDMA_DRQ_TYPE_SDRAM,
    .ddma_drq_sdram		= SUN4I_DDMA_DRQ_TYPE_SDRAM,
    .max_burst		= SUN4I_MAX_BURST,
    .has_reset		= false,
    };
    static struct sun4i_dma_config suniv_f1c100s_dma_cfg = {
    .ndma_nr_max_channels	= SUNIV_NDMA_NR_MAX_CHANNELS,
    .ndma_nr_max_vchans	= SUNIV_NDMA_NR_MAX_VCHANS,
    .ddma_nr_max_channels	= SUNIV_DDMA_NR_MAX_CHANNELS,
    .ddma_nr_max_vchans	= SUNIV_DDMA_NR_MAX_VCHANS,
    .dma_nr_max_channels	= SUNIV_NDMA_NR_MAX_CHANNELS +
    SUNIV_DDMA_NR_MAX_CHANNELS,
    .set_dst_data_width	= set_dst_data_width_f1c100s,
    .set_src_data_width	= set_src_data_width_f1c100s,
    .convert_burst		= convert_burst_f1c100s,
    .ndma_drq_sdram		= SUNIV_NDMA_DRQ_TYPE_SDRAM,
    .ddma_drq_sdram		= SUNIV_DDMA_DRQ_TYPE_SDRAM,
    .max_burst		= SUNIV_MAX_BURST,
    .has_reset		= true,
    };
    static const struct of_device_id sun4i_dma_match[] = {
    { .compatible = "allwinner,sun4i-a10-dma", .data = &sun4i_a10_dma_cfg },
    { .compatible = "allwinner,suniv-f1c100s-dma",
    .data = &suniv_f1c100s_dma_cfg },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, sun4i_dma_match);
    static struct platform_driver sun4i_dma_driver = {
    .probe	= sun4i_dma_probe,
    .remove = sun4i_dma_remove,
    .driver	= {
    .name		= "sun4i-dma",
    .of_match_table	= sun4i_dma_match,
    },
    };
    module_platform_driver(sun4i_dma_driver);
    MODULE_DESCRIPTION("Allwinner A10 Dedicated DMA Controller Driver");
    MODULE_AUTHOR("Emilio López <emilio@elopez.com.ar>");
    MODULE_LICENSE("GPL");
