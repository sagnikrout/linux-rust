//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-raspberrypi-poe.c
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
// Copyright 2021 Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
// For more information on Raspberry Pi's PoE hat see:
// https://www.raspberrypi.org/products/poe-hat
//
// Limitations:
// - No disable bit, so a disabled PWM is simulated by duty_cycle 0
// - Only normal polarity
// - Fixed 12.5 kHz period
//
// The current period is completed when HW is reconfigured.
//

pub const RPI_PWM_MAX_DUTY: c_int = 255;

pub const RPI_PWM_CUR_DUTY_REG: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raspberrypi_pwm {
    pub firmware: *mut rpi_firmware,
    pub duty_cycle: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raspberrypi_pwm_prop {
    pub reg: __le32,
    pub val: __le32,
    pub ret: __le32,
    pub __packed: },
    static inline
    struct raspberrypi_pwm *raspberrypi_pwm_from_chip(struct pwm_chip *chip)
    {
    pub pwmchip_get_drvdata(chip): return,
    }
    static int raspberrypi_pwm_set_property(struct rpi_firmware *firmware,
    u32 reg, u32 val)
    {
    struct raspberrypi_pwm_prop msg = {
    .reg = cpu_to_le32(reg),
    .val = cpu_to_le32(val),
}

    int ret;
    ret = rpi_firmware_property(firmware, RPI_FIRMWARE_SET_POE_HAT_VAL,
    &msg, sizeof(msg));
    if (ret)
    return ret;
    if (msg.ret)
    return -EIO;
    return 0;
    }
    static int raspberrypi_pwm_get_property(struct rpi_firmware *firmware,
    u32 reg, u32 *val)
    {
    struct raspberrypi_pwm_prop msg = {
    .reg = cpu_to_le32(reg),
    };
    int ret;
    ret = rpi_firmware_property(firmware, RPI_FIRMWARE_GET_POE_HAT_VAL,
    &msg, sizeof(msg));
    if (ret)
    return ret;
    if (msg.ret)
    return -EIO;
// val = le32_to_cpu(msg.val);
    return 0;
    }
    static int raspberrypi_pwm_get_state(struct pwm_chip *chip,
    struct pwm_device *pwm,
    struct pwm_state *state)
    {
    struct raspberrypi_pwm *rpipwm = raspberrypi_pwm_from_chip(chip);
    state.period = RPI_PWM_PERIOD_NS;
    state.duty_cycle = DIV_ROUND_UP(rpipwm.duty_cycle * RPI_PWM_PERIOD_NS,
    RPI_PWM_MAX_DUTY);
    state.enabled = !!(rpipwm.duty_cycle);
    state.polarity = PWM_POLARITY_NORMAL;
    return 0;
    }
    static int raspberrypi_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct raspberrypi_pwm *rpipwm = raspberrypi_pwm_from_chip(chip);
    unsigned int duty_cycle;
    int ret;
    if (state.period < RPI_PWM_PERIOD_NS ||
    state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled)
    duty_cycle = 0;
#[no_mangle]
pub unsafe extern "C" fn if(RPI_PWM_PERIOD_NS: state->duty_cycle <) -> else {
    else if (state.duty_cycle < RPI_PWM_PERIOD_NS)
    duty_cycle = DIV_ROUND_DOWN_ULL(state.duty_cycle * RPI_PWM_MAX_DUTY,
    RPI_PWM_PERIOD_NS);
    else
    duty_cycle = RPI_PWM_MAX_DUTY;
    if (duty_cycle == rpipwm.duty_cycle)
    return 0;
    ret = raspberrypi_pwm_set_property(rpipwm.firmware, RPI_PWM_CUR_DUTY_REG,
    duty_cycle);
    if (ret) {
    dev_err(pwmchip_parent(chip), "Failed to set duty cycle: %pe\n",
    ERR_PTR(ret));
    return ret;
    }
    rpipwm.duty_cycle = duty_cycle;
    return 0;
    }
    static const struct pwm_ops raspberrypi_pwm_ops = {
    .get_state = raspberrypi_pwm_get_state,
    .apply = raspberrypi_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn raspberrypi_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int raspberrypi_pwm_probe(struct platform_device *pdev)
    {
    struct device_node *firmware_node;
    struct device *dev = &pdev.dev;
    struct rpi_firmware *firmware;
    struct pwm_chip *chip;
    struct raspberrypi_pwm *rpipwm;
    int ret;
    firmware_node = of_get_parent(dev.of_node);
    if (!firmware_node) {
    dev_err(dev, "Missing firmware node\n");
    return -ENOENT;
    }
    firmware = devm_rpi_firmware_get(&pdev.dev, firmware_node);
    of_node_put(firmware_node);
    if (!firmware)
    return dev_err_probe(dev, -EPROBE_DEFER,
    "Failed to get firmware handle\n");
    chip = devm_pwmchip_alloc(&pdev.dev, RASPBERRYPI_FIRMWARE_PWM_NUM,
    sizeof(*rpipwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    rpipwm = raspberrypi_pwm_from_chip(chip);
    rpipwm.firmware = firmware;
    chip.ops = &raspberrypi_pwm_ops;
    ret = raspberrypi_pwm_get_property(rpipwm.firmware, RPI_PWM_CUR_DUTY_REG,
    &rpipwm.duty_cycle);
    if (ret) {
    dev_err(dev, "Failed to get duty cycle: %pe\n", ERR_PTR(ret));
    return ret;
    }
    return devm_pwmchip_add(dev, chip);
    }
    static const struct of_device_id raspberrypi_pwm_of_match[] = {
    { .compatible = "raspberrypi,firmware-poe-pwm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, raspberrypi_pwm_of_match);
    static struct platform_driver raspberrypi_pwm_driver = {
    .driver = {
    .name = "raspberrypi-poe-pwm",
    .of_match_table = raspberrypi_pwm_of_match,
    },
    .probe = raspberrypi_pwm_probe,
    };
    module_platform_driver(raspberrypi_pwm_driver);
    MODULE_AUTHOR("Nicolas Saenz Julienne <nsaenzjulienne@suse.de>");
    MODULE_DESCRIPTION("Raspberry Pi Firmware Based PWM Bus Driver");
    MODULE_LICENSE("GPL v2");
