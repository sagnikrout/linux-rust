//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-rzg2l-gpt.c
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
// Renesas RZ/G2L General PWM Timer (GPT) driver
//
// Copyright (C) 2025 Renesas Electronics Corporation
//
// Hardware manual for this IP can be found here
// https://www.renesas.com/eu/en/document/mah/rzg2l-group-rzg2lc-group-users-manual-hardware-0?language=en
//
// Limitations:
// - Counter must be stopped before modifying Mode and Prescaler.
// - When PWM is disabled, the output is driven to inactive.
// - While the hardware supports both polarities, the driver (for now)
// only handles normal polarity.
// - General PWM Timer (GPT) has 8 HW channels for PWM operations and
// each HW channel have 2 IOs.
// - Each IO is modelled as an independent PWM channel.
// - When both channels are used, disabling the channel on one stops the
// other.
// - When both channels are used, the period of both IOs in the HW channel
// must be same (for now).
//

pub const RZG2L_INIT_OUT_HI_OUT_HI_END_TOGGLE: c_uint = 0x1b;

    (RZG2L_INIT_OUT_HI_OUT_HI_END_TOGGLE | RZG2L_GTIOR_OAE)

    (FIELD_PREP(RZG2L_GTIOR_GTIOB, RZG2L_INIT_OUT_HI_OUT_HI_END_TOGGLE) | RZG2L_GTIOR_OBE)

    ((sub_ch) ? RZG2L_GTIOR_GTIOB_OUT_HI_END_TOGGLE_CMP_MATCH : \
    RZG2L_GTIOR_GTIOA_OUT_HI_END_TOGGLE_CMP_MATCH)
pub const RZG2L_MAX_HW_CHANNELS: c_int = 8;
pub const RZG2L_CHANNELS_PER_IO: c_int = 2;

pub const RZG2L_MAX_SCALE_FACTOR: c_int = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_gpt_chip {
    pub mmio: *mut void __iomem,
    pub /: *mut *mut mutex lock; / lock to protect shared channel resources,
    pub rate_khz: c_ulong,
    pub period_ticks: [u64; RZG2L_MAX_HW_CHANNELS],
    pub channel_request_count: [u32; RZG2L_MAX_HW_CHANNELS],
    pub channel_enable_count: [u32; RZG2L_MAX_HW_CHANNELS],
}

    static inline struct rzg2l_gpt_chip *to_rzg2l_gpt_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
