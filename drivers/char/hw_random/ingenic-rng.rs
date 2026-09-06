//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/ingenic-rng.c
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
// Ingenic Random Number Generator driver
// Copyright (c) 2017 PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>
// Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// RNG register offsets
pub const RNG_REG_ERNG_OFFSET: c_uint = 0x0;
pub const RNG_REG_RNG_OFFSET: c_uint = 0x4;
// bits within the ERND register

    enum ingenic_rng_version {
    ID_JZ4780,
    ID_X1000,
    };
// Device associated memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_rng {
    pub version: enum ingenic_rng_version,
    pub base: *mut void __iomem,
    pub rng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn ingenic_rng_init(rng: *mut hwrng) -> c_int {
    static int ingenic_rng_init(struct hwrng *rng)
    {
    struct ingenic_rng *priv = container_of(rng, struct ingenic_rng, rng);
    writel(ERNG_ENABLE, priv.base + RNG_REG_ERNG_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rng_cleanup(rng: *mut hwrng) {
    static void ingenic_rng_cleanup(struct hwrng *rng)
    {
    struct ingenic_rng *priv = container_of(rng, struct ingenic_rng, rng);
    writel(0, priv.base + RNG_REG_ERNG_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int ingenic_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct ingenic_rng *priv = container_of(rng, struct ingenic_rng, rng);
    u32 *data = buf;
    u32 status;
    int ret;
    if (priv.version >= ID_X1000) {
    ret = readl_poll_timeout(priv.base + RNG_REG_ERNG_OFFSET, status,
    status & ERNG_READY, 10, 1000);
    if (ret == -ETIMEDOUT) {
    pr_err("%s: Wait for RNG data ready timeout\n", __func__);
    return ret;
    }
    } else {
//
// A delay is required so that the current RNG data is not bit shifted
// version of previous RNG data which could happen if random data is
// read continuously from this device.
//
    udelay(20);
    }
// data = readl(priv->base + RNG_REG_RNG_OFFSET);
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rng_probe(pdev: *mut platform_device) -> c_int {
    static int ingenic_rng_probe(struct platform_device *pdev)
    {
    struct ingenic_rng *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    pr_err("%s: Failed to map RNG registers\n", __func__);
    return PTR_ERR(priv.base);
    }
    priv.version = (enum ingenic_rng_version)(uintptr_t)of_device_get_match_data(&pdev.dev);
    priv.rng.name = pdev.name;
    priv.rng.init = ingenic_rng_init;
    priv.rng.cleanup = ingenic_rng_cleanup;
    priv.rng.read = ingenic_rng_read;
    ret = hwrng_register(&priv.rng);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register hwrng\n");
    return ret;
    }
    platform_set_drvdata(pdev, priv);
    dev_info(&pdev.dev, "Ingenic RNG driver registered\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_rng_remove(pdev: *mut platform_device) {
    static void ingenic_rng_remove(struct platform_device *pdev)
    {
    struct ingenic_rng *priv = platform_get_drvdata(pdev);
    hwrng_unregister(&priv.rng);
    writel(0, priv.base + RNG_REG_ERNG_OFFSET);
    }
    static const struct of_device_id ingenic_rng_of_match[] = {
    { .compatible = "ingenic,jz4780-rng", .data = (void *) ID_JZ4780 },
    { .compatible = "ingenic,x1000-rng", .data = (void *) ID_X1000 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ingenic_rng_of_match);
    static struct platform_driver ingenic_rng_driver = {
    .probe		= ingenic_rng_probe,
    .remove		= ingenic_rng_remove,
    .driver		= {
    .name	= "ingenic-rng",
    .of_match_table = ingenic_rng_of_match,
    },
    };
    module_platform_driver(ingenic_rng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("PrasannaKumar Muralidharan <prasannatsmkumar@gmail.com>");
    MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
    MODULE_DESCRIPTION("Ingenic Random Number Generator driver");
