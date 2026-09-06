//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-atmel-hlcdc.c
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
// Copyright (C) 2014 Free Electrons
// Copyright (C) 2014 Atmel
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//

pub const ATMEL_HLCDC_PWMPS_MAX: c_uint = 0x6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_pwm_errata {
    pub slow_clk_erratum: bool,
    pub div1_clk_erratum: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_pwm {
    pub hlcdc: *mut atmel_hlcdc,
    pub cur_clk: *mut clk,
    pub errata: *const atmel_hlcdc_pwm_errata,
}

    static inline struct atmel_hlcdc_pwm *to_atmel_hlcdc_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int atmel_hlcdc_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct atmel_hlcdc_pwm *atmel = to_atmel_hlcdc_pwm(chip);
    struct atmel_hlcdc *hlcdc = atmel.hlcdc;
    unsigned int status;
    int ret;
    if (state.enabled) {
    struct clk *new_clk = hlcdc.slow_clk;
    let mut pwmcval: u64 = state.duty_cycle * 256;
    unsigned long clk_freq;
    u64 clk_period_ns;
    u32 pwmcfg;
    int pres;
    if (!atmel.errata || !atmel.errata.slow_clk_erratum) {
    clk_freq = clk_get_rate(new_clk);
    if (!clk_freq)
    return -EINVAL;
    clk_period_ns = (u64)NSEC_PER_SEC * 256;
    do_div(clk_period_ns, clk_freq);
    }
// Errata: cannot use slow clk on some IP revisions
    if ((atmel.errata && atmel.errata.slow_clk_erratum) ||
    clk_period_ns > state.period) {
    new_clk = hlcdc.sys_clk;
    clk_freq = clk_get_rate(new_clk);
    if (!clk_freq)
    return -EINVAL;
    clk_period_ns = (u64)NSEC_PER_SEC * 256;
    do_div(clk_period_ns, clk_freq);
    }
    for (pres = 0; pres <= ATMEL_HLCDC_PWMPS_MAX; pres++) {
// Errata: cannot divide by 1 on some IP revisions
    if (!pres && atmel.errata &&
    atmel.errata.div1_clk_erratum)
    continue;
    if ((clk_period_ns << pres) >= state.period)
    break;
    }
    if (pres > ATMEL_HLCDC_PWMPS_MAX)
    return -EINVAL;
    pwmcfg = ATMEL_HLCDC_PWMPS(pres);
    if (new_clk != atmel.cur_clk) {
    let mut gencfg: u32 = 0;
    int ret;
    ret = clk_prepare_enable(new_clk);
    if (ret)
    return ret;
    clk_disable_unprepare(atmel.cur_clk);
    atmel.cur_clk = new_clk;
    if (new_clk == hlcdc.sys_clk)
    gencfg = ATMEL_HLCDC_CLKPWMSEL;
    ret = regmap_update_bits(hlcdc.regmap,
    ATMEL_HLCDC_CFG(0),
    ATMEL_HLCDC_CLKPWMSEL,
    gencfg);
    if (ret)
    return ret;
    }
    do_div(pwmcval, state.period);
//
// The PWM duty cycle is configurable from 0/256 to 255/256 of
// the period cycle. Hence we can't set a duty cycle occupying
// the whole period cycle if we're asked to.
// Set it to 255 if pwmcval is greater than 256.
//
    if (pwmcval > 255)
    pwmcval = 255;
    pwmcfg |= ATMEL_HLCDC_PWMCVAL(pwmcval);
    if (state.polarity == PWM_POLARITY_NORMAL)
    pwmcfg |= ATMEL_HLCDC_PWMPOL;
    ret = regmap_update_bits(hlcdc.regmap, ATMEL_HLCDC_CFG(6),
    ATMEL_HLCDC_PWMCVAL_MASK |
    ATMEL_HLCDC_PWMPS_MASK |
    ATMEL_HLCDC_PWMPOL,
    pwmcfg);
    if (ret)
    return ret;
    ret = regmap_write(hlcdc.regmap, ATMEL_HLCDC_EN,
    ATMEL_HLCDC_PWM);
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(hlcdc.regmap, ATMEL_HLCDC_SR,
    status,
    status & ATMEL_HLCDC_PWM,
    10, 0);
    if (ret)
    return ret;
    } else {
    ret = regmap_write(hlcdc.regmap, ATMEL_HLCDC_DIS,
    ATMEL_HLCDC_PWM);
    if (ret)
    return ret;
    ret = regmap_read_poll_timeout(hlcdc.regmap, ATMEL_HLCDC_SR,
    status,
    !(status & ATMEL_HLCDC_PWM),
    10, 0);
    if (ret)
    return ret;
    clk_disable_unprepare(atmel.cur_clk);
    atmel.cur_clk = core::ptr::null_mut();
    }
    return 0;
    }
    static const struct pwm_ops atmel_hlcdc_pwm_ops = {
    .apply = atmel_hlcdc_pwm_apply,
    };
    static const struct atmel_hlcdc_pwm_errata atmel_hlcdc_pwm_at91sam9x5_errata = {
    .slow_clk_erratum = true,
    };
    static const struct atmel_hlcdc_pwm_errata atmel_hlcdc_pwm_sama5d3_errata = {
    .div1_clk_erratum = true,
    };
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_pwm_suspend(dev: *mut device) -> c_int {
    static int atmel_hlcdc_pwm_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct atmel_hlcdc_pwm *atmel = to_atmel_hlcdc_pwm(chip);
    struct pwm_device *pwm = &chip.pwms[0];
// Keep the periph clock enabled if the PWM is still running.
    if (!pwm.state.enabled)
    clk_disable_unprepare(atmel.hlcdc.periph_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_pwm_resume(dev: *mut device) -> c_int {
    static int atmel_hlcdc_pwm_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct atmel_hlcdc_pwm *atmel = to_atmel_hlcdc_pwm(chip);
    struct pwm_device *pwm = &chip.pwms[0];
    int ret;
// Re-enable the periph clock it was stopped during suspend.
    if (!pwm.state.enabled) {
    ret = clk_prepare_enable(atmel.hlcdc.periph_clk);
    if (ret)
    return ret;
    }
    return atmel_hlcdc_pwm_apply(chip, pwm, &pwm.state);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(atmel_hlcdc_pwm_pm_ops,
    atmel_hlcdc_pwm_suspend, atmel_hlcdc_pwm_resume);
    static const struct of_device_id atmel_hlcdc_dt_ids[] = {
    {
    .compatible = "atmel,at91sam9n12-hlcdc",
// 9n12 has same errata as 9x5 HLCDC PWM
    .data = &atmel_hlcdc_pwm_at91sam9x5_errata,
    }, {
    .compatible = "atmel,at91sam9x5-hlcdc",
    .data = &atmel_hlcdc_pwm_at91sam9x5_errata,
    }, {
    .compatible = "atmel,sama5d2-hlcdc",
    }, {
    .compatible = "atmel,sama5d3-hlcdc",
    .data = &atmel_hlcdc_pwm_sama5d3_errata,
    }, {
    .compatible = "atmel,sama5d4-hlcdc",
    .data = &atmel_hlcdc_pwm_sama5d3_errata,
    }, {
    .compatible = "microchip,sam9x60-hlcdc",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_hlcdc_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_hlcdc_pwm_probe(struct platform_device *pdev)
    {
    const struct of_device_id *match;
    struct device *dev = &pdev.dev;
    struct pwm_chip *chip;
    struct atmel_hlcdc_pwm *atmel;
    struct atmel_hlcdc *hlcdc;
    int ret;
    hlcdc = dev_get_drvdata(dev.parent);
    chip = devm_pwmchip_alloc(dev, 1, sizeof(*atmel));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    atmel = to_atmel_hlcdc_pwm(chip);
    ret = clk_prepare_enable(hlcdc.periph_clk);
    if (ret)
    return ret;
    match = of_match_node(atmel_hlcdc_dt_ids, dev.parent.of_node);
    if (match)
    atmel.errata = match.data;
    atmel.hlcdc = hlcdc;
    chip.ops = &atmel_hlcdc_pwm_ops;
    ret = pwmchip_add(chip);
    if (ret) {
    clk_disable_unprepare(hlcdc.periph_clk);
    return ret;
    }
    platform_set_drvdata(pdev, chip);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_pwm_remove(pdev: *mut platform_device) {
    static void atmel_hlcdc_pwm_remove(struct platform_device *pdev)
    {
    struct pwm_chip *chip = platform_get_drvdata(pdev);
    struct atmel_hlcdc_pwm *atmel = to_atmel_hlcdc_pwm(chip);
    pwmchip_remove(chip);
    clk_disable_unprepare(atmel.hlcdc.periph_clk);
    }
    static const struct of_device_id atmel_hlcdc_pwm_dt_ids[] = {
    { .compatible = "atmel,hlcdc-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_hlcdc_pwm_dt_ids);
    static struct platform_driver atmel_hlcdc_pwm_driver = {
    .driver = {
    .name = "atmel-hlcdc-pwm",
    .of_match_table = atmel_hlcdc_pwm_dt_ids,
    .pm = pm_ptr(&atmel_hlcdc_pwm_pm_ops),
    },
    .probe = atmel_hlcdc_pwm_probe,
    .remove = atmel_hlcdc_pwm_remove,
    };
    module_platform_driver(atmel_hlcdc_pwm_driver);
    MODULE_ALIAS("platform:atmel-hlcdc-pwm");
    MODULE_AUTHOR("Boris Brezillon <boris.brezillon@free-electrons.com>");
    MODULE_DESCRIPTION("Atmel HLCDC PWM driver");
    MODULE_LICENSE("GPL v2");
