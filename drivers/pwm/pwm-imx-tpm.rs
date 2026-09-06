//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-imx-tpm.c
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
// Copyright 2018-2019 NXP.
//
// Limitations:
// - The TPM counter and period counter are shared between
// multiple channels, so all channels should use same period
// settings.
// - Changes to polarity cannot be latched at the time of the
// next period start.
// - Changing period and duty cycle together isn't atomic,
// with the wrong timing it might happen that a period is
// produced with old duty cycle but new period settings.
//

pub const PWM_IMX_TPM_PARAM: c_uint = 0x4;
pub const PWM_IMX_TPM_GLOBAL: c_uint = 0x8;
pub const PWM_IMX_TPM_SC: c_uint = 0x10;
pub const PWM_IMX_TPM_CNT: c_uint = 0x14;
pub const PWM_IMX_TPM_MOD: c_uint = 0x18;

//
// The reference manual describes this field as two separate bits. The
// semantic of the two bits isn't orthogonal though, so they are treated
// together as a 2-bit field here.
//

pub const PWM_IMX_TPM_MOD_WIDTH: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_tpm_pwm_chip {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub lock: mutex,
    pub user_count: u32,
    pub enable_count: u32,
    pub real_period: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_tpm_pwm_param {
    pub prescale: u8,
    pub mod: u32,
    pub val: u32,
}

    static inline struct imx_tpm_pwm_chip *
    to_imx_tpm_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
//
// This function determines for a given pwm_state *state that a consumer
// might request the pwm_state *real_state that eventually is implemented
// by the hardware and the necessary register values (in *p) to achieve
// this.
//
    static int pwm_imx_tpm_round_state(struct pwm_chip *chip,
    struct imx_tpm_pwm_param *p,
    struct pwm_state *real_state,
    const struct pwm_state *state)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    u32 rate, prescale, period_count, clock_unit;
    u64 tmp;
    rate = clk_get_rate(tpm.clk);
    tmp = (u64)state.period * rate;
    clock_unit = DIV_ROUND_CLOSEST_ULL(tmp, NSEC_PER_SEC);
    if (clock_unit <= PWM_IMX_TPM_MOD_MOD)
    prescale = 0;
    else
    prescale = ilog2(clock_unit) + 1 - PWM_IMX_TPM_MOD_WIDTH;
    if ((!FIELD_FIT(PWM_IMX_TPM_SC_PS, prescale)))
    return -ERANGE;
    p.prescale = prescale;
    period_count = (clock_unit + ((1 << prescale) >> 1)) >> prescale;
    if (period_count == 0)
    return -EINVAL;
    p.mod = period_count - 1;
// calculate real period HW can support
    tmp = (u64)period_count << prescale;
    tmp *= NSEC_PER_SEC;
    real_state.period = DIV_ROUND_CLOSEST_ULL(tmp, rate);
//
// if eventually the PWM output is inactive, either
// duty cycle is 0 or status is disabled, need to
// make sure the output pin is inactive.
//
    if (!state.enabled)
    real_state.duty_cycle = 0;
    else
    real_state.duty_cycle = state.duty_cycle;
    tmp = (u64)p.mod * real_state.duty_cycle;
    p.val = DIV64_U64_ROUND_CLOSEST(tmp, real_state.period);
    real_state.polarity = state.polarity;
    real_state.enabled = state.enabled;
    return 0;
    }
    static int pwm_imx_tpm_get_state(struct pwm_chip *chip,
    struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    u32 rate, val, prescale;
    u64 tmp;
// get period
    state.period = tpm.real_period;
// get duty cycle
    rate = clk_get_rate(tpm.clk);
    val = readl(tpm.base + PWM_IMX_TPM_SC);
    prescale = FIELD_GET(PWM_IMX_TPM_SC_PS, val);
    tmp = readl(tpm.base + PWM_IMX_TPM_CnV(pwm.hwpwm));
    tmp = (tmp << prescale) * NSEC_PER_SEC;
    state.duty_cycle = DIV_ROUND_CLOSEST_ULL(tmp, rate);
// get polarity
    val = readl(tpm.base + PWM_IMX_TPM_CnSC(pwm.hwpwm));
    if ((val & PWM_IMX_TPM_CnSC_ELS) == PWM_IMX_TPM_CnSC_ELS_INVERSED)
    state.polarity = PWM_POLARITY_INVERSED;
    else
//
// Assume reserved values (2b00 and 2b11) to yield
// normal polarity.
//
    state.polarity = PWM_POLARITY_NORMAL;
// get channel status
    state.enabled = FIELD_GET(PWM_IMX_TPM_CnSC_ELS, val) ? true : false;
    return 0;
    }
