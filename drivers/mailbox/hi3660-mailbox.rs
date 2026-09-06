//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/hi3660-mailbox.c
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
// Copyright (c) 2017-2018 HiSilicon Limited.
// Copyright (c) 2017-2018 Linaro Limited.

pub const MBOX_CHAN_MAX: c_int = 32;
pub const MBOX_RX: c_uint = 0x0;
pub const MBOX_TX: c_uint = 0x1;

pub const MBOX_SRC_REG: c_uint = 0x00;
pub const MBOX_DST_REG: c_uint = 0x04;
pub const MBOX_DCLR_REG: c_uint = 0x08;
pub const MBOX_DSTAT_REG: c_uint = 0x0c;
pub const MBOX_MODE_REG: c_uint = 0x10;
pub const MBOX_IMASK_REG: c_uint = 0x14;
pub const MBOX_ICLR_REG: c_uint = 0x18;
pub const MBOX_SEND_REG: c_uint = 0x1c;
pub const MBOX_DATA_REG: c_uint = 0x20;
pub const MBOX_IPC_LOCK_REG: c_uint = 0xa00;
pub const MBOX_IPC_UNLOCK: c_uint = 0x1acce551;
pub const MBOX_AUTOMATIC_ACK: c_int = 1;

pub const MBOX_MSG_LEN: c_int = 8;
//
// struct hi3660_chan_info - Hi3660 mailbox channel information
// @dst_irq:	Interrupt vector for remote processor
// @ack_irq:	Interrupt vector for local processor
//
// A channel can be used for TX or RX, it can trigger remote
// processor interrupt to notify remote processor and can receive
// interrupt if it has an incoming message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_chan_info {
    pub dst_irq: c_uint,
    pub ack_irq: c_uint,
}

