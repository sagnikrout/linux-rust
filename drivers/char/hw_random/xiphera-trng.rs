//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/xiphera-trng.c
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
// Copyright (C) 2020 Xiphera Ltd.

pub const CONTROL_REG: c_uint = 0x00000000;
pub const STATUS_REG: c_uint = 0x00000004;
pub const RAND_REG: c_uint = 0x00000000;
pub const HOST_TO_TRNG_RESET: c_uint = 0x00000001;
pub const HOST_TO_TRNG_RELEASE_RESET: c_uint = 0x00000002;
pub const HOST_TO_TRNG_ENABLE: c_uint = 0x80000000;
pub const HOST_TO_TRNG_ZEROIZE: c_uint = 0x80000004;
pub const HOST_TO_TRNG_ACK_ZEROIZE: c_uint = 0x80000008;
pub const HOST_TO_TRNG_READ: c_uint = 0x8000000F;
// trng statuses
pub const TRNG_ACK_RESET: c_uint = 0x000000AC;
pub const TRNG_SUCCESSFUL_STARTUP: c_uint = 0x00000057;
pub const TRNG_FAILED_STARTUP: c_uint = 0x000000FA;
pub const TRNG_NEW_RAND_AVAILABLE: c_uint = 0x000000ED;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xiphera_trng {
    pub mem: *mut void __iomem,
    pub rng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn xiphera_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int xiphera_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct xiphera_trng *trng = container_of(rng, struct xiphera_trng, rng);
    let mut ret: c_int = 0;
    while (max >= sizeof(u32)) {
// check for data
    if (readl(trng.mem + STATUS_REG) == TRNG_NEW_RAND_AVAILABLE) {
// (u32 *)buf = readl(trng->mem + RAND_REG);
//
// Inform the trng of the read
// and re-enable it to produce a new random number
//
    writel(HOST_TO_TRNG_READ, trng.mem + CONTROL_REG);
    writel(HOST_TO_TRNG_ENABLE, trng.mem + CONTROL_REG);
    ret += sizeof(u32);
    buf += sizeof(u32);
    max -= sizeof(u32);
    } else {
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xiphera_trng_probe(pdev: *mut platform_device) -> c_int {
    static int xiphera_trng_probe(struct platform_device *pdev)
    {
    int ret;
    struct xiphera_trng *trng;
    struct device *dev = &pdev.dev;
    trng = devm_kzalloc(dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.mem = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.mem))
    return PTR_ERR(trng.mem);
//
// the trng needs to be reset first which might not happen in time,
// hence we incorporate a small delay to ensure proper behaviour
//
    writel(HOST_TO_TRNG_RESET, trng.mem + CONTROL_REG);
    usleep_range(100, 200);
    if (readl(trng.mem + STATUS_REG) != TRNG_ACK_RESET) {
//
// there is a small chance the trng is just not ready yet,
// so we try one more time. If the second time fails, we give up
//
    usleep_range(100, 200);
    if (readl(trng.mem + STATUS_REG) != TRNG_ACK_RESET) {
    dev_err(dev, "failed to reset the trng ip\n");
    return -ENODEV;
    }
    }
//
// once again, to ensure proper behaviour we sleep
// for a while after zeroizing the trng
//
    writel(HOST_TO_TRNG_RELEASE_RESET, trng.mem + CONTROL_REG);
    writel(HOST_TO_TRNG_ENABLE, trng.mem + CONTROL_REG);
    writel(HOST_TO_TRNG_ZEROIZE, trng.mem + CONTROL_REG);
    msleep(20);
    if (readl(trng.mem + STATUS_REG) != TRNG_SUCCESSFUL_STARTUP) {
// diagnose the reason for the failure
    if (readl(trng.mem + STATUS_REG) == TRNG_FAILED_STARTUP) {
    dev_err(dev, "trng ip startup-tests failed\n");
    return -ENODEV;
    }
    dev_err(dev, "startup-tests yielded no response\n");
    return -ENODEV;
    }
    writel(HOST_TO_TRNG_ACK_ZEROIZE, trng.mem + CONTROL_REG);
    trng.rng.name = pdev.name;
    trng.rng.read = xiphera_trng_read;
    trng.rng.quality = 900;
    ret = devm_hwrng_register(dev, &trng.rng);
    if (ret) {
    dev_err(dev, "failed to register rng device: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id xiphera_trng_of_match[] = {
    { .compatible = "xiphera,xip8001b-trng", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xiphera_trng_of_match);
    static struct platform_driver xiphera_trng_driver = {
    .driver = {
    .name = "xiphera-trng",
    .of_match_table	= xiphera_trng_of_match,
    },
    .probe = xiphera_trng_probe,
    };
    module_platform_driver(xiphera_trng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Atte Tommiska");
    MODULE_DESCRIPTION("Xiphera FPGA-based true random number generator driver");
