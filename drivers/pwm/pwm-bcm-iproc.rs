//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-bcm-iproc.c
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
// Copyright (C) 2016 Broadcom

pub const IPROC_PWM_CTRL_OFFSET: c_uint = 0x00;

pub const IPROC_PWM_PERIOD_MIN: c_uint = 0x02;
pub const IPROC_PWM_PERIOD_MAX: c_uint = 0xffff;

pub const IPROC_PWM_DUTY_CYCLE_MIN: c_uint = 0x00;
pub const IPROC_PWM_DUTY_CYCLE_MAX: c_uint = 0xffff;
pub const IPROC_PWM_PRESCALE_OFFSET: c_uint = 0x24;
pub const IPROC_PWM_PRESCALE_BITS: c_uint = 0x06;

    IPROC_PWM_PRESCALE_BITS)

    IPROC_PWM_PRESCALE_SHIFT(ch))
pub const IPROC_PWM_PRESCALE_MIN: c_uint = 0x00;
pub const IPROC_PWM_PRESCALE_MAX: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pwmc {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline struct iproc_pwmc *to_iproc_pwmc(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
unsafe extern "C" fn iproc_pwmc_enable(ip: *mut iproc_pwmc, channel: c_uint) {
    static void iproc_pwmc_enable(struct iproc_pwmc *ip, unsigned int channel)
    {
    u32 value;
    value = readl(ip.base + IPROC_PWM_CTRL_OFFSET);
    value |= 1 << IPROC_PWM_CTRL_EN_SHIFT(channel);
    writel(value, ip.base + IPROC_PWM_CTRL_OFFSET);
// must be a 400 ns delay between clearing and setting enable bit
    ndelay(400);
    }
#[no_mangle]
unsafe extern "C" fn iproc_pwmc_disable(ip: *mut iproc_pwmc, channel: c_uint) {
    static void iproc_pwmc_disable(struct iproc_pwmc *ip, unsigned int channel)
    {
    u32 value;
    value = readl(ip.base + IPROC_PWM_CTRL_OFFSET);
    value &= ~(1 << IPROC_PWM_CTRL_EN_SHIFT(channel));
    writel(value, ip.base + IPROC_PWM_CTRL_OFFSET);
// must be a 400 ns delay between clearing and setting enable bit
    ndelay(400);
    }
    static int iproc_pwmc_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct iproc_pwmc *ip = to_iproc_pwmc(chip);
    u64 tmp, multi, rate;
    u32 value, prescale;
    value = readl(ip.base + IPROC_PWM_CTRL_OFFSET);
    if (value & BIT(IPROC_PWM_CTRL_EN_SHIFT(pwm.hwpwm)))
    state.enabled = true;
    else
    state.enabled = false;
    if (value & BIT(IPROC_PWM_CTRL_POLARITY_SHIFT(pwm.hwpwm)))
    state.polarity = PWM_POLARITY_NORMAL;
    else
    state.polarity = PWM_POLARITY_INVERSED;
    rate = clk_get_rate(ip.clk);
    if (rate == 0) {
    state.period = 0;
    state.duty_cycle = 0;
    return 0;
    }
    value = readl(ip.base + IPROC_PWM_PRESCALE_OFFSET);
    prescale = value >> IPROC_PWM_PRESCALE_SHIFT(pwm.hwpwm);
    prescale &= IPROC_PWM_PRESCALE_MAX;
    multi = NSEC_PER_SEC * (prescale + 1);
    value = readl(ip.base + IPROC_PWM_PERIOD_OFFSET(pwm.hwpwm));
    tmp = (value & IPROC_PWM_PERIOD_MAX) * multi;
    state.period = div64_u64(tmp, rate);
    value = readl(ip.base + IPROC_PWM_DUTY_CYCLE_OFFSET(pwm.hwpwm));
    tmp = (value & IPROC_PWM_PERIOD_MAX) * multi;
    state.duty_cycle = div64_u64(tmp, rate);
    return 0;
    }
    static int iproc_pwmc_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    let mut prescale: c_ulong = IPROC_PWM_PRESCALE_MIN;
    struct iproc_pwmc *ip = to_iproc_pwmc(chip);
    u32 value, period, duty;
    u64 rate;
    rate = clk_get_rate(ip.clk);
//
// Find period count, duty count and prescale to suit duty_cycle and
// period. This is done according to formulas described below:
//
// period_ns = 10^9 * (PRESCALE + 1) * PC / PWM_CLK_RATE
// duty_ns = 10^9 * (PRESCALE + 1) * DC / PWM_CLK_RATE
//
// PC = (PWM_CLK_RATE * period_ns) / (10^9 * (PRESCALE + 1))
// DC = (PWM_CLK_RATE * duty_ns) / (10^9 * (PRESCALE + 1))
//
    while (1) {
    u64 value, div;
    div = NSEC_PER_SEC * (prescale + 1);
    value = rate * state.period;
    period = div64_u64(value, div);
    value = rate * state.duty_cycle;
    duty = div64_u64(value, div);
    if (period < IPROC_PWM_PERIOD_MIN)
    return -EINVAL;
    if (period <= IPROC_PWM_PERIOD_MAX &&
    duty <= IPROC_PWM_DUTY_CYCLE_MAX)
    break;
// Otherwise, increase prescale and recalculate counts
    if (++prescale > IPROC_PWM_PRESCALE_MAX)
    return -EINVAL;
    }
    iproc_pwmc_disable(ip, pwm.hwpwm);
// Set prescale
    value = readl(ip.base + IPROC_PWM_PRESCALE_OFFSET);
    value &= ~IPROC_PWM_PRESCALE_MASK(pwm.hwpwm);
    value |= prescale << IPROC_PWM_PRESCALE_SHIFT(pwm.hwpwm);
    writel(value, ip.base + IPROC_PWM_PRESCALE_OFFSET);
// set period and duty cycle
    writel(period, ip.base + IPROC_PWM_PERIOD_OFFSET(pwm.hwpwm));
    writel(duty, ip.base + IPROC_PWM_DUTY_CYCLE_OFFSET(pwm.hwpwm));
// set polarity
    value = readl(ip.base + IPROC_PWM_CTRL_OFFSET);
    if (state.polarity == PWM_POLARITY_NORMAL)
    value |= 1 << IPROC_PWM_CTRL_POLARITY_SHIFT(pwm.hwpwm);
    else
    value &= ~(1 << IPROC_PWM_CTRL_POLARITY_SHIFT(pwm.hwpwm));
    writel(value, ip.base + IPROC_PWM_CTRL_OFFSET);
    if (state.enabled)
    iproc_pwmc_enable(ip, pwm.hwpwm);
    return 0;
    }
    static const struct pwm_ops iproc_pwm_ops = {
    .apply = iproc_pwmc_apply,
    .get_state = iproc_pwmc_get_state,
    };
#[no_mangle]
unsafe extern "C" fn iproc_pwmc_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_pwmc_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct iproc_pwmc *ip;
    unsigned int i;
    u32 value;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 4, sizeof(*ip));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    ip = to_iproc_pwmc(chip);
    platform_set_drvdata(pdev, ip);
    chip.ops = &iproc_pwm_ops;
    ip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ip.base))
    return PTR_ERR(ip.base);
    ip.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(ip.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(ip.clk),
    "failed to get clock\n");
// Set full drive and normal polarity for all channels
    value = readl(ip.base + IPROC_PWM_CTRL_OFFSET);
    for (i = 0; i < chip.npwm; i++) {
    value &= ~(1 << IPROC_PWM_CTRL_TYPE_SHIFT(i));
    value |= 1 << IPROC_PWM_CTRL_POLARITY_SHIFT(i);
    }
    writel(value, ip.base + IPROC_PWM_CTRL_OFFSET);
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add PWM chip\n");
    return 0;
    }
    static const struct of_device_id bcm_iproc_pwmc_dt[] = {
    { .compatible = "brcm,iproc-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bcm_iproc_pwmc_dt);
    static struct platform_driver iproc_pwmc_driver = {
    .driver = {
    .name = "bcm-iproc-pwm",
    .of_match_table = bcm_iproc_pwmc_dt,
    },
    .probe = iproc_pwmc_probe,
    };
    module_platform_driver(iproc_pwmc_driver);
    MODULE_AUTHOR("Yendapally Reddy Dhananjaya Reddy <yendapally.reddy@broadcom.com>");
    MODULE_DESCRIPTION("Broadcom iProc PWM driver");
    MODULE_LICENSE("GPL v2");
