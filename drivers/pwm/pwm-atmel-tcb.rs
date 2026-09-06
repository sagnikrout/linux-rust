//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-atmel-tcb.c
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
// Copyright (C) Overkiz SAS 2012
//
// Author: Boris BREZILLON <b.brezillon@overkiz.com>
//

pub const NPWM: c_int = 2;

    ATMEL_TC_AEEVT | ATMEL_TC_ASWTRG)

    ATMEL_TC_BEEVT | ATMEL_TC_BSWTRG)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tcb_pwm_device {
    pub /: *mut *mut unsigned div; / PWM clock divider,
    pub /: *mut *mut unsigned duty; / PWM duty expressed in clk cycles,
    pub /: *mut *mut unsigned period; / PWM period expressed in clk cycles,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tcb_channel {
    pub enabled: u32,
    pub cmr: u32,
    pub ra: u32,
    pub rb: u32,
    pub rc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tcb_pwm_chip {
    pub lock: spinlock_t,
    pub channel: u8,
    pub width: u8,
    pub rate: c_ulong,
    pub slow_rate: c_ulong,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub gclk: *mut clk,
    pub slow_clk: *mut clk,
    pub pwms: [atmel_tcb_pwm_device; NPWM],
    pub bkup: atmel_tcb_channel,
}

    static const u8 atmel_tcb_divisors[] = { 2, 8, 32, 128, 0, };
    static inline struct atmel_tcb_pwm_chip *to_tcb_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int atmel_tcb_pwm_request(struct pwm_chip *chip,
    struct pwm_device *pwm)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_pwm_device *tcbpwm = &tcbpwmc.pwms[pwm.hwpwm];
    unsigned cmr;
    int ret;
    ret = clk_prepare_enable(tcbpwmc.clk);
    if (ret)
    return ret;
    tcbpwm.duty = 0;
    tcbpwm.period = 0;
    tcbpwm.div = 0;
    guard(spinlock)(&tcbpwmc.lock);
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), &cmr);
//
// Get init config from Timer Counter registers if
// Timer Counter is already configured as a PWM generator.
//
    if (cmr & ATMEL_TC_WAVE) {
    if (pwm.hwpwm == 0)
    regmap_read(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, RA),
    &tcbpwm.duty);
    else
    regmap_read(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, RB),
    &tcbpwm.duty);
    tcbpwm.div = cmr & ATMEL_TC_TCCLKS;
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, RC),
    &tcbpwm.period);
    cmr &= (ATMEL_TC_TCCLKS | ATMEL_TC_ACMR_MASK |
    ATMEL_TC_BCMR_MASK);
    } else
    cmr = 0;
    cmr |= ATMEL_TC_WAVE | ATMEL_TC_WAVESEL_UP_AUTO | ATMEL_TC_EEVT_XC0;
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), cmr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tcb_pwm_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void atmel_tcb_pwm_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    clk_disable_unprepare(tcbpwmc.clk);
    }
    static void atmel_tcb_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm,
    enum pwm_polarity polarity)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_pwm_device *tcbpwm = &tcbpwmc.pwms[pwm.hwpwm];
    unsigned cmr;
//
// If duty is 0 the timer will be stopped and we have to
// configure the output correctly on software trigger:
// - set output to high if PWM_POLARITY_INVERSED
// - set output to low if PWM_POLARITY_NORMAL
//
// This is why we're reverting polarity in this case.
//
    if (tcbpwm.duty == 0)
    polarity = !polarity;
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), &cmr);
// flush old setting and set the new one
    if (pwm.hwpwm == 0) {
    cmr &= ~ATMEL_TC_ACMR_MASK;
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_ASWTRG_CLEAR;
    else
    cmr |= ATMEL_TC_ASWTRG_SET;
    } else {
    cmr &= ~ATMEL_TC_BCMR_MASK;
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_BSWTRG_CLEAR;
    else
    cmr |= ATMEL_TC_BSWTRG_SET;
    }
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), cmr);
//
// Use software trigger to apply the new setting.
// If both PWM devices in this group are disabled we stop the clock.
//
    if (!(cmr & (ATMEL_TC_ACPC | ATMEL_TC_BCPC))) {
    regmap_write(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, CCR),
    ATMEL_TC_SWTRG | ATMEL_TC_CLKDIS);
    tcbpwmc.bkup.enabled = 1;
    } else {
    regmap_write(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, CCR),
    ATMEL_TC_SWTRG);
    tcbpwmc.bkup.enabled = 0;
    }
    }
    static int atmel_tcb_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm,
    enum pwm_polarity polarity)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_pwm_device *tcbpwm = &tcbpwmc.pwms[pwm.hwpwm];
    u32 cmr;
