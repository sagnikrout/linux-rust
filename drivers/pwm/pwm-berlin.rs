//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-berlin.c
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


//
// Marvell Berlin PWM driver
//
// Copyright (C) 2015 Marvell Technology Group Ltd.
//
// Author: Antoine Tenart <antoine.tenart@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const BERLIN_PWM_EN: c_uint = 0x0;

pub const BERLIN_PWM_CONTROL: c_uint = 0x4;
//
// The prescaler claims to support 8 different moduli, configured using the
// low three bits of PWM_CONTROL. (Sequentially, they are 1, 4, 8, 16, 64,
// 256, 1024, and 4096.)  However, the moduli from 4 to 1024 appear to be
// implemented by internally shifting TCNT left without adding additional
// bits. So, the max TCNT that actually works for a modulus of 4 is 0x3fff;
// for 8, 0x1fff; and so on. This means that those moduli are entirely
// useless, as we could just do the shift ourselves. The 4096 modulus is
// implemented with a real prescaler, so we do use that, but we treat it
// as a flag instead of pretending the modulus is actually configurable.
//
pub const BERLIN_PWM_PRESCALE_4096: c_uint = 0x7;

pub const BERLIN_PWM_DUTY: c_uint = 0x8;
pub const BERLIN_PWM_TCNT: c_uint = 0xc;
pub const BERLIN_PWM_MAX_TCNT: c_int = 65535;
pub const BERLIN_PWM_NUMPWMS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin_pwm_channel {
    pub enable: u32,
    pub ctrl: u32,
    pub duty: u32,
    pub tcnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin_pwm_chip {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub channel: [berlin_pwm_channel; BERLIN_PWM_NUMPWMS],
}

    static inline struct berlin_pwm_chip *to_berlin_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static inline u32 berlin_pwm_readl(struct berlin_pwm_chip *bpc,
    unsigned int channel, unsigned long offset)
    {
    return readl_relaxed(bpc.base + channel * 0x10 + offset);
    }
    static inline void berlin_pwm_writel(struct berlin_pwm_chip *bpc,
    unsigned int channel, u32 value,
    unsigned long offset)
    {
    writel_relaxed(value, bpc.base + channel * 0x10 + offset);
    }
    static int berlin_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    u64 duty_ns, u64 period_ns)
    {
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    let mut prescale_4096: bool = false;
    u32 value, duty, period;
    u64 cycles;
    cycles = clk_get_rate(bpc.clk);
    cycles *= period_ns;
    do_div(cycles, NSEC_PER_SEC);
    if (cycles > BERLIN_PWM_MAX_TCNT) {
    prescale_4096 = true;
    cycles >>= 12; // Prescaled by 4096
    if (cycles > BERLIN_PWM_MAX_TCNT)
    return -ERANGE;
    }
    period = cycles;
    cycles *= duty_ns;
    do_div(cycles, period_ns);
    duty = cycles;
    value = berlin_pwm_readl(bpc, pwm.hwpwm, BERLIN_PWM_CONTROL);
    if (prescale_4096)
    value |= BERLIN_PWM_PRESCALE_4096;
    else
    value &= ~BERLIN_PWM_PRESCALE_4096;
    berlin_pwm_writel(bpc, pwm.hwpwm, value, BERLIN_PWM_CONTROL);
    berlin_pwm_writel(bpc, pwm.hwpwm, duty, BERLIN_PWM_DUTY);
    berlin_pwm_writel(bpc, pwm.hwpwm, period, BERLIN_PWM_TCNT);
    return 0;
    }
    static int berlin_pwm_set_polarity(struct pwm_chip *chip,
    struct pwm_device *pwm,
    enum pwm_polarity polarity)
    {
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    u32 value;
    value = berlin_pwm_readl(bpc, pwm.hwpwm, BERLIN_PWM_CONTROL);
    if (polarity == PWM_POLARITY_NORMAL)
    value &= ~BERLIN_PWM_INVERT_POLARITY;
    else
    value |= BERLIN_PWM_INVERT_POLARITY;
    berlin_pwm_writel(bpc, pwm.hwpwm, value, BERLIN_PWM_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn berlin_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int berlin_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    u32 value;
    value = berlin_pwm_readl(bpc, pwm.hwpwm, BERLIN_PWM_EN);
    value |= BERLIN_PWM_ENABLE;
    berlin_pwm_writel(bpc, pwm.hwpwm, value, BERLIN_PWM_EN);
    return 0;
    }
    static void berlin_pwm_disable(struct pwm_chip *chip,
    struct pwm_device *pwm)
    {
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    u32 value;
    value = berlin_pwm_readl(bpc, pwm.hwpwm, BERLIN_PWM_EN);
    value &= ~BERLIN_PWM_ENABLE;
    berlin_pwm_writel(bpc, pwm.hwpwm, value, BERLIN_PWM_EN);
    }
    static int berlin_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    let mut enabled: bool = pwm.state.enabled;
    if (state.polarity != pwm.state.polarity) {
    if (enabled) {
    berlin_pwm_disable(chip, pwm);
    enabled = false;
    }
    err = berlin_pwm_set_polarity(chip, pwm, state.polarity);
    if (err)
    return err;
    }
    if (!state.enabled) {
    if (enabled)
    berlin_pwm_disable(chip, pwm);
    return 0;
    }
    err = berlin_pwm_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!enabled)
    return berlin_pwm_enable(chip, pwm);
    return 0;
    }
    static const struct pwm_ops berlin_pwm_ops = {
    .apply = berlin_pwm_apply,
    };
    static const struct of_device_id berlin_pwm_match[] = {
    { .compatible = "marvell,berlin-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, berlin_pwm_match);
#[no_mangle]
unsafe extern "C" fn berlin_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int berlin_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct berlin_pwm_chip *bpc;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, BERLIN_PWM_NUMPWMS, sizeof(*bpc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    bpc = to_berlin_pwm_chip(chip);
    bpc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(bpc.base))
    return PTR_ERR(bpc.base);
    bpc.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(bpc.clk))
    return PTR_ERR(bpc.clk);
    chip.ops = &berlin_pwm_ops;
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "failed to add PWM chip\n");
    platform_set_drvdata(pdev, chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn berlin_pwm_suspend(dev: *mut device) -> c_int {
    static int berlin_pwm_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    unsigned int i;
    for (i = 0; i < chip.npwm; i++) {
    struct berlin_pwm_channel *channel = &bpc.channel[i];
    channel.enable = berlin_pwm_readl(bpc, i, BERLIN_PWM_EN);
    channel.ctrl = berlin_pwm_readl(bpc, i, BERLIN_PWM_CONTROL);
    channel.duty = berlin_pwm_readl(bpc, i, BERLIN_PWM_DUTY);
    channel.tcnt = berlin_pwm_readl(bpc, i, BERLIN_PWM_TCNT);
    }
    clk_disable_unprepare(bpc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn berlin_pwm_resume(dev: *mut device) -> c_int {
    static int berlin_pwm_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct berlin_pwm_chip *bpc = to_berlin_pwm_chip(chip);
    unsigned int i;
    int ret;
    ret = clk_prepare_enable(bpc.clk);
    if (ret)
    return ret;
    for (i = 0; i < chip.npwm; i++) {
    struct berlin_pwm_channel *channel = &bpc.channel[i];
    berlin_pwm_writel(bpc, i, channel.ctrl, BERLIN_PWM_CONTROL);
    berlin_pwm_writel(bpc, i, channel.duty, BERLIN_PWM_DUTY);
    berlin_pwm_writel(bpc, i, channel.tcnt, BERLIN_PWM_TCNT);
    berlin_pwm_writel(bpc, i, channel.enable, BERLIN_PWM_EN);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(berlin_pwm_pm_ops, berlin_pwm_suspend,
    berlin_pwm_resume);
    static struct platform_driver berlin_pwm_driver = {
    .probe = berlin_pwm_probe,
    .driver = {
    .name = "berlin-pwm",
    .of_match_table = berlin_pwm_match,
    .pm = pm_ptr(&berlin_pwm_pm_ops),
    },
    };
    module_platform_driver(berlin_pwm_driver);
    MODULE_AUTHOR("Antoine Tenart <antoine.tenart@free-electrons.com>");
    MODULE_DESCRIPTION("Marvell Berlin PWM driver");
    MODULE_LICENSE("GPL v2");
