//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-tiecap.c
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
// ECAP PWM driver
//
// Copyright (C) 2012 Texas Instruments, Inc. - https://www.ti.com
//
// Hardware properties:
// - On disable the PWM pin becomes an input, so the behaviour depends on
// external wiring.
//

// ECAP registers and bits definitions
pub const CAP1: c_uint = 0x08;
pub const CAP2: c_uint = 0x0C;
pub const CAP3: c_uint = 0x10;
pub const CAP4: c_uint = 0x14;
pub const ECCTL2: c_uint = 0x2A;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecap_context {
    pub cap3: u32,
    pub cap4: u32,
    pub ecctl2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecap_pwm_chip {
    pub clk_rate: c_uint,
    pub mmio_base: *mut void __iomem,
    pub ctx: ecap_context,
}

    static inline struct ecap_pwm_chip *to_ecap_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
//
// period_ns = 10^9 * period_cycles / PWM_CLK_RATE
// duty_ns   = 10^9 * duty_cycles / PWM_CLK_RATE
//
    static int ecap_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns, int enabled)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    u32 period_cycles, duty_cycles;
    unsigned long long c;
    u16 value;
    c = pc.clk_rate;
    c = c * period_ns;
    do_div(c, NSEC_PER_SEC);
    period_cycles = (u32)c;
    if (period_cycles < 1) {
    period_cycles = 1;
    duty_cycles = 1;
    } else {
    c = pc.clk_rate;
    c = c * duty_ns;
    do_div(c, NSEC_PER_SEC);
    duty_cycles = (u32)c;
    }
    pm_runtime_get_sync(pwmchip_parent(chip));
    value = readw(pc.mmio_base + ECCTL2);
// Configure APWM mode & disable sync option
    value |= ECCTL2_APWM_MODE | ECCTL2_SYNC_SEL_DISA;
    writew(value, pc.mmio_base + ECCTL2);
    if (!enabled) {
// Update active registers if not running
    writel(duty_cycles, pc.mmio_base + CAP2);
    writel(period_cycles, pc.mmio_base + CAP1);
    } else {
//
// Update shadow registers to configure period and
// compare values. This helps current PWM period to
// complete on reconfiguring
//
    writel(duty_cycles, pc.mmio_base + CAP4);
    writel(period_cycles, pc.mmio_base + CAP3);
    }
    if (!enabled) {
    value = readw(pc.mmio_base + ECCTL2);
// Disable APWM mode to put APWM output Low
    value &= ~ECCTL2_APWM_MODE;
    writew(value, pc.mmio_base + ECCTL2);
    }
    pm_runtime_put_sync(pwmchip_parent(chip));
    return 0;
    }
    static int ecap_pwm_set_polarity(struct pwm_chip *chip, struct pwm_device *pwm,
    enum pwm_polarity polarity)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    u16 value;
    pm_runtime_get_sync(pwmchip_parent(chip));
    value = readw(pc.mmio_base + ECCTL2);
    if (polarity == PWM_POLARITY_INVERSED)
// Duty cycle defines LOW period of PWM
    value |= ECCTL2_APWM_POL_LOW;
    else
// Duty cycle defines HIGH period of PWM
    value &= ~ECCTL2_APWM_POL_LOW;
    writew(value, pc.mmio_base + ECCTL2);
    pm_runtime_put_sync(pwmchip_parent(chip));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int ecap_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    u16 value;
// Leave clock enabled on enabling PWM
    pm_runtime_get_sync(pwmchip_parent(chip));
