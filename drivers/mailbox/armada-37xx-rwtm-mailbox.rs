//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/armada-37xx-rwtm-mailbox.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// rWTM BIU Mailbox driver for Armada 37xx
//
// Author: Marek Behún <kabel@kernel.org>
//

// relative to rWTM BIU Mailbox Registers

pub const RWTM_MBOX_COMMAND: c_uint = 0x40;
pub const RWTM_MBOX_RETURN_STATUS: c_uint = 0x80;

pub const RWTM_MBOX_FIFO_STATUS: c_uint = 0xc4;
pub const FIFO_STS_RDY: c_uint = 0x100;
pub const FIFO_STS_CNTR_MASK: c_uint = 0x7;
pub const FIFO_STS_CNTR_MAX: c_int = 4;
pub const RWTM_HOST_INT_RESET: c_uint = 0xc8;
pub const RWTM_HOST_INT_MASK: c_uint = 0xcc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a37xx_mbox {
    pub dev: *mut device,
    pub controller: mbox_controller,
    pub base: *mut void __iomem,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn a37xx_mbox_receive(chan: *mut mbox_chan) {
    static void a37xx_mbox_receive(struct mbox_chan *chan)
    {
    struct a37xx_mbox *mbox = chan.con_priv;
    struct armada_37xx_rwtm_rx_msg rx_msg;
    int i;
    rx_msg.retval = readl(mbox.base + RWTM_MBOX_RETURN_STATUS);
    for (i = 0; i < 16; ++i)
    rx_msg.status[i] = readl(mbox.base + RWTM_MBOX_STATUS(i));
    mbox_chan_received_data(chan, &rx_msg);
    }
#[no_mangle]
unsafe extern "C" fn a37xx_mbox_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t a37xx_mbox_irq_handler(int irq, void *data)
    {
    struct mbox_chan *chan = data;
    struct a37xx_mbox *mbox = chan.con_priv;
    u32 reg;
    reg = readl(mbox.base + RWTM_HOST_INT_RESET);
    if (reg & SP_CMD_COMPLETE)
    a37xx_mbox_receive(chan);
    if (reg & (SP_CMD_QUEUE_FULL_ACCESS | SP_CMD_QUEUE_FULL))
    dev_err(mbox.dev, "Secure processor command queue full\n");
    writel(reg, mbox.base + RWTM_HOST_INT_RESET);
    if (reg)
    mbox_chan_txdone(chan, 0);
    return reg ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn a37xx_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int a37xx_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct a37xx_mbox *mbox = chan.con_priv;
    struct armada_37xx_rwtm_tx_msg *msg = data;
    int i;
    u32 reg;
    if (!data)
    return -EINVAL;
    reg = readl(mbox.base + RWTM_MBOX_FIFO_STATUS);
    if (!(reg & FIFO_STS_RDY))
    dev_warn(mbox.dev, "Secure processor not ready\n");
    if ((reg & FIFO_STS_CNTR_MASK) >= FIFO_STS_CNTR_MAX) {
    dev_err(mbox.dev, "Secure processor command queue full\n");
    return -EBUSY;
    }
    for (i = 0; i < 16; ++i)
    writel(msg.args[i], mbox.base + RWTM_MBOX_PARAM(i));
    writel(msg.command, mbox.base + RWTM_MBOX_COMMAND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a37xx_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int a37xx_mbox_startup(struct mbox_chan *chan)
    {
    struct a37xx_mbox *mbox = chan.con_priv;
    u32 reg;
    int ret;
    ret = devm_request_irq(mbox.dev, mbox.irq, a37xx_mbox_irq_handler, 0,
    DRIVER_NAME, chan);
    if (ret < 0)
    return ret;
// enable IRQ generation
    reg = readl(mbox.base + RWTM_HOST_INT_MASK);
    reg &= ~(SP_CMD_COMPLETE | SP_CMD_QUEUE_FULL_ACCESS | SP_CMD_QUEUE_FULL);
    writel(reg, mbox.base + RWTM_HOST_INT_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a37xx_mbox_shutdown(chan: *mut mbox_chan) {
    static void a37xx_mbox_shutdown(struct mbox_chan *chan)
    {
    u32 reg;
    struct a37xx_mbox *mbox = chan.con_priv;
// disable interrupt generation
    reg = readl(mbox.base + RWTM_HOST_INT_MASK);
    reg |= SP_CMD_COMPLETE | SP_CMD_QUEUE_FULL_ACCESS | SP_CMD_QUEUE_FULL;
    writel(reg, mbox.base + RWTM_HOST_INT_MASK);
    devm_free_irq(mbox.dev, mbox.irq, chan);
    }
    static const struct mbox_chan_ops a37xx_mbox_ops = {
    .send_data	= a37xx_mbox_send_data,
    .startup	= a37xx_mbox_startup,
    .shutdown	= a37xx_mbox_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn armada_37xx_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int armada_37xx_mbox_probe(struct platform_device *pdev)
    {
    struct a37xx_mbox *mbox;
    struct mbox_chan *chans;
    int ret;
    mbox = devm_kzalloc(&pdev.dev, sizeof(*mbox), GFP_KERNEL);
    if (!mbox)
    return -ENOMEM;
// Allocated one channel
    chans = devm_kzalloc(&pdev.dev, sizeof(*chans), GFP_KERNEL);
    if (!chans)
    return -ENOMEM;
    mbox.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mbox.base))
    return PTR_ERR(mbox.base);
    mbox.irq = platform_get_irq(pdev, 0);
    if (mbox.irq < 0)
    return mbox.irq;
    mbox.dev = &pdev.dev;
// Hardware supports only one channel.
    chans[0].con_priv = mbox;
    mbox.controller.dev = mbox.dev;
    mbox.controller.num_chans = 1;
    mbox.controller.chans = chans;
    mbox.controller.ops = &a37xx_mbox_ops;
    mbox.controller.txdone_irq = true;
    ret = devm_mbox_controller_register(mbox.dev, &mbox.controller);
    if (ret) {
    dev_err(&pdev.dev, "Could not register mailbox controller\n");
    return ret;
    }
    platform_set_drvdata(pdev, mbox);
    return ret;
    }
    static const struct of_device_id armada_37xx_mbox_match[] = {
    { .compatible = "marvell,armada-3700-rwtm-mailbox" },
    { },
    };
    MODULE_DEVICE_TABLE(of, armada_37xx_mbox_match);
    static struct platform_driver armada_37xx_mbox_driver = {
    .probe	= armada_37xx_mbox_probe,
    .driver	= {
    .name		= DRIVER_NAME,
    .of_match_table	= armada_37xx_mbox_match,
    },
    };
    module_platform_driver(armada_37xx_mbox_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("rWTM BIU Mailbox driver for Armada 37xx");
    MODULE_AUTHOR("Marek Behun <kabel@kernel.org>");
