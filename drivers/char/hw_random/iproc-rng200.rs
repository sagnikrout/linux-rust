//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/iproc-rng200.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2015 Broadcom Corporation
//
// DESCRIPTION: The Broadcom iProc RNG200 Driver
//

// Registers
pub const RNG_CTRL_OFFSET: c_uint = 0x00;
pub const RNG_CTRL_RNG_RBGEN_MASK: c_uint = 0x00001FFF;
pub const RNG_CTRL_RNG_RBGEN_ENABLE: c_uint = 0x00000001;
pub const RNG_SOFT_RESET_OFFSET: c_uint = 0x04;
pub const RNG_SOFT_RESET: c_uint = 0x00000001;
pub const RBG_SOFT_RESET_OFFSET: c_uint = 0x08;
pub const RBG_SOFT_RESET: c_uint = 0x00000001;
pub const RNG_INT_STATUS_OFFSET: c_uint = 0x18;
pub const RNG_INT_STATUS_MASTER_FAIL_LOCKOUT_IRQ_MASK: c_uint = 0x80000000;
pub const RNG_INT_STATUS_STARTUP_TRANSITIONS_MET_IRQ_MASK: c_uint = 0x00020000;
pub const RNG_INT_STATUS_NIST_FAIL_IRQ_MASK: c_uint = 0x00000020;
pub const RNG_INT_STATUS_TOTAL_BITS_COUNT_IRQ_MASK: c_uint = 0x00000001;
pub const RNG_FIFO_DATA_OFFSET: c_uint = 0x20;
pub const RNG_FIFO_COUNT_OFFSET: c_uint = 0x24;
pub const RNG_FIFO_COUNT_RNG_FIFO_COUNT_MASK: c_uint = 0x000000FF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_rng200_dev {
    pub rng: hwrng,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn iproc_rng200_enable_set(rng_base: *mut void __iomem, enable: bool) {
    static void iproc_rng200_enable_set(void __iomem *rng_base, bool enable)
    {
    u32 val;
    val = ioread32(rng_base + RNG_CTRL_OFFSET);
    val &= ~RNG_CTRL_RNG_RBGEN_MASK;
    if (enable)
    val |= RNG_CTRL_RNG_RBGEN_ENABLE;
    iowrite32(val, rng_base + RNG_CTRL_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_restart(rng_base: *mut void __iomem) {
    static void iproc_rng200_restart(void __iomem *rng_base)
    {
    uint32_t val;
    iproc_rng200_enable_set(rng_base, false);
// Clear all interrupt status
    iowrite32(0xFFFFFFFFUL, rng_base + RNG_INT_STATUS_OFFSET);
// Reset RNG and RBG
    val = ioread32(rng_base + RBG_SOFT_RESET_OFFSET);
    val |= RBG_SOFT_RESET;
    iowrite32(val, rng_base + RBG_SOFT_RESET_OFFSET);
    val = ioread32(rng_base + RNG_SOFT_RESET_OFFSET);
    val |= RNG_SOFT_RESET;
    iowrite32(val, rng_base + RNG_SOFT_RESET_OFFSET);
    val = ioread32(rng_base + RNG_SOFT_RESET_OFFSET);
    val &= ~RNG_SOFT_RESET;
    iowrite32(val, rng_base + RNG_SOFT_RESET_OFFSET);
    val = ioread32(rng_base + RBG_SOFT_RESET_OFFSET);
    val &= ~RBG_SOFT_RESET;
    iowrite32(val, rng_base + RBG_SOFT_RESET_OFFSET);
    iproc_rng200_enable_set(rng_base, true);
    }
    static int iproc_rng200_read(struct hwrng *rng, void *buf, size_t max,
    bool wait)
    {
    struct iproc_rng200_dev *priv = to_rng_priv(rng);
    let mut num_remaining: u32 = max;
    uint32_t status;
pub const MAX_RESETS_PER_READ: c_int = 1;
    let mut num_resets: u32 = 0;

    let mut idle_endtime: c_ulong = jiffies + MAX_IDLE_TIME;
    while ((num_remaining > 0) && time_before(jiffies, idle_endtime)) {
// Is RNG sane? If not, reset it.
    status = ioread32(priv.base + RNG_INT_STATUS_OFFSET);
    if ((status & (RNG_INT_STATUS_MASTER_FAIL_LOCKOUT_IRQ_MASK |
    RNG_INT_STATUS_NIST_FAIL_IRQ_MASK)) != 0) {
    if (num_resets >= MAX_RESETS_PER_READ)
    return max - num_remaining;
    iproc_rng200_restart(priv.base);
    num_resets++;
    }
// Are there any random numbers available?
    if ((ioread32(priv.base + RNG_FIFO_COUNT_OFFSET) &
    RNG_FIFO_COUNT_RNG_FIFO_COUNT_MASK) > 0) {
    if (num_remaining >= sizeof(uint32_t)) {
// Buffer has room to store entire word
// (uint32_t *)buf = ioread32(priv->base +
    RNG_FIFO_DATA_OFFSET);
    buf += sizeof(uint32_t);
    num_remaining -= sizeof(uint32_t);
    } else {
// Buffer can only store partial word
    uint32_t rnd_number = ioread32(priv.base +
    RNG_FIFO_DATA_OFFSET);
    memcpy(buf, &rnd_number, num_remaining);
    buf += num_remaining;
    num_remaining = 0;
    }
// Reset the IDLE timeout
    idle_endtime = jiffies + MAX_IDLE_TIME;
    } else {
    if (!wait)
// Cannot wait, return immediately
    return max - num_remaining;
// Can wait, give others chance to run
    usleep_range(min(num_remaining * 10, 500U), 500);
    }
    }
    return max - num_remaining;
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_init(rng: *mut hwrng) -> c_int {
    static int iproc_rng200_init(struct hwrng *rng)
    {
    struct iproc_rng200_dev *priv = to_rng_priv(rng);
    iproc_rng200_enable_set(priv.base, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_cleanup(rng: *mut hwrng) {
    static void iproc_rng200_cleanup(struct hwrng *rng)
    {
    struct iproc_rng200_dev *priv = to_rng_priv(rng);
    iproc_rng200_enable_set(priv.base, false);
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_rng200_probe(struct platform_device *pdev)
    {
    struct iproc_rng200_dev *priv;
    struct device *dev = &pdev.dev;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Map peripheral
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    dev_err(dev, "failed to remap rng regs\n");
    return PTR_ERR(priv.base);
    }
    dev_set_drvdata(dev, priv);
    priv.rng.name = "iproc-rng200";
    priv.rng.read = iproc_rng200_read;
    priv.rng.init = iproc_rng200_init;
    priv.rng.cleanup = iproc_rng200_cleanup;
// Register driver
    ret = devm_hwrng_register(dev, &priv.rng);
    if (ret) {
    dev_err(dev, "hwrng registration failed\n");
    return ret;
    }
    dev_info(dev, "hwrng registered\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused iproc_rng200_suspend(struct device *dev)
    {
    struct iproc_rng200_dev *priv = dev_get_drvdata(dev);
    iproc_rng200_cleanup(&priv.rng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_rng200_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused iproc_rng200_resume(struct device *dev)
    {
    struct iproc_rng200_dev *priv =  dev_get_drvdata(dev);
    iproc_rng200_init(&priv.rng);
    return 0;
    }
    static const struct dev_pm_ops iproc_rng200_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(iproc_rng200_suspend, iproc_rng200_resume)
    };
    static const struct of_device_id iproc_rng200_of_match[] = {
    { .compatible = "brcm,bcm2711-rng200", },
    { .compatible = "brcm,bcm7211-rng200", },
    { .compatible = "brcm,bcm7278-rng200", },
    { .compatible = "brcm,iproc-rng200", },
    {},
    };
    MODULE_DEVICE_TABLE(of, iproc_rng200_of_match);
    static struct platform_driver iproc_rng200_driver = {
    .driver = {
    .name		= "iproc-rng200",
    .of_match_table = iproc_rng200_of_match,
    .pm		= &iproc_rng200_pm_ops,
    },
    .probe		= iproc_rng200_probe,
    };
    module_platform_driver(iproc_rng200_driver);
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("iProc RNG200 Random Number Generator driver");
    MODULE_LICENSE("GPL v2");