pub unsafe extern "C" fn rzg2l_gpt_subchannel(hwpwm: c_uint) -> c_uint {
    static inline unsigned int rzg2l_gpt_subchannel(unsigned int hwpwm)
    {
    return hwpwm & 0x1;
    }
#[no_mangle]
pub unsafe extern "C" fn rzg2l_gpt_sibling(hwpwm: c_uint) -> c_uint {
    static inline unsigned int rzg2l_gpt_sibling(unsigned int hwpwm)
    {
    return hwpwm ^ 0x1;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_write(rzg2l_gpt: *mut rzg2l_gpt_chip, reg: u32, data: u32) {
    static void rzg2l_gpt_write(struct rzg2l_gpt_chip *rzg2l_gpt, u32 reg, u32 data)
    {
    writel(data, rzg2l_gpt.mmio + reg);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_read(rzg2l_gpt: *mut rzg2l_gpt_chip, reg: u32) -> u32 {
    static u32 rzg2l_gpt_read(struct rzg2l_gpt_chip *rzg2l_gpt, u32 reg)
    {
    return readl(rzg2l_gpt.mmio + reg);
    }
    static void rzg2l_gpt_modify(struct rzg2l_gpt_chip *rzg2l_gpt, u32 reg, u32 clr,
    u32 set)
    {
    rzg2l_gpt_write(rzg2l_gpt, reg,
    (rzg2l_gpt_read(rzg2l_gpt, reg) & ~clr) | set);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_calculate_prescale(period_ticks: u64) -> u8 {
    static u8 rzg2l_gpt_calculate_prescale(u64 period_ticks)
    {
    u32 prescaled_period_ticks;
    u8 prescale;
    prescaled_period_ticks = period_ticks >> 32;
    if (prescaled_period_ticks >= 256)
    prescale = 5;
    else
    prescale = (fls(prescaled_period_ticks) + 1) / 2;
    return prescale;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int rzg2l_gpt_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    let mut ch: u32 = RZG2L_GET_CH(pwm.hwpwm);
    guard(mutex)(&rzg2l_gpt.lock);
    rzg2l_gpt.channel_request_count[ch]++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void rzg2l_gpt_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    let mut ch: u32 = RZG2L_GET_CH(pwm.hwpwm);
    guard(mutex)(&rzg2l_gpt.lock);
    rzg2l_gpt.channel_request_count[ch]--;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_is_ch_enabled(rzg2l_gpt: *mut rzg2l_gpt_chip, hwpwm: u8) -> bool {
    static bool rzg2l_gpt_is_ch_enabled(struct rzg2l_gpt_chip *rzg2l_gpt, u8 hwpwm)
    {
    let mut ch: u8 = RZG2L_GET_CH(hwpwm);
    u32 val;
    val = rzg2l_gpt_read(rzg2l_gpt, RZG2L_GTCR(ch));
    if (!(val & RZG2L_GTCR_CST))
    return false;
    val = rzg2l_gpt_read(rzg2l_gpt, RZG2L_GTIOR(ch));
    return val & RZG2L_GTIOR_OxE(rzg2l_gpt_subchannel(hwpwm));
    }
// Caller holds the lock while calling rzg2l_gpt_enable()
    static void rzg2l_gpt_enable(struct rzg2l_gpt_chip *rzg2l_gpt,
    struct pwm_device *pwm)
    {
    let mut sub_ch: u8 = rzg2l_gpt_subchannel(pwm.hwpwm);
    let mut val: u32 = RZG2L_GTIOR_GTIOx(sub_ch) | RZG2L_GTIOR_OxE(sub_ch);
    let mut ch: u8 = RZG2L_GET_CH(pwm.hwpwm);
// Enable pin output
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTIOR(ch), val,
    RZG2L_GTIOR_GTIOx_OUT_HI_END_TOGGLE_CMP_MATCH(sub_ch));
    if (!rzg2l_gpt.channel_enable_count[ch])
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch), 0, RZG2L_GTCR_CST);
    rzg2l_gpt.channel_enable_count[ch]++;
    }
// Caller holds the lock while calling rzg2l_gpt_disable()
    static void rzg2l_gpt_disable(struct rzg2l_gpt_chip *rzg2l_gpt,
    struct pwm_device *pwm)
    {
    let mut sub_ch: u8 = rzg2l_gpt_subchannel(pwm.hwpwm);
    let mut ch: u8 = RZG2L_GET_CH(pwm.hwpwm);
// Stop count, Output low on GTIOCx pin when counting stops
    rzg2l_gpt.channel_enable_count[ch]--;
    if (!rzg2l_gpt.channel_enable_count[ch])
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch), RZG2L_GTCR_CST, 0);
// Disable pin output
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTIOR(ch), RZG2L_GTIOR_OxE(sub_ch), 0);
    }
    static u64 rzg2l_gpt_calculate_period_or_duty(struct rzg2l_gpt_chip *rzg2l_gpt,
    u32 val, u8 prescale)
    {
    u64 tmp;
//
// The calculation doesn't overflow an u64 because prescale ≤ 5 and so
// tmp = val << (2 * prescale) * USEC_PER_SEC
// < 2^32 * 2^10 * 10^6
// < 2^32 * 2^10 * 2^20
// = 2^62
//
    tmp = (u64)val << (2 * prescale);
    tmp *= USEC_PER_SEC;
    return DIV64_U64_ROUND_UP(tmp, rzg2l_gpt.rate_khz);
    }
    static int rzg2l_gpt_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    state.enabled = rzg2l_gpt_is_ch_enabled(rzg2l_gpt, pwm.hwpwm);
    if (state.enabled) {
    let mut sub_ch: u32 = rzg2l_gpt_subchannel(pwm.hwpwm);
    let mut ch: u32 = RZG2L_GET_CH(pwm.hwpwm);
    u8 prescale;
    u32 val;
    val = rzg2l_gpt_read(rzg2l_gpt, RZG2L_GTCR(ch));
    prescale = FIELD_GET(RZG2L_GTCR_TPCS, val);
    val = rzg2l_gpt_read(rzg2l_gpt, RZG2L_GTPR(ch));
    state.period = rzg2l_gpt_calculate_period_or_duty(rzg2l_gpt, val, prescale);
    val = rzg2l_gpt_read(rzg2l_gpt, RZG2L_GTCCR(ch, sub_ch));
    state.duty_cycle = rzg2l_gpt_calculate_period_or_duty(rzg2l_gpt, val, prescale);
    if (state.duty_cycle > state.period)
    state.duty_cycle = state.period;
    }
    state.polarity = PWM_POLARITY_NORMAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_calculate_pv_or_dc(period_or_duty_cycle: u64, prescale: u8) -> u32 {
    static u32 rzg2l_gpt_calculate_pv_or_dc(u64 period_or_duty_cycle, u8 prescale)
    {
    return min_t(u64, DIV_ROUND_DOWN_ULL(period_or_duty_cycle, 1 << (2 * prescale)),
    U32_MAX);
    }
// Caller holds the lock while calling rzg2l_gpt_config()
    static int rzg2l_gpt_config(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    let mut sub_ch: u8 = rzg2l_gpt_subchannel(pwm.hwpwm);
    let mut ch: u8 = RZG2L_GET_CH(pwm.hwpwm);
    u64 period_ticks, duty_ticks;
    unsigned long pv, dc;
    u8 prescale;
// Limit period/duty cycle to max value supported by the HW
    period_ticks = mul_u64_u64_div_u64(state.period, rzg2l_gpt.rate_khz, USEC_PER_SEC);
    if (period_ticks > RZG2L_MAX_TICKS)
    period_ticks = RZG2L_MAX_TICKS;
//
// GPT counter is shared by the two IOs of a single channel, so
// prescale and period can NOT be modified when there are multiple IOs
// in use with different settings.
//
    if (rzg2l_gpt.channel_request_count[ch] > 1) {
    let mut sibling_ch: u8 = rzg2l_gpt_sibling(pwm.hwpwm);
    if (rzg2l_gpt_is_ch_enabled(rzg2l_gpt, sibling_ch)) {
    if (period_ticks < rzg2l_gpt.period_ticks[ch])
    return -EBUSY;
    period_ticks = rzg2l_gpt.period_ticks[ch];
    }
    }
    prescale = rzg2l_gpt_calculate_prescale(period_ticks);
    pv = rzg2l_gpt_calculate_pv_or_dc(period_ticks, prescale);
    duty_ticks = mul_u64_u64_div_u64(state.duty_cycle, rzg2l_gpt.rate_khz, USEC_PER_SEC);
    if (duty_ticks > period_ticks)
    duty_ticks = period_ticks;
    dc = rzg2l_gpt_calculate_pv_or_dc(duty_ticks, prescale);
//
// GPT counter is shared by multiple channels, we cache the period ticks
// from the first enabled channel and use the same value for both
// channels.
//
    rzg2l_gpt.period_ticks[ch] = period_ticks;
//
// Counter must be stopped before modifying mode, prescaler, timer
// counter and buffer enable registers. These registers are shared
// between both channels. So allow updating these registers only for the
// first enabled channel.
//
    if (rzg2l_gpt.channel_enable_count[ch] <= 1) {
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch), RZG2L_GTCR_CST, 0);
// GPT set operating mode (saw-wave up-counting)
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch), RZG2L_GTCR_MD,
    RZG2L_GTCR_MD_SAW_WAVE_PWM_MODE);
// Set count direction
    rzg2l_gpt_write(rzg2l_gpt, RZG2L_GTUDDTYC(ch), RZG2L_GTUDDTYC_UP_COUNTING);
// Select count clock
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch), RZG2L_GTCR_TPCS,
    FIELD_PREP(RZG2L_GTCR_TPCS, prescale));
// Set period
    rzg2l_gpt_write(rzg2l_gpt, RZG2L_GTPR(ch), pv);
    }