//
// If duty is 0 the timer will be stopped and we have to
// configure the output correctly on software trigger:
// - set output to high if PWM_POLARITY_INVERSED
// - set output to low if PWM_POLARITY_NORMAL
//
// This is why we're reverting polarity in this case.
//
    if (tcbpwm.duty == 0)
    polarity = !polarity;
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), &cmr);
// flush old setting and set the new one
    cmr &= ~ATMEL_TC_TCCLKS;
    if (pwm.hwpwm == 0) {
    cmr &= ~ATMEL_TC_ACMR_MASK;
// Set CMR flags according to given polarity
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_ASWTRG_CLEAR;
    else
    cmr |= ATMEL_TC_ASWTRG_SET;
    } else {
    cmr &= ~ATMEL_TC_BCMR_MASK;
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_BSWTRG_CLEAR;
    else
    cmr |= ATMEL_TC_BSWTRG_SET;
    }
//
// If duty is 0 or equal to period there's no need to register
// a specific action on RA/RB and RC compare.
// The output will be configured on software trigger and keep
// this config till next config call.
//
    if (tcbpwm.duty != tcbpwm.period && tcbpwm.duty > 0) {
    if (pwm.hwpwm == 0) {
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_ACPA_SET | ATMEL_TC_ACPC_CLEAR;
    else
    cmr |= ATMEL_TC_ACPA_CLEAR | ATMEL_TC_ACPC_SET;
    } else {
    if (polarity == PWM_POLARITY_INVERSED)
    cmr |= ATMEL_TC_BCPB_SET | ATMEL_TC_BCPC_CLEAR;
    else
    cmr |= ATMEL_TC_BCPB_CLEAR | ATMEL_TC_BCPC_SET;
    }
    }
    cmr |= (tcbpwm.div & ATMEL_TC_TCCLKS);
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CMR), cmr);
    if (pwm.hwpwm == 0)
    regmap_write(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, RA),
    tcbpwm.duty);
    else
    regmap_write(tcbpwmc.regmap,
    ATMEL_TC_REG(tcbpwmc.channel, RB),
    tcbpwm.duty);
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, RC),
    tcbpwm.period);
// Use software trigger to apply the new setting
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(tcbpwmc.channel, CCR),
    ATMEL_TC_SWTRG | ATMEL_TC_CLKEN);
    tcbpwmc.bkup.enabled = 1;
    return 0;
    }
    static int atmel_tcb_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_pwm_device *tcbpwm = &tcbpwmc.pwms[pwm.hwpwm];
// companion PWM sharing register values period and div
    struct atmel_tcb_pwm_device *atcbpwm = &tcbpwmc.pwms[pwm.hwpwm ^ 1];
    let mut i: c_int = 0;
    let mut slowclk: c_int = 0;
    unsigned period;
    unsigned duty;
    let mut rate: c_ulong = tcbpwmc.rate;
    unsigned long long min;
    unsigned long long max;
//
// Find best clk divisor:
// the smallest divisor which can fulfill the period_ns requirements.
// If there is a gclk, the first divisor is actually the gclk selector
//
    if (tcbpwmc.gclk)
    i = 1;
    for (; i < ARRAY_SIZE(atmel_tcb_divisors); ++i) {
    if (atmel_tcb_divisors[i] == 0) {
    slowclk = i;
    continue;
    }
    min = div_u64((u64)NSEC_PER_SEC * atmel_tcb_divisors[i], rate);
    max = min << tcbpwmc.width;
    if (max >= period_ns)
    break;
    }
//
// If none of the divisor are small enough to represent period_ns
// take slow clock (32KHz).
//
    if (i == ARRAY_SIZE(atmel_tcb_divisors)) {
    i = slowclk;
    rate = tcbpwmc.slow_rate;
    min = div_u64(NSEC_PER_SEC, rate);
    max = min << tcbpwmc.width;
// If period is too big return ERANGE error
    if (max < period_ns)
    return -ERANGE;
    }
    duty = div_u64(duty_ns, min);
    period = div_u64(period_ns, min);
