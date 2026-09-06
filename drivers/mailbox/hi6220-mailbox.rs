//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/hi6220-mailbox.c
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
// Hisilicon's Hi6220 mailbox driver
//
// Copyright (c) 2015 HiSilicon Limited.
// Copyright (c) 2015 Linaro Limited.
//
// Author: Leo Yan <leo.yan@linaro.org>
//

pub const MBOX_CHAN_MAX: c_int = 32;
pub const MBOX_TX: c_uint = 0x1;
// Mailbox message length: 8 words
pub const MBOX_MSG_LEN: c_int = 8;
// Mailbox Registers

// IPC registers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_mbox_chan {
//
// Description for channel's hardware info:
// - direction: tx or rx
// - dst irq: peer core's irq number
// - ack irq: local irq number
// - slot number
//
    pub ack_irq: unsigned int dir, dst_irq,,
    pub slot: c_uint,
    pub parent: *mut hi6220_mbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_mbox {
    pub dev: *mut device,
    pub irq: c_int,
// flag of enabling tx's irq mode
    pub tx_irq_mode: bool,
// region for ipc event
    pub ipc: *mut void __iomem,
// region for mailbox
    pub base: *mut void __iomem,
    pub irq_map_chan: [*mut c_void; MBOX_CHAN_MAX],
    pub chan: *mut mbox_chan,
    pub controller: mbox_controller,
    pub chan_num: c_uint,
    pub __counted_by(chan_num): hi6220_mbox_chan mchan[],
}

    static void mbox_set_state(struct hi6220_mbox *mbox,
    unsigned int slot, u32 val)
    {
    u32 status;
    status = readl(mbox.base + MBOX_MODE_REG(slot));
    status = (status & ~MBOX_STATE_MASK) | val;
    writel(status, mbox.base + MBOX_MODE_REG(slot));
    }
    static void mbox_set_mode(struct hi6220_mbox *mbox,
    unsigned int slot, u32 val)
    {
    u32 mode;
    mode = readl(mbox.base + MBOX_MODE_REG(slot));
    mode = (mode & ~MBOX_ACK_CONFIG_MASK) | val;
    writel(mode, mbox.base + MBOX_MODE_REG(slot));
    }
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool hi6220_mbox_last_tx_done(struct mbox_chan *chan)
    {
    struct hi6220_mbox_chan *mchan = chan.con_priv;
    struct hi6220_mbox *mbox = mchan.parent;
    u32 state;
// Only set idle state for polling mode
    BUG_ON(mbox.tx_irq_mode);
    state = readl(mbox.base + MBOX_MODE_REG(mchan.slot));
    return ((state & MBOX_STATE_MASK) == MBOX_STATE_IDLE);
    }
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_send_data(chan: *mut mbox_chan, msg: *mut c_void) -> c_int {
    static int hi6220_mbox_send_data(struct mbox_chan *chan, void *msg)
    {
    struct hi6220_mbox_chan *mchan = chan.con_priv;
    struct hi6220_mbox *mbox = mchan.parent;
    let mut slot: c_uint = mchan.slot;
    u32 *buf = msg;
    int i;
// indicate as a TX channel
    mchan.dir = MBOX_TX;
    mbox_set_state(mbox, slot, MBOX_STATE_TX);
    if (mbox.tx_irq_mode)
    mbox_set_mode(mbox, slot, MBOX_ACK_IRQ);
    else
    mbox_set_mode(mbox, slot, MBOX_ACK_AUTOMATIC);
    for (i = 0; i < MBOX_MSG_LEN; i++)
    writel(buf[i], mbox.base + MBOX_DATA_REG(slot) + i * 4);
// trigger remote request
    writel(BIT(mchan.dst_irq), DST_INT_RAW_REG(mbox.ipc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_interrupt(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi6220_mbox_interrupt(int irq, void *p)
    {
    struct hi6220_mbox *mbox = p;
    struct hi6220_mbox_chan *mchan;
    struct mbox_chan *chan;
    unsigned int state, intr_bit, i;
    u32 msg[MBOX_MSG_LEN];
    state = readl(ACK_INT_STAT_REG(mbox.ipc));
    if (!state) {
    dev_warn(mbox.dev, "%s: spurious interrupt\n",
    __func__);
    return IRQ_HANDLED;
    }
    while (state) {
    intr_bit = __ffs(state);
    state &= (state - 1);
    chan = mbox.irq_map_chan[intr_bit];
    if (!chan) {
    dev_warn(mbox.dev, "%s: unexpected irq vector %d\n",
    __func__, intr_bit);
    continue;
    }
    mchan = chan.con_priv;
    if (mchan.dir == MBOX_TX)
    mbox_chan_txdone(chan, 0);
    else {
    for (i = 0; i < MBOX_MSG_LEN; i++)
    msg[i] = readl(mbox.base +
    MBOX_DATA_REG(mchan.slot) + i * 4);
    mbox_chan_received_data(chan, (void *)msg);
    }
// clear IRQ source
    writel(BIT(mchan.ack_irq), ACK_INT_CLR_REG(mbox.ipc));
    mbox_set_state(mbox, mchan.slot, MBOX_STATE_IDLE);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int hi6220_mbox_startup(struct mbox_chan *chan)
    {
    struct hi6220_mbox_chan *mchan = chan.con_priv;
    struct hi6220_mbox *mbox = mchan.parent;
    mchan.dir = 0;
// enable interrupt
    writel(BIT(mchan.ack_irq), ACK_INT_ENA_REG(mbox.ipc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_shutdown(chan: *mut mbox_chan) {
    static void hi6220_mbox_shutdown(struct mbox_chan *chan)
    {
    struct hi6220_mbox_chan *mchan = chan.con_priv;
    struct hi6220_mbox *mbox = mchan.parent;
// disable interrupt
    writel(BIT(mchan.ack_irq), ACK_INT_DIS_REG(mbox.ipc));
    mbox.irq_map_chan[mchan.ack_irq] = core::ptr::null_mut();
    }
    static const struct mbox_chan_ops hi6220_mbox_ops = {
    .send_data    = hi6220_mbox_send_data,
    .startup      = hi6220_mbox_startup,
    .shutdown     = hi6220_mbox_shutdown,
    .last_tx_done = hi6220_mbox_last_tx_done,
    };
    static struct mbox_chan *hi6220_mbox_xlate(struct mbox_controller *controller,
    const struct of_phandle_args *spec)
    {
    struct hi6220_mbox *mbox = dev_get_drvdata(controller.dev);
    struct hi6220_mbox_chan *mchan;
    struct mbox_chan *chan;
    let mut i: c_uint = spec.args[0];
    let mut dst_irq: c_uint = spec.args[1];
    let mut ack_irq: c_uint = spec.args[2];
// Bounds checking
    if (i >= mbox.chan_num || dst_irq >= mbox.chan_num ||
    ack_irq >= mbox.chan_num) {
    dev_err(mbox.dev,
    "Invalid channel idx %d dst_irq %d ack_irq %d\n",
    i, dst_irq, ack_irq);
    return ERR_PTR(-EINVAL);
    }
// Is requested channel free?
    chan = &mbox.chan[i];
    if (mbox.irq_map_chan[ack_irq] == (void *)chan) {
    dev_err(mbox.dev, "Channel in use\n");
    return ERR_PTR(-EBUSY);
    }
    mchan = chan.con_priv;
    mchan.dst_irq = dst_irq;
    mchan.ack_irq = ack_irq;
    mbox.irq_map_chan[ack_irq] = (void *)chan;
    return chan;
    }
    static const struct of_device_id hi6220_mbox_of_match[] = {
    { .compatible = "hisilicon,hi6220-mbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, hi6220_mbox_of_match);
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int hi6220_mbox_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct hi6220_mbox *mbox;
    int i, err;
    mbox = devm_kzalloc(dev, struct_size(mbox, mchan, MBOX_CHAN_MAX), GFP_KERNEL);
    if (!mbox)
    return -ENOMEM;
    mbox.chan_num = MBOX_CHAN_MAX;
    mbox.dev = dev;
    mbox.chan = devm_kcalloc(dev,
    mbox.chan_num, sizeof(*mbox.chan), GFP_KERNEL);
    if (!mbox.chan)
    return -ENOMEM;
    mbox.irq = platform_get_irq(pdev, 0);
    if (mbox.irq < 0)
    return mbox.irq;
    mbox.ipc = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mbox.ipc)) {
    dev_err(dev, "ioremap ipc failed\n");
    return PTR_ERR(mbox.ipc);
    }
    mbox.base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(mbox.base)) {
    dev_err(dev, "ioremap buffer failed\n");
    return PTR_ERR(mbox.base);
    }
    err = devm_request_irq(dev, mbox.irq, hi6220_mbox_interrupt, 0,
    dev_name(dev), mbox);
    if (err)
    return -ENODEV;
    mbox.controller.dev = dev;
    mbox.controller.chans = &mbox.chan[0];
    mbox.controller.num_chans = mbox.chan_num;
    mbox.controller.ops = &hi6220_mbox_ops;
    mbox.controller.of_xlate = hi6220_mbox_xlate;
    for (i = 0; i < mbox.chan_num; i++) {
    mbox.chan[i].con_priv = &mbox.mchan[i];
    mbox.irq_map_chan[i] = core::ptr::null_mut();
    mbox.mchan[i].parent = mbox;
    mbox.mchan[i].slot   = i;
    }
// mask and clear all interrupt vectors
    writel(0x0,  ACK_INT_MSK_REG(mbox.ipc));
    writel(~0x0, ACK_INT_CLR_REG(mbox.ipc));
// use interrupt for tx's ack
    mbox.tx_irq_mode = !of_property_read_bool(node, "hi6220,mbox-tx-noirq");
    if (mbox.tx_irq_mode)
    mbox.controller.txdone_irq = true;
    else {
    mbox.controller.txdone_poll = true;
    mbox.controller.txpoll_period = 5;
    }
    err = devm_mbox_controller_register(dev, &mbox.controller);
    if (err) {
    dev_err(dev, "Failed to register mailbox %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, mbox);
    dev_info(dev, "Mailbox enabled\n");
    return 0;
    }
    static struct platform_driver hi6220_mbox_driver = {
    .driver = {
    .name = "hi6220-mbox",
    .of_match_table = hi6220_mbox_of_match,
    },
    .probe	= hi6220_mbox_probe,
    };
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_init() -> int __init {
    static int __init hi6220_mbox_init(void)
    {
    return platform_driver_register(&hi6220_mbox_driver);
    }
    core_initcall(hi6220_mbox_init);
#[no_mangle]
unsafe extern "C" fn hi6220_mbox_exit() -> void __exit {
    static void __exit hi6220_mbox_exit(void)
    {
    platform_driver_unregister(&hi6220_mbox_driver);
    }
    module_exit(hi6220_mbox_exit);
    MODULE_AUTHOR("Leo Yan <leo.yan@linaro.org>");
    MODULE_DESCRIPTION("Hi6220 mailbox driver");
    MODULE_LICENSE("GPL v2");