// Set duty cycle
    rzg2l_gpt_write(rzg2l_gpt, RZG2L_GTCCR(ch, sub_ch), dc);
    if (rzg2l_gpt.channel_enable_count[ch] <= 1) {
// Set initial value for counter
    rzg2l_gpt_write(rzg2l_gpt, RZG2L_GTCNT(ch), 0);
// Set no buffer operation
    rzg2l_gpt_write(rzg2l_gpt, RZG2L_GTBER(ch), 0);
// Restart the counter after updating the registers
    rzg2l_gpt_modify(rzg2l_gpt, RZG2L_GTCR(ch),
    RZG2L_GTCR_CST, RZG2L_GTCR_CST);
    }
    return 0;
    }
    static int rzg2l_gpt_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    let mut enabled: bool = pwm.state.enabled;
    int ret;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    guard(mutex)(&rzg2l_gpt.lock);
    if (!state.enabled) {
    if (enabled)
    rzg2l_gpt_disable(rzg2l_gpt, pwm);
    return 0;
    }
    ret = rzg2l_gpt_config(chip, pwm, state);
    if (!ret && !enabled)
    rzg2l_gpt_enable(rzg2l_gpt, pwm);
    return ret;
    }
    static const struct pwm_ops rzg2l_gpt_ops = {
    .request = rzg2l_gpt_request,
    .free = rzg2l_gpt_free,
    .get_state = rzg2l_gpt_get_state,
    .apply = rzg2l_gpt_apply,
    };
