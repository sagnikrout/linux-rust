//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/meson-rng.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2016 BayLibre, SAS.
// Author: Neil Armstrong <narmstrong@baylibre.com>
// Copyright (C) 2014 Amlogic, Inc.
//

pub const RNG_DATA: c_uint = 0x00;
pub const RNG_S4_DATA: c_uint = 0x08;
pub const RNG_S4_CFG: c_uint = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_rng_priv {
    pub wait): *mut *mut *mut *mut int (read)(struct hwrng rng, void buf, size_t max, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_rng_data {
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn meson_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int meson_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct meson_rng_data *data =
    container_of(rng, struct meson_rng_data, rng);
// (u32 *)buf = readl_relaxed(data->base + RNG_DATA);
    return sizeof(u32);
    }
#[no_mangle]
unsafe extern "C" fn meson_rng_wait_status(cfg_addr: *mut void __iomem, bit: c_int) -> c_int {
    static int meson_rng_wait_status(void __iomem *cfg_addr, int bit)
    {
    let mut status: u32 = 0;
    int ret;
    ret = readl_relaxed_poll_timeout_atomic(cfg_addr,
    status, !(status & bit),
    10, 10000);
    if (ret)
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_s4_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int meson_s4_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct meson_rng_data *data =
    container_of(rng, struct meson_rng_data, rng);
    void __iomem *cfg_addr = data.base + RNG_S4_CFG;
    int err;
    writel_relaxed(readl_relaxed(cfg_addr) | SEED_READY_STS_BIT, cfg_addr);
    err = meson_rng_wait_status(cfg_addr, SEED_READY_STS_BIT);
    if (err) {
    dev_err(data.dev, "Seed isn't ready, try again\n");
    return err;
    }
    err = meson_rng_wait_status(cfg_addr, RUN_BIT);
    if (err) {
    dev_err(data.dev, "Can't get random number, try again\n");
    return err;
    }
// (u32 *)buf = readl_relaxed(data->base + RNG_S4_DATA);
    return sizeof(u32);
    }
#[no_mangle]
unsafe extern "C" fn meson_rng_probe(pdev: *mut platform_device) -> c_int {
    static int meson_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct meson_rng_data *data;
    struct clk *core_clk;
    const struct meson_rng_priv *priv;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    priv = device_get_match_data(&pdev.dev);
    if (!priv)
    return -ENODEV;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base))
    return PTR_ERR(data.base);
    core_clk = devm_clk_get_optional_enabled(dev, "core");
    if (IS_ERR(core_clk))
    return dev_err_probe(dev, PTR_ERR(core_clk),
    "Failed to get core clock\n");
    data.rng.name = pdev.name;
    data.rng.read = priv.read;
    data.dev = &pdev.dev;
    return devm_hwrng_register(dev, &data.rng);
    }
    static const struct meson_rng_priv meson_rng_priv = {
    .read = meson_rng_read,
    };
    static const struct meson_rng_priv meson_rng_priv_s4 = {
    .read = meson_s4_rng_read,
    };
    static const struct of_device_id meson_rng_of_match[] = {
    {
    .compatible = "amlogic,meson-rng",
    .data = (void *)&meson_rng_priv,
    },
    {
    .compatible = "amlogic,meson-s4-rng",
    .data = (void *)&meson_rng_priv_s4,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, meson_rng_of_match);
    static struct platform_driver meson_rng_driver = {
    .probe	= meson_rng_probe,
    .driver	= {
    .name = "meson-rng",
    .of_match_table = meson_rng_of_match,
    },
    };
    module_platform_driver(meson_rng_driver);
    MODULE_DESCRIPTION("Meson H/W Random Number Generator driver");
    MODULE_AUTHOR("Lawrence Mok <lawrence.mok@amlogic.com>");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_LICENSE("Dual BSD/GPL");
