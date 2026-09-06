//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-atmel.c
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
// Driver for Atmel Pulse Width Modulation Controller
//
// Copyright (C) 2013 Atmel Corporation
// Bo Shen <voice.shen@atmel.com>
//
// Links to reference manuals for the supported PWM chips can be found in
// Documentation/arch/arm/microchip.rst.
//
// Limitations:
// - Periods start with the inactive level.
// - Hardware has to be stopped in general to update settings.
//
// Software bugs/possible improvements:
// - When atmel_pwm_apply() is called with state->enabled=false a change in
// state->polarity isn't honored.
// - Instead of sleeping to wait for a completed period, the interrupt
// functionality could be used.
//

// The following is global registers for PWM controller
pub const PWM_ENA: c_uint = 0x04;
pub const PWM_DIS: c_uint = 0x08;
pub const PWM_SR: c_uint = 0x0C;
pub const PWM_ISR: c_uint = 0x1C;
// Bit field in SR
pub const PWM_SR_ALL_CH_MASK: c_uint = 0x0F;
// The following register is PWM channel related registers
pub const PWM_CH_REG_OFFSET: c_uint = 0x200;
pub const PWM_CH_REG_SIZE: c_uint = 0x20;
pub const PWM_CMR: c_uint = 0x0;
// Bit field in CMR

