//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/xgene-rng.c
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
// APM X-Gene SoC RNG Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Author: Rameshwar Prasad Sahu <rsahu@apm.com>
// Shamal Winchurkar <swinchurkar@apm.com>
// Feng Kan <fkan@apm.com>
//

pub const RNG_MAX_DATUM: c_int = 4;
pub const MAX_TRY: c_int = 100;
pub const XGENE_RNG_RETRY_COUNT: c_int = 20;
pub const XGENE_RNG_RETRY_INTERVAL: c_int = 10;
// RNG  Registers
pub const RNG_INOUT_0: c_uint = 0x00;
pub const RNG_INTR_STS_ACK: c_uint = 0x10;
pub const RNG_CONTROL: c_uint = 0x14;
pub const RNG_CONFIG: c_uint = 0x18;
pub const RNG_ALARMCNT: c_uint = 0x1c;
pub const RNG_FROENABLE: c_uint = 0x20;
pub const RNG_FRODETUNE: c_uint = 0x24;
pub const RNG_ALARMMASK: c_uint = 0x28;
pub const RNG_ALARMSTOP: c_uint = 0x2c;
pub const RNG_OPTIONS: c_uint = 0x78;
pub const RNG_EIP_REV: c_uint = 0x7c;

    ((dst & ~0xffff0000) | (((u32)src << 16) & 0xffff0000))

    ((dst & ~0x000000ff) | (((u32)src) & 0x000000ff))

    ((dst & ~0x000000ff) | (((u32)src) & 0x000000ff))

    ((dst & ~BIT(10)) | (((u32)src << 10) & BIT(10)))

    ((dst & ~BIT(8)) | (((u32)src << 8) & BIT(8)))

    ((dst & ~BIT(7)) | (((u32)src << 7) & BIT(7)))

    ((dst & ~BIT(6)) | (((u32)src << 6) & BIT(6)))

    ((dst & ~BIT(5)) | (((u32)src << 5) & BIT(5)))

    ((dst & ~BIT(4)) | (((u32)src << 4) & BIT(4)))

    ((dst & ~BIT(3)) | (((u32)src << 3) & BIT(3)))

    ((dst & ~BIT(2)) | (((u32)src << 2) & BIT(2)))

    ((dst & ~BIT(1)) | (((u32)src << 1) & BIT(1)))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_rng_dev {
    pub irq: u32,
    pub csr_base: *mut void __iomem,
    pub revision: u32,
    pub datum_size: u32,
    pub /: *mut *mut u32 failure_cnt; / Failure count last minute,
    pub /: *mut *mut unsigned long failure_ts;/ First failure timestamp,
    pub failure_timer: timer_list,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn xgene_rng_expired_timer(t: *mut timer_list) {
    static void xgene_rng_expired_timer(struct timer_list *t)
    {
    struct xgene_rng_dev *ctx = timer_container_of(ctx, t, failure_timer);
// Clear failure counter as timer expired
    disable_irq(ctx.irq);
    ctx.failure_cnt = 0;
    timer_delete(&ctx.failure_timer);
    enable_irq(ctx.irq);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_start_timer(ctx: *mut xgene_rng_dev) {
    static void xgene_rng_start_timer(struct xgene_rng_dev *ctx)
    {
    ctx.failure_timer.expires = jiffies + 120 * HZ;
    add_timer(&ctx.failure_timer);
    }
//
// Initialize or reinit free running oscillators (FROs)
//
#[no_mangle]
unsafe extern "C" fn xgene_rng_init_fro(ctx: *mut xgene_rng_dev, fro_val: u32) {
    static void xgene_rng_init_fro(struct xgene_rng_dev *ctx, u32 fro_val)
    {
    writel(fro_val, ctx.csr_base + RNG_FRODETUNE);
    writel(0x00000000, ctx.csr_base + RNG_ALARMMASK);
    writel(0x00000000, ctx.csr_base + RNG_ALARMSTOP);
    writel(0xFFFFFFFF, ctx.csr_base + RNG_FROENABLE);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_chk_overflow(ctx: *mut xgene_rng_dev) {
    static void xgene_rng_chk_overflow(struct xgene_rng_dev *ctx)
    {
    u32 val;
    val = readl(ctx.csr_base + RNG_INTR_STS_ACK);
    if (val & MONOBIT_FAIL_MASK)
//
// LFSR detected an out-of-bounds number of 1s after
// checking 20,000 bits (test T1 as specified in the
// AIS-31 standard)
//
    dev_err(ctx.dev, "test monobit failure error 0x%08X\n", val);
    if (val & POKER_FAIL_MASK)
//
// LFSR detected an out-of-bounds value in at least one
// of the 16 poker_count_X counters or an out of bounds sum
// of squares value after checking 20,000 bits (test T2 as
// specified in the AIS-31 standard)
//
    dev_err(ctx.dev, "test poker failure error 0x%08X\n", val);
    if (val & LONG_RUN_FAIL_MASK)
//
// LFSR detected a sequence of 34 identical bits
// (test T4 as specified in the AIS-31 standard)
//
    dev_err(ctx.dev, "test long run failure error 0x%08X\n", val);
    if (val & RUN_FAIL_MASK)
//
// LFSR detected an outof-bounds value for at least one
// of the running counters after checking 20,000 bits
// (test T3 as specified in the AIS-31 standard)
//
    dev_err(ctx.dev, "test run failure error 0x%08X\n", val);
    if (val & NOISE_FAIL_MASK)
// LFSR detected a sequence of 48 identical bits
    dev_err(ctx.dev, "noise failure error 0x%08X\n", val);
    if (val & STUCK_OUT_MASK)
//
// Detected output data registers generated same value twice
// in a row
//
    dev_err(ctx.dev, "stuck out failure error 0x%08X\n", val);
    if (val & SHUTDOWN_OFLO_MASK) {
    u32 frostopped;
// FROs shut down after a second error event. Try recover.
    if (++ctx.failure_cnt == 1) {
// 1st time, just recover
    ctx.failure_ts = jiffies;
    frostopped = readl(ctx.csr_base + RNG_ALARMSTOP);
    xgene_rng_init_fro(ctx, frostopped);
//
// We must start a timer to clear out this error
// in case the system timer wrap around
//
    xgene_rng_start_timer(ctx);
    } else {
// 2nd time failure in lesser than 1 minute?
    if (time_after(ctx.failure_ts + 60 * HZ, jiffies)) {
    dev_err(ctx.dev,
    "FRO shutdown failure error 0x%08X\n",
    val);
    } else {
// 2nd time failure after 1 minutes, recover
    ctx.failure_ts = jiffies;
    ctx.failure_cnt = 1;
//
// We must start a timer to clear out this
// error in case the system timer wrap
// around
//
    xgene_rng_start_timer(ctx);
    }
    frostopped = readl(ctx.csr_base + RNG_ALARMSTOP);
    xgene_rng_init_fro(ctx, frostopped);
    }
    }
// Clear them all
    writel(val, ctx.csr_base + RNG_INTR_STS_ACK);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_irq_handler(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xgene_rng_irq_handler(int irq, void *id)
    {
    struct xgene_rng_dev *ctx = id;
// RNG Alarm Counter overflow
    xgene_rng_chk_overflow(ctx);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int xgene_rng_data_present(struct hwrng *rng, int wait)
    {
    struct xgene_rng_dev *ctx = (struct xgene_rng_dev *) rng.priv;
    u32 i, val = 0;
    for (i = 0; i < XGENE_RNG_RETRY_COUNT; i++) {
    val = readl(ctx.csr_base + RNG_INTR_STS_ACK);
    if ((val & READY_MASK) || !wait)
    break;
    udelay(XGENE_RNG_RETRY_INTERVAL);
    }
    return (val & READY_MASK);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int xgene_rng_data_read(struct hwrng *rng, u32 *data)
    {
    struct xgene_rng_dev *ctx = (struct xgene_rng_dev *) rng.priv;
    int i;
    for (i = 0; i < ctx.datum_size; i++)
    data[i] = readl(ctx.csr_base + RNG_INOUT_0 + i * 4);
// Clear ready bit to start next transaction
    writel(READY_MASK, ctx.csr_base + RNG_INTR_STS_ACK);
    return ctx.datum_size << 2;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_init_internal(ctx: *mut xgene_rng_dev) {
    static void xgene_rng_init_internal(struct xgene_rng_dev *ctx)
    {
    u32 val;
    writel(0x00000000, ctx.csr_base + RNG_CONTROL);
    val = MAX_REFILL_CYCLES_SET(0, 10);
    val = MIN_REFILL_CYCLES_SET(val, 10);
    writel(val, ctx.csr_base + RNG_CONFIG);
    val = ALARM_THRESHOLD_SET(0, 0xFF);
    writel(val, ctx.csr_base + RNG_ALARMCNT);
    xgene_rng_init_fro(ctx, 0);
    writel(MONOBIT_FAIL_MASK |
    POKER_FAIL_MASK	|
    LONG_RUN_FAIL_MASK |
    RUN_FAIL_MASK |
    NOISE_FAIL_MASK |
    STUCK_OUT_MASK |
    SHUTDOWN_OFLO_MASK |
    READY_MASK, ctx.csr_base + RNG_INTR_STS_ACK);
    val = ENABLE_RNG_SET(0, 1);
    val = MONOBIT_FAIL_MASK_SET(val, 1);
    val = POKER_FAIL_MASK_SET(val, 1);
    val = LONG_RUN_FAIL_MASK_SET(val, 1);
    val = RUN_FAIL_MASK_SET(val, 1);
    val = NOISE_FAIL_MASK_SET(val, 1);
    val = STUCK_OUT_MASK_SET(val, 1);
    val = SHUTDOWN_OFLO_MASK_SET(val, 1);
    writel(val, ctx.csr_base + RNG_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_init(rng: *mut hwrng) -> c_int {
    static int xgene_rng_init(struct hwrng *rng)
    {
    struct xgene_rng_dev *ctx = (struct xgene_rng_dev *) rng.priv;
    ctx.failure_cnt = 0;
    timer_setup(&ctx.failure_timer, xgene_rng_expired_timer, 0);
    ctx.revision = readl(ctx.csr_base + RNG_EIP_REV);
    dev_dbg(ctx.dev, "Rev %d.%d.%d\n",
    MAJOR_HW_REV_RD(ctx.revision),
    MINOR_HW_REV_RD(ctx.revision),
    HW_PATCH_LEVEL_RD(ctx.revision));
    dev_dbg(ctx.dev, "Options 0x%08X",
    readl(ctx.csr_base + RNG_OPTIONS));
    xgene_rng_init_internal(ctx);
    ctx.datum_size = RNG_MAX_DATUM;
    return 0;
    }

    static const struct acpi_device_id xgene_rng_acpi_match[] = {
    { .id = "APMC0D18" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, xgene_rng_acpi_match);

    static struct hwrng xgene_rng_func = {
    .name		= "xgene-rng",
    .init		= xgene_rng_init,
    .data_present	= xgene_rng_data_present,
    .data_read	= xgene_rng_data_read,
    };
#[no_mangle]
unsafe extern "C" fn xgene_rng_probe(pdev: *mut platform_device) -> c_int {
    static int xgene_rng_probe(struct platform_device *pdev)
    {
    struct xgene_rng_dev *ctx;
    struct clk *clk;
    let mut rc: c_int = 0;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.dev = &pdev.dev;
    ctx.csr_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.csr_base))
    return PTR_ERR(ctx.csr_base);
    rc = platform_get_irq(pdev, 0);
    if (rc < 0)
    return rc;
    ctx.irq = rc;
    dev_dbg(&pdev.dev, "APM X-Gene RNG BASE %p ALARM IRQ %d",
    ctx.csr_base, ctx.irq);
    rc = devm_request_irq(&pdev.dev, ctx.irq, xgene_rng_irq_handler, 0,
    dev_name(&pdev.dev), ctx);
    if (rc)
    return rc;
// Enable IP clock
    clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk), "Couldn't get the clock for RNG\n");
    xgene_rng_func.priv = (unsigned long) ctx;
    rc = devm_hwrng_register(&pdev.dev, &xgene_rng_func);
    if (rc)
    return dev_err_probe(&pdev.dev, rc, "RNG registering failed\n");
    rc = device_init_wakeup(&pdev.dev, 1);
    if (rc)
    return dev_err_probe(&pdev.dev, rc, "RNG device_init_wakeup failed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgene_rng_remove(pdev: *mut platform_device) {
    static void xgene_rng_remove(struct platform_device *pdev)
    {
    int rc;
    rc = device_init_wakeup(&pdev.dev, 0);
    if (rc)
    dev_err(&pdev.dev, "RNG init wakeup failed error %d\n", rc);
    }
    static const struct of_device_id xgene_rng_of_match[] = {
    { .compatible = "apm,xgene-rng" },
    { }
    };
    MODULE_DEVICE_TABLE(of, xgene_rng_of_match);
    static struct platform_driver xgene_rng_driver = {
    .probe = xgene_rng_probe,
    .remove = xgene_rng_remove,
    .driver = {
    .name		= "xgene-rng",
    .of_match_table = xgene_rng_of_match,
    .acpi_match_table = ACPI_PTR(xgene_rng_acpi_match),
    },
    };
    module_platform_driver(xgene_rng_driver);
    MODULE_DESCRIPTION("APM X-Gene RNG driver");
    MODULE_LICENSE("GPL");
