//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-bcm2835.c
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
// Copyright 2014 Bart Tanghe <bart.tanghe@thomasmore.be>
//

pub const PWM_CONTROL: c_uint = 0x000;

pub const PWM_CONTROL_MASK: c_uint = 0xff;
pub const PWM_MODE: c_uint = 0x80		/* set timer in PWM mode */;

pub const PERIOD_MIN: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pwm {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rate: c_ulong,
}

    static inline struct bcm2835_pwm *to_bcm2835_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int bcm2835_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct bcm2835_pwm *pc = to_bcm2835_pwm(chip);
    unsigned long long period_cycles;
    u64 max_period;
    u32 val;
//
// period_cycles must be a 32 bit value, so period * rate / NSEC_PER_SEC
// must be <= U32_MAX. As U32_MAX * NSEC_PER_SEC < U64_MAX the
// multiplication period * rate doesn't overflow.
// To calculate the maximal possible period that guarantees the
// above inequality:
//
// round(period * rate / NSEC_PER_SEC) <= U32_MAX
// <=> period * rate / NSEC_PER_SEC < U32_MAX + 0.5
// <=> period * rate < (U32_MAX + 0.5) * NSEC_PER_SEC
// <=> period < ((U32_MAX + 0.5) * NSEC_PER_SEC) / rate
// <=> period < ((U32_MAX * NSEC_PER_SEC + NSEC_PER_SEC/2) / rate
// <=> period <= ceil((U32_MAX * NSEC_PER_SEC + NSEC_PER_SEC/2) / rate) - 1
//
    max_period = DIV_ROUND_UP_ULL((u64)U32_MAX * NSEC_PER_SEC + NSEC_PER_SEC / 2, pc.rate) - 1;
    if (state.period > max_period)
    return -EINVAL;
// set period
    period_cycles = DIV_ROUND_CLOSEST_ULL(state.period * pc.rate, NSEC_PER_SEC);
// don't accept a period that is too small
    if (period_cycles < PERIOD_MIN)
    return -EINVAL;
    writel(period_cycles, pc.base + PERIOD(pwm.hwpwm));
// set duty cycle
    val = DIV_ROUND_CLOSEST_ULL(state.duty_cycle * pc.rate, NSEC_PER_SEC);
    writel(val, pc.base + DUTY(pwm.hwpwm));
// set polarity
    val = readl(pc.base + PWM_CONTROL);
    val &= ~(PWM_CONTROL_MASK << PWM_CONTROL_SHIFT(pwm.hwpwm));
    val |= PWM_MODE << PWM_CONTROL_SHIFT(pwm.hwpwm);
    if (state.polarity == PWM_POLARITY_NORMAL)
    val &= ~(PWM_POLARITY << PWM_CONTROL_SHIFT(pwm.hwpwm));
    else
    val |= PWM_POLARITY << PWM_CONTROL_SHIFT(pwm.hwpwm);
// enable/disable
    if (state.enabled)
    val |= PWM_ENABLE << PWM_CONTROL_SHIFT(pwm.hwpwm);
    else
    val &= ~(PWM_ENABLE << PWM_CONTROL_SHIFT(pwm.hwpwm));
    writel(val, pc.base + PWM_CONTROL);
    return 0;
    }
    static const struct pwm_ops bcm2835_pwm_ops = {
    .apply = bcm2835_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn bcm2835_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_pwm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwm_chip *chip;
    struct bcm2835_pwm *pc;
    int ret;
    chip = devm_pwmchip_alloc(dev, 2, sizeof(*pc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pc = to_bcm2835_pwm(chip);
    pc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pc.base))
    return PTR_ERR(pc.base);
    pc.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(pc.clk))
    return dev_err_probe(dev, PTR_ERR(pc.clk),
    "clock not found\n");
    ret = devm_clk_rate_exclusive_get(dev, pc.clk);
    if (ret)
    return dev_err_probe(dev, ret,
    "fail to get exclusive rate\n");
    pc.rate = clk_get_rate(pc.clk);
    if (!pc.rate)
    return dev_err_probe(dev, -EINVAL,
    "failed to get clock rate\n");
    chip.ops = &bcm2835_pwm_ops;
    chip.atomic = true;
    platform_set_drvdata(pdev, pc);
    ret = devm_pwmchip_add(dev, chip);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to add pwmchip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pwm_suspend(dev: *mut device) -> c_int {
    static int bcm2835_pwm_suspend(struct device *dev)
    {
    struct bcm2835_pwm *pc = dev_get_drvdata(dev);
    clk_disable_unprepare(pc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_pwm_resume(dev: *mut device) -> c_int {
    static int bcm2835_pwm_resume(struct device *dev)
    {
    struct bcm2835_pwm *pc = dev_get_drvdata(dev);
    return clk_prepare_enable(pc.clk);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(bcm2835_pwm_pm_ops, bcm2835_pwm_suspend,
    bcm2835_pwm_resume);
    static const struct of_device_id bcm2835_pwm_of_match[] = {
    { .compatible = "brcm,bcm2835-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, bcm2835_pwm_of_match);
    static struct platform_driver bcm2835_pwm_driver = {
    .driver = {
    .name = "bcm2835-pwm",
    .of_match_table = bcm2835_pwm_of_match,
    .pm = pm_ptr(&bcm2835_pwm_pm_ops),
    },
    .probe = bcm2835_pwm_probe,
    };
    module_platform_driver(bcm2835_pwm_driver);
    MODULE_AUTHOR("Bart Tanghe <bart.tanghe@thomasmore.be>");
    MODULE_DESCRIPTION("Broadcom BCM2835 PWM driver");
    MODULE_LICENSE("GPL v2");
