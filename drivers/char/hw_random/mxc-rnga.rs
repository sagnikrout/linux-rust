//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/mxc-rnga.c
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
// RNG driver for Freescale RNGA
//
// Copyright 2008-2009 Freescale Semiconductor, Inc. All Rights Reserved.
// Author: Alan Carvalho de Assis <acassis@gmail.com>
//
// This driver is based on other RNG drivers.
//

// RNGA Registers
pub const RNGA_CONTROL: c_uint = 0x00;
pub const RNGA_STATUS: c_uint = 0x04;
pub const RNGA_ENTROPY: c_uint = 0x08;
pub const RNGA_OUTPUT_FIFO: c_uint = 0x0c;
pub const RNGA_MODE: c_uint = 0x10;
pub const RNGA_VERIFICATION_CONTROL: c_uint = 0x14;
pub const RNGA_OSC_CONTROL_COUNTER: c_uint = 0x18;
pub const RNGA_OSC1_COUNTER: c_uint = 0x1c;
pub const RNGA_OSC2_COUNTER: c_uint = 0x20;
pub const RNGA_OSC_COUNTER_STATUS: c_uint = 0x24;
// RNGA Registers Range
pub const RNG_ADDR_RANGE: c_uint = 0x28;
// RNGA Control Register
pub const RNGA_CONTROL_SLEEP: c_uint = 0x00000010;
pub const RNGA_CONTROL_CLEAR_INT: c_uint = 0x00000008;
pub const RNGA_CONTROL_MASK_INTS: c_uint = 0x00000004;
pub const RNGA_CONTROL_HIGH_ASSURANCE: c_uint = 0x00000002;
pub const RNGA_CONTROL_GO: c_uint = 0x00000001;
pub const RNGA_STATUS_LEVEL_MASK: c_uint = 0x0000ff00;
// RNGA Status Register
pub const RNGA_STATUS_OSC_DEAD: c_uint = 0x80000000;
pub const RNGA_STATUS_SLEEP: c_uint = 0x00000010;
pub const RNGA_STATUS_ERROR_INT: c_uint = 0x00000008;
pub const RNGA_STATUS_FIFO_UNDERFLOW: c_uint = 0x00000004;
pub const RNGA_STATUS_LAST_READ_STATUS: c_uint = 0x00000002;
pub const RNGA_STATUS_SECURITY_VIOLATION: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_rng {
    pub dev: *mut device,
    pub rng: hwrng,
    pub mem: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn mxc_rnga_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int mxc_rnga_data_present(struct hwrng *rng, int wait)
    {
    int i;
    struct mxc_rng *mxc_rng = container_of(rng, struct mxc_rng, rng);
    for (i = 0; i < 20; i++) {
// how many random numbers are in FIFO? [0-16]
    int level = (__raw_readl(mxc_rng.mem + RNGA_STATUS) &
    RNGA_STATUS_LEVEL_MASK) >> 8;
    if (level || !wait)
    return !!level;
    udelay(10);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rnga_data_read(rng: *mut hwrng, data: *mut *mut u32) -> c_int {
    static int mxc_rnga_data_read(struct hwrng *rng, u32 * data)
    {
    int err;
    u32 ctrl;
    struct mxc_rng *mxc_rng = container_of(rng, struct mxc_rng, rng);
// retrieve a random number from FIFO
// data = __raw_readl(mxc_rng->mem + RNGA_OUTPUT_FIFO);
// some error while reading this random number?
    err = __raw_readl(mxc_rng.mem + RNGA_STATUS) & RNGA_STATUS_ERROR_INT;
// if error: clear error interrupt, but doesn't return random number
    if (err) {
    dev_dbg(mxc_rng.dev, "Error while reading random number!\n");
    ctrl = __raw_readl(mxc_rng.mem + RNGA_CONTROL);
    __raw_writel(ctrl | RNGA_CONTROL_CLEAR_INT,
    mxc_rng.mem + RNGA_CONTROL);
    return 0;
    } else
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rnga_init(rng: *mut hwrng) -> c_int {
    static int mxc_rnga_init(struct hwrng *rng)
    {
    u32 ctrl, osc;
    struct mxc_rng *mxc_rng = container_of(rng, struct mxc_rng, rng);
// wake up
    ctrl = __raw_readl(mxc_rng.mem + RNGA_CONTROL);
    __raw_writel(ctrl & ~RNGA_CONTROL_SLEEP, mxc_rng.mem + RNGA_CONTROL);
// verify if oscillator is working
    osc = __raw_readl(mxc_rng.mem + RNGA_STATUS);
    if (osc & RNGA_STATUS_OSC_DEAD) {
    dev_err(mxc_rng.dev, "RNGA Oscillator is dead!\n");
    return -ENODEV;
    }
// go running
    ctrl = __raw_readl(mxc_rng.mem + RNGA_CONTROL);
    __raw_writel(ctrl | RNGA_CONTROL_GO, mxc_rng.mem + RNGA_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rnga_cleanup(rng: *mut hwrng) {
    static void mxc_rnga_cleanup(struct hwrng *rng)
    {
    u32 ctrl;
    struct mxc_rng *mxc_rng = container_of(rng, struct mxc_rng, rng);
    ctrl = __raw_readl(mxc_rng.mem + RNGA_CONTROL);
// stop rnga
    __raw_writel(ctrl & ~RNGA_CONTROL_GO, mxc_rng.mem + RNGA_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn mxc_rnga_probe(pdev: *mut platform_device) -> c_int {
    static int mxc_rnga_probe(struct platform_device *pdev)
    {
    int err;
    struct mxc_rng *mxc_rng;
    mxc_rng = devm_kzalloc(&pdev.dev, sizeof(*mxc_rng), GFP_KERNEL);
    if (!mxc_rng)
    return -ENOMEM;
    mxc_rng.dev = &pdev.dev;
    mxc_rng.rng.name = "mxc-rnga";
    mxc_rng.rng.init = mxc_rnga_init;
    mxc_rng.rng.cleanup = mxc_rnga_cleanup;
    mxc_rng.rng.data_present = mxc_rnga_data_present;
    mxc_rng.rng.data_read = mxc_rnga_data_read;
    mxc_rng.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mxc_rng.clk)) {
    dev_err(&pdev.dev, "Could not get rng_clk!\n");
    return PTR_ERR(mxc_rng.clk);
    }
    mxc_rng.mem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mxc_rng.mem)) {
    err = PTR_ERR(mxc_rng.mem);
    return err;
    }
    err = hwrng_register(&mxc_rng.rng);
    if (err) {
    dev_err(&pdev.dev, "MXC RNGA registering failed (%d)\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxc_rnga_remove(pdev: *mut platform_device) {
    static void mxc_rnga_remove(struct platform_device *pdev)
    {
    struct mxc_rng *mxc_rng = platform_get_drvdata(pdev);
    hwrng_unregister(&mxc_rng.rng);
    }
    static const struct of_device_id mxc_rnga_of_match[] = {
    { .compatible = "fsl,imx21-rnga", },
    { .compatible = "fsl,imx31-rnga", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, mxc_rnga_of_match);
    static struct platform_driver mxc_rnga_driver = {
    .driver = {
    .name = "mxc_rnga",
    .of_match_table = mxc_rnga_of_match,
    },
    .probe = mxc_rnga_probe,
    .remove = mxc_rnga_remove,
    };
    module_platform_driver(mxc_rnga_driver);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("H/W RNGA driver for i.MX");
    MODULE_LICENSE("GPL");
