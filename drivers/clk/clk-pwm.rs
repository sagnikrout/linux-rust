//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-pwm.c
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
// Copyright (C) 2014 Philipp Zabel, Pengutronix
//
// PWM (mis)used as clock output
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pwm {
    pub hw: clk_hw,
    pub pwm: *mut pwm_device,
    pub state: pwm_state,
    pub fixed_rate: u32,
}

    static inline struct clk_pwm *to_clk_pwm(struct clk_hw *hw)
    {
    return container_of(hw, struct clk_pwm, hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pwm_enable(struct clk_hw *hw)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    return pwm_apply_atomic(clk_pwm.pwm, &clk_pwm.state);
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_disable(hw: *mut clk_hw) {
    static void clk_pwm_disable(struct clk_hw *hw)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    let mut state: pwm_state = clk_pwm.state;
    state.enabled = false;
    pwm_apply_atomic(clk_pwm.pwm, &state);
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_pwm_prepare(struct clk_hw *hw)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    return pwm_apply_might_sleep(clk_pwm.pwm, &clk_pwm.state);
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_unprepare(hw: *mut clk_hw) {
    static void clk_pwm_unprepare(struct clk_hw *hw)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    pwm_disable(clk_pwm.pwm);
    }
    static unsigned long clk_pwm_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    return clk_pwm.fixed_rate;
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_get_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> c_int {
    static int clk_pwm_get_duty_cycle(struct clk_hw *hw, struct clk_duty *duty)
    {
    struct clk_pwm *clk_pwm = to_clk_pwm(hw);
    struct pwm_state state;
    int ret;
    ret = pwm_get_state_hw(clk_pwm.pwm, &state);
    if (ret)
    return ret;
    duty.num = state.duty_cycle;
    duty.den = state.period;
    return 0;
    }
    static const struct clk_ops clk_pwm_ops_atomic = {
    .enable = clk_pwm_enable,
    .disable = clk_pwm_disable,
    .recalc_rate = clk_pwm_recalc_rate,
    .get_duty_cycle = clk_pwm_get_duty_cycle,
    };
    static const struct clk_ops clk_pwm_ops = {
    .prepare = clk_pwm_prepare,
    .unprepare = clk_pwm_unprepare,
    .recalc_rate = clk_pwm_recalc_rate,
    .get_duty_cycle = clk_pwm_get_duty_cycle,
    };
#[no_mangle]
unsafe extern "C" fn clk_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int clk_pwm_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct clk_init_data init;
    struct clk_pwm *clk_pwm;
    struct pwm_device *pwm;
    struct pwm_args pargs;
    const char *clk_name;
    int ret;
    clk_pwm = devm_kzalloc(&pdev.dev, sizeof(*clk_pwm), GFP_KERNEL);
    if (!clk_pwm)
    return -ENOMEM;
    pwm = devm_pwm_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pwm))
    return PTR_ERR(pwm);
    pwm_get_args(pwm, &pargs);
    if (!pargs.period) {
    dev_err(&pdev.dev, "invalid PWM period\n");
    return -EINVAL;
    }
    if (of_property_read_u32(node, "clock-frequency", &clk_pwm.fixed_rate))
    clk_pwm.fixed_rate = div64_u64(NSEC_PER_SEC, pargs.period);
    if (!clk_pwm.fixed_rate) {
    dev_err(&pdev.dev, "fixed_rate cannot be zero\n");
    return -EINVAL;
    }
    if (pargs.period != NSEC_PER_SEC / clk_pwm.fixed_rate &&
    pargs.period != DIV_ROUND_UP(NSEC_PER_SEC, clk_pwm.fixed_rate)) {
    dev_err(&pdev.dev,
    "clock-frequency does not match PWM period\n");
    return -EINVAL;
    }
    pwm_init_state(pwm, &clk_pwm.state);
    pwm_set_relative_duty_cycle(&clk_pwm.state, 1, 2);
    clk_pwm.state.enabled = true;
    clk_name = node.name;
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    if (pwm_might_sleep(pwm))
    init.ops = &clk_pwm_ops;
    else
    init.ops = &clk_pwm_ops_atomic;
    init.flags = 0;
    init.num_parents = 0;
    clk_pwm.pwm = pwm;
    clk_pwm.hw.init = &init;
    ret = devm_clk_hw_register(&pdev.dev, &clk_pwm.hw);
    if (ret)
    return ret;
    return of_clk_add_hw_provider(node, of_clk_hw_simple_get, &clk_pwm.hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_pwm_remove(pdev: *mut platform_device) {
    static void clk_pwm_remove(struct platform_device *pdev)
    {
    of_clk_del_provider(pdev.dev.of_node);
    }
    static const struct of_device_id clk_pwm_dt_ids[] = {
    { .compatible = "pwm-clock" },
    { }
    };
    MODULE_DEVICE_TABLE(of, clk_pwm_dt_ids);
    static struct platform_driver clk_pwm_driver = {
    .probe = clk_pwm_probe,
    .remove = clk_pwm_remove,
    .driver = {
    .name = "pwm-clock",
    .of_match_table = clk_pwm_dt_ids,
    },
    };
    module_platform_driver(clk_pwm_driver);
    MODULE_AUTHOR("Philipp Zabel <p.zabel@pengutronix.de>");
    MODULE_DESCRIPTION("PWM clock driver");
    MODULE_LICENSE("GPL");
