//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-ntxec.c
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
// The Netronix embedded controller is a microcontroller found in some
// e-book readers designed by the original design manufacturer Netronix, Inc.
// It contains RTC, battery monitoring, system power management, and PWM
// functionality.
//
// This driver implements PWM output.
//
// Copyright 2020 Jonathan Neuschäfer <j.neuschaefer@gmx.net>
//
// Limitations:
// - The get_state callback is not implemented, because the current state of
// the PWM output can't be read back from the hardware.
// - The hardware can only generate normal polarity output.
// - The period and duty cycle can't be changed together in one atomic action.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntxec_pwm {
    pub ec: *mut ntxec,
}

    static struct ntxec_pwm *ntxec_pwm_from_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
pub const NTXEC_REG_AUTO_OFF_HI: c_uint = 0xa1;
pub const NTXEC_REG_AUTO_OFF_LO: c_uint = 0xa2;
pub const NTXEC_REG_ENABLE: c_uint = 0xa3;
pub const NTXEC_REG_PERIOD_LOW: c_uint = 0xa4;
pub const NTXEC_REG_PERIOD_HIGH: c_uint = 0xa5;
pub const NTXEC_REG_DUTY_LOW: c_uint = 0xa6;
pub const NTXEC_REG_DUTY_HIGH: c_uint = 0xa7;
//
// The time base used in the EC is 8MHz, or 125ns. Period and duty cycle are
// measured in this unit.
//
pub const TIME_BASE_NS: c_int = 125;
//
// The maximum input value (in nanoseconds) is determined by the time base and
// the range of the hardware registers that hold the converted value.
// It fits into 32 bits, so we can do our calculations in 32 bits as well.
//

    static int ntxec_pwm_set_raw_period_and_duty_cycle(struct pwm_chip *chip,
    int period, int duty)
    {
    struct ntxec_pwm *priv = ntxec_pwm_from_chip(chip);
//
// Changes to the period and duty cycle take effect as soon as the
// corresponding low byte is written, so the hardware may be configured
// to an inconsistent state after the period is written and before the
// duty cycle is fully written. If, in such a case, the old duty cycle
// is longer than the new period, the EC may output 100% for a moment.
//
// To minimize the time between the changes to period and duty cycle
// taking effect, the writes are interleaved.
//
    struct reg_sequence regs[] = {
    { NTXEC_REG_PERIOD_HIGH, ntxec_reg8(period >> 8) },
    { NTXEC_REG_DUTY_HIGH, ntxec_reg8(duty >> 8) },
    { NTXEC_REG_PERIOD_LOW, ntxec_reg8(period) },
    { NTXEC_REG_DUTY_LOW, ntxec_reg8(duty) },
    };
    return regmap_multi_reg_write(priv.ec.regmap, regs, ARRAY_SIZE(regs));
    }
    static int ntxec_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm_dev,
    const struct pwm_state *state)
    {
    struct ntxec_pwm *priv = ntxec_pwm_from_chip(chip);
    unsigned int period, duty;
    int res;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    period = min_t(u64, state.period, MAX_PERIOD_NS);
    duty   = min_t(u64, state.duty_cycle, period);
    period /= TIME_BASE_NS;
    duty   /= TIME_BASE_NS;
//
// Writing a duty cycle of zero puts the device into a state where
// writing a higher duty cycle doesn't result in the brightness that it
// usually results in. This can be fixed by cycling the ENABLE register.
//
// As a workaround, write ENABLE=0 when the duty cycle is zero.
// The case that something has previously set the duty cycle to zero
// but ENABLE=1, is not handled.
//
    if (state.enabled && duty != 0) {
    res = ntxec_pwm_set_raw_period_and_duty_cycle(chip, period, duty);
    if (res)
    return res;
    res = regmap_write(priv.ec.regmap, NTXEC_REG_ENABLE, ntxec_reg8(1));
    if (res)
    return res;
// Disable the auto-off timer
    res = regmap_write(priv.ec.regmap, NTXEC_REG_AUTO_OFF_HI, ntxec_reg8(0xff));
    if (res)
    return res;
    return regmap_write(priv.ec.regmap, NTXEC_REG_AUTO_OFF_LO, ntxec_reg8(0xff));
    } else {
    return regmap_write(priv.ec.regmap, NTXEC_REG_ENABLE, ntxec_reg8(0));
    }
    }
    static const struct pwm_ops ntxec_pwm_ops = {
    .apply = ntxec_pwm_apply,
//
// No .get_state callback, because the current state cannot be read
// back from the hardware.
//
    };
#[no_mangle]
unsafe extern "C" fn ntxec_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int ntxec_pwm_probe(struct platform_device *pdev)
    {
    struct ntxec *ec = dev_get_drvdata(pdev.dev.parent);
    struct ntxec_pwm *priv;
    struct pwm_chip *chip;
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*priv));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    priv = ntxec_pwm_from_chip(chip);
    priv.ec = ec;
    chip.ops = &ntxec_pwm_ops;
    return devm_pwmchip_add(&pdev.dev, chip);
    }
    static struct platform_driver ntxec_pwm_driver = {
    .driver = {
    .name = "ntxec-pwm",
    },
    .probe = ntxec_pwm_probe,
    };
    module_platform_driver(ntxec_pwm_driver);
    MODULE_AUTHOR("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
    MODULE_DESCRIPTION("PWM driver for Netronix EC");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ntxec-pwm");
