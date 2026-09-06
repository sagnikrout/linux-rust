//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-apple.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Driver for the Apple SoC PWM controller
//
// Copyright The Asahi Linux Contributors
//
// Limitations:
// - The writes to cycle registers are shadowed until a write to
// the control register.
// - If both OFF_CYCLES and ON_CYCLES are set to 0, the output
// is a constant off signal.
// - When APPLE_PWM_CTRL is set to 0, the output is constant low
//

pub const APPLE_PWM_CTRL: c_uint = 0x00;
pub const APPLE_PWM_ON_CYCLES: c_uint = 0x1c;
pub const APPLE_PWM_OFF_CYCLES: c_uint = 0x18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pwm {
    pub base: *mut void __iomem,
    pub clkrate: u64,
}

    static inline struct apple_pwm *to_apple_pwm(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static int apple_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct apple_pwm *fpwm;
    if (state.polarity == PWM_POLARITY_INVERSED)
    return -EINVAL;
    fpwm = to_apple_pwm(chip);
    if (state.enabled) {
    u64 on_cycles, off_cycles;
    on_cycles = mul_u64_u64_div_u64(fpwm.clkrate,
    state.duty_cycle, NSEC_PER_SEC);
    if (on_cycles > 0xFFFFFFFF)
    on_cycles = 0xFFFFFFFF;
    off_cycles = mul_u64_u64_div_u64(fpwm.clkrate,
    state.period, NSEC_PER_SEC) - on_cycles;
    if (off_cycles > 0xFFFFFFFF)
    off_cycles = 0xFFFFFFFF;
    writel(on_cycles, fpwm.base + APPLE_PWM_ON_CYCLES);
    writel(off_cycles, fpwm.base + APPLE_PWM_OFF_CYCLES);
    writel(APPLE_PWM_CTRL_ENABLE | APPLE_PWM_CTRL_OUTPUT_ENABLE | APPLE_PWM_CTRL_UPDATE,
    fpwm.base + APPLE_PWM_CTRL);
    } else {
    writel(0, fpwm.base + APPLE_PWM_CTRL);
    }
    return 0;
    }
    static int apple_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct apple_pwm *fpwm;
    u32 on_cycles, off_cycles, ctrl;
    fpwm = to_apple_pwm(chip);
    ctrl = readl(fpwm.base + APPLE_PWM_CTRL);
    on_cycles = readl(fpwm.base + APPLE_PWM_ON_CYCLES);
    off_cycles = readl(fpwm.base + APPLE_PWM_OFF_CYCLES);
    state.enabled = (ctrl & APPLE_PWM_CTRL_ENABLE) && (ctrl & APPLE_PWM_CTRL_OUTPUT_ENABLE);
    state.polarity = PWM_POLARITY_NORMAL;
// on_cycles + off_cycles is 33 bits, NSEC_PER_SEC is 30, there is no overflow
    state.duty_cycle = DIV64_U64_ROUND_UP((u64)on_cycles * NSEC_PER_SEC, fpwm.clkrate);
    state.period = DIV64_U64_ROUND_UP(((u64)off_cycles + (u64)on_cycles) *
    NSEC_PER_SEC, fpwm.clkrate);
    return 0;
    }
    static const struct pwm_ops apple_pwm_ops = {
    .apply = apple_pwm_apply,
    .get_state = apple_pwm_get_state,
    };
#[no_mangle]
unsafe extern "C" fn apple_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int apple_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct apple_pwm *fpwm;
    struct clk *clk;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*fpwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    fpwm = to_apple_pwm(chip);
    fpwm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(fpwm.base))
    return PTR_ERR(fpwm.base);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk), "unable to get the clock");
//
// Uses the 24MHz system clock on all existing devices, can only
// happen if the device tree is broken
//
// This check is done to prevent an overflow in .apply
//
    fpwm.clkrate = clk_get_rate(clk);
    if (fpwm.clkrate > NSEC_PER_SEC)
    return dev_err_probe(&pdev.dev, -EINVAL, "pwm clock out of range");
    chip.ops = &apple_pwm_ops;
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "unable to add pwm chip");
    return 0;
    }
    static const struct of_device_id apple_pwm_of_match[] = {
    { .compatible = "apple,s5l-fpwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, apple_pwm_of_match);
    static struct platform_driver apple_pwm_driver = {
    .probe = apple_pwm_probe,
    .driver = {
    .name = "apple-pwm",
    .of_match_table = apple_pwm_of_match,
    },
    };
    module_platform_driver(apple_pwm_driver);
    MODULE_DESCRIPTION("Apple SoC PWM driver");
    MODULE_LICENSE("Dual MIT/GPL");
