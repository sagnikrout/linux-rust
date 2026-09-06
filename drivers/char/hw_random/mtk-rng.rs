//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/mtk-rng.c
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
// Driver for Mediatek Hardware Random Number Generator
//
// Copyright (C) 2017 Sean Wang <sean.wang@mediatek.com>
// Copyright (C) 2026 Daniel Golle <daniel@makrotopia.org>
//

// Runtime PM autosuspend timeout:
pub const RNG_AUTOSUSPEND_TIMEOUT: c_int = 100;
pub const USEC_POLL: c_int = 2;
pub const TIMEOUT_POLL: c_int = 60;
pub const RNG_CTRL: c_uint = 0x00;

pub const RNG_DATA: c_uint = 0x08;
// Driver feature flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rng {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rng: hwrng,
    pub dev: *mut device,
    pub flags: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn mtk_rng_init(rng: *mut hwrng) -> c_int {
    static int mtk_rng_init(struct hwrng *rng)
    {
    struct mtk_rng *priv = to_mtk_rng(rng);
    u32 val;
    int err;
    err = clk_prepare_enable(priv.clk);
    if (err)
    return err;
    val = readl(priv.base + RNG_CTRL);
    val |= RNG_EN;
    writel(val, priv.base + RNG_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_cleanup(rng: *mut hwrng) {
    static void mtk_rng_cleanup(struct hwrng *rng)
    {
    struct mtk_rng *priv = to_mtk_rng(rng);
    u32 val;
    val = readl(priv.base + RNG_CTRL);
    val &= ~RNG_EN;
    writel(val, priv.base + RNG_CTRL);
    clk_disable_unprepare(priv.clk);
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_wait_ready(rng: *mut hwrng, wait: bool) -> bool {
    static bool mtk_rng_wait_ready(struct hwrng *rng, bool wait)
    {
    struct mtk_rng *priv = to_mtk_rng(rng);
    int ready;
    ready = readl(priv.base + RNG_CTRL) & RNG_READY;
    if (!ready && wait)
    readl_poll_timeout_atomic(priv.base + RNG_CTRL, ready,
    ready & RNG_READY, USEC_POLL,
    TIMEOUT_POLL);
    return !!(ready & RNG_READY);
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int mtk_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct mtk_rng *priv = to_mtk_rng(rng);
    let mut retval: c_int = 0;
    pm_runtime_get_sync(priv.dev);
    while (max >= sizeof(u32)) {
    if (!mtk_rng_wait_ready(rng, wait))
    break;
// (u32 *)buf = readl(priv->base + RNG_DATA);
    retval += sizeof(u32);
    buf += sizeof(u32);
    max -= sizeof(u32);
    }
    pm_runtime_put_sync_autosuspend(priv.dev);
    return retval || !wait ? retval : -EIO;
    }
    static int mtk_rng_read_smc(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct arm_smccc_res res;
    let mut retval: c_int = 0;
    while (max >= sizeof(u32)) {
    arm_smccc_smc(MTK_SIP_KERNEL_GET_RND, 0, 0, 0, 0, 0, 0, 0,
    &res);
    if (res.a0)
    break;
// (u32 *)buf = res.a1;
    retval += sizeof(u32);
    buf += sizeof(u32);
    max -= sizeof(u32);
    }
    return retval || !wait ? retval : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_hw_accessible(priv: *mut mtk_rng) -> bool {
    static bool mtk_rng_hw_accessible(struct mtk_rng *priv)
    {
    u32 val;
    int err;
    err = clk_prepare_enable(priv.clk);
    if (err)
    return false;
    val = readl(priv.base + RNG_CTRL);
    val |= RNG_EN;
    writel(val, priv.base + RNG_CTRL);
    val = readl(priv.base + RNG_CTRL);
    if (val & RNG_EN) {
// HW is accessible, clean up: disable RNG and clock
    writel(val & ~RNG_EN, priv.base + RNG_CTRL);
    clk_disable_unprepare(priv.clk);
    return true;
    }
//
// If TF-A blocks direct access, the register reads back as 0.
// Leave the clock enabled as TF-A needs it.
//
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_rng_probe(struct platform_device *pdev)
    {
    int ret;
    struct mtk_rng *priv;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.rng.name = pdev.name;
    priv.rng.quality = 900;
    priv.flags = (unsigned long)device_get_match_data(&pdev.dev);
    if (!(priv.flags & MTK_RNG_SMC)) {
    priv.clk = devm_clk_get(&pdev.dev, "rng");
    if (IS_ERR(priv.clk)) {
    ret = PTR_ERR(priv.clk);
    dev_err(&pdev.dev, "no clock for device: %d\n", ret);
    return ret;
    }
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    if (IS_ENABLED(CONFIG_HAVE_ARM_SMCCC) &&
    of_device_is_compatible(pdev.dev.of_node,
    "mediatek,mt7986-rng") &&
    !mtk_rng_hw_accessible(priv)) {
    priv.flags |= MTK_RNG_SMC;
    dev_info(&pdev.dev,
    "HW RNG not MMIO accessible, using SMC\n");
    }
    }
    if (priv.flags & MTK_RNG_SMC) {
    if (!IS_ENABLED(CONFIG_HAVE_ARM_SMCCC))
    return -ENODEV;
    priv.rng.read = mtk_rng_read_smc;
    } else {

    priv.rng.init = mtk_rng_init;
    priv.rng.cleanup = mtk_rng_cleanup;

    priv.rng.read = mtk_rng_read;
    }
    ret = devm_hwrng_register(&pdev.dev, &priv.rng);
    if (ret) {
    dev_err(&pdev.dev, "failed to register rng device: %d\n",
    ret);
    return ret;
    }
    if (!(priv.flags & MTK_RNG_SMC)) {
    dev_set_drvdata(&pdev.dev, priv);
    pm_runtime_set_autosuspend_delay(&pdev.dev,
    RNG_AUTOSUSPEND_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return ret;
    }
    dev_info(&pdev.dev, "registered RNG driver\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mtk_rng_runtime_suspend(dev: *mut device) -> c_int {
    static int mtk_rng_runtime_suspend(struct device *dev)
    {
    struct mtk_rng *priv = dev_get_drvdata(dev);
    mtk_rng_cleanup(&priv.rng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_rng_runtime_resume(dev: *mut device) -> c_int {
    static int mtk_rng_runtime_resume(struct device *dev)
    {
    struct mtk_rng *priv = dev_get_drvdata(dev);
    return mtk_rng_init(&priv.rng);
    }
    static const struct dev_pm_ops mtk_rng_pm_ops = {
    SET_RUNTIME_PM_OPS(mtk_rng_runtime_suspend,
    mtk_rng_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };

    static const struct of_device_id mtk_rng_match[] = {
    { .compatible = "mediatek,mt7623-rng" },
    { .compatible = "mediatek,mt7981-rng", .data = (void *)MTK_RNG_SMC },
    { .compatible = "mediatek,mt7986-rng" },
    { .compatible = "mediatek,mt7987-rng", .data = (void *)MTK_RNG_SMC },
    { .compatible = "mediatek,mt7988-rng", .data = (void *)MTK_RNG_SMC },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_rng_match);
    static struct platform_driver mtk_rng_driver = {
    .probe          = mtk_rng_probe,
    .driver = {
    .name = MTK_RNG_DEV,
    .pm = MTK_RNG_PM_OPS,
    .of_match_table = mtk_rng_match,
    },
    };
    module_platform_driver(mtk_rng_driver);
    MODULE_DESCRIPTION("Mediatek Random Number Generator Driver");
    MODULE_AUTHOR("Sean Wang <sean.wang@mediatek.com>");
    MODULE_AUTHOR("Daniel Golle <daniel@makrotopia.org>");
    MODULE_LICENSE("GPL");
