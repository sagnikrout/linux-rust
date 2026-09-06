//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/hisi-trng-v2.c
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
// Copyright (c) 2019 HiSilicon Limited.

pub const HISI_TRNG_REG: c_uint = 0x00F0;
pub const HISI_TRNG_BYTES: c_int = 4;
pub const HISI_TRNG_QUALITY: c_int = 512;
pub const SLEEP_US: c_int = 10;
pub const TIMEOUT_US: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_trng {
    pub base: *mut void __iomem,
    pub rng: hwrng,
}

#[no_mangle]
unsafe extern "C" fn hisi_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int hisi_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct hisi_trng *trng;
    let mut currsize: c_int = 0;
    let mut val: u32 = 0;
    int ret;
    trng = container_of(rng, struct hisi_trng, rng);
    do {
    ret = readl_poll_timeout(trng.base + HISI_TRNG_REG, val,
    val, SLEEP_US, TIMEOUT_US);
    if (ret)
    return currsize;
    if (max - currsize >= HISI_TRNG_BYTES) {
    memcpy(buf + currsize, &val, HISI_TRNG_BYTES);
    currsize += HISI_TRNG_BYTES;
    if (currsize == max)
    return currsize;
    continue;
    }
// copy remaining bytes
    memcpy(buf + currsize, &val, max - currsize);
    currsize = max;
    } while (currsize < max);
    return currsize;
    }
#[no_mangle]
unsafe extern "C" fn hisi_trng_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_trng_probe(struct platform_device *pdev)
    {
    struct hisi_trng *trng;
    int ret;
    trng = devm_kzalloc(&pdev.dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.base))
    return PTR_ERR(trng.base);
    trng.rng.name = pdev.name;
    trng.rng.read = hisi_trng_read;
    trng.rng.quality = HISI_TRNG_QUALITY;
    ret = devm_hwrng_register(&pdev.dev, &trng.rng);
    if (ret)
    dev_err(&pdev.dev, "failed to register hwrng: %d!\n", ret);
    return ret;
    }
    static const struct acpi_device_id hisi_trng_acpi_match[] = {
    { .id = "HISI02B3" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hisi_trng_acpi_match);
    static struct platform_driver hisi_trng_driver = {
    .probe		= hisi_trng_probe,
    .driver		= {
    .name	= "hisi-trng-v2",
    .acpi_match_table = ACPI_PTR(hisi_trng_acpi_match),
    },
    };
    module_platform_driver(hisi_trng_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Weili Qian <qianweili@huawei.com>");
    MODULE_AUTHOR("Zaibo Xu <xuzaibo@huawei.com>");
    MODULE_DESCRIPTION("HiSilicon true random number generator V2 driver");
