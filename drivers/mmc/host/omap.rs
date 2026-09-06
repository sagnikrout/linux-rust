//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/omap.c
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
// linux/drivers/mmc/host/omap.c
//
// Copyright (C) 2004 Nokia Corporation
// Written by Tuukka Tikkanen and Juha Yrjölä<juha.yrjola@nokia.com>
// Misc hacks here and there by Tony Lindgren <tony@atomide.com>
// Other hacks (DMA, SD, etc) by David Brownell
//

pub const OMAP_MMC_REG_CMD: c_uint = 0x00;
pub const OMAP_MMC_REG_ARGL: c_uint = 0x01;
pub const OMAP_MMC_REG_ARGH: c_uint = 0x02;
pub const OMAP_MMC_REG_CON: c_uint = 0x03;
pub const OMAP_MMC_REG_STAT: c_uint = 0x04;
pub const OMAP_MMC_REG_IE: c_uint = 0x05;
pub const OMAP_MMC_REG_CTO: c_uint = 0x06;
pub const OMAP_MMC_REG_DTO: c_uint = 0x07;
pub const OMAP_MMC_REG_DATA: c_uint = 0x08;
pub const OMAP_MMC_REG_BLEN: c_uint = 0x09;
pub const OMAP_MMC_REG_NBLK: c_uint = 0x0a;
pub const OMAP_MMC_REG_BUF: c_uint = 0x0b;
pub const OMAP_MMC_REG_SDIO: c_uint = 0x0d;
pub const OMAP_MMC_REG_REV: c_uint = 0x0f;
pub const OMAP_MMC_REG_RSP0: c_uint = 0x10;
pub const OMAP_MMC_REG_RSP1: c_uint = 0x11;
pub const OMAP_MMC_REG_RSP2: c_uint = 0x12;
pub const OMAP_MMC_REG_RSP3: c_uint = 0x13;
pub const OMAP_MMC_REG_RSP4: c_uint = 0x14;
pub const OMAP_MMC_REG_RSP5: c_uint = 0x15;
pub const OMAP_MMC_REG_RSP6: c_uint = 0x16;
pub const OMAP_MMC_REG_RSP7: c_uint = 0x17;
pub const OMAP_MMC_REG_IOSR: c_uint = 0x18;
pub const OMAP_MMC_REG_SYSC: c_uint = 0x19;
pub const OMAP_MMC_REG_SYSS: c_uint = 0x1a;

//
// Command types
//
pub const OMAP_MMC_CMDTYPE_BC: c_int = 0;
pub const OMAP_MMC_CMDTYPE_BCR: c_int = 1;
pub const OMAP_MMC_CMDTYPE_AC: c_int = 2;
pub const OMAP_MMC_CMDTYPE_ADTC: c_int = 3;

