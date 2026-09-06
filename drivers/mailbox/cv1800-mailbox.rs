//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/cv1800-mailbox.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2024 Sophgo Technology Inc.
// Copyright (C) 2024 Yuntao Dai <d1581209858@live.com>
// Copyright (C) 2025 Junhui Liu <junhui.liu@pigmoral.tech>
//

pub const RECV_CPU: c_int = 1;
pub const MAILBOX_MAX_CHAN: c_int = 8;
pub const MAILBOX_MSG_LEN: c_int = 8;

pub const MBOX_SET_REG: c_uint = 0x60;
pub const MAILBOX_CONTEXT_OFFSET: c_uint = 0x0400;
pub const MAILBOX_CONTEXT_SIZE: c_uint = 0x0040;

    ((u64 __iomem *)(base + MAILBOX_CONTEXT_OFFSET) + index)
//
// struct cv1800_mbox_chan_priv - cv1800 mailbox channel private data
// @idx: index of channel
// @cpu: send to which processor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_mbox_chan_priv {
    pub idx: c_int,
    pub cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_mbox {
    pub mbox: mbox_controller,
    pub priv: [cv1800_mbox_chan_priv; MAILBOX_MAX_CHAN],
    pub chans: [mbox_chan; MAILBOX_MAX_CHAN],
    pub content: [*mut u64 __iomem; MAILBOX_MAX_CHAN],
    pub mbox_base: *mut void __iomem,
    pub recvid: c_int,
}

#[no_mangle]
unsafe extern "C" fn cv1800_mbox_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cv1800_mbox_isr(int irq, void *dev_id)
    {
    struct cv1800_mbox *mbox = (struct cv1800_mbox *)dev_id;
    size_t i;
    u64 msg;
    let mut ret: c_int = IRQ_NONE;
    for (i = 0; i < MAILBOX_MAX_CHAN; i++) {
    if (mbox.content[i] && mbox.chans[i].cl) {
    memcpy_fromio(&msg, mbox.content[i], MAILBOX_MSG_LEN);
    mbox.content[i] = core::ptr::null_mut();
    mbox_chan_received_data(&mbox.chans[i], (void *)&msg);
    ret = IRQ_HANDLED;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cv1800_mbox_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cv1800_mbox_irq(int irq, void *dev_id)
    {
    struct cv1800_mbox *mbox = (struct cv1800_mbox *)dev_id;
    u8 set, valid;
    size_t i;
    let mut ret: c_int = IRQ_NONE;
    set = readb(mbox.mbox_base + MBOX_SET_INT_REG(RECV_CPU));
    if (!set)
    return ret;
    for (i = 0; i < MAILBOX_MAX_CHAN; i++) {
    valid = set & BIT(i);
    if (valid) {
    mbox.content[i] =
    MBOX_CONTEXT_BASE_INDEX(mbox.mbox_base, i);
    writeb(valid, mbox.mbox_base +
    MBOX_SET_CLR_REG(RECV_CPU));
    writeb(~valid, mbox.mbox_base + MBOX_EN_REG(RECV_CPU));
    ret = IRQ_WAKE_THREAD;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cv1800_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int cv1800_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct cv1800_mbox_chan_priv *priv =
    (struct cv1800_mbox_chan_priv *)chan.con_priv;
    struct cv1800_mbox *mbox = dev_get_drvdata(chan.mbox.dev);
    let mut idx: c_int = priv.idx;
    let mut cpu: c_int = priv.cpu;
    u8 en, valid;
    memcpy_toio(MBOX_CONTEXT_BASE_INDEX(mbox.mbox_base, idx),
    data, MAILBOX_MSG_LEN);
    valid = BIT(idx);
    writeb(valid, mbox.mbox_base + MBOX_SET_CLR_REG(cpu));
    en = readb(mbox.mbox_base + MBOX_EN_REG(cpu));
    writeb(en | valid, mbox.mbox_base + MBOX_EN_REG(cpu));
    writeb(valid, mbox.mbox_base + MBOX_SET_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool cv1800_last_tx_done(struct mbox_chan *chan)
    {
    struct cv1800_mbox_chan_priv *priv =
    (struct cv1800_mbox_chan_priv *)chan.con_priv;
    struct cv1800_mbox *mbox = dev_get_drvdata(chan.mbox.dev);
    u8 en;
    en = readb(mbox.mbox_base + MBOX_EN_REG(priv.cpu));
    return !(en & BIT(priv.idx));
    }
    static const struct mbox_chan_ops cv1800_mbox_chan_ops = {
    .send_data = cv1800_mbox_send_data,
    .last_tx_done = cv1800_last_tx_done,
    };
    static struct mbox_chan *cv1800_mbox_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *spec)
    {
    struct cv1800_mbox_chan_priv *priv;
    let mut idx: c_int = spec.args[0];
    let mut cpu: c_int = spec.args[1];
    if (idx >= mbox.num_chans)
    return ERR_PTR(-EINVAL);
    priv = mbox.chans[idx].con_priv;
    priv.cpu = cpu;
    return &mbox.chans[idx];
    }
    static const struct of_device_id cv1800_mbox_of_match[] = {
    { .compatible = "sophgo,cv1800b-mailbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, cv1800_mbox_of_match);
#[no_mangle]
unsafe extern "C" fn cv1800_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800_mbox_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cv1800_mbox *mb;
    int irq, idx, err;
    mb = devm_kzalloc(dev, sizeof(*mb), GFP_KERNEL);
    if (!mb)
    return -ENOMEM;
    mb.mbox_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mb.mbox_base))
    return dev_err_probe(dev, PTR_ERR(mb.mbox_base),
    "Failed to map resource\n");
    mb.mbox.dev = dev;
    mb.mbox.chans = mb.chans;
    mb.mbox.txdone_poll = true;
    mb.mbox.ops = &cv1800_mbox_chan_ops;
    mb.mbox.num_chans = MAILBOX_MAX_CHAN;
    mb.mbox.of_xlate = cv1800_mbox_xlate;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_threaded_irq(dev, irq, cv1800_mbox_irq,
    cv1800_mbox_isr, IRQF_ONESHOT,
    dev_name(&pdev.dev), mb);
    if (err < 0)
    return err;
    for (idx = 0; idx < MAILBOX_MAX_CHAN; idx++) {
    mb.priv[idx].idx = idx;
    mb.mbox.chans[idx].con_priv = &mb.priv[idx];
    }
    platform_set_drvdata(pdev, mb);
    err = devm_mbox_controller_register(dev, &mb.mbox);
    if (err)
    return dev_err_probe(dev, err, "Failed to register mailbox\n");
    return 0;
    }
    static struct platform_driver cv1800_mbox_driver = {
    .driver = {
    .name = "cv1800-mbox",
    .of_match_table = cv1800_mbox_of_match,
    },
    .probe	= cv1800_mbox_probe,
    };
    module_platform_driver(cv1800_mbox_driver);
    MODULE_DESCRIPTION("cv1800 mailbox driver");
    MODULE_LICENSE("GPL");