//
// struct hi3660_mbox - Hi3660 mailbox controller data
// @dev:	Device to which it is attached
// @base:	Base address of the register mapping region
// @chan:	Representation of channels in mailbox controller
// @mchan:	Representation of channel info
// @controller:	Representation of a communication channel controller
//
// Mailbox controller includes 32 channels and can allocate
// channel for message transferring.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_mbox {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub chan: [mbox_chan; MBOX_CHAN_MAX],
    pub mchan: [hi3660_chan_info; MBOX_CHAN_MAX],
    pub controller: mbox_controller,
}

    static struct hi3660_mbox *to_hi3660_mbox(struct mbox_controller *mbox)
    {
    return container_of(mbox, struct hi3660_mbox, controller);
    }
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_check_state(chan: *mut mbox_chan) -> c_int {
    static int hi3660_mbox_check_state(struct mbox_chan *chan)
    {
    let mut ch: c_ulong = (unsigned long)chan.con_priv;
    struct hi3660_mbox *mbox = to_hi3660_mbox(chan.mbox);
    struct hi3660_chan_info *mchan = &mbox.mchan[ch];
    void __iomem *base = MBOX_BASE(mbox, ch);
    unsigned long val;
    unsigned int ret;
// Mailbox is ready to use
    if (readl(base + MBOX_MODE_REG) & MBOX_STATE_READY)
    return 0;
// Wait for acknowledge from remote
    ret = readx_poll_timeout_atomic(readl, base + MBOX_MODE_REG,
    val, (val & MBOX_STATE_ACK), 1000, 300000);
    if (ret) {
    dev_err(mbox.dev, "%s: timeout for receiving ack\n", __func__);
    return ret;
    }
// clear ack state, mailbox will get back to ready state
    writel(BIT(mchan.ack_irq), base + MBOX_ICLR_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_unlock(chan: *mut mbox_chan) -> c_int {
    static int hi3660_mbox_unlock(struct mbox_chan *chan)
    {
    struct hi3660_mbox *mbox = to_hi3660_mbox(chan.mbox);
    unsigned int val, retry = 3;
    do {
    writel(MBOX_IPC_UNLOCK, mbox.base + MBOX_IPC_LOCK_REG);
    val = readl(mbox.base + MBOX_IPC_LOCK_REG);
    if (!val)
    break;
    udelay(10);
    } while (retry--);
    if (val)
    dev_err(mbox.dev, "%s: failed to unlock mailbox\n", __func__);
    return (!val) ? 0 : -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_acquire_channel(chan: *mut mbox_chan) -> c_int {
    static int hi3660_mbox_acquire_channel(struct mbox_chan *chan)
    {
    let mut ch: c_ulong = (unsigned long)chan.con_priv;
    struct hi3660_mbox *mbox = to_hi3660_mbox(chan.mbox);
    struct hi3660_chan_info *mchan = &mbox.mchan[ch];
    void __iomem *base = MBOX_BASE(mbox, ch);
    unsigned int val, retry;
    for (retry = 10; retry; retry--) {
// Check if channel is in idle state
    if (readl(base + MBOX_MODE_REG) & MBOX_STATE_IDLE) {
    writel(BIT(mchan.ack_irq), base + MBOX_SRC_REG);
// Check ack bit has been set successfully
    val = readl(base + MBOX_SRC_REG);
    if (val & BIT(mchan.ack_irq))
    break;
    }
    }
    if (!retry)
    dev_err(mbox.dev, "%s: failed to acquire channel\n", __func__);
    return retry ? 0 : -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int hi3660_mbox_startup(struct mbox_chan *chan)
    {
    int ret;
    ret = hi3660_mbox_unlock(chan);
    if (ret)
    return ret;
    ret = hi3660_mbox_acquire_channel(chan);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_send_data(chan: *mut mbox_chan, msg: *mut c_void) -> c_int {
    static int hi3660_mbox_send_data(struct mbox_chan *chan, void *msg)
    {
    let mut ch: c_ulong = (unsigned long)chan.con_priv;
    struct hi3660_mbox *mbox = to_hi3660_mbox(chan.mbox);
    struct hi3660_chan_info *mchan = &mbox.mchan[ch];
    void __iomem *base = MBOX_BASE(mbox, ch);
    u32 *buf = msg;
    unsigned int i;
    int ret;
    ret = hi3660_mbox_check_state(chan);
    if (ret)
    return ret;
// Clear mask for destination interrupt
    writel_relaxed(~BIT(mchan.dst_irq), base + MBOX_IMASK_REG);
// Config destination for interrupt vector
    writel_relaxed(BIT(mchan.dst_irq), base + MBOX_DST_REG);
// Automatic acknowledge mode
    writel_relaxed(MBOX_AUTOMATIC_ACK, base + MBOX_MODE_REG);
// Fill message data
    for (i = 0; i < MBOX_MSG_LEN; i++)
    writel_relaxed(buf[i], base + MBOX_DATA_REG + i * 4);
// Trigger data transferring
    writel(BIT(mchan.ack_irq), base + MBOX_SEND_REG);
    return 0;
    }
    static const struct mbox_chan_ops hi3660_mbox_ops = {
    .startup	= hi3660_mbox_startup,
    .send_data	= hi3660_mbox_send_data,
    };
    static struct mbox_chan *hi3660_mbox_xlate(struct mbox_controller *controller,
    const struct of_phandle_args *spec)
    {
    struct hi3660_mbox *mbox = to_hi3660_mbox(controller);
    struct hi3660_chan_info *mchan;
    let mut ch: c_uint = spec.args[0];
    if (ch >= MBOX_CHAN_MAX) {
    dev_err(mbox.dev, "Invalid channel idx %d\n", ch);
    return ERR_PTR(-EINVAL);
    }
    mchan = &mbox.mchan[ch];
    mchan.dst_irq = spec.args[1];
    mchan.ack_irq = spec.args[2];
    return &mbox.chan[ch];
    }
    static const struct of_device_id hi3660_mbox_of_match[] = {
    { .compatible = "hisilicon,hi3660-mbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, hi3660_mbox_of_match);
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int hi3660_mbox_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct hi3660_mbox *mbox;
    struct mbox_chan *chan;
    unsigned long ch;
    int err;
    mbox = devm_kzalloc(dev, sizeof(*mbox), GFP_KERNEL);
    if (!mbox)
    return -ENOMEM;
    mbox.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mbox.base))
    return PTR_ERR(mbox.base);
    mbox.dev = dev;
    mbox.controller.dev = dev;
    mbox.controller.chans = mbox.chan;
    mbox.controller.num_chans = MBOX_CHAN_MAX;
    mbox.controller.ops = &hi3660_mbox_ops;
    mbox.controller.of_xlate = hi3660_mbox_xlate;
// Initialize mailbox channel data
    chan = mbox.chan;
    for (ch = 0; ch < MBOX_CHAN_MAX; ch++)
    chan[ch].con_priv = (void *)ch;
    err = devm_mbox_controller_register(dev, &mbox.controller);
    if (err) {
    dev_err(dev, "Failed to register mailbox %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, mbox);
    dev_info(dev, "Mailbox enabled\n");
    return 0;
    }
    static struct platform_driver hi3660_mbox_driver = {
    .probe  = hi3660_mbox_probe,
    .driver = {
    .name = "hi3660-mbox",
    .of_match_table = hi3660_mbox_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_init() -> int __init {
    static int __init hi3660_mbox_init(void)
    {
    return platform_driver_register(&hi3660_mbox_driver);
    }
    core_initcall(hi3660_mbox_init);
#[no_mangle]
unsafe extern "C" fn hi3660_mbox_exit() -> void __exit {
    static void __exit hi3660_mbox_exit(void)
    {
    platform_driver_unregister(&hi3660_mbox_driver);
    }
    module_exit(hi3660_mbox_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Hisilicon Hi3660 Mailbox Controller");
    MODULE_AUTHOR("Leo Yan <leo.yan@linaro.org>");
