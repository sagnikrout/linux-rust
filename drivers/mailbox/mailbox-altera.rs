//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/mailbox-altera.c
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
// Copyright Altera Corporation (C) 2013-2014. All rights reserved
//

pub const MAILBOX_CMD_REG: c_uint = 0x00;
pub const MAILBOX_PTR_REG: c_uint = 0x04;
pub const MAILBOX_STS_REG: c_uint = 0x08;
pub const MAILBOX_INTMASK_REG: c_uint = 0x0C;
pub const INT_PENDING_MSK: c_uint = 0x1;
pub const INT_SPACE_MSK: c_uint = 0x2;
pub const STS_PENDING_MSK: c_uint = 0x1;
pub const STS_FULL_MSK: c_uint = 0x2;
pub const STS_FULL_OFT: c_uint = 0x1;

    enum altera_mbox_msg {
    MBOX_CMD = 0,
    MBOX_PTR,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_mbox {
    pub /: *mut *mut bool is_sender; / 1-sender, 0-receiver,
    pub intr_mode: bool,
    pub irq: c_int,
    pub mbox_base: *mut void __iomem,
    pub dev: *mut device,
    pub controller: mbox_controller,
// If the controller supports only RX polling mode
    pub rxpoll_timer: timer_list,
    pub chan: *mut mbox_chan,
}

    static struct altera_mbox *mbox_chan_to_altera_mbox(struct mbox_chan *chan)
    {
    if (!chan || !chan.con_priv)
    return core::ptr::null_mut();
    return (struct altera_mbox *)chan.con_priv;
    }
#[no_mangle]
pub unsafe extern "C" fn altera_mbox_full(mbox: *mut altera_mbox) -> c_int {
    static inline int altera_mbox_full(struct altera_mbox *mbox)
    {
    u32 status;
    status = readl_relaxed(mbox.mbox_base + MAILBOX_STS_REG);
    return MBOX_FULL(status);
    }
#[no_mangle]
pub unsafe extern "C" fn altera_mbox_pending(mbox: *mut altera_mbox) -> c_int {
    static inline int altera_mbox_pending(struct altera_mbox *mbox)
    {
    u32 status;
    status = readl_relaxed(mbox.mbox_base + MAILBOX_STS_REG);
    return MBOX_PENDING(status);
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_rx_intmask(mbox: *mut altera_mbox, enable: bool) {
    static void altera_mbox_rx_intmask(struct altera_mbox *mbox, bool enable)
    {
    u32 mask;
    mask = readl_relaxed(mbox.mbox_base + MAILBOX_INTMASK_REG);
    if (enable)
    mask |= INT_PENDING_MSK;
    else
    mask &= ~INT_PENDING_MSK;
    writel_relaxed(mask, mbox.mbox_base + MAILBOX_INTMASK_REG);
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_tx_intmask(mbox: *mut altera_mbox, enable: bool) {
    static void altera_mbox_tx_intmask(struct altera_mbox *mbox, bool enable)
    {
    u32 mask;
    mask = readl_relaxed(mbox.mbox_base + MAILBOX_INTMASK_REG);
    if (enable)
    mask |= INT_SPACE_MSK;
    else
    mask &= ~INT_SPACE_MSK;
    writel_relaxed(mask, mbox.mbox_base + MAILBOX_INTMASK_REG);
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_is_sender(mbox: *mut altera_mbox) -> bool {
    static bool altera_mbox_is_sender(struct altera_mbox *mbox)
    {
    u32 reg;
// Write a magic number to PTR register and read back this register.
// This register is read-write if it is a sender.
//
pub const MBOX_MAGIC: c_uint = 0xA5A5AA55;
    writel_relaxed(MBOX_MAGIC, mbox.mbox_base + MAILBOX_PTR_REG);
    reg = readl_relaxed(mbox.mbox_base + MAILBOX_PTR_REG);
    if (reg == MBOX_MAGIC) {
// Clear to 0
    writel_relaxed(0, mbox.mbox_base + MAILBOX_PTR_REG);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_rx_data(chan: *mut mbox_chan) {
    static void altera_mbox_rx_data(struct mbox_chan *chan)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    u32 data[2];
    if (altera_mbox_pending(mbox)) {
    data[MBOX_PTR] =
    readl_relaxed(mbox.mbox_base + MAILBOX_PTR_REG);
    data[MBOX_CMD] =
    readl_relaxed(mbox.mbox_base + MAILBOX_CMD_REG);
    mbox_chan_received_data(chan, (void *)data);
    }
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_poll_rx(t: *mut timer_list) {
    static void altera_mbox_poll_rx(struct timer_list *t)
    {
    struct altera_mbox *mbox = timer_container_of(mbox, t, rxpoll_timer);
    altera_mbox_rx_data(mbox.chan);
    mod_timer(&mbox.rxpoll_timer,
    jiffies + msecs_to_jiffies(MBOX_POLLING_MS));
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_tx_interrupt(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t altera_mbox_tx_interrupt(int irq, void *p)
    {
    struct mbox_chan *chan = (struct mbox_chan *)p;
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    altera_mbox_tx_intmask(mbox, false);
    mbox_chan_txdone(chan, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_rx_interrupt(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t altera_mbox_rx_interrupt(int irq, void *p)
    {
    struct mbox_chan *chan = (struct mbox_chan *)p;
    altera_mbox_rx_data(chan);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_startup_sender(chan: *mut mbox_chan) -> c_int {
    static int altera_mbox_startup_sender(struct mbox_chan *chan)
    {
    int ret;
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    if (mbox.intr_mode) {
    ret = request_irq(mbox.irq, altera_mbox_tx_interrupt, 0,
    DRIVER_NAME, chan);
    if (unlikely(ret)) {
    dev_err(mbox.dev,
    "failed to register mailbox interrupt:%d\n",
    ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_startup_receiver(chan: *mut mbox_chan) -> c_int {
    static int altera_mbox_startup_receiver(struct mbox_chan *chan)
    {
    int ret;
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    if (mbox.intr_mode) {
    ret = request_irq(mbox.irq, altera_mbox_rx_interrupt, 0,
    DRIVER_NAME, chan);
    if (unlikely(ret)) {
    mbox.intr_mode = false;
    goto polling; /* use polling if failed */
    }
    altera_mbox_rx_intmask(mbox, true);
    return 0;
    }
    polling:
// Setup polling timer
    mbox.chan = chan;
    timer_setup(&mbox.rxpoll_timer, altera_mbox_poll_rx, 0);
    mod_timer(&mbox.rxpoll_timer,
    jiffies + msecs_to_jiffies(MBOX_POLLING_MS));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int altera_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    u32 *udata = (u32 *)data;
    if (!mbox || !data)
    return -EINVAL;
    if (!mbox.is_sender) {
    dev_warn(mbox.dev,
    "failed to send. This is receiver mailbox.\n");
    return -EINVAL;
    }
    if (altera_mbox_full(mbox))
    return -EBUSY;
// Enable interrupt before send
    if (mbox.intr_mode)
    altera_mbox_tx_intmask(mbox, true);
// Pointer register must write before command register
    writel_relaxed(udata[MBOX_PTR], mbox.mbox_base + MAILBOX_PTR_REG);
    writel_relaxed(udata[MBOX_CMD], mbox.mbox_base + MAILBOX_CMD_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool altera_mbox_last_tx_done(struct mbox_chan *chan)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
// Return false if mailbox is full
    return altera_mbox_full(mbox) ? false : true;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_peek_data(chan: *mut mbox_chan) -> bool {
    static bool altera_mbox_peek_data(struct mbox_chan *chan)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    return altera_mbox_pending(mbox) ? true : false;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int altera_mbox_startup(struct mbox_chan *chan)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    let mut ret: c_int = 0;
    if (!mbox)
    return -EINVAL;
    if (mbox.is_sender)
    ret = altera_mbox_startup_sender(chan);
    else
    ret = altera_mbox_startup_receiver(chan);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn altera_mbox_shutdown(chan: *mut mbox_chan) {
    static void altera_mbox_shutdown(struct mbox_chan *chan)
    {
    struct altera_mbox *mbox = mbox_chan_to_altera_mbox(chan);
    if (mbox.intr_mode) {
// Unmask all interrupt masks
    writel_relaxed(~0, mbox.mbox_base + MAILBOX_INTMASK_REG);
    free_irq(mbox.irq, chan);
    } else if (!mbox.is_sender) {
    timer_delete_sync(&mbox.rxpoll_timer);
    }
    }
    static const struct mbox_chan_ops altera_mbox_ops = {
    .send_data = altera_mbox_send_data,
    .startup = altera_mbox_startup,
    .shutdown = altera_mbox_shutdown,
    .last_tx_done = altera_mbox_last_tx_done,
    .peek_data = altera_mbox_peek_data,
    };
#[no_mangle]
unsafe extern "C" fn altera_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int altera_mbox_probe(struct platform_device *pdev)
    {
    struct altera_mbox *mbox;
    struct mbox_chan *chans;
    int ret;
    mbox = devm_kzalloc(&pdev.dev, sizeof(*mbox),
    GFP_KERNEL);
    if (!mbox)
    return -ENOMEM;
// Allocated one channel
    chans = devm_kzalloc(&pdev.dev, sizeof(*chans), GFP_KERNEL);
    if (!chans)
    return -ENOMEM;
    mbox.mbox_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mbox.mbox_base))
    return PTR_ERR(mbox.mbox_base);
// Check is it a sender or receiver?
    mbox.is_sender = altera_mbox_is_sender(mbox);
    mbox.irq = platform_get_irq(pdev, 0);
    if (mbox.irq >= 0)
    mbox.intr_mode = true;
    mbox.dev = &pdev.dev;
// Hardware supports only one channel.
    chans[0].con_priv = mbox;
    mbox.controller.dev = mbox.dev;
    mbox.controller.num_chans = 1;
    mbox.controller.chans = chans;
    mbox.controller.ops = &altera_mbox_ops;
    if (mbox.is_sender) {
    if (mbox.intr_mode) {
    mbox.controller.txdone_irq = true;
    } else {
    mbox.controller.txdone_poll = true;
    mbox.controller.txpoll_period = MBOX_POLLING_MS;
    }
    }
    ret = devm_mbox_controller_register(&pdev.dev, &mbox.controller);
    if (ret) {
    dev_err(&pdev.dev, "Register mailbox failed\n");
    goto err;
    }
    platform_set_drvdata(pdev, mbox);
    err:
    return ret;
    }
    static const struct of_device_id altera_mbox_match[] = {
    { .compatible = "altr,mailbox-1.0" },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, altera_mbox_match);
    static struct platform_driver altera_mbox_driver = {
    .probe	= altera_mbox_probe,
    .driver	= {
    .name	= DRIVER_NAME,
    .of_match_table	= altera_mbox_match,
    },
    };
    module_platform_driver(altera_mbox_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Altera mailbox specific functions");
    MODULE_AUTHOR("Ley Foon Tan <lftan@altera.com>");
    MODULE_ALIAS("platform:altera-mailbox");
