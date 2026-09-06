//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-lpc32xx.c
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
// Copyright 2012 Alexandre Pereira da Silva <aletes.xgr@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_pwm_chip {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
}

    static inline struct lpc32xx_pwm_chip *to_lpc32xx_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int lpc32xx_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns)
    {
    struct lpc32xx_pwm_chip *lpc32xx = to_lpc32xx_pwm_chip(chip);
    unsigned long long c;
    int period_cycles, duty_cycles;
    u32 val;
    c = clk_get_rate(lpc32xx.clk);
// The highest acceptable divisor is 256, which is represented by 0
    period_cycles = div64_u64(c * period_ns,
    (unsigned long long)NSEC_PER_SEC * 256);
    if (!period_cycles || period_cycles > 256)
    return -ERANGE;
    if (period_cycles == 256)
    period_cycles = 0;
// Compute 256 x #duty/period value and care for corner cases
    duty_cycles = div64_u64((unsigned long long)(period_ns - duty_ns) * 256,
    period_ns);
    if (!duty_cycles)
    duty_cycles = 1;
    if (duty_cycles > 255)
    duty_cycles = 255;
    val = readl(lpc32xx.base);
    val &= ~0xFFFF;
    val |= (period_cycles << 8) | duty_cycles;
    writel(val, lpc32xx.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int lpc32xx_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct lpc32xx_pwm_chip *lpc32xx = to_lpc32xx_pwm_chip(chip);
    u32 val;
    int ret;
    ret = clk_prepare_enable(lpc32xx.clk);
    if (ret)
    return ret;
    val = readl(lpc32xx.base);
    val |= PWM_ENABLE;
    writel(val, lpc32xx.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_pwm_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void lpc32xx_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct lpc32xx_pwm_chip *lpc32xx = to_lpc32xx_pwm_chip(chip);
    u32 val;
    val = readl(lpc32xx.base);
    val &= ~PWM_ENABLE;
    writel(val, lpc32xx.base);
    clk_disable_unprepare(lpc32xx.clk);
    }
    static int lpc32xx_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (pwm.state.enabled)
    lpc32xx_pwm_disable(chip, pwm);
    return 0;
    }
    err = lpc32xx_pwm_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    err = lpc32xx_pwm_enable(chip, pwm);
    return err;
    }
    static const struct pwm_ops lpc32xx_pwm_ops = {
    .apply = lpc32xx_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct lpc32xx_pwm_chip *lpc32xx;
    int ret;
    u32 val;
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*lpc32xx));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    lpc32xx = to_lpc32xx_pwm_chip(chip);
    lpc32xx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lpc32xx.base))
    return PTR_ERR(lpc32xx.base);
    lpc32xx.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(lpc32xx.clk))
    return PTR_ERR(lpc32xx.clk);
    chip.ops = &lpc32xx_pwm_ops;
// If PWM is disabled, configure the output to the default value
    val = readl(lpc32xx.base);
    val &= ~PWM_PIN_LEVEL;
    writel(val, lpc32xx.base);
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to add PWM chip, error %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id lpc32xx_pwm_dt_ids[] = {
    { .compatible = "nxp,lpc3220-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, lpc32xx_pwm_dt_ids);
    static struct platform_driver lpc32xx_pwm_driver = {
    .driver = {
    .name = "lpc32xx-pwm",
    .of_match_table = lpc32xx_pwm_dt_ids,
    },
    .probe = lpc32xx_pwm_probe,
    };
    module_platform_driver(lpc32xx_pwm_driver);
    MODULE_ALIAS("platform:lpc32xx-pwm");
    MODULE_AUTHOR("Alexandre Pereira da Silva <aletes.xgr@gmail.com>");
    MODULE_DESCRIPTION("LPC32XX PWM Driver");
    MODULE_LICENSE("GPL v2");