#[no_mangle]
unsafe extern "C" fn rzg2l_gpt_probe(pdev: *mut platform_device) -> c_int {
    static int rzg2l_gpt_probe(struct platform_device *pdev)
    {
    struct rzg2l_gpt_chip *rzg2l_gpt;
    struct device *dev = &pdev.dev;
    struct reset_control *rstc;
    struct pwm_chip *chip;
    unsigned long rate;
    struct clk *clk;
    int ret;
    chip = devm_pwmchip_alloc(dev, RZG2L_MAX_PWM_CHANNELS, sizeof(*rzg2l_gpt));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    rzg2l_gpt = to_rzg2l_gpt_chip(chip);
    rzg2l_gpt.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rzg2l_gpt.mmio))
    return PTR_ERR(rzg2l_gpt.mmio);
    rstc = devm_reset_control_get_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc), "Cannot deassert reset control\n");
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Cannot get clock\n");
    ret = devm_clk_rate_exclusive_get(dev, clk);
    if (ret)
    return ret;
    rate = clk_get_rate(clk);
    if (!rate)
    return dev_err_probe(dev, -EINVAL, "The gpt clk rate is 0\n");
//
// Refuse clk rates > 1 GHz to prevent overflow later for computing
// period and duty cycle.
//
    if (rate > NSEC_PER_SEC)
    return dev_err_probe(dev, -EINVAL, "The gpt clk rate is > 1GHz\n");
//
// Rate is in MHz and is always integer for peripheral clk
// 2^32 * 2^10 (prescalar) * 10^6 (rate_khz) < 2^64
// So make sure rate is multiple of 1000.
//
    rzg2l_gpt.rate_khz = rate / KILO;
    if (rzg2l_gpt.rate_khz * KILO != rate)
    return dev_err_probe(dev, -EINVAL, "Rate is not multiple of 1000\n");
    mutex_init(&rzg2l_gpt.lock);
    chip.ops = &rzg2l_gpt_ops;
    ret = devm_pwmchip_add(dev, chip);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add PWM chip\n");
    return 0;
    }
    static const struct of_device_id rzg2l_gpt_of_table[] = {
    { .compatible = "renesas,rzg2l-gpt" },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzg2l_gpt_of_table);
    static struct platform_driver rzg2l_gpt_driver = {
    .driver = {
    .name = "pwm-rzg2l-gpt",
    .of_match_table = rzg2l_gpt_of_table,
    },
    .probe = rzg2l_gpt_probe,
    };
    module_platform_driver(rzg2l_gpt_driver);
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/G2L General PWM Timer (GPT) Driver");
    MODULE_LICENSE("GPL");
