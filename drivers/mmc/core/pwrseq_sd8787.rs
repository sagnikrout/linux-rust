//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/core/pwrseq_sd8787.c
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
// pwrseq_sd8787.c - power sequence support for Marvell SD8787 BT + Wifi chip
//
// Copyright (C) 2016 Matt Ranostay <matt@ranostay.consulting>
//
// Based on the original work pwrseq_simple.c
// Copyright (C) 2014 Linaro Ltd
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_pwrseq_sd8787 {
    pub pwrseq: mmc_pwrseq,
    pub reset_gpio: *mut gpio_desc,
    pub pwrdn_gpio: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_sd8787_pre_power_on(host: *mut mmc_host) {
    static void mmc_pwrseq_sd8787_pre_power_on(struct mmc_host *host)
    {
    struct mmc_pwrseq_sd8787 *pwrseq = to_pwrseq_sd8787(host.pwrseq);
    gpiod_set_value_cansleep(pwrseq.reset_gpio, 1);
    msleep(300);
    gpiod_set_value_cansleep(pwrseq.pwrdn_gpio, 1);
    }
#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_sd8787_power_off(host: *mut mmc_host) {
    static void mmc_pwrseq_sd8787_power_off(struct mmc_host *host)
    {
    struct mmc_pwrseq_sd8787 *pwrseq = to_pwrseq_sd8787(host.pwrseq);
    gpiod_set_value_cansleep(pwrseq.pwrdn_gpio, 0);
    gpiod_set_value_cansleep(pwrseq.reset_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_wilc1000_pre_power_on(host: *mut mmc_host) {
    static void mmc_pwrseq_wilc1000_pre_power_on(struct mmc_host *host)
    {
    struct mmc_pwrseq_sd8787 *pwrseq = to_pwrseq_sd8787(host.pwrseq);
// The pwrdn_gpio is really CHIP_EN, reset_gpio is RESETN
    gpiod_set_value_cansleep(pwrseq.pwrdn_gpio, 1);
    msleep(5);
    gpiod_set_value_cansleep(pwrseq.reset_gpio, 1);
    }
#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_wilc1000_power_off(host: *mut mmc_host) {
    static void mmc_pwrseq_wilc1000_power_off(struct mmc_host *host)
    {
    struct mmc_pwrseq_sd8787 *pwrseq = to_pwrseq_sd8787(host.pwrseq);
    gpiod_set_value_cansleep(pwrseq.reset_gpio, 0);
    gpiod_set_value_cansleep(pwrseq.pwrdn_gpio, 0);
    }
    static const struct mmc_pwrseq_ops mmc_pwrseq_sd8787_ops = {
    .pre_power_on = mmc_pwrseq_sd8787_pre_power_on,
    .power_off = mmc_pwrseq_sd8787_power_off,
    };
    static const struct mmc_pwrseq_ops mmc_pwrseq_wilc1000_ops = {
    .pre_power_on = mmc_pwrseq_wilc1000_pre_power_on,
    .power_off = mmc_pwrseq_wilc1000_power_off,
    };
    static const struct of_device_id mmc_pwrseq_sd8787_of_match[] = {
    { .compatible = "mmc-pwrseq-sd8787", .data = &mmc_pwrseq_sd8787_ops },
    { .compatible = "mmc-pwrseq-wilc1000", .data = &mmc_pwrseq_wilc1000_ops },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, mmc_pwrseq_sd8787_of_match);
#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_sd8787_probe(pdev: *mut platform_device) -> c_int {
    static int mmc_pwrseq_sd8787_probe(struct platform_device *pdev)
    {
    struct mmc_pwrseq_sd8787 *pwrseq;
    struct device *dev = &pdev.dev;
    const struct of_device_id *match;
    pwrseq = devm_kzalloc(dev, sizeof(*pwrseq), GFP_KERNEL);
    if (!pwrseq)
    return -ENOMEM;
    match = of_match_node(mmc_pwrseq_sd8787_of_match, pdev.dev.of_node);
    pwrseq.pwrdn_gpio = devm_gpiod_get(dev, "powerdown", GPIOD_OUT_LOW);
    if (IS_ERR(pwrseq.pwrdn_gpio))
    return PTR_ERR(pwrseq.pwrdn_gpio);
    pwrseq.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(pwrseq.reset_gpio))
    return PTR_ERR(pwrseq.reset_gpio);
    pwrseq.pwrseq.dev = dev;
    pwrseq.pwrseq.ops = match.data;
    pwrseq.pwrseq.owner = THIS_MODULE;
    platform_set_drvdata(pdev, pwrseq);
    return mmc_pwrseq_register(&pwrseq.pwrseq);
    }
#[no_mangle]
unsafe extern "C" fn mmc_pwrseq_sd8787_remove(pdev: *mut platform_device) {
    static void mmc_pwrseq_sd8787_remove(struct platform_device *pdev)
    {
    struct mmc_pwrseq_sd8787 *pwrseq = platform_get_drvdata(pdev);
    mmc_pwrseq_unregister(&pwrseq.pwrseq);
    }
    static struct platform_driver mmc_pwrseq_sd8787_driver = {
    .probe = mmc_pwrseq_sd8787_probe,
    .remove = mmc_pwrseq_sd8787_remove,
    .driver = {
    .name = "pwrseq_sd8787",
    .of_match_table = mmc_pwrseq_sd8787_of_match,
    },
    };
    module_platform_driver(mmc_pwrseq_sd8787_driver);
    MODULE_DESCRIPTION("Power sequence support for Marvell SD8787 BT + Wifi chip");
    MODULE_LICENSE("GPL v2");
