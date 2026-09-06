//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/atmel-rng.c
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


//
// Copyright (c) 2011 Peter Korsgaard <jacmet@sunsite.dk>
//
// This file is licensed under  the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const TRNG_CR: c_uint = 0x00;
pub const TRNG_MR: c_uint = 0x04;
pub const TRNG_ISR: c_uint = 0x1c;

pub const TRNG_ODATA: c_uint = 0x50;
pub const TRNG_KEY: c_uint = 0x524e4700 /* RNG */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_trng_data {
    pub has_half_rate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_trng {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub dev: *mut device,
    pub has_half_rate: bool,
}

#[no_mangle]
unsafe extern "C" fn atmel_trng_wait_ready(trng: *mut atmel_trng, wait: bool) -> bool {
    static bool atmel_trng_wait_ready(struct atmel_trng *trng, bool wait)
    {
    int ready;
    ready = readl(trng.base + TRNG_ISR) & TRNG_ISR_DATRDY;
    if (!ready && wait)
    readl_poll_timeout(trng.base + TRNG_ISR, ready,
    ready & TRNG_ISR_DATRDY, 1000, 20000);
    return !!ready;
    }
    static int atmel_trng_read(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct atmel_trng *trng = container_of(rng, struct atmel_trng, rng);
    u32 *data = buf;
    int ret;
    ret = pm_runtime_get_sync(trng.dev);
    if (ret < 0) {
    pm_runtime_put_sync(trng.dev);
    return ret;
    }
    ret = atmel_trng_wait_ready(trng, wait);
    if (!ret)
    goto out;
// data = readl(trng->base + TRNG_ODATA);
//
// ensure data ready is only set again AFTER the next data word is ready
// in case it got set between checking ISR and reading ODATA, so we
// don't risk re-reading the same word
//
    readl(trng.base + TRNG_ISR);
    ret = 4;
    out:
    pm_runtime_put_sync_autosuspend(trng.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_init(trng: *mut atmel_trng) -> c_int {
    static int atmel_trng_init(struct atmel_trng *trng)
    {
    unsigned long rate;
    int ret;
    ret = clk_prepare_enable(trng.clk);
    if (ret)
    return ret;
    if (trng.has_half_rate) {
    rate = clk_get_rate(trng.clk);
// if peripheral clk is above 100MHz, set HALFR
    if (rate > 100000000)
    writel(TRNG_HALFR, trng.base + TRNG_MR);
    }
    writel(TRNG_KEY | 1, trng.base + TRNG_CR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_cleanup(trng: *mut atmel_trng) {
    static void atmel_trng_cleanup(struct atmel_trng *trng)
    {
    writel(TRNG_KEY, trng.base + TRNG_CR);
    clk_disable_unprepare(trng.clk);
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_trng_probe(struct platform_device *pdev)
    {
    struct atmel_trng *trng;
    const struct atmel_trng_data *data;
    int ret;
    trng = devm_kzalloc(&pdev.dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.base))
    return PTR_ERR(trng.base);
    trng.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(trng.clk))
    return PTR_ERR(trng.clk);
    data = of_device_get_match_data(&pdev.dev);
    if (!data)
    return -ENODEV;
    trng.has_half_rate = data.has_half_rate;
    trng.dev = &pdev.dev;
    trng.rng.name = pdev.name;
    trng.rng.read = atmel_trng_read;
    platform_set_drvdata(pdev, trng);

    ret = atmel_trng_init(trng);
    if (ret)
    return ret;

    pm_runtime_set_autosuspend_delay(&pdev.dev, 100);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = devm_hwrng_register(&pdev.dev, &trng.rng);
    if (ret) {
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);

    atmel_trng_cleanup(trng);

    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_remove(pdev: *mut platform_device) {
    static void atmel_trng_remove(struct platform_device *pdev)
    {
    struct atmel_trng *trng = platform_get_drvdata(pdev);
    atmel_trng_cleanup(trng);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused atmel_trng_runtime_suspend(struct device *dev)
    {
    struct atmel_trng *trng = dev_get_drvdata(dev);
    atmel_trng_cleanup(trng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_trng_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused atmel_trng_runtime_resume(struct device *dev)
    {
    struct atmel_trng *trng = dev_get_drvdata(dev);
    return atmel_trng_init(trng);
    }
    static const struct dev_pm_ops atmel_trng_pm_ops = {
    SET_RUNTIME_PM_OPS(atmel_trng_runtime_suspend,
    atmel_trng_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct atmel_trng_data at91sam9g45_config = {
    .has_half_rate = false,
    };
    static const struct atmel_trng_data sam9x60_config = {
    .has_half_rate = true,
    };
    static const struct of_device_id atmel_trng_dt_ids[] = {
    {
    .compatible = "atmel,at91sam9g45-trng",
    .data = &at91sam9g45_config,
    }, {
    .compatible = "microchip,sam9x60-trng",
    .data = &sam9x60_config,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, atmel_trng_dt_ids);
    static struct platform_driver atmel_trng_driver = {
    .probe		= atmel_trng_probe,
    .remove		= atmel_trng_remove,
    .driver		= {
    .name	= "atmel-trng",
    .pm	= pm_ptr(&atmel_trng_pm_ops),
    .of_match_table = atmel_trng_dt_ids,
    },
    };
    module_platform_driver(atmel_trng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Peter Korsgaard <jacmet@sunsite.dk>");
    MODULE_DESCRIPTION("Atmel true random number generator driver");
