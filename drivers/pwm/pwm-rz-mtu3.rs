//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-rz-mtu3.c
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
// Renesas RZ/G2L MTU3a PWM Timer driver
//
// Copyright (C) 2023 Renesas Electronics Corporation
//
// Hardware manual for this IP can be found here
// https://www.renesas.com/eu/en/document/mah/rzg2l-group-rzg2lc-group-users-manual-hardware-0?language=en
//
// Limitations:
// - When PWM is disabled, the output is driven to Hi-Z.
// - While the hardware supports both polarities, the driver (for now)
// only handles normal polarity.
// - HW uses one counter and two match components to configure duty_cycle
// and period.
// - Multi-Function Timer Pulse Unit (a.k.a MTU) has 7 HW channels for PWM
// operations. (The channels are MTU{0..4, 6, 7}.)
// - MTU{1, 2} channels have a single IO, whereas all other HW channels have
// 2 IOs.
// - Each IO is modelled as an independent PWM channel.
// - rz_mtu3_channel_io_map table is used to map the PWM channel to the
// corresponding HW channel as there are difference in number of IOs
// between HW channels.
//

pub const RZ_MTU3_MAX_PWM_CHANNELS: c_int = 12;
pub const RZ_MTU3_MAX_HW_CHANNELS: c_int = 7;
//
// struct rz_mtu3_channel_io_map - MTU3 pwm channel map
//
// @base_pwm_number: First PWM of a channel
// @num_channel_ios: number of IOs on the HW channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_mtu3_channel_io_map {
    pub base_pwm_number: u8,
    pub num_channel_ios: u8,
}

//
// struct rz_mtu3_pwm_channel - MTU3 pwm channel data
//
// @mtu: MTU3 channel data
// @map: MTU3 pwm channel map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_mtu3_pwm_channel {
    pub mtu: *mut rz_mtu3_channel,
    pub map: *const rz_mtu3_channel_io_map,
}

//
// struct rz_mtu3_pwm_chip - MTU3 pwm private data
//
// @clk: MTU3 module clock
// @lock: Lock to prevent concurrent access for usage count
// @rate: MTU3 clock rate
// @user_count: MTU3 usage count
// @enable_count: MTU3 enable count
// @prescale: MTU3 prescale
// @channel_data: MTU3 pwm channel data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_mtu3_pwm_chip {
    pub clk: *mut clk,
    pub lock: mutex,
    pub rate: c_ulong,
    pub user_count: [u32; RZ_MTU3_MAX_HW_CHANNELS],
    pub enable_count: [u32; RZ_MTU3_MAX_HW_CHANNELS],
    pub prescale: [u8; RZ_MTU3_MAX_HW_CHANNELS],
    pub channel_data: [rz_mtu3_pwm_channel; RZ_MTU3_MAX_HW_CHANNELS],
}

//
// The MTU channels are {0..4, 6, 7} and the number of IO on MTU1
// and MTU2 channel is 1 compared to 2 on others.
//
    static const struct rz_mtu3_channel_io_map channel_map[] = {
    { 0, 2 }, { 2, 1 }, { 3, 1 }, { 4, 2 }, { 6, 2 }, { 8, 2 }, { 10, 2 }
    };
    static inline struct rz_mtu3_pwm_chip *to_rz_mtu3_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static void rz_mtu3_pwm_read_tgr_registers(struct rz_mtu3_pwm_channel *priv,
    u16 reg_pv_offset, u16 *pv_val,
    u16 reg_dc_offset, u16 *dc_val)
    {
// pv_val = rz_mtu3_16bit_ch_read(priv->mtu, reg_pv_offset);
// dc_val = rz_mtu3_16bit_ch_read(priv->mtu, reg_dc_offset);
    }
    static void rz_mtu3_pwm_write_tgr_registers(struct rz_mtu3_pwm_channel *priv,
    u16 reg_pv_offset, u16 pv_val,
    u16 reg_dc_offset, u16 dc_val)
    {
    rz_mtu3_16bit_ch_write(priv.mtu, reg_pv_offset, pv_val);
    rz_mtu3_16bit_ch_write(priv.mtu, reg_dc_offset, dc_val);
    }
    static u8 rz_mtu3_pwm_calculate_prescale(struct rz_mtu3_pwm_chip *rz_mtu3,
    u64 period_cycles)
    {
    u32 prescaled_period_cycles;
    u8 prescale;
//
// Supported prescale values are 1, 4, 16 and 64.
// TODO: Support prescale values 2, 8, 32, 256 and 1024.
//
    prescaled_period_cycles = period_cycles >> 16;
    if (prescaled_period_cycles >= 16)
    prescale = 3;
    else
    prescale = (fls(prescaled_period_cycles) + 1) / 2;
    return prescale;
    }
    static struct rz_mtu3_pwm_channel *
    rz_mtu3_get_channel(struct rz_mtu3_pwm_chip *rz_mtu3_pwm, u32 hwpwm)
    {
    struct rz_mtu3_pwm_channel *priv = rz_mtu3_pwm.channel_data;
    unsigned int ch;
    for (ch = 0; ch < RZ_MTU3_MAX_HW_CHANNELS; ch++, priv++) {
    if (priv.map.base_pwm_number + priv.map.num_channel_ios > hwpwm)
    break;
    }
    return priv;
    }
    static bool rz_mtu3_pwm_is_ch_enabled(struct rz_mtu3_pwm_chip *rz_mtu3_pwm,
    u32 hwpwm)
    {
    struct rz_mtu3_pwm_channel *priv;
    bool is_channel_en;
    u8 val;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, hwpwm);
    is_channel_en = rz_mtu3_is_enabled(priv.mtu);
    if (!is_channel_en)
    return false;
    if (priv.map.base_pwm_number == hwpwm)
    val = rz_mtu3_8bit_ch_read(priv.mtu, RZ_MTU3_TIORH);
    else
    val = rz_mtu3_8bit_ch_read(priv.mtu, RZ_MTU3_TIORL);
    return val & RZ_MTU3_TIOR_IOA;
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int rz_mtu3_pwm_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    struct rz_mtu3_pwm_channel *priv;
    bool is_mtu3_channel_available;
    u32 ch;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    ch = priv - rz_mtu3_pwm.channel_data;
    mutex_lock(&rz_mtu3_pwm.lock);