//
// PWM devices provided by the TCB driver are grouped by 2.
// PWM devices in a given group must be configured with the
// same period_ns.
//
// We're checking the period value of the second PWM device
// in this group before applying the new config.
//
    if ((atcbpwm.duty > 0 && atcbpwm.duty != atcbpwm.period) &&
    (atcbpwm.div != i || atcbpwm.period != period)) {
    dev_err(pwmchip_parent(chip),
    "failed to configure period_ns: PWM group already configured with a different value\n");
    return -EINVAL;
    }
    tcbpwm.period = period;
    tcbpwm.div = i;
    tcbpwm.duty = duty;
    return 0;
    }
    static int atmel_tcb_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    int duty_cycle, period;
    int ret;
    guard(spinlock)(&tcbpwmc.lock);
    if (!state.enabled) {
    atmel_tcb_pwm_disable(chip, pwm, state.polarity);
    return 0;
    }
    period = min(state.period, INT_MAX);
    duty_cycle = min(state.duty_cycle, INT_MAX);
    ret = atmel_tcb_pwm_config(chip, pwm, duty_cycle, period);
    if (ret)
    return ret;
    return atmel_tcb_pwm_enable(chip, pwm, state.polarity);
    }
    static const struct pwm_ops atmel_tcb_pwm_ops = {
    .request = atmel_tcb_pwm_request,
    .free = atmel_tcb_pwm_free,
    .apply = atmel_tcb_pwm_apply,
    };
    static struct atmel_tcb_config tcb_rm9200_config = {
    .counter_width = 16,
    };
    static struct atmel_tcb_config tcb_sam9x5_config = {
    .counter_width = 32,
    };
    static struct atmel_tcb_config tcb_sama5d2_config = {
    .counter_width = 32,
    .has_gclk = 1,
    };
    static const struct of_device_id atmel_tcb_of_match[] = {
    { .compatible = "atmel,at91rm9200-tcb", .data = &tcb_rm9200_config },
    { .compatible = "atmel,at91sam9x5-tcb", .data = &tcb_sam9x5_config },
    { .compatible = "atmel,sama5d2-tcb", .data = &tcb_sama5d2_config },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn atmel_tcb_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_tcb_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    const struct of_device_id *match;
    struct atmel_tcb_pwm_chip *tcbpwmc;
    const struct atmel_tcb_config *config;
    struct device_node *np = pdev.dev.of_node;
    char clk_name[] = "t0_clk";
    int err;
    int channel;
    chip = devm_pwmchip_alloc(&pdev.dev, NPWM, sizeof(*tcbpwmc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    tcbpwmc = to_tcb_chip(chip);
    err = of_property_read_u32(np, "reg", &channel);
    if (err < 0) {
    dev_err(&pdev.dev,
    "failed to get Timer Counter Block channel from device tree (error: %d)\n",
    err);
    return err;
    }
    tcbpwmc.regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(tcbpwmc.regmap))
    return PTR_ERR(tcbpwmc.regmap);
    tcbpwmc.slow_clk = of_clk_get_by_name(np.parent, "slow_clk");
    if (IS_ERR(tcbpwmc.slow_clk))
    return PTR_ERR(tcbpwmc.slow_clk);
    clk_name[1] += channel;
    tcbpwmc.clk = of_clk_get_by_name(np.parent, clk_name);
    if (IS_ERR(tcbpwmc.clk))
    tcbpwmc.clk = of_clk_get_by_name(np.parent, "t0_clk");
    if (IS_ERR(tcbpwmc.clk)) {
    err = PTR_ERR(tcbpwmc.clk);
    goto err_slow_clk;
    }
    match = of_match_node(atmel_tcb_of_match, np.parent);
    config = match.data;
    if (config.has_gclk) {
    tcbpwmc.gclk = of_clk_get_by_name(np.parent, "gclk");
    if (IS_ERR(tcbpwmc.gclk)) {
    err = PTR_ERR(tcbpwmc.gclk);
    goto err_clk;
    }
    }
    chip.ops = &atmel_tcb_pwm_ops;
    chip.atomic = true;
    tcbpwmc.channel = channel;
    tcbpwmc.width = config.counter_width;
    err = clk_prepare_enable(tcbpwmc.clk);
    if (err)
    goto err_gclk;
    err = clk_prepare_enable(tcbpwmc.slow_clk);
    if (err)
    goto err_disable_clk;
    err = clk_rate_exclusive_get(tcbpwmc.clk);
    if (err)
    goto err_disable_slow_clk;
    err = clk_rate_exclusive_get(tcbpwmc.slow_clk);
    if (err)
    goto err_clk_unlock;
    tcbpwmc.rate = clk_get_rate(tcbpwmc.clk);
    tcbpwmc.slow_rate = clk_get_rate(tcbpwmc.slow_clk);
    spin_lock_init(&tcbpwmc.lock);
    err = pwmchip_add(chip);
    if (err < 0)
    goto err_slow_clk_unlock;
    platform_set_drvdata(pdev, chip);
    return 0;
    err_slow_clk_unlock:
    clk_rate_exclusive_put(tcbpwmc.slow_clk);
    err_clk_unlock:
    clk_rate_exclusive_put(tcbpwmc.clk);
    err_disable_clk:
    clk_disable_unprepare(tcbpwmc.clk);
    err_disable_slow_clk:
    clk_disable_unprepare(tcbpwmc.slow_clk);
    err_gclk:
    clk_put(tcbpwmc.gclk);
    err_clk:
    clk_put(tcbpwmc.clk);
    err_slow_clk:
    clk_put(tcbpwmc.slow_clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tcb_pwm_remove(pdev: *mut platform_device) {
    static void atmel_tcb_pwm_remove(struct platform_device *pdev)
    {
    struct pwm_chip *chip = platform_get_drvdata(pdev);
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    pwmchip_remove(chip);
    clk_rate_exclusive_put(tcbpwmc.slow_clk);
    clk_rate_exclusive_put(tcbpwmc.clk);
    clk_disable_unprepare(tcbpwmc.clk);
    clk_disable_unprepare(tcbpwmc.slow_clk);
    clk_put(tcbpwmc.gclk);
    clk_put(tcbpwmc.clk);
    clk_put(tcbpwmc.slow_clk);
    }
    static const struct of_device_id atmel_tcb_pwm_dt_ids[] = {
    { .compatible = "atmel,tcb-pwm", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_tcb_pwm_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmel_tcb_pwm_suspend(dev: *mut device) -> c_int {
    static int atmel_tcb_pwm_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_channel *chan = &tcbpwmc.bkup;
    let mut channel: c_uint = tcbpwmc.channel;
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(channel, CMR), &chan.cmr);
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(channel, RA), &chan.ra);
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(channel, RB), &chan.rb);
    regmap_read(tcbpwmc.regmap, ATMEL_TC_REG(channel, RC), &chan.rc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tcb_pwm_resume(dev: *mut device) -> c_int {
    static int atmel_tcb_pwm_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct atmel_tcb_pwm_chip *tcbpwmc = to_tcb_chip(chip);
    struct atmel_tcb_channel *chan = &tcbpwmc.bkup;
    let mut channel: c_uint = tcbpwmc.channel;
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(channel, CMR), chan.cmr);
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(channel, RA), chan.ra);
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(channel, RB), chan.rb);
    regmap_write(tcbpwmc.regmap, ATMEL_TC_REG(channel, RC), chan.rc);
    if (chan.enabled)
    regmap_write(tcbpwmc.regmap,
    ATMEL_TC_CLKEN | ATMEL_TC_SWTRG,
    ATMEL_TC_REG(channel, CCR));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(atmel_tcb_pwm_pm_ops, atmel_tcb_pwm_suspend,
    atmel_tcb_pwm_resume);
    static struct platform_driver atmel_tcb_pwm_driver = {
    .driver = {
    .name = "atmel-tcb-pwm",
    .of_match_table = atmel_tcb_pwm_dt_ids,
    .pm = pm_ptr(&atmel_tcb_pwm_pm_ops),
    },
    .probe = atmel_tcb_pwm_probe,
    .remove = atmel_tcb_pwm_remove,
    };
    module_platform_driver(atmel_tcb_pwm_driver);
    MODULE_AUTHOR("Boris BREZILLON <b.brezillon@overkiz.com>");
    MODULE_DESCRIPTION("Atmel Timer Counter Pulse Width Modulation Driver");
    MODULE_LICENSE("GPL v2");
