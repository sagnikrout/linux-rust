//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-sunplus.c
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
// PWM device driver for SUNPLUS SP7021 SoC
//
// Links:
// Reference Manual:
// https://sunplus-tibbo.atlassian.net/wiki/spaces/doc/overview
//
// Reference Manual(PWM module):
// https://sunplus.atlassian.net/wiki/spaces/doc/pages/461144198/12.+Pulse+Width+Modulation+PWM
//
// Limitations:
// - Only supports normal polarity.
// - It output low when PWM channel disabled.
// - When the parameters change, current running period will not be completed
// and run new settings immediately.
// - In .apply() PWM output need to write register FREQ and DUTY. When first write FREQ
// done and not yet write DUTY, it has short timing gap use new FREQ and old DUTY.
//
// Author: Hammer Hsieh <hammerh0314@gmail.com>
//

pub const SP7021_PWM_MODE0: c_uint = 0x000;

pub const SP7021_PWM_MODE1: c_uint = 0x004;

pub const SP7021_PWM_FREQ_SCALER: c_int = 256;
pub const SP7021_PWM_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunplus_pwm {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline struct sunplus_pwm *to_sunplus_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int sunplus_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct sunplus_pwm *priv = to_sunplus_pwm(chip);
    u32 dd_freq, duty, mode0, mode1;
    u64 clk_rate;
    if (state.polarity != pwm.state.polarity)
    return -EINVAL;
    if (!state.enabled) {
// disable pwm channel output
    mode0 = readl(priv.base + SP7021_PWM_MODE0);
    mode0 &= ~SP7021_PWM_MODE0_PWMEN(pwm.hwpwm);
    writel(mode0, priv.base + SP7021_PWM_MODE0);
// disable pwm channel clk source
    mode1 = readl(priv.base + SP7021_PWM_MODE1);
    mode1 &= ~SP7021_PWM_MODE1_CNT_EN(pwm.hwpwm);
    writel(mode1, priv.base + SP7021_PWM_MODE1);
    return 0;
    }
    clk_rate = clk_get_rate(priv.clk);
//
// The following calculations might overflow if clk is bigger
// than 256 GHz. In practise it's 202.5MHz, so this limitation
// is only theoretic.
//
    if (clk_rate > (u64)SP7021_PWM_FREQ_SCALER * NSEC_PER_SEC)
    return -EINVAL;
//
// With clk_rate limited above we have dd_freq <= state->period,
// so this cannot overflow.
//
    dd_freq = mul_u64_u64_div_u64(clk_rate, state.period, (u64)SP7021_PWM_FREQ_SCALER
// NSEC_PER_SEC);
    if (dd_freq == 0)
    return -EINVAL;
    if (dd_freq > SP7021_PWM_FREQ_MAX)
    dd_freq = SP7021_PWM_FREQ_MAX;
    writel(dd_freq, priv.base + SP7021_PWM_FREQ(pwm.hwpwm));
// cal and set pwm duty
    mode0 = readl(priv.base + SP7021_PWM_MODE0);
    mode0 |= SP7021_PWM_MODE0_PWMEN(pwm.hwpwm);
    mode1 = readl(priv.base + SP7021_PWM_MODE1);
    mode1 |= SP7021_PWM_MODE1_CNT_EN(pwm.hwpwm);
    if (state.duty_cycle == state.period) {
// PWM channel output = high
    mode0 |= SP7021_PWM_MODE0_BYPASS(pwm.hwpwm);
    duty = SP7021_PWM_DUTY_DD_SEL(pwm.hwpwm) | SP7021_PWM_DUTY_MAX;
    } else {
    mode0 &= ~SP7021_PWM_MODE0_BYPASS(pwm.hwpwm);
//
// duty_ns <= period_ns 27 bits, clk_rate 28 bits, won't overflow.
//
    duty = mul_u64_u64_div_u64(state.duty_cycle, clk_rate,
    (u64)dd_freq * NSEC_PER_SEC);
    duty = SP7021_PWM_DUTY_DD_SEL(pwm.hwpwm) | duty;
    }
    writel(duty, priv.base + SP7021_PWM_DUTY(pwm.hwpwm));
    writel(mode1, priv.base + SP7021_PWM_MODE1);
    writel(mode0, priv.base + SP7021_PWM_MODE0);
    return 0;
    }
    static int sunplus_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct sunplus_pwm *priv = to_sunplus_pwm(chip);
    u32 mode0, dd_freq, duty;
    u64 clk_rate;
    mode0 = readl(priv.base + SP7021_PWM_MODE0);
    if (mode0 & BIT(pwm.hwpwm)) {
    clk_rate = clk_get_rate(priv.clk);
    dd_freq = readl(priv.base + SP7021_PWM_FREQ(pwm.hwpwm));
    duty = readl(priv.base + SP7021_PWM_DUTY(pwm.hwpwm));
    duty = FIELD_GET(SP7021_PWM_DUTY_MASK, duty);
//
// dd_freq 16 bits, SP7021_PWM_FREQ_SCALER 8 bits
// NSEC_PER_SEC 30 bits, won't overflow.
//
    state.period = DIV64_U64_ROUND_UP((u64)dd_freq * (u64)SP7021_PWM_FREQ_SCALER
// NSEC_PER_SEC, clk_rate);
//
// dd_freq 16 bits, duty 8 bits, NSEC_PER_SEC 30 bits, won't overflow.
//
    state.duty_cycle = DIV64_U64_ROUND_UP((u64)dd_freq * (u64)duty * NSEC_PER_SEC,
    clk_rate);
    state.enabled = true;
    } else {
    state.enabled = false;
    }
    state.polarity = PWM_POLARITY_NORMAL;
    return 0;
    }
    static const struct pwm_ops sunplus_pwm_ops = {
    .apply = sunplus_pwm_apply,
    .get_state = sunplus_pwm_get_state,
    };
#[no_mangle]
unsafe extern "C" fn sunplus_pwm_clk_release(data: *mut c_void) {
    static void sunplus_pwm_clk_release(void *data)
    {
    struct clk *clk = data;
    clk_disable_unprepare(clk);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int sunplus_pwm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwm_chip *chip;
    struct sunplus_pwm *priv;
    int ret;
    chip = devm_pwmchip_alloc(dev, SP7021_PWM_NUM, sizeof(*priv));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    priv = to_sunplus_pwm(chip);
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "get pwm clock failed\n");
    ret = clk_prepare_enable(priv.clk);
    if (ret < 0) {
    dev_err(dev, "failed to enable clock: %d\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(dev, sunplus_pwm_clk_release, priv.clk);
    if (ret < 0) {
    dev_err(dev, "failed to release clock: %d\n", ret);
    return ret;
    }
    chip.ops = &sunplus_pwm_ops;
    ret = devm_pwmchip_add(dev, chip);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Cannot register sunplus PWM\n");
    return 0;
    }
    static const struct of_device_id sunplus_pwm_of_match[] = {
    { .compatible = "sunplus,sp7021-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sunplus_pwm_of_match);
    static struct platform_driver sunplus_pwm_driver = {
    .probe		= sunplus_pwm_probe,
    .driver		= {
    .name	= "sunplus-pwm",
    .of_match_table = sunplus_pwm_of_match,
    },
    };
    module_platform_driver(sunplus_pwm_driver);
    MODULE_DESCRIPTION("Sunplus SoC PWM Driver");
    MODULE_AUTHOR("Hammer Hsieh <hammerh0314@gmail.com>");
    MODULE_LICENSE("GPL");
