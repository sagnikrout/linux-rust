//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/sun6i-msgbox.c
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
// Copyright (c) 2017-2019 Samuel Holland <samuel@sholland.org>

pub const NUM_CHANS: c_int = 8;

pub const REMOTE_IRQ_EN_REG: c_uint = 0x0040;
pub const REMOTE_IRQ_STAT_REG: c_uint = 0x0050;
pub const LOCAL_IRQ_EN_REG: c_uint = 0x0060;
pub const LOCAL_IRQ_STAT_REG: c_uint = 0x0070;

pub const RX_IRQ_MASK: c_uint = 0x5555;

pub const TX_IRQ_MASK: c_uint = 0xaaaa;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_msgbox {
    pub controller: mbox_controller,
    pub clk: *mut clk,
    pub lock: spinlock_t,
    pub regs: *mut void __iomem,
}

    static bool sun6i_msgbox_last_tx_done(struct mbox_chan *chan);
    static bool sun6i_msgbox_peek_data(struct mbox_chan *chan);
#[no_mangle]
pub unsafe extern "C" fn channel_number(chan: *mut mbox_chan) -> c_int {
    static inline int channel_number(struct mbox_chan *chan)
    {
    return chan - chan.mbox.chans;
    }
    static inline struct sun6i_msgbox *to_sun6i_msgbox(struct mbox_chan *chan)
    {
    return chan.con_priv;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun6i_msgbox_irq(int irq, void *dev_id)
    {
    struct sun6i_msgbox *mbox = dev_id;
    uint32_t status;
    int n;
// Only examine channels that are currently enabled.
    status = readl(mbox.regs + LOCAL_IRQ_EN_REG) &
    readl(mbox.regs + LOCAL_IRQ_STAT_REG);
    if (!(status & RX_IRQ_MASK))
    return IRQ_NONE;
    for (n = 0; n < NUM_CHANS; ++n) {
    struct mbox_chan *chan = &mbox.controller.chans[n];
    if (!(status & RX_IRQ(n)))
    continue;
    while (sun6i_msgbox_peek_data(chan)) {
    let mut msg: u32 = readl(mbox.regs + MSG_DATA_REG(n));
    mbox_dbg(mbox, "Channel %d received 0x%08x\n", n, msg);
    mbox_chan_received_data(chan, &msg);
    }
// The IRQ can be cleared only once the FIFO is empty.
    writel(RX_IRQ(n), mbox.regs + LOCAL_IRQ_STAT_REG);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int sun6i_msgbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct sun6i_msgbox *mbox = to_sun6i_msgbox(chan);
    let mut n: c_int = channel_number(chan);
    let mut msg: u32 = *(uint32_t *)data;
// Using a channel backwards gets the hardware into a bad state.
    if (WARN_ON_ONCE(!(readl(mbox.regs + CTRL_REG(n)) & CTRL_TX(n))))
    return 0;
    writel(msg, mbox.regs + MSG_DATA_REG(n));
    mbox_dbg(mbox, "Channel %d sent 0x%08x\n", n, msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_startup(chan: *mut mbox_chan) -> c_int {
    static int sun6i_msgbox_startup(struct mbox_chan *chan)
    {
    struct sun6i_msgbox *mbox = to_sun6i_msgbox(chan);
    let mut n: c_int = channel_number(chan);
// The coprocessor is responsible for setting channel directions.
    if (readl(mbox.regs + CTRL_REG(n)) & CTRL_RX(n)) {
// Flush the receive FIFO.
    while (sun6i_msgbox_peek_data(chan))
    readl(mbox.regs + MSG_DATA_REG(n));
    writel(RX_IRQ(n), mbox.regs + LOCAL_IRQ_STAT_REG);
// Enable the receive IRQ.
    spin_lock(&mbox.lock);
    writel(readl(mbox.regs + LOCAL_IRQ_EN_REG) | RX_IRQ(n),
    mbox.regs + LOCAL_IRQ_EN_REG);
    spin_unlock(&mbox.lock);
    }
    mbox_dbg(mbox, "Channel %d startup complete\n", n);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_shutdown(chan: *mut mbox_chan) {
    static void sun6i_msgbox_shutdown(struct mbox_chan *chan)
    {
    struct sun6i_msgbox *mbox = to_sun6i_msgbox(chan);
    let mut n: c_int = channel_number(chan);
    if (readl(mbox.regs + CTRL_REG(n)) & CTRL_RX(n)) {
// Disable the receive IRQ.
    spin_lock(&mbox.lock);
    writel(readl(mbox.regs + LOCAL_IRQ_EN_REG) & ~RX_IRQ(n),
    mbox.regs + LOCAL_IRQ_EN_REG);
    spin_unlock(&mbox.lock);
// Attempt to flush the FIFO until the IRQ is cleared.
    do {
    while (sun6i_msgbox_peek_data(chan))
    readl(mbox.regs + MSG_DATA_REG(n));
    writel(RX_IRQ(n), mbox.regs + LOCAL_IRQ_STAT_REG);
    } while (readl(mbox.regs + LOCAL_IRQ_STAT_REG) & RX_IRQ(n));
    }
    mbox_dbg(mbox, "Channel %d shutdown complete\n", n);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool sun6i_msgbox_last_tx_done(struct mbox_chan *chan)
    {
    struct sun6i_msgbox *mbox = to_sun6i_msgbox(chan);
    let mut n: c_int = channel_number(chan);
//
// The hardware allows snooping on the remote user's IRQ statuses.
// We consider a message to be acknowledged only once the receive IRQ
// for that channel is cleared. Since the receive IRQ for a channel
// cannot be cleared until the FIFO for that channel is empty, this
// ensures that the message has actually been read. It also gives the
// recipient an opportunity to perform minimal processing before
// acknowledging the message.
//
    return !(readl(mbox.regs + REMOTE_IRQ_STAT_REG) & RX_IRQ(n));
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_peek_data(chan: *mut mbox_chan) -> bool {
    static bool sun6i_msgbox_peek_data(struct mbox_chan *chan)
    {
    struct sun6i_msgbox *mbox = to_sun6i_msgbox(chan);
    let mut n: c_int = channel_number(chan);
    return readl(mbox.regs + MSG_STAT_REG(n)) & MSG_STAT_MASK;
    }
    static const struct mbox_chan_ops sun6i_msgbox_chan_ops = {
    .send_data    = sun6i_msgbox_send_data,
    .startup      = sun6i_msgbox_startup,
    .shutdown     = sun6i_msgbox_shutdown,
    .last_tx_done = sun6i_msgbox_last_tx_done,
    .peek_data    = sun6i_msgbox_peek_data,
    };
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_msgbox_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mbox_chan *chans;
    struct reset_control *reset;
    struct sun6i_msgbox *mbox;
    int i, ret;
    mbox = devm_kzalloc(dev, sizeof(*mbox), GFP_KERNEL);
    if (!mbox)
    return -ENOMEM;
    chans = devm_kcalloc(dev, NUM_CHANS, sizeof(*chans), GFP_KERNEL);
    if (!chans)
    return -ENOMEM;
    for (i = 0; i < NUM_CHANS; ++i)
    chans[i].con_priv = mbox;
    mbox.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(mbox.clk)) {
    ret = PTR_ERR(mbox.clk);
    dev_err(dev, "Failed to get clock: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(mbox.clk);
    if (ret) {
    dev_err(dev, "Failed to enable clock: %d\n", ret);
    return ret;
    }
    reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(reset)) {
    ret = PTR_ERR(reset);
    dev_err(dev, "Failed to get reset control: %d\n", ret);
    goto err_disable_unprepare;
    }
//
// NOTE: We rely on platform firmware to preconfigure the channel
// directions, and we share this hardware block with other firmware
// that runs concurrently with Linux (e.g. a trusted monitor).
//
// Therefore, we do *not* assert the reset line if probing fails or
// when removing the device.
//
    ret = reset_control_deassert(reset);
    if (ret) {
    dev_err(dev, "Failed to deassert reset: %d\n", ret);
    goto err_disable_unprepare;
    }
    mbox.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mbox.regs)) {
    ret = PTR_ERR(mbox.regs);
    dev_err(dev, "Failed to map MMIO resource: %d\n", ret);
    goto err_disable_unprepare;
    }
// Disable all IRQs for this end of the msgbox.
    writel(0, mbox.regs + LOCAL_IRQ_EN_REG);
    ret = devm_request_irq(dev, irq_of_parse_and_map(dev.of_node, 0),
    sun6i_msgbox_irq, 0, dev_name(dev), mbox);
    if (ret)
    goto err_disable_unprepare;
    mbox.controller.dev           = dev;
    mbox.controller.ops           = &sun6i_msgbox_chan_ops;
    mbox.controller.chans         = chans;
    mbox.controller.num_chans     = NUM_CHANS;
    mbox.controller.txdone_irq    = false;
    mbox.controller.txdone_poll   = true;
    mbox.controller.txpoll_period = 5;
    spin_lock_init(&mbox.lock);
    platform_set_drvdata(pdev, mbox);
    ret = mbox_controller_register(&mbox.controller);
    if (ret) {
    dev_err(dev, "Failed to register controller: %d\n", ret);
    goto err_disable_unprepare;
    }
    return 0;
    err_disable_unprepare:
    clk_disable_unprepare(mbox.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_msgbox_remove(pdev: *mut platform_device) {
    static void sun6i_msgbox_remove(struct platform_device *pdev)
    {
    struct sun6i_msgbox *mbox = platform_get_drvdata(pdev);
    mbox_controller_unregister(&mbox.controller);
// See the comment in sun6i_msgbox_probe about the reset line.
    clk_disable_unprepare(mbox.clk);
    }
    static const struct of_device_id sun6i_msgbox_of_match[] = {
    { .compatible = "allwinner,sun6i-a31-msgbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, sun6i_msgbox_of_match);
    static struct platform_driver sun6i_msgbox_driver = {
    .driver = {
    .name = "sun6i-msgbox",
    .of_match_table = sun6i_msgbox_of_match,
    },
    .probe = sun6i_msgbox_probe,
    .remove = sun6i_msgbox_remove,
    };
    module_platform_driver(sun6i_msgbox_driver);
    MODULE_AUTHOR("Samuel Holland <samuel@sholland.org>");
    MODULE_DESCRIPTION("Allwinner sun6i/sun8i/sun9i/sun50i Message Box");
    MODULE_LICENSE("GPL v2");
