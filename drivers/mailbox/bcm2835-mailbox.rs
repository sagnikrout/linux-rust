//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/bcm2835-mailbox.c
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
// Copyright (C) 2010,2015 Broadcom
// Copyright (C) 2013-2014 Lubomir Rintel
// Copyright (C) 2013 Craig McGeachie
//
// Parts of the driver are based on:
// - arch/arm/mach-bcm2708/vcio.c file written by Gray Girling that was
// obtained from branch "rpi-3.6.y" of git://github.com/raspberrypi
// linux.git
// - drivers/mailbox/bcm2835-ipc.c by Lubomir Rintel at
// https://github.com/hackerspace/rpi-linux/blob/lr-raspberry-pi/drivers
// mailbox/bcm2835-ipc.c
// - documentation available on the following web site:
// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface
//

// Mailboxes
pub const ARM_0_MAIL0: c_uint = 0x00;
pub const ARM_0_MAIL1: c_uint = 0x20;
//
// Mailbox registers. We basically only support mailbox 0 & 1. We
// deliver to the VC in mailbox 1, it delivers to us in mailbox 0. See
// BCM2835-ARM-Peripherals.pdf section 1.3 for an explanation about
// the placement of memory barriers.
//

// Status register: FIFO state.

// Configuration register: Enable interrupts.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_mbox {
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
    pub controller: mbox_controller,
}

    static struct bcm2835_mbox *bcm2835_link_mbox(struct mbox_chan *link)
    {
    return container_of(link.mbox, struct bcm2835_mbox, controller);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_mbox_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm2835_mbox_irq(int irq, void *dev_id)
    {
    struct bcm2835_mbox *mbox = dev_id;
    struct device *dev = mbox.controller.dev;
    struct mbox_chan *link = &mbox.controller.chans[0];
    while (!(readl(mbox.regs + MAIL0_STA) & ARM_MS_EMPTY)) {
    let mut msg: u32 = readl(mbox.regs + MAIL0_RD);
    dev_dbg(dev, "Reply 0x%08X\n", msg);
    mbox_chan_received_data(link, &msg);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_send_data(link: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int bcm2835_send_data(struct mbox_chan *link, void *data)
    {
    struct bcm2835_mbox *mbox = bcm2835_link_mbox(link);
    let mut msg: u32 = *(u32 *)data;
    spin_lock(&mbox.lock);
    writel(msg, mbox.regs + MAIL1_WRT);
    dev_dbg(mbox.controller.dev, "Request 0x%08X\n", msg);
    spin_unlock(&mbox.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_startup(link: *mut mbox_chan) -> c_int {
    static int bcm2835_startup(struct mbox_chan *link)
    {
    struct bcm2835_mbox *mbox = bcm2835_link_mbox(link);
// Enable the interrupt on data reception
    writel(ARM_MC_IHAVEDATAIRQEN, mbox.regs + MAIL0_CNF);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_shutdown(link: *mut mbox_chan) {
    static void bcm2835_shutdown(struct mbox_chan *link)
    {
    struct bcm2835_mbox *mbox = bcm2835_link_mbox(link);
    writel(0, mbox.regs + MAIL0_CNF);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_last_tx_done(link: *mut mbox_chan) -> bool {
    static bool bcm2835_last_tx_done(struct mbox_chan *link)
    {
    struct bcm2835_mbox *mbox = bcm2835_link_mbox(link);
    bool ret;
    spin_lock(&mbox.lock);
    ret = !(readl(mbox.regs + MAIL1_STA) & ARM_MS_FULL);
    spin_unlock(&mbox.lock);
    return ret;
    }
    static const struct mbox_chan_ops bcm2835_mbox_chan_ops = {
    .send_data	= bcm2835_send_data,
    .startup	= bcm2835_startup,
    .shutdown	= bcm2835_shutdown,
    .last_tx_done	= bcm2835_last_tx_done
    };
    static struct mbox_chan *bcm2835_mbox_index_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *sp)
    {
    if (sp.args_count != 0)
    return ERR_PTR(-EINVAL);
    return &mbox.chans[0];
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_mbox_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *regs;
    let mut ret: c_int = 0;
    int irq;
    struct bcm2835_mbox *mbox;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    mbox = devm_kzalloc(dev, sizeof(*mbox), GFP_KERNEL);
    if (mbox == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&mbox.lock);
    mbox.regs = regs;
    ret = devm_request_irq(dev, irq, bcm2835_mbox_irq,
    IRQF_NO_SUSPEND, dev_name(dev), mbox);
    if (ret)
    return ret;
    mbox.controller.txdone_poll = true;
    mbox.controller.txpoll_period = 5;
    mbox.controller.ops = &bcm2835_mbox_chan_ops;
    mbox.controller.of_xlate = &bcm2835_mbox_index_xlate;
    mbox.controller.dev = dev;
    mbox.controller.num_chans = 1;
    mbox.controller.chans = devm_kzalloc(dev,
    sizeof(*mbox.controller.chans), GFP_KERNEL);
    if (!mbox.controller.chans)
    return -ENOMEM;
    return devm_mbox_controller_register(dev, &mbox.controller);
    }
    static const struct of_device_id bcm2835_mbox_of_match[] = {
    { .compatible = "brcm,bcm2835-mbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_mbox_of_match);
    static struct platform_driver bcm2835_mbox_driver = {
    .driver = {
    .name = "bcm2835-mbox",
    .of_match_table = bcm2835_mbox_of_match,
    },
    .probe		= bcm2835_mbox_probe,
    };
    module_platform_driver(bcm2835_mbox_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("BCM2835 mailbox IPC driver");
    MODULE_LICENSE("GPL v2");
