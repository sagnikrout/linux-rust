//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/bcm2835-rng.c
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
// Copyright (c) 2010-2012 Broadcom. All rights reserved.
// Copyright (c) 2013 Lubomir Rintel
//

pub const RNG_CTRL: c_uint = 0x0;
pub const RNG_STATUS: c_uint = 0x4;
pub const RNG_DATA: c_uint = 0x8;
pub const RNG_INT_MASK: c_uint = 0x10;
// enable rng
pub const RNG_RBGEN: c_uint = 0x1;
// the initial numbers generated are "less random" so will be discarded
pub const RNG_WARMUP_COUNT: c_uint = 0x40000;
pub const RNG_INT_OFF: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_rng_priv {
    pub rng: hwrng,
    pub base: *mut void __iomem,
    pub mask_interrupts: bool,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
}

    static inline struct bcm2835_rng_priv *to_rng_priv(struct hwrng *rng)
    {
    return container_of(rng, struct bcm2835_rng_priv, rng);
    }
#[no_mangle]
pub unsafe extern "C" fn rng_readl(priv: *mut bcm2835_rng_priv, offset: u32) -> u32 {
    static inline u32 rng_readl(struct bcm2835_rng_priv *priv, u32 offset)
    {
// MIPS chips strapped for BE will automagically configure the
// peripheral registers for CPU-native byte order.
//
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    return __raw_readl(priv.base + offset);
    else
    return readl(priv.base + offset);
    }
    static inline void rng_writel(struct bcm2835_rng_priv *priv, u32 val,
    u32 offset)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    __raw_writel(val, priv.base + offset);
    else
    writel(val, priv.base + offset);
    }
    static int bcm2835_rng_read(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct bcm2835_rng_priv *priv = to_rng_priv(rng);
    let mut max_words: u32 = max / sizeof(u32);
    u32 num_words, count;
    while ((rng_readl(priv, RNG_STATUS) >> 24) == 0) {
    if (!wait)
    return 0;
    hwrng_yield(rng);
    }
    num_words = rng_readl(priv, RNG_STATUS) >> 24;
    if (num_words > max_words)
    num_words = max_words;
    for (count = 0; count < num_words; count++)
    ((u32 *)buf)[count] = rng_readl(priv, RNG_DATA);
    return num_words * sizeof(u32);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_rng_init(rng: *mut hwrng) -> c_int {
    static int bcm2835_rng_init(struct hwrng *rng)
    {
    struct bcm2835_rng_priv *priv = to_rng_priv(rng);
    let mut ret: c_int = 0;
    u32 val;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    ret = reset_control_reset(priv.reset);
    if (ret) {
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    if (priv.mask_interrupts) {
// mask the interrupt
    val = rng_readl(priv, RNG_INT_MASK);
    val |= RNG_INT_OFF;
    rng_writel(priv, val, RNG_INT_MASK);
    }
// set warm-up count & enable
    rng_writel(priv, RNG_WARMUP_COUNT, RNG_STATUS);
    rng_writel(priv, RNG_RBGEN, RNG_CTRL);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_rng_cleanup(rng: *mut hwrng) {
    static void bcm2835_rng_cleanup(struct hwrng *rng)
    {
    struct bcm2835_rng_priv *priv = to_rng_priv(rng);
// disable rng hardware
    rng_writel(priv, 0, RNG_CTRL);
    clk_disable_unprepare(priv.clk);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_rng_of_data {
    pub mask_interrupts: bool,
}

    static const struct bcm2835_rng_of_data nsp_rng_of_data = {
    .mask_interrupts = true,
    };
    static const struct of_device_id bcm2835_rng_of_match[] = {
    { .compatible = "brcm,bcm2835-rng"},
    { .compatible = "brcm,bcm-nsp-rng", .data = &nsp_rng_of_data },
    { .compatible = "brcm,bcm5301x-rng", .data = &nsp_rng_of_data },
    { .compatible = "brcm,bcm6368-rng"},
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_rng_of_match);
#[no_mangle]
unsafe extern "C" fn bcm2835_rng_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm2835_rng_priv *priv;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// map peripheral
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
// Clock is optional on most platforms
    priv.clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.reset = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    priv.rng.name = pdev.name;
    priv.rng.init = bcm2835_rng_init;
    priv.rng.read = bcm2835_rng_read;
    priv.rng.cleanup = bcm2835_rng_cleanup;
    if (dev_of_node(dev)) {
    const struct bcm2835_rng_of_data *of_data;
// Check for rng init function, execute it
    of_data = of_device_get_match_data(dev);
    if (of_data)
    priv.mask_interrupts = of_data.mask_interrupts;
    }
// register driver
    err = devm_hwrng_register(dev, &priv.rng);
    if (err)
    dev_err(dev, "hwrng registration failed\n");
    else
    dev_info(dev, "hwrng registered\n");
    return err;
    }
    static const struct platform_device_id bcm2835_rng_devtype[] = {
    { .name = "bcm2835-rng" },
    { .name = "bcm63xx-rng" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, bcm2835_rng_devtype);
    static struct platform_driver bcm2835_rng_driver = {
    .driver = {
    .name = "bcm2835-rng",
    .of_match_table = bcm2835_rng_of_match,
    },
    .probe		= bcm2835_rng_probe,
    .id_table	= bcm2835_rng_devtype,
    };
    module_platform_driver(bcm2835_rng_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("BCM2835 Random Number Generator (RNG) driver");
    MODULE_LICENSE("GPL v2");
