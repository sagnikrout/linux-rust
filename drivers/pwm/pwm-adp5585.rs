//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-adp5585.c
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
// Analog Devices ADP5585 PWM driver
//
// Copyright 2022 NXP
// Copyright 2024 Ideas on Board Oy
//
// Limitations:
// - The .apply() operation executes atomically, but may not wait for the
// period to complete (this is not documented and would need to be tested).
// - Disabling the PWM drives the output pin to a low level immediately.
// - The hardware can only generate normal polarity output.
//

pub const ADP5585_PWM_CHAN_NUM: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5585_pwm_chip {
    pub pwm_cfg: c_uint,
    pub pwm_offt_low: c_uint,
    pub pwm_ont_low: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5585_pwm {
    pub info: *const adp5585_pwm_chip,
    pub regmap: *mut regmap,
    pub ext_cfg: c_uint,
}

#[no_mangle]
unsafe extern "C" fn pwm_adp5585_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int pwm_adp5585_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct adp5585_pwm *adp5585_pwm = pwmchip_get_drvdata(chip);
// Configure the R3 pin as PWM output.
    return regmap_update_bits(adp5585_pwm.regmap, adp5585_pwm.ext_cfg,
    ADP5585_R3_EXTEND_CFG_MASK,
    ADP5585_R3_EXTEND_CFG_PWM_OUT);
    }
#[no_mangle]
unsafe extern "C" fn pwm_adp5585_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void pwm_adp5585_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct adp5585_pwm *adp5585_pwm = pwmchip_get_drvdata(chip);
    regmap_update_bits(adp5585_pwm.regmap, adp5585_pwm.ext_cfg,
    ADP5585_R3_EXTEND_CFG_MASK,
    ADP5585_R3_EXTEND_CFG_GPIO4);
    }
    static int pwm_adp5585_apply(struct pwm_chip *chip,
    struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct adp5585_pwm *adp5585_pwm = pwmchip_get_drvdata(chip);
    const struct adp5585_pwm_chip *info = adp5585_pwm.info;
    struct regmap *regmap = adp5585_pwm.regmap;
    u64 period, duty_cycle;
    u32 on, off;
    __le16 val;
    int ret;
    if (!state.enabled) {
    regmap_clear_bits(regmap, info.pwm_cfg, ADP5585_PWM_EN);
    return 0;
    }
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (state.period < ADP5585_PWM_MIN_PERIOD_NS)
    return -EINVAL;
    period = min(state.period, ADP5585_PWM_MAX_PERIOD_NS);
    duty_cycle = min(state.duty_cycle, period);
//
// Compute the on and off time. As the internal oscillator frequency is
// 1MHz, the calculation can be simplified without loss of precision.
//
    on = div_u64(duty_cycle, NSEC_PER_SEC / ADP5585_PWM_OSC_FREQ_HZ);
    off = div_u64(period, NSEC_PER_SEC / ADP5585_PWM_OSC_FREQ_HZ) - on;
    val = cpu_to_le16(off);
    ret = regmap_bulk_write(regmap, info.pwm_offt_low, &val, 2);
    if (ret)
    return ret;
    val = cpu_to_le16(on);
    ret = regmap_bulk_write(regmap, info.pwm_ont_low, &val, 2);
    if (ret)
    return ret;
// Enable PWM in continuous mode and no external AND'ing.
    ret = regmap_update_bits(regmap, info.pwm_cfg,
    ADP5585_PWM_IN_AND | ADP5585_PWM_MODE |
    ADP5585_PWM_EN, ADP5585_PWM_EN);
    if (ret)
    return ret;
    return regmap_set_bits(regmap, info.pwm_cfg, ADP5585_PWM_EN);
    }
    static int pwm_adp5585_get_state(struct pwm_chip *chip,
    struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct adp5585_pwm *adp5585_pwm = pwmchip_get_drvdata(chip);
    const struct adp5585_pwm_chip *info = adp5585_pwm.info;
    struct regmap *regmap = adp5585_pwm.regmap;
    unsigned int on, off;
    unsigned int val;
    __le16 on_off;
    int ret;
    ret = regmap_bulk_read(regmap, info.pwm_offt_low, &on_off, 2);
    if (ret)
    return ret;
    off = le16_to_cpu(on_off);
    ret = regmap_bulk_read(regmap, info.pwm_ont_low, &on_off, 2);
    if (ret)
    return ret;
    on = le16_to_cpu(on_off);
    state.duty_cycle = on * (NSEC_PER_SEC / ADP5585_PWM_OSC_FREQ_HZ);
    state.period = (on + off) * (NSEC_PER_SEC / ADP5585_PWM_OSC_FREQ_HZ);
    state.polarity = PWM_POLARITY_NORMAL;
    regmap_read(regmap, info.pwm_cfg, &val);
    state.enabled = !!(val & ADP5585_PWM_EN);
    return 0;
    }
    static const struct pwm_ops adp5585_pwm_ops = {
    .request = pwm_adp5585_request,
    .free = pwm_adp5585_free,
    .apply = pwm_adp5585_apply,
    .get_state = pwm_adp5585_get_state,
    };
