//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/jh7110-trng.c
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
//
// TRNG driver for the StarFive JH7110 SoC
//
// Copyright (C) 2022 StarFive Technology Co.
//

// trng register offset
pub const STARFIVE_CTRL: c_uint = 0x00;
pub const STARFIVE_STAT: c_uint = 0x04;
pub const STARFIVE_MODE: c_uint = 0x08;
pub const STARFIVE_SMODE: c_uint = 0x0C;
pub const STARFIVE_IE: c_uint = 0x10;
pub const STARFIVE_ISTAT: c_uint = 0x14;
pub const STARFIVE_RAND0: c_uint = 0x20;
pub const STARFIVE_RAND1: c_uint = 0x24;
pub const STARFIVE_RAND2: c_uint = 0x28;
pub const STARFIVE_RAND3: c_uint = 0x2C;
pub const STARFIVE_RAND4: c_uint = 0x30;
pub const STARFIVE_RAND5: c_uint = 0x34;
pub const STARFIVE_RAND6: c_uint = 0x38;
pub const STARFIVE_RAND7: c_uint = 0x3C;
pub const STARFIVE_AUTO_RQSTS: c_uint = 0x60;
pub const STARFIVE_AUTO_AGE: c_uint = 0x64;
// CTRL CMD
pub const STARFIVE_CTRL_EXEC_NOP: c_uint = 0x0;
pub const STARFIVE_CTRL_GENE_RANDNUM: c_uint = 0x1;
pub const STARFIVE_CTRL_EXEC_RANDRESEED: c_uint = 0x2;
// STAT

// MODE

// SMODE

// IE

    STARFIVE_IE_RAND_RDY_EN | \
    STARFIVE_IE_SEED_DONE_EN | \
    STARFIVE_IE_LFSR_LOCKUP_EN)
