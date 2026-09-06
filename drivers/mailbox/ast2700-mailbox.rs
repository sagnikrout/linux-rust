//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/ast2700-mailbox.c
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
// Copyright Aspeed Technology Inc. (C) 2025. All rights reserved
//

// Each bit in the register represents an IPC ID
pub const IPCR_TX_TRIG: c_uint = 0x00;
pub const IPCR_ENABLE: c_uint = 0x04;
pub const IPCR_STATUS: c_uint = 0x08;

pub const RX_IRQ_MASK: c_uint = 0xf;
pub const IPCR_DATA: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast2700_mbox_data {
    pub num_chans: u8,
    pub msg_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast2700_mbox {
    pub mbox: mbox_controller,
    pub msg_size: u8,
    pub tx_regs: *mut void __iomem,
    pub rx_regs: *mut void __iomem,
    pub lock: spinlock_t,
}

#[no_mangle]
pub unsafe extern "C" fn ch_num(chan: *mut mbox_chan) -> c_int {
    static inline int ch_num(struct mbox_chan *chan)
    {
    return chan - chan.mbox.chans;
    }
#[no_mangle]
pub unsafe extern "C" fn ast2700_mbox_tx_done(mb: *mut ast2700_mbox, idx: c_int) -> bool {
    static inline bool ast2700_mbox_tx_done(struct ast2700_mbox *mb, int idx)
    {
    return !(readl(mb.tx_regs + IPCR_STATUS) & BIT(idx));
    }
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_irq(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ast2700_mbox_irq(int irq, void *p)
    {
    struct ast2700_mbox *mb = p;
    void __iomem *data_reg;
    let mut num_words: c_int = mb.msg_size / sizeof(u32);
    u32 *word_data;
    u32 status;
    int n, i;
// Only examine channels that are currently enabled.
    status = readl(mb.rx_regs + IPCR_ENABLE) &
    readl(mb.rx_regs + IPCR_STATUS);
    if (!(status & RX_IRQ_MASK))
    return IRQ_NONE;
    for (n = 0; n < mb.mbox.num_chans; ++n) {
    struct mbox_chan *chan = &mb.mbox.chans[n];
    if (!(status & RX_IRQ(n)))
    continue;
    data_reg = mb.rx_regs + IPCR_DATA + mb.msg_size * n;
    word_data = chan.con_priv;
// Read the message data
    for (i = 0; i < num_words; i++)
    word_data[i] = readl(data_reg + i * sizeof(u32));
    mbox_chan_received_data(chan, chan.con_priv);
// The IRQ can be cleared only once the FIFO is empty.
    writel(RX_IRQ(n), mb.rx_regs + IPCR_STATUS);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int ast2700_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct ast2700_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    let mut idx: c_int = ch_num(chan);
    void __iomem *data_reg = mb.tx_regs + IPCR_DATA + mb.msg_size * idx;
    u32 *word_data = data;
    let mut num_words: c_int = mb.msg_size / sizeof(u32);
    int i;
    if (!(readl(mb.tx_regs + IPCR_ENABLE) & BIT(idx))) {
    dev_warn(mb.mbox.dev, "%s: Ch-%d not enabled yet\n", __func__, idx);
    return -ENODEV;
    }
    if (!(ast2700_mbox_tx_done(mb, idx))) {
    dev_warn(mb.mbox.dev, "%s: Ch-%d last data has not finished\n", __func__, idx);
    return -EBUSY;
    }
// Write the message data
    for (i = 0 ; i < num_words; i++)
    writel(word_data[i], data_reg + i * sizeof(u32));
    writel(BIT(idx), mb.tx_regs + IPCR_TX_TRIG);
    dev_dbg(mb.mbox.dev, "%s: Ch-%d sent\n", __func__, idx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int ast2700_mbox_startup(struct mbox_chan *chan)
    {
    struct ast2700_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    let mut idx: c_int = ch_num(chan);
    void __iomem *reg = mb.rx_regs + IPCR_ENABLE;
    unsigned long flags;
    spin_lock_irqsave(&mb.lock, flags);
    writel(readl(reg) | BIT(idx), reg);
    spin_unlock_irqrestore(&mb.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_shutdown(chan: *mut mbox_chan) {
    static void ast2700_mbox_shutdown(struct mbox_chan *chan)
    {
    struct ast2700_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    let mut idx: c_int = ch_num(chan);
    void __iomem *reg = mb.rx_regs + IPCR_ENABLE;
    unsigned long flags;
    spin_lock_irqsave(&mb.lock, flags);
    writel(readl(reg) & ~BIT(idx), reg);
    spin_unlock_irqrestore(&mb.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool ast2700_mbox_last_tx_done(struct mbox_chan *chan)
    {
    struct ast2700_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    let mut idx: c_int = ch_num(chan);
    return ast2700_mbox_tx_done(mb, idx);
    }
    static const struct mbox_chan_ops ast2700_mbox_chan_ops = {
    .send_data	= ast2700_mbox_send_data,
    .startup	= ast2700_mbox_startup,
    .shutdown	= ast2700_mbox_shutdown,
    .last_tx_done	= ast2700_mbox_last_tx_done,
    };
#[no_mangle]
unsafe extern "C" fn ast2700_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int ast2700_mbox_probe(struct platform_device *pdev)
    {
    struct ast2700_mbox *mb;
    const struct ast2700_mbox_data *dev_data;
    struct device *dev = &pdev.dev;
    int irq, ret;
    if (!pdev.dev.of_node)
    return -ENODEV;
    dev_data = device_get_match_data(&pdev.dev);
    mb = devm_kzalloc(dev, sizeof(*mb), GFP_KERNEL);
    if (!mb)
    return -ENOMEM;
    mb.mbox.chans = devm_kcalloc(&pdev.dev, dev_data.num_chans,
    sizeof(*mb.mbox.chans), GFP_KERNEL);
    if (!mb.mbox.chans)
    return -ENOMEM;
// con_priv of each channel is used to store the message received
    for (int i = 0; i < dev_data.num_chans; i++) {
    mb.mbox.chans[i].con_priv = devm_kcalloc(dev, dev_data.msg_size,
    sizeof(u8), GFP_KERNEL);
    if (!mb.mbox.chans[i].con_priv)
    return -ENOMEM;
    }
    platform_set_drvdata(pdev, mb);
    mb.tx_regs = devm_platform_ioremap_resource_byname(pdev, "tx");
    if (IS_ERR(mb.tx_regs))
    return PTR_ERR(mb.tx_regs);
    mb.rx_regs = devm_platform_ioremap_resource_byname(pdev, "rx");
    if (IS_ERR(mb.rx_regs))
    return PTR_ERR(mb.rx_regs);
    mb.msg_size = dev_data.msg_size;
    mb.mbox.dev = dev;
    mb.mbox.num_chans = dev_data.num_chans;
    mb.mbox.ops = &ast2700_mbox_chan_ops;
    mb.mbox.txdone_irq = false;
    mb.mbox.txdone_poll = true;
    mb.mbox.txpoll_period = 5;
    spin_lock_init(&mb.lock);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, ast2700_mbox_irq, 0, dev_name(dev), mb);
    if (ret)
    return ret;
    return devm_mbox_controller_register(dev, &mb.mbox);
    }
    static const struct ast2700_mbox_data ast2700_dev_data = {
    .num_chans = 4,
    .msg_size = 0x20,
    };
    static const struct of_device_id ast2700_mbox_of_match[] = {
    { .compatible = "aspeed,ast2700-mailbox", .data = &ast2700_dev_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, ast2700_mbox_of_match);
    static struct platform_driver ast2700_mbox_driver = {
    .driver = {
    .name = "ast2700-mailbox",
    .of_match_table = ast2700_mbox_of_match,
    },
    .probe = ast2700_mbox_probe,
    };
    module_platform_driver(ast2700_mbox_driver);
    MODULE_AUTHOR("Jammy Huang <jammy_huang@aspeedtech.com>");
    MODULE_DESCRIPTION("ASPEED AST2700 IPC driver");
    MODULE_LICENSE("GPL");
