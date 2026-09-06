//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-keembay.c
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
// Intel Keem Bay PWM driver
//
// Copyright (C) 2020 Intel Corporation
// Authors: Lai Poey Seng <poey.seng.lai@intel.com>
// Vineetha G. Jaya Kumaran <vineetha.g.jaya.kumaran@intel.com>
//
// Limitations:
// - Upon disabling a channel, the currently running
// period will not be completed. However, upon
// reconfiguration of the duty cycle/period, the
// currently running period will be completed first.
//

pub const KMB_TOTAL_PWM_CHANNELS: c_int = 6;

// Mask

// PWM Register offset

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keembay_pwm {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub base: *mut void __iomem,
}

    static inline struct keembay_pwm *to_keembay_pwm_dev(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
unsafe extern "C" fn keembay_clk_unprepare(data: *mut c_void) {
    static void keembay_clk_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn keembay_clk_enable(dev: *mut device, clk: *mut clk) -> c_int {
    static int keembay_clk_enable(struct device *dev, struct clk *clk)
    {
    int ret;
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, keembay_clk_unprepare, clk);
    }
//
// With gcc 10, CONFIG_CC_OPTIMIZE_FOR_SIZE and only "inline" instead of
// "__always_inline" this fails to compile because the compiler doesn't notice
// for all valid masks (e.g. KMB_PWM_LEADIN_MASK) that they are ok.
//
    static __always_inline void keembay_pwm_update_bits(struct keembay_pwm *priv, u32 mask,
    u32 val, u32 offset)
    {
    let mut buff: u32 = readl(priv.base + offset);
    buff = u32_replace_bits(buff, val, mask);
    writel(buff, priv.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn keembay_pwm_enable(priv: *mut keembay_pwm, ch: c_int) {
    static void keembay_pwm_enable(struct keembay_pwm *priv, int ch)
    {
    keembay_pwm_update_bits(priv, KMB_PWM_EN_BIT, 1,
    KMB_PWM_LEADIN_OFFSET(ch));
    }
#[no_mangle]
unsafe extern "C" fn keembay_pwm_disable(priv: *mut keembay_pwm, ch: c_int) {
    static void keembay_pwm_disable(struct keembay_pwm *priv, int ch)
    {
    keembay_pwm_update_bits(priv, KMB_PWM_EN_BIT, 0,
    KMB_PWM_LEADIN_OFFSET(ch));
    }
    static int keembay_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct keembay_pwm *priv = to_keembay_pwm_dev(chip);
    unsigned long long high, low;
    unsigned long clk_rate;
    u32 highlow;
    clk_rate = clk_get_rate(priv.clk);
// Read channel enabled status
    highlow = readl(priv.base + KMB_PWM_LEADIN_OFFSET(pwm.hwpwm));
    if (highlow & KMB_PWM_EN_BIT)
    state.enabled = true;
    else
    state.enabled = false;
// Read period and duty cycle
    highlow = readl(priv.base + KMB_PWM_HIGHLOW_OFFSET(pwm.hwpwm));
    low = FIELD_GET(KMB_PWM_LOW_MASK, highlow) * NSEC_PER_SEC;
    high = FIELD_GET(KMB_PWM_HIGH_MASK, highlow) * NSEC_PER_SEC;
    state.duty_cycle = DIV_ROUND_UP_ULL(high, clk_rate);
    state.period = DIV_ROUND_UP_ULL(high + low, clk_rate);
    state.polarity = PWM_POLARITY_NORMAL;
    return 0;
    }
    static int keembay_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct keembay_pwm *priv = to_keembay_pwm_dev(chip);
    struct pwm_state current_state;
    unsigned long long div;
    unsigned long clk_rate;
    let mut pwm_count: u32 = 0;
    u16 high, low;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
//
// Configure the pwm repeat count as infinite at (15:0) and leadin
// low time as 0 at (30:16), which is in terms of clock cycles.
//
    keembay_pwm_update_bits(priv, KMB_PWM_LEADIN_MASK, 0,
    KMB_PWM_LEADIN_OFFSET(pwm.hwpwm));
    keembay_pwm_get_state(chip, pwm, &current_state);
    if (!state.enabled) {
    if (current_state.enabled)
    keembay_pwm_disable(priv, pwm.hwpwm);
    return 0;
    }
//
// The upper 16 bits and lower 16 bits of the KMB_PWM_HIGHLOW_OFFSET
// register contain the high time and low time of waveform accordingly.
// All the values are in terms of clock cycles.
//
    clk_rate = clk_get_rate(priv.clk);
    div = clk_rate * state.duty_cycle;
    div = DIV_ROUND_DOWN_ULL(div, NSEC_PER_SEC);
    if (div > KMB_PWM_COUNT_MAX)
    return -ERANGE;
    high = div;
    div = clk_rate * state.period;
    div = DIV_ROUND_DOWN_ULL(div, NSEC_PER_SEC);
    div = div - high;
    if (div > KMB_PWM_COUNT_MAX)
    return -ERANGE;
    low = div;
    pwm_count = FIELD_PREP(KMB_PWM_HIGH_MASK, high) |
    FIELD_PREP(KMB_PWM_LOW_MASK, low);
    writel(pwm_count, priv.base + KMB_PWM_HIGHLOW_OFFSET(pwm.hwpwm));
    if (state.enabled && !current_state.enabled)
    keembay_pwm_enable(priv, pwm.hwpwm);
    return 0;
    }
    static const struct pwm_ops keembay_pwm_ops = {
    .apply = keembay_pwm_apply,
    .get_state = keembay_pwm_get_state,
    };
#[no_mangle]
unsafe extern "C" fn keembay_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int keembay_pwm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwm_chip *chip;
    struct keembay_pwm *priv;
    int ret;
    chip = devm_pwmchip_alloc(dev, KMB_TOTAL_PWM_CHANNELS, sizeof(*priv));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    priv = to_keembay_pwm_dev(chip);
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk), "Failed to get clock\n");
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    ret = keembay_clk_enable(dev, priv.clk);
    if (ret)
    return ret;
    chip.ops = &keembay_pwm_ops;
    ret = devm_pwmchip_add(dev, chip);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add PWM chip\n");
    return 0;
    }
    static const struct of_device_id keembay_pwm_of_match[] = {
    { .compatible = "intel,keembay-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, keembay_pwm_of_match);
    static struct platform_driver keembay_pwm_driver = {
    .probe	= keembay_pwm_probe,
    .driver	= {
    .name = "pwm-keembay",
    .of_match_table = keembay_pwm_of_match,
    },
    };
    module_platform_driver(keembay_pwm_driver);
    MODULE_ALIAS("platform:pwm-keembay");
    MODULE_DESCRIPTION("Intel Keem Bay PWM driver");
    MODULE_LICENSE("GPL v2");