// Specifies how often in millisecs to poll for card status changes
// when the cover switch is open
pub const OMAP_MMC_COVER_POLL_DELAY: c_int = 500;
    struct mmc_omap_host;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_omap_slot {
    pub id: c_int,
    pub vdd: c_uint,
    pub saved_con: u16,
    pub bus_mode: u16,
    pub power_mode: u16,
    pub fclk_freq: c_uint,
    pub cover_bh_work: work_struct,
    pub cover_timer: timer_list,
    pub cover_open: unsigned,
    pub mrq: *mut mmc_request,
    pub host: *mut mmc_omap_host,
    pub mmc: *mut mmc_host,
    pub vsd: *mut gpio_desc,
    pub vio: *mut gpio_desc,
    pub cover: *mut gpio_desc,
    pub pdata: *mut omap_mmc_slot_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_omap_host {
    pub initialized: c_int,
    pub mrq: *mut *mut mmc_request,
    pub cmd: *mut *mut mmc_command,
    pub data: *mut *mut mmc_data,
    pub mmc: *mut *mut mmc_host,
    pub dev: *mut *mut device,
    pub /: *mut *mut unsigned char id; / 16xx chips have 2 MMC blocks,
    pub iclk: *mut *mut clk,
    pub fclk: *mut *mut clk,
    pub dma_rx: *mut dma_chan,
    pub dma_rx_burst: u32,
    pub dma_tx: *mut dma_chan,
    pub dma_tx_burst: u32,
    pub virt_base: *mut void __iomem,
    pub phys_base: c_uint,
    pub irq: c_int,
    pub bus_mode: c_uchar,
    pub reg_shift: c_uint,
    pub slot_switch: *mut gpio_desc,
    pub cmd_abort_work: work_struct,
    pub abort:1: unsigned,
    pub cmd_abort_timer: timer_list,
    pub slot_release_work: work_struct,
    pub next_slot: *mut mmc_omap_slot,
    pub send_stop_work: work_struct,
    pub stop_data: *mut mmc_data,
    pub sg_miter: sg_mapping_iter,
    pub sg_len: c_uint,
    pub total_bytes_left: u32,
    pub features: unsigned,
    pub dma_done:1: unsigned brs_received:1,,
    pub dma_in_use:1: unsigned,
    pub dma_lock: spinlock_t,
    pub slots: [*mut mmc_omap_slot; OMAP_MMC_MAX_SLOTS],
    pub current_slot: *mut mmc_omap_slot,
    pub slot_lock: spinlock_t,
    pub slot_wq: wait_queue_head_t,
    pub nr_slots: c_int,
    pub clk_timer: timer_list,
    pub /: *mut *mut spinlock_t clk_lock; / for changing enabled state,
    pub fclk_enabled:1: c_uint,
    pub mmc_omap_wq: *mut workqueue_struct,
    pub pdata: *mut omap_mmc_platform_data,
}

#[no_mangle]
unsafe extern "C" fn mmc_omap_fclk_offdelay(slot: *mut mmc_omap_slot) {
    static void mmc_omap_fclk_offdelay(struct mmc_omap_slot *slot)
    {
    unsigned long tick_ns;
    if (slot != core::ptr::null_mut() && slot.host.fclk_enabled && slot.fclk_freq > 0) {
    tick_ns = DIV_ROUND_UP(NSEC_PER_SEC, slot.fclk_freq);
    ndelay(8 * tick_ns);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_fclk_enable(host: *mut mmc_omap_host, enable: c_uint) {
    static void mmc_omap_fclk_enable(struct mmc_omap_host *host, unsigned int enable)
    {
    unsigned long flags;
    spin_lock_irqsave(&host.clk_lock, flags);
    if (host.fclk_enabled != enable) {
    host.fclk_enabled = enable;
    if (enable)
    clk_enable(host.fclk);
    else
    clk_disable(host.fclk);
    }
    spin_unlock_irqrestore(&host.clk_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_select_slot(slot: *mut mmc_omap_slot, claimed: c_int) {
    static void mmc_omap_select_slot(struct mmc_omap_slot *slot, int claimed)
    {
    struct mmc_omap_host *host = slot.host;
    unsigned long flags;
    if (claimed)
    goto no_claim;
    spin_lock_irqsave(&host.slot_lock, flags);
    while (host.mmc != core::ptr::null_mut()) {
    spin_unlock_irqrestore(&host.slot_lock, flags);
    wait_event(host.slot_wq, host.mmc == core::ptr::null_mut());
    spin_lock_irqsave(&host.slot_lock, flags);
    }
    host.mmc = slot.mmc;
    spin_unlock_irqrestore(&host.slot_lock, flags);
    no_claim:
    timer_delete(&host.clk_timer);
    if (host.current_slot != slot || !claimed)
    mmc_omap_fclk_offdelay(host.current_slot);
    if (host.current_slot != slot) {
    OMAP_MMC_WRITE(host, CON, slot.saved_con & 0xFC00);
    if (host.slot_switch)
//
// With two slots and a simple GPIO switch, setting
// the GPIO to 0 selects slot ID 0, setting it to 1
// selects slot ID 1.
//
    gpiod_set_value(host.slot_switch, slot.id);
    host.current_slot = slot;
    }
    if (claimed) {
    mmc_omap_fclk_enable(host, 1);
// Doing the dummy read here seems to work around some bug
// at least in OMAP24xx silicon where the command would not
// start after writing the CMD register. Sigh.
    OMAP_MMC_READ(host, CON);
    OMAP_MMC_WRITE(host, CON, slot.saved_con);
    } else
    mmc_omap_fclk_enable(host, 0);
    }
    static void mmc_omap_start_request(struct mmc_omap_host *host,
    struct mmc_request *req);
#[no_mangle]
unsafe extern "C" fn mmc_omap_slot_release_work(work: *mut work_struct) {
    static void mmc_omap_slot_release_work(struct work_struct *work)
    {
    struct mmc_omap_host *host = container_of(work, struct mmc_omap_host,
    slot_release_work);
    struct mmc_omap_slot *next_slot = host.next_slot;
    struct mmc_request *rq;
    host.next_slot = core::ptr::null_mut();
    mmc_omap_select_slot(next_slot, 1);
    rq = next_slot.mrq;
    next_slot.mrq = core::ptr::null_mut();
    mmc_omap_start_request(host, rq);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_release_slot(slot: *mut mmc_omap_slot, clk_enabled: c_int) {
    static void mmc_omap_release_slot(struct mmc_omap_slot *slot, int clk_enabled)
    {
    struct mmc_omap_host *host = slot.host;
    unsigned long flags;
    int i;
    BUG_ON(slot == core::ptr::null_mut() || host.mmc == core::ptr::null_mut());
    if (clk_enabled)
// Keeps clock running for at least 8 cycles on valid freq
    mod_timer(&host.clk_timer, jiffies  + HZ/10);
    else {
    timer_delete(&host.clk_timer);
    mmc_omap_fclk_offdelay(slot);
    mmc_omap_fclk_enable(host, 0);
    }
    spin_lock_irqsave(&host.slot_lock, flags);
// Check for any pending requests
    for (i = 0; i < host.nr_slots; i++) {
    struct mmc_omap_slot *new_slot;
    if (host.slots[i] == core::ptr::null_mut() || host.slots[i].mrq == core::ptr::null_mut())
    continue;
    BUG_ON(host.next_slot != core::ptr::null_mut());
    new_slot = host.slots[i];
// The current slot should not have a request in queue
    BUG_ON(new_slot == host.current_slot);
    host.next_slot = new_slot;
    host.mmc = new_slot.mmc;
    spin_unlock_irqrestore(&host.slot_lock, flags);
    queue_work(host.mmc_omap_wq, &host.slot_release_work);
    return;
    }
    host.mmc = core::ptr::null_mut();
    wake_up(&host.slot_wq);
    spin_unlock_irqrestore(&host.slot_lock, flags);
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn mmc_omap_cover_is_open(slot: *mut mmc_omap_slot) -> c_int {
    int mmc_omap_cover_is_open(struct mmc_omap_slot *slot)
    {
// If we have a GPIO then use that
    if (slot.cover)
    return gpiod_get_value(slot.cover);
    if (slot.pdata.get_cover_state)
    return slot.pdata.get_cover_state(mmc_dev(slot.mmc),
    slot.id);
    return 0;
    }
    static ssize_t
    mmc_omap_show_cover_switch(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mmc_host *mmc = container_of(dev, struct mmc_host, class_dev);
    struct mmc_omap_slot *slot = mmc_priv(mmc);
    return sprintf(buf, "%s\n", mmc_omap_cover_is_open(slot) ? "open" :
    "closed");
    }
    static DEVICE_ATTR(cover_switch, 0444, mmc_omap_show_cover_switch, core::ptr::null_mut());
    static ssize_t
    mmc_omap_show_slot_name(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct mmc_host *mmc = container_of(dev, struct mmc_host, class_dev);
    struct mmc_omap_slot *slot = mmc_priv(mmc);
    return sprintf(buf, "%s\n", slot.pdata.name);
    }
    static DEVICE_ATTR(slot_name, 0444, mmc_omap_show_slot_name, core::ptr::null_mut());
    static void
    mmc_omap_start_command(struct mmc_omap_host *host, struct mmc_command *cmd)
    {
    u32 cmdreg;
    u32 resptype;
    u32 cmdtype;
    u16 irq_mask;
    host.cmd = cmd;
    resptype = 0;
    cmdtype = 0;
// Our hardware needs to know exact type
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_NONE:
    break;
    case MMC_RSP_R1:
    case MMC_RSP_R1B:
// resp 1, 1b, 6, 7
    resptype = 1;
    break;
    case MMC_RSP_R2:
    resptype = 2;
    break;
    case MMC_RSP_R3:
    resptype = 3;
    break;
    default:
    dev_err(mmc_dev(host.mmc), "Invalid response type: %04x\n", mmc_resp_type(cmd));
    break;
    }
    if (mmc_cmd_type(cmd) == MMC_CMD_ADTC) {
    cmdtype = OMAP_MMC_CMDTYPE_ADTC;
    } else if (mmc_cmd_type(cmd) == MMC_CMD_BC) {
    cmdtype = OMAP_MMC_CMDTYPE_BC;
    } else if (mmc_cmd_type(cmd) == MMC_CMD_BCR) {
    cmdtype = OMAP_MMC_CMDTYPE_BCR;
    } else {
    cmdtype = OMAP_MMC_CMDTYPE_AC;
    }
    cmdreg = cmd.opcode | (resptype << 8) | (cmdtype << 12);
    if (host.current_slot.bus_mode == MMC_BUSMODE_OPENDRAIN)
    cmdreg |= 1 << 6;
    if (cmd.flags & MMC_RSP_BUSY)
    cmdreg |= 1 << 11;
    if (host.data && !(host.data.flags & MMC_DATA_WRITE))
    cmdreg |= 1 << 15;
    mod_timer(&host.cmd_abort_timer, jiffies + HZ/2);
    OMAP_MMC_WRITE(host, CTO, 200);
    OMAP_MMC_WRITE(host, ARGL, cmd.arg & 0xffff);
    OMAP_MMC_WRITE(host, ARGH, cmd.arg >> 16);
    irq_mask = OMAP_MMC_STAT_A_EMPTY    | OMAP_MMC_STAT_A_FULL    |
    OMAP_MMC_STAT_CMD_CRC    | OMAP_MMC_STAT_CMD_TOUT  |
    OMAP_MMC_STAT_DATA_CRC   | OMAP_MMC_STAT_DATA_TOUT |
    OMAP_MMC_STAT_END_OF_CMD | OMAP_MMC_STAT_CARD_ERR  |
    OMAP_MMC_STAT_END_OF_DATA;
    if (cmd.opcode == MMC_ERASE)
    irq_mask &= ~OMAP_MMC_STAT_DATA_TOUT;
    OMAP_MMC_WRITE(host, IE, irq_mask);
    OMAP_MMC_WRITE(host, CMD, cmdreg);
    }
    static void
    mmc_omap_release_dma(struct mmc_omap_host *host, struct mmc_data *data,
    int abort)
    {
    enum dma_data_direction dma_data_dir;
    struct device *dev = mmc_dev(host.mmc);
    struct dma_chan *c;
    if (data.flags & MMC_DATA_WRITE) {
    dma_data_dir = DMA_TO_DEVICE;
    c = host.dma_tx;
    } else {
    dma_data_dir = DMA_FROM_DEVICE;
    c = host.dma_rx;
    }
    if (c) {
    if (data.error) {
    dmaengine_terminate_all(c);
// Claim nothing transferred on error...
    data.bytes_xfered = 0;
    }
    dev = c.device.dev;
    }
    dma_unmap_sg(dev, data.sg, host.sg_len, dma_data_dir);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_send_stop_work(work: *mut work_struct) {
    static void mmc_omap_send_stop_work(struct work_struct *work)
    {
    struct mmc_omap_host *host = container_of(work, struct mmc_omap_host,
    send_stop_work);
    struct mmc_omap_slot *slot = host.current_slot;
    struct mmc_data *data = host.stop_data;
    unsigned long tick_ns;
    tick_ns = DIV_ROUND_UP(NSEC_PER_SEC, slot.fclk_freq);
    ndelay(8*tick_ns);
    mmc_omap_start_command(host, data.stop);
    }
    static void
    mmc_omap_xfer_done(struct mmc_omap_host *host, struct mmc_data *data)
    {
    if (host.dma_in_use)
    mmc_omap_release_dma(host, data, data.error);
    else
    sg_miter_stop(&host.sg_miter);
    host.data = core::ptr::null_mut();
    host.sg_len = 0;
// NOTE:  MMC layer will sometimes poll-wait CMD13 next, issuing
// dozens of requests until the card finishes writing data.
// It'd be cheaper to just wait till an EOFB interrupt arrives...
//
    if (!data.stop) {
    struct mmc_host *mmc;
    host.mrq = core::ptr::null_mut();
    mmc = host.mmc;
    mmc_omap_release_slot(host.current_slot, 1);
    mmc_request_done(mmc, data.mrq);
    return;
    }
    host.stop_data = data;
    queue_work(host.mmc_omap_wq, &host.send_stop_work);
    }
    static void
    mmc_omap_send_abort(struct mmc_omap_host *host, int maxloops)
    {
    struct mmc_omap_slot *slot = host.current_slot;
    unsigned int restarts, passes, timeout;
    let mut stat: u16 = 0;
// Sending abort takes 80 clocks. Have some extra and round up
    timeout = DIV_ROUND_UP(120 * USEC_PER_SEC, slot.fclk_freq);
    restarts = 0;
    while (restarts < maxloops) {
    OMAP_MMC_WRITE(host, STAT, 0xFFFF);
    OMAP_MMC_WRITE(host, CMD, (3 << 12) | (1 << 7));
    passes = 0;
    while (passes < timeout) {
    stat = OMAP_MMC_READ(host, STAT);
    if (stat & OMAP_MMC_STAT_END_OF_CMD)
    goto out;
    udelay(1);
    passes++;
    }
    restarts++;
    }
    out:
    OMAP_MMC_WRITE(host, STAT, stat);
    }
    static void
    mmc_omap_abort_xfer(struct mmc_omap_host *host, struct mmc_data *data)
    {
    if (host.dma_in_use)
    mmc_omap_release_dma(host, data, 1);
    host.data = core::ptr::null_mut();
    host.sg_len = 0;
    mmc_omap_send_abort(host, 10000);
    }
    static void
    mmc_omap_end_of_data(struct mmc_omap_host *host, struct mmc_data *data)
    {
    unsigned long flags;
    int done;
    if (!host.dma_in_use) {
    mmc_omap_xfer_done(host, data);
    return;
    }
    done = 0;
    spin_lock_irqsave(&host.dma_lock, flags);
    if (host.dma_done)
    done = 1;
    else
    host.brs_received = 1;
    spin_unlock_irqrestore(&host.dma_lock, flags);
    if (done)
    mmc_omap_xfer_done(host, data);
    }
    static void
    mmc_omap_dma_done(struct mmc_omap_host *host, struct mmc_data *data)
    {
    unsigned long flags;
    int done;
    done = 0;
    spin_lock_irqsave(&host.dma_lock, flags);
    if (host.brs_received)
    done = 1;
    else
    host.dma_done = 1;
    spin_unlock_irqrestore(&host.dma_lock, flags);
    if (done)
    mmc_omap_xfer_done(host, data);
    }
    static void
    mmc_omap_cmd_done(struct mmc_omap_host *host, struct mmc_command *cmd)
    {
    host.cmd = core::ptr::null_mut();
    timer_delete(&host.cmd_abort_timer);
    if (cmd.flags & MMC_RSP_PRESENT) {
    if (cmd.flags & MMC_RSP_136) {
// response type 2
    cmd.resp[3] =
    OMAP_MMC_READ(host, RSP0) |
    (OMAP_MMC_READ(host, RSP1) << 16);
    cmd.resp[2] =
    OMAP_MMC_READ(host, RSP2) |
    (OMAP_MMC_READ(host, RSP3) << 16);
    cmd.resp[1] =
    OMAP_MMC_READ(host, RSP4) |
    (OMAP_MMC_READ(host, RSP5) << 16);
    cmd.resp[0] =
    OMAP_MMC_READ(host, RSP6) |
    (OMAP_MMC_READ(host, RSP7) << 16);
    } else {
// response types 1, 1b, 3, 4, 5, 6
    cmd.resp[0] =
    OMAP_MMC_READ(host, RSP6) |
    (OMAP_MMC_READ(host, RSP7) << 16);
    }
    }
    if (host.data == core::ptr::null_mut() || cmd.error) {
    struct mmc_host *mmc;
    if (host.data != core::ptr::null_mut())
    mmc_omap_abort_xfer(host, host.data);
    host.mrq = core::ptr::null_mut();
    mmc = host.mmc;
    mmc_omap_release_slot(host.current_slot, 1);
    mmc_request_done(mmc, cmd.mrq);
    }
    }
//
// Abort stuck command. Can occur when card is removed while it is being
// read.
//
#[no_mangle]
unsafe extern "C" fn mmc_omap_abort_command(work: *mut work_struct) {
    static void mmc_omap_abort_command(struct work_struct *work)
    {
    struct mmc_omap_host *host = container_of(work, struct mmc_omap_host,
    cmd_abort_work);
    BUG_ON(!host.cmd);
    dev_dbg(mmc_dev(host.mmc), "Aborting stuck command CMD%d\n",
    host.cmd.opcode);
    if (host.cmd.error == 0)
    host.cmd.error = -ETIMEDOUT;
    if (host.data == core::ptr::null_mut()) {
    struct mmc_command *cmd;
    struct mmc_host    *mmc;
    cmd = host.cmd;
    host.cmd = core::ptr::null_mut();
    mmc_omap_send_abort(host, 10000);
    host.mrq = core::ptr::null_mut();
    mmc = host.mmc;
    mmc_omap_release_slot(host.current_slot, 1);
    mmc_request_done(mmc, cmd.mrq);
    } else
    mmc_omap_cmd_done(host, host.cmd);
    host.abort = 0;
    enable_irq(host.irq);
    }
    static void
    mmc_omap_cmd_timer(struct timer_list *t)
    {
    struct mmc_omap_host *host = timer_container_of(host, t,
    cmd_abort_timer);
    unsigned long flags;
    spin_lock_irqsave(&host.slot_lock, flags);
    if (host.cmd != core::ptr::null_mut() && !host.abort) {
    OMAP_MMC_WRITE(host, IE, 0);
    disable_irq(host.irq);
    host.abort = 1;
    queue_work(host.mmc_omap_wq, &host.cmd_abort_work);
    }
    spin_unlock_irqrestore(&host.slot_lock, flags);
    }
    static void
    mmc_omap_clk_timer(struct timer_list *t)
    {
    struct mmc_omap_host *host = timer_container_of(host, t, clk_timer);
    mmc_omap_fclk_enable(host, 0);
    }
// PIO only
    static void
    mmc_omap_xfer_data(struct mmc_omap_host *host, int write)
    {
    struct sg_mapping_iter *sgm = &host.sg_miter;
    int n, nwords;
    u16 *buffer;
    if (!sg_miter_next(sgm)) {
// This should not happen
    dev_err(mmc_dev(host.mmc), "ran out of scatterlist prematurely\n");
    return;
    }
    buffer = sgm.addr;
    n = 64;
    if (n > sgm.length)
    n = sgm.length;
    if (n > host.total_bytes_left)
    n = host.total_bytes_left;
// Round up to handle odd number of bytes to transfer
    nwords = DIV_ROUND_UP(n, 2);
    sgm.consumed = n;
    host.total_bytes_left -= n;
    host.data.bytes_xfered += n;
    if (write) {
    __raw_writesw(host.virt_base + OMAP_MMC_REG(host, DATA),
    buffer, nwords);
    } else {
    __raw_readsw(host.virt_base + OMAP_MMC_REG(host, DATA),
    buffer, nwords);
    }
    }

#[no_mangle]
unsafe extern "C" fn mmc_omap_report_irq(host: *mut mmc_omap_host, status: u16) {
    static void mmc_omap_report_irq(struct mmc_omap_host *host, u16 status)
    {
    static const char *mmc_omap_status_bits[] = {
    "EOC", "CD", "CB", "BRS", "EOFB", "DTO", "DCRC", "CTO",
    "CCRC", "CRW", "AF", "AE", "OCRB", "CIRQ", "CERR"
    };
    int i;
    char res[64], *buf = res;
    buf += sprintf(buf, "MMC IRQ 0x%x:", status);
    for (i = 0; i < ARRAY_SIZE(mmc_omap_status_bits); i++)
    if (status & (1 << i))
    buf += sprintf(buf, " %s", mmc_omap_status_bits[i]);
    dev_vdbg(mmc_dev(host.mmc), "%s\n", res);
    }

#[no_mangle]
unsafe extern "C" fn mmc_omap_report_irq(host: *mut mmc_omap_host, status: u16) {
    static void mmc_omap_report_irq(struct mmc_omap_host *host, u16 status)
    {
    }

#[no_mangle]
unsafe extern "C" fn mmc_omap_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mmc_omap_irq(int irq, void *dev_id)
    {
    let mut host: *mut mmc_omap_host = (struct mmc_omap_host *)dev_id;
    u16 status;
    int end_command;
    int end_transfer;
    int transfer_error, cmd_error;
    if (host.cmd == core::ptr::null_mut() && host.data == core::ptr::null_mut()) {
    status = OMAP_MMC_READ(host, STAT);
    dev_info(mmc_dev(host.slots[0].mmc),
    "Spurious IRQ 0x%04x\n", status);
    if (status != 0) {
    OMAP_MMC_WRITE(host, STAT, status);
    OMAP_MMC_WRITE(host, IE, 0);
    }
    return IRQ_HANDLED;
    }
    end_command = 0;
    end_transfer = 0;
    transfer_error = 0;
    cmd_error = 0;
    while ((status = OMAP_MMC_READ(host, STAT)) != 0) {
    int cmd;
    OMAP_MMC_WRITE(host, STAT, status);
    if (host.cmd != core::ptr::null_mut())
    cmd = host.cmd.opcode;
    else
    cmd = -1;
    dev_dbg(mmc_dev(host.mmc), "MMC IRQ %04x (CMD %d): ",
    status, cmd);
    mmc_omap_report_irq(host, status);
    if (host.total_bytes_left) {
    if ((status & OMAP_MMC_STAT_A_FULL) ||
    (status & OMAP_MMC_STAT_END_OF_DATA))
    mmc_omap_xfer_data(host, 0);
    if (status & OMAP_MMC_STAT_A_EMPTY)
    mmc_omap_xfer_data(host, 1);
    }
    if (status & OMAP_MMC_STAT_END_OF_DATA)
    end_transfer = 1;
    if (status & OMAP_MMC_STAT_DATA_TOUT) {
    dev_dbg(mmc_dev(host.mmc), "data timeout (CMD%d)\n",
    cmd);
    if (host.data) {
    host.data.error = -ETIMEDOUT;
    transfer_error = 1;
    }
    }
    if (status & OMAP_MMC_STAT_DATA_CRC) {
    if (host.data) {
    host.data.error = -EILSEQ;
    dev_dbg(mmc_dev(host.mmc),
    "data CRC error, bytes left %d\n",
    host.total_bytes_left);
    transfer_error = 1;
    } else {
    dev_dbg(mmc_dev(host.mmc), "data CRC error\n");
    }
    }
    if (status & OMAP_MMC_STAT_CMD_TOUT) {
// Timeouts are routine with some commands
    if (host.cmd) {
    struct mmc_omap_slot *slot =
    host.current_slot;
    if (slot == core::ptr::null_mut() ||
    !mmc_omap_cover_is_open(slot))
    dev_err(mmc_dev(host.mmc),
    "command timeout (CMD%d)\n",
    cmd);
    host.cmd.error = -ETIMEDOUT;
    end_command = 1;
    cmd_error = 1;
    }
    }
    if (status & OMAP_MMC_STAT_CMD_CRC) {
    if (host.cmd) {
    dev_err(mmc_dev(host.mmc),
    "command CRC error (CMD%d, arg 0x%08x)\n",
    cmd, host.cmd.arg);
    host.cmd.error = -EILSEQ;
    end_command = 1;
    cmd_error = 1;
    } else
    dev_err(mmc_dev(host.mmc),
    "command CRC error without cmd?\n");
    }
    if (status & OMAP_MMC_STAT_CARD_ERR) {
    dev_dbg(mmc_dev(host.mmc),
    "ignoring card status error (CMD%d)\n",
    cmd);
    end_command = 1;
    }
//
// NOTE: On 1610 the END_OF_CMD may come too early when
// starting a write
//
    if ((status & OMAP_MMC_STAT_END_OF_CMD) &&
    (!(status & OMAP_MMC_STAT_A_EMPTY))) {
    end_command = 1;
    }
    }
    if (cmd_error && host.data) {
    timer_delete(&host.cmd_abort_timer);
    host.abort = 1;
    OMAP_MMC_WRITE(host, IE, 0);
    disable_irq_nosync(host.irq);
    queue_work(host.mmc_omap_wq, &host.cmd_abort_work);
    return IRQ_HANDLED;
    }
    if (end_command && host.cmd)
    mmc_omap_cmd_done(host, host.cmd);
    if (host.data != core::ptr::null_mut()) {
    if (transfer_error)
    mmc_omap_xfer_done(host, host.data);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: end_transfer) -> else {
    else if (end_transfer)
    mmc_omap_end_of_data(host, host.data);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn omap_mmc_notify_cover_event(dev: *mut device, num: c_int, is_closed: c_int) {
    void omap_mmc_notify_cover_event(struct device *dev, int num, int is_closed)
    {
    int cover_open;
    struct mmc_omap_host *host = dev_get_drvdata(dev);
    struct mmc_omap_slot *slot = host.slots[num];
    BUG_ON(num >= host.nr_slots);
// Other subsystems can call in here before we're initialised.
    if (host.nr_slots == 0 || !host.slots[num])
    return;
    cover_open = mmc_omap_cover_is_open(slot);
    if (cover_open != slot.cover_open) {
    slot.cover_open = cover_open;
    sysfs_notify(&slot.mmc.class_dev.kobj, core::ptr::null_mut(), "cover_switch");
    }
    queue_work(system_bh_highpri_wq, &slot.cover_bh_work);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_cover_timer(t: *mut timer_list) {
    static void mmc_omap_cover_timer(struct timer_list *t)
    {
    struct mmc_omap_slot *slot = timer_container_of(slot, t, cover_timer);
    queue_work(system_bh_wq, &slot.cover_bh_work);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_cover_bh_handler(t: *mut work_struct) {
    static void mmc_omap_cover_bh_handler(struct work_struct *t)
    {
    struct mmc_omap_slot *slot = from_work(slot, t, cover_bh_work);
    let mut cover_open: c_int = mmc_omap_cover_is_open(slot);
    mmc_detect_change(slot.mmc, 0);
    if (!cover_open)
    return;
//
// If no card is inserted, we postpone polling until
// the cover has been closed.
//
    if (slot.mmc.card == core::ptr::null_mut())
    return;
    mod_timer(&slot.cover_timer,
    jiffies + msecs_to_jiffies(OMAP_MMC_COVER_POLL_DELAY));
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_dma_callback(priv: *mut c_void) {
    static void mmc_omap_dma_callback(void *priv)
    {
    struct mmc_omap_host *host = priv;
    struct mmc_data *data = host.data;
// If we got to the end of DMA, assume everything went well
    data.bytes_xfered += data.blocks * data.blksz;
    mmc_omap_dma_done(host, data);
    }
#[no_mangle]
pub unsafe extern "C" fn set_cmd_timeout(host: *mut mmc_omap_host, req: *mut mmc_request) {
    static inline void set_cmd_timeout(struct mmc_omap_host *host, struct mmc_request *req)
    {
    u16 reg;
    reg = OMAP_MMC_READ(host, SDIO);
    reg &= ~(1 << 5);
    OMAP_MMC_WRITE(host, SDIO, reg);
// Set maximum timeout
    OMAP_MMC_WRITE(host, CTO, 0xfd);
    }
#[no_mangle]
pub unsafe extern "C" fn set_data_timeout(host: *mut mmc_omap_host, req: *mut mmc_request) {
    static inline void set_data_timeout(struct mmc_omap_host *host, struct mmc_request *req)
    {
    unsigned int timeout, cycle_ns;
    u16 reg;
    cycle_ns = 1000000000 / host.current_slot.fclk_freq;
    timeout = req.data.timeout_ns / cycle_ns;
    timeout += req.data.timeout_clks;
// Check if we need to use timeout multiplier register
    reg = OMAP_MMC_READ(host, SDIO);
    if (timeout > 0xffff) {
    reg |= (1 << 5);
    timeout /= 1024;
    } else
    reg &= ~(1 << 5);
    OMAP_MMC_WRITE(host, SDIO, reg);
    OMAP_MMC_WRITE(host, DTO, timeout);
    }
    static void
    mmc_omap_prepare_data(struct mmc_omap_host *host, struct mmc_request *req)
    {
    unsigned int miter_flags = SG_MITER_ATOMIC; /* Used from IRQ */
    struct mmc_data *data = req.data;
    int i, use_dma = 1, block_size;
    struct scatterlist *sg;
    unsigned sg_len;
    host.data = data;
    if (data == core::ptr::null_mut()) {
    OMAP_MMC_WRITE(host, BLEN, 0);
    OMAP_MMC_WRITE(host, NBLK, 0);
    OMAP_MMC_WRITE(host, BUF, 0);
    host.dma_in_use = 0;
    set_cmd_timeout(host, req);
    return;
    }
    block_size = data.blksz;
    OMAP_MMC_WRITE(host, NBLK, data.blocks - 1);
    OMAP_MMC_WRITE(host, BLEN, block_size - 1);
    set_data_timeout(host, req);
// cope with calling layer confusion; it issues "single
// block" writes using multi-block scatterlists.
//
    sg_len = (data.blocks == 1) ? 1 : data.sg_len;
// Only do DMA for entire blocks
    for_each_sg(data.sg, sg, sg_len, i) {
    if ((sg.length % block_size) != 0) {
    use_dma = 0;
    break;
    }
    }
    if (use_dma) {
    enum dma_data_direction dma_data_dir;
    struct dma_async_tx_descriptor *tx;
    struct dma_chan *c;
    u32 burst, *bp;
    u16 buf;
//
// FIFO is 16x2 bytes on 15xx, and 32x2 bytes on 16xx
// and 24xx. Use 16 or 32 word frames when the
// blocksize is at least that large. Blocksize is
// usually 512 bytes; but not for some SD reads.
//
    burst = mmc_omap15xx() ? 32 : 64;
    if (burst > data.blksz)
    burst = data.blksz;
    burst >>= 1;
    if (data.flags & MMC_DATA_WRITE) {
    c = host.dma_tx;
    bp = &host.dma_tx_burst;
    buf = 0x0f80 | (burst - 1) << 0;
    dma_data_dir = DMA_TO_DEVICE;
    } else {
    c = host.dma_rx;
    bp = &host.dma_rx_burst;
    buf = 0x800f | (burst - 1) << 8;
    dma_data_dir = DMA_FROM_DEVICE;
    }
    if (!c)
    goto use_pio;
// Only reconfigure if we have a different burst size
    if (*bp != burst) {
    struct dma_slave_config cfg = {
    .src_addr = host.phys_base +
    OMAP_MMC_REG(host, DATA),
    .dst_addr = host.phys_base +
    OMAP_MMC_REG(host, DATA),
    .src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES,
    .dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES,
    .src_maxburst = burst,
    .dst_maxburst = burst,
    };
    if (dmaengine_slave_config(c, &cfg))
    goto use_pio;
// bp = burst;
    }
    host.sg_len = dma_map_sg(c.device.dev, data.sg, sg_len,
    dma_data_dir);
    if (host.sg_len == 0)
    goto use_pio;
    tx = dmaengine_prep_slave_sg(c, data.sg, host.sg_len,
    data.flags & MMC_DATA_WRITE ? DMA_MEM_TO_DEV : DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx)
    goto use_pio;
    OMAP_MMC_WRITE(host, BUF, buf);
    tx.callback = mmc_omap_dma_callback;
    tx.callback_param = host;
    dmaengine_submit(tx);
    host.brs_received = 0;
    host.dma_done = 0;
    host.dma_in_use = 1;
    return;
    }
    use_pio:
// Revert to PIO?
    OMAP_MMC_WRITE(host, BUF, 0x1f1f);
    host.total_bytes_left = data.blocks * block_size;
    host.sg_len = sg_len;
    if (data.flags & MMC_DATA_READ)
    miter_flags |= SG_MITER_TO_SG;
    else
    miter_flags |= SG_MITER_FROM_SG;
    sg_miter_start(&host.sg_miter, data.sg, data.sg_len, miter_flags);
    host.dma_in_use = 0;
    }
    static void mmc_omap_start_request(struct mmc_omap_host *host,
    struct mmc_request *req)
    {
    BUG_ON(host.mrq != core::ptr::null_mut());
    host.mrq = req;
// only touch fifo AFTER the controller readies it
    mmc_omap_prepare_data(host, req);
    mmc_omap_start_command(host, req.cmd);
    if (host.dma_in_use) {
    struct dma_chan *c = host.data.flags & MMC_DATA_WRITE ?
    host.dma_tx : host.dma_rx;
    dma_async_issue_pending(c);
    }
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_request(mmc: *mut mmc_host, req: *mut mmc_request) {
    static void mmc_omap_request(struct mmc_host *mmc, struct mmc_request *req)
    {
    struct mmc_omap_slot *slot = mmc_priv(mmc);
    struct mmc_omap_host *host = slot.host;
    unsigned long flags;
    spin_lock_irqsave(&host.slot_lock, flags);
    if (host.mmc != core::ptr::null_mut()) {
    BUG_ON(slot.mrq != core::ptr::null_mut());
    slot.mrq = req;
    spin_unlock_irqrestore(&host.slot_lock, flags);
    return;
    } else
    host.mmc = mmc;
    spin_unlock_irqrestore(&host.slot_lock, flags);
    mmc_omap_select_slot(slot, 1);
    mmc_omap_start_request(host, req);
    }
    static void mmc_omap_set_power(struct mmc_omap_slot *slot, int power_on,
    int vdd)
    {
    struct mmc_omap_host *host;
    host = slot.host;
    if (power_on) {
    if (slot.vsd) {
    gpiod_set_value(slot.vsd, power_on);
    msleep(1);
    }
    if (slot.vio) {
    gpiod_set_value(slot.vio, power_on);
    msleep(1);
    }
    } else {
    if (slot.vio) {
    gpiod_set_value(slot.vio, power_on);
    msleep(50);
    }
    if (slot.vsd) {
    gpiod_set_value(slot.vsd, power_on);
    msleep(50);
    }
    }
    if (slot.pdata.set_power != core::ptr::null_mut())
    slot.pdata.set_power(mmc_dev(slot.mmc), slot.id, power_on,
    vdd);
    if (mmc_omap2()) {
    u16 w;
    if (power_on) {
    w = OMAP_MMC_READ(host, CON);
    OMAP_MMC_WRITE(host, CON, w | (1 << 11));
    } else {
    w = OMAP_MMC_READ(host, CON);
    OMAP_MMC_WRITE(host, CON, w & ~(1 << 11));
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_calc_divisor(mmc: *mut mmc_host, ios: *mut mmc_ios) -> c_int {
    static int mmc_omap_calc_divisor(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct mmc_omap_slot *slot = mmc_priv(mmc);
    struct mmc_omap_host *host = slot.host;
    let mut func_clk_rate: c_int = clk_get_rate(host.fclk);
    int dsor;
    if (ios.clock == 0)
    return 0;
    dsor = func_clk_rate / ios.clock;
    if (dsor < 1)
    dsor = 1;
    if (func_clk_rate / dsor > ios.clock)
    dsor++;
    if (dsor > 250)
    dsor = 250;
    slot.fclk_freq = func_clk_rate / dsor;
    if (ios.bus_width == MMC_BUS_WIDTH_4)
    dsor |= 1 << 15;
    return dsor;
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void mmc_omap_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct mmc_omap_slot *slot = mmc_priv(mmc);
    struct mmc_omap_host *host = slot.host;
    int i, dsor;
    int clk_enabled, init_stream;
    mmc_omap_select_slot(slot, 0);
    dsor = mmc_omap_calc_divisor(mmc, ios);
    if (ios.vdd != slot.vdd)
    slot.vdd = ios.vdd;
    clk_enabled = 0;
    init_stream = 0;
    switch (ios.power_mode) {
    case MMC_POWER_OFF:
    mmc_omap_set_power(slot, 0, ios.vdd);
    break;
    case MMC_POWER_UP:
// Cannot touch dsor yet, just power up MMC
    mmc_omap_set_power(slot, 1, ios.vdd);
    slot.power_mode = ios.power_mode;
    goto exit;
    case MMC_POWER_ON:
    mmc_omap_fclk_enable(host, 1);
    clk_enabled = 1;
    dsor |= 1 << 11;
    if (slot.power_mode != MMC_POWER_ON)
    init_stream = 1;
    break;
    }
    slot.power_mode = ios.power_mode;
    if (slot.bus_mode != ios.bus_mode) {
    if (slot.pdata.set_bus_mode != core::ptr::null_mut())
    slot.pdata.set_bus_mode(mmc_dev(mmc), slot.id,
    ios.bus_mode);
    slot.bus_mode = ios.bus_mode;
    }
// On insanely high arm_per frequencies something sometimes
// goes somehow out of sync, and the POW bit is not being set,
// which results in the while loop below getting stuck.
// Writing to the CON register twice seems to do the trick.
    for (i = 0; i < 2; i++)
    OMAP_MMC_WRITE(host, CON, dsor);
    slot.saved_con = dsor;
    if (init_stream) {
// worst case at 400kHz, 80 cycles makes 200 microsecs
    let mut usecs: c_int = 250;
// Send clock cycles, poll completion
    OMAP_MMC_WRITE(host, IE, 0);
    OMAP_MMC_WRITE(host, STAT, 0xffff);
    OMAP_MMC_WRITE(host, CMD, 1 << 7);
    while (usecs > 0 && (OMAP_MMC_READ(host, STAT) & 1) == 0) {
    udelay(1);
    usecs--;
    }
    OMAP_MMC_WRITE(host, STAT, 1);
    }
    exit:
    mmc_omap_release_slot(slot, clk_enabled);
    }
    static const struct mmc_host_ops mmc_omap_ops = {
    .request	= mmc_omap_request,
    .set_ios	= mmc_omap_set_ios,
    };
#[no_mangle]
unsafe extern "C" fn mmc_omap_new_slot(host: *mut mmc_omap_host, id: c_int) -> c_int {
    static int mmc_omap_new_slot(struct mmc_omap_host *host, int id)
    {
    struct mmc_omap_slot *slot = core::ptr::null_mut();
    struct mmc_host *mmc;
    int r;
    mmc = devm_mmc_alloc_host(host.dev, sizeof(*slot));
    if (mmc == core::ptr::null_mut())
    return -ENOMEM;
    slot = mmc_priv(mmc);
    slot.host = host;
    slot.mmc = mmc;
    slot.id = id;
    slot.power_mode = MMC_POWER_UNDEFINED;
    slot.pdata = &host.pdata.slots[id];
// Check for some optional GPIO controls
    slot.vsd = devm_gpiod_get_index_optional(host.dev, "vsd",
    id, GPIOD_OUT_LOW);
    if (IS_ERR(slot.vsd))
    return dev_err_probe(host.dev, PTR_ERR(slot.vsd),
    "error looking up VSD GPIO\n");
    slot.vio = devm_gpiod_get_index_optional(host.dev, "vio",
    id, GPIOD_OUT_LOW);
    if (IS_ERR(slot.vio))
    return dev_err_probe(host.dev, PTR_ERR(slot.vio),
    "error looking up VIO GPIO\n");
    slot.cover = devm_gpiod_get_index_optional(host.dev, "cover",
    id, GPIOD_IN);
    if (IS_ERR(slot.cover))
    return dev_err_probe(host.dev, PTR_ERR(slot.cover),
    "error looking up cover switch GPIO\n");
    host.slots[id] = slot;
    mmc.caps = 0;
    if (host.pdata.slots[id].wires >= 4)
    mmc.caps |= MMC_CAP_4_BIT_DATA;
    mmc.ops = &mmc_omap_ops;
    mmc.f_min = 400000;
    if (mmc_omap2())
    mmc.f_max = 48000000;
    else
    mmc.f_max = 24000000;
    if (host.pdata.max_freq)
    mmc.f_max = min(host.pdata.max_freq, mmc.f_max);
    mmc.ocr_avail = slot.pdata.ocr_mask;
// Use scatterlist DMA to reduce per-transfer costs.
// NOTE max_seg_size assumption that small blocks aren't
// normally used (except e.g. for reading SD registers).
//
    mmc.max_segs = 32;
    mmc.max_blk_size = 2048;	/* BLEN is 11 bits (+1) */
    mmc.max_blk_count = 2048;	/* NBLK is 11 bits (+1) */
    mmc.max_req_size = mmc.max_blk_size * mmc.max_blk_count;
    mmc.max_seg_size = mmc.max_req_size;
    if (slot.pdata.get_cover_state != core::ptr::null_mut()) {
    timer_setup(&slot.cover_timer, mmc_omap_cover_timer, 0);
    INIT_WORK(&slot.cover_bh_work, mmc_omap_cover_bh_handler);
    }
    r = mmc_add_host(mmc);
    if (r < 0)
    goto err_remove_host;
    if (slot.pdata.name != core::ptr::null_mut()) {
    r = device_create_file(&mmc.class_dev,
    &dev_attr_slot_name);
    if (r < 0)
    goto err_remove_host;
    }
    if (slot.pdata.get_cover_state != core::ptr::null_mut()) {
    r = device_create_file(&mmc.class_dev,
    &dev_attr_cover_switch);
    if (r < 0)
    goto err_remove_slot_name;
    queue_work(system_bh_wq, &slot.cover_bh_work);
    }
    return 0;
    err_remove_slot_name:
    if (slot.pdata.name != core::ptr::null_mut())
    device_remove_file(&mmc.class_dev, &dev_attr_slot_name);
    err_remove_host:
    mmc_remove_host(mmc);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_remove_slot(slot: *mut mmc_omap_slot) {
    static void mmc_omap_remove_slot(struct mmc_omap_slot *slot)
    {
    struct mmc_host *mmc = slot.mmc;
    if (slot.pdata.name != core::ptr::null_mut())
    device_remove_file(&mmc.class_dev, &dev_attr_slot_name);
    if (slot.pdata.get_cover_state != core::ptr::null_mut())
    device_remove_file(&mmc.class_dev, &dev_attr_cover_switch);
    cancel_work_sync(&slot.cover_bh_work);
    timer_delete_sync(&slot.cover_timer);
    flush_workqueue(slot.host.mmc_omap_wq);
    mmc_remove_host(mmc);
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_probe(pdev: *mut platform_device) -> c_int {
    static int mmc_omap_probe(struct platform_device *pdev)
    {
    struct omap_mmc_platform_data *pdata = pdev.dev.platform_data;
    struct mmc_omap_host *host = core::ptr::null_mut();
    struct resource *res;
    int i, ret = 0;
    int irq;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "platform data missing\n");
    return -ENXIO;
    }
    if (pdata.nr_slots == 0) {
    dev_err(&pdev.dev, "no slots\n");
    return -EPROBE_DEFER;
    }
    host = devm_kzalloc(&pdev.dev, sizeof(struct mmc_omap_host),
    GFP_KERNEL);
    if (host == core::ptr::null_mut())
    return -ENOMEM;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    host.virt_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(host.virt_base))
    return PTR_ERR(host.virt_base);
    INIT_WORK(&host.slot_release_work, mmc_omap_slot_release_work);
    INIT_WORK(&host.send_stop_work, mmc_omap_send_stop_work);
    INIT_WORK(&host.cmd_abort_work, mmc_omap_abort_command);
    timer_setup(&host.cmd_abort_timer, mmc_omap_cmd_timer, 0);
    spin_lock_init(&host.clk_lock);
    timer_setup(&host.clk_timer, mmc_omap_clk_timer, 0);
    spin_lock_init(&host.dma_lock);
    spin_lock_init(&host.slot_lock);
    init_waitqueue_head(&host.slot_wq);
    host.pdata = pdata;
    host.features = host.pdata.slots[0].features;
    host.dev = &pdev.dev;
    platform_set_drvdata(pdev, host);
    host.slot_switch = devm_gpiod_get_optional(host.dev, "switch",
    GPIOD_OUT_LOW);
    if (IS_ERR(host.slot_switch))
    return dev_err_probe(host.dev, PTR_ERR(host.slot_switch),
    "error looking up slot switch GPIO\n");
    host.id = pdev.id;
    host.irq = irq;
    host.phys_base = res.start;
    host.iclk = clk_get(&pdev.dev, "ick");
    if (IS_ERR(host.iclk))
    return PTR_ERR(host.iclk);
    clk_prepare_enable(host.iclk);
    host.fclk = clk_get(&pdev.dev, "fck");
    if (IS_ERR(host.fclk)) {
    ret = PTR_ERR(host.fclk);
    goto err_free_iclk;
    }
    ret = clk_prepare(host.fclk);
    if (ret)
    goto err_put_fclk;
    host.dma_tx_burst = -1;
    host.dma_rx_burst = -1;
    host.dma_tx = dma_request_chan(&pdev.dev, "tx");
    if (IS_ERR(host.dma_tx)) {
    ret = PTR_ERR(host.dma_tx);
    if (ret == -EPROBE_DEFER)
    goto err_free_fclk;
    host.dma_tx = core::ptr::null_mut();
    dev_warn(host.dev, "TX DMA channel request failed\n");
    }
    host.dma_rx = dma_request_chan(&pdev.dev, "rx");
    if (IS_ERR(host.dma_rx)) {
    ret = PTR_ERR(host.dma_rx);
    if (ret == -EPROBE_DEFER) {
    if (host.dma_tx)
    dma_release_channel(host.dma_tx);
    goto err_free_fclk;
    }
    host.dma_rx = core::ptr::null_mut();
    dev_warn(host.dev, "RX DMA channel request failed\n");
    }
    ret = request_irq(host.irq, mmc_omap_irq, 0, DRIVER_NAME, host);
    if (ret)
    goto err_free_dma;
    if (pdata.init != core::ptr::null_mut()) {
    ret = pdata.init(&pdev.dev);
    if (ret < 0)
    goto err_free_irq;
    }
    host.nr_slots = pdata.nr_slots;
    host.reg_shift = (mmc_omap7xx() ? 1 : 2);
    host.mmc_omap_wq = alloc_workqueue("mmc_omap", WQ_PERCPU, 0);
    if (!host.mmc_omap_wq) {
    ret = -ENOMEM;
    goto err_plat_cleanup;
    }
    for (i = 0; i < pdata.nr_slots; i++) {
    ret = mmc_omap_new_slot(host, i);
    if (ret < 0) {
    while (--i >= 0)
    mmc_omap_remove_slot(host.slots[i]);
    goto err_destroy_wq;
    }
    }
    return 0;
    err_destroy_wq:
    destroy_workqueue(host.mmc_omap_wq);
    err_plat_cleanup:
    if (pdata.cleanup)
    pdata.cleanup(&pdev.dev);
    err_free_irq:
    free_irq(host.irq, host);
    err_free_dma:
    if (host.dma_tx)
    dma_release_channel(host.dma_tx);
    if (host.dma_rx)
    dma_release_channel(host.dma_rx);
    err_free_fclk:
    clk_unprepare(host.fclk);
    err_put_fclk:
    clk_put(host.fclk);
    err_free_iclk:
    clk_disable_unprepare(host.iclk);
    clk_put(host.iclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mmc_omap_remove(pdev: *mut platform_device) {
    static void mmc_omap_remove(struct platform_device *pdev)
    {
    struct mmc_omap_host *host = platform_get_drvdata(pdev);
    int i;
    BUG_ON(host == core::ptr::null_mut());
    for (i = 0; i < host.nr_slots; i++)
    mmc_omap_remove_slot(host.slots[i]);
    if (host.pdata.cleanup)
    host.pdata.cleanup(&pdev.dev);
    mmc_omap_fclk_enable(host, 0);
    free_irq(host.irq, host);
    clk_unprepare(host.fclk);
    clk_put(host.fclk);
    clk_disable_unprepare(host.iclk);
    clk_put(host.iclk);
    if (host.dma_tx)
    dma_release_channel(host.dma_tx);
    if (host.dma_rx)
    dma_release_channel(host.dma_rx);
    destroy_workqueue(host.mmc_omap_wq);
    }

    static const struct of_device_id mmc_omap_match[] = {
    { .compatible = "ti,omap2420-mmc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mmc_omap_match);

    static struct platform_driver mmc_omap_driver = {
    .probe		= mmc_omap_probe,
    .remove		= mmc_omap_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(mmc_omap_match),
    },
    };
    module_platform_driver(mmc_omap_driver);
    MODULE_DESCRIPTION("OMAP Multimedia Card driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRIVER_NAME);
    MODULE_AUTHOR("Juha Yrjölä");
