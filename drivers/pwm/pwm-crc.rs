//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-crc.c
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
// Copyright (C) 2015 Intel Corporation. All rights reserved.
//
// Author: Shobhit Kumar <shobhit.kumar@intel.com>
//

pub const PWM0_CLK_DIV: c_uint = 0x4B;

pub const PWM_DIV_CLK_0: c_uint = 0x00 /* DIVIDECLK = BASECLK */;
pub const PWM_DIV_CLK_100: c_uint = 0x63 /* DIVIDECLK = BASECLK/100 */;
pub const PWM_DIV_CLK_128: c_uint = 0x7F /* DIVIDECLK = BASECLK/128 */;
pub const PWM0_DUTY_CYCLE: c_uint = 0x4E;
pub const BACKLIGHT_EN: c_uint = 0x51;
pub const PWM_MAX_LEVEL: c_uint = 0xFF;

//
// struct crystalcove_pwm - Crystal Cove PWM controller
// @regmap: the regmap from the parent device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crystalcove_pwm {
    pub regmap: *mut regmap,
}

    static inline struct crystalcove_pwm *to_crc_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
unsafe extern "C" fn crc_pwm_calc_clk_div(period_ns: c_int) -> c_int {
    static int crc_pwm_calc_clk_div(int period_ns)
    {
    int clk_div;
    clk_div = PWM_BASE_CLK_MHZ * period_ns / (256 * NSEC_PER_USEC);
// clk_div 1 - 128, maps to register values 0-127
    if (clk_div > 0)
    clk_div--;
    return clk_div;
    }
    static int crc_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct crystalcove_pwm *crc_pwm = to_crc_pwm(chip);
    struct device *dev = pwmchip_parent(chip);
    int err;
    if (state.period > PWM_MAX_PERIOD_NS) {
    dev_err(dev, "un-supported period_ns\n");
    return -EINVAL;
    }
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (pwm_is_enabled(pwm) && !state.enabled) {
    err = regmap_write(crc_pwm.regmap, BACKLIGHT_EN, 0);
    if (err) {
    dev_err(dev, "Error writing BACKLIGHT_EN %d\n", err);
    return err;
    }
    }
    if (pwm_get_duty_cycle(pwm) != state.duty_cycle ||
    pwm_get_period(pwm) != state.period) {
    let mut level: u64 = state.duty_cycle * PWM_MAX_LEVEL;
    do_div(level, state.period);
    err = regmap_write(crc_pwm.regmap, PWM0_DUTY_CYCLE, level);
    if (err) {
    dev_err(dev, "Error writing PWM0_DUTY_CYCLE %d\n", err);
    return err;
    }
    }
    if (pwm_is_enabled(pwm) && state.enabled &&
    pwm_get_period(pwm) != state.period) {
// changing the clk divisor, clear PWM_OUTPUT_ENABLE first
    err = regmap_write(crc_pwm.regmap, PWM0_CLK_DIV, 0);
    if (err) {
    dev_err(dev, "Error writing PWM0_CLK_DIV %d\n", err);
    return err;
    }
    }
    if (pwm_get_period(pwm) != state.period ||
    pwm_is_enabled(pwm) != state.enabled) {
    let mut clk_div: c_int = crc_pwm_calc_clk_div(state.period);
    let mut pwm_output_enable: c_int = state.enabled ? PWM_OUTPUT_ENABLE : 0;
    err = regmap_write(crc_pwm.regmap, PWM0_CLK_DIV,
    clk_div | pwm_output_enable);
    if (err) {
    dev_err(dev, "Error writing PWM0_CLK_DIV %d\n", err);
    return err;
    }
    }
    if (!pwm_is_enabled(pwm) && state.enabled) {
    err = regmap_write(crc_pwm.regmap, BACKLIGHT_EN, 1);
    if (err) {
    dev_err(dev, "Error writing BACKLIGHT_EN %d\n", err);
    return err;
    }
    }
    return 0;
    }
    static int crc_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct crystalcove_pwm *crc_pwm = to_crc_pwm(chip);
    struct device *dev = pwmchip_parent(chip);
    unsigned int clk_div, clk_div_reg, duty_cycle_reg;
    int error;
    error = regmap_read(crc_pwm.regmap, PWM0_CLK_DIV, &clk_div_reg);
    if (error) {
    dev_err(dev, "Error reading PWM0_CLK_DIV %d\n", error);
    return error;
    }
    error = regmap_read(crc_pwm.regmap, PWM0_DUTY_CYCLE, &duty_cycle_reg);
    if (error) {
    dev_err(dev, "Error reading PWM0_DUTY_CYCLE %d\n", error);
    return error;
    }
    clk_div = (clk_div_reg & ~PWM_OUTPUT_ENABLE) + 1;
    state.period =
    DIV_ROUND_UP(clk_div * NSEC_PER_USEC * 256, PWM_BASE_CLK_MHZ);
    state.duty_cycle =
    DIV_ROUND_UP_ULL(duty_cycle_reg * state.period, PWM_MAX_LEVEL);
    state.polarity = PWM_POLARITY_NORMAL;
    state.enabled = !!(clk_div_reg & PWM_OUTPUT_ENABLE);
    return 0;
    }
    static const struct pwm_ops crc_pwm_ops = {
    .apply = crc_pwm_apply,
    .get_state = crc_pwm_get_state,
    };
#[no_mangle]
unsafe extern "C" fn crystalcove_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int crystalcove_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct crystalcove_pwm *crc_pwm;
    struct device *dev = pdev.dev.parent;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev);
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*crc_pwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    crc_pwm = to_crc_pwm(chip);
    chip.ops = &crc_pwm_ops;
// get the PMIC regmap
    crc_pwm.regmap = pmic.regmap;
    return devm_pwmchip_add(&pdev.dev, chip);
    }
    static struct platform_driver crystalcove_pwm_driver = {
    .probe = crystalcove_pwm_probe,
    .driver = {
    .name = "crystal_cove_pwm",
    },
    };
    module_platform_driver(crystalcove_pwm_driver);
    MODULE_ALIAS("platform:crystal_cove_pwm");
    MODULE_DESCRIPTION("Intel Crystalcove (CRC) PWM support");
    MODULE_LICENSE("GPL");
