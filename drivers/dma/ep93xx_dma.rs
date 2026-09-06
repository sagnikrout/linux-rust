//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ep93xx_dma.c
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
// Driver for the Cirrus Logic EP93xx DMA Controller
//
// Copyright (C) 2011 Mika Westerberg
//
// DMA M2P implementation is based on the original
// arch/arm/mach-ep93xx/dma-m2p.c which has following copyrights:
//
// Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
// Copyright (C) 2006 Applied Data Systems
// Copyright (C) 2009 Ryan Mallon <rmallon@gmail.com>
//
// This driver is based on dw_dmac and amba-pl08x drivers.
//

// M2P registers
pub const M2P_CONTROL: c_uint = 0x0000;

pub const M2P_INTERRUPT: c_uint = 0x0004;

pub const M2P_PPALLOC: c_uint = 0x0008;
pub const M2P_STATUS: c_uint = 0x000c;
pub const M2P_MAXCNT0: c_uint = 0x0020;
pub const M2P_BASE0: c_uint = 0x0024;
pub const M2P_MAXCNT1: c_uint = 0x0030;
pub const M2P_BASE1: c_uint = 0x0034;
pub const M2P_STATE_IDLE: c_int = 0;
pub const M2P_STATE_STALL: c_int = 1;
pub const M2P_STATE_ON: c_int = 2;
pub const M2P_STATE_NEXT: c_int = 3;
// M2M registers
pub const M2M_CONTROL: c_uint = 0x0000;

pub const M2M_CONTROL_PW_SHIFT: c_int = 9;

pub const M2M_CONTROL_TM_SHIFT: c_int = 13;

pub const M2M_CONTROL_RSS_SHIFT: c_int = 22;

pub const M2M_CONTROL_PWSC_SHIFT: c_int = 25;
pub const M2M_INTERRUPT: c_uint = 0x0004;
pub const M2M_INTERRUPT_MASK: c_int = 6;
pub const M2M_STATUS: c_uint = 0x000c;
pub const M2M_STATUS_CTL_SHIFT: c_int = 1;

pub const M2M_STATUS_BUF_SHIFT: c_int = 4;

pub const M2M_BCR0: c_uint = 0x0010;
pub const M2M_BCR1: c_uint = 0x0014;
pub const M2M_SAR_BASE0: c_uint = 0x0018;
pub const M2M_SAR_BASE1: c_uint = 0x001c;
pub const M2M_DAR_BASE0: c_uint = 0x002c;
pub const M2M_DAR_BASE1: c_uint = 0x0030;
pub const DMA_MAX_CHAN_BYTES: c_uint = 0xffff;
pub const DMA_MAX_CHAN_DESCRIPTORS: c_int = 32;
//
// M2P channels.
//
// Note that these values are also directly used for setting the PPALLOC
// register.
//
pub const EP93XX_DMA_I2S1: c_int = 0;
pub const EP93XX_DMA_I2S2: c_int = 1;
pub const EP93XX_DMA_AAC1: c_int = 2;
pub const EP93XX_DMA_AAC2: c_int = 3;
pub const EP93XX_DMA_AAC3: c_int = 4;
pub const EP93XX_DMA_I2S3: c_int = 5;
pub const EP93XX_DMA_UART1: c_int = 6;
pub const EP93XX_DMA_UART2: c_int = 7;
pub const EP93XX_DMA_UART3: c_int = 8;
pub const EP93XX_DMA_IRDA: c_int = 9;
// M2M channels
pub const EP93XX_DMA_SSP: c_int = 10;
pub const EP93XX_DMA_IDE: c_int = 11;
    enum ep93xx_dma_type {
    M2P_DMA,
    M2M_DMA,
    };
    struct ep93xx_dma_engine;
    static int ep93xx_dma_slave_config_write(struct dma_chan *chan,
    enum dma_transfer_direction dir,
    struct dma_slave_config *config);
//
// struct ep93xx_dma_desc - EP93xx specific transaction descriptor
// @src_addr: source address of the transaction
// @dst_addr: destination address of the transaction
// @size: size of the transaction (in bytes)
// @complete: this descriptor is completed
// @txd: dmaengine API descriptor
// @tx_list: list of linked descriptors
// @node: link used for putting this into a channel queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_dma_desc {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub size: usize,
    pub complete: bool,
    pub txd: dma_async_tx_descriptor,
    pub tx_list: list_head,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_dma_chan_cfg {
    pub port: u8,
    pub dir: enum dma_transfer_direction,
}

//
// struct ep93xx_dma_chan - an EP93xx DMA M2P/M2M channel
// @chan: dmaengine API channel
// @edma: pointer to the engine device
// @regs: memory mapped registers
// @dma_cfg: channel number, direction
// @irq: interrupt number of the channel
// @clk: clock used by this channel
// @tasklet: channel specific tasklet used for callbacks
// @lock: lock protecting the fields following
// @flags: flags for the channel
// @buffer: which buffer to use next (0/1)
// @active: flattened chain of descriptors currently being processed
// @queue: pending descriptors which are handled next
// @free_list: list of free descriptors which can be used
// @runtime_addr: physical address currently used as dest/src (M2M only). This
// is set via .device_config before slave operation is
// prepared
// @runtime_ctrl: M2M runtime values for the control register.
// @slave_config: slave configuration
//
// As EP93xx DMA controller doesn't support real chained DMA descriptors we
// will have slightly different scheme here: @active points to a head of
// flattened DMA descriptor chain.
//
// @queue holds pending transactions. These are linked through the first
// descriptor in the chain. When a descriptor is moved to the @active queue,
// the first and chained descriptors are flattened into a single list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_dma_chan {
    pub chan: dma_chan,
    pub edma: *const ep93xx_dma_engine,
    pub regs: *mut void __iomem,
    pub dma_cfg: ep93xx_dma_chan_cfg,
    pub irq: c_int,
    pub clk: *mut clk,
    pub tasklet: tasklet_struct,
// protects the fields following
    pub lock: spinlock_t,
    pub flags: c_ulong,
// Channel is configured for cyclic transfers
pub const EP93XX_DMA_IS_CYCLIC: c_int = 0;
    pub buffer: c_int,
    pub active: list_head,
    pub queue: list_head,
    pub free_list: list_head,
    pub runtime_addr: u32,
    pub runtime_ctrl: u32,
    pub slave_config: dma_slave_config,
}