#[no_mangle]
unsafe extern "C" fn adp5585_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int adp5585_pwm_probe(struct platform_device *pdev)
    {
    const struct platform_device_id *id = platform_get_device_id(pdev);
    struct device *dev = &pdev.dev;
    struct adp5585_dev *adp5585 = dev_get_drvdata(dev.parent);
    struct adp5585_pwm *adp5585_pwm;
    struct pwm_chip *chip;
    int ret;
    chip = devm_pwmchip_alloc(dev, ADP5585_PWM_CHAN_NUM,
    sizeof(*adp5585_pwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    adp5585_pwm = pwmchip_get_drvdata(chip);
    adp5585_pwm.regmap = adp5585.regmap;
    adp5585_pwm.ext_cfg = adp5585.regs.ext_cfg;
    adp5585_pwm.info = (const struct adp5585_pwm_chip *)id.driver_data;
    if (!adp5585_pwm.info)
    return -ENODEV;
    device_set_of_node_from_dev(dev, dev.parent);
    chip.ops = &adp5585_pwm_ops;
    ret = devm_pwmchip_add(dev, chip);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add PWM chip\n");
    return 0;
    }
    static const struct adp5585_pwm_chip adp5585_pwm_chip_info = {
    .pwm_cfg = ADP5585_PWM_CFG,
    .pwm_offt_low = ADP5585_PWM_OFFT_LOW,
    .pwm_ont_low = ADP5585_PWM_ONT_LOW,
    };
    static const struct adp5585_pwm_chip adp5589_pwm_chip_info = {
    .pwm_cfg = ADP5589_PWM_CFG,
    .pwm_offt_low = ADP5589_PWM_OFFT_LOW,
    .pwm_ont_low = ADP5589_PWM_ONT_LOW,
    };
    static const struct platform_device_id adp5585_pwm_id_table[] = {
    { .name = "adp5585-pwm", .driver_data = (kernel_ulong_t)&adp5585_pwm_chip_info },
    { .name = "adp5589-pwm", .driver_data = (kernel_ulong_t)&adp5589_pwm_chip_info },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, adp5585_pwm_id_table);
    static struct platform_driver adp5585_pwm_driver = {
    .driver	= {
    .name = "adp5585-pwm",
    },
    .probe = adp5585_pwm_probe,
    .id_table = adp5585_pwm_id_table,
    };
    module_platform_driver(adp5585_pwm_driver);
    MODULE_AUTHOR("Xiaoning Wang <xiaoning.wang@nxp.com>");
    MODULE_DESCRIPTION("ADP5585 PWM Driver");
    MODULE_LICENSE("GPL");
