//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-spear.c
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
// ST Microelectronics SPEAr Pulse Width Modulator driver
//
// Copyright (C) 2012 ST Microelectronics
// Shiraz Hashim <shiraz.linux.kernel@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const NUM_PWM: c_int = 4;
// PWM registers and bits definitions
pub const PWMCR: c_uint = 0x00	/* Control Register */;
pub const PWMCR_PWM_ENABLE: c_uint = 0x1;
pub const PWMCR_PRESCALE_SHIFT: c_int = 2;
pub const PWMCR_MIN_PRESCALE: c_uint = 0x00;
pub const PWMCR_MAX_PRESCALE: c_uint = 0x3FFF;
pub const PWMDCR: c_uint = 0x04	/* Duty Cycle Register */;
pub const PWMDCR_MIN_DUTY: c_uint = 0x0001;
pub const PWMDCR_MAX_DUTY: c_uint = 0xFFFF;
pub const PWMPCR: c_uint = 0x08	/* Period Register */;
pub const PWMPCR_MIN_PERIOD: c_uint = 0x0001;
pub const PWMPCR_MAX_PERIOD: c_uint = 0xFFFF;
// Following only available on 13xx SoCs
pub const PWMMCR: c_uint = 0x3C	/* Master Control Register */;
pub const PWMMCR_PWM_ENABLE: c_uint = 0x1;
//
// struct spear_pwm_chip - struct representing pwm chip
//
// @mmio_base: base address of pwm chip
// @clk: pointer to clk structure of pwm chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_pwm_chip {
    pub mmio_base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline struct spear_pwm_chip *to_spear_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static inline u32 spear_pwm_readl(struct spear_pwm_chip *chip, unsigned int num,
    unsigned long offset)
    {
    return readl_relaxed(chip.mmio_base + (num << 4) + offset);
    }
    static inline void spear_pwm_writel(struct spear_pwm_chip *chip,
    unsigned int num, unsigned long offset,
    unsigned long val)
    {
    writel_relaxed(val, chip.mmio_base + (num << 4) + offset);
    }
    static int spear_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    u64 duty_ns, u64 period_ns)
    {
    struct spear_pwm_chip *pc = to_spear_pwm_chip(chip);
    u64 val, div, clk_rate;
    let mut prescale: c_ulong = PWMCR_MIN_PRESCALE, pv, dc;
    int ret;
//
// Find pv, dc and prescale to suit duty_ns and period_ns. This is done
// according to formulas described below:
//
// period_ns = 10^9 * (PRESCALE + 1) * PV / PWM_CLK_RATE
// duty_ns = 10^9 * (PRESCALE + 1) * DC / PWM_CLK_RATE
//
// PV = (PWM_CLK_RATE * period_ns) / (10^9 * (PRESCALE + 1))
// DC = (PWM_CLK_RATE * duty_ns) / (10^9 * (PRESCALE + 1))
//
    clk_rate = clk_get_rate(pc.clk);
    while (1) {
    div = 1000000000;
    div *= 1 + prescale;
    val = clk_rate * period_ns;
    pv = div64_u64(val, div);
    val = clk_rate * duty_ns;
    dc = div64_u64(val, div);
// if duty_ns and period_ns are not achievable then return
    if (pv < PWMPCR_MIN_PERIOD || dc < PWMDCR_MIN_DUTY)
    return -EINVAL;
//
// if pv and dc have crossed their upper limit, then increase
// prescale and recalculate pv and dc.
//
    if (pv > PWMPCR_MAX_PERIOD || dc > PWMDCR_MAX_DUTY) {
    if (++prescale > PWMCR_MAX_PRESCALE)
    return -EINVAL;
    continue;
    }
    break;
    }
//
// NOTE: the clock to PWM has to be enabled first before writing to the
// registers.
//
    ret = clk_enable(pc.clk);
    if (ret)
    return ret;
    spear_pwm_writel(pc, pwm.hwpwm, PWMCR,
    prescale << PWMCR_PRESCALE_SHIFT);
    spear_pwm_writel(pc, pwm.hwpwm, PWMDCR, dc);
    spear_pwm_writel(pc, pwm.hwpwm, PWMPCR, pv);
    clk_disable(pc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int spear_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct spear_pwm_chip *pc = to_spear_pwm_chip(chip);
    let mut rc: c_int = 0;
    u32 val;
    rc = clk_enable(pc.clk);
    if (rc)
    return rc;
    val = spear_pwm_readl(pc, pwm.hwpwm, PWMCR);
    val |= PWMCR_PWM_ENABLE;
    spear_pwm_writel(pc, pwm.hwpwm, PWMCR, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_pwm_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void spear_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct spear_pwm_chip *pc = to_spear_pwm_chip(chip);
    u32 val;
    val = spear_pwm_readl(pc, pwm.hwpwm, PWMCR);
    val &= ~PWMCR_PWM_ENABLE;
    spear_pwm_writel(pc, pwm.hwpwm, PWMCR, val);
    clk_disable(pc.clk);
    }
    static int spear_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (pwm.state.enabled)
    spear_pwm_disable(chip, pwm);
    return 0;
    }
    err = spear_pwm_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    return spear_pwm_enable(chip, pwm);
    return 0;
    }
    static const struct pwm_ops spear_pwm_ops = {
    .apply = spear_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn spear_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int spear_pwm_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct pwm_chip *chip;
    struct spear_pwm_chip *pc;
    int ret;
    u32 val;
    chip = devm_pwmchip_alloc(&pdev.dev, NUM_PWM, sizeof(*pc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pc = to_spear_pwm_chip(chip);
    pc.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pc.mmio_base))
    return PTR_ERR(pc.mmio_base);
    pc.clk = devm_clk_get_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(pc.clk),
    "Failed to get clock\n");
    chip.ops = &spear_pwm_ops;
    if (of_device_is_compatible(np, "st,spear1340-pwm")) {
    ret = clk_enable(pc.clk);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to enable clk\n");
//
// Following enables PWM chip, channels would still be
// enabled individually through their control register
//
    val = readl_relaxed(pc.mmio_base + PWMMCR);
    val |= PWMMCR_PWM_ENABLE;
    writel_relaxed(val, pc.mmio_base + PWMMCR);
    clk_disable(pc.clk);
    }
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "pwmchip_add() failed\n");
    return 0;
    }
    static const struct of_device_id spear_pwm_of_match[] = {
    { .compatible = "st,spear320-pwm" },
    { .compatible = "st,spear1340-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, spear_pwm_of_match);
    static struct platform_driver spear_pwm_driver = {
    .driver = {
    .name = "spear-pwm",
    .of_match_table = spear_pwm_of_match,
    },
    .probe = spear_pwm_probe,
    };
    module_platform_driver(spear_pwm_driver);
    MODULE_DESCRIPTION("ST Microelectronics SPEAr Pulse Width Modulator driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Shiraz Hashim <shiraz.linux.kernel@gmail.com>");
    MODULE_AUTHOR("Viresh Kumar <viresh.kumar@linaro.com>");
    MODULE_ALIAS("platform:spear-pwm");
