//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/hw/cdma_hw.c
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
// Tegra host1x Command DMA
//
// Copyright (c) 2010-2013, NVIDIA Corporation.
//

//
// Put the restart at the end of pushbuffer memory
//
#[no_mangle]
unsafe extern "C" fn push_buffer_init(pb: *mut push_buffer) {
    static void push_buffer_init(struct push_buffer *pb)
    {
// (u32 *)(pb->mapped + pb->size) = host1x_opcode_restart(0);
    }
//
// Increment timedout buffer's syncpt via CPU.
//
    static void cdma_timeout_cpu_incr(struct host1x_cdma *cdma, u32 getptr,
    u32 syncpt_incrs, u32 syncval, u32 nr_slots)
    {
    unsigned int i;
    for (i = 0; i < syncpt_incrs; i++)
    host1x_syncpt_incr(cdma.timeout.syncpt);
// after CPU incr, ensure shadow is up to date
    host1x_syncpt_load(cdma.timeout.syncpt);
    }
//
// Start channel DMA
//
#[no_mangle]
unsafe extern "C" fn cdma_start(cdma: *mut host1x_cdma) {
    static void cdma_start(struct host1x_cdma *cdma)
    {
    struct host1x_channel *ch = cdma_to_channel(cdma);
    u64 start, end;
    if (cdma.running)
    return;
    cdma.last_pos = cdma.push_buffer.pos;
    start = cdma.push_buffer.dma;
    end = cdma.push_buffer.size + 4;
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP,
    HOST1X_CHANNEL_DMACTRL);
// set base, put and end pointer
    host1x_ch_writel(ch, lower_32_bits(start), HOST1X_CHANNEL_DMASTART);

    host1x_ch_writel(ch, upper_32_bits(start), HOST1X_CHANNEL_DMASTART_HI);

    host1x_ch_writel(ch, cdma.push_buffer.pos, HOST1X_CHANNEL_DMAPUT);

    host1x_ch_writel(ch, 0, HOST1X_CHANNEL_DMAPUT_HI);

    host1x_ch_writel(ch, lower_32_bits(end), HOST1X_CHANNEL_DMAEND);

    host1x_ch_writel(ch, upper_32_bits(end), HOST1X_CHANNEL_DMAEND_HI);

// reset GET
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP |
    HOST1X_CHANNEL_DMACTRL_DMAGETRST |
    HOST1X_CHANNEL_DMACTRL_DMAINITGET,
    HOST1X_CHANNEL_DMACTRL);
// start the command DMA
    host1x_ch_writel(ch, 0, HOST1X_CHANNEL_DMACTRL);
    cdma.running = true;
    }
//
// Similar to cdma_start(), but rather than starting from an idle
// state (where DMA GET is set to DMA PUT), on a timeout we restore
// DMA GET from an explicit value (so DMA may again be pending).
//
#[no_mangle]
unsafe extern "C" fn cdma_timeout_restart(cdma: *mut host1x_cdma, getptr: u32) {
    static void cdma_timeout_restart(struct host1x_cdma *cdma, u32 getptr)
    {
    struct host1x *host1x = cdma_to_host1x(cdma);
    struct host1x_channel *ch = cdma_to_channel(cdma);
    u64 start, end;
    if (cdma.running)
    return;
    cdma.last_pos = cdma.push_buffer.pos;
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP,
    HOST1X_CHANNEL_DMACTRL);
    start = cdma.push_buffer.dma;
    end = cdma.push_buffer.size + 4;
// set base, end pointer (all of memory)
    host1x_ch_writel(ch, lower_32_bits(start), HOST1X_CHANNEL_DMASTART);

    host1x_ch_writel(ch, upper_32_bits(start), HOST1X_CHANNEL_DMASTART_HI);

    host1x_ch_writel(ch, lower_32_bits(end), HOST1X_CHANNEL_DMAEND);

    host1x_ch_writel(ch, upper_32_bits(end), HOST1X_CHANNEL_DMAEND_HI);

// set GET, by loading the value in PUT (then reset GET)
    host1x_ch_writel(ch, getptr, HOST1X_CHANNEL_DMAPUT);
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP |
    HOST1X_CHANNEL_DMACTRL_DMAGETRST |
    HOST1X_CHANNEL_DMACTRL_DMAINITGET,
    HOST1X_CHANNEL_DMACTRL);
    dev_dbg(host1x.dev,
    "%s: DMA GET 0x%x, PUT HW 0x%x / shadow 0x%x\n", __func__,
    host1x_ch_readl(ch, HOST1X_CHANNEL_DMAGET),
    host1x_ch_readl(ch, HOST1X_CHANNEL_DMAPUT),
    cdma.last_pos);