pub const PWM_CMR_CPRE_MSK: c_uint = 0xF;
// The following registers for PWM v1
pub const PWMV1_CDTY: c_uint = 0x04;
pub const PWMV1_CPRD: c_uint = 0x08;
pub const PWMV1_CUPD: c_uint = 0x10;
// The following registers for PWM v2
pub const PWMV2_CDTY: c_uint = 0x04;
pub const PWMV2_CDTYUPD: c_uint = 0x08;
pub const PWMV2_CPRD: c_uint = 0x0C;
pub const PWMV2_CPRDUPD: c_uint = 0x10;
pub const PWM_MAX_PRES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pwm_registers {
    pub period: u8,
    pub period_upd: u8,
    pub duty: u8,
    pub duty_upd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pwm_config {
    pub period_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pwm_data {
    pub regs: atmel_pwm_registers,
    pub cfg: atmel_pwm_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_pwm_chip {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub data: *const atmel_pwm_data,
//
// The hardware supports a mechanism to update a channel's duty cycle at
// the end of the currently running period. When such an update is
// pending we delay disabling the PWM until the new configuration is
// active because otherwise pmw_config(duty_cycle=0); pwm_disable();
// might not result in an inactive output.
// This bitmask tracks for which channels an update is pending in
// hardware.
//
    pub update_pending: u32,
}

    static inline struct atmel_pwm_chip *to_atmel_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static inline u32 atmel_pwm_readl(struct atmel_pwm_chip *chip,
    unsigned long offset)
    {
    return readl_relaxed(chip.base + offset);
    }
    static inline void atmel_pwm_writel(struct atmel_pwm_chip *chip,
    unsigned long offset, unsigned long val)
    {
    writel_relaxed(val, chip.base + offset);
    }
    static inline u32 atmel_pwm_ch_readl(struct atmel_pwm_chip *chip,
    unsigned int ch, unsigned long offset)
    {
    let mut base: c_ulong = PWM_CH_REG_OFFSET + ch * PWM_CH_REG_SIZE;
    return atmel_pwm_readl(chip, base + offset);
    }
    static inline void atmel_pwm_ch_writel(struct atmel_pwm_chip *chip,
    unsigned int ch, unsigned long offset,
    unsigned long val)
    {
    let mut base: c_ulong = PWM_CH_REG_OFFSET + ch * PWM_CH_REG_SIZE;
    atmel_pwm_writel(chip, base + offset, val);
    }
#[no_mangle]
unsafe extern "C" fn atmel_pwm_update_pending(chip: *mut atmel_pwm_chip) {
    static void atmel_pwm_update_pending(struct atmel_pwm_chip *chip)
    {
//
// Each channel that has its bit in ISR set started a new period since
// ISR was cleared and so there is no more update pending.  Note that
// reading ISR clears it, so this needs to handle all channels to not
// loose information.
//
    let mut isr: u32 = atmel_pwm_readl(chip, PWM_ISR);
    chip.update_pending &= ~isr;
    }
#[no_mangle]
unsafe extern "C" fn atmel_pwm_set_pending(chip: *mut atmel_pwm_chip, ch: c_uint) {
    static void atmel_pwm_set_pending(struct atmel_pwm_chip *chip, unsigned int ch)
    {
//
// Clear pending flags in hardware because otherwise there might still
// be a stale flag in ISR.
//
    atmel_pwm_update_pending(chip);
    chip.update_pending |= (1 << ch);
    }
#[no_mangle]
unsafe extern "C" fn atmel_pwm_test_pending(chip: *mut atmel_pwm_chip, ch: c_uint) -> c_int {
    static int atmel_pwm_test_pending(struct atmel_pwm_chip *chip, unsigned int ch)
    {
    let mut ret: c_int = 0;
    if (chip.update_pending & (1 << ch)) {
    atmel_pwm_update_pending(chip);
    if (chip.update_pending & (1 << ch))
    ret = 1;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_pwm_wait_nonpending(chip: *mut atmel_pwm_chip, ch: c_uint) -> c_int {
    static int atmel_pwm_wait_nonpending(struct atmel_pwm_chip *chip, unsigned int ch)
    {
    let mut timeout: c_ulong = jiffies + 2 * HZ;
    int ret;
    while ((ret = atmel_pwm_test_pending(chip, ch)) &&
    time_before(jiffies, timeout))
    usleep_range(10, 100);
    return ret ? -ETIMEDOUT : 0;
    }
    static int atmel_pwm_calculate_cprd_and_pres(struct pwm_chip *chip,
    unsigned long clkrate,
    const struct pwm_state *state,
    unsigned long *cprd, u32 *pres)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    let mut cycles: c_ulonglong = state.period;
    int shift;
// Calculate the period cycles and prescale value
    cycles *= clkrate;
    do_div(cycles, NSEC_PER_SEC);
//
// The register for the period length is cfg.period_bits bits wide.
// So for each bit the number of clock cycles is wider divide the input
// clock frequency by two using pres and shift cprd accordingly.
//
    shift = fls(cycles) - atmel_pwm.data.cfg.period_bits;
    if (shift > PWM_MAX_PRES) {
    dev_err(pwmchip_parent(chip), "pres exceeds the maximum value\n");
    return -EINVAL;
    } else if (shift > 0) {
// pres = shift;
    cycles >>= *pres;
    } else {
// pres = 0;
    }
// cprd = cycles;
    return 0;
    }
    static void atmel_pwm_calculate_cdty(const struct pwm_state *state,
    unsigned long clkrate, unsigned long cprd,
    u32 pres, unsigned long *cdty)
    {
    let mut cycles: c_ulonglong = state.duty_cycle;
    cycles *= clkrate;
    do_div(cycles, NSEC_PER_SEC);
    cycles >>= pres;
// cdty = cprd - cycles;
    }
    static void atmel_pwm_update_cdty(struct pwm_chip *chip, struct pwm_device *pwm,
    unsigned long cdty)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    u32 val;
    if (atmel_pwm.data.regs.duty_upd ==
    atmel_pwm.data.regs.period_upd) {
    val = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm, PWM_CMR);
    val &= ~PWM_CMR_UPD_CDTY;
    atmel_pwm_ch_writel(atmel_pwm, pwm.hwpwm, PWM_CMR, val);
    }
    atmel_pwm_ch_writel(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.duty_upd, cdty);
    atmel_pwm_set_pending(atmel_pwm, pwm.hwpwm);
    }
    static void atmel_pwm_set_cprd_cdty(struct pwm_chip *chip,
    struct pwm_device *pwm,
    unsigned long cprd, unsigned long cdty)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    atmel_pwm_ch_writel(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.duty, cdty);
    atmel_pwm_ch_writel(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.period, cprd);
    }
    static void atmel_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm,
    bool disable_clk)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    unsigned long timeout;
    atmel_pwm_wait_nonpending(atmel_pwm, pwm.hwpwm);
    atmel_pwm_writel(atmel_pwm, PWM_DIS, 1 << pwm.hwpwm);
//
// Wait for the PWM channel disable operation to be effective before
// stopping the clock.
//
    timeout = jiffies + 2 * HZ;
    while ((atmel_pwm_readl(atmel_pwm, PWM_SR) & (1 << pwm.hwpwm)) &&
    time_before(jiffies, timeout))
    usleep_range(10, 100);
    if (disable_clk)
    clk_disable(atmel_pwm.clk);
    }
    static int atmel_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    unsigned long cprd, cdty;
    u32 pres, val;
    int ret;
    if (state.enabled) {
    let mut clkrate: c_ulong = clk_get_rate(atmel_pwm.clk);
    if (pwm.state.enabled &&
    pwm.state.polarity == state.polarity &&
    pwm.state.period == state.period) {
    let mut cmr: u32 = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm, PWM_CMR);
    cprd = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.period);
    pres = cmr & PWM_CMR_CPRE_MSK;
    atmel_pwm_calculate_cdty(state, clkrate, cprd, pres, &cdty);
    atmel_pwm_update_cdty(chip, pwm, cdty);
    return 0;
    }
    ret = atmel_pwm_calculate_cprd_and_pres(chip, clkrate, state, &cprd,
    &pres);
    if (ret) {
    dev_err(pwmchip_parent(chip),
    "failed to calculate cprd and prescaler\n");
    return ret;
    }
    atmel_pwm_calculate_cdty(state, clkrate, cprd, pres, &cdty);
    if (pwm.state.enabled) {
    atmel_pwm_disable(chip, pwm, false);
    } else {
    ret = clk_enable(atmel_pwm.clk);
    if (ret) {
    dev_err(pwmchip_parent(chip), "failed to enable clock\n");
    return ret;
    }
    }
// It is necessary to preserve CPOL, inside CMR
    val = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm, PWM_CMR);
    val = (val & ~PWM_CMR_CPRE_MSK) | (pres & PWM_CMR_CPRE_MSK);
    if (state.polarity == PWM_POLARITY_NORMAL)
    val &= ~PWM_CMR_CPOL;
    else
    val |= PWM_CMR_CPOL;
    atmel_pwm_ch_writel(atmel_pwm, pwm.hwpwm, PWM_CMR, val);
    atmel_pwm_set_cprd_cdty(chip, pwm, cprd, cdty);
    atmel_pwm_writel(atmel_pwm, PWM_ENA, 1 << pwm.hwpwm);
    } else if (pwm.state.enabled) {
    atmel_pwm_disable(chip, pwm, true);
    }
    return 0;
    }
    static int atmel_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    u32 sr, cmr;
    sr = atmel_pwm_readl(atmel_pwm, PWM_SR);
    cmr = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm, PWM_CMR);
    if (sr & (1 << pwm.hwpwm)) {
    let mut rate: c_ulong = clk_get_rate(atmel_pwm.clk);
    u32 cdty, cprd, pres;
    u64 tmp;
    pres = cmr & PWM_CMR_CPRE_MSK;
    cprd = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.period);
    tmp = (u64)cprd * NSEC_PER_SEC;
    tmp <<= pres;
    state.period = DIV64_U64_ROUND_UP(tmp, rate);