//
// struct ep93xx_dma_engine - the EP93xx DMA engine instance
// @dma_dev: holds the dmaengine device
// @m2m: is this an M2M or M2P device
// @hw_setup: method which sets the channel up for operation
// @hw_synchronize: synchronizes DMA channel termination to current context
// @hw_shutdown: shuts the channel down and flushes whatever is left
// @hw_submit: pushes active descriptor(s) to the hardware
// @hw_interrupt: handle the interrupt
// @num_channels: number of channels for this instance
// @channels: array of channels
//
// There is one instance of this struct for the M2P channels and one for the
// M2M channels. hw_xxx() methods are used to perform operations which are
// different on M2M and M2P channels. These methods are called with channel
// lock held and interrupts disabled so they cannot sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_dma_engine {
    pub dma_dev: dma_device,
    pub m2m: bool,
    pub ): *mut *mut int (hw_setup)(struct ep93xx_dma_chan,
    pub ): *mut *mut void (hw_synchronize)(struct ep93xx_dma_chan,
    pub ): *mut *mut void (hw_shutdown)(struct ep93xx_dma_chan,
    pub ): *mut *mut void (hw_submit)(struct ep93xx_dma_chan,
    pub ): *mut *mut int (hw_interrupt)(struct ep93xx_dma_chan,
pub const INTERRUPT_UNKNOWN: c_int = 0;
pub const INTERRUPT_DONE: c_int = 1;
pub const INTERRUPT_NEXT_BUFFER: c_int = 2;
    pub num_channels: usize,
    pub __counted_by(num_channels): ep93xx_dma_chan channels[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_edma_data {
    pub id: u32,
    pub num_channels: usize,
}

    static inline struct device *chan2dev(struct ep93xx_dma_chan *edmac)
    {
    return &edmac.chan.dev.device;
    }
    static struct ep93xx_dma_chan *to_ep93xx_dma_chan(struct dma_chan *chan)
    {
    return container_of(chan, struct ep93xx_dma_chan, chan);
    }
#[no_mangle]
pub unsafe extern "C" fn ep93xx_dma_chan_is_m2p(chan: *mut dma_chan) -> bool {
    static inline bool ep93xx_dma_chan_is_m2p(struct dma_chan *chan)
    {
    if (device_is_compatible(chan.device.dev, "cirrus,ep9301-dma-m2p"))
    return true;
    return !strcmp(dev_name(chan.device.dev), "ep93xx-dma-m2p");
    }
//
// ep93xx_dma_chan_direction - returns direction the channel can be used
//
// This function can be used in filter functions to find out whether the
// channel supports given DMA direction. Only M2P channels have such
// limitation, for M2M channels the direction is configurable.
//
    static inline enum dma_transfer_direction
    ep93xx_dma_chan_direction(struct dma_chan *chan)
    {
    if (!ep93xx_dma_chan_is_m2p(chan))
    return DMA_TRANS_NONE;
// even channels are for TX, odd for RX
    return (chan.chan_id % 2 == 0) ? DMA_MEM_TO_DEV : DMA_DEV_TO_MEM;
    }
//
// ep93xx_dma_set_active - set new active descriptor chain
// @edmac: channel
// @desc: head of the new active descriptor chain
//
// Sets @desc to be the head of the new active descriptor chain. This is the
// chain which is processed next. The active list must be empty before calling
// this function.
//
// Called with @edmac->lock held and interrupts disabled.
//
    static void ep93xx_dma_set_active(struct ep93xx_dma_chan *edmac,
    struct ep93xx_dma_desc *desc)
    {
    BUG_ON(!list_empty(&edmac.active));
    list_add_tail(&desc.node, &edmac.active);
// Flatten the @desc->tx_list chain into @edmac->active list
    while (!list_empty(&desc.tx_list)) {
    struct ep93xx_dma_desc *d = list_first_entry(&desc.tx_list,
    struct ep93xx_dma_desc, node);
//
// We copy the callback parameters from the first descriptor
// to all the chained descriptors. This way we can call the
// callback without having to find out the first descriptor in
// the chain. Useful for cyclic transfers.
//
    d.txd.callback = desc.txd.callback;
    d.txd.callback_param = desc.txd.callback_param;
    list_move_tail(&d.node, &edmac.active);
    }
    }
// Called with @edmac->lock held and interrupts disabled
    static struct ep93xx_dma_desc *
    ep93xx_dma_get_active(struct ep93xx_dma_chan *edmac)
    {
    return list_first_entry_or_null(&edmac.active,
    struct ep93xx_dma_desc, node);
    }
//
// ep93xx_dma_advance_active - advances to the next active descriptor
// @edmac: channel
//
// Function advances active descriptor to the next in the @edmac->active and
// returns %true if we still have descriptors in the chain to process.
// Otherwise returns %false.
//
// When the channel is in cyclic mode always returns %true.
//
// Called with @edmac->lock held and interrupts disabled.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_advance_active(edmac: *mut ep93xx_dma_chan) -> bool {
    static bool ep93xx_dma_advance_active(struct ep93xx_dma_chan *edmac)
    {
    struct ep93xx_dma_desc *desc;
    list_rotate_left(&edmac.active);
    if (test_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags))
    return true;
    desc = ep93xx_dma_get_active(edmac);
    if (!desc)
    return false;
//
// If txd.cookie is set it means that we are back in the first
// descriptor in the chain and hence done with it.
//
    return !desc.txd.cookie;
    }
//
// M2P DMA implementation
//
#[no_mangle]
unsafe extern "C" fn m2p_set_control(edmac: *mut ep93xx_dma_chan, control: u32) {
    static void m2p_set_control(struct ep93xx_dma_chan *edmac, u32 control)
    {
    writel(control, edmac.regs + M2P_CONTROL);
//
// EP93xx User's Guide states that we must perform a dummy read after
// write to the control register.
//
    readl(edmac.regs + M2P_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn m2p_hw_setup(edmac: *mut ep93xx_dma_chan) -> c_int {
    static int m2p_hw_setup(struct ep93xx_dma_chan *edmac)
    {
    u32 control;
    writel(edmac.dma_cfg.port & 0xf, edmac.regs + M2P_PPALLOC);
    control = M2P_CONTROL_CH_ERROR_INT | M2P_CONTROL_ICE
    | M2P_CONTROL_ENABLE;
    m2p_set_control(edmac, control);
    edmac.buffer = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn m2p_channel_state(edmac: *mut ep93xx_dma_chan) -> u32 {
    static inline u32 m2p_channel_state(struct ep93xx_dma_chan *edmac)
    {
    return (readl(edmac.regs + M2P_STATUS) >> 4) & 0x3;
    }
#[no_mangle]
unsafe extern "C" fn m2p_hw_synchronize(edmac: *mut ep93xx_dma_chan) {
    static void m2p_hw_synchronize(struct ep93xx_dma_chan *edmac)
    {
    unsigned long flags;
    u32 control;
    spin_lock_irqsave(&edmac.lock, flags);
    control = readl(edmac.regs + M2P_CONTROL);
    control &= ~(M2P_CONTROL_STALLINT | M2P_CONTROL_NFBINT);
    m2p_set_control(edmac, control);
    spin_unlock_irqrestore(&edmac.lock, flags);
    while (m2p_channel_state(edmac) >= M2P_STATE_ON)
    schedule();
    }
#[no_mangle]
unsafe extern "C" fn m2p_hw_shutdown(edmac: *mut ep93xx_dma_chan) {
    static void m2p_hw_shutdown(struct ep93xx_dma_chan *edmac)
    {
    m2p_set_control(edmac, 0);
    while (m2p_channel_state(edmac) != M2P_STATE_IDLE)
    dev_warn(chan2dev(edmac), "M2P: Not yet IDLE\n");
    }
#[no_mangle]
unsafe extern "C" fn m2p_fill_desc(edmac: *mut ep93xx_dma_chan) {
    static void m2p_fill_desc(struct ep93xx_dma_chan *edmac)
    {
    struct ep93xx_dma_desc *desc;
    u32 bus_addr;
    desc = ep93xx_dma_get_active(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac), "M2P: empty descriptor list\n");
    return;
    }
    if (ep93xx_dma_chan_direction(&edmac.chan) == DMA_MEM_TO_DEV)
    bus_addr = desc.src_addr;
    else
    bus_addr = desc.dst_addr;
    if (edmac.buffer == 0) {
    writel(desc.size, edmac.regs + M2P_MAXCNT0);
    writel(bus_addr, edmac.regs + M2P_BASE0);
    } else {
    writel(desc.size, edmac.regs + M2P_MAXCNT1);
    writel(bus_addr, edmac.regs + M2P_BASE1);
    }
    edmac.buffer ^= 1;
    }
#[no_mangle]
unsafe extern "C" fn m2p_hw_submit(edmac: *mut ep93xx_dma_chan) {
    static void m2p_hw_submit(struct ep93xx_dma_chan *edmac)
    {
    let mut control: u32 = readl(edmac.regs + M2P_CONTROL);
    m2p_fill_desc(edmac);
    control |= M2P_CONTROL_STALLINT;
    if (ep93xx_dma_advance_active(edmac)) {
    m2p_fill_desc(edmac);
    control |= M2P_CONTROL_NFBINT;
    }
    m2p_set_control(edmac, control);
    }
#[no_mangle]
unsafe extern "C" fn m2p_hw_interrupt(edmac: *mut ep93xx_dma_chan) -> c_int {
    static int m2p_hw_interrupt(struct ep93xx_dma_chan *edmac)
    {
    let mut irq_status: u32 = readl(edmac.regs + M2P_INTERRUPT);
    u32 control;
    if (irq_status & M2P_INTERRUPT_ERROR) {
    struct ep93xx_dma_desc *desc = ep93xx_dma_get_active(edmac);
// Clear the error interrupt
    writel(1, edmac.regs + M2P_INTERRUPT);
//
// It seems that there is no easy way of reporting errors back
// to client so we just report the error here and continue as
// usual.
//
// Revisit this when there is a mechanism to report back the
// errors.
//
    dev_err(chan2dev(edmac),
    "DMA transfer failed! Details:\n"
    "\tcookie	: %d\n"
    "\tsrc_addr	: 0x%08x\n"
    "\tdst_addr	: 0x%08x\n"
    "\tsize		: %zu\n",
    desc.txd.cookie, desc.src_addr, desc.dst_addr,
    desc.size);
    }
//
// Even latest E2 silicon revision sometimes assert STALL interrupt
// instead of NFB. Therefore we treat them equally, basing on the
// amount of data we still have to transfer.
//
    if (!(irq_status & (M2P_INTERRUPT_STALL | M2P_INTERRUPT_NFB)))
    return INTERRUPT_UNKNOWN;
    if (ep93xx_dma_advance_active(edmac)) {
    m2p_fill_desc(edmac);
    return INTERRUPT_NEXT_BUFFER;
    }
// Disable interrupts
    control = readl(edmac.regs + M2P_CONTROL);
    control &= ~(M2P_CONTROL_STALLINT | M2P_CONTROL_NFBINT);
    m2p_set_control(edmac, control);
    return INTERRUPT_DONE;
    }
//
// M2M DMA implementation
//
#[no_mangle]
unsafe extern "C" fn m2m_hw_setup(edmac: *mut ep93xx_dma_chan) -> c_int {
    static int m2m_hw_setup(struct ep93xx_dma_chan *edmac)
    {
    let mut control: u32 = 0;
    if (edmac.dma_cfg.dir == DMA_MEM_TO_MEM) {
// This is memcpy channel, nothing to configure
    writel(control, edmac.regs + M2M_CONTROL);
    return 0;
    }
    switch (edmac.dma_cfg.port) {
    case EP93XX_DMA_SSP:
//
// This was found via experimenting - anything less than 5
// causes the channel to perform only a partial transfer which
// leads to problems since we don't get DONE interrupt then.
//
    control = (5 << M2M_CONTROL_PWSC_SHIFT);
    control |= M2M_CONTROL_NO_HDSK;
    if (edmac.dma_cfg.dir == DMA_MEM_TO_DEV) {
    control |= M2M_CONTROL_DAH;
    control |= M2M_CONTROL_TM_TX;
    control |= M2M_CONTROL_RSS_SSPTX;
    } else {
    control |= M2M_CONTROL_SAH;
    control |= M2M_CONTROL_TM_RX;
    control |= M2M_CONTROL_RSS_SSPRX;
    }
    break;
    case EP93XX_DMA_IDE:
//
// This IDE part is totally untested. Values below are taken
// from the EP93xx Users's Guide and might not be correct.
//
    if (edmac.dma_cfg.dir == DMA_MEM_TO_DEV) {
// Worst case from the UG
    control = (3 << M2M_CONTROL_PWSC_SHIFT);
    control |= M2M_CONTROL_DAH;
    control |= M2M_CONTROL_TM_TX;
    } else {
    control = (2 << M2M_CONTROL_PWSC_SHIFT);
    control |= M2M_CONTROL_SAH;
    control |= M2M_CONTROL_TM_RX;
    }
    control |= M2M_CONTROL_NO_HDSK;
    control |= M2M_CONTROL_RSS_IDE;
    control |= M2M_CONTROL_PW_16;
    break;
    default:
    return -EINVAL;
    }
    writel(control, edmac.regs + M2M_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m2m_hw_shutdown(edmac: *mut ep93xx_dma_chan) {
    static void m2m_hw_shutdown(struct ep93xx_dma_chan *edmac)
    {
// Just disable the channel
    writel(0, edmac.regs + M2M_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn m2m_fill_desc(edmac: *mut ep93xx_dma_chan) {
    static void m2m_fill_desc(struct ep93xx_dma_chan *edmac)
    {
    struct ep93xx_dma_desc *desc;
    desc = ep93xx_dma_get_active(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac), "M2M: empty descriptor list\n");
    return;
    }
    if (edmac.buffer == 0) {
    writel(desc.src_addr, edmac.regs + M2M_SAR_BASE0);
    writel(desc.dst_addr, edmac.regs + M2M_DAR_BASE0);
    writel(desc.size, edmac.regs + M2M_BCR0);
    } else {
    writel(desc.src_addr, edmac.regs + M2M_SAR_BASE1);
    writel(desc.dst_addr, edmac.regs + M2M_DAR_BASE1);
    writel(desc.size, edmac.regs + M2M_BCR1);
    }
    edmac.buffer ^= 1;
    }
#[no_mangle]
unsafe extern "C" fn m2m_hw_submit(edmac: *mut ep93xx_dma_chan) {
    static void m2m_hw_submit(struct ep93xx_dma_chan *edmac)
    {
    let mut control: u32 = readl(edmac.regs + M2M_CONTROL);
//
// Since we allow clients to configure PW (peripheral width) we always
// clear PW bits here and then set them according what is given in
// the runtime configuration.
//
    control &= ~M2M_CONTROL_PW_MASK;
    control |= edmac.runtime_ctrl;
    m2m_fill_desc(edmac);
    control |= M2M_CONTROL_DONEINT;
    if (ep93xx_dma_advance_active(edmac)) {
    m2m_fill_desc(edmac);
    control |= M2M_CONTROL_NFBINT;
    }
//
// Now we can finally enable the channel. For M2M channel this must be
// done _after_ the BCRx registers are programmed.
//
    control |= M2M_CONTROL_ENABLE;
    writel(control, edmac.regs + M2M_CONTROL);
    if (edmac.dma_cfg.dir == DMA_MEM_TO_MEM) {
//
// For memcpy channels the software trigger must be asserted
// in order to start the memcpy operation.
//
    control |= M2M_CONTROL_START;
    writel(control, edmac.regs + M2M_CONTROL);
    }
    }
//
// According to EP93xx User's Guide, we should receive DONE interrupt when all
// M2M DMA controller transactions complete normally. This is not always the
// case - sometimes EP93xx M2M DMA asserts DONE interrupt when the DMA channel
// is still running (channel Buffer FSM in DMA_BUF_ON state, and channel
// Control FSM in DMA_MEM_RD state, observed at least in IDE-DMA operation).
// In effect, disabling the channel when only DONE bit is set could stop
// currently running DMA transfer. To avoid this, we use Buffer FSM and
// Control FSM to check current state of DMA channel.
//
#[no_mangle]
unsafe extern "C" fn m2m_hw_interrupt(edmac: *mut ep93xx_dma_chan) -> c_int {
    static int m2m_hw_interrupt(struct ep93xx_dma_chan *edmac)
    {
    let mut status: u32 = readl(edmac.regs + M2M_STATUS);
    let mut ctl_fsm: u32 = status & M2M_STATUS_CTL_MASK;
    let mut buf_fsm: u32 = status & M2M_STATUS_BUF_MASK;
    let mut done: bool = status & M2M_STATUS_DONE;
    bool last_done;
    u32 control;
    struct ep93xx_dma_desc *desc;
// Accept only DONE and NFB interrupts
    if (!(readl(edmac.regs + M2M_INTERRUPT) & M2M_INTERRUPT_MASK))
    return INTERRUPT_UNKNOWN;
    if (done) {
// Clear the DONE bit
    writel(0, edmac.regs + M2M_INTERRUPT);
    }
//
// Check whether we are done with descriptors or not. This, together
// with DMA channel state, determines action to take in interrupt.
//
    desc = ep93xx_dma_get_active(edmac);
    last_done = !desc || desc.txd.cookie;
//
// Use M2M DMA Buffer FSM and Control FSM to check current state of
// DMA channel. Using DONE and NFB bits from channel status register
// or bits from channel interrupt register is not reliable.
//
    if (!last_done &&
    (buf_fsm == M2M_STATUS_BUF_NO ||
    buf_fsm == M2M_STATUS_BUF_ON)) {
//
// Two buffers are ready for update when Buffer FSM is in
// DMA_NO_BUF state. Only one buffer can be prepared without
// disabling the channel or polling the DONE bit.
// To simplify things, always prepare only one buffer.
//
    if (ep93xx_dma_advance_active(edmac)) {
    m2m_fill_desc(edmac);
    if (done && edmac.dma_cfg.dir == DMA_MEM_TO_MEM) {
// Software trigger for memcpy channel
    control = readl(edmac.regs + M2M_CONTROL);
    control |= M2M_CONTROL_START;
    writel(control, edmac.regs + M2M_CONTROL);
    }
    return INTERRUPT_NEXT_BUFFER;
    } else {
    last_done = true;
    }
    }
//
// Disable the channel only when Buffer FSM is in DMA_NO_BUF state
// and Control FSM is in DMA_STALL state.
//
    if (last_done &&
    buf_fsm == M2M_STATUS_BUF_NO &&
    ctl_fsm == M2M_STATUS_CTL_STALL) {
// Disable interrupts and the channel
    control = readl(edmac.regs + M2M_CONTROL);
    control &= ~(M2M_CONTROL_DONEINT | M2M_CONTROL_NFBINT
    | M2M_CONTROL_ENABLE);
    writel(control, edmac.regs + M2M_CONTROL);
    return INTERRUPT_DONE;
    }
//
// Nothing to do this time.
//
    return INTERRUPT_NEXT_BUFFER;
    }
//
// DMA engine API implementation
//
    static struct ep93xx_dma_desc *
    ep93xx_dma_desc_get(struct ep93xx_dma_chan *edmac)
    {
    struct ep93xx_dma_desc *desc, *_desc;
    struct ep93xx_dma_desc *ret = core::ptr::null_mut();
    unsigned long flags;
    spin_lock_irqsave(&edmac.lock, flags);
    list_for_each_entry_safe(desc, _desc, &edmac.free_list, node) {
    if (async_tx_test_ack(&desc.txd)) {
    list_del_init(&desc.node);
// Re-initialize the descriptor
    desc.src_addr = 0;
    desc.dst_addr = 0;
    desc.size = 0;
    desc.complete = false;
    desc.txd.cookie = 0;
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_param = core::ptr::null_mut();
    ret = desc;
    break;
    }
    }
    spin_unlock_irqrestore(&edmac.lock, flags);
    return ret;
    }
    static void ep93xx_dma_desc_put(struct ep93xx_dma_chan *edmac,
    struct ep93xx_dma_desc *desc)
    {
    if (desc) {
    unsigned long flags;
    spin_lock_irqsave(&edmac.lock, flags);
    list_splice_init(&desc.tx_list, &edmac.free_list);
    list_add(&desc.node, &edmac.free_list);
    spin_unlock_irqrestore(&edmac.lock, flags);
    }
    }
//
// ep93xx_dma_advance_work - start processing the next pending transaction
// @edmac: channel
//
// If we have pending transactions queued and we are currently idling, this
// function takes the next queued transaction from the @edmac->queue and
// pushes it to the hardware for execution.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_advance_work(edmac: *mut ep93xx_dma_chan) {
    static void ep93xx_dma_advance_work(struct ep93xx_dma_chan *edmac)
    {
    struct ep93xx_dma_desc *new;
    unsigned long flags;
    spin_lock_irqsave(&edmac.lock, flags);
    if (!list_empty(&edmac.active) || list_empty(&edmac.queue)) {
    spin_unlock_irqrestore(&edmac.lock, flags);
    return;
    }
// Take the next descriptor from the pending queue
    new = list_first_entry(&edmac.queue, struct ep93xx_dma_desc, node);
    list_del_init(&new.node);
    ep93xx_dma_set_active(edmac, new);
// Push it to the hardware
    edmac.edma.hw_submit(edmac);
    spin_unlock_irqrestore(&edmac.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_tasklet(t: *mut tasklet_struct) {
    static void ep93xx_dma_tasklet(struct tasklet_struct *t)
    {
    struct ep93xx_dma_chan *edmac = from_tasklet(edmac, t, tasklet);
    struct ep93xx_dma_desc *desc, *d;
    struct dmaengine_desc_callback cb;
    LIST_HEAD(list);
    memset(&cb, 0, sizeof(cb));
    spin_lock_irq(&edmac.lock);
//
// If dma_terminate_all() was called before we get to run, the active
// list has become empty. If that happens we aren't supposed to do
// anything more than call ep93xx_dma_advance_work().
//
    desc = ep93xx_dma_get_active(edmac);
    if (desc) {
    if (desc.complete) {
// mark descriptor complete for non cyclic case only
    if (!test_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags))
    dma_cookie_complete(&desc.txd);
    list_splice_init(&edmac.active, &list);
    }
    dmaengine_desc_get_callback(&desc.txd, &cb);
    }
    spin_unlock_irq(&edmac.lock);
// Pick up the next descriptor from the queue
    ep93xx_dma_advance_work(edmac);
// Now we can release all the chained descriptors
    list_for_each_entry_safe(desc, d, &list, node) {
    dma_descriptor_unmap(&desc.txd);
    ep93xx_dma_desc_put(edmac, desc);
    }
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ep93xx_dma_interrupt(int irq, void *dev_id)
    {
    struct ep93xx_dma_chan *edmac = dev_id;
    struct ep93xx_dma_desc *desc;
    let mut ret: irqreturn_t = IRQ_HANDLED;
    spin_lock(&edmac.lock);
    desc = ep93xx_dma_get_active(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac),
    "got interrupt while active list is empty\n");
    spin_unlock(&edmac.lock);
    return IRQ_NONE;
    }
    switch (edmac.edma.hw_interrupt(edmac)) {
    case INTERRUPT_DONE:
    desc.complete = true;
    tasklet_schedule(&edmac.tasklet);
    break;
    case INTERRUPT_NEXT_BUFFER:
    if (test_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags))
    tasklet_schedule(&edmac.tasklet);
    break;
    default:
    dev_warn(chan2dev(edmac), "unknown interrupt!\n");
    ret = IRQ_NONE;
    break;
    }
    spin_unlock(&edmac.lock);
    return ret;
    }
//
// ep93xx_dma_tx_submit - set the prepared descriptor(s) to be executed
// @tx: descriptor to be executed
//
// Function will execute given descriptor on the hardware or if the hardware
// is busy, queue the descriptor to be executed later on. Returns cookie which
// can be used to poll the status of the descriptor.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    static dma_cookie_t ep93xx_dma_tx_submit(struct dma_async_tx_descriptor *tx)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(tx.chan);
    struct ep93xx_dma_desc *desc;
    dma_cookie_t cookie;
    unsigned long flags;
    spin_lock_irqsave(&edmac.lock, flags);
    cookie = dma_cookie_assign(tx);
    desc = container_of(tx, struct ep93xx_dma_desc, txd);
//
// If nothing is currently processed, we push this descriptor
// directly to the hardware. Otherwise we put the descriptor
// to the pending queue.
//
    if (list_empty(&edmac.active)) {
    ep93xx_dma_set_active(edmac, desc);
    edmac.edma.hw_submit(edmac);
    } else {
    list_add_tail(&desc.node, &edmac.queue);
    }
    spin_unlock_irqrestore(&edmac.lock, flags);
    return cookie;
    }
//
// ep93xx_dma_alloc_chan_resources - allocate resources for the channel
// @chan: channel to allocate resources
//
// Function allocates necessary resources for the given DMA channel and
// returns number of allocated descriptors for the channel. Negative errno
// is returned in case of failure.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int ep93xx_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    const char *name = dma_chan_name(chan);
    int ret, i;
// Sanity check the channel parameters
    if (!edmac.edma.m2m) {
    if (edmac.dma_cfg.port > EP93XX_DMA_IRDA)
    return -EINVAL;
    if (edmac.dma_cfg.dir != ep93xx_dma_chan_direction(chan))
    return -EINVAL;
    } else {
    if (edmac.dma_cfg.dir != DMA_MEM_TO_MEM) {
    switch (edmac.dma_cfg.port) {
    case EP93XX_DMA_SSP:
    case EP93XX_DMA_IDE:
    if (!is_slave_direction(edmac.dma_cfg.dir))
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    }
    }
    ret = clk_prepare_enable(edmac.clk);
    if (ret)
    return ret;
    ret = request_irq(edmac.irq, ep93xx_dma_interrupt, 0, name, edmac);
    if (ret)
    goto fail_clk_disable;
    spin_lock_irq(&edmac.lock);
    dma_cookie_init(&edmac.chan);
    ret = edmac.edma.hw_setup(edmac);
    spin_unlock_irq(&edmac.lock);
    if (ret)
    goto fail_free_irq;
    for (i = 0; i < DMA_MAX_CHAN_DESCRIPTORS; i++) {
    struct ep93xx_dma_desc *desc;
    desc = kzalloc_obj(*desc);
    if (!desc) {
    dev_warn(chan2dev(edmac), "not enough descriptors\n");
    break;
    }
    INIT_LIST_HEAD(&desc.tx_list);
    dma_async_tx_descriptor_init(&desc.txd, chan);
    desc.txd.flags = DMA_CTRL_ACK;
    desc.txd.tx_submit = ep93xx_dma_tx_submit;
    ep93xx_dma_desc_put(edmac, desc);
    }
    return i;
    fail_free_irq:
    free_irq(edmac.irq, edmac);
    fail_clk_disable:
    clk_disable_unprepare(edmac.clk);
    return ret;
    }
//
// ep93xx_dma_free_chan_resources - release resources for the channel
// @chan: channel
//
// Function releases all the resources allocated for the given channel.
// The channel must be idle when this is called.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_free_chan_resources(chan: *mut dma_chan) {
    static void ep93xx_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_desc *desc, *d;
    unsigned long flags;
    LIST_HEAD(list);
    BUG_ON(!list_empty(&edmac.active));
    BUG_ON(!list_empty(&edmac.queue));
    spin_lock_irqsave(&edmac.lock, flags);
    edmac.edma.hw_shutdown(edmac);
    edmac.runtime_addr = 0;
    edmac.runtime_ctrl = 0;
    edmac.buffer = 0;
    list_splice_init(&edmac.free_list, &list);
    spin_unlock_irqrestore(&edmac.lock, flags);
    list_for_each_entry_safe(desc, d, &list, node)
    kfree(desc);
    clk_disable_unprepare(edmac.clk);
    free_irq(edmac.irq, edmac);
    }
//
// ep93xx_dma_prep_dma_memcpy - prepare a memcpy DMA operation
// @chan: channel
// @dest: destination bus address
// @src: source bus address
// @len: size of the transaction
// @flags: flags for the descriptor
//
// Returns a valid DMA descriptor or %NULL in case of failure.
//
    static struct dma_async_tx_descriptor *
    ep93xx_dma_prep_dma_memcpy(struct dma_chan *chan, dma_addr_t dest,
    dma_addr_t src, size_t len, unsigned long flags)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_desc *desc, *first;
    size_t bytes, offset;
    first = core::ptr::null_mut();
    for (offset = 0; offset < len; offset += bytes) {
    desc = ep93xx_dma_desc_get(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac), "couldn't get descriptor\n");
    goto fail;
    }
    bytes = min_t(size_t, len - offset, DMA_MAX_CHAN_BYTES);
    desc.src_addr = src + offset;
    desc.dst_addr = dest + offset;
    desc.size = bytes;
    if (!first)
    first = desc;
    else
    list_add_tail(&desc.node, &first.tx_list);
    }
    first.txd.cookie = -EBUSY;
    first.txd.flags = flags;
    return &first.txd;
    fail:
    ep93xx_dma_desc_put(edmac, first);
    return core::ptr::null_mut();
    }
//
// ep93xx_dma_prep_slave_sg - prepare a slave DMA operation
// @chan: channel
// @sgl: list of buffers to transfer
// @sg_len: number of entries in @sgl
// @dir: direction of the DMA transfer
// @flags: flags for the descriptor
// @context: operation context (ignored)
//
// Returns a valid DMA descriptor or %NULL in case of failure.
//
    static struct dma_async_tx_descriptor *
    ep93xx_dma_prep_slave_sg(struct dma_chan *chan, struct scatterlist *sgl,
    unsigned int sg_len, enum dma_transfer_direction dir,
    unsigned long flags, void *context)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_desc *desc, *first;
    struct scatterlist *sg;
    int i;
    if (!edmac.edma.m2m && dir != ep93xx_dma_chan_direction(chan)) {
    dev_warn(chan2dev(edmac),
    "channel was configured with different direction\n");
    return core::ptr::null_mut();
    }
    if (test_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags)) {
    dev_warn(chan2dev(edmac),
    "channel is already used for cyclic transfers\n");
    return core::ptr::null_mut();
    }
    ep93xx_dma_slave_config_write(chan, dir, &edmac.slave_config);
    first = core::ptr::null_mut();
    for_each_sg(sgl, sg, sg_len, i) {
    let mut len: usize = sg_dma_len(sg);
    if (len > DMA_MAX_CHAN_BYTES) {
    dev_warn(chan2dev(edmac), "too big transfer size %zu\n",
    len);
    goto fail;
    }
    desc = ep93xx_dma_desc_get(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac), "couldn't get descriptor\n");
    goto fail;
    }
    if (dir == DMA_MEM_TO_DEV) {
    desc.src_addr = sg_dma_address(sg);
    desc.dst_addr = edmac.runtime_addr;
    } else {
    desc.src_addr = edmac.runtime_addr;
    desc.dst_addr = sg_dma_address(sg);
    }
    desc.size = len;
    if (!first)
    first = desc;
    else
    list_add_tail(&desc.node, &first.tx_list);
    }
    first.txd.cookie = -EBUSY;
    first.txd.flags = flags;
    return &first.txd;
    fail:
    ep93xx_dma_desc_put(edmac, first);
    return core::ptr::null_mut();
    }
//
// ep93xx_dma_prep_dma_cyclic - prepare a cyclic DMA operation
// @chan: channel
// @dma_addr: DMA mapped address of the buffer
// @buf_len: length of the buffer (in bytes)
// @period_len: length of a single period
// @dir: direction of the operation
// @flags: tx descriptor status flags
//
// Prepares a descriptor for cyclic DMA operation. This means that once the
// descriptor is submitted, we will be submitting in a @period_len sized
// buffers and calling callback once the period has been elapsed. Transfer
// terminates only when client calls dmaengine_terminate_all() for this
// channel.
//
// Returns a valid DMA descriptor or %NULL in case of failure.
//
    static struct dma_async_tx_descriptor *
    ep93xx_dma_prep_dma_cyclic(struct dma_chan *chan, dma_addr_t dma_addr,
    size_t buf_len, size_t period_len,
    enum dma_transfer_direction dir, unsigned long flags)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_desc *desc, *first;
    let mut offset: usize = 0;
    if (!edmac.edma.m2m && dir != ep93xx_dma_chan_direction(chan)) {
    dev_warn(chan2dev(edmac),
    "channel was configured with different direction\n");
    return core::ptr::null_mut();
    }
    if (test_and_set_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags)) {
    dev_warn(chan2dev(edmac),
    "channel is already used for cyclic transfers\n");
    return core::ptr::null_mut();
    }
    if (period_len > DMA_MAX_CHAN_BYTES) {
    dev_warn(chan2dev(edmac), "too big period length %zu\n",
    period_len);
    return core::ptr::null_mut();
    }
    ep93xx_dma_slave_config_write(chan, dir, &edmac.slave_config);
// Split the buffer into period size chunks
    first = core::ptr::null_mut();
    for (offset = 0; offset < buf_len; offset += period_len) {
    desc = ep93xx_dma_desc_get(edmac);
    if (!desc) {
    dev_warn(chan2dev(edmac), "couldn't get descriptor\n");
    goto fail;
    }
    if (dir == DMA_MEM_TO_DEV) {
    desc.src_addr = dma_addr + offset;
    desc.dst_addr = edmac.runtime_addr;
    } else {
    desc.src_addr = edmac.runtime_addr;
    desc.dst_addr = dma_addr + offset;
    }
    desc.size = period_len;
    if (!first)
    first = desc;
    else
    list_add_tail(&desc.node, &first.tx_list);
    }
    first.txd.cookie = -EBUSY;
    return &first.txd;
    fail:
    ep93xx_dma_desc_put(edmac, first);
    return core::ptr::null_mut();
    }
//
// ep93xx_dma_synchronize - Synchronizes the termination of transfers to the
// current context.
// @chan: channel
//
// Synchronizes the DMA channel termination to the current context. When this
// function returns it is guaranteed that all transfers for previously issued
// descriptors have stopped and it is safe to free the memory associated
// with them. Furthermore it is guaranteed that all complete callback functions
// for a previously submitted descriptor have finished running and it is safe to
// free resources accessed from within the complete callbacks.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_synchronize(chan: *mut dma_chan) {
    static void ep93xx_dma_synchronize(struct dma_chan *chan)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    if (edmac.edma.hw_synchronize)
    edmac.edma.hw_synchronize(edmac);
    }
//
// ep93xx_dma_terminate_all - terminate all transactions
// @chan: channel
//
// Stops all DMA transactions. All descriptors are put back to the
// @edmac->free_list and callbacks are _not_ called.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int ep93xx_dma_terminate_all(struct dma_chan *chan)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_desc *desc, *_d;
    unsigned long flags;
    LIST_HEAD(list);
    spin_lock_irqsave(&edmac.lock, flags);
// First we disable and flush the DMA channel
    edmac.edma.hw_shutdown(edmac);
    clear_bit(EP93XX_DMA_IS_CYCLIC, &edmac.flags);
    list_splice_init(&edmac.active, &list);
    list_splice_init(&edmac.queue, &list);
//
// We then re-enable the channel. This way we can continue submitting
// the descriptors by just calling ->hw_submit() again.
//
    edmac.edma.hw_setup(edmac);
    spin_unlock_irqrestore(&edmac.lock, flags);
    list_for_each_entry_safe(desc, _d, &list, node)
    ep93xx_dma_desc_put(edmac, desc);
    return 0;
    }
    static int ep93xx_dma_slave_config(struct dma_chan *chan,
    struct dma_slave_config *config)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    memcpy(&edmac.slave_config, config, sizeof(*config));
    return 0;
    }
    static int ep93xx_dma_slave_config_write(struct dma_chan *chan,
    enum dma_transfer_direction dir,
    struct dma_slave_config *config)
    {
    struct ep93xx_dma_chan *edmac = to_ep93xx_dma_chan(chan);
    enum dma_slave_buswidth width;
    unsigned long flags;
    u32 addr, ctrl;
    if (!edmac.edma.m2m)
    return -EINVAL;
    switch (dir) {
    case DMA_DEV_TO_MEM:
    width = config.src_addr_width;
    addr = config.src_addr;
    break;
    case DMA_MEM_TO_DEV:
    width = config.dst_addr_width;
    addr = config.dst_addr;
    break;
    default:
    return -EINVAL;
    }
    switch (width) {
    case DMA_SLAVE_BUSWIDTH_1_BYTE:
    ctrl = 0;
    break;
    case DMA_SLAVE_BUSWIDTH_2_BYTES:
    ctrl = M2M_CONTROL_PW_16;
    break;
    case DMA_SLAVE_BUSWIDTH_4_BYTES:
    ctrl = M2M_CONTROL_PW_32;
    break;
    default:
    return -EINVAL;
    }
    spin_lock_irqsave(&edmac.lock, flags);
    edmac.runtime_addr = addr;
    edmac.runtime_ctrl = ctrl;
    spin_unlock_irqrestore(&edmac.lock, flags);
    return 0;
    }
//
// ep93xx_dma_tx_status - check if a transaction is completed
// @chan: channel
// @cookie: transaction specific cookie
// @state: state of the transaction is stored here if given
//
// This function can be used to query state of a given transaction.
//
    static enum dma_status ep93xx_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie,
    struct dma_tx_state *state)
    {
    return dma_cookie_status(chan, cookie, state);
    }
//
// ep93xx_dma_issue_pending - push pending transactions to the hardware
// @chan: channel
//
// When this function is called, all pending transactions are pushed to the
// hardware and executed.
//
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_issue_pending(chan: *mut dma_chan) {
    static void ep93xx_dma_issue_pending(struct dma_chan *chan)
    {
    ep93xx_dma_advance_work(to_ep93xx_dma_chan(chan));
    }
    static struct ep93xx_dma_engine *ep93xx_dma_of_probe(struct platform_device *pdev)
    {
    const struct ep93xx_edma_data *data;
    struct device *dev = &pdev.dev;
    struct ep93xx_dma_engine *edma;
    struct dma_device *dma_dev;
    char dma_clk_name[5];
    int i;
    data = device_get_match_data(dev);
    if (!data)
    return ERR_PTR(dev_err_probe(dev, -ENODEV, "No device match found\n"));
    edma = devm_kzalloc(dev, struct_size(edma, channels, data.num_channels),
    GFP_KERNEL);
    if (!edma)
    return ERR_PTR(-ENOMEM);
    edma.m2m = data.id;
    edma.num_channels = data.num_channels;
    dma_dev = &edma.dma_dev;
    INIT_LIST_HEAD(&dma_dev.channels);
    for (i = 0; i < edma.num_channels; i++) {
    struct ep93xx_dma_chan *edmac = &edma.channels[i];
    int len;
    edmac.chan.device = dma_dev;
    edmac.regs = devm_platform_ioremap_resource(pdev, i);
    if (IS_ERR(edmac.regs))
    return ERR_CAST(edmac.regs);
    edmac.irq = fwnode_irq_get(dev_fwnode(dev), i);
    if (edmac.irq < 0)
    return ERR_PTR(edmac.irq);
    edmac.edma = edma;
    if (edma.m2m)
    len = snprintf(dma_clk_name, sizeof(dma_clk_name), "m2m%u", i);
    else
    len = snprintf(dma_clk_name, sizeof(dma_clk_name), "m2p%u", i);
    if (len >= sizeof(dma_clk_name))
    return ERR_PTR(-ENOBUFS);
    edmac.clk = devm_clk_get(dev, dma_clk_name);
    if (IS_ERR(edmac.clk)) {
    dev_err_probe(dev, PTR_ERR(edmac.clk),
    "no %s clock found\n", dma_clk_name);
    return ERR_CAST(edmac.clk);
    }
    spin_lock_init(&edmac.lock);
    INIT_LIST_HEAD(&edmac.active);
    INIT_LIST_HEAD(&edmac.queue);
    INIT_LIST_HEAD(&edmac.free_list);
    tasklet_setup(&edmac.tasklet, ep93xx_dma_tasklet);
    list_add_tail(&edmac.chan.device_node,
    &dma_dev.channels);
    }
    return edma;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_m2p_dma_filter(chan: *mut dma_chan, filter_param: *mut c_void) -> bool {
    static bool ep93xx_m2p_dma_filter(struct dma_chan *chan, void *filter_param)
    {
    struct ep93xx_dma_chan *echan = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_chan_cfg *cfg = filter_param;
    if (cfg.dir != ep93xx_dma_chan_direction(chan))
    return false;
    echan.dma_cfg = *cfg;
    return true;
    }
    static struct dma_chan *ep93xx_m2p_dma_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct ep93xx_dma_engine *edma = ofdma.of_dma_data;
    let mut mask: dma_cap_mask_t = edma.dma_dev.cap_mask;
    struct ep93xx_dma_chan_cfg dma_cfg;
    let mut port: u8 = dma_spec.args[0];
    let mut direction: u8 = dma_spec.args[1];
    if (port > EP93XX_DMA_IRDA)
    return core::ptr::null_mut();
    if (!is_slave_direction(direction))
    return core::ptr::null_mut();
    dma_cfg.port = port;
    dma_cfg.dir = direction;
    return __dma_request_channel(&mask, ep93xx_m2p_dma_filter, &dma_cfg, ofdma.of_node);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_m2m_dma_filter(chan: *mut dma_chan, filter_param: *mut c_void) -> bool {
    static bool ep93xx_m2m_dma_filter(struct dma_chan *chan, void *filter_param)
    {
    struct ep93xx_dma_chan *echan = to_ep93xx_dma_chan(chan);
    struct ep93xx_dma_chan_cfg *cfg = filter_param;
    echan.dma_cfg = *cfg;
    return true;
    }
    static struct dma_chan *ep93xx_m2m_dma_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct ep93xx_dma_engine *edma = ofdma.of_dma_data;
    let mut mask: dma_cap_mask_t = edma.dma_dev.cap_mask;
    struct ep93xx_dma_chan_cfg dma_cfg;
    let mut port: u8 = dma_spec.args[0];
    let mut direction: u8 = dma_spec.args[1];
    if (!is_slave_direction(direction))
    return core::ptr::null_mut();
    switch (port) {
    case EP93XX_DMA_SSP:
    case EP93XX_DMA_IDE:
    break;
    default:
    return core::ptr::null_mut();
    }
    dma_cfg.port = port;
    dma_cfg.dir = direction;
    return __dma_request_channel(&mask, ep93xx_m2m_dma_filter, &dma_cfg, ofdma.of_node);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_dma_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xx_dma_probe(struct platform_device *pdev)
    {
    struct ep93xx_dma_engine *edma;
    struct dma_device *dma_dev;
    int ret;
    edma = ep93xx_dma_of_probe(pdev);
    if (IS_ERR(edma))
    return PTR_ERR(edma);
    dma_dev = &edma.dma_dev;
    dma_cap_zero(dma_dev.cap_mask);
    dma_cap_set(DMA_SLAVE, dma_dev.cap_mask);
    dma_cap_set(DMA_CYCLIC, dma_dev.cap_mask);
    dma_dev.dev = &pdev.dev;
    dma_dev.device_alloc_chan_resources = ep93xx_dma_alloc_chan_resources;
    dma_dev.device_free_chan_resources = ep93xx_dma_free_chan_resources;
    dma_dev.device_prep_slave_sg = ep93xx_dma_prep_slave_sg;
    dma_dev.device_prep_dma_cyclic = ep93xx_dma_prep_dma_cyclic;
    dma_dev.device_config = ep93xx_dma_slave_config;
    dma_dev.device_synchronize = ep93xx_dma_synchronize;
    dma_dev.device_terminate_all = ep93xx_dma_terminate_all;
    dma_dev.device_issue_pending = ep93xx_dma_issue_pending;
    dma_dev.device_tx_status = ep93xx_dma_tx_status;
    dma_set_max_seg_size(dma_dev.dev, DMA_MAX_CHAN_BYTES);
    if (edma.m2m) {
    dma_cap_set(DMA_MEMCPY, dma_dev.cap_mask);
    dma_dev.device_prep_dma_memcpy = ep93xx_dma_prep_dma_memcpy;
    edma.hw_setup = m2m_hw_setup;
    edma.hw_shutdown = m2m_hw_shutdown;
    edma.hw_submit = m2m_hw_submit;
    edma.hw_interrupt = m2m_hw_interrupt;
    } else {
    dma_cap_set(DMA_PRIVATE, dma_dev.cap_mask);
    edma.hw_synchronize = m2p_hw_synchronize;
    edma.hw_setup = m2p_hw_setup;
    edma.hw_shutdown = m2p_hw_shutdown;
    edma.hw_submit = m2p_hw_submit;
    edma.hw_interrupt = m2p_hw_interrupt;
    }
    ret = dma_async_device_register(dma_dev);
    if (ret)
    return ret;
    if (edma.m2m) {
    ret = of_dma_controller_register(pdev.dev.of_node, ep93xx_m2m_dma_of_xlate,
    edma);
    } else {
    ret = of_dma_controller_register(pdev.dev.of_node, ep93xx_m2p_dma_of_xlate,
    edma);
    }
    if (ret)
    goto err_dma_unregister;
    dev_info(dma_dev.dev, "EP93xx M2%s DMA ready\n", edma.m2m ? "M" : "P");
    return 0;
    err_dma_unregister:
    dma_async_device_unregister(dma_dev);
    return ret;
    }
    static const struct ep93xx_edma_data edma_m2p = {
    .id = M2P_DMA,
    .num_channels = 10,
    };
    static const struct ep93xx_edma_data edma_m2m = {
    .id = M2M_DMA,
    .num_channels = 2,
    };
    static const struct of_device_id ep93xx_dma_of_ids[] = {
    { .compatible = "cirrus,ep9301-dma-m2p", .data = &edma_m2p },
    { .compatible = "cirrus,ep9301-dma-m2m", .data = &edma_m2m },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ep93xx_dma_of_ids);
    static struct platform_driver ep93xx_dma_driver = {
    .driver		= {
    .name	= "ep93xx-dma",
    .of_match_table = ep93xx_dma_of_ids,
    },
    .probe		= ep93xx_dma_probe,
    };
    module_platform_driver(ep93xx_dma_driver);
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@iki.fi>");
    MODULE_DESCRIPTION("EP93xx DMA driver");
