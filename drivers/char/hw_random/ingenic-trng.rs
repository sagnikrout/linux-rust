//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/ingenic-trng.c
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
// Ingenic True Random Number Generator driver
// Copyright (c) 2019 漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>
// Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// DTRNG register offsets
pub const TRNG_REG_CFG_OFFSET: c_uint = 0x00;
pub const TRNG_REG_RANDOMNUM_OFFSET: c_uint = 0x04;
pub const TRNG_REG_STATUS_OFFSET: c_uint = 0x08;
// bits within the CFG register

// bits within the STATUS register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_trng {
    pub base: *mut void __iomem,
    pub rng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn ingenic_trng_init(rng: *mut hwrng) -> c_int {
    static int ingenic_trng_init(struct hwrng *rng)
    {
    struct ingenic_trng *trng = container_of(rng, struct ingenic_trng, rng);
    unsigned int ctrl;
    ctrl = readl(trng.base + TRNG_REG_CFG_OFFSET);
    ctrl |= CFG_GEN_EN;
    writel(ctrl, trng.base + TRNG_REG_CFG_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_trng_cleanup(rng: *mut hwrng) {
    static void ingenic_trng_cleanup(struct hwrng *rng)
    {
    struct ingenic_trng *trng = container_of(rng, struct ingenic_trng, rng);
    unsigned int ctrl;
    ctrl = readl(trng.base + TRNG_REG_CFG_OFFSET);
    ctrl &= ~CFG_GEN_EN;
    writel(ctrl, trng.base + TRNG_REG_CFG_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int ingenic_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct ingenic_trng *trng = container_of(rng, struct ingenic_trng, rng);
    u32 *data = buf;
    u32 status;
    int ret;
    ret = readl_poll_timeout(trng.base + TRNG_REG_STATUS_OFFSET, status,
    status & STATUS_RANDOM_RDY, 10, 1000);
    if (ret == -ETIMEDOUT) {
    pr_err("%s: Wait for DTRNG data ready timeout\n", __func__);
    return ret;
    }
// data = readl(trng->base + TRNG_REG_RANDOMNUM_OFFSET);
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_trng_probe(pdev: *mut platform_device) -> c_int {
    static int ingenic_trng_probe(struct platform_device *pdev)
    {
    struct ingenic_trng *trng;
    struct clk *clk;
    int ret;
    trng = devm_kzalloc(&pdev.dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.base))
    return dev_err_probe(&pdev.dev, PTR_ERR(trng.base),
    "%s: Failed to map DTRNG registers\n", __func__);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk),
    "%s: Cannot get and enable DTRNG clock\n", __func__);
    trng.rng.name = pdev.name;
    trng.rng.init = ingenic_trng_init;
    trng.rng.cleanup = ingenic_trng_cleanup;
    trng.rng.read = ingenic_trng_read;
    ret = devm_hwrng_register(&pdev.dev, &trng.rng);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register hwrng\n");
    platform_set_drvdata(pdev, trng);
    dev_info(&pdev.dev, "Ingenic DTRNG driver registered\n");
    return 0;
    }
    static const struct of_device_id ingenic_trng_of_match[] = {
    { .compatible = "ingenic,x1830-dtrng" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ingenic_trng_of_match);
    static struct platform_driver ingenic_trng_driver = {
    .probe		= ingenic_trng_probe,
    .driver		= {
    .name	= "ingenic-trng",
    .of_match_table = ingenic_trng_of_match,
    },
    };
    module_platform_driver(ingenic_trng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("漆鹏振 (Qi Pengzhen) <aric.pzqi@ingenic.com>");
    MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
    MODULE_DESCRIPTION("Ingenic True Random Number Generator driver");
