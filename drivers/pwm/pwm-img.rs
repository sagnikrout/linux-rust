//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-img.c
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
// Imagination Technologies Pulse Width Modulator driver
//
// Copyright (c) 2014-2015, Imagination Technologies
//
// Based on drivers/pwm/pwm-tegra.c, Copyright (c) 2010, NVIDIA Corporation
//

// PWM registers
pub const PWM_CTRL_CFG: c_uint = 0x0000;
pub const PWM_CTRL_CFG_NO_SUB_DIV: c_int = 0;
pub const PWM_CTRL_CFG_SUB_DIV0: c_int = 1;
pub const PWM_CTRL_CFG_SUB_DIV1: c_int = 2;
pub const PWM_CTRL_CFG_SUB_DIV0_DIV1: c_int = 3;

pub const PWM_CTRL_CFG_DIV_MASK: c_uint = 0x3;

pub const PWM_CH_CFG_TMBASE_SHIFT: c_int = 0;
pub const PWM_CH_CFG_DUTY_SHIFT: c_int = 16;
pub const PERIP_PWM_PDM_CONTROL: c_uint = 0x0140;
pub const PERIP_PWM_PDM_CONTROL_CH_MASK: c_uint = 0x1;

//
// PWM period is specified with a timebase register,
// in number of step periods. The PWM duty cycle is also
// specified in step periods, in the [0, $timebase] range.
// In other words, the timebase imposes the duty cycle
// resolution. Therefore, let's constraint the timebase to
// a minimum value to allow a sane range of duty cycle values.
// Imposing a minimum timebase, will impose a maximum PWM frequency.
//
// The value chosen is completely arbitrary.
//
pub const MIN_TMBASE_STEPS: c_int = 16;
pub const IMG_PWM_NPWM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_pwm_soc_data {
    pub max_timebase: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_pwm_chip {
    pub pwm_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub base: *mut void __iomem,
    pub periph_regs: *mut regmap,
    pub max_period_ns: c_int,
    pub min_period_ns: c_int,
    pub data: *const img_pwm_soc_data,
    pub suspend_ctrl_cfg: u32,
    pub suspend_ch_cfg: [u32; IMG_PWM_NPWM],
}

    static inline struct img_pwm_chip *to_img_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static inline void img_pwm_writel(struct img_pwm_chip *imgchip,
    u32 reg, u32 val)
    {
    writel(val, imgchip.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn img_pwm_readl(imgchip: *mut img_pwm_chip, reg: u32) -> u32 {
    static inline u32 img_pwm_readl(struct img_pwm_chip *imgchip, u32 reg)
    {
    return readl(imgchip.base + reg);
    }
    static int img_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns)
    {
    u32 val, div, duty, timebase;
    unsigned long mul, output_clk_hz, input_clk_hz;
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    let mut max_timebase: c_uint = imgchip.data.max_timebase;
    int ret;
    if (period_ns < imgchip.min_period_ns ||
    period_ns > imgchip.max_period_ns) {
    dev_err(pwmchip_parent(chip), "configured period not in range\n");
    return -ERANGE;
    }
    input_clk_hz = clk_get_rate(imgchip.pwm_clk);
    output_clk_hz = DIV_ROUND_UP(NSEC_PER_SEC, period_ns);
    mul = DIV_ROUND_UP(input_clk_hz, output_clk_hz);
    if (mul <= max_timebase) {
    div = PWM_CTRL_CFG_NO_SUB_DIV;
    timebase = DIV_ROUND_UP(mul, 1);
    } else if (mul <= max_timebase * 8) {
    div = PWM_CTRL_CFG_SUB_DIV0;
    timebase = DIV_ROUND_UP(mul, 8);
    } else if (mul <= max_timebase * 64) {
    div = PWM_CTRL_CFG_SUB_DIV1;
    timebase = DIV_ROUND_UP(mul, 64);
    } else if (mul <= max_timebase * 512) {
    div = PWM_CTRL_CFG_SUB_DIV0_DIV1;
    timebase = DIV_ROUND_UP(mul, 512);
    } else {
    dev_err(pwmchip_parent(chip),
    "failed to configure timebase steps/divider value\n");
    return -EINVAL;
    }
    duty = DIV_ROUND_UP(timebase * duty_ns, period_ns);
    ret = pm_runtime_resume_and_get(pwmchip_parent(chip));
    if (ret < 0)
    return ret;
    val = img_pwm_readl(imgchip, PWM_CTRL_CFG);
    val &= ~(PWM_CTRL_CFG_DIV_MASK << PWM_CTRL_CFG_DIV_SHIFT(pwm.hwpwm));
    val |= (div & PWM_CTRL_CFG_DIV_MASK) <<
    PWM_CTRL_CFG_DIV_SHIFT(pwm.hwpwm);
    img_pwm_writel(imgchip, PWM_CTRL_CFG, val);
    val = (duty << PWM_CH_CFG_DUTY_SHIFT) |
    (timebase << PWM_CH_CFG_TMBASE_SHIFT);
    img_pwm_writel(imgchip, PWM_CH_CFG(pwm.hwpwm), val);
    pm_runtime_put_autosuspend(pwmchip_parent(chip));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int img_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    u32 val;
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    int ret;
    ret = pm_runtime_resume_and_get(pwmchip_parent(chip));
    if (ret < 0)
    return ret;
    val = img_pwm_readl(imgchip, PWM_CTRL_CFG);
    val |= BIT(pwm.hwpwm);
    img_pwm_writel(imgchip, PWM_CTRL_CFG, val);
    regmap_clear_bits(imgchip.periph_regs, PERIP_PWM_PDM_CONTROL,
    PERIP_PWM_PDM_CONTROL_CH_MASK <<
    PERIP_PWM_PDM_CONTROL_CH_SHIFT(pwm.hwpwm));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void img_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    u32 val;
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    val = img_pwm_readl(imgchip, PWM_CTRL_CFG);
    val &= ~BIT(pwm.hwpwm);
    img_pwm_writel(imgchip, PWM_CTRL_CFG, val);
    pm_runtime_put_autosuspend(pwmchip_parent(chip));
    }
    static int img_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (pwm.state.enabled)
    img_pwm_disable(chip, pwm);
    return 0;
    }
    err = img_pwm_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    err = img_pwm_enable(chip, pwm);
    return err;
    }
    static const struct pwm_ops img_pwm_ops = {
    .apply = img_pwm_apply,
    };
    static const struct img_pwm_soc_data pistachio_pwm = {
    .max_timebase = 255,
    };
    static const struct of_device_id img_pwm_of_match[] = {
    {
    .compatible = "img,pistachio-pwm",
    .data = &pistachio_pwm,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, img_pwm_of_match);
#[no_mangle]
unsafe extern "C" fn img_pwm_runtime_suspend(dev: *mut device) -> c_int {
    static int img_pwm_runtime_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    clk_disable_unprepare(imgchip.pwm_clk);
    clk_disable_unprepare(imgchip.sys_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_runtime_resume(dev: *mut device) -> c_int {
    static int img_pwm_runtime_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    int ret;
    ret = clk_prepare_enable(imgchip.sys_clk);
    if (ret < 0) {
    dev_err(dev, "could not prepare or enable sys clock\n");
    return ret;
    }
    ret = clk_prepare_enable(imgchip.pwm_clk);
    if (ret < 0) {
    dev_err(dev, "could not prepare or enable pwm clock\n");
    clk_disable_unprepare(imgchip.sys_clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int img_pwm_probe(struct platform_device *pdev)
    {
    int ret;
    u64 val;
    unsigned long clk_rate;
    struct pwm_chip *chip;
    struct img_pwm_chip *imgchip;
    chip = devm_pwmchip_alloc(&pdev.dev, IMG_PWM_NPWM, sizeof(*imgchip));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    imgchip = to_img_pwm_chip(chip);
    imgchip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imgchip.base))
    return PTR_ERR(imgchip.base);
    imgchip.data = device_get_match_data(&pdev.dev);
    imgchip.periph_regs = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "img,cr-periph");
    if (IS_ERR(imgchip.periph_regs))
    return PTR_ERR(imgchip.periph_regs);
    imgchip.sys_clk = devm_clk_get(&pdev.dev, "sys");
    if (IS_ERR(imgchip.sys_clk)) {
    dev_err(&pdev.dev, "failed to get system clock\n");
    return PTR_ERR(imgchip.sys_clk);
    }
    imgchip.pwm_clk = devm_clk_get(&pdev.dev, "pwm");
    if (IS_ERR(imgchip.pwm_clk)) {
    dev_err(&pdev.dev, "failed to get pwm clock\n");
    return PTR_ERR(imgchip.pwm_clk);
    }
    platform_set_drvdata(pdev, chip);
    pm_runtime_set_autosuspend_delay(&pdev.dev, IMG_PWM_PM_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = img_pwm_runtime_resume(&pdev.dev);
    if (ret)
    goto err_pm_disable;
    }
    clk_rate = clk_get_rate(imgchip.pwm_clk);
    if (!clk_rate) {
    dev_err(&pdev.dev, "imgchip clock has no frequency\n");
    ret = -EINVAL;
    goto err_suspend;
    }
// The maximum input clock divider is 512
    val = (u64)NSEC_PER_SEC * 512 * imgchip.data.max_timebase;
    do_div(val, clk_rate);
    imgchip.max_period_ns = val;
    val = (u64)NSEC_PER_SEC * MIN_TMBASE_STEPS;
    do_div(val, clk_rate);
    imgchip.min_period_ns = val;
    chip.ops = &img_pwm_ops;
    ret = pwmchip_add(chip);
    if (ret < 0) {
    dev_err(&pdev.dev, "pwmchip_add failed: %d\n", ret);
    goto err_suspend;
    }
    return 0;
    err_suspend:
    if (!pm_runtime_enabled(&pdev.dev))
    img_pwm_runtime_suspend(&pdev.dev);
    err_pm_disable:
    pm_runtime_disable(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_remove(pdev: *mut platform_device) {
    static void img_pwm_remove(struct platform_device *pdev)
    {
    struct pwm_chip *chip = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    img_pwm_runtime_suspend(&pdev.dev);
    pwmchip_remove(chip);
    }

#[no_mangle]
unsafe extern "C" fn img_pwm_suspend(dev: *mut device) -> c_int {
    static int img_pwm_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    int i, ret;
    if (pm_runtime_status_suspended(dev)) {
    ret = img_pwm_runtime_resume(dev);
    if (ret)
    return ret;
    }
    for (i = 0; i < chip.npwm; i++)
    imgchip.suspend_ch_cfg[i] = img_pwm_readl(imgchip,
    PWM_CH_CFG(i));
    imgchip.suspend_ctrl_cfg = img_pwm_readl(imgchip, PWM_CTRL_CFG);
    img_pwm_runtime_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_pwm_resume(dev: *mut device) -> c_int {
    static int img_pwm_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct img_pwm_chip *imgchip = to_img_pwm_chip(chip);
    int ret;
    int i;
    ret = img_pwm_runtime_resume(dev);
    if (ret)
    return ret;
    for (i = 0; i < chip.npwm; i++)
    img_pwm_writel(imgchip, PWM_CH_CFG(i),
    imgchip.suspend_ch_cfg[i]);
    img_pwm_writel(imgchip, PWM_CTRL_CFG, imgchip.suspend_ctrl_cfg);
    for (i = 0; i < chip.npwm; i++)
    if (imgchip.suspend_ctrl_cfg & BIT(i))
    regmap_clear_bits(imgchip.periph_regs,
    PERIP_PWM_PDM_CONTROL,
    PERIP_PWM_PDM_CONTROL_CH_MASK <<
    PERIP_PWM_PDM_CONTROL_CH_SHIFT(i));
    if (pm_runtime_status_suspended(dev))
    img_pwm_runtime_suspend(dev);
    return 0;
    }

    static const struct dev_pm_ops img_pwm_pm_ops = {
    SET_RUNTIME_PM_OPS(img_pwm_runtime_suspend,
    img_pwm_runtime_resume,
    core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(img_pwm_suspend, img_pwm_resume)
    };
    static struct platform_driver img_pwm_driver = {
    .driver = {
    .name = "img-pwm",
    .pm = &img_pwm_pm_ops,
    .of_match_table = img_pwm_of_match,
    },
    .probe = img_pwm_probe,
    .remove = img_pwm_remove,
    };
    module_platform_driver(img_pwm_driver);
    MODULE_AUTHOR("Sai Masarapu <Sai.Masarapu@imgtec.com>");
    MODULE_DESCRIPTION("Imagination Technologies PWM DAC driver");
    MODULE_LICENSE("GPL v2");
