//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-imx1.c
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
// simple driver for PWM (Pulse Width Modulator) controller
//
// Derived from pxa PWM driver by eric miao <eric.miao@marvell.com>
//

pub const MX1_PWMC: c_uint = 0x00   /* PWM Control Register */;
pub const MX1_PWMS: c_uint = 0x04   /* PWM Sample Register */;
pub const MX1_PWMP: c_uint = 0x08   /* PWM Period Register */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_imx1_chip {
    pub clk_ipg: *mut clk,
    pub clk_per: *mut clk,
    pub mmio_base: *mut void __iomem,
}

    static inline struct pwm_imx1_chip *to_pwm_imx1_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx1_clk_prepare_enable(chip: *mut pwm_chip) -> c_int {
    static int pwm_imx1_clk_prepare_enable(struct pwm_chip *chip)
    {
    struct pwm_imx1_chip *imx = to_pwm_imx1_chip(chip);
    int ret;
    ret = clk_prepare_enable(imx.clk_ipg);
    if (ret)
    return ret;
    ret = clk_prepare_enable(imx.clk_per);
    if (ret) {
    clk_disable_unprepare(imx.clk_ipg);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx1_clk_disable_unprepare(chip: *mut pwm_chip) {
    static void pwm_imx1_clk_disable_unprepare(struct pwm_chip *chip)
    {
    struct pwm_imx1_chip *imx = to_pwm_imx1_chip(chip);
    clk_disable_unprepare(imx.clk_per);
    clk_disable_unprepare(imx.clk_ipg);
    }
    static int pwm_imx1_config(struct pwm_chip *chip,
    struct pwm_device *pwm, u64 duty_ns, u64 period_ns)
    {
    struct pwm_imx1_chip *imx = to_pwm_imx1_chip(chip);
    u32 max, p;
//
// The PWM subsystem allows for exact frequencies. However,
// I cannot connect a scope on my device to the PWM line and
// thus cannot provide the program the PWM controller
// exactly. Instead, I'm relying on the fact that the
// Bootloader (u-boot or WinCE+haret) has programmed the PWM
// function group already. So I'll just modify the PWM sample
// register to follow the ratio of duty_ns vs. period_ns
// accordingly.
//
// This is good enough for programming the brightness of
// the LCD backlight.
//
// The real implementation would divide PERCLK[0] first by
// both the prescaler (/1 .. /128) and then by CLKSEL
// (/2 .. /16).
//
    max = readl(imx.mmio_base + MX1_PWMP);
    p = mul_u64_u64_div_u64(max, duty_ns, period_ns);
    writel(max - p, imx.mmio_base + MX1_PWMS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx1_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int pwm_imx1_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct pwm_imx1_chip *imx = to_pwm_imx1_chip(chip);
    u32 value;
    int ret;
    ret = pwm_imx1_clk_prepare_enable(chip);
    if (ret < 0)
    return ret;
    value = readl(imx.mmio_base + MX1_PWMC);
    value |= MX1_PWMC_EN;
    writel(value, imx.mmio_base + MX1_PWMC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx1_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void pwm_imx1_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct pwm_imx1_chip *imx = to_pwm_imx1_chip(chip);
    u32 value;
    value = readl(imx.mmio_base + MX1_PWMC);
    value &= ~MX1_PWMC_EN;
    writel(value, imx.mmio_base + MX1_PWMC);
    pwm_imx1_clk_disable_unprepare(chip);
    }
    static int pwm_imx1_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (pwm.state.enabled)
    pwm_imx1_disable(chip, pwm);
    return 0;
    }
    err = pwm_imx1_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    return pwm_imx1_enable(chip, pwm);
    return 0;
    }
    static const struct pwm_ops pwm_imx1_ops = {
    .apply = pwm_imx1_apply,
    };
    static const struct of_device_id pwm_imx1_dt_ids[] = {
    { .compatible = "fsl,imx1-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pwm_imx1_dt_ids);
#[no_mangle]
unsafe extern "C" fn pwm_imx1_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_imx1_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct pwm_imx1_chip *imx;
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*imx));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    imx = to_pwm_imx1_chip(chip);
    imx.clk_ipg = devm_clk_get(&pdev.dev, "ipg");
    if (IS_ERR(imx.clk_ipg))
    return dev_err_probe(&pdev.dev, PTR_ERR(imx.clk_ipg),
    "getting ipg clock failed\n");
    imx.clk_per = devm_clk_get(&pdev.dev, "per");
    if (IS_ERR(imx.clk_per))
    return dev_err_probe(&pdev.dev, PTR_ERR(imx.clk_per),
    "failed to get peripheral clock\n");
    chip.ops = &pwm_imx1_ops;
    imx.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imx.mmio_base))
    return PTR_ERR(imx.mmio_base);
    return devm_pwmchip_add(&pdev.dev, chip);
    }
    static struct platform_driver pwm_imx1_driver = {
    .driver = {
    .name = "pwm-imx1",
    .of_match_table = pwm_imx1_dt_ids,
    },
    .probe = pwm_imx1_probe,
    };
    module_platform_driver(pwm_imx1_driver);
    MODULE_DESCRIPTION("i.MX1 and i.MX21 Pulse Width Modulator driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
