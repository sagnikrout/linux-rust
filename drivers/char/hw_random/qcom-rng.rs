//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/qcom-rng.c
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
// Copyright (c) 2017-18 Linaro Limited
//
// Based on msm-rng.c and downstream driver

// Device specific register offsets
pub const PRNG_DATA_OUT: c_uint = 0x0000;
pub const PRNG_STATUS: c_uint = 0x0004;
pub const PRNG_LFSR_CFG: c_uint = 0x0100;
pub const PRNG_CONFIG: c_uint = 0x0104;
// Device specific register masks and config values
pub const PRNG_LFSR_CFG_MASK: c_uint = 0x0000ffff;
pub const PRNG_LFSR_CFG_CLOCKS: c_uint = 0x0000dddd;

pub const WORD_SZ: c_int = 4;
pub const QCOM_TRNG_QUALITY: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rng {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub hwrng: hwrng,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rng_match_data {
    pub hwrng_support: bool,
}

#[no_mangle]
unsafe extern "C" fn qcom_rng_read(rng: *mut qcom_rng, data: *mut u8, max: c_uint) -> c_int {
    static int qcom_rng_read(struct qcom_rng *rng, u8 *data, unsigned int max)
    {
    let mut currsize: c_uint = 0;
    u32 val;
    int ret;
// read random data from hardware
    do {
    ret = readl_poll_timeout(rng.base + PRNG_STATUS, val,
    val & PRNG_STATUS_DATA_AVAIL,
    200, 10000);
    if (ret)
    return ret;
    val = readl_relaxed(rng.base + PRNG_DATA_OUT);
    if ((max - currsize) >= WORD_SZ) {
    memcpy(data, &val, WORD_SZ);
    data += WORD_SZ;
    currsize += WORD_SZ;
    } else {
// copy only remaining bytes
    memcpy(data, &val, max - currsize);
    currsize = max;
    }
    } while (currsize < max);
    return currsize;
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwrng_init(hwrng: *mut hwrng) -> c_int {
    static int qcom_hwrng_init(struct hwrng *hwrng)
    {
    struct qcom_rng *qrng = container_of(hwrng, struct qcom_rng, hwrng);
    return clk_prepare_enable(qrng.clk);
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwrng_read(hwrng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int qcom_hwrng_read(struct hwrng *hwrng, void *data, size_t max, bool wait)
    {
    struct qcom_rng *qrng = container_of(hwrng, struct qcom_rng, hwrng);
    return qcom_rng_read(qrng, data, max);
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwrng_cleanup(hwrng: *mut hwrng) {
    static void qcom_hwrng_cleanup(struct hwrng *hwrng)
    {
    struct qcom_rng *qrng = container_of(hwrng, struct qcom_rng, hwrng);
    clk_disable_unprepare(qrng.clk);
    }
#[no_mangle]
unsafe extern "C" fn qcom_rng_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_rng_probe(struct platform_device *pdev)
    {
    const struct qcom_rng_match_data *match_data;
    struct qcom_rng *rng;
    int ret;
    match_data = device_get_match_data(&pdev.dev);
    if (match_data == core::ptr::null_mut() || !match_data.hwrng_support) {
    dev_info(&pdev.dev, "TRNG support not detected\n");
//
// In this case the driver does nothing except the dev_info(),
// but bind the device anyway to avoid effects on GCC state.
//
    return 0;
    }
    rng = devm_kzalloc(&pdev.dev, sizeof(*rng), GFP_KERNEL);
    if (!rng)
    return -ENOMEM;
    rng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rng.base))
    return PTR_ERR(rng.base);
    rng.clk = devm_clk_get_optional(&pdev.dev, "core");
    if (IS_ERR(rng.clk))
    return PTR_ERR(rng.clk);
    rng.hwrng.name = "qcom_hwrng";
    rng.hwrng.init = qcom_hwrng_init;
    rng.hwrng.read = qcom_hwrng_read;
    rng.hwrng.cleanup = qcom_hwrng_cleanup;
    rng.hwrng.quality = QCOM_TRNG_QUALITY;
    ret = devm_hwrng_register(&pdev.dev, &rng.hwrng);
    if (ret)
    dev_err(&pdev.dev, "Register hwrng failed: %d\n", ret);
    return ret;
    }
    static struct qcom_rng_match_data qcom_prng_match_data = {
    .hwrng_support = false,
    };
    static struct qcom_rng_match_data qcom_prng_ee_match_data = {
    .hwrng_support = false,
    };
    static struct qcom_rng_match_data qcom_trng_match_data = {
    .hwrng_support = true,
    };
    static const struct acpi_device_id __maybe_unused qcom_rng_acpi_match[] = {
    { .id = "QCOM8160", .driver_data = (kernel_ulong_t)&qcom_prng_ee_match_data },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, qcom_rng_acpi_match);
    static const struct of_device_id __maybe_unused qcom_rng_of_match[] = {
    { .compatible = "qcom,prng", .data = &qcom_prng_match_data },
    { .compatible = "qcom,prng-ee", .data = &qcom_prng_ee_match_data },
    { .compatible = "qcom,trng", .data = &qcom_trng_match_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_rng_of_match);
    static struct platform_driver qcom_rng_driver = {
    .probe = qcom_rng_probe,
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = qcom_rng_of_match,
    .acpi_match_table = ACPI_PTR(qcom_rng_acpi_match),
    }
    };
    module_platform_driver(qcom_rng_driver);
    MODULE_ALIAS("platform:" KBUILD_MODNAME);
    MODULE_DESCRIPTION("Qualcomm random number generator driver");
    MODULE_LICENSE("GPL v2");