//
// Enable 'Free run Time stamp counter mode' to start counter
// and  'APWM mode' to enable APWM output
//
    value = readw(pc.mmio_base + ECCTL2);
    value |= ECCTL2_TSCTR_FREERUN | ECCTL2_APWM_MODE;
    writew(value, pc.mmio_base + ECCTL2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void ecap_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    u16 value;
//
// Disable 'Free run Time stamp counter mode' to stop counter
// and 'APWM mode' to put APWM output to low
//
    value = readw(pc.mmio_base + ECCTL2);
    value &= ~(ECCTL2_TSCTR_FREERUN | ECCTL2_APWM_MODE);
    writew(value, pc.mmio_base + ECCTL2);
// Disable clock on PWM disable
    pm_runtime_put_sync(pwmchip_parent(chip));
    }
    static int ecap_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    let mut enabled: c_int = pwm.state.enabled;
    if (state.polarity != pwm.state.polarity) {
    if (enabled) {
    ecap_pwm_disable(chip, pwm);
    enabled = false;
    }
    err = ecap_pwm_set_polarity(chip, pwm, state.polarity);
    if (err)
    return err;
    }
    if (!state.enabled) {
    if (enabled)
    ecap_pwm_disable(chip, pwm);
    return 0;
    }
    if (state.period > NSEC_PER_SEC)
    return -ERANGE;
    err = ecap_pwm_config(chip, pwm, state.duty_cycle,
    state.period, enabled);
    if (err)
    return err;
    if (!enabled)
    return ecap_pwm_enable(chip, pwm);
    return 0;
    }
    static const struct pwm_ops ecap_pwm_ops = {
    .apply = ecap_pwm_apply,
    };
    static const struct of_device_id ecap_of_match[] = {
    { .compatible = "ti,am3352-ecap" },
    { .compatible = "ti,am33xx-ecap" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ecap_of_match);
#[no_mangle]
unsafe extern "C" fn ecap_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int ecap_pwm_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct ecap_pwm_chip *pc;
    struct pwm_chip *chip;
    struct clk *clk;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*pc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pc = to_ecap_pwm_chip(chip);
    clk = devm_clk_get(&pdev.dev, "fck");
    if (IS_ERR(clk)) {
    if (of_device_is_compatible(np, "ti,am33xx-ecap")) {
    dev_warn(&pdev.dev, "Binding is obsolete.\n");
    clk = devm_clk_get(pdev.dev.parent, "fck");
    }
    }
    if (IS_ERR(clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    return PTR_ERR(clk);
    }
    pc.clk_rate = clk_get_rate(clk);
    if (!pc.clk_rate) {
    dev_err(&pdev.dev, "failed to get clock rate\n");
    return -EINVAL;
    }
    chip.ops = &ecap_pwm_ops;
    pc.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pc.mmio_base))
    return PTR_ERR(pc.mmio_base);
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0) {
    dev_err(&pdev.dev, "pwmchip_add() failed: %d\n", ret);
    return ret;
    }
    platform_set_drvdata(pdev, chip);
    pm_runtime_enable(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_remove(pdev: *mut platform_device) {
    static void ecap_pwm_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_save_context(chip: *mut pwm_chip) {
    static void ecap_pwm_save_context(struct pwm_chip *chip)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    pm_runtime_get_sync(pwmchip_parent(chip));
    pc.ctx.ecctl2 = readw(pc.mmio_base + ECCTL2);
    pc.ctx.cap4 = readl(pc.mmio_base + CAP4);
    pc.ctx.cap3 = readl(pc.mmio_base + CAP3);
    pm_runtime_put_sync(pwmchip_parent(chip));
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_restore_context(chip: *mut pwm_chip) {
    static void ecap_pwm_restore_context(struct pwm_chip *chip)
    {
    struct ecap_pwm_chip *pc = to_ecap_pwm_chip(chip);
    writel(pc.ctx.cap3, pc.mmio_base + CAP3);
    writel(pc.ctx.cap4, pc.mmio_base + CAP4);
    writew(pc.ctx.ecctl2, pc.mmio_base + ECCTL2);
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_suspend(dev: *mut device) -> c_int {
    static int ecap_pwm_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct pwm_device *pwm = chip.pwms;
    ecap_pwm_save_context(chip);
// Disable explicitly if PWM is running
    if (pwm_is_enabled(pwm))
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecap_pwm_resume(dev: *mut device) -> c_int {
    static int ecap_pwm_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct pwm_device *pwm = chip.pwms;
// Enable explicitly if PWM was running
    if (pwm_is_enabled(pwm))
    pm_runtime_get_sync(dev);
    ecap_pwm_restore_context(chip);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ecap_pwm_pm_ops, ecap_pwm_suspend, ecap_pwm_resume);
    static struct platform_driver ecap_pwm_driver = {
    .driver = {
    .name = "ecap",
    .of_match_table = ecap_of_match,
    .pm = pm_ptr(&ecap_pwm_pm_ops),
    },
    .probe = ecap_pwm_probe,
    .remove = ecap_pwm_remove,
    };
    module_platform_driver(ecap_pwm_driver);
    MODULE_DESCRIPTION("ECAP PWM driver");
    MODULE_AUTHOR("Texas Instruments");
    MODULE_LICENSE("GPL");