// ISTAT

    enum reseed {
    RANDOM_RESEED,
    NONCE_RESEED,
    };
    enum mode {
    PRNG_128BIT,
    PRNG_256BIT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct starfive_trng {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub hclk: *mut clk,
    pub ahb: *mut clk,
    pub rst: *mut reset_control,
    pub rng: hwrng,
    pub random_done: completion,
    pub reseed_done: completion,
    pub mode: u32,
    pub mission: u32,
    pub reseed: u32,
// protects against concurrent write to ctrl register
    pub write_lock: spinlock_t,
}

    static u16 autoreq;
    module_param(autoreq, ushort, 0);
    MODULE_PARM_DESC(autoreq, "Auto-reseeding after random number requests by host reaches specified counter:\n"
    " 0 - disable counter\n"
    " other - reload value for internal counter");
    static u16 autoage;
    module_param(autoage, ushort, 0);
    MODULE_PARM_DESC(autoage, "Auto-reseeding after specified timer countdowns to 0:\n"
    " 0 - disable timer\n"
    " other - reload value for internal timer");
#[no_mangle]
pub unsafe extern "C" fn starfive_trng_wait_idle(trng: *mut starfive_trng) -> c_int {
    static inline int starfive_trng_wait_idle(struct starfive_trng *trng)
    {
    u32 stat;
    return readl_relaxed_poll_timeout(trng.base + STARFIVE_STAT, stat,
    !(stat & (STARFIVE_STAT_RAND_GENERATING |
    STARFIVE_STAT_RAND_SEEDING)),
    10, 100000);
    }
#[no_mangle]
pub unsafe extern "C" fn starfive_trng_irq_mask_clear(trng: *mut starfive_trng) {
    static inline void starfive_trng_irq_mask_clear(struct starfive_trng *trng)
    {
// clear register: ISTAT
    let mut data: u32 = readl(trng.base + STARFIVE_ISTAT);
    writel(data, trng.base + STARFIVE_ISTAT);
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_cmd(trng: *mut starfive_trng, cmd: u32, wait: bool) -> c_int {
    static int starfive_trng_cmd(struct starfive_trng *trng, u32 cmd, bool wait)
    {
    let mut wait_time: c_int = 1000;
// allow up to 40 us for wait == 0
    if (!wait)
    wait_time = 40;
    switch (cmd) {
    case STARFIVE_CTRL_GENE_RANDNUM:
    reinit_completion(&trng.random_done);
    spin_lock_irq(&trng.write_lock);
    writel(cmd, trng.base + STARFIVE_CTRL);
    spin_unlock_irq(&trng.write_lock);
    if (!wait_for_completion_timeout(&trng.random_done, usecs_to_jiffies(wait_time)))
    return -ETIMEDOUT;
    break;
    case STARFIVE_CTRL_EXEC_RANDRESEED:
    reinit_completion(&trng.reseed_done);
    spin_lock_irq(&trng.write_lock);
    writel(cmd, trng.base + STARFIVE_CTRL);
    spin_unlock_irq(&trng.write_lock);
    if (!wait_for_completion_timeout(&trng.reseed_done, usecs_to_jiffies(wait_time)))
    return -ETIMEDOUT;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_init(rng: *mut hwrng) -> c_int {
    static int starfive_trng_init(struct hwrng *rng)
    {
    struct starfive_trng *trng = to_trng(rng);
    u32 mode, intr = 0;
// setup Auto Request/Age register
    writel(autoage, trng.base + STARFIVE_AUTO_AGE);
    writel(autoreq, trng.base + STARFIVE_AUTO_RQSTS);
// clear register: ISTAT
    starfive_trng_irq_mask_clear(trng);
    intr |= STARFIVE_IE_ALL;
    writel(intr, trng.base + STARFIVE_IE);
    mode  = readl(trng.base + STARFIVE_MODE);
    switch (trng.mode) {
    case PRNG_128BIT:
    mode &= ~STARFIVE_MODE_R256;
    break;
    case PRNG_256BIT:
    mode |= STARFIVE_MODE_R256;
    break;
    default:
    mode |= STARFIVE_MODE_R256;
    break;
    }
    writel(mode, trng.base + STARFIVE_MODE);
    return starfive_trng_cmd(trng, STARFIVE_CTRL_EXEC_RANDRESEED, 1);
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_irq(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t starfive_trng_irq(int irq, void *priv)
    {
    u32 status;
    struct starfive_trng *trng = (struct starfive_trng *)priv;
    status = readl(trng.base + STARFIVE_ISTAT);
    if (status & STARFIVE_ISTAT_RAND_RDY) {
    writel(STARFIVE_ISTAT_RAND_RDY, trng.base + STARFIVE_ISTAT);
    complete(&trng.random_done);
    }
    if (status & STARFIVE_ISTAT_SEED_DONE) {
    writel(STARFIVE_ISTAT_SEED_DONE, trng.base + STARFIVE_ISTAT);
    complete(&trng.reseed_done);
    }
    if (status & STARFIVE_ISTAT_LFSR_LOCKUP) {
    writel(STARFIVE_ISTAT_LFSR_LOCKUP, trng.base + STARFIVE_ISTAT);
// SEU occurred, reseeding required
    spin_lock(&trng.write_lock);
    writel(STARFIVE_CTRL_EXEC_RANDRESEED, trng.base + STARFIVE_CTRL);
    spin_unlock(&trng.write_lock);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_cleanup(rng: *mut hwrng) {
    static void starfive_trng_cleanup(struct hwrng *rng)
    {
    struct starfive_trng *trng = to_trng(rng);
    writel(0, trng.base + STARFIVE_CTRL);
    reset_control_assert(trng.rst);
    clk_disable_unprepare(trng.hclk);
    clk_disable_unprepare(trng.ahb);
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int starfive_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct starfive_trng *trng = to_trng(rng);
    int ret;
    pm_runtime_get_sync(trng.dev);
    if (trng.mode == PRNG_256BIT)
    max = min_t(size_t, max, (STARFIVE_RAND_LEN * 8));
    else
    max = min_t(size_t, max, (STARFIVE_RAND_LEN * 4));
    if (wait) {
    ret = starfive_trng_wait_idle(trng);
    if (ret) {
    ret = -ETIMEDOUT;
    goto out_put;
    }
    }
    ret = starfive_trng_cmd(trng, STARFIVE_CTRL_GENE_RANDNUM, wait);
    if (ret)
    goto out_put;
    memcpy_fromio(buf, trng.base + STARFIVE_RAND0, max);
    ret = max;
    out_put:
    pm_runtime_put_sync_autosuspend(trng.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_probe(pdev: *mut platform_device) -> c_int {
    static int starfive_trng_probe(struct platform_device *pdev)
    {
    int ret;
    int irq;
    struct starfive_trng *trng;
    trng = devm_kzalloc(&pdev.dev, sizeof(*trng), GFP_KERNEL);
    if (!trng)
    return -ENOMEM;
    platform_set_drvdata(pdev, trng);
    trng.dev = &pdev.dev;
    trng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(trng.base))
    return dev_err_probe(&pdev.dev, PTR_ERR(trng.base),
    "Error remapping memory for platform device.\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    init_completion(&trng.random_done);
    init_completion(&trng.reseed_done);
    spin_lock_init(&trng.write_lock);
    ret = devm_request_irq(&pdev.dev, irq, starfive_trng_irq, 0, pdev.name,
    (void *)trng);
    if (ret)
    return ret;
    trng.hclk = devm_clk_get(&pdev.dev, "hclk");
    if (IS_ERR(trng.hclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(trng.hclk),
    "Error getting hardware reference clock\n");
    trng.ahb = devm_clk_get(&pdev.dev, "ahb");
    if (IS_ERR(trng.ahb))
    return dev_err_probe(&pdev.dev, PTR_ERR(trng.ahb),
    "Error getting ahb reference clock\n");
    trng.rst = devm_reset_control_get_shared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(trng.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(trng.rst),
    "Error getting hardware reset line\n");
    clk_prepare_enable(trng.hclk);
    clk_prepare_enable(trng.ahb);
    reset_control_deassert(trng.rst);
    trng.rng.name = dev_driver_string(&pdev.dev);
    trng.rng.init = starfive_trng_init;
    trng.rng.cleanup = starfive_trng_cleanup;
    trng.rng.read = starfive_trng_read;
    trng.mode = PRNG_256BIT;
    trng.mission = 1;
    trng.reseed = RANDOM_RESEED;
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 100);
    pm_runtime_enable(&pdev.dev);
    ret = devm_hwrng_register(&pdev.dev, &trng.rng);
    if (ret) {
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(trng.rst);
    clk_disable_unprepare(trng.ahb);
    clk_disable_unprepare(trng.hclk);
    return dev_err_probe(&pdev.dev, ret, "Failed to register hwrng\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused starfive_trng_suspend(struct device *dev)
    {
    struct starfive_trng *trng = dev_get_drvdata(dev);
    clk_disable_unprepare(trng.hclk);
    clk_disable_unprepare(trng.ahb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn starfive_trng_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused starfive_trng_resume(struct device *dev)
    {
    struct starfive_trng *trng = dev_get_drvdata(dev);
    clk_prepare_enable(trng.hclk);
    clk_prepare_enable(trng.ahb);
    return 0;
    }
    static const struct dev_pm_ops starfive_trng_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(starfive_trng_suspend,
    starfive_trng_resume)
    SET_RUNTIME_PM_OPS(starfive_trng_suspend,
    starfive_trng_resume, core::ptr::null_mut())
    };
    static const struct of_device_id trng_dt_ids[] __maybe_unused = {
    { .compatible = "starfive,jh7110-trng" },
    { }
    };
    MODULE_DEVICE_TABLE(of, trng_dt_ids);
    static struct platform_driver starfive_trng_driver = {
    .probe	= starfive_trng_probe,
    .driver	= {
    .name		= "jh7110-trng",
    .pm		= &starfive_trng_pm_ops,
    .of_match_table	= of_match_ptr(trng_dt_ids),
    },
    };
    module_platform_driver(starfive_trng_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("StarFive True Random Number Generator");
