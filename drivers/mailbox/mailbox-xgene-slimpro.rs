//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/mailbox-xgene-slimpro.c
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
// APM X-Gene SLIMpro MailBox Driver
//
// Copyright (c) 2015, Applied Micro Circuits Corporation
// Author: Feng Kan fkan@apm.com
//

pub const MBOX_REG_SET_OFFSET: c_uint = 0x1000;
pub const MBOX_CNT: c_int = 8;

// Configuration and Status Registers
pub const REG_DB_IN: c_uint = 0x00;
pub const REG_DB_DIN0: c_uint = 0x04;
pub const REG_DB_DIN1: c_uint = 0x08;
pub const REG_DB_OUT: c_uint = 0x10;
pub const REG_DB_DOUT0: c_uint = 0x14;
pub const REG_DB_DOUT1: c_uint = 0x18;
pub const REG_DB_STAT: c_uint = 0x20;
pub const REG_DB_STATMASK: c_uint = 0x24;
//
// X-Gene SlimPRO mailbox channel information
//
// @dev:	Device to which it is attached
// @chan:	Pointer to mailbox communication channel
// @reg:	Base address to access channel registers
// @irq:	Interrupt number of the channel
// @rx_msg:	Received message storage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slimpro_mbox_chan {
    pub dev: *mut device,
    pub chan: *mut mbox_chan,
    pub reg: *mut void __iomem,
    pub irq: c_int,
    pub rx_msg: [u32; 3],
}

//
// X-Gene SlimPRO Mailbox controller data
//
// X-Gene SlimPRO Mailbox controller has 8 communication channels.
// Each channel has a separate IRQ number assigned to it.
//
// @mb_ctrl:	Representation of the communication channel controller
// @mc:		Array of SlimPRO mailbox channels of the controller
// @chans:	Array of mailbox communication channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slimpro_mbox {
    pub mb_ctrl: mbox_controller,
    pub mc: [slimpro_mbox_chan; MBOX_CNT],
    pub chans: [mbox_chan; MBOX_CNT],
}

