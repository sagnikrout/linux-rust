//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/pic32-rng.c
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
// PIC32 RNG driver
//
// Joshua Henderson <joshua.henderson@microchip.com>
// Copyright (C) 2016 Microchip Technology Inc.  All rights reserved.
//

pub const RNGCON: c_uint = 0x04;

pub const RNGSEED1: c_uint = 0x18;
pub const RNGSEED2: c_uint = 0x1C;
pub const RNGRCNT: c_uint = 0x20;
pub const RCNT_MASK: c_uint = 0x7F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_rng {
    pub base: *mut void __iomem,
    pub rng: hwrng,
}

//
// The TRNG can generate up to 24Mbps. This is a timeout that should be safe
// enough given the instructions in the loop and that the TRNG may not always
// be at maximum rate.
//
pub const RNG_TIMEOUT: c_int = 500;
#[no_mangle]
unsafe extern "C" fn pic32_rng_init(rng: *mut hwrng) -> c_int {
    static int pic32_rng_init(struct hwrng *rng)
    {
    struct pic32_rng *priv = container_of(rng, struct pic32_rng, rng);
// enable TRNG in enhanced mode
    writel(TRNGEN | TRNGMOD, priv.base + RNGCON);
    return 0;
    }
    static int pic32_rng_read(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct pic32_rng *priv = container_of(rng, struct pic32_rng, rng);
    u64 *data = buf;
    u32 t;
    let mut timeout: c_uint = RNG_TIMEOUT;
    do {
    t = readl(priv.base + RNGRCNT) & RCNT_MASK;
    if (t == 64) {
// TRNG value comes through the seed registers
// data = ((u64)readl(priv->base + RNGSEED2) << 32) +
    readl(priv.base + RNGSEED1);
    return 8;
    }
    } while (wait && --timeout);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rng_cleanup(rng: *mut hwrng) {
    static void pic32_rng_cleanup(struct hwrng *rng)
    {
    struct pic32_rng *priv = container_of(rng, struct pic32_rng, rng);
    writel(0, priv.base + RNGCON);
    }
#[no_mangle]
unsafe extern "C" fn pic32_rng_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_rng_probe(struct platform_device *pdev)
    {
    struct pic32_rng *priv;
    struct clk *clk;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    priv.rng.name = pdev.name;
    priv.rng.init = pic32_rng_init;
    priv.rng.read = pic32_rng_read;
    priv.rng.cleanup = pic32_rng_cleanup;
    return devm_hwrng_register(&pdev.dev, &priv.rng);
    }
    static const struct of_device_id pic32_rng_of_match[] __maybe_unused = {
    { .compatible	= "microchip,pic32mzda-rng", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pic32_rng_of_match);
    static struct platform_driver pic32_rng_driver = {
    .probe		= pic32_rng_probe,
    .driver		= {
    .name	= "pic32-rng",
    .of_match_table = pic32_rng_of_match,
    },
    };
    module_platform_driver(pic32_rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joshua Henderson <joshua.henderson@microchip.com>");
    MODULE_DESCRIPTION("Microchip PIC32 RNG Driver");