// this function is supposed to be called with mutex hold
    static int pwm_imx_tpm_apply_hw(struct pwm_chip *chip,
    struct imx_tpm_pwm_param *p,
    struct pwm_state *state,
    struct pwm_device *pwm)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    let mut period_update: bool = false;
    let mut duty_update: bool = false;
    u32 val, cmod, cur_prescale;
    unsigned long timeout;
    struct pwm_state c;
    if (state.period != tpm.real_period) {
//
// TPM counter is shared by multiple channels, so
// prescale and period can NOT be modified when
// there are multiple channels in use with different
// period settings.
//
    if (tpm.user_count > 1)
    return -EBUSY;
    val = readl(tpm.base + PWM_IMX_TPM_SC);
    cmod = FIELD_GET(PWM_IMX_TPM_SC_CMOD, val);
    cur_prescale = FIELD_GET(PWM_IMX_TPM_SC_PS, val);
    if (cmod && cur_prescale != p.prescale)
    return -EBUSY;
// set TPM counter prescale
    val &= ~PWM_IMX_TPM_SC_PS;
    val |= FIELD_PREP(PWM_IMX_TPM_SC_PS, p.prescale);
    writel(val, tpm.base + PWM_IMX_TPM_SC);
//
// if the counter is disabled (CMOD == 0), programming the new
// period length (MOD) will not reset the counter (CNT). If
// CNT.COUNT happens to be bigger than the new MOD value then
// the counter will end up being reset way too late. Therefore,
// manually reset it to 0.
//
    if (!cmod)
    writel(0x0, tpm.base + PWM_IMX_TPM_CNT);
//
// set period count:
// if the PWM is disabled (CMOD[1:0] = 2b00), then MOD register
// is updated when MOD register is written.
//
// if the PWM is enabled (CMOD[1:0] ≠ 2b00), the period length
// is latched into hardware when the next period starts.
//
    writel(p.mod, tpm.base + PWM_IMX_TPM_MOD);
    tpm.real_period = state.period;
    period_update = true;
    }
    pwm_imx_tpm_get_state(chip, pwm, &c);
// polarity is NOT allowed to be changed if PWM is active
    if (c.enabled && c.polarity != state.polarity)
    return -EBUSY;
    if (state.duty_cycle != c.duty_cycle) {
//
// set channel value:
// if the PWM is disabled (CMOD[1:0] = 2b00), then CnV register
// is updated when CnV register is written.
//
// if the PWM is enabled (CMOD[1:0] ≠ 2b00), the duty length
// is latched into hardware when the next period starts.
//
    writel(p.val, tpm.base + PWM_IMX_TPM_CnV(pwm.hwpwm));
    duty_update = true;
    }
// make sure MOD & CnV registers are updated
    if (period_update || duty_update) {
    timeout = jiffies + msecs_to_jiffies(tpm.real_period /
    NSEC_PER_MSEC + 1);
    while (readl(tpm.base + PWM_IMX_TPM_MOD) != p.mod
    || readl(tpm.base + PWM_IMX_TPM_CnV(pwm.hwpwm))
    != p.val) {
    if (time_after(jiffies, timeout))
    return -ETIME;
    cpu_relax();
    }
    }
//
// polarity settings will enabled/disable output status
// immediately, so if the channel is disabled, need to
// make sure MSA/MSB/ELS are set to 0 which means channel
// disabled.
//
    val = readl(tpm.base + PWM_IMX_TPM_CnSC(pwm.hwpwm));
    val &= ~(PWM_IMX_TPM_CnSC_ELS | PWM_IMX_TPM_CnSC_MSA |
    PWM_IMX_TPM_CnSC_MSB);
    if (state.enabled) {
//
// set polarity (for edge-aligned PWM modes)
//
// ELS[1:0] = 2b10 yields normal polarity behaviour,
// ELS[1:0] = 2b01 yields inversed polarity.
// The other values are reserved.
//
    val |= PWM_IMX_TPM_CnSC_MSB;
    val |= (state.polarity == PWM_POLARITY_NORMAL) ?
    PWM_IMX_TPM_CnSC_ELS_NORMAL :
    PWM_IMX_TPM_CnSC_ELS_INVERSED;
    }
    writel(val, tpm.base + PWM_IMX_TPM_CnSC(pwm.hwpwm));
