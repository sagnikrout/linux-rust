//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/histb-rng.c
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


// SPDX-License-Identifier: GPL-2.0-or-later OR MIT
//
// Copyright (c) 2023 David Yang
//

pub const RNG_CTRL: c_uint = 0x0;

pub const RNG_NUMBER: c_uint = 0x4;
pub const RNG_STAT: c_uint = 0x8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct histb_rng_priv {
    pub rng: hwrng,
    pub base: *mut void __iomem,
}

//
// Observed:
// depth = 1 -> ~1ms
// depth = 255 -> ~16ms
//
#[no_mangle]
unsafe extern "C" fn histb_rng_wait(base: *mut void __iomem) -> c_int {
    static int histb_rng_wait(void __iomem *base)
    {
    u32 val;
    return readl_relaxed_poll_timeout(base + RNG_STAT, val,
    val & DATA_COUNT, 1000, 30 * 1000);
    }
#[no_mangle]
unsafe extern "C" fn histb_rng_init(base: *mut void __iomem, depth: c_uint) {
    static void histb_rng_init(void __iomem *base, unsigned int depth)
    {
    u32 val;
    val = readl_relaxed(base + RNG_CTRL);
    val &= ~RNG_SOURCE;
    val |= 2;
    val &= ~POST_PROCESS_DEPTH;
    val |= min(depth, 0xffu) << 8;
    val |= POST_PROCESS_ENABLE;
    val |= DROP_ENABLE;
    writel_relaxed(val, base + RNG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn histb_rng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int histb_rng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    struct histb_rng_priv *priv = container_of(rng, typeof(*priv), rng);
    void __iomem *base = priv.base;
    for (int i = 0; i < max; i += sizeof(u32)) {
    if (!(readl_relaxed(base + RNG_STAT) & DATA_COUNT)) {
    if (!wait)
    return i;
    if (histb_rng_wait(base)) {
    pr_err("failed to generate random number, generated %d\n",
    i);
    return i ? i : -ETIMEDOUT;
    }
    }
// (u32 *) (data + i) = readl_relaxed(base + RNG_NUMBER);
    }
    return max;
    }
#[no_mangle]
unsafe extern "C" fn histb_rng_get_depth(base: *mut void __iomem) -> c_uint {
    static unsigned int histb_rng_get_depth(void __iomem *base)
    {
    return (readl_relaxed(base + RNG_CTRL) & POST_PROCESS_DEPTH) >> 8;
    }
    static ssize_t
    depth_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct histb_rng_priv *priv = dev_get_drvdata(dev);
    void __iomem *base = priv.base;
    return sprintf(buf, "%u\n", histb_rng_get_depth(base));
    }
    static ssize_t
    depth_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct histb_rng_priv *priv = dev_get_drvdata(dev);
    void __iomem *base = priv.base;
    unsigned int depth;
    if (kstrtouint(buf, 0, &depth))
    return -ERANGE;
    histb_rng_init(base, depth);
    return count;
    }
    static DEVICE_ATTR_RW(depth);
    static struct attribute *histb_rng_attrs[] = {
    &dev_attr_depth.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(histb_rng);
#[no_mangle]
unsafe extern "C" fn histb_rng_probe(pdev: *mut platform_device) -> c_int {
    static int histb_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct histb_rng_priv *priv;
    void __iomem *base;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    histb_rng_init(base, 144);
    if (histb_rng_wait(base)) {
    dev_err(dev, "cannot bring up device\n");
    return -ENODEV;
    }
    priv.base = base;
    priv.rng.name = pdev.name;
    priv.rng.read = histb_rng_read;
    ret = devm_hwrng_register(dev, &priv.rng);
    if (ret) {
    dev_err(dev, "failed to register hwrng: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, priv);
    dev_set_drvdata(dev, priv);
    return 0;
    }
    static const struct of_device_id histb_rng_of_match[] = {
    { .compatible = "hisilicon,histb-rng", },
    { }
    };
    MODULE_DEVICE_TABLE(of, histb_rng_of_match);
    static struct platform_driver histb_rng_driver = {
    .probe = histb_rng_probe,
    .driver = {
    .name = "histb-rng",
    .of_match_table = histb_rng_of_match,
    .dev_groups = histb_rng_groups,
    },
    };
    module_platform_driver(histb_rng_driver);
    MODULE_DESCRIPTION("Hisilicon STB random number generator driver");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_AUTHOR("David Yang <mmyangfl@gmail.com>");
