//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-clps711x.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Cirrus Logic CLPS711X PWM driver
// Author: Alexander Shiyan <shc_work@mail.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clps711x_chip {
    pub pmpcon: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline struct clps711x_chip *to_clps711x_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_pwm_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int clps711x_pwm_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct clps711x_chip *priv = to_clps711x_chip(chip);
    let mut freq: c_uint = clk_get_rate(priv.clk);
    if (!freq)
    return -EINVAL;
// Store constant period value
    pwm.args.period = DIV_ROUND_CLOSEST(NSEC_PER_SEC, freq);
    return 0;
    }
    static int clps711x_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct clps711x_chip *priv = to_clps711x_chip(chip);
// PWM0 - bits 4..7, PWM1 - bits 8..11
    let mut shift: u32 = (pwm.hwpwm + 1) * 4;
    u32 pmpcon, val;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (state.period != pwm.args.period)
    return -EINVAL;
    if (state.enabled)
    val = mul_u64_u64_div_u64(state.duty_cycle, 0xf, state.period);
    else
    val = 0;
    pmpcon = readl(priv.pmpcon);
    pmpcon &= ~(0xf << shift);
    pmpcon |= val << shift;
    writel(pmpcon, priv.pmpcon);
    return 0;
    }
    static const struct pwm_ops clps711x_pwm_ops = {
    .request = clps711x_pwm_request,
    .apply = clps711x_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn clps711x_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int clps711x_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct clps711x_chip *priv;
    chip = devm_pwmchip_alloc(&pdev.dev, 2, sizeof(*priv));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    priv = to_clps711x_chip(chip);
    priv.pmpcon = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.pmpcon))
    return PTR_ERR(priv.pmpcon);
    priv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    chip.ops = &clps711x_pwm_ops;
    return devm_pwmchip_add(&pdev.dev, chip);
    }
    static const struct of_device_id clps711x_pwm_dt_ids[] = {
    { .compatible = "cirrus,ep7209-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, clps711x_pwm_dt_ids);
    static struct platform_driver clps711x_pwm_driver = {
    .driver = {
    .name = "clps711x-pwm",
    .of_match_table = clps711x_pwm_dt_ids,
    },
    .probe = clps711x_pwm_probe,
    };
    module_platform_driver(clps711x_pwm_driver);
    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("Cirrus Logic CLPS711X PWM driver");
    MODULE_LICENSE("GPL");
