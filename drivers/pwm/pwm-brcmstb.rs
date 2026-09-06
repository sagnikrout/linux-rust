//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-brcmstb.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Broadcom BCM7038 PWM driver
// Author: Florian Fainelli
//
// Copyright (C) 2015 Broadcom Corporation
//

pub const PWM_CTRL: c_uint = 0x00;

pub const CTRL_CHAN_OFFS: c_int = 4;
pub const PWM_CTRL2: c_uint = 0x04;

pub const PWM_CH_SIZE: c_uint = 0x8;

// Number of bits for the CWORD value
pub const CWORD_BIT_SIZE: c_int = 16;
//
// Maximum control word value allowed when variable-frequency PWM is used as a
// clock for the constant-frequency PMW.
//
pub const CONST_VAR_F_MAX: c_int = 32768;
pub const CONST_VAR_F_MIN: c_int = 1;

pub const PWM_ON_MIN: c_int = 1;

pub const PWM_PERIOD_MIN: c_int = 0;
pub const PWM_ON_PERIOD_MAX: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_pwm {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline u32 brcmstb_pwm_readl(struct brcmstb_pwm *p,
    unsigned int offset)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    return __raw_readl(p.base + offset);
    else
    return readl_relaxed(p.base + offset);
    }
    static inline void brcmstb_pwm_writel(struct brcmstb_pwm *p, u32 value,
    unsigned int offset)
    {
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    __raw_writel(value, p.base + offset);
    else
    writel_relaxed(value, p.base + offset);
    }
    static inline struct brcmstb_pwm *to_brcmstb_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
//
// Fv is derived from the variable frequency output. The variable frequency
// output is configured using this formula:
//
// W = cword, if cword < 2 ^ 15 else 16-bit 2's complement of cword
//
// Fv = W x 2 ^ -16 x 27Mhz (reference clock)
//
// The period is: (period + 1) / Fv and "on" time is on / (period + 1)
//
// The PWM core framework specifies that the "duty_ns" parameter is in fact the
// "on" time, so this translates directly into our HW programming here.
//
    static int brcmstb_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    u64 duty_ns, u64 period_ns)
    {
    struct brcmstb_pwm *p = to_brcmstb_pwm(chip);
    unsigned long pc, dc, cword = CONST_VAR_F_MAX;
    let mut channel: c_uint = pwm.hwpwm;
    u32 value;
//
// If asking for a duty_ns equal to period_ns, we need to substract
// the period value by 1 to make it shorter than the "on" time and
// produce a flat 100% duty cycle signal, and max out the "on" time
//
    if (duty_ns == period_ns) {
    dc = PWM_ON_PERIOD_MAX;
    pc = PWM_ON_PERIOD_MAX - 1;
    goto done;
    }
    while (1) {
    u64 rate;
//
// Calculate the base rate from base frequency and current
// cword
//
    rate = (u64)clk_get_rate(p.clk) * (u64)cword;
    rate >>= CWORD_BIT_SIZE;
    pc = mul_u64_u64_div_u64(period_ns, rate, NSEC_PER_SEC);
    dc = mul_u64_u64_div_u64(duty_ns + 1, rate, NSEC_PER_SEC);
//
// We can be called with separate duty and period updates,
// so do not reject dc == 0 right away
//
    if (pc == PWM_PERIOD_MIN || (dc < PWM_ON_MIN && duty_ns))
    return -EINVAL;
// We converged on a calculation
    if (pc <= PWM_ON_PERIOD_MAX && dc <= PWM_ON_PERIOD_MAX)
    break;
//
// The cword needs to be a power of 2 for the variable
// frequency generator to output a 50% duty cycle variable
// frequency which is used as input clock to the fixed
// frequency generator.
//
    cword >>= 1;
//
// Desired periods are too large, we do not have a divider
// for them
//
    if (cword < CONST_VAR_F_MIN)
    return -EINVAL;
    }
    done:
//
// Configure the defined "cword" value to have the variable frequency
// generator output a base frequency for the constant frequency
// generator to derive from.
//
    brcmstb_pwm_writel(p, cword >> 8, PWM_CWORD_MSB(channel));
    brcmstb_pwm_writel(p, cword & 0xff, PWM_CWORD_LSB(channel));
// Select constant frequency signal output
    value = brcmstb_pwm_readl(p, PWM_CTRL2);
    value |= CTRL2_OUT_SELECT << (channel * CTRL_CHAN_OFFS);
    brcmstb_pwm_writel(p, value, PWM_CTRL2);
// Configure on and period value
    brcmstb_pwm_writel(p, pc, PWM_PERIOD(channel));
    brcmstb_pwm_writel(p, dc, PWM_ON(channel));
    return 0;
    }
    static inline void brcmstb_pwm_enable_set(struct brcmstb_pwm *p,
    unsigned int channel, bool enable)
    {
    let mut shift: c_uint = channel * CTRL_CHAN_OFFS;
    u32 value;
    value = brcmstb_pwm_readl(p, PWM_CTRL);
    if (enable) {
    value &= ~(CTRL_OEB << shift);
    value |= (CTRL_START | CTRL_OPENDRAIN) << shift;
    } else {
    value &= ~((CTRL_START | CTRL_OPENDRAIN) << shift);
    value |= CTRL_OEB << shift;
    }
    brcmstb_pwm_writel(p, value, PWM_CTRL);
    }
    static int brcmstb_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct brcmstb_pwm *p = to_brcmstb_pwm(chip);
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (pwm.state.enabled)
    brcmstb_pwm_enable_set(p, pwm.hwpwm, false);
    return 0;
    }
    err = brcmstb_pwm_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    brcmstb_pwm_enable_set(p, pwm.hwpwm, true);
    return 0;
    }
    static const struct pwm_ops brcmstb_pwm_ops = {
    .apply = brcmstb_pwm_apply,
    };
    static const struct of_device_id brcmstb_pwm_of_match[] = {
    { .compatible = "brcm,bcm7038-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, brcmstb_pwm_of_match);
#[no_mangle]
unsafe extern "C" fn brcmstb_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct brcmstb_pwm *p;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 2, sizeof(*p));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    p = to_brcmstb_pwm(chip);
    p.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(p.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(p.clk),
    "failed to obtain clock\n");
    platform_set_drvdata(pdev, p);
    chip.ops = &brcmstb_pwm_ops;
    p.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(p.base))
    return PTR_ERR(p.base);
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to add PWM chip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_pwm_suspend(dev: *mut device) -> c_int {
    static int brcmstb_pwm_suspend(struct device *dev)
    {
    struct brcmstb_pwm *p = dev_get_drvdata(dev);
    clk_disable_unprepare(p.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_pwm_resume(dev: *mut device) -> c_int {
    static int brcmstb_pwm_resume(struct device *dev)
    {
    struct brcmstb_pwm *p = dev_get_drvdata(dev);
    return clk_prepare_enable(p.clk);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(brcmstb_pwm_pm_ops, brcmstb_pwm_suspend,
    brcmstb_pwm_resume);
    static struct platform_driver brcmstb_pwm_driver = {
    .probe = brcmstb_pwm_probe,
    .driver = {
    .name = "pwm-brcmstb",
    .of_match_table = brcmstb_pwm_of_match,
    .pm = pm_ptr(&brcmstb_pwm_pm_ops),
    },
    };
    module_platform_driver(brcmstb_pwm_driver);
    MODULE_AUTHOR("Florian Fainelli <f.fainelli@gmail.com>");
    MODULE_DESCRIPTION("Broadcom STB PWM driver");
    MODULE_ALIAS("platform:pwm-brcmstb");
    MODULE_LICENSE("GPL");
