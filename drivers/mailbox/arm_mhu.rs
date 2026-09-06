//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/arm_mhu.c
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
// Copyright (C) 2013-2015 Fujitsu Semiconductor Ltd.
// Copyright (C) 2015 Linaro Ltd.
// Author: Jassi Brar <jaswinder.singh@linaro.org>
//

pub const INTR_STAT_OFS: c_uint = 0x0;
pub const INTR_SET_OFS: c_uint = 0x8;
pub const INTR_CLR_OFS: c_uint = 0x10;
pub const MHU_LP_OFFSET: c_uint = 0x0;
pub const MHU_HP_OFFSET: c_uint = 0x20;
pub const MHU_SEC_OFFSET: c_uint = 0x200;
pub const TX_REG_OFFSET: c_uint = 0x100;
pub const MHU_CHANS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhu_link {
    pub irq: unsigned,
    pub tx_reg: *mut void __iomem,
    pub rx_reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_mhu {
    pub base: *mut void __iomem,
    pub mlink: [mhu_link; MHU_CHANS],
    pub chan: [mbox_chan; MHU_CHANS],
    pub mbox: mbox_controller,
}

#[no_mangle]
unsafe extern "C" fn mhu_rx_interrupt(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t mhu_rx_interrupt(int irq, void *p)
    {
    struct mbox_chan *chan = p;
    struct mhu_link *mlink = chan.con_priv;
    u32 val;
    val = readl_relaxed(mlink.rx_reg + INTR_STAT_OFS);
    if (!val)
    return IRQ_NONE;
    mbox_chan_received_data(chan, (void *)&val);
    writel_relaxed(val, mlink.rx_reg + INTR_CLR_OFS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mhu_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool mhu_last_tx_done(struct mbox_chan *chan)
    {
    struct mhu_link *mlink = chan.con_priv;
    let mut val: u32 = readl_relaxed(mlink.tx_reg + INTR_STAT_OFS);
    return (val == 0);
    }
#[no_mangle]
unsafe extern "C" fn mhu_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int mhu_send_data(struct mbox_chan *chan, void *data)
    {
    struct mhu_link *mlink = chan.con_priv;
    u32 *arg = data;
    writel_relaxed(*arg, mlink.tx_reg + INTR_SET_OFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhu_startup(chan: *mut mbox_chan) -> c_int {
    static int mhu_startup(struct mbox_chan *chan)
    {
    struct mhu_link *mlink = chan.con_priv;
    u32 val;
    int ret;
    val = readl_relaxed(mlink.tx_reg + INTR_STAT_OFS);
    writel_relaxed(val, mlink.tx_reg + INTR_CLR_OFS);
    ret = request_irq(mlink.irq, mhu_rx_interrupt,
    IRQF_SHARED, "mhu_link", chan);
    if (ret) {
    dev_err(chan.mbox.dev,
    "Unable to acquire IRQ %d\n", mlink.irq);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhu_shutdown(chan: *mut mbox_chan) {
    static void mhu_shutdown(struct mbox_chan *chan)
    {
    struct mhu_link *mlink = chan.con_priv;
    free_irq(mlink.irq, chan);
    }
    static const struct mbox_chan_ops mhu_ops = {
    .send_data = mhu_send_data,
    .startup = mhu_startup,
    .shutdown = mhu_shutdown,
    .last_tx_done = mhu_last_tx_done,
    };
#[no_mangle]
unsafe extern "C" fn mhu_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int mhu_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int i, err;
    struct arm_mhu *mhu;
    struct device *dev = &adev.dev;
    int mhu_reg[MHU_CHANS] = {MHU_LP_OFFSET, MHU_HP_OFFSET, MHU_SEC_OFFSET};
    if (!of_device_is_compatible(dev.of_node, "arm,mhu"))
    return -ENODEV;
// Allocate memory for device
    mhu = devm_kzalloc(dev, sizeof(*mhu), GFP_KERNEL);
    if (!mhu)
    return -ENOMEM;
    mhu.base = devm_ioremap_resource(dev, &adev.res);
    if (IS_ERR(mhu.base))
    return PTR_ERR(mhu.base);
    for (i = 0; i < MHU_CHANS; i++) {
    mhu.chan[i].con_priv = &mhu.mlink[i];
    mhu.mlink[i].irq = adev.irq[i];
    mhu.mlink[i].rx_reg = mhu.base + mhu_reg[i];
    mhu.mlink[i].tx_reg = mhu.mlink[i].rx_reg + TX_REG_OFFSET;
    }
    mhu.mbox.dev = dev;
    mhu.mbox.chans = &mhu.chan[0];
    mhu.mbox.num_chans = MHU_CHANS;
    mhu.mbox.ops = &mhu_ops;
    mhu.mbox.txdone_irq = false;
    mhu.mbox.txdone_poll = true;
    mhu.mbox.txpoll_period = 1;
    amba_set_drvdata(adev, mhu);
    err = devm_mbox_controller_register(dev, &mhu.mbox);
    if (err) {
    dev_err(dev, "Failed to register mailboxes %d\n", err);
    return err;
    }
    dev_info(dev, "ARM MHU Mailbox registered\n");
    return 0;
    }
    static const struct amba_id mhu_ids[] = {
    {
    .id	= 0x1bb098,
    .mask	= 0xffffff,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, mhu_ids);
    static struct amba_driver arm_mhu_driver = {
    .drv = {
    .name	= "mhu",
    },
    .id_table	= mhu_ids,
    .probe		= mhu_probe,
    };
    module_amba_driver(arm_mhu_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ARM MHU Driver");
    MODULE_AUTHOR("Jassi Brar <jassisinghbrar@gmail.com>");
