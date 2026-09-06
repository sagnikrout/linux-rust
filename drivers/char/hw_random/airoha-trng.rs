//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/airoha-trng.c
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
// Copyright (C) 2024 Christian Marangi

pub const TRNG_IP_RDY: c_uint = 0x800;

pub const TRNG_NS_SEK_AND_DAT_EN: c_uint = 0x804;

pub const TRNG_HEALTH_TEST_SW_RST: c_uint = 0x808;

pub const TRNG_INTR_EN: c_uint = 0x818;

// Notice that Health Test are done only out of Reset and with RNG_EN
pub const TRNG_HEALTH_TEST_STATUS: c_uint = 0x824;

pub const TRNG_RAW_DATA_OUT: c_uint = 0x828;
pub const TRNG_CNT_TRANS_VALID: c_uint = 0x80;
pub const BUSY_LOOP_SLEEP: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_trng {
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub dev: *mut device,
    pub rng_op_done: completion,
}

#[no_mangle]
unsafe extern "C" fn airoha_trng_irq_mask(trng: *mut airoha_trng) -> c_int {
    static int airoha_trng_irq_mask(struct airoha_trng *trng)
    {
    u32 val;
    val = readl(trng.base + TRNG_INTR_EN);
    val |= INTR_MASK;
    writel(val, trng.base + TRNG_INTR_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_irq_unmask(trng: *mut airoha_trng) -> c_int {
    static int airoha_trng_irq_unmask(struct airoha_trng *trng)
    {
    u32 val;
    val = readl(trng.base + TRNG_INTR_EN);
    val &= ~INTR_MASK;
    writel(val, trng.base + TRNG_INTR_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_init(rng: *mut hwrng) -> c_int {
    static int airoha_trng_init(struct hwrng *rng)
    {
    struct airoha_trng *trng = container_of(rng, struct airoha_trng, rng);
    int ret;
    u32 val;
    val = readl(trng.base + TRNG_NS_SEK_AND_DAT_EN);
    val |= RNG_EN;
    writel(val, trng.base + TRNG_NS_SEK_AND_DAT_EN);
// Set out of SW Reset
    airoha_trng_irq_unmask(trng);
    writel(0, trng.base + TRNG_HEALTH_TEST_SW_RST);
    ret = wait_for_completion_timeout(&trng.rng_op_done, BUSY_LOOP_TIMEOUT);
    if (ret <= 0) {
    dev_err(trng.dev, "Timeout waiting for Health Check\n");
    airoha_trng_irq_mask(trng);
    return -ENODEV;
    }
// Check if Health Test Failed
    val = readl(trng.base + TRNG_HEALTH_TEST_STATUS);
    if (val & (RST_STARTUP_AP_TEST_FAIL | RST_STARTUP_RC_TEST_FAIL)) {
    dev_err(trng.dev, "Health Check fail: %s test fail\n",
    val & RST_STARTUP_AP_TEST_FAIL ? "AP" : "RC");
    return -ENODEV;
    }
// Check if IP is ready
    ret = readl_poll_timeout(trng.base + TRNG_IP_RDY, val,
    val & SAMPLE_RDY, 10, 1000);
    if (ret < 0) {
    dev_err(trng.dev, "Timeout waiting for IP ready");
    return -ENODEV;
    }
// CNT_TRANS must be 0x80 for IP to be considered ready
    ret = readl_poll_timeout(trng.base + TRNG_IP_RDY, val,
    FIELD_GET(CNT_TRANS, val) == TRNG_CNT_TRANS_VALID,
    10, 1000);
    if (ret < 0) {
    dev_err(trng.dev, "Timeout waiting for IP ready");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_cleanup(rng: *mut hwrng) {
    static void airoha_trng_cleanup(struct hwrng *rng)
    {
    struct airoha_trng *trng = container_of(rng, struct airoha_trng, rng);
    u32 val;
    val = readl(trng.base + TRNG_NS_SEK_AND_DAT_EN);
    val &= ~RNG_EN;
    writel(val, trng.base + TRNG_NS_SEK_AND_DAT_EN);
// Put it in SW Reset
    writel(SW_RST, trng.base + TRNG_HEALTH_TEST_SW_RST);
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int airoha_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct airoha_trng *trng = container_of(rng, struct airoha_trng, rng);
    u32 *data = buf;
    u32 status;
    int ret;
    ret = readl_poll_timeout(trng.base + TRNG_HEALTH_TEST_STATUS, status,
    status & RAW_DATA_VALID, 10, 1000);
    if (ret < 0) {
    dev_err(trng.dev, "Timeout waiting for TRNG RAW Data valid\n");
    return ret;
    }
// data = readl(trng->base + TRNG_RAW_DATA_OUT);
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_irq(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t airoha_trng_irq(int irq, void *priv)
    {
    struct airoha_trng *trng = (struct airoha_trng *)priv;
    airoha_trng_irq_mask(trng);
// Just complete the task, we will read the value later
    complete(&trng.rng_op_done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn airoha_trng_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_trng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct airoha_trng *trng;
    int irq, ret;
    u32 val;
    trng = devm_kzalloc(dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    trng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.base))
    return PTR_ERR(trng.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    airoha_trng_irq_mask(trng);
    ret = devm_request_irq(&pdev.dev, irq, airoha_trng_irq, 0,
    pdev.name, (void *)trng);
    if (ret)
    return ret;
    init_completion(&trng.rng_op_done);
// Enable interrupt for SW reset Health Check
    val = readl(trng.base + TRNG_INTR_EN);
    val |= RST_STARTUP_INITR_EN;
    writel(val, trng.base + TRNG_INTR_EN);
// Set output to raw data
    val = readl(trng.base + TRNG_NS_SEK_AND_DAT_EN);
    val |= RAW_DATA_EN;
    writel(val, trng.base + TRNG_NS_SEK_AND_DAT_EN);
// Put it in SW Reset
    writel(SW_RST, trng.base + TRNG_HEALTH_TEST_SW_RST);
    trng.dev = dev;
    trng.rng.name = pdev.name;
    trng.rng.init = airoha_trng_init;
    trng.rng.cleanup = airoha_trng_cleanup;
    trng.rng.read = airoha_trng_read;
    trng.rng.quality = 900;
    ret = devm_hwrng_register(dev, &trng.rng);
    if (ret) {
    dev_err(dev, "failed to register rng device: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id airoha_trng_of_match[] = {
    { .compatible = "airoha,en7581-trng", },
    {},
    };
    MODULE_DEVICE_TABLE(of, airoha_trng_of_match);
    static struct platform_driver airoha_trng_driver = {
    .driver = {
    .name = "airoha-trng",
    .of_match_table	= airoha_trng_of_match,
    },
    .probe = airoha_trng_probe,
    };
    module_platform_driver(airoha_trng_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("Airoha True Random Number Generator driver");