// deassert GET reset and set PUT
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP,
    HOST1X_CHANNEL_DMACTRL);
    host1x_ch_writel(ch, cdma.push_buffer.pos, HOST1X_CHANNEL_DMAPUT);
// start the command DMA
    host1x_ch_writel(ch, 0, HOST1X_CHANNEL_DMACTRL);
    cdma.running = true;
    }
//
// Kick channel DMA into action by writing its PUT offset (if it has changed)
//
#[no_mangle]
unsafe extern "C" fn cdma_flush(cdma: *mut host1x_cdma) {
    static void cdma_flush(struct host1x_cdma *cdma)
    {
    struct host1x_channel *ch = cdma_to_channel(cdma);
    if (cdma.push_buffer.pos != cdma.last_pos) {
    host1x_ch_writel(ch, cdma.push_buffer.pos,
    HOST1X_CHANNEL_DMAPUT);
    cdma.last_pos = cdma.push_buffer.pos;
    }
    }
#[no_mangle]
unsafe extern "C" fn cdma_stop(cdma: *mut host1x_cdma) {
    static void cdma_stop(struct host1x_cdma *cdma)
    {
    struct host1x_channel *ch = cdma_to_channel(cdma);
    mutex_lock(&cdma.lock);
    if (cdma.running) {
    host1x_cdma_wait_locked(cdma, CDMA_EVENT_SYNC_QUEUE_EMPTY);
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP,
    HOST1X_CHANNEL_DMACTRL);
    cdma.running = false;
    }
    mutex_unlock(&cdma.lock);
    }
    static void cdma_hw_cmdproc_stop(struct host1x *host, struct host1x_channel *ch,
    bool stop)
    {

    host1x_ch_writel(ch, stop ? 0x1 : 0x0, HOST1X_CHANNEL_CMDPROC_STOP);

    let mut cmdproc_stop: u32 = host1x_sync_readl(host, HOST1X_SYNC_CMDPROC_STOP);
    if (stop)
    cmdproc_stop |= BIT(ch.id);
    else
    cmdproc_stop &= ~BIT(ch.id);
    host1x_sync_writel(host, cmdproc_stop, HOST1X_SYNC_CMDPROC_STOP);

    }
#[no_mangle]
unsafe extern "C" fn cdma_hw_teardown(host: *mut host1x, ch: *mut host1x_channel) {
    static void cdma_hw_teardown(struct host1x *host, struct host1x_channel *ch)
    {

    host1x_ch_writel(ch, 0x1, HOST1X_CHANNEL_TEARDOWN);

    host1x_sync_writel(host, BIT(ch.id), HOST1X_SYNC_CH_TEARDOWN);

    }
//
// Stops both channel's command processor and CDMA immediately.
// Also, tears down the channel and resets corresponding module.
//
#[no_mangle]
unsafe extern "C" fn cdma_freeze(cdma: *mut host1x_cdma) {
    static void cdma_freeze(struct host1x_cdma *cdma)
    {
    struct host1x *host = cdma_to_host1x(cdma);
    struct host1x_channel *ch = cdma_to_channel(cdma);
    if (cdma.torndown && !cdma.running) {
    dev_warn(host.dev, "Already torn down\n");
    return;
    }
    dev_dbg(host.dev, "freezing channel (id %d)\n", ch.id);
    cdma_hw_cmdproc_stop(host, ch, true);
    dev_dbg(host.dev, "%s: DMA GET 0x%x, PUT HW 0x%x / shadow 0x%x\n",
    __func__, host1x_ch_readl(ch, HOST1X_CHANNEL_DMAGET),
    host1x_ch_readl(ch, HOST1X_CHANNEL_DMAPUT),
    cdma.last_pos);
    host1x_ch_writel(ch, HOST1X_CHANNEL_DMACTRL_DMASTOP,
    HOST1X_CHANNEL_DMACTRL);
    cdma_hw_teardown(host, ch);
    cdma.running = false;
    cdma.torndown = true;
    }
#[no_mangle]
unsafe extern "C" fn cdma_resume(cdma: *mut host1x_cdma, getptr: u32) {
    static void cdma_resume(struct host1x_cdma *cdma, u32 getptr)
    {
    struct host1x *host1x = cdma_to_host1x(cdma);
    struct host1x_channel *ch = cdma_to_channel(cdma);
    dev_dbg(host1x.dev,
    "resuming channel (id %u, DMAGET restart = 0x%x)\n",
    ch.id, getptr);
    cdma_hw_cmdproc_stop(host1x, ch, false);
    cdma.torndown = false;
    cdma_timeout_restart(cdma, getptr);
    }
#[no_mangle]
unsafe extern "C" fn timeout_release_mlock(cdma: *mut host1x_cdma) {
    static void timeout_release_mlock(struct host1x_cdma *cdma)
    {

// Tegra186 and Tegra194 require a more complicated MLOCK release
// sequence. Furthermore, those chips by default don't enforce MLOCKs,
// so it turns out that if we don't /actually/ need MLOCKs, we can just
// ignore them.
//
// As such, for now just implement this on Tegra234 and above where things
// are stricter but also easy to implement.
//
    struct host1x_channel *ch = cdma_to_channel(cdma);
    struct host1x *host1x = cdma_to_host1x(cdma);
    u32 offset;
    switch (ch.client.class) {
    case HOST1X_CLASS_VIC:
    offset = HOST1X_COMMON_VIC_MLOCK;
    break;

    case HOST1X_CLASS_NVJPG1:
    offset = HOST1X_COMMON_NVJPG1_MLOCK;
    break;
    case HOST1X_CLASS_NVENC:
    offset = HOST1X_COMMON_NVENC_MLOCK;
    break;
    case HOST1X_CLASS_NVJPG:
    offset = HOST1X_COMMON_NVJPG_MLOCK;
    break;
    case HOST1X_CLASS_NVDEC:
    offset = HOST1X_COMMON_NVDEC_MLOCK;
    break;
    case HOST1X_CLASS_OFA:
    offset = HOST1X_COMMON_OFA_MLOCK;
    break;

    default:
    WARN(1, "%s was not updated for class %u", __func__, ch.client.class);
    return;
    }
    host1x_common_writel(host1x, 0x0, offset);

    }
//
// If this timeout fires, it indicates the current sync_queue entry has
// exceeded its TTL and the userctx should be timed out and remaining
// submits already issued cleaned up (future submits return an error).
//
#[no_mangle]
unsafe extern "C" fn cdma_timeout_handler(work: *mut work_struct) {
    static void cdma_timeout_handler(struct work_struct *work)
    {
    u32 syncpt_val;
    struct host1x_cdma *cdma;
    struct host1x *host1x;
    struct host1x_channel *ch;
    cdma = container_of(to_delayed_work(work), struct host1x_cdma,
    timeout.wq);
    host1x = cdma_to_host1x(cdma);
    ch = cdma_to_channel(cdma);
    host1x_debug_dump(cdma_to_host1x(cdma));
    mutex_lock(&cdma.lock);
    if (!cdma.timeout.client) {
    dev_dbg(host1x.dev,
    "cdma_timeout: expired, but has no clientid\n");
    mutex_unlock(&cdma.lock);
    return;
    }
// stop processing to get a clean snapshot
    cdma_hw_cmdproc_stop(host1x, ch, true);
    syncpt_val = host1x_syncpt_load(cdma.timeout.syncpt);
// has buffer actually completed?
    if ((s32)(syncpt_val - cdma.timeout.syncpt_val) >= 0) {
    dev_dbg(host1x.dev,
    "cdma_timeout: expired, but buffer had completed\n");
// restore
    cdma_hw_cmdproc_stop(host1x, ch, false);
    mutex_unlock(&cdma.lock);
    return;
    }
    dev_warn(host1x.dev, "%s: timeout: %u (%s), HW thresh %d, done %d\n",
    __func__, cdma.timeout.syncpt.id, cdma.timeout.syncpt.name,
    syncpt_val, cdma.timeout.syncpt_val);
// stop HW, resetting channel/module
    host1x_hw_cdma_freeze(host1x, cdma);
// release any held MLOCK
    timeout_release_mlock(cdma);
    host1x_cdma_update_sync_queue(cdma, ch.dev);
    mutex_unlock(&cdma.lock);
    }
//
// Init timeout resources
//
#[no_mangle]
unsafe extern "C" fn cdma_timeout_init(cdma: *mut host1x_cdma) -> c_int {
    static int cdma_timeout_init(struct host1x_cdma *cdma)
    {
    INIT_DELAYED_WORK(&cdma.timeout.wq, cdma_timeout_handler);
    cdma.timeout.initialized = true;
    return 0;
    }
//
// Clean up timeout resources
//
#[no_mangle]
unsafe extern "C" fn cdma_timeout_destroy(cdma: *mut host1x_cdma) {
    static void cdma_timeout_destroy(struct host1x_cdma *cdma)
    {
    if (cdma.timeout.initialized)
    cancel_delayed_work_sync(&cdma.timeout.wq);
    cdma.timeout.initialized = false;
    }
    static const struct host1x_cdma_ops host1x_cdma_ops = {
    .start = cdma_start,
    .stop = cdma_stop,
    .flush = cdma_flush,
    .timeout_init = cdma_timeout_init,
    .timeout_destroy = cdma_timeout_destroy,
    .freeze = cdma_freeze,
    .resume = cdma_resume,
    .timeout_cpu_incr = cdma_timeout_cpu_incr,
    };
    static const struct host1x_pushbuffer_ops host1x_pushbuffer_ops = {
    .init = push_buffer_init,
    };