//
// Each channel must be requested only once, so if the channel
// serves two PWMs and the other is already requested, skip over
// rz_mtu3_request_channel()
//
    if (!rz_mtu3_pwm.user_count[ch]) {
    is_mtu3_channel_available = rz_mtu3_request_channel(priv.mtu);
    if (!is_mtu3_channel_available) {
    mutex_unlock(&rz_mtu3_pwm.lock);
    return -EBUSY;
    }
    }
    rz_mtu3_pwm.user_count[ch]++;
    mutex_unlock(&rz_mtu3_pwm.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void rz_mtu3_pwm_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    struct rz_mtu3_pwm_channel *priv;
    u32 ch;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    ch = priv - rz_mtu3_pwm.channel_data;
    mutex_lock(&rz_mtu3_pwm.lock);
    rz_mtu3_pwm.user_count[ch]--;
    if (!rz_mtu3_pwm.user_count[ch])
    rz_mtu3_release_channel(priv.mtu);
    mutex_unlock(&rz_mtu3_pwm.lock);
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int rz_mtu3_pwm_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    struct rz_mtu3_pwm_channel *priv;
    u32 ch;
    u8 val;
    int rc;
    rc = pm_runtime_resume_and_get(pwmchip_parent(chip));
    if (rc)
    return rc;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    ch = priv - rz_mtu3_pwm.channel_data;
    val = RZ_MTU3_TIOR_OC_IOB_TOGGLE | RZ_MTU3_TIOR_OC_IOA_H_COMP_MATCH;
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TMDR1, RZ_MTU3_TMDR1_MD_PWMMODE1);
    if (priv.map.base_pwm_number == pwm.hwpwm)
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TIORH, val);
    else
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TIORL, val);
    mutex_lock(&rz_mtu3_pwm.lock);
    if (!rz_mtu3_pwm.enable_count[ch])
    rz_mtu3_enable(priv.mtu);
    rz_mtu3_pwm.enable_count[ch]++;
    mutex_unlock(&rz_mtu3_pwm.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void rz_mtu3_pwm_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    struct rz_mtu3_pwm_channel *priv;
    u32 ch;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    ch = priv - rz_mtu3_pwm.channel_data;
// Disable output pins of MTU3 channel
    if (priv.map.base_pwm_number == pwm.hwpwm)
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TIORH, RZ_MTU3_TIOR_OC_RETAIN);
    else
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TIORL, RZ_MTU3_TIOR_OC_RETAIN);
    mutex_lock(&rz_mtu3_pwm.lock);
    rz_mtu3_pwm.enable_count[ch]--;
    if (!rz_mtu3_pwm.enable_count[ch])
    rz_mtu3_disable(priv.mtu);
    mutex_unlock(&rz_mtu3_pwm.lock);
    pm_runtime_put_sync(pwmchip_parent(chip));
    }
    static int rz_mtu3_pwm_get_state(struct pwm_chip *chip, struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    int rc;
    rc = pm_runtime_resume_and_get(pwmchip_parent(chip));
    if (rc)
    return rc;
    state.enabled = rz_mtu3_pwm_is_ch_enabled(rz_mtu3_pwm, pwm.hwpwm);
    if (state.enabled) {
    struct rz_mtu3_pwm_channel *priv;
    u8 prescale, val;
    u16 dc, pv;
    u64 tmp;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    if (priv.map.base_pwm_number == pwm.hwpwm)
    rz_mtu3_pwm_read_tgr_registers(priv, RZ_MTU3_TGRA, &pv,
    RZ_MTU3_TGRB, &dc);
    else
    rz_mtu3_pwm_read_tgr_registers(priv, RZ_MTU3_TGRC, &pv,
    RZ_MTU3_TGRD, &dc);
    val = rz_mtu3_8bit_ch_read(priv.mtu, RZ_MTU3_TCR);
    prescale = FIELD_GET(RZ_MTU3_TCR_TPCS, val);
// With prescale <= 7 and pv <= 0xffff this doesn't overflow.
    tmp = NSEC_PER_SEC * (u64)pv << (2 * prescale);
    state.period = DIV_ROUND_UP_ULL(tmp, rz_mtu3_pwm.rate);
    tmp = NSEC_PER_SEC * (u64)dc << (2 * prescale);
    state.duty_cycle = DIV_ROUND_UP_ULL(tmp, rz_mtu3_pwm.rate);
    if (state.duty_cycle > state.period)
    state.duty_cycle = state.period;
    }
    state.polarity = PWM_POLARITY_NORMAL;
    pm_runtime_put(pwmchip_parent(chip));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_calculate_pv_or_dc(period_or_duty_cycle: u64, prescale: u8) -> u16 {
    static u16 rz_mtu3_pwm_calculate_pv_or_dc(u64 period_or_duty_cycle, u8 prescale)
    {
    return min(period_or_duty_cycle >> (2 * prescale), (u64)U16_MAX);
    }
    static int rz_mtu3_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    struct rz_mtu3_pwm_channel *priv;
    u64 period_cycles;
    u64 duty_cycles;
    u8 prescale;
    u16 pv, dc;
    u8 val;
    u32 ch;
    priv = rz_mtu3_get_channel(rz_mtu3_pwm, pwm.hwpwm);
    ch = priv - rz_mtu3_pwm.channel_data;
    period_cycles = mul_u64_u32_div(state.period, rz_mtu3_pwm.rate,
    NSEC_PER_SEC);
    prescale = rz_mtu3_pwm_calculate_prescale(rz_mtu3_pwm, period_cycles);
//
// Prescalar is shared by multiple channels, so prescale can
// NOT be modified when there are multiple channels in use with
// different settings. Modify prescalar if other PWM is off or handle
// it, if current prescale value is less than the one we want to set.
//
    if (rz_mtu3_pwm.enable_count[ch] > 1) {
    if (rz_mtu3_pwm.prescale[ch] > prescale)
    return -EBUSY;
    prescale = rz_mtu3_pwm.prescale[ch];
    }
    pv = rz_mtu3_pwm_calculate_pv_or_dc(period_cycles, prescale);
    duty_cycles = mul_u64_u32_div(state.duty_cycle, rz_mtu3_pwm.rate,
    NSEC_PER_SEC);
    dc = rz_mtu3_pwm_calculate_pv_or_dc(duty_cycles, prescale);
//
// If the PWM channel is disabled, make sure to turn on the clock
// before writing the register.
//
    if (!pwm.state.enabled) {
    int rc;
    rc = pm_runtime_resume_and_get(pwmchip_parent(chip));
    if (rc)
    return rc;
    }
    val = RZ_MTU3_TCR_CKEG_RISING | prescale;
// Counter must be stopped while updating TCR register
    if (rz_mtu3_pwm.prescale[ch] != prescale && rz_mtu3_pwm.enable_count[ch])
    rz_mtu3_disable(priv.mtu);
    if (priv.map.base_pwm_number == pwm.hwpwm) {
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TCR,
    RZ_MTU3_TCR_CCLR_TGRA | val);
    rz_mtu3_pwm_write_tgr_registers(priv, RZ_MTU3_TGRA, pv,
    RZ_MTU3_TGRB, dc);
    } else {
    rz_mtu3_8bit_ch_write(priv.mtu, RZ_MTU3_TCR,
    RZ_MTU3_TCR_CCLR_TGRC | val);
    rz_mtu3_pwm_write_tgr_registers(priv, RZ_MTU3_TGRC, pv,
    RZ_MTU3_TGRD, dc);
    }
    if (rz_mtu3_pwm.prescale[ch] != prescale) {
//
// Prescalar is shared by multiple channels, we cache the
// prescalar value from first enabled channel and use the same
// value for both channels.
//
    rz_mtu3_pwm.prescale[ch] = prescale;
    if (rz_mtu3_pwm.enable_count[ch])
    rz_mtu3_enable(priv.mtu);
    }
// If the PWM is not enabled, turn the clock off again to save power.
    if (!pwm.state.enabled)
    pm_runtime_put(pwmchip_parent(chip));
    return 0;
    }
    static int rz_mtu3_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    let mut enabled: bool = pwm.state.enabled;
    int ret;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    if (enabled)
    rz_mtu3_pwm_disable(chip, pwm);
    return 0;
    }
    mutex_lock(&rz_mtu3_pwm.lock);
    ret = rz_mtu3_pwm_config(chip, pwm, state);
    mutex_unlock(&rz_mtu3_pwm.lock);
    if (ret)
    return ret;
    if (!enabled)
    ret = rz_mtu3_pwm_enable(chip, pwm);
    return ret;
    }
    static const struct pwm_ops rz_mtu3_pwm_ops = {
    .request = rz_mtu3_pwm_request,
    .free = rz_mtu3_pwm_free,
    .get_state = rz_mtu3_pwm_get_state,
    .apply = rz_mtu3_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int rz_mtu3_pwm_pm_runtime_suspend(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    clk_disable_unprepare(rz_mtu3_pwm.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_pm_runtime_resume(dev: *mut device) -> c_int {
    static int rz_mtu3_pwm_pm_runtime_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    return clk_prepare_enable(rz_mtu3_pwm.clk);
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(rz_mtu3_pwm_pm_ops,
    rz_mtu3_pwm_pm_runtime_suspend,
    rz_mtu3_pwm_pm_runtime_resume, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_pm_disable(data: *mut c_void) {
    static void rz_mtu3_pwm_pm_disable(void *data)
    {
    struct pwm_chip *chip = data;
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    clk_rate_exclusive_put(rz_mtu3_pwm.clk);
    pm_runtime_disable(pwmchip_parent(chip));
    pm_runtime_set_suspended(pwmchip_parent(chip));
    }
#[no_mangle]
unsafe extern "C" fn rz_mtu3_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int rz_mtu3_pwm_probe(struct platform_device *pdev)
    {
    struct rz_mtu3 *parent_ddata = dev_get_drvdata(pdev.dev.parent);
    struct rz_mtu3_pwm_chip *rz_mtu3_pwm;
    struct pwm_chip *chip;
    struct device *dev = &pdev.dev;
    unsigned int i, j = 0;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, RZ_MTU3_MAX_PWM_CHANNELS,
    sizeof(*rz_mtu3_pwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    rz_mtu3_pwm = to_rz_mtu3_pwm_chip(chip);
    rz_mtu3_pwm.clk = parent_ddata.clk;
    for (i = 0; i < RZ_MTU_NUM_CHANNELS; i++) {
    if (i == RZ_MTU3_CHAN_5 || i == RZ_MTU3_CHAN_8)
    continue;
    rz_mtu3_pwm.channel_data[j].mtu = &parent_ddata.channels[i];
    rz_mtu3_pwm.channel_data[j].mtu.dev = dev;
    rz_mtu3_pwm.channel_data[j].map = &channel_map[j];
    j++;
    }
    mutex_init(&rz_mtu3_pwm.lock);
    platform_set_drvdata(pdev, chip);
    ret = clk_prepare_enable(rz_mtu3_pwm.clk);
    if (ret)
    return dev_err_probe(dev, ret, "Clock enable failed\n");
    clk_rate_exclusive_get(rz_mtu3_pwm.clk);
    rz_mtu3_pwm.rate = clk_get_rate(rz_mtu3_pwm.clk);
//
// Refuse clk rates > 1 GHz to prevent overflow later for computing
// period and duty cycle.
//
    if (rz_mtu3_pwm.rate > NSEC_PER_SEC) {
    ret = -EINVAL;
    clk_rate_exclusive_put(rz_mtu3_pwm.clk);
    goto disable_clock;
    }
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = devm_add_action_or_reset(&pdev.dev, rz_mtu3_pwm_pm_disable,
    chip);
    if (ret < 0)
    return ret;
    chip.ops = &rz_mtu3_pwm_ops;
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to add PWM chip\n");
    pm_runtime_idle(&pdev.dev);
    return 0;
    disable_clock:
    clk_disable_unprepare(rz_mtu3_pwm.clk);
    return ret;
    }
    static struct platform_driver rz_mtu3_pwm_driver = {
    .driver = {
    .name = "pwm-rz-mtu3",
    .pm = pm_ptr(&rz_mtu3_pwm_pm_ops),
    },
    .probe = rz_mtu3_pwm_probe,
    };
    module_platform_driver(rz_mtu3_pwm_driver);
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_ALIAS("platform:pwm-rz-mtu3");
    MODULE_DESCRIPTION("Renesas RZ/G2L MTU3a PWM Timer Driver");
    MODULE_LICENSE("GPL");
