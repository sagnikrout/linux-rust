//! Automatically rewritten from C to Rust
//! Source: drivers/soc/canaan/k210-sysctl.c
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
// Copyright (c) 2019 Christoph Hellwig.
// Copyright (c) 2019 Western Digital Corporation or its affiliates.
//

#[no_mangle]
unsafe extern "C" fn k210_sysctl_probe(pdev: *mut platform_device) -> c_int {
    static int k210_sysctl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct clk *pclk;
    int ret;
    dev_info(dev, "K210 system controller\n");
// Get power bus clock
    pclk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pclk))
    return dev_err_probe(dev, PTR_ERR(pclk),
    "Get bus clock failed\n");
    ret = clk_prepare_enable(pclk);
    if (ret) {
    dev_err(dev, "Enable bus clock failed\n");
    return ret;
    }
// Populate children
    ret = devm_of_platform_populate(dev);
    if (ret)
    dev_err(dev, "Populate platform failed %d\n", ret);
    return ret;
    }
    static const struct of_device_id k210_sysctl_of_match[] = {
    { .compatible = "canaan,k210-sysctl", },
    { /* sentinel */ },
    };
    static struct platform_driver k210_sysctl_driver = {
    .driver	= {
    .name		= "k210-sysctl",
    .of_match_table	= k210_sysctl_of_match,
    },
    .probe			= k210_sysctl_probe,
    };
    builtin_platform_driver(k210_sysctl_driver);
//
// System controller registers base address and size.
//
pub const K210_SYSCTL_BASE_ADDR: c_uint = 0x50440000ULL;
pub const K210_SYSCTL_BASE_SIZE: c_uint = 0x1000;
//
// This needs to be called very early during initialization, given that
// PLL1 needs to be enabled to be able to use all SRAM.
//
#[no_mangle]
unsafe extern "C" fn k210_soc_early_init(fdt: *const c_void) -> void __init {
    static void __init k210_soc_early_init(const void *fdt)
    {
    void __iomem *sysctl_base;
    sysctl_base = ioremap(K210_SYSCTL_BASE_ADDR, K210_SYSCTL_BASE_SIZE);
    if (!sysctl_base)
    panic("k210-sysctl: ioremap failed");
    k210_clk_early_init(sysctl_base);
    iounmap(sysctl_base);
    }
    SOC_EARLY_INIT_DECLARE(k210_soc, "canaan,kendryte-k210", k210_soc_early_init);
