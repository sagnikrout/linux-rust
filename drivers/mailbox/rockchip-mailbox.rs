//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/rockchip-mailbox.c
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
// Copyright (c) 2015, Fuzhou Rockchip Electronics Co., Ltd
//

pub const MAILBOX_A2B_INTEN: c_uint = 0x00;
pub const MAILBOX_A2B_STATUS: c_uint = 0x04;

pub const MAILBOX_B2A_INTEN: c_uint = 0x28;
pub const MAILBOX_B2A_STATUS: c_uint = 0x2C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mbox_msg {
    pub cmd: u32,
    pub rx_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mbox_data {
    pub num_chans: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mbox_chan {
    pub idx: c_int,
    pub irq: c_int,
    pub msg: *mut rockchip_mbox_msg,
    pub mb: *mut rockchip_mbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mbox {
    pub mbox: mbox_controller,
    pub mbox_base: *mut void __iomem,
// The maximum size of buf for each channel
    pub buf_size: u32,
    pub chans: [rockchip_mbox_chan; ],
}

#[no_mangle]
unsafe extern "C" fn rockchip_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int rockchip_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct rockchip_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    struct rockchip_mbox_msg *msg = data;
    struct rockchip_mbox_chan *chans = mb.chans;
    if (!msg)
    return -EINVAL;
    if (msg.rx_size > mb.buf_size) {
    dev_err(mb.mbox.dev, "Transmit size over buf size(%d)\n",
    mb.buf_size);
    return -EINVAL;
    }
    dev_dbg(mb.mbox.dev, "Chan[%d]: A2B message, cmd 0x%08x\n",
    chans.idx, msg.cmd);
    mb.chans[chans.idx].msg = msg;
    writel_relaxed(msg.cmd, mb.mbox_base + MAILBOX_A2B_CMD(chans.idx));
    writel_relaxed(msg.rx_size, mb.mbox_base +
    MAILBOX_A2B_DAT(chans.idx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int rockchip_mbox_startup(struct mbox_chan *chan)
    {
    struct rockchip_mbox *mb = dev_get_drvdata(chan.mbox.dev);
// Enable all B2A interrupts
    writel_relaxed((1 << mb.mbox.num_chans) - 1,
    mb.mbox_base + MAILBOX_B2A_INTEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_mbox_shutdown(chan: *mut mbox_chan) {
    static void rockchip_mbox_shutdown(struct mbox_chan *chan)
    {
    struct rockchip_mbox *mb = dev_get_drvdata(chan.mbox.dev);
    struct rockchip_mbox_chan *chans = mb.chans;
// Disable all B2A interrupts
    writel_relaxed(0, mb.mbox_base + MAILBOX_B2A_INTEN);
    mb.chans[chans.idx].msg = core::ptr::null_mut();
    }
    static const struct mbox_chan_ops rockchip_mbox_chan_ops = {
    .send_data	= rockchip_mbox_send_data,
    .startup	= rockchip_mbox_startup,
    .shutdown	= rockchip_mbox_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn rockchip_mbox_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rockchip_mbox_irq(int irq, void *dev_id)
    {
    int idx;
    struct rockchip_mbox *mb = (struct rockchip_mbox *)dev_id;
    let mut status: u32 = readl_relaxed(mb.mbox_base + MAILBOX_B2A_STATUS);
    for (idx = 0; idx < mb.mbox.num_chans; idx++) {
    if ((status & (1 << idx)) && (irq == mb.chans[idx].irq)) {
// Clear mbox interrupt
    writel_relaxed(1 << idx,
    mb.mbox_base + MAILBOX_B2A_STATUS);
    return IRQ_WAKE_THREAD;
    }
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_mbox_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rockchip_mbox_isr(int irq, void *dev_id)
    {
    int idx;
    struct rockchip_mbox_msg *msg = core::ptr::null_mut();
    struct rockchip_mbox *mb = (struct rockchip_mbox *)dev_id;
    for (idx = 0; idx < mb.mbox.num_chans; idx++) {
    if (irq != mb.chans[idx].irq)
    continue;
    msg = mb.chans[idx].msg;
    if (!msg) {
    dev_err(mb.mbox.dev,
    "Chan[%d]: B2A message is core::ptr::null_mut()\n", idx);
    break; /* spurious */
    }
    mbox_chan_received_data(&mb.mbox.chans[idx], msg);
    mb.chans[idx].msg = core::ptr::null_mut();
    dev_dbg(mb.mbox.dev, "Chan[%d]: B2A message, cmd 0x%08x\n",
    idx, msg.cmd);
    break;
    }
    return IRQ_HANDLED;
    }
    static const struct rockchip_mbox_data rk3368_drv_data = {
    .num_chans = 4,
    };
    static const struct of_device_id rockchip_mbox_of_match[] = {
    { .compatible = "rockchip,rk3368-mailbox", .data = &rk3368_drv_data},
    { },
    };
    MODULE_DEVICE_TABLE(of, rockchip_mbox_of_match);
#[no_mangle]
unsafe extern "C" fn rockchip_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_mbox_probe(struct platform_device *pdev)
    {
    struct rockchip_mbox *mb;
    const struct rockchip_mbox_data *drv_data;
    struct resource *res;
    struct clk *pclk;
    int ret, irq, i;
    if (!pdev.dev.of_node)
    return -ENODEV;
    drv_data = (const struct rockchip_mbox_data *) device_get_match_data(&pdev.dev);
    mb = devm_kzalloc(&pdev.dev, struct_size(mb, chans, drv_data.num_chans), GFP_KERNEL);
    if (!mb)
    return -ENOMEM;
    mb.mbox.chans = devm_kcalloc(&pdev.dev, drv_data.num_chans,
    sizeof(*mb.mbox.chans), GFP_KERNEL);
    if (!mb.mbox.chans)
    return -ENOMEM;
    platform_set_drvdata(pdev, mb);
    mb.mbox.dev = &pdev.dev;
    mb.mbox.num_chans = drv_data.num_chans;
    mb.mbox.ops = &rockchip_mbox_chan_ops;
    mb.mbox.txdone_irq = true;
    mb.mbox_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(mb.mbox_base))
    return PTR_ERR(mb.mbox_base);
// Each channel has two buffers for A2B and B2A
    mb.buf_size = (size_t)resource_size(res) / (drv_data.num_chans * 2);
    pclk = devm_clk_get_enabled(&pdev.dev, "pclk_mailbox");
    if (IS_ERR(pclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(pclk),
    "failed to get and enable pclk_mailbox clock\n");
    for (i = 0; i < mb.mbox.num_chans; i++) {
    irq = platform_get_irq(pdev, i);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(&pdev.dev, irq,
    rockchip_mbox_irq,
    rockchip_mbox_isr, IRQF_ONESHOT,
    dev_name(&pdev.dev), mb);
    if (ret < 0)
    return ret;
    mb.chans[i].idx = i;
    mb.chans[i].irq = irq;
    mb.chans[i].mb = mb;
    mb.chans[i].msg = core::ptr::null_mut();
    }
    ret = devm_mbox_controller_register(&pdev.dev, &mb.mbox);
    if (ret < 0)
    dev_err(&pdev.dev, "Failed to register mailbox: %d\n", ret);
    return ret;
    }
    static struct platform_driver rockchip_mbox_driver = {
    .probe	= rockchip_mbox_probe,
    .driver = {
    .name = "rockchip-mailbox",
    .of_match_table = rockchip_mbox_of_match,
    },
    };
    module_platform_driver(rockchip_mbox_driver);
    MODULE_DESCRIPTION("Rockchip mailbox: communicate between CPU cores and MCU");
    MODULE_AUTHOR("Addy Ke <addy.ke@rock-chips.com>");
    MODULE_AUTHOR("Caesar Wang <wxt@rock-chips.com>");
