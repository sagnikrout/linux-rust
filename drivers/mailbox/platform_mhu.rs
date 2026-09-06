//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/platform_mhu.c
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
// Copyright (C) 2016 BayLibre SAS.
// Author: Neil Armstrong <narmstrong@baylibre.com>
// Synchronised with arm_mhu.c from :
// Copyright (C) 2013-2015 Fujitsu Semiconductor Ltd.
// Copyright (C) 2015 Linaro Ltd.
// Author: Jassi Brar <jaswinder.singh@linaro.org>
//

pub const INTR_SET_OFS: c_uint = 0x0;
pub const INTR_STAT_OFS: c_uint = 0x4;
pub const INTR_CLR_OFS: c_uint = 0x8;
pub const MHU_SEC_OFFSET: c_uint = 0x0;
pub const MHU_LP_OFFSET: c_uint = 0xc;
pub const MHU_HP_OFFSET: c_uint = 0x18;
pub const TX_REG_OFFSET: c_uint = 0x24;
pub const MHU_CHANS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_mhu_link {
    pub irq: c_int,
    pub tx_reg: *mut void __iomem,
    pub rx_reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_mhu {
    pub base: *mut void __iomem,
    pub mlink: [platform_mhu_link; MHU_CHANS],
    pub chan: [mbox_chan; MHU_CHANS],
    pub mbox: mbox_controller,
}

#[no_mangle]
unsafe extern "C" fn platform_mhu_rx_interrupt(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t platform_mhu_rx_interrupt(int irq, void *p)
    {
    struct mbox_chan *chan = p;
    struct platform_mhu_link *mlink = chan.con_priv;
    u32 val;
    val = readl_relaxed(mlink.rx_reg + INTR_STAT_OFS);
    if (!val)
    return IRQ_NONE;
    mbox_chan_received_data(chan, (void *)&val);
    writel_relaxed(val, mlink.rx_reg + INTR_CLR_OFS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn platform_mhu_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool platform_mhu_last_tx_done(struct mbox_chan *chan)
    {
    struct platform_mhu_link *mlink = chan.con_priv;
    let mut val: u32 = readl_relaxed(mlink.tx_reg + INTR_STAT_OFS);
    return (val == 0);
    }
#[no_mangle]
unsafe extern "C" fn platform_mhu_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int platform_mhu_send_data(struct mbox_chan *chan, void *data)
    {
    struct platform_mhu_link *mlink = chan.con_priv;
    u32 *arg = data;
    writel_relaxed(*arg, mlink.tx_reg + INTR_SET_OFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_mhu_startup(chan: *mut mbox_chan) -> c_int {
    static int platform_mhu_startup(struct mbox_chan *chan)
    {
    struct platform_mhu_link *mlink = chan.con_priv;
    u32 val;
    int ret;
    val = readl_relaxed(mlink.tx_reg + INTR_STAT_OFS);
    writel_relaxed(val, mlink.tx_reg + INTR_CLR_OFS);
    ret = request_irq(mlink.irq, platform_mhu_rx_interrupt,
    IRQF_SHARED, "platform_mhu_link", chan);
    if (ret) {
    dev_err(chan.mbox.dev,
    "Unable to acquire IRQ %d\n", mlink.irq);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_mhu_shutdown(chan: *mut mbox_chan) {
    static void platform_mhu_shutdown(struct mbox_chan *chan)
    {
    struct platform_mhu_link *mlink = chan.con_priv;
    free_irq(mlink.irq, chan);
    }
    static const struct mbox_chan_ops platform_mhu_ops = {
    .send_data = platform_mhu_send_data,
    .startup = platform_mhu_startup,
    .shutdown = platform_mhu_shutdown,
    .last_tx_done = platform_mhu_last_tx_done,
    };
#[no_mangle]
unsafe extern "C" fn platform_mhu_probe(pdev: *mut platform_device) -> c_int {
    static int platform_mhu_probe(struct platform_device *pdev)
    {
    int i, err;
    struct platform_mhu *mhu;
    struct device *dev = &pdev.dev;
    int platform_mhu_reg[MHU_CHANS] = {
    MHU_SEC_OFFSET, MHU_LP_OFFSET, MHU_HP_OFFSET
    };
// Allocate memory for device
    mhu = devm_kzalloc(dev, sizeof(*mhu), GFP_KERNEL);
    if (!mhu)
    return -ENOMEM;
    mhu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mhu.base)) {
    dev_err(dev, "ioremap failed\n");
    return PTR_ERR(mhu.base);
    }
    for (i = 0; i < MHU_CHANS; i++) {
    mhu.chan[i].con_priv = &mhu.mlink[i];
    mhu.mlink[i].irq = platform_get_irq(pdev, i);
    if (mhu.mlink[i].irq < 0)
    return mhu.mlink[i].irq;
    mhu.mlink[i].rx_reg = mhu.base + platform_mhu_reg[i];
    mhu.mlink[i].tx_reg = mhu.mlink[i].rx_reg + TX_REG_OFFSET;
    }
    mhu.mbox.dev = dev;
    mhu.mbox.chans = &mhu.chan[0];
    mhu.mbox.num_chans = MHU_CHANS;
    mhu.mbox.ops = &platform_mhu_ops;
    mhu.mbox.txdone_irq = false;
    mhu.mbox.txdone_poll = true;
    mhu.mbox.txpoll_period = 1;
    platform_set_drvdata(pdev, mhu);
    err = devm_mbox_controller_register(dev, &mhu.mbox);
    if (err) {
    dev_err(dev, "Failed to register mailboxes %d\n", err);
    return err;
    }
    dev_info(dev, "Platform MHU Mailbox registered\n");
    return 0;
    }
    static const struct of_device_id platform_mhu_dt_ids[] = {
    { .compatible = "amlogic,meson-gxbb-mhu", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, platform_mhu_dt_ids);
    static struct platform_driver platform_mhu_driver = {
    .probe	= platform_mhu_probe,
    .driver = {
    .name = "platform-mhu",
    .of_match_table	= platform_mhu_dt_ids,
    },
    };
    module_platform_driver(platform_mhu_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:platform-mhu");
    MODULE_DESCRIPTION("Platform MHU Driver");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