#[no_mangle]
unsafe extern "C" fn mb_chan_send_msg(mb_chan: *mut slimpro_mbox_chan, msg: *mut u32) {
    static void mb_chan_send_msg(struct slimpro_mbox_chan *mb_chan, u32 *msg)
    {
    writel(msg[1], mb_chan.reg + REG_DB_DOUT0);
    writel(msg[2], mb_chan.reg + REG_DB_DOUT1);
    writel(msg[0], mb_chan.reg + REG_DB_OUT);
    }
#[no_mangle]
unsafe extern "C" fn mb_chan_recv_msg(mb_chan: *mut slimpro_mbox_chan) {
    static void mb_chan_recv_msg(struct slimpro_mbox_chan *mb_chan)
    {
    mb_chan.rx_msg[1] = readl(mb_chan.reg + REG_DB_DIN0);
    mb_chan.rx_msg[2] = readl(mb_chan.reg + REG_DB_DIN1);
    mb_chan.rx_msg[0] = readl(mb_chan.reg + REG_DB_IN);
    }
#[no_mangle]
unsafe extern "C" fn mb_chan_status_ack(mb_chan: *mut slimpro_mbox_chan) -> c_int {
    static int mb_chan_status_ack(struct slimpro_mbox_chan *mb_chan)
    {
    let mut val: u32 = readl(mb_chan.reg + REG_DB_STAT);
    if (val & MBOX_STATUS_ACK_MASK) {
    writel(MBOX_STATUS_ACK_MASK, mb_chan.reg + REG_DB_STAT);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mb_chan_status_avail(mb_chan: *mut slimpro_mbox_chan) -> c_int {
    static int mb_chan_status_avail(struct slimpro_mbox_chan *mb_chan)
    {
    let mut val: u32 = readl(mb_chan.reg + REG_DB_STAT);
    if (val & MBOX_STATUS_AVAIL_MASK) {
    mb_chan_recv_msg(mb_chan);
    writel(MBOX_STATUS_AVAIL_MASK, mb_chan.reg + REG_DB_STAT);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_irq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t slimpro_mbox_irq(int irq, void *id)
    {
    struct slimpro_mbox_chan *mb_chan = id;
    if (mb_chan_status_ack(mb_chan))
    mbox_chan_txdone(mb_chan.chan, 0);
    if (mb_chan_status_avail(mb_chan))
    mbox_chan_received_data(mb_chan.chan, mb_chan.rx_msg);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_send_data(chan: *mut mbox_chan, msg: *mut c_void) -> c_int {
    static int slimpro_mbox_send_data(struct mbox_chan *chan, void *msg)
    {
    struct slimpro_mbox_chan *mb_chan = chan.con_priv;
    mb_chan_send_msg(mb_chan, msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int slimpro_mbox_startup(struct mbox_chan *chan)
    {
    struct slimpro_mbox_chan *mb_chan = chan.con_priv;
    int rc;
    u32 val;
    rc = devm_request_irq(mb_chan.dev, mb_chan.irq, slimpro_mbox_irq, 0,
    MBOX_CON_NAME, mb_chan);
    if (unlikely(rc)) {
    dev_err(mb_chan.dev, "failed to register mailbox interrupt %d\n",
    mb_chan.irq);
    return rc;
    }
// Enable HW interrupt
    writel(MBOX_STATUS_ACK_MASK | MBOX_STATUS_AVAIL_MASK,
    mb_chan.reg + REG_DB_STAT);
// Unmask doorbell status interrupt
    val = readl(mb_chan.reg + REG_DB_STATMASK);
    val &= ~(MBOX_STATUS_ACK_MASK | MBOX_STATUS_AVAIL_MASK);
    writel(val, mb_chan.reg + REG_DB_STATMASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_shutdown(chan: *mut mbox_chan) {
    static void slimpro_mbox_shutdown(struct mbox_chan *chan)
    {
    struct slimpro_mbox_chan *mb_chan = chan.con_priv;
    u32 val;
// Mask doorbell status interrupt
    val = readl(mb_chan.reg + REG_DB_STATMASK);
    val |= (MBOX_STATUS_ACK_MASK | MBOX_STATUS_AVAIL_MASK);
    writel(val, mb_chan.reg + REG_DB_STATMASK);
    devm_free_irq(mb_chan.dev, mb_chan.irq, mb_chan);
    }
    static const struct mbox_chan_ops slimpro_mbox_ops = {
    .send_data = slimpro_mbox_send_data,
    .startup = slimpro_mbox_startup,
    .shutdown = slimpro_mbox_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int slimpro_mbox_probe(struct platform_device *pdev)
    {
    struct slimpro_mbox *ctx;
    void __iomem *mb_base;
    int rc;
    int i;
    ctx = devm_kzalloc(&pdev.dev, sizeof(struct slimpro_mbox), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    platform_set_drvdata(pdev, ctx);
    mb_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mb_base))
    return PTR_ERR(mb_base);
// Setup mailbox links
    for (i = 0; i < MBOX_CNT; i++) {
    ctx.mc[i].irq = platform_get_irq(pdev, i);
    if (ctx.mc[i].irq < 0) {
    if (i == 0) {
    dev_err(&pdev.dev, "no available IRQ\n");
    return -EINVAL;
    }
    dev_info(&pdev.dev, "no IRQ for channel %d\n", i);
    break;
    }
    ctx.mc[i].dev = &pdev.dev;
    ctx.mc[i].reg = mb_base + i * MBOX_REG_SET_OFFSET;
    ctx.mc[i].chan = &ctx.chans[i];
    ctx.chans[i].con_priv = &ctx.mc[i];
    }
// Setup mailbox controller
    ctx.mb_ctrl.dev = &pdev.dev;
    ctx.mb_ctrl.chans = ctx.chans;
    ctx.mb_ctrl.txdone_irq = true;
    ctx.mb_ctrl.ops = &slimpro_mbox_ops;
    ctx.mb_ctrl.num_chans = i;
    rc = devm_mbox_controller_register(&pdev.dev, &ctx.mb_ctrl);
    if (rc) {
    dev_err(&pdev.dev,
    "APM X-Gene SLIMpro MailBox register failed:%d\n", rc);
    return rc;
    }
    dev_info(&pdev.dev, "APM X-Gene SLIMpro MailBox registered\n");
    return 0;
    }
    static const struct of_device_id slimpro_of_match[] = {
    {.compatible = "apm,xgene-slimpro-mbox" },
    { },
    };
    MODULE_DEVICE_TABLE(of, slimpro_of_match);

    static const struct acpi_device_id slimpro_acpi_ids[] = {
    {"APMC0D01", 0},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, slimpro_acpi_ids);

    static struct platform_driver slimpro_mbox_driver = {
    .probe	= slimpro_mbox_probe,
    .driver	= {
    .name = "xgene-slimpro-mbox",
    .of_match_table = of_match_ptr(slimpro_of_match),
    .acpi_match_table = ACPI_PTR(slimpro_acpi_ids)
    },
    };
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_init() -> int __init {
    static int __init slimpro_mbox_init(void)
    {
    return platform_driver_register(&slimpro_mbox_driver);
    }
#[no_mangle]
unsafe extern "C" fn slimpro_mbox_exit() -> void __exit {
    static void __exit slimpro_mbox_exit(void)
    {
    platform_driver_unregister(&slimpro_mbox_driver);
    }
    subsys_initcall(slimpro_mbox_init);
    module_exit(slimpro_mbox_exit);
    MODULE_DESCRIPTION("APM X-Gene SLIMpro Mailbox Driver");
    MODULE_LICENSE("GPL");
