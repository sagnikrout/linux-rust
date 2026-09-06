//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/pwm-beeper.c
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
// Copyright (C) 2010, Lars-Peter Clausen <lars@metafoo.de>
// PWM beeper driver
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_beeper {
    pub input: *mut input_dev,
    pub pwm: *mut pwm_device,
    pub amplifier: *mut regulator,
    pub work: work_struct,
    pub period: c_ulong,
    pub bell_frequency: c_uint,
    pub suspended: bool,
    pub amplifier_on: bool,
}

#[no_mangle]
unsafe extern "C" fn pwm_beeper_on(beeper: *mut pwm_beeper, period: c_ulong) -> c_int {
    static int pwm_beeper_on(struct pwm_beeper *beeper, unsigned long period)
    {
    struct pwm_state state;
    int error;
    pwm_get_state(beeper.pwm, &state);
    state.enabled = true;
    state.period = period;
    pwm_set_relative_duty_cycle(&state, 50, 100);
    error = pwm_apply_might_sleep(beeper.pwm, &state);
    if (error)
    return error;
    if (!beeper.amplifier_on) {
    error = regulator_enable(beeper.amplifier);
    if (error) {
    pwm_disable(beeper.pwm);
    return error;
    }
    beeper.amplifier_on = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_off(beeper: *mut pwm_beeper) {
    static void pwm_beeper_off(struct pwm_beeper *beeper)
    {
    if (beeper.amplifier_on) {
    regulator_disable(beeper.amplifier);
    beeper.amplifier_on = false;
    }
    pwm_disable(beeper.pwm);
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_work(work: *mut work_struct) {
    static void pwm_beeper_work(struct work_struct *work)
    {
    struct pwm_beeper *beeper = container_of(work, struct pwm_beeper, work);
    let mut period: c_ulong = READ_ONCE(beeper.period);
    if (period)
    pwm_beeper_on(beeper, period);
    else
    pwm_beeper_off(beeper);
    }
    static int pwm_beeper_event(struct input_dev *input,
    unsigned int type, unsigned int code, int value)
    {
    struct pwm_beeper *beeper = input_get_drvdata(input);
    if (type != EV_SND || value < 0)
    return -EINVAL;
    switch (code) {
    case SND_BELL:
    value = value ? beeper.bell_frequency : 0;
    break;
    case SND_TONE:
    break;
    default:
    return -EINVAL;
    }
    if (value == 0)
    beeper.period = 0;
    else
    beeper.period = HZ_TO_NANOSECONDS(value);
    if (!beeper.suspended)
    schedule_work(&beeper.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_stop(beeper: *mut pwm_beeper) {
    static void pwm_beeper_stop(struct pwm_beeper *beeper)
    {
    cancel_work_sync(&beeper.work);
    pwm_beeper_off(beeper);
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_close(input: *mut input_dev) {
    static void pwm_beeper_close(struct input_dev *input)
    {
    struct pwm_beeper *beeper = input_get_drvdata(input);
    pwm_beeper_stop(beeper);
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_beeper_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwm_beeper *beeper;
    struct pwm_state state;
    u32 bell_frequency;
    int error;
    beeper = devm_kzalloc(dev, sizeof(*beeper), GFP_KERNEL);
    if (!beeper)
    return -ENOMEM;
    beeper.pwm = devm_pwm_get(dev, core::ptr::null_mut());
    if (IS_ERR(beeper.pwm))
    return dev_err_probe(dev, PTR_ERR(beeper.pwm), "Failed to request PWM device\n");
// Sync up PWM state and ensure it is off.
    pwm_init_state(beeper.pwm, &state);
    state.enabled = false;
    error = pwm_apply_might_sleep(beeper.pwm, &state);
    if (error) {
    dev_err(dev, "failed to apply initial PWM state: %d\n",
    error);
    return error;
    }
    beeper.amplifier = devm_regulator_get(dev, "amp");
    if (IS_ERR(beeper.amplifier))
    return dev_err_probe(dev, PTR_ERR(beeper.amplifier),
    "Failed to get 'amp' regulator\n");
    INIT_WORK(&beeper.work, pwm_beeper_work);
    error = device_property_read_u32(dev, "beeper-hz", &bell_frequency);
    if (error) {
    bell_frequency = 1000;
    dev_dbg(dev,
    "failed to parse 'beeper-hz' property, using default: %uHz\n",
    bell_frequency);
    }
    beeper.bell_frequency = bell_frequency;
    beeper.input = devm_input_allocate_device(dev);
    if (!beeper.input) {
    dev_err(dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    beeper.input.name = "pwm-beeper";
    beeper.input.phys = "pwm/input0";
    beeper.input.id.bustype = BUS_HOST;
    beeper.input.id.vendor = 0x001f;
    beeper.input.id.product = 0x0001;
    beeper.input.id.version = 0x0100;
    input_set_capability(beeper.input, EV_SND, SND_TONE);
    input_set_capability(beeper.input, EV_SND, SND_BELL);
    beeper.input.event = pwm_beeper_event;
    beeper.input.close = pwm_beeper_close;
    input_set_drvdata(beeper.input, beeper);
    error = input_register_device(beeper.input);
    if (error) {
    dev_err(dev, "Failed to register input device: %d\n", error);
    return error;
    }
    platform_set_drvdata(pdev, beeper);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_suspend(dev: *mut device) -> c_int {
    static int pwm_beeper_suspend(struct device *dev)
    {
    struct pwm_beeper *beeper = dev_get_drvdata(dev);
//
// Spinlock is taken here is not to protect write to
// beeper->suspended, but to ensure that pwm_beeper_event
// does not re-submit work once flag is set.
//
    scoped_guard(spinlock_irq, &beeper.input.event_lock) {
    beeper.suspended = true;
    }
    pwm_beeper_stop(beeper);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_beeper_resume(dev: *mut device) -> c_int {
    static int pwm_beeper_resume(struct device *dev)
    {
    struct pwm_beeper *beeper = dev_get_drvdata(dev);
    scoped_guard(spinlock_irq, &beeper.input.event_lock) {
    beeper.suspended = false;
    }
// Let worker figure out if we should resume beeping
    schedule_work(&beeper.work);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pwm_beeper_pm_ops,
    pwm_beeper_suspend, pwm_beeper_resume);

    static const struct of_device_id pwm_beeper_match[] = {
    { .compatible = "pwm-beeper", },
    { },
    };
    MODULE_DEVICE_TABLE(of, pwm_beeper_match);

    static struct platform_driver pwm_beeper_driver = {
    .probe	= pwm_beeper_probe,
    .driver = {
    .name	= "pwm-beeper",
    .pm	= pm_sleep_ptr(&pwm_beeper_pm_ops),
    .of_match_table = of_match_ptr(pwm_beeper_match),
    },
    };
    module_platform_driver(pwm_beeper_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("PWM beeper driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:pwm-beeper");