// control the counter status
    if (state.enabled != c.enabled) {
    val = readl(tpm.base + PWM_IMX_TPM_SC);
    if (state.enabled) {
    if (++tpm.enable_count == 1)
    val |= PWM_IMX_TPM_SC_CMOD_INC_EVERY_CLK;
    } else {
    if (--tpm.enable_count == 0)
    val &= ~PWM_IMX_TPM_SC_CMOD;
    }
    writel(val, tpm.base + PWM_IMX_TPM_SC);
    }
    return 0;
    }
    static int pwm_imx_tpm_apply(struct pwm_chip *chip,
    struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    struct imx_tpm_pwm_param param;
    struct pwm_state real_state;
    int ret;
    ret = pwm_imx_tpm_round_state(chip, &param, &real_state, state);
    if (ret)
    return ret;
    mutex_lock(&tpm.lock);
    ret = pwm_imx_tpm_apply_hw(chip, &param, &real_state, pwm);
    mutex_unlock(&tpm.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx_tpm_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int pwm_imx_tpm_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    mutex_lock(&tpm.lock);
    tpm.user_count++;
    mutex_unlock(&tpm.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx_tpm_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void pwm_imx_tpm_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct imx_tpm_pwm_chip *tpm = to_imx_tpm_pwm_chip(chip);
    mutex_lock(&tpm.lock);
    tpm.user_count--;
    mutex_unlock(&tpm.lock);
    }
    static const struct pwm_ops imx_tpm_pwm_ops = {
    .request = pwm_imx_tpm_request,
    .free = pwm_imx_tpm_free,
    .get_state = pwm_imx_tpm_get_state,
    .apply = pwm_imx_tpm_apply,
    };
#[no_mangle]
unsafe extern "C" fn pwm_imx_tpm_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_imx_tpm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct imx_tpm_pwm_chip *tpm;
    struct clk *clk;
    void __iomem *base;
    int ret;
    unsigned int i, npwm;
    u32 val;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk),
    "failed to get PWM clock\n");
// get number of channels
    val = readl(base + PWM_IMX_TPM_PARAM);
    npwm = FIELD_GET(PWM_IMX_TPM_PARAM_CHAN, val);
    chip = devm_pwmchip_alloc(&pdev.dev, npwm, sizeof(*tpm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    tpm = to_imx_tpm_pwm_chip(chip);
    platform_set_drvdata(pdev, tpm);
    tpm.base = base;
    tpm.clk = clk;
    chip.ops = &imx_tpm_pwm_ops;
    mutex_init(&tpm.lock);
// count the enabled channels
    for (i = 0; i < npwm; ++i) {
    val = readl(base + PWM_IMX_TPM_CnSC(i));
    if (FIELD_GET(PWM_IMX_TPM_CnSC_ELS, val))
    ++tpm.enable_count;
    }
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to add PWM chip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx_tpm_suspend(dev: *mut device) -> c_int {
    static int pwm_imx_tpm_suspend(struct device *dev)
    {
    struct imx_tpm_pwm_chip *tpm = dev_get_drvdata(dev);
    int ret;
    if (tpm.enable_count > 0)
    return -EBUSY;
//
// Force 'real_period' to be zero to force period update code
// can be executed after system resume back, since suspend causes
// the period related registers to become their reset values.
//
    tpm.real_period = 0;
    clk_disable_unprepare(tpm.clk);
    ret = pinctrl_pm_select_sleep_state(dev);
    if (ret)
    clk_prepare_enable(tpm.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pwm_imx_tpm_resume(dev: *mut device) -> c_int {
    static int pwm_imx_tpm_resume(struct device *dev)
    {
    struct imx_tpm_pwm_chip *tpm = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    ret = pinctrl_pm_select_default_state(dev);
    if (ret)
    return ret;
    ret = clk_prepare_enable(tpm.clk);
    if (ret) {
    dev_err(dev, "failed to prepare or enable clock: %d\n", ret);
    pinctrl_pm_select_sleep_state(dev);
    }
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(imx_tpm_pwm_pm,
    pwm_imx_tpm_suspend, pwm_imx_tpm_resume);
    static const struct of_device_id imx_tpm_pwm_dt_ids[] = {
    { .compatible = "fsl,imx7ulp-pwm" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_tpm_pwm_dt_ids);
    static struct platform_driver imx_tpm_pwm_driver = {
    .driver = {
    .name = "imx7ulp-tpm-pwm",
    .of_match_table = imx_tpm_pwm_dt_ids,
    .pm = pm_ptr(&imx_tpm_pwm_pm),
    },
    .probe	= pwm_imx_tpm_probe,
    };
    module_platform_driver(imx_tpm_pwm_driver);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("i.MX TPM PWM Driver");
    MODULE_LICENSE("GPL v2");
