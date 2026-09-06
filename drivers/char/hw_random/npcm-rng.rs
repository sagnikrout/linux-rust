//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/npcm-rng.c
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
// Copyright (c) 2019 Nuvoton Technology corporation.

pub const NPCM_RNGCS_REG: c_uint = 0x00	/* Control and status register */;
pub const NPCM_RNGD_REG: c_uint = 0x04	/* Data register */;
pub const NPCM_RNGMODE_REG: c_uint = 0x08	/* Mode register */;

pub const NPCM_RNG_TIMEOUT_USEC: c_int = 20000;
pub const NPCM_RNG_POLL_USEC: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npcm_rng {
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub dev: *mut device,
    pub clkp: u32,
}

#[no_mangle]
unsafe extern "C" fn npcm_rng_init(rng: *mut hwrng) -> c_int {
    static int npcm_rng_init(struct hwrng *rng)
    {
    struct npcm_rng *priv = to_npcm_rng(rng);
    writel(priv.clkp | NPCM_RNG_ENABLE, priv.base + NPCM_RNGCS_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_rng_cleanup(rng: *mut hwrng) {
    static void npcm_rng_cleanup(struct hwrng *rng)
    {
    struct npcm_rng *priv = to_npcm_rng(rng);
    writel(priv.clkp, priv.base + NPCM_RNGCS_REG);
    }
#[no_mangle]
unsafe extern "C" fn npcm_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int npcm_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct npcm_rng *priv = to_npcm_rng(rng);
    let mut retval: c_int = 0;
    int ready;
    pm_runtime_get_sync(priv.dev);
    while (max) {
    if (wait) {
    if (readb_poll_timeout(priv.base + NPCM_RNGCS_REG,
    ready,
    ready & NPCM_RNG_DATA_VALID,
    NPCM_RNG_POLL_USEC,
    NPCM_RNG_TIMEOUT_USEC))
    break;
    } else {
    if ((readb(priv.base + NPCM_RNGCS_REG) &
    NPCM_RNG_DATA_VALID) == 0)
    break;
    }
// (u8 *)buf = readb(priv->base + NPCM_RNGD_REG);
    retval++;
    buf++;
    max--;
    }
    pm_runtime_put_sync_autosuspend(priv.dev);
    return retval || !wait ? retval : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn npcm_rng_probe(pdev: *mut platform_device) -> c_int {
    static int npcm_rng_probe(struct platform_device *pdev)
    {
    struct npcm_rng *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    dev_set_drvdata(&pdev.dev, priv);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 100);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_enable(&pdev.dev);

    priv.rng.init = npcm_rng_init;
    priv.rng.cleanup = npcm_rng_cleanup;

    priv.rng.name = pdev.name;
    priv.rng.read = npcm_rng_read;
    priv.dev = &pdev.dev;
    priv.clkp = (u32)(uintptr_t)of_device_get_match_data(&pdev.dev);
    writel(NPCM_RNG_M1ROSEL, priv.base + NPCM_RNGMODE_REG);
    ret = devm_hwrng_register(&pdev.dev, &priv.rng);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register rng device: %d\n",
    ret);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_rng_remove(pdev: *mut platform_device) {
    static void npcm_rng_remove(struct platform_device *pdev)
    {
    struct npcm_rng *priv = platform_get_drvdata(pdev);
    devm_hwrng_unregister(&pdev.dev, &priv.rng);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    }

#[no_mangle]
unsafe extern "C" fn npcm_rng_runtime_suspend(dev: *mut device) -> c_int {
    static int npcm_rng_runtime_suspend(struct device *dev)
    {
    struct npcm_rng *priv = dev_get_drvdata(dev);
    npcm_rng_cleanup(&priv.rng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm_rng_runtime_resume(dev: *mut device) -> c_int {
    static int npcm_rng_runtime_resume(struct device *dev)
    {
    struct npcm_rng *priv = dev_get_drvdata(dev);
    return npcm_rng_init(&priv.rng);
    }

    static const struct dev_pm_ops npcm_rng_pm_ops = {
    SET_RUNTIME_PM_OPS(npcm_rng_runtime_suspend,
    npcm_rng_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id rng_dt_id[] __maybe_unused = {
    { .compatible = "nuvoton,npcm750-rng",
    .data = (void *)NPCM_RNG_CLK_SET_25MHZ },
    { .compatible = "nuvoton,npcm845-rng",
    .data = (void *)NPCM_RNG_CLK_SET_62_5MHZ },
    {},
    };
    MODULE_DEVICE_TABLE(of, rng_dt_id);
    static struct platform_driver npcm_rng_driver = {
    .driver = {
    .name		= "npcm-rng",
    .pm		= &npcm_rng_pm_ops,
    .of_match_table = of_match_ptr(rng_dt_id),
    },
    .probe		= npcm_rng_probe,
    .remove		= npcm_rng_remove,
    };
    module_platform_driver(npcm_rng_driver);
    MODULE_DESCRIPTION("Nuvoton NPCM Random Number Generator Driver");
    MODULE_AUTHOR("Tomer Maimon <tomer.maimon@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
