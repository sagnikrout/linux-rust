//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/hw/debug_hw_1x06.c
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
// Copyright (C) 2010 Google, Inc.
// Author: Erik Gilling <konkers@android.com>
//
// Copyright (C) 2011-2017 NVIDIA Corporation
//

    static void host1x_debug_show_channel_cdma(struct host1x *host,
    struct host1x_channel *ch,
    struct output *o)
    {
    struct host1x_cdma *cdma = &ch.cdma;
    let mut dmastart: dma_addr_t = 0, dmaend = 0;
    u32 dmaput, dmaget, dmactrl;
    u32 offset, class;
    u32 ch_stat;

    dmastart = host1x_ch_readl(ch, HOST1X_CHANNEL_DMASTART_HI);
    dmastart <<= 32;

    dmastart |= host1x_ch_readl(ch, HOST1X_CHANNEL_DMASTART);

    dmaend = host1x_ch_readl(ch, HOST1X_CHANNEL_DMAEND_HI);
    dmaend <<= 32;

    dmaend |= host1x_ch_readl(ch, HOST1X_CHANNEL_DMAEND);
    dmaput = host1x_ch_readl(ch, HOST1X_CHANNEL_DMAPUT);
    dmaget = host1x_ch_readl(ch, HOST1X_CHANNEL_DMAGET);
    dmactrl = host1x_ch_readl(ch, HOST1X_CHANNEL_DMACTRL);
    offset = host1x_ch_readl(ch, HOST1X_CHANNEL_CMDP_OFFSET);
    class = host1x_ch_readl(ch, HOST1X_CHANNEL_CMDP_CLASS);
    ch_stat = host1x_ch_readl(ch, HOST1X_CHANNEL_CHANNELSTAT);
    host1x_debug_output(o, "%u-%s: ", ch.id, dev_name(ch.dev));
    if (dmactrl & HOST1X_CHANNEL_DMACTRL_DMASTOP ||
    !ch.cdma.push_buffer.mapped) {
    host1x_debug_output(o, "inactive\n\n");
    return;
    }
    if (class == HOST1X_CLASS_HOST1X && offset == HOST1X_UCLASS_WAIT_SYNCPT)
    host1x_debug_output(o, "waiting on syncpt\n");
    else
    host1x_debug_output(o, "active class %02x, offset %04x\n",
    class, offset);
    host1x_debug_output(o, "DMASTART %pad, DMAEND %pad\n", &dmastart, &dmaend);
    host1x_debug_output(o, "DMAPUT %08x DMAGET %08x DMACTL %08x\n",
    dmaput, dmaget, dmactrl);
    host1x_debug_output(o, "CHANNELSTAT %02x\n", ch_stat);
    show_channel_gathers(o, cdma);
    host1x_debug_output(o, "\n");
    }
    static void host1x_debug_show_channel_fifo(struct host1x *host,
    struct host1x_channel *ch,
    struct output *o)
    {

    u32 rd_ptr, wr_ptr, start, end;
    let mut payload: u32 = INVALID_PAYLOAD;
    let mut data_count: c_uint = 0;

    u32 val;
    host1x_debug_output(o, "%u: fifo:\n", ch.id);
    val = host1x_ch_readl(ch, HOST1X_CHANNEL_CMDFIFO_STAT);
    host1x_debug_output(o, "CMDFIFO_STAT %08x\n", val);
    if (val & HOST1X_CHANNEL_CMDFIFO_STAT_EMPTY) {
    host1x_debug_output(o, "[empty]\n");
    return;
    }
    val = host1x_ch_readl(ch, HOST1X_CHANNEL_CMDFIFO_RDATA);
    host1x_debug_output(o, "CMDFIFO_RDATA %08x\n", val);

// Peek pointer values are invalid during SLCG, so disable it
    host1x_hypervisor_writel(host, 0x1, HOST1X_HV_ICG_EN_OVERRIDE);
    val = 0;
    val |= HOST1X_HV_CMDFIFO_PEEK_CTRL_ENABLE;
    val |= HOST1X_HV_CMDFIFO_PEEK_CTRL_CHANNEL(ch.id);
    host1x_hypervisor_writel(host, val, HOST1X_HV_CMDFIFO_PEEK_CTRL);
    val = host1x_hypervisor_readl(host, HOST1X_HV_CMDFIFO_PEEK_PTRS);
    rd_ptr = HOST1X_HV_CMDFIFO_PEEK_PTRS_RD_PTR_V(val);
    wr_ptr = HOST1X_HV_CMDFIFO_PEEK_PTRS_WR_PTR_V(val);
    val = host1x_hypervisor_readl(host, HOST1X_HV_CMDFIFO_SETUP(ch.id));
    start = HOST1X_HV_CMDFIFO_SETUP_BASE_V(val);
    end = HOST1X_HV_CMDFIFO_SETUP_LIMIT_V(val);
    do {
    val = 0;
    val |= HOST1X_HV_CMDFIFO_PEEK_CTRL_ENABLE;
    val |= HOST1X_HV_CMDFIFO_PEEK_CTRL_CHANNEL(ch.id);
    val |= HOST1X_HV_CMDFIFO_PEEK_CTRL_ADDR(rd_ptr);
    host1x_hypervisor_writel(host, val,
    HOST1X_HV_CMDFIFO_PEEK_CTRL);
    val = host1x_hypervisor_readl(host,
    HOST1X_HV_CMDFIFO_PEEK_READ);
    if (!data_count) {
    host1x_debug_output(o, "%03x 0x%08x: ",
    rd_ptr - start, val);
    data_count = show_channel_command(o, val, &payload);
    } else {
    host1x_debug_cont(o, "%08x%s", val,
    data_count > 1 ? ", " : "])\n");
    data_count--;
    }
    if (rd_ptr == end)
    rd_ptr = start;
    else
    rd_ptr++;
    } while (rd_ptr != wr_ptr);
    if (data_count)
    host1x_debug_cont(o, ", ...])\n");
    host1x_debug_output(o, "\n");
    host1x_hypervisor_writel(host, 0x0, HOST1X_HV_CMDFIFO_PEEK_CTRL);
    host1x_hypervisor_writel(host, 0x0, HOST1X_HV_ICG_EN_OVERRIDE);

    }
#[no_mangle]
unsafe extern "C" fn host1x_debug_show_mlocks(host: *mut host1x, o: *mut output) {
    static void host1x_debug_show_mlocks(struct host1x *host, struct output *o)
    {
// TODO
    }
