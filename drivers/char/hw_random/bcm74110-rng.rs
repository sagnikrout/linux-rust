//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/bcm74110-rng.c
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
// Copyright (c) 2024 Broadcom
//

pub const HOST_REV_ID: c_uint = 0x00;
pub const HOST_FIFO_DEPTH: c_uint = 0x04;
pub const HOST_FIFO_COUNT: c_uint = 0x08;
pub const HOST_FIFO_THRESHOLD: c_uint = 0x0c;
pub const HOST_FIFO_DATA: c_uint = 0x10;
pub const HOST_FIFO_COUNT_MASK: c_uint = 0xffff;
// Delay range in microseconds
pub const FIFO_DELAY_MIN_US: c_int = 3;
pub const FIFO_DELAY_MAX_US: c_int = 7;
pub const FIFO_DELAY_MAX_COUNT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm74110_priv {
    pub base: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn bcm74110_rng_fifo_count(mem: *mut void __iomem) -> c_int {
    static inline int bcm74110_rng_fifo_count(void __iomem *mem)
    {
    return readl_relaxed(mem) & HOST_FIFO_COUNT_MASK;
    }
    static int bcm74110_rng_read(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct bcm74110_priv *priv = (struct bcm74110_priv *)rng.priv;
    void __iomem *fc_addr = priv.base + HOST_FIFO_COUNT;
    void __iomem *fd_addr = priv.base + HOST_FIFO_DATA;
    let mut underrun_count: unsigned = 0;
    let mut max_words: u32 = max / sizeof(u32);
    u32 num_words;
    unsigned i;
//
// We need to check how many words are available in the RNG FIFO. If
// there aren't any, we need to wait for some to become available.
//
    while ((num_words = bcm74110_rng_fifo_count(fc_addr)) == 0) {
    if (!wait)
    return 0;
//
// As a precaution, limit how long we wait. If the FIFO doesn't
// refill within the allotted time, return 0 (=no data) to the
// caller.
//
    if (likely(underrun_count < FIFO_DELAY_MAX_COUNT))
    usleep_range(FIFO_DELAY_MIN_US, FIFO_DELAY_MAX_US);
    else
    return 0;
    underrun_count++;
    }
    if (num_words > max_words)
    num_words = max_words;
// Bail early if we run out of random numbers unexpectedly
    for (i = 0; i < num_words && bcm74110_rng_fifo_count(fc_addr) > 0; i++)
    ((u32 *)buf)[i] = readl_relaxed(fd_addr);
    return i * sizeof(u32);
    }
    static struct hwrng bcm74110_hwrng = {
    .read = bcm74110_rng_read,
    };
#[no_mangle]
unsafe extern "C" fn bcm74110_rng_probe(pdev: *mut platform_device) -> c_int {
    static int bcm74110_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm74110_priv *priv;
    int rc;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    bcm74110_hwrng.name = pdev.name;
    bcm74110_hwrng.priv = (unsigned long)priv;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    rc = devm_hwrng_register(dev, &bcm74110_hwrng);
    if (rc)
    dev_err(dev, "hwrng registration failed (%d)\n", rc);
    else
    dev_info(dev, "hwrng registered\n");
    return rc;
    }
    static const struct of_device_id bcm74110_rng_match[] = {
    { .compatible	= "brcm,bcm74110-rng", },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm74110_rng_match);
    static struct platform_driver bcm74110_rng_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = bcm74110_rng_match,
    },
    .probe	= bcm74110_rng_probe,
    };
    module_platform_driver(bcm74110_rng_driver);
    MODULE_AUTHOR("Markus Mayer <mmayer@broadcom.com>");
    MODULE_DESCRIPTION("BCM 74110 Random Number Generator (RNG) driver");
    MODULE_LICENSE("GPL v2");
