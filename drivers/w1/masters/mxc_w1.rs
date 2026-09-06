//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/mxc_w1.c
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
// Copyright 2005-2008 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright 2008 Luotao Fu, kernel@pengutronix.de
//

//
// MXC W1 Register offsets
//
pub const MXC_W1_CONTROL: c_uint = 0x00;

pub const MXC_W1_TIME_DIVIDER: c_uint = 0x02;
pub const MXC_W1_RESET: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_w1_device {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub bus_master: w1_bus_master,
}

//
// this is the low level routine to
// reset the device on the One Wire interface
// on the hardware
//
#[no_mangle]
unsafe extern "C" fn mxc_w1_ds2_reset_bus(data: *mut c_void) -> u8 {
    static u8 mxc_w1_ds2_reset_bus(void *data)
    {
    struct mxc_w1_device *dev = data;
    ktime_t timeout;
    writeb(MXC_W1_CONTROL_RPP, dev.regs + MXC_W1_CONTROL);
// Wait for reset sequence 511+512us, use 1500us for sure
    timeout = ktime_add_us(ktime_get(), 1500);
    udelay(511 + 512);
    do {
    let mut ctrl: u8 = readb(dev.regs + MXC_W1_CONTROL);
// PST bit is valid after the RPP bit is self-cleared
    if (!(ctrl & MXC_W1_CONTROL_RPP))
    return !(ctrl & MXC_W1_CONTROL_PST);
    } while (ktime_before(ktime_get(), timeout));
    return 1;
    }
//
// this is the low level routine to read/write a bit on the One Wire
// interface on the hardware. It does write 0 if parameter bit is set
// to 0, otherwise a write 1/read.
//
#[no_mangle]
unsafe extern "C" fn mxc_w1_ds2_touch_bit(data: *mut c_void, bit: u8) -> u8 {
    static u8 mxc_w1_ds2_touch_bit(void *data, u8 bit)
    {
    struct mxc_w1_device *dev = data;
    ktime_t timeout;
    writeb(MXC_W1_CONTROL_WR(bit), dev.regs + MXC_W1_CONTROL);
// Wait for read/write bit (60us, Max 120us), use 200us for sure
    timeout = ktime_add_us(ktime_get(), 200);
    udelay(60);
    do {
    let mut ctrl: u8 = readb(dev.regs + MXC_W1_CONTROL);
// RDST bit is valid after the WR1/RD bit is self-cleared
    if (!(ctrl & MXC_W1_CONTROL_WR(bit)))
    return !!(ctrl & MXC_W1_CONTROL_RDST);
    } while (ktime_before(ktime_get(), timeout));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_w1_probe(pdev: *mut platform_device) -> c_int {
    static int mxc_w1_probe(struct platform_device *pdev)
    {
    struct mxc_w1_device *mdev;
    unsigned long clkrate;
    unsigned int clkdiv;
    int err;
    mdev = devm_kzalloc(&pdev.dev, sizeof(struct mxc_w1_device),
    GFP_KERNEL);
    if (!mdev)
    return -ENOMEM;
    mdev.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mdev.clk))
    return PTR_ERR(mdev.clk);
    err = clk_prepare_enable(mdev.clk);
    if (err)
    return err;
    clkrate = clk_get_rate(mdev.clk);
    if (clkrate < 10000000)
    dev_warn(&pdev.dev,
    "Low clock frequency causes improper function\n");
    clkdiv = DIV_ROUND_CLOSEST(clkrate, 1000000);
    clkrate /= clkdiv;
    if ((clkrate < 980000) || (clkrate > 1020000))
    dev_warn(&pdev.dev,
    "Incorrect time base frequency %lu Hz\n", clkrate);
    mdev.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mdev.regs)) {
    err = PTR_ERR(mdev.regs);
    goto out_disable_clk;
    }
// Software reset 1-Wire module
    writeb(MXC_W1_RESET_RST, mdev.regs + MXC_W1_RESET);
    writeb(0, mdev.regs + MXC_W1_RESET);
    writeb(clkdiv - 1, mdev.regs + MXC_W1_TIME_DIVIDER);
    mdev.bus_master.data = mdev;
    mdev.bus_master.reset_bus = mxc_w1_ds2_reset_bus;
    mdev.bus_master.touch_bit = mxc_w1_ds2_touch_bit;
    platform_set_drvdata(pdev, mdev);
    err = w1_add_master_device(&mdev.bus_master);
    if (err)
    goto out_disable_clk;
    return 0;
    out_disable_clk:
    clk_disable_unprepare(mdev.clk);
    return err;
    }
//
// disassociate the w1 device from the driver
//
#[no_mangle]
unsafe extern "C" fn mxc_w1_remove(pdev: *mut platform_device) {
    static void mxc_w1_remove(struct platform_device *pdev)
    {
    struct mxc_w1_device *mdev = platform_get_drvdata(pdev);
    w1_remove_master_device(&mdev.bus_master);
    clk_disable_unprepare(mdev.clk);
    }
    static const struct of_device_id mxc_w1_dt_ids[] = {
    { .compatible = "fsl,imx21-owire" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mxc_w1_dt_ids);
    static struct platform_driver mxc_w1_driver = {
    .driver = {
    .name = "mxc_w1",
    .of_match_table = mxc_w1_dt_ids,
    },
    .probe = mxc_w1_probe,
    .remove = mxc_w1_remove,
    };
    module_platform_driver(mxc_w1_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Freescale Semiconductors Inc");
    MODULE_DESCRIPTION("Driver for One-Wire on MXC");
