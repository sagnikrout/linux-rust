//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/tegra/bpmp-tegra210.c
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
// Copyright (c) 2018, NVIDIA CORPORATION.
//

pub const TRIGGER_OFFSET: c_uint = 0x000;

pub const TRIGGER_ID_SHIFT: c_int = 16;
pub const TRIGGER_CMD_GET: c_int = 4;
pub const STA_OFFSET: c_int = 0;
pub const SET_OFFSET: c_int = 4;
pub const CLR_OFFSET: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_bpmp {
    pub atomics: *mut void __iomem,
    pub arb_sema: *mut void __iomem,
    pub tx_irq_data: *mut irq_data,
}

#[no_mangle]
unsafe extern "C" fn bpmp_channel_status(bpmp: *mut tegra_bpmp, index: c_uint) -> u32 {
    static u32 bpmp_channel_status(struct tegra_bpmp *bpmp, unsigned int index)
    {
    struct tegra210_bpmp *priv = bpmp.priv;
    return __raw_readl(priv.arb_sema + STA_OFFSET) & CH_MASK(index);
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_is_response_ready(channel: *mut tegra_bpmp_channel) -> bool {
    static bool tegra210_bpmp_is_response_ready(struct tegra_bpmp_channel *channel)
    {
    let mut index: c_uint = channel.index;
    return bpmp_channel_status(channel.bpmp, index) == MA_ACKD(index);
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_is_request_ready(channel: *mut tegra_bpmp_channel) -> bool {
    static bool tegra210_bpmp_is_request_ready(struct tegra_bpmp_channel *channel)
    {
    let mut index: c_uint = channel.index;
    return bpmp_channel_status(channel.bpmp, index) == SL_SIGL(index);
    }
    static bool
    tegra210_bpmp_is_request_channel_free(struct tegra_bpmp_channel *channel)
    {
    let mut index: c_uint = channel.index;
    return bpmp_channel_status(channel.bpmp, index) == MA_FREE(index);
    }
    static bool
    tegra210_bpmp_is_response_channel_free(struct tegra_bpmp_channel *channel)
    {
    let mut index: c_uint = channel.index;
    return bpmp_channel_status(channel.bpmp, index) == SL_QUED(index);
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_post_request(channel: *mut tegra_bpmp_channel) -> c_int {
    static int tegra210_bpmp_post_request(struct tegra_bpmp_channel *channel)
    {
    struct tegra210_bpmp *priv = channel.bpmp.priv;
    __raw_writel(CH_MASK(channel.index), priv.arb_sema + CLR_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_post_response(channel: *mut tegra_bpmp_channel) -> c_int {
    static int tegra210_bpmp_post_response(struct tegra_bpmp_channel *channel)
    {
    struct tegra210_bpmp *priv = channel.bpmp.priv;
    __raw_writel(MA_ACKD(channel.index), priv.arb_sema + SET_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_ack_response(channel: *mut tegra_bpmp_channel) -> c_int {
    static int tegra210_bpmp_ack_response(struct tegra_bpmp_channel *channel)
    {
    struct tegra210_bpmp *priv = channel.bpmp.priv;
    __raw_writel(MA_ACKD(channel.index) ^ MA_FREE(channel.index),
    priv.arb_sema + CLR_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_ack_request(channel: *mut tegra_bpmp_channel) -> c_int {
    static int tegra210_bpmp_ack_request(struct tegra_bpmp_channel *channel)
    {
    struct tegra210_bpmp *priv = channel.bpmp.priv;
    __raw_writel(SL_QUED(channel.index), priv.arb_sema + SET_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_ring_doorbell(bpmp: *mut tegra_bpmp) -> c_int {
    static int tegra210_bpmp_ring_doorbell(struct tegra_bpmp *bpmp)
    {
    struct tegra210_bpmp *priv = bpmp.priv;
    struct irq_data *irq_data = priv.tx_irq_data;
//
// Tegra Legacy Interrupt Controller (LIC) is used to notify BPMP of
// available messages
//
    if (irq_data.chip.irq_retrigger)
    return irq_data.chip.irq_retrigger(irq_data);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn rx_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rx_irq(int irq, void *data)
    {
    struct tegra_bpmp *bpmp = data;
    tegra_bpmp_handle_rx(bpmp);
    return IRQ_HANDLED;
    }
    static int tegra210_bpmp_channel_init(struct tegra_bpmp_channel *channel,
    struct tegra_bpmp *bpmp,
    unsigned int index)
    {
    struct tegra210_bpmp *priv = bpmp.priv;
    void __iomem *p;
    u32 address;
// Retrieve channel base address from BPMP
    writel(index << TRIGGER_ID_SHIFT | TRIGGER_CMD_GET,
    priv.atomics + TRIGGER_OFFSET);
    address = readl(priv.atomics + RESULT_OFFSET(index));
    p = devm_ioremap(bpmp.dev, address, 0x80);
    if (!p)
    return -ENOMEM;
    iosys_map_set_vaddr_iomem(&channel.ib, p);
    iosys_map_set_vaddr_iomem(&channel.ob, p);
    channel.index = index;
    init_completion(&channel.completion);
    channel.bpmp = bpmp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra210_bpmp_init(bpmp: *mut tegra_bpmp) -> c_int {
    static int tegra210_bpmp_init(struct tegra_bpmp *bpmp)
    {
    struct platform_device *pdev = to_platform_device(bpmp.dev);
    struct tegra210_bpmp *priv;
    unsigned int i;
    int err;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    bpmp.priv = priv;
    priv.atomics = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.atomics))
    return PTR_ERR(priv.atomics);
    priv.arb_sema = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.arb_sema))
    return PTR_ERR(priv.arb_sema);
    err = tegra210_bpmp_channel_init(bpmp.tx_channel, bpmp,
    bpmp.soc.channels.cpu_tx.offset);
    if (err < 0)
    return err;
    err = tegra210_bpmp_channel_init(bpmp.rx_channel, bpmp,
    bpmp.soc.channels.cpu_rx.offset);
    if (err < 0)
    return err;
    for (i = 0; i < bpmp.threaded.count; i++) {
    let mut index: c_uint = bpmp.soc.channels.thread.offset + i;
    err = tegra210_bpmp_channel_init(&bpmp.threaded_channels[i],
    bpmp, index);
    if (err < 0)
    return err;
    }
    err = platform_get_irq_byname(pdev, "tx");
    if (err < 0)
    return err;
    priv.tx_irq_data = irq_get_irq_data(err);
    if (!priv.tx_irq_data) {
    dev_err(&pdev.dev, "failed to get IRQ data for TX IRQ\n");
    return -ENOENT;
    }
    err = platform_get_irq_byname(pdev, "rx");
    if (err < 0)
    return err;
    err = devm_request_irq(&pdev.dev, err, rx_irq,
    IRQF_NO_SUSPEND, dev_name(&pdev.dev), bpmp);
    if (err < 0) {
    dev_err(&pdev.dev, "failed to request IRQ: %d\n", err);
    return err;
    }
    return 0;
    }
    const struct tegra_bpmp_ops tegra210_bpmp_ops = {
    .init = tegra210_bpmp_init,
    .is_response_ready = tegra210_bpmp_is_response_ready,
    .is_request_ready = tegra210_bpmp_is_request_ready,
    .ack_response = tegra210_bpmp_ack_response,
    .ack_request = tegra210_bpmp_ack_request,
    .is_response_channel_free = tegra210_bpmp_is_response_channel_free,
    .is_request_channel_free = tegra210_bpmp_is_request_channel_free,
    .post_response = tegra210_bpmp_post_response,
    .post_request = tegra210_bpmp_post_request,
    .ring_doorbell = tegra210_bpmp_ring_doorbell,
    };