// Wait for an updated duty_cycle queued in hardware
    atmel_pwm_wait_nonpending(atmel_pwm, pwm.hwpwm);
    cdty = atmel_pwm_ch_readl(atmel_pwm, pwm.hwpwm,
    atmel_pwm.data.regs.duty);
    tmp = (u64)(cprd - cdty) * NSEC_PER_SEC;
    tmp <<= pres;
    state.duty_cycle = DIV64_U64_ROUND_UP(tmp, rate);
    state.enabled = true;
    } else {
    state.enabled = false;
    }
    if (cmr & PWM_CMR_CPOL)
    state.polarity = PWM_POLARITY_INVERSED;
    else
    state.polarity = PWM_POLARITY_NORMAL;
    return 0;
    }
    static const struct pwm_ops atmel_pwm_ops = {
    .apply = atmel_pwm_apply,
    .get_state = atmel_pwm_get_state,
    };
    static const struct atmel_pwm_data atmel_sam9rl_pwm_data = {
    .regs = {
    .period		= PWMV1_CPRD,
    .period_upd	= PWMV1_CUPD,
    .duty		= PWMV1_CDTY,
    .duty_upd	= PWMV1_CUPD,
    },
    .cfg = {
// 16 bits to keep period and duty.
    .period_bits	= 16,
    },
    };
    static const struct atmel_pwm_data atmel_sama5_pwm_data = {
    .regs = {
    .period		= PWMV2_CPRD,
    .period_upd	= PWMV2_CPRDUPD,
    .duty		= PWMV2_CDTY,
    .duty_upd	= PWMV2_CDTYUPD,
    },
    .cfg = {
// 16 bits to keep period and duty.
    .period_bits	= 16,
    },
    };
    static const struct atmel_pwm_data mchp_sam9x60_pwm_data = {
    .regs = {
    .period		= PWMV1_CPRD,
    .period_upd	= PWMV1_CUPD,
    .duty		= PWMV1_CDTY,
    .duty_upd	= PWMV1_CUPD,
    },
    .cfg = {
// 32 bits to keep period and duty.
    .period_bits	= 32,
    },
    };
    static const struct of_device_id atmel_pwm_dt_ids[] = {
    {
    .compatible = "atmel,at91sam9rl-pwm",
    .data = &atmel_sam9rl_pwm_data,
    }, {
    .compatible = "atmel,sama5d3-pwm",
    .data = &atmel_sama5_pwm_data,
    }, {
    .compatible = "atmel,sama5d2-pwm",
    .data = &atmel_sama5_pwm_data,
    }, {
    .compatible = "microchip,sam9x60-pwm",
    .data = &mchp_sam9x60_pwm_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_pwm_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmel_pwm_enable_clk_if_on(chip: *mut pwm_chip, on: bool) -> c_int {
    static int atmel_pwm_enable_clk_if_on(struct pwm_chip *chip, bool on)
    {
    struct atmel_pwm_chip *atmel_pwm = to_atmel_pwm_chip(chip);
    unsigned int i, cnt = 0;
    unsigned long sr;
    let mut ret: c_int = 0;
    sr = atmel_pwm_readl(atmel_pwm, PWM_SR) & PWM_SR_ALL_CH_MASK;
    if (!sr)
    return 0;
    cnt = bitmap_weight(&sr, chip.npwm);
    if (!on)
    goto disable_clk;
    for (i = 0; i < cnt; i++) {
    ret = clk_enable(atmel_pwm.clk);
    if (ret) {
    dev_err(pwmchip_parent(chip),
    "failed to enable clock for pwm %pe\n",
    ERR_PTR(ret));
    cnt = i;
    goto disable_clk;
    }
    }
    return 0;
    disable_clk:
    while (cnt--)
    clk_disable(atmel_pwm.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_pwm_probe(struct platform_device *pdev)
    {
    struct atmel_pwm_chip *atmel_pwm;
    struct pwm_chip *chip;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 4, sizeof(*atmel_pwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    atmel_pwm = to_atmel_pwm_chip(chip);
    atmel_pwm.data = of_device_get_match_data(&pdev.dev);
    atmel_pwm.update_pending = 0;
    atmel_pwm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(atmel_pwm.base))
    return PTR_ERR(atmel_pwm.base);
    atmel_pwm.clk = devm_clk_get_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(atmel_pwm.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(atmel_pwm.clk),
    "failed to get prepared PWM clock\n");
    chip.ops = &atmel_pwm_ops;
    ret = atmel_pwm_enable_clk_if_on(chip, true);
    if (ret < 0)
    return ret;
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0) {
    dev_err_probe(&pdev.dev, ret, "failed to add PWM chip\n");
    goto disable_clk;
    }
    return 0;
    disable_clk:
    atmel_pwm_enable_clk_if_on(chip, false);
    return ret;
    }
    static struct platform_driver atmel_pwm_driver = {
    .driver = {
    .name = "atmel-pwm",
    .of_match_table = atmel_pwm_dt_ids,
    },
    .probe = atmel_pwm_probe,
    };
    module_platform_driver(atmel_pwm_driver);
    MODULE_ALIAS("platform:atmel-pwm");
    MODULE_AUTHOR("Bo Shen <voice.shen@atmel.com>");
    MODULE_DESCRIPTION("Atmel PWM driver");
    MODULE_LICENSE("GPL v2");
